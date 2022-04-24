use criterion::{criterion_group, criterion_main, BatchSize, Criterion};
use kvs::{KvStore, KvsEngine, SledKvsEngine, thread_pool::RayonThreadPool};
use rand::{prelude::*, rngs::SmallRng};
use sled;
use tempfile::TempDir;

fn set_bench(c: &mut Criterion) {
    let num_cpus = num_cpus::get() as u32;
    let mut group = c.benchmark_group("set_bench");
    group.bench_function("kvs", |b| {
        b.iter_batched(
            || {
                let rt = tokio::runtime::Runtime::new().unwrap();
                let temp_dir = TempDir::new().unwrap();
                (KvStore::<RayonThreadPool>::open(temp_dir.path(), num_cpus).unwrap(), temp_dir, rt)
            },
            |(store, _temp_dir, rt)| {
                rt.block_on(async {
                    for i in 1..(1 << 5) {
                        let _ = store.set(format!("key{}", i), "value".to_string()).await;
                    }
                })
            },
            BatchSize::SmallInput,
        )
    });
    group.bench_function("sled", |b| {
        b.iter_batched(
            || {
                let rt = tokio::runtime::Runtime::new().unwrap();
                let temp_dir = TempDir::new().unwrap();
                (SledKvsEngine::<RayonThreadPool>::new(sled::open(&temp_dir).unwrap(), num_cpus).unwrap(), temp_dir, rt)
            },
            |(db, _temp_dir, rt)| {
                rt.block_on(async {
                    for i in 1..(1 << 5) {
                        let _ = db.set(format!("key{}", i), "value".to_string()).await;
                    }
                })
            },
            BatchSize::SmallInput,
        )
    });
    group.finish();
}

fn get_bench(c: &mut Criterion) {
    let num_cpus = num_cpus::get() as u32;
    let mut group = c.benchmark_group("get_bench");
    for i in &vec![5] {
        group.bench_with_input(format!("kvs_{}", i), i, |b, i| {
            let rt = tokio::runtime::Runtime::new().unwrap();
            let temp_dir = TempDir::new().unwrap();
            let store = KvStore::<RayonThreadPool>::open(temp_dir.path(), num_cpus).unwrap();
            for key_i in 1..(1 << i) {
                rt.block_on(async {
                    let _ = store.set(format!("key{}", key_i), "value".to_string()).await;
                })
            }
            let mut rng = SmallRng::from_seed([0; 32]);
            b.iter(|| {
                rt.block_on(async {
                    let _ = store.get(format!("key{}", rng.gen_range::<i32, _>(1..1 << i))).await;
                })
            })
        });
    }
    for i in &vec![5] {
        group.bench_with_input(format!("sled_{}", i), i, |b, i| {
            let rt = tokio::runtime::Runtime::new().unwrap();
            let temp_dir = TempDir::new().unwrap();
            let db = SledKvsEngine::<RayonThreadPool>::new(sled::open(&temp_dir).unwrap(), num_cpus).unwrap();
            for key_i in 1..(1 << i) {
                rt.block_on(async {
                    let _ = db.set(format!("key{}", key_i), "value".to_string()).await;
                });
            }
            let mut rng = SmallRng::from_seed([0; 32]);
            b.iter(|| {
                rt.block_on(async {
                    let _ = db.get(format!("key{}", rng.gen_range::<i32, _>(1..1 << i))).await;
                })
            })
        });
    }
    group.finish();
}

criterion_group!(benches, set_bench, get_bench);
criterion_main!(benches);
