use crate::*;

use uuid::Uuid;

#[derive(Debug, Clone, serde::Serialize)]
pub struct Message {
	pub id: Uuid,
	pub channel_id: Uuid,
	pub content: String,
}

pub async fn create_message(pool: &sqlx::sqlite::SqlitePool, channel_id: Uuid, content: &str) -> error::Result<Message> {
	let id = Uuid::now_v7();

	Ok(sqlx::query_as!(
		Message,
		r#"
            INSERT INTO messages
                (id, channel_id, content)
            VALUES
                (?, ?, ?)
            RETURNING
                id as "id!: Uuid",
                channel_id as "channel_id!: Uuid",
                content as "content!"
            ;
        "#,
		id,
		channel_id,
		content
	)
	.fetch_one(pool)
	.await?)
}

pub async fn get_messages(pool: &sqlx::sqlite::SqlitePool, channel_id: Uuid, before_id: Option<Uuid>, limit: i64) -> error::Result<Vec<Message>> {
	Ok(sqlx::query_as!(
		Message,
		r#"
            SELECT
                id as "id!: Uuid",
                channel_id as "channel_id!: Uuid",
                content as "content!"
            FROM (
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
