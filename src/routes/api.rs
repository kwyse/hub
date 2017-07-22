use diesel::prelude::*;
use rocket::Route;
use rocket::response::status;
use rocket_contrib::Json;

use db::DatabaseConnection;
use db::models::Article;
use db::schema::articles::dsl::*;

pub fn get() -> Vec<Route> {
    routes![
        show
    ]
}

#[get("/articles/<article_id>")]
pub fn show(article_id: i32, conn: DatabaseConnection) -> Result<Json<Article>, status::NotFound<String>> {
    articles.find(article_id)
        .first::<Article>(&*conn)
        .map(|article| Json(article))
        .map_err(|_| status::NotFound(String::from("No article found for provided ID")))
}
