// we use RwLock to allow multiple readers or one writer
// RwLock will block the current thread until the lock is available
use std::{
    sync::{Arc, RwLock},
    thread,
};

#[derive(Debug)]
#[allow(dead_code)]
struct Node {
    value: RwLock<String>,
    // we use Arc to allow multiple ownership of the same node
    adjacent: Vec<Arc<Node>>,
}

fn add_urgency(node: &Node) {
    // wrap it in a block to release the lock as soon as we are done
    {
        let mut current_value = node.value.write().unwrap();
        current_value.push_str("!");
    }
    for adj in node.adjacent.iter() {
        add_urgency(&adj);
    }
}

fn main() {
    // we use here Arc to allow multiple ownership of the same node

    // we first declare the nodes
    let a = Arc::new(Node {
        value: RwLock::new("abc".to_owned()),
        adjacent: vec![],
    });

    let b = Arc::new(Node {
        value: RwLock::new("def".to_owned()),
        adjacent: vec![a.clone()],
    });

    let c = Arc::new(Node {
        value: RwLock::new("ghi".to_owned()),
        adjacent: vec![a.clone()],
    });

     // we spawn two threads to add urgency to all nodes
    let t1_b = b.clone();
    let t1 = thread::spawn(move || {
        add_urgency(&t1_b);
    });

    let t2_c = c.clone();
    let t2 = thread::spawn(move || {
        add_urgency(&t2_c);
    });

    // wait for the threads to finish
    t1.join().unwrap();
    t2.join().unwrap();

    println!("After adding one to all nodes:");
    // we can read the value without blocking
    // we use &* to dereference the RwLock
    dbg!(&*a);
    dbg!(&*b);
}
