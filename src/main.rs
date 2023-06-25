// we use Mutex to allow multiple ownership of the same node
use std::{
    sync::{Arc, Mutex},
    thread,
};

#[derive(Debug)]
#[allow(dead_code)]
struct Node {
    value: Mutex<String>,
    // we use Arc to allow multiple ownership of the same node
    adjacent: Vec<Arc<Node>>,
}

fn add_urgency(node: &Node) {
    // wrap it in a block to release the lock as soon as we are done
    {
        let mut current_value = node.value.lock().unwrap();
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
        // replace RwLock with Mutex
        // difference of Mutex with RwLock is that Mutex is not reentrant
        // meaning that if a thread already has the lock, it cannot acquire it again
        // this is to prevent deadlocks
        value: Mutex::new("abc".to_owned()),
        adjacent: vec![],
    });

    let b = Arc::new(Node {
        value: Mutex::new("def".to_owned()),
        adjacent: vec![a.clone()],
    });

    let c = Arc::new(Node {
        value: Mutex::new("ghi".to_owned()),
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
