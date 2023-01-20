#![allow(clippy::missing_docs_in_private_items)]

#[derive(Debug)]
struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

impl Node {
    pub fn new(value: i32) -> Self {
        Self { value, next: None }
    }
    pub fn set_next(&mut self, node: Node) {
        self.next = Some(Box::from(node));
    }
    pub fn get_value(&self) -> &i32 {
        &self.value
    }
    pub fn get_next(&mut self) -> Option<&mut Box<Node>> {
        self.next.as_mut()
    }
}

#[derive(Debug)]
struct Head {
    next: Option<Box<Node>>,
}

impl Head {
    pub fn new() -> Self {
        Self { next: None }
    }
    pub fn insert(&mut self, value: i32) {
        if self.next.is_none() {
            self.next = Some(Box::from(Node::new(value)));
        } else{
            let mut prev: &mut Box<Node> = self.next.as_mut().unwrap();
            'looper: loop {
                    // cannot call a method since it will take a &mut ref  another time.
                    if let Some(ref mut v) = prev.next {
                        prev = v;
                    } else {
                        prev.set_next(Node::new(value));
                        break 'looper;
                    }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const VALS: [i32; 5] = [1, 2, 3, 4, 5];

    #[test]
    fn it_works() {
        let mut head = Head::new();
        let mut tmp: Option<&Box<Node>> = None;
        for (i, &value) in VALS.iter().enumerate() {
            head.insert(value);
        }
        println!("{head:?}");
    }
}