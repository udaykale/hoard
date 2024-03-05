// This file holds implementation for a Remote FIFO cache.
// It will support sharding and replication.
// Sharding can be configured by num_shards
// Replicas can be configured by num_shards_copies
// Same in case num_shards_copies > 0, the instance of the cache will not host more than one copy of the same shard.
#[cfg(test)]
mod tests {}
