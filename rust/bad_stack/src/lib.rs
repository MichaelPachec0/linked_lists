pub enum List {
    Elem(i32, Option<Box<List>>),
    Head(Box<List>)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {

    }
}
