#[derive(Debug)]
// add unused attribute to suppress warnings, and derived debug
#[allow(dead_code)]
// we add a lifetime parameter to the struct
struct Node<'a> {
    value: i32,
    adjacent: Vec<&'a Node<'a>>,
}


fn main() {
    let a =  Node { value: 1, adjacent: vec![] };
    let b = Node { value: 2, adjacent: vec![&a]};
    let c  = Node { value: 3, adjacent: vec![&a, &b]};

    dbg!(&a);
    dbg!(&b);
    dbg!(&c);
}
