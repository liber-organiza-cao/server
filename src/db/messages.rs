use crate::*;

#[derive(Debug, Clone, serde::Serialize)]
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

pub async fn get_messages(pool: &sqlx::sqlite::SqlitePool, before_id: Option<i64>, limit: i64) -> error::Result<Vec<Message>> {
	Ok(sqlx::query_as!(
		Message,
		r#"
        SELECT id, content FROM (
            SELECT
                id, content
            FROM
                messages
            WHERE
                (?1 IS NULL OR id < ?1)
            ORDER BY
                id DESC
            LIMIT
                ?2
        )
        ORDER BY id ASC
        "#,
		before_id,
		limit
	)
	.fetch_all(pool)
	.await?)
}
