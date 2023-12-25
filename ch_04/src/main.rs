use std::rc::Rc;

struct Node {
    value: i32,
    next: Option<Rc<Node>>,
}

fn main() {
    let mut a = Rc::new(Node {
        value: 1,
        next: None,
    });
    let b = Rc::new(Node {
        value: 2,
        next: Some(a.clone()),
    });
    let c = Rc::new(Node {
        value: 3,
        next: Some(b.clone()),
    });

    // Create a reference cycle by setting a's next value to c
    a.next = Some(c.clone()); // error: cannot assign to data in an `Rc`
}
