use diesel::prelude::*;

use db::DatabaseConnection;
use db::models::BlogPost;
use db::schema::blog_posts::dsl::*;

pub fn find_post_by_id(post_id: i32, conn: DatabaseConnection) -> Option<BlogPost> {
    blog_posts.find(post_id)
        .first::<BlogPost>(&*conn)
        .ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_one_post_by_id_successfully() {
        let conn = connection_with_one_published_blog_post();
        let post = find_post_by_id(1, conn);

        assert!(post.is_some());
    }

    #[test]
    fn fail_to_find_one_post_by_id() {
        let conn = connection_with_no_blog_posts();
        let post = find_post_by_id(1, conn);

        assert!(post.is_none());
    }

    fn connection_with_no_blog_posts() -> DatabaseConnection {
        use diesel::pg::PgConnection;
        use r2d2::{Config, Pool};
        use r2d2_diesel::ConnectionManager;
        use std::env;

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let config = Config::builder().pool_size(1).build();
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::new(config, manager).unwrap();

        let conn = pool.get().unwrap();
        conn.begin_test_transaction().unwrap();
        DatabaseConnection(conn)
    }

    fn connection_with_one_published_blog_post() -> DatabaseConnection {
        let conn = connection_with_no_blog_posts();
        conn.execute("
            INSERT INTO blog_posts
            VALUES
            (1, 'Post Title', 'The content of the post', true)
        ").unwrap();
        conn
    }
}
