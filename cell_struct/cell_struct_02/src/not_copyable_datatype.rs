use std::cell::RefCell;

#[derive(Debug)]
struct Node<'a> {
    val: RefCell<String>,
    adjacent: Vec<&'a Node<'a>>
}


fn add_one(node: &Node) {
    let mut curr_val = node.val.borrow_mut();
    curr_val.push_str("!");
    for adj in node.adjacent.iter() {
            add_one(&adj);
        }
}

pub fn run() {

    let a = Node {
        val: RefCell::new("aaa".to_string()),
        adjacent: vec![]
    };

    let b = Node {
        val: RefCell::new("bbb".to_string()),
        adjacent: vec![&a]
    };

    let c = Node {
        val: RefCell::new("ccc".to_string()),
        adjacent: vec![&a]
    };

    dbg!(&b);

    add_one(&b);

    dbg!(&b);
    dbg!(c);
}


