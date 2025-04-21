use std::sync::Arc;

use async_nats::jetstream;
use flux_lib::error::Error;
use flux_users_api::users_service_client::UsersServiceClient;
use sea_orm::{ConnectOptions, Database, DbConn};
use tonic::transport::Channel;

use super::{
    pushes::state::PushesState,
    settings::{AppSettings, ClientsSettings},
    AppJS,
};

#[derive(Clone)]
pub struct AppState {
    pub settings: AppSettings,
    pub js: Arc<AppJS>,
    pub db: Arc<DbConn>,
    pub clients: Clients,
    pub pushes_state: PushesState,
}

impl AppState {
    pub async fn new(settings: AppSettings) -> Result<Self, Error> {
        let nats = async_nats::connect(&settings.nats.endpoint).await?;
        let js = Arc::new(jetstream::new(nats));

        let opt = ConnectOptions::new(&settings.db.endpoint);
        let db = Arc::new(Database::connect(opt).await?);

        let pushes_state = PushesState::new(&settings.pushes).await?;

        let clients = Clients::new(&settings.clients).await?;

        Ok(Self {
            settings,
            clients,
            db,
            js,
            pushes_state,
        })
    }
}

#[derive(Clone)]
pub struct Clients {
    pub users_service_client: UsersServiceChannel,
}

impl Clients {
    pub async fn new(settings: &ClientsSettings) -> Result<Self, Error> {
        let users_service_client =
            Self::users_service_client(settings.flux_users.endpoint.clone()).await?;

        Ok(Self {
            users_service_client,
        })
    }

    async fn users_service_client(dst: String) -> Result<UsersServiceChannel, Error> {
        let ch = tonic::transport::Endpoint::new(dst)?.connect_lazy();

        Ok(UsersServiceClient::new(ch))
    }
}

pub type UsersServiceChannel = UsersServiceClient<Channel>;
