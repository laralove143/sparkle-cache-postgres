use crate::Cache;

#[tokio::test]
async fn new_cache() {
    Cache::new(env!("DATABASE_URL")).await.unwrap();
}
