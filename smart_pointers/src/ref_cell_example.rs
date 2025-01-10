use std::cell::RefCell;

#[derive(Debug)]
struct Node<'a> {
    val: RefCell<String>,
    adjacent: Vec<&'a Node<'a>>,
}

fn add_urgency(node: &Node) {
    let mut val = node.val.borrow_mut();
    val.push('!');

    for adj in node.adjacent.iter() {
        add_urgency(&adj);
    }
}

pub fn rf_cell_example() {
    let a = Node {
        val: RefCell::new(String::from("aaa")),
        adjacent: vec![],
    };

    let b = Node {
        val: RefCell::new(String::from("bbb")),
        adjacent: vec![&a],
    };

    let c = Node {
        val: RefCell::new(String::from("ccc")),
        adjacent: vec![&a],
    };

    add_urgency(&b);

    dbg!(&a);
    dbg!(&b);
    dbg!(&c);
}
