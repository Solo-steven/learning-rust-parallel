pub mod work_pool;
pub mod thread_saft_node;
pub mod traversal;

use crate::traversal::{parallel_visit,recursive_visit};
use crate::work_pool::WokerPool;
use crate::thread_saft_node::create_full_tree;
use std::sync::{Arc, Mutex};
use std::time::Instant;

fn main() {
    let pool = WokerPool::new(5);
    let mut root = create_full_tree(20);

    let parallel_now = Instant::now();
    parallel_visit(&mut root, &Arc::new(Mutex::new(pool)));
    println!("Parallel Traversal: {}", parallel_now.elapsed().as_nanos());
    let recursive_now = Instant::now();
    recursive_visit(&mut root);
    println!("Recursive Traversal: {}", recursive_now.elapsed().as_nanos());
}