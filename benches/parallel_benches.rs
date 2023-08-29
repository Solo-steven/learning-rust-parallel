use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use learning_rust_parallel::thread_saft_node::create_full_tree;
use learning_rust_parallel::traversal::{parallel_visit, recursive_visit};
use learning_rust_parallel::work_pool::WokerPool;
use std::sync::{Arc, Mutex};

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_with_input(
        BenchmarkId::new("parallel traversal", "string of boostrap css"), 
        &create_full_tree(15), 
        |b, root| {
            b.iter(|| { 
                let worker_pool_for_parallel = WokerPool::new(1);
                parallel_visit(&mut Arc::clone(root), &Arc::new(Mutex::new(worker_pool_for_parallel)));
            })
        }
    );
    c.bench_with_input(
        BenchmarkId::new("recursive traversal", "string of boostrap css"), 
        &create_full_tree(15), 
        |b, root| {
            b.iter(|| { 
                let worker_pool_for_parallel = WokerPool::new(10);
                let _arc =Arc::new(Mutex::new(worker_pool_for_parallel));
                recursive_visit(&mut Arc::clone(root));
            })
        }
    );

}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);