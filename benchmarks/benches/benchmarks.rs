// Copyright (c) The Starcoin Core Contributors
// SPDX-License-Identifier: Apache-2.0
use benchmarks::storage::StorageBencher;
use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use crypto::{hash::CryptoHash, HashValue};
use starcoin_accumulator::node::ACCUMULATOR_PLACEHOLDER_HASH;
use starcoin_accumulator::{Accumulator, MerkleAccumulator};
use std::sync::Arc;
use storage::cache_storage::CacheStorage;
use storage::db_storage::DBStorage;
use storage::storage::StorageInstance;
use storage::IntoSuper;
use storage::Storage;

//
// Storage benchmarks
//
fn storage_transaction(c: &mut Criterion) {
    c.bench_function("storage_transaction", |b| {
        let cache_storage = Arc::new(CacheStorage::new());
        let db_storage = Arc::new(DBStorage::new(std::env::temp_dir()));
        let storage = Storage::new(StorageInstance::new_cache_and_db_instance(
            cache_storage,
            db_storage,
        ))
        .unwrap();
        // let storage =
        //     Storage::new(StorageInstance::new_cache_instance(CacheStorage::new())).unwrap();
        // let storage = Storage::new(StorageInstance::new_db_instance(db_storage)).unwrap();
        let bencher = StorageBencher::new(storage);
        bencher.bench(b)
    });
}

/// accumulator benchmarks
fn accumulator_append(c: &mut Criterion) {
    c.bench_function("accumulator_append", |b| {
        // let storage = Arc::new(
        //     Storage::new(StorageInstance::new_cache_instance(CacheStorage::new())).unwrap(),
        // );
        let cache_storage = Arc::new(CacheStorage::new());
        let db_storage = Arc::new(DBStorage::new(std::env::temp_dir()));
        let storage = Arc::new(
            Storage::new(StorageInstance::new_cache_and_db_instance(
                cache_storage,
                db_storage,
            ))
            .unwrap(),
        );
        let leaves = create_leaves(0..100);
        b.iter_batched(
            || {
                MerkleAccumulator::new(
                    HashValue::random(),
                    *ACCUMULATOR_PLACEHOLDER_HASH,
                    vec![],
                    0,
                    0,
                    storage.clone().into_super_arc(),
                )
                .unwrap()
            },
            |bench| {
                bench.append(&leaves);
                bench.flush().unwrap();
            },
            BatchSize::LargeInput,
        )
    });
}
fn create_leaves(nums: std::ops::Range<usize>) -> Vec<HashValue> {
    nums.map(|x| x.to_be_bytes().as_ref().crypto_hash())
        .collect()
}
criterion_group!(starcoin_benches, storage_transaction, accumulator_append);
criterion_main!(starcoin_benches);
