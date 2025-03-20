use anyhow::{Ok, Result};
use redis::AsyncCommands;

use crate::traits::BuildsRepository;

#[derive(Debug, Clone)]
pub struct BuildsRedisDatabase {
    client: redis::Client,
}

#[allow(dead_code)]
impl BuildsRedisDatabase {
    pub async fn new(url: String) -> Result<Self> {
        Ok(BuildsRedisDatabase {
            client: redis::Client::open(url)?,
        })
    }
}

impl Default for BuildsRedisDatabase {
    fn default() -> Self {
        BuildsRedisDatabase {
            client: redis::Client::open("redis://localhost:6379").unwrap(),
        }
    }
}

impl BuildsRepository for BuildsRedisDatabase {
    async fn insert_build(&self, build: String) -> anyhow::Result<String> {
        let mut connection = self.client.get_multiplexed_tokio_connection().await?;
        let id = uuid::Uuid::new_v4().to_string();
        let _: () = connection.set(&id, build).await?;
        Ok(id)
    }

    async fn get_build(&self, id: String) -> anyhow::Result<String> {
        let mut connection = self.client.get_multiplexed_tokio_connection().await?;
        let build: String = connection.get(id).await?;
        Ok(build)
    }
}

// pub struct BuildsPostgresDatabase {
//     db: Pool<Postgres>
// }

// impl BuildsPostgresDatabase {
//     async fn new(db: Pool<Postgres>) -> Self {
//         BuildsPostgresDatabase { db }
//     }
// }

// impl BuildsRepository for BuildsPostgresDatabase {
//     async fn insert_build(&self, build: String) -> Result<String> {
//         sqlx::query!("INSERT INTO builds (build) VALUES ($1) RETURNING id", build)
//             .fetch_one(self.db)
//             .await
//             .map(|row| row.id.to_string())
//     }

//     async fn get_build(&self, id: String) -> Result<String> {
//         sqlx::query!("SELECT build FROM builds WHERE id = $1", id)
//             .fetch_one(self.db)
//             .await
//             .map(|row| row.build)
//     }
// }
