use std::cell::RefCell;
use std::thread;

#[derive(Debug)]
#[allow(dead_code)]
struct Node<'a> {
    // RefCell is a container type that allows mutating the value inside
    // have methods such as
    // borrow , borrow_mut , try_borrow , try_borrow_mut
    value: RefCell<String>,
    adjacent: Vec<&'a Node<'a>>,
}

fn add_urgency(node: &Node) {
    // this would panic if we tried to borrow_mut twice
    let mut current_value = node.value.borrow_mut();
    current_value.push_str("!");
    // We can also use try_borrow_mut to get a Result instead of panicking

    // let mut current_value = node.value.try_borrow_mut().unwrap();
    // *current_value += "!";


    for adj in node.adjacent.iter() {
        add_urgency(&adj);
    }
}

fn main() {
    let a = Node {
        value: RefCell::new("abc".to_owned()),
        adjacent: vec![],
    };
    let b = Node {
        value: RefCell::new("def".to_owned()),
        adjacent: vec![&a],
    };
    let c = Node {
        value: RefCell::new("ghi".to_owned()),
        adjacent: vec![&a],
    };

    thread::spawn(|| {
        add_urgency(&a);
    });

    println!("After adding one to all nodes:");

    dbg!(&a);
    dbg!(&b);
}
