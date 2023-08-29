use std::sync::{Arc, Mutex};
#[derive(Debug)]
pub struct Node {
    pub value: i32,
    pub left: ThreadSafeNodeLink,
    pub right: ThreadSafeNodeLink,
}
pub type ThreadSafeNode = Arc<Mutex<Node>>;
pub type ThreadSafeNodeLink = Option<ThreadSafeNode>;

pub fn create_full_tree(size: u32) -> ThreadSafeNode {
    let mut value = 0;
    let mut root = Some(Arc::new(Mutex::new( Node {
        value, 
        left: None,
        right: None,
    })));
    value += 1;
    let mut layer = vec![Arc::clone(root.as_mut().unwrap())];
    for _i in 0..size {
        let mut next_layer = vec![];
        for node_ref in layer.into_iter() {
            node_ref.lock().unwrap().left = Some(Arc::new(Mutex::new(Node { value, left: None, right: None })));
            value += 1;
            node_ref.lock().unwrap().right = Some(Arc::new(Mutex::new(Node { value, left: None, right: None })));
            value += 1;

            next_layer.push(Arc::clone(node_ref.lock().unwrap().left.as_mut().unwrap()));
            next_layer.push(Arc::clone(node_ref.lock().unwrap().right.as_mut().unwrap()));
        }
        layer = next_layer;
    } 
    root.unwrap()
}
