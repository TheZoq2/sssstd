pub mod snek;
#[macro_use]
pub mod macros;


#[cfg(test)]
mod tests {
    use crate::snek::Snek;
    #[macro_use]
    use crate::macros;

    #[test]
    fn it_works() {
        assert_eq!(Snek::<i32>::new(), Vec::new());
    }

    #[test]
    fn macro_test() {
        assert_eq!(snek!(1,2,3 as i32), vec!(1,2,3));
    }
}
