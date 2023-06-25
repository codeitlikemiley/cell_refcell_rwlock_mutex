#[derive(Debug)]
// add unused attribute to suppress warnings, and derived debug
#[allow(dead_code)]
// we add a lifetime parameter to the struct
struct Node<'a> {
    value: i32,
    adjacent: Vec<&'a Node<'a>>,
}

// this causes error , since we are trying to mutate a borrowed value
fn add_one(node: Node) {
    node.val = node.value + 1;
    for adj in node.adjacent {
        add_one(&adj);
    }
}


fn main() {
    let a =  Node { value: 1, adjacent: vec![] };
    let b = Node { value: 2, adjacent: vec![&a]};
    let c  = Node { value: 3, adjacent: vec![&a, &b]};

    dbg!(&a);
    dbg!(&b);
    dbg!(&c);
}
