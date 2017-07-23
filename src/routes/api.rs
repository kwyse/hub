use diesel::prelude::*;
use rocket::{Request, State, Outcome};
use rocket::http::Status;
use rocket::request::{FromRequest, Outcome as RequestOutcome};
use rocket_contrib::{Json, Value};
use serde::Serialize;

use db::DatabaseConnection;
use db::models::Article;
use db::schema::articles::dsl::*;
use settings::Settings;

#[get("/articles/<article_id>")]
pub fn show(article_id: i32, conn: DatabaseConnection, _key: ApiKey) -> Json<Value> {
    match articles.find(article_id).first::<Article>(&*conn) {
        Ok(article) => json_as_success(article),
        Err(_) => json_as_error("Article not found")
    }
}

fn json_as_success<T: Serialize>(data: T) -> Json<Value> {
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
