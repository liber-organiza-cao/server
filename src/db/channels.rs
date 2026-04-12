use crate::*;

use uuid::Uuid;

#[derive(Debug, Clone, serde::Serialize)]
pub struct Channel {
	pub id: Uuid,
	pub name: String,
}

pub async fn create_channel(pool: &sqlx::sqlite::SqlitePool, name: &str) -> error::Result<Channel> {
	let id = Uuid::now_v7();

	Ok(sqlx::query_as!(
		Channel,
		r#"
			INSERT INTO channels
				(id, name)
			VALUES
				(?, ?)
			RETURNING
				id as "id!: Uuid",
				name
		;"#,
		id,
		name
	)
	.fetch_one(pool)
	.await?)
}

pub async fn delete_channel(pool: &sqlx::sqlite::SqlitePool, id: Uuid) -> error::Result<bool> {
	Ok(sqlx::query(
		r#"
			DELETE FROM
				channels
			WHERE
				id = ?
		;"#,
	)
	.bind(id)
	.execute(pool)
	.await?
	.rows_affected()
		> 0)
}

pub async fn get_channels(pool: &sqlx::sqlite::SqlitePool) -> error::Result<Vec<Channel>> {
	Ok(sqlx::query_as!(
		Channel,
		r#"
			SELECT
				id as "id!: Uuid",
				name
			FROM 
				channels
			ORDER BY
				id ASC
		;"#,
	)
	.fetch_all(pool)
	.await?)
}
