mod shard;
use shard::{Shard, ShardedDb};
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let sharded_db = ShardedDb::new().await;
    let shard1_id = sharded_db
        .add_shard("postgres://user:password@localhost/shard1")
        .await;
    let shard2_id = sharded_db
        .add_shard("postgres://user:password@localhost/shard2")
        .await;

    println!("Added Shard 1 with ID: {}", shard1_id);
    println!("Added Shard 2 with ID: {}", shard2_id);
    let user_id = 123456;
    let shard_id = get_shard_id(user_id, sharded_db.shard_count().await);
    let shard = sharded_db.get_shard(shard_id).await.unwrap();
    println!("User {} is assigned to Shard {}", user_id, shard_id);
}

fn get_shard_id(user_id: i32, shard_count: usize) -> Uuid {
    let shard_index = (user_id % shard_count as i32) as usize;
    let shards = vec![
        "00000000-0000-0000-0000-000000000001",
        "00000000-0000-0000-0000-000000000002",
    ];
    Uuid::parse_str(shards[shard_index]).unwrap()
}
