use sqlx::PgPool;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Clone)]
pub struct Shard {
    pub id: Uuid,
    pub pool: PgPool,
}
impl Shard {
    pub async fn new(connecttion_str: &str) -> Self {
        let pool = PgPool::connect(connecttion_str).await.unwrap();
        Shard {
            id: Uuid::new_v4(),
            pool,
        }
    }
}

pub struct ShardedDb {
    shards: Arc<Mutex<HashMap<Uuid, Shard>>>,
}
impl ShardedDb {
    pub async fn new() -> Self {
        ShardedDb {
            shards: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    pub async fn add_shard(&self, connect_str: &str) -> Uuid {
        let shard = Shard::new(connect_str).await;
        let id = shard.id;
        self.shards.lock().await.insert(id, shard);
        id
    }
    pub async fn get_shard(&self, id: Uuid) -> Option<Shard> {
        let shards = self.shards.lock().await;
        shards.get(&id).cloned()
    }
    pub async fn shard_count(&self) -> usize {
        let shard = self.shards.lock().await;
        shard.len()
    }
}

