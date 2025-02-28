use anyhow::Error;
use axum::http::uri::Builder;
use axum::http::Uri;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use chrono::{Duration, Utc};
use jsonwebtoken::{EncodingKey, Header};
use p256::pkcs8::EncodePrivateKey;

use p256::{PublicKey, SecretKey};
use reqwest::header::{AUTHORIZATION, CONTENT_ENCODING, CONTENT_LENGTH, CONTENT_TYPE};
use reqwest::{Client, Response};
use serde::Serialize;

pub struct Vapid {
    pub private_key: SecretKey,
    pub public_key: PublicKey,
    client: Client,
    ttl: i64,
    sub: String,
}

impl Vapid {
    pub fn new(private_key: SecretKey, public_key: PublicKey, sub: String, ttl: i64) -> Self {
        let client = Client::new();

        Self {
            client,
            private_key,
            public_key,
            ttl,
            sub,
        }
    }

    pub async fn send(
        &self,
        data: Vec<u8>,
        endpoint: String,
        auth: Vec<u8>,
        public_key: Vec<u8>,
    ) -> Result<Response, Error> {
        // TODO: Swith to native instead of openssl
        let ciphertext = ece::encrypt(&public_key, &auth, &data).unwrap();
        let claims = VapidClaims::new(endpoint.clone().try_into()?, self.sub.clone(), self.ttl)?;
        let jwt = self.create_jwt(claims).await?;

        let res = self
            .client
            .post(endpoint)
            .header(
                AUTHORIZATION,
                format!(
                    "vapid t={}, k={}",
                    jwt,
                    URL_SAFE_NO_PAD.encode(self.public_key.to_sec1_bytes())
                ),
            )
            .header(CONTENT_ENCODING, "aes128gcm")
            .header(CONTENT_TYPE, "application/octet-stream")
            .header("TTL", self.ttl)
            .header(CONTENT_LENGTH, ciphertext.len())
            .body(ciphertext)
            .send()
            .await?;

        // TODO: handle not 200 errors

        Ok(res)
    }

    async fn create_jwt(&self, claims: VapidClaims) -> Result<String, Error> {
        let header = Header::new(jsonwebtoken::Algorithm::ES256);
        let key = EncodingKey::from_ec_pem(
            self.private_key
                .to_pkcs8_pem(Default::default())?
                .to_string()
                .as_ref(),
        )?;

        Ok(jsonwebtoken::encode(&header, &claims, &key)?)
    }
}

#[derive(Serialize)]
struct VapidClaims {
    pub sub: String,
    pub aud: String,
    pub exp: i64,
}

impl VapidClaims {
    pub fn new(endpoint: Uri, sub: String, ttl: i64) -> Result<Self, Error> {
        let aud = Builder::from(endpoint)
            .path_and_query("/")
            .build()?
            .to_string();
        let aud = aud[0..aud.len() - 1].into();
        let exp = (Utc::now() + Duration::seconds(ttl)).timestamp();

        Ok(Self { sub, aud, exp })
    }
}

#[cfg(test)]
mod tests {
    use super::VapidClaims;

    #[test]
    fn test_path_is_ok() {
        let claims = VapidClaims::new(
            "https://example.com/path?query".try_into().unwrap(),
            "sub".into(),
            0,
        )
        .unwrap();

        assert_eq!(claims.aud, "https://example.com");
        assert_eq!(claims.sub, "sub");
    }
}
