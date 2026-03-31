use crate::*;

pub struct Message {
	pub id: i64,
	pub content: String,
}

pub async fn create_message(pool: &sqlx::sqlite::SqlitePool, content: &str) -> error::Result<Message> {
	Ok(sqlx::query_as!(
		Message,
		r#"
        INSERT INTO messages
            (content)
        VALUES
            (?)
        RETURNING
            id, content
    	;
    "#,
		content
	)
	.fetch_one(pool)
	.await?)
}

pub async fn get_messages(pool: &sqlx::sqlite::SqlitePool) -> error::Result<Vec<Message>> {
	Ok(sqlx::query_as!(
		Message,
		r#"
        SELECT id, content FROM messages ORDER BY id ASC;
    "#
	)
	.fetch_all(pool)
	.await?)
}
