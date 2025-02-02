/// Storage on disk and in memory, cleaning up lower utility items when too full.
///
/// * This is a 'cache' (without hashable keys etc), so only for data that can be regenerated.
/// * Items should be the same after de/ser with serde, otherwise disk and memory won't behave the same.
/// * Items are given unique u64 numbers. If this overflows, the cache is wiped.
///   Users should clear all their references, otherwise there may be collisions.
/// * Not optimized for `Copy` types; assumes relatively big data.
///
/// This is just the storage backend; hashing and lookups should happen in `db`.

mod common;
mod store;
mod memory;
mod disk;
