use crate::*;

#[derive(Debug, Clone, serde::Serialize)]
pub struct Channel {
	pub id: i64,
	pub name: String,
}

pub async fn create_channel(pool: &sqlx::sqlite::SqlitePool, name: &str) -> error::Result<()> {
	sqlx::query!(
		r#"
        INSERT INTO channels
            (name)
        VALUES
            (?)
    	;
	"#,
		name
	)
	.execute(pool)
	.await?;

	Ok(())
}

pub async fn get_channels(pool: &sqlx::sqlite::SqlitePool) -> error::Result<Vec<Channel>> {
	Ok(sqlx::query_as!(
		Channel,
		r#"
		SELECT
			id, name
		FROM 
			channels
		ORDER BY
			id ASC
		"#,
	)
	.fetch_all(pool)
	.await?)
}
