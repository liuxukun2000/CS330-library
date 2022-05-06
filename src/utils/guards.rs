use rocket::http::CookieJar;
use rocket::http::Status;
use rocket::outcome::IntoOutcome;
use rocket::Request;
use rocket::request::{FromRequest, Outcome};
use rocket::State;

use crate::user::models::User;

#[derive(Debug)]
pub enum AuthError {
    Missing,
    Invalid,
}

pub struct IsLogin(
    pub(crate) User
);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for IsLogin {
    type Error = AuthError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match request.cookies().get_private("token") {
            Some(userinfo) => {
                let userinfo = userinfo.value();
                if let Ok(userinfo) = serde_json::from_str::<User>(userinfo) {
                    return Outcome::Success(IsLogin(userinfo));
                }
                Outcome::Failure((Status::Unauthorized, AuthError::Invalid))
            }
            None => {
                Outcome::Failure((Status::Unauthorized, AuthError::Missing))
            }
        }
    }
}