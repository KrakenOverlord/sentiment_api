use anyhow::Result;
use serde::Serialize;
use sqlx::{MySqlConnection, Connection, types::chrono::NaiveDate};

// https://docs.rs/sqlx/latest/sqlx/mysql/types/index.html
#[derive(Debug, Serialize)]
pub struct Rollup {
    pub id:         i32,        // Maps to MariaDB INT type
    pub date:       NaiveDate,  // Maps to MariaDB DATE type
    pub sentiment:  f32,        // Maps to MariaDB FLOAT type
    pub price:      i32,        // Maps to MariaDB INT type
}

#[derive(Debug)]
pub struct Database {
    conn: MySqlConnection,
}

impl Database {
    pub async fn new() -> Result<Self> {
        let url = std::env::var("DATABASE_URL")?;
        let conn = MySqlConnection::connect(&url).await?;
        Ok(Database { conn })
    }

    pub async fn get_rollups(&mut self, date: NaiveDate) -> Result<Vec<Rollup>> {
        let rollups = sqlx::query_as!(Rollup, "SELECT * FROM rollups WHERE date >= ? ORDER BY DATE ASC", date)
            .fetch_all(&mut self.conn)
            .await?;
        Ok(rollups)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Duration;
    use sqlx::types::chrono::{Utc, self};
    use dotenv::dotenv;

    #[tokio::test]
    async fn test() {
        // dotenv().ok();
        // let mut database = Database::new().await.unwrap();
        // let date = (Utc::now() - Duration::days(1)).date_naive();

        // let rollups = database .get_rollups(date).await.expect("Couldn't retrieve rollups");
        // for rollup in rollups {
        //     println!("{:?}", rollup);
        // }
    }
}