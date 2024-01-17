use std::cell::Cell;

#[derive(Debug)]
struct Node<'a> {
    val: Cell<i32>,
    adjacent: Vec<&'a Node<'a>>
}


fn add_one(node: &Node) {
    let curr_val = node.val.get();
    node.val.set(curr_val + 1);
    for adj in node.adjacent.iter() {
            add_one(&adj);
        }
}

pub fn run() {

    let a = Node {
        val: Cell::from(1),
        adjacent: vec![]
    };

    let b = Node {
        val: Cell::from(2),
        adjacent: vec![&a]
    };

    let c = Node {
        val: Cell::from(3),
        adjacent: vec![&a]
    };

    dbg!(&b);

    add_one(&b);

    dbg!(&b);
    dbg!(c);
}

