#![allow(
    clippy::missing_docs_in_private_items,
    clippy::missing_errors_doc,
    clippy::new_without_default,
    clippy::missing_inline_in_public_items
)]

use core::convert;

#[derive(Debug, Default)]
pub struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    #[must_use]
    pub fn new(value: T) -> Self {
        Self { value, next: None }
    }
    pub fn set_next(&mut self, node: Node<T>) {
        self.next = Some(Box::from(node));
    }
    pub fn get_value(&self) -> &T {
        &self.value
    }
    pub fn get_mut(&mut self) -> &mut T {
        &mut self.value
    }
    pub fn get_next(&mut self) -> Option<&mut Box<Node<T>>> {
        self.next.as_mut()
    }
}

#[derive(Debug)]
pub struct List<T> {
    next: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    #[must_use]
    pub fn new() -> Self {
        Self { next: None }
    }
    pub fn from<A: Into<Vec<T>>>(values: A) -> Self {
        fn inner<T>(vec: Vec<T>) -> List<T> {
            let mut list = List::new();
            vec.into_iter().for_each(|t| list.push(t));
            list
        }
        inner(values.into())
    }
    pub fn push(&mut self, value: T) {
        self.insert(value, 0);
    }
    pub fn insert(&mut self, value: T, at: usize) {
        if at == 0 {
            let dangling_wrp_node = self.split_off_raw(0);
            let mut node = Box::from(Node::new(value));
            node.next = dangling_wrp_node;
            self.next = Some(node);
        } else {
            // TODO: Need to handle cases where at > len
            let loc = at - 1;
            let wrp_prev_node = self.get_mut_ref(loc);
            if let Some(prev_node) = wrp_prev_node {
                let mut node = Box::from(Node::new(value));
                node.next = core::mem::take(&mut prev_node.next);
                prev_node.next = Some(node);
            }
        }
    }
    #[must_use]
    pub fn search(&self, value: T) -> Option<usize>
    where
        T: PartialOrd,
    {
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
    pub fn split_off(&mut self, at: usize) -> Option<List<T>> {
        // TODO: Decide if split should return an empty list when Original list is empty or
        //  where_at > len or return None
        self.next
            .is_some()
            .then(|| {
                let node = self.split_off_raw(at);
                node.map(|node| {
                    let mut list = List::new();
                    list.next = Some(node);
                    list
                })
            })?
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
        while let Some(unwrapped_item) = wrapped_item {
            len += 1;
            wrapped_item = unwrapped_item.next.as_ref();
        }
        len
    }
    pub fn pop(&mut self) -> Option<T> {
        self.next.take().map(|node| {
            self.next = node.next;
            node.value
        })
    }
    pub fn rpop(&mut self) -> Option<T> {
        self.next.is_some().then(|| {
                let loc = self.len().checked_sub(1)?;
                let unwrapped = self.split_off_raw(loc);
                unwrapped.map(|node| node.value)
            })?
    }
    fn split_off_raw(&mut self, at: usize) -> Option<Box<Node<T>>> {
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
    fn get_mut_ref(&mut self, at: usize) -> Option<&mut Box<Node<T>>> {
        let mut loc = 0;
        // let mut wrp_node = self.next.as_ref();
        if let Some(mut node) = self.next.as_mut() {
            loop {
                if loc == at {
                    return Some(node);
                } else if let Some(ref mut new_node) = node.next {
                    loc += 1;
                    node = new_node;
                } else {
                    break;
                }
            }
        }
        None
    }
    fn pop_node(&mut self) -> Option<Box<Node<T>>> {
        let wrapped = core::mem::take(&mut self.next);
        wrapped.is_some().then(|| {
            let mut node = wrapped.unwrap();
            self.next = core::mem::take(&mut node.next);
            node
        })
    }
    pub fn peek(&self) -> Option<&T> {
        self.next.as_ref().map(|t| &t.value)
    }
    pub fn mut_peek(&mut self) -> Option<&mut T> {
        self.next.as_mut().map(|t| &mut t.value)
    }
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter {
            next: self.next.as_ref().map(convert::AsRef::as_ref),
            index: 0,
        }
    }
    pub fn iter_mut<'a>(&'a mut self) -> IterMut<'a, T> {
        IterMut {
            next: self.next.as_mut().map(convert::AsMut::as_mut),
            index: 0,
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while let Some(mut _node) = self.pop_node() {}
    }
}

pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

