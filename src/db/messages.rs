use crate::*;

#[derive(Debug, Clone, serde::Serialize)]
pub struct Message {
	pub id: i64,
	pub channel_id: i64,
	pub content: String,
}

pub async fn create_message(pool: &sqlx::sqlite::SqlitePool, channel_id: i64, content: &str) -> error::Result<Message> {
	Ok(sqlx::query_as!(
		Message,
		r#"
            INSERT INTO messages
                (channel_id, content)
            VALUES
                (?, ?)
            RETURNING
                id as "id!", channel_id as "channel_id!", content as "content!"
            ;
        "#,
		channel_id,
		content
	)
	.fetch_one(pool)
	.await?)
}

pub async fn get_messages(pool: &sqlx::sqlite::SqlitePool, channel_id: i64, before_id: Option<i64>, limit: i64) -> error::Result<Vec<Message>> {
	Ok(sqlx::query_as!(
		Message,
		r#"
            SELECT id, channel_id, content FROM (
                SELECT
                    id, channel_id, content
                FROM
                    messages
                WHERE
                    channel_id = ?1 AND (?2 IS NULL OR id < ?2)
                ORDER BY
                    id DESC
                LIMIT
                    ?3
            )
            ORDER BY id ASC
        "#,
		channel_id,
		before_id,
		limit
	)
	.fetch_all(pool)
	.await?)
}
