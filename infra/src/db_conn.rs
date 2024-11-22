use sea_orm::{Database, DatabaseConnection, DbErr};

pub async fn db_connection(db_url: &str) -> Result<DatabaseConnection, DbErr> {
  let db = Database::connect(db_url).await?;
  Ok(db)
}