pub struct Iter<'a, T> {
    // keep a reference here, so that iter is O(n) as opposed to O(n^2)
    next: Option<&'a Node<T>>,
    index: usize,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().and_then(|node| {
            self.next = node.next.as_deref();
            self.index = self.index.checked_add(1)?;
            Some(&node.value)
        })
    }
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
    index: usize,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().and_then(|node| {
            // get a mutable boxed reference to node next
            // unwrap the node from the box, and grab a mutable reference while we are at it.
            // ie Option<&mut Box<Node<T>>> => Option<&mut Node<T>>
            self.next = node.next.as_deref_mut();
            self.index.checked_add(1)?;
            Some(&mut node.value)
        })
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::use_debug)]

    use super::*;
    use core::ops::Range;
    use rand::Rng;

    const VALS: [i32; 9] = [0, 1, 2, 3, 4, 5, 6, 7, 8];

    fn get_list() -> List<i32> {
        let mut list = List::new();
        VALS.iter().for_each(|&val| list.push(val));
        list
    }

    #[test]
    fn create_list() {
        let _list: List<i32> = List::new();
    }
    #[test]
    fn push_list() {
        let mut list = List::new();
        // let len = VALS.len();
        for (i, &value) in VALS.iter().enumerate() {
            println!(
                "INSERTING {} OF {} VALUES, VAL: {value} ",
                i + 1,
                VALS.len()
            );
            list.push(value);
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
        VALS.iter().rev().enumerate().for_each(|(i, &val)| {
            let actual = list.search(val);
            assert!(actual.is_some(), "SEARCH IS NOT WORKING! SHOULD HAVE FOUND VALUE {val} IN DATA STRUCTURE {list:?}");
            // Already made sure that actual has a value, if actual was `None` then the above would panic
            let actual_loc = actual.unwrap();
            assert_eq!(actual_loc , i, "WRONG LOCATION, SHOULD HAVA FOUND VALUE {val} IN LOCATION {i} FOUND IN {actual_loc}, STRUCTURE: {list:?}");
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
        let list: List<i32> = List::new();
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
                assert_eq!(
                    actual, expected,
                    "SHOULD HAVE GOT VALUE {expected} GOT {actual}."
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
    #[test]
    fn insert() {
        let mut rng = rand::thread_rng();
        let mut list = get_list();
        let vec = (0..40)
            .into_iter()
            .map(|_| rng.gen::<i32>())
            .collect::<Vec<i32>>();
        for (i, &item) in vec.iter().enumerate() {
            let end = list.len() + 1;
            let loc = rng.gen_range::<usize, Range<usize>>(0..end);
            println!("INSERTING {} of {} {item} AT {loc}", i + 1, vec.len());
            list.insert(item, loc);
            let actual_loc = match list.search(item) {
                Some(loc) => loc,
                None => unreachable!(),
            };
            assert_eq!(loc, actual_loc, "INSERTION LOCATION IS WRONG LIST {list:?}");
        }
        let expected_len = VALS.len() + vec.len();
        let actual_len = list.len();
        println!("{list:?}");
        assert_eq!(
            expected_len, actual_len,
            "EXPECTED LENGTH {expected_len} DOES NOT EQUAL LIST LENGTH {actual_len}"
        );
    }
    #[test]
    fn lrwetmll_initial_tests() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
    #[test]
    fn peek() {
        let list = get_list();
        let peeked = list.peek();
        assert_eq!(
            Some(&8),
            peeked,
            "PEEK DOES NOT WORK GOT VALUE: {peeked:?} WITH LIST {list:?}"
        );
    }
    #[test]
    fn mut_peek() {
        let mut list = get_list();
        {
            // Make sure that we get a mut borrow that only lasts until reassignment
            let peek = list.mut_peek();
            assert_eq!(Some(&mut 8), peek, "INITIAL PEEK DOES NOT WORK!");
            if let Some(num) = peek {
                *num = 9;
            }
        }
        let peek = list.mut_peek();
        let mut expected = 9;
        let expected = Some(&mut expected);
        assert_eq!(
            expected, peek,
            "MUTABLE PEEK DID NOT WORK, VALUE: {peek:?} EXPECTED {expected:?}"
        );
    }
    #[test]
    fn into_iter() {
        let list = get_list();
        let mut list_owned_iter = list.into_iter();
        // Cheat and force use a mutable reference instead of the actual variable. This means that
        //  iterator is not consumed in the loop chain.
        for (item, &value) in (&mut list_owned_iter).zip(VALS.iter().rev()) {
            println!("VAL: {item} ");
            assert_eq!(item, value);
        }
        // Make sure the iterator is empty, since there should not be any values in the
        assert_eq!(list_owned_iter.next(), None);
    }
    #[test]
    fn iter() {
        let list = get_list();
        for _ in 0..2 {
            let mut iter = list.iter();
            for ((i, &value), &check) in (&mut iter).enumerate().zip(VALS.iter().rev()) {
                println!("INDEX: {i} VAL: {value}");
                assert_eq!(
                    value, check,
                    "VALUE: {value} at INDEX: {i} DOES NOT EQUAL EXPECTED {check}"
                );
            }
            // This should also be none.
            assert_eq!(iter.next(), None);
        }
    }
    #[test]
    fn iter_mut() {
        for iteration in 1..10 {
            let mut list = get_list();
            let vals_iter = VALS.iter().rev();
            let iter_mut = list.iter_mut();
            for (i, value) in iter_mut.enumerate() {
                let old_value = *value;
                *value = old_value + iteration;
                println!("INDEX: {i} ORIGINAL VAL: {old_value} NEW VALUE {}", *value);
            }
            // then use a read only iterator to check the values.
            for ((i, &value), &check) in list.iter().enumerate().zip(vals_iter) {
                let expected = check + iteration;
                assert_eq!(
                    value, expected,
                    "ITERATION {i} VALUE {value} DOES NOT EQUAL EXPECTED {expected}"
                );
            }
        }
    }
}
