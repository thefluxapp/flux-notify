use crate::app::{error::AppError, state::AppState};

pub async fn message(
    AppState {
        clients,
        js,
        settings,
        ..
    }: AppState,
    req: message::Request,
) -> Result<(), AppError> {
    let user = message::get_user(&clients.users_service_client, &req.user_id).await?;

    js.publish(
        settings.events.messaging.event.subject,
        req.try_into_bytes(&user)?,
    )
    .await?;

    Ok(())
}

pub mod message {
    use bytes::Bytes;
    use flux_notify_api::{event, Event};
    use flux_users_api::GetUsersRequest;
    use prost::Message as _;
    use prost_types::Timestamp;

    use crate::app::{error::AppError, state::UsersServiceChannel};

    #[derive(Debug)]
    pub struct Request {
        pub message_id: String,
        pub user_id: String,
        pub text: String,
        pub code: String,
        pub order: i64,
        pub stream: Option<Stream>,
        pub created_at: Timestamp,
        pub updated_at: Timestamp,
    }

    #[derive(Debug)]
    pub struct Stream {
        pub stream_id: String,
        pub message_id: String,
    }

    pub struct User(flux_users_api::get_users_response::User);

    impl Request {
        pub fn try_into_bytes(self, user: &User) -> Result<Bytes, AppError> {
            let message = event::Payload::Message(flux_notify_api::Message {
                message_id: Some(self.message_id.into()),
                text: Some(self.text),
                code: Some(self.code),
                order: Some(self.order),
                user: Some(user.into()),
                stream: self.stream.map(Into::into),
                created_at: Some(self.created_at),
                updated_at: Some(self.updated_at),
            });

            let event = Event {
                payload: Some(message),
            };

            let mut buf = Vec::with_capacity(0);
            event.encode(&mut buf)?;
            Ok(buf.into())
        }
    }

    impl From<Stream> for flux_notify_api::message::Stream {
        fn from(stream: Stream) -> Self {
            Self {
                stream_id: Some(stream.stream_id),
                message_id: Some(stream.message_id),
            }
        }
    }

    impl From<&User> for flux_notify_api::message::User {
        fn from(user: &User) -> Self {
            let user = &user.0;

            Self {
                user_id: Some(user.user_id().into()),
                first_name: Some(user.first_name().into()),
                last_name: Some(user.last_name().into()),
                name: Some(user.name().into()),
                abbr: Some(user.abbr().into()),
                color: Some(user.color().into()),
            }
        }
    }

    pub async fn get_user(
        users_service_client: &UsersServiceChannel,
        user_id: &String,
    ) -> Result<User, AppError> {
        let response = users_service_client
            .clone()
            .get_users(GetUsersRequest {
                user_ids: vec![user_id.clone()],
            })
            .await?
            .into_inner();

        let user = response
            .users
            .into_iter()
            .find(|user| user.user_id() == user_id)
            .ok_or(AppError::Empty)?;

        Ok(User(user))
    }
}
