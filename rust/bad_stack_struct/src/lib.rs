#![allow(clippy::missing_docs_in_private_items)]

#[derive(Debug, Default)]
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
        if let Some(mut prev) = self.next.as_mut() {
            'looper: loop {
                // cannot call a method since it will take a &mut ref  another time.
                if let Some(ref mut v) = prev.next {
                    prev = v;
                } else {
                    prev.next = Some(Box::from(Node::new(value)));
                    break 'looper;
                }
            }
        } else {
            self.next = Some(Box::from(Node::new(value)));
        }
    }
    pub fn search(&self, value:i32) -> Option<usize> {
        let mut location = 0;
        if let Some(mut prev) = self.next.as_ref() {
            'looper: loop {
                if prev.value == value {
                    return Some(location);
                } else if let Some(ref v) = prev.next {
                    location += 1;
                    prev = v;
                } else {
                    return None;
                }
            }
        } else {
            None
        }
    }
    pub fn split_at(&mut self, where_at: usize) -> Result<Head, liberr::Err> {
        let mut loc: usize = 0;
        // first need to check if it wants the first item;
        if let Some(mut item) = self.next.as_mut() {
            'looper: loop {
                if loc == where_at {
                    let mut head = Head::new();
                    head.next = Some(core::mem::take(item));
                    return Ok(head);
                } else if let Some(ref mut next) = item.next {
                    loc += 1;
                    item = next;
                } else {
                    return Err(liberr::Err::new("NO NODE AT LOCATION {where_at} FOUND".to_owned(), line!()));
                }
            }
        } else {
            Err(liberr::Err::new("NO ITEMS".to_owned(), line!()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const VALS: [i32; 5] = [1, 2, 3, 4, 5];

    #[test]
    fn it_works() -> Result<(), liberr::Err> {
        let mut head = Head::new();
        let tmp: Option<&Box<Node>> = None;
        for (i, &value) in VALS.iter().enumerate() {
            head.insert(value);
        }
        println!("{head:?}");
        if let Some(val)  = head.search(5) {
            println!("Found value {val}");
        } else {
            println!("Found NOTHING");
        }
        let new_head = head.split_at(2)?;
        println!("{new_head:?}");
        println!("{head:?}");
        Ok(())
    }
}
