#![allow(
    clippy::missing_docs_in_private_items,
    clippy::missing_errors_doc,
    clippy::new_without_default,
    clippy::missing_inline_in_public_items
)]

#[derive(Debug, Default)]
pub struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

impl Node {
    #[must_use]
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
pub struct List {
    next: Option<Box<Node>>,
}

impl List {
    #[must_use]
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
    #[must_use]
    pub fn search(&self, value: i32) -> Option<usize> {
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
    pub fn split_off(&mut self, at: usize) -> Option<List> {
        // TODO: Decide if split should return an empty list when Original list is empty or
        //  where_at > len or return None
        self.next
            .is_some()
            .then(|| {
                let node = self.split_off_raw(at);
                node.is_some().then(|| {
                    let mut list = List::new();
                    list.next = node;
                    list
                })
            })
            .unwrap_or_default()
    }
    #[inline]
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.next.is_none()
    }
    #[must_use]
    pub fn len(&self) -> usize {
        if self.is_empty() {
            return 0;
        }
        let mut len = 0;
        let mut wrapped_item = self.next.as_ref();
        loop {
            if let Some(unwrapped_item) = wrapped_item {
                len += 1;
                if let Some(_) = &unwrapped_item.next {
                    wrapped_item = unwrapped_item.next.as_ref();
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        len
    }
    pub fn pop(&mut self) -> Option<Box<Node>> {
        self.next
            .is_some()
            .then(|| {
                let last_loc = self.len();
                self.split_off_raw(last_loc - 1)
            })
            .unwrap_or_default()
    }
    pub fn split_off_raw(&mut self, at: usize) -> Option<Box<Node>> {
        // loc variable needs to be 1, as we are already checking for where_at is at the 0th index.
        // this means we automatically skip when `loc` should be 0.
        let mut loc = 1;
        let mut wrapped_node = self.next.as_mut();
        loop {
            if at == 0 || loc == at {
                let take = match at {
                    0 => core::mem::take(&mut self.next),
                    _ => {
                        if let Some(node) = wrapped_node {
                            core::mem::take(&mut node.next)
                        } else {
                            break;
                        }
                    }
                };
                return take;
            } else if let Some(node) = wrapped_node {
                loc += 1;
                wrapped_node = node.next.as_mut();
            } else {
                break;
            }
        }
        None
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
    fn create_list() {
        let _list = List::new();
    }
    #[test]
    fn insert_list() {
        let mut list = List::new();
        // let len = VALS.len();
        for (i, &value) in VALS.iter().enumerate() {
            println!(
                "INSERTING {} OF {} VALUES, VAL: {value} ",
                i + 1,
                VALS.len()
            );
            list.insert(value);
        }
        println!("{list:?}");
    }
    #[test]
    fn split() {
        let split_loc = 3;
        let initial_len = VALS.len();
        let mut list = get_list();
        let new_list = list.split_off(split_loc);
        if let Some(new_list) = new_list {
            let len_new_actual = new_list.len();
            let len_new_expected = initial_len - split_loc;
            let len_old_expected = split_loc;
            let len_old_actual = list.len();
            assert_eq!(len_new_expected, len_new_actual, "NEW LIST WAS NOT SPLIT IN THE RIGHT LOCATION.\n NEW LIST SHOULD HAVE LENGTH {len_new_expected} HAS {len_new_actual}");
            assert_eq!(len_old_expected, len_old_actual, "OLD LIST WAS NOT SPLIT IN THE RIGHT LOCATION.\n OLD LIST SHOULD HAVE LENGTH {len_old_expected} HAS {len_old_actual}");
        } else {
            assert!(new_list.is_none(), "NEW LIST SHOULD NOT BE EMPTY.");
        }
    }
    #[test]
    fn search() {
        let list = get_list();
        VALS.iter().enumerate().for_each(|(i, &val)| {
            let actual = list.search(val);
            assert!(actual.is_some(), "SEARCH IS NOT WORKING! SHOULD HAVE FOUND VALUE {val} IN DATA STRUCTURE {list:?}");
            // Already made sure that actual has a value, if actual was `None` then the above would panic
            let actual_loc = actual.unwrap();
            assert_eq!(actual_loc , i, "WRONG LOCATION, SHOULD HAVA FOUND VALUE {val} IN LOCATION {i} FOUND IN {actual_loc}");
        });
    }
    #[test]
    fn test_len() {
        let list = get_list();
        let expected = VALS.len();
        let actual = list.len();
        assert_eq!(
            actual, expected,
            "EXPECTED {expected} DOES NOT MATCH LIST's ACTUAL {actual} LENGTH"
        );
    }
    #[test]
    fn test_empty() {
        let list = List::new();
        assert!(
            list.is_empty(),
            "NEW LIST SHOULD BE RETURN WHEN CALLING IS_EMPTY. LIST {list:?}"
        );
    }
    #[test]
    fn test_pop() {
        let mut list = get_list();
        let actual = list.pop();
        let expected = VALS.last();
        if let Some(&expected) = expected {
            if let Some(actual) = actual {
                let actual_val = actual.value;
                assert_eq!(
                    actual.value, expected,
                    "SHOULD HAVE GOT VALUE {expected} GOT {actual_val}. NODE {actual:?}"
                );
            } else {
                assert!(
                    actual.is_some(),
                    "NO VALUE GIVEN BY POP(), SHOULD HAVE VALUE {expected}"
                );
            }
        } else {
            unreachable!()
        }
    }
}
