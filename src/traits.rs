use anyhow::Result;

pub trait BuildsRepository {
    // async fn new(db: Pool<Postgres>) -> Self {}
    
    async fn insert_build(&self, build: String) -> Result<String>;
    async fn get_build(&self, id: String) -> Result<String>;
}