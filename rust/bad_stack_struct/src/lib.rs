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
                    break 'looper;
                }
            }
        }
        None
    }
    pub fn split_at(&mut self, where_at: usize) -> Result<List, liberr::Err> {
        // loc variable needs to be 1, as we are already checking for where_at is at the 0th index.
        // this means we automatically skip when `loc` should be 0.
        let mut loc: usize = 1;
        let mut item = self.next.as_mut();
        loop {
            if where_at == 0 {
                // Special case where we want to take the whole list. Leaves Head empty
                let take = core::mem::take(&mut self.next);
                let mut head = Head::new();
                head.next = take;
                return Ok(head);
            } else if loc == where_at {
                let take = match item {
                    Some(node) => {
                        if node.next.is_some() {
                            core::mem::take(&mut node.next)
                        } else {
                            return Err(liberr::Err::new("NOTHING HERE CHIEF".to_owned(), line!()));
                        }
                    }
                    None => return Err(liberr::Err::new("NOTHING HERE CHIEF".to_owned(), line!())),
                };
                let mut head = Head::new();
                head.next = take;
                return Ok(head);
            } else if let Some(node) = item {
                loc += 1;
                item = node.next.as_mut();
            } else {
                return Err(liberr::Err::new(
                    "NO NODE AT LOCATION {where_at} FOUND".to_owned(),
                    line!(),
                ));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::use_debug)]
    use super::*;
    const VALS: [i32; 9] = [0, 1, 2, 3, 4, 5, 6, 7, 8];

    fn get_list() -> List {
        let mut list = List::new();
        VALS.iter().for_each(|&val| list.insert(val));
        list
    }

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
    }
    #[test]
    fn split() -> Result<(), liberr::Err> {
        let split_loc = 3;
        let initial_len = VALS.len();
        let mut list = get_list();
        let new_list = list.split_at(split_loc)?;
        assert!(!new_list.is_empty(), "NEW LIST SHOULD NOT BE EMPTY.");
        let len_new_actual = new_list.len();
        let len_new_expected = initial_len - split_loc;
        let len_old_expected = split_loc;
        let len_old_actual = list.len();
        assert_eq!(len_new_expected, len_new_actual, "NEW LIST WAS NOT SPLIT IN THE RIGHT LOCATION.\n NEW LIST SHOULD HAVE LENGTH {len_new_expected} HAS {len_new_actual}");
        assert_eq!(len_old_expected, len_old_actual, "OLD LIST WAS NOT SPLIT IN THE RIGHT LOCATION.\n OLD LIST SHOULD HAVE LENGTH {len_old_expected} HAS {len_old_actual}");
        Ok(())
    }
}
