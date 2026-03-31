use crate::*;

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
