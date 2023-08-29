use crate::work_pool::ThreadSafeWorkPool;
use crate::thread_saft_node::ThreadSafeNode;
use std::sync::Arc;
 
pub fn parallel_visit(root: &mut ThreadSafeNode, worker_pool: &ThreadSafeWorkPool ) {
    //println!("Visit index: {:?}", root.lock().unwrap().value);
    if !worker_pool.lock().unwrap().is_full() {
        if root.lock().unwrap().right.is_some() {
            let worker_pool_arc = Arc::clone(&worker_pool);
            let mut root_right_arc = Arc::clone(root.lock().unwrap().right.as_mut().unwrap());
            worker_pool.lock().unwrap().execute(move || {
                parallel_visit(&mut root_right_arc, &worker_pool_arc)
            });
            if root.lock().unwrap().left.is_some() {
                parallel_visit(root.lock().unwrap().left.as_mut().unwrap(), worker_pool);
            }
            return;
        };
    };
    if root.lock().unwrap().left.is_some() {
        parallel_visit(root.lock().unwrap().left.as_mut().unwrap(), worker_pool);
    }
    if root.lock().unwrap().right.is_some() {
        parallel_visit(root.lock().unwrap().right.as_mut().unwrap(), worker_pool);
    }
}
pub fn recursive_visit(root: &mut ThreadSafeNode,) {
   //println!("Visit index: {:?}", root.lock().unwrap().value);
    if root.lock().unwrap().left.is_some() {
        recursive_visit(root.lock().unwrap().left.as_mut().unwrap());
    }
    if root.lock().unwrap().right.is_some() {
        recursive_visit(root.lock().unwrap().right.as_mut().unwrap());
    }
}
