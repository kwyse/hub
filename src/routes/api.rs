use rocket::{Request, State, Outcome};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome as RequestOutcome};
use rocket_contrib::{Json, Value};
use serde::Serialize;

use db::api as db_api;
use db::DatabaseConnection;
use settings::Settings;

#[get("/posts/<post_id>")]
pub fn show(post_id: i32, conn: DatabaseConnection, _key: ApiKey) -> Json<Value> {
    match db_api::find_post_by_id(post_id, conn) {
        Some(post) => json_as_success(&post),
        None => json_as_error("Blog post not found")
    }
}

fn json_as_success<T: Serialize>(data: &T) -> Json<Value> {
    Json(json!({
        "status": "success",
        "data": data,
    }))
}

fn json_as_error(reason: &str) -> Json<Value> {
    Json(json!({
        "status": "error",
        "reason": reason,
    }))
}

pub struct ApiKey(String);

impl<'a, 'r> FromRequest<'a, 'r> for ApiKey {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> RequestOutcome<ApiKey, ()> {
        let settings = request.guard::<State<Settings>>()?;
        match request.headers().get_one("Authorization") {
            Some(key) => {
                if key == settings.api_key {
                    Outcome::Success(ApiKey(key.to_string()))
                } else {
                    Outcome::Failure((Status::Unauthorized, ()))
                }
            },
            None => Outcome::Failure((Status::Unauthorized, ()))
        }
    }
}
