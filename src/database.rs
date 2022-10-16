struct Shard {
    name: String,
    index: i32,
    address: String,
    config: std::path::PathBuf,
    shard: String,
    replica: String
}