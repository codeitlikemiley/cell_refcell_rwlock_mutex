
#[derive(Debug)]
struct Node {
    value: i32,
    adjacent: Vec<&Node>
}




fn main() {
    let a =  Node { value: 1, adjacent: vec![] };
    let b = Node { value: 2, adjacent: vec![&a]};

    dbg!(a);
}
