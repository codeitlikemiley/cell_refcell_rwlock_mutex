use std::cell::Cell;

#[derive(Debug)]
#[allow(dead_code)]
struct Node<'a> {
    // we use Cell to allow mutation of the value field
    // if we have a piece of data that is copyable
    // and we dont mind copying it , which not gonna be a performance hit
    // just like in this case with i32 , then Cell is a good choice
    // If we have something that is not copyable like a [String] , then we can use RefCell
    value: Cell<String>,
    adjacent: Vec<&'a Node<'a>>,
}

fn add_one(node: &Node) {
    // we have setter and getter methods added by Cell
    let current_value = node.value.get();
    node.value.set(current_value + 1);

    // for adj in &node.adjacent {
        // add_one(&adj);
    // }

    // we use iter() to grab shared references to the adjacent nodes
    // we dont wanna take ownership here
    for adj in node.adjacent.iter() {
        add_one(&adj);
    }
}

fn main() {
    let a = Node {
        value: Cell::new(1),
        adjacent: vec![],
    };
    let b = Node {
        value: Cell::new(2),
        adjacent: vec![&a],
    };
    let c = Node {
        value: Cell::new(4),
        adjacent: vec![&a],
    };

    add_one(&b);

    println!("After adding one to all nodes:");

    dbg!(&a);
    dbg!(&b);
}
