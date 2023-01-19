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
        let len = 3;
        let mut loops = 0;
        let list = List::Head(Some(Box::from(List::Elem(32, Some(Box::from(List::Elem(5, Some(Box::from(List::Elem(8, None))))))))));
        // always assume that the first element after creation of our Linked list will be the Head.
        let mut ptr = if let List::Head(ref elem) = list { elem.as_ref() } else {unreachable!()};
        'looping: loop {
            ptr = if let Some(wrapped_ptr) = ptr {
                if let List::Elem(val, next) = wrapped_ptr.as_ref() {
                    println!("NODE WITH VALUE {val}");
                    loops += 1;
                    next.as_ref()
                } else{
                    // we should never be here, but break just in case;
                    break 'looping;
                }
            } else {
                println!("We are at the end of the line!");
                break 'looping;
            };
        }
        assert_eq!(len, loops, "DID NOT LOOP THROUGH ALL THE VALUES");
    }
}
