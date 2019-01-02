struct Node<'a, T: 'a> {
    value: T,
    next: Option<&'a Node<'a, T>>,
    previous: Option<&'a Node<'a, T>>,
}

struct List<'b, T: 'b> {
    head: Option<&'b Node<'b, T>>,
}

impl<'b, T> List<'b, T> {
    fn add_node(&self) -> &List<'b, T> {
        & self
    }
}

fn main() {
    let t1 = Node {
        value: 5,
        next: None,
        previous: None,
    };

    let t2 = Node {
        value: 10,
        next: Some(&t1),
        previous: None,
    };
}
