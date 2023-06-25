use std::cell::Cell;

#[derive(Debug)]
#[allow(dead_code)]
struct Node<'a> {
    value: Cell<i32>,
    adjacent: Vec<&'a Node<'a>>,
}

fn add_one(node: &Node) {
    // we have setter and getter methods added by Cell
    let current_value = node.value.get();
    node.value.set(current_value + 1);

    for adj in &node.adjacent {
        add_one(adj);
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
        value: Cell::new(3),
        adjacent: vec![&a, &b],
    };

    dbg!(&a);
    dbg!(&b);
    dbg!(&c);

    add_one(&c);

    println!("After adding one to all nodes:");

    dbg!(&a);
    dbg!(&b);
    dbg!(&c);
}
