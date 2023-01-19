#![allow(clippy::missing_docs_in_private_items, clippy::exhaustive_enums)]
pub enum List {
    Elem(i32, Option<Box<List>>),
    Head(Option<Box<List>>)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let list = List::Head(Some(Box::from(List::Elem(32, Some(Box::from(List::Elem(5, Some(Box::from(List::Elem(8, None))))))))));
        // always assume that the first element after creation of our Linked list will be the Head.
        let mut ptr = if let List::Head(ref elem) = list { elem } else {unreachable!()};
        loop {
            ptr = match ptr {
                &Some(ref wrapped_ptr) => {
                    let x  = &**wrapped_ptr;
                    match x {
                        // clippy tells me that this is the wrong type
                        List::Elem(val, next) => {
                            println!("NODE WITH VALUE {val}");
                            next
                        },
                        List::Head(_) => unreachable!()
                    }
                },
                None => {
                    println!("We are at the end of the line!");
                    return;
                }
            };
        }
    }
}
