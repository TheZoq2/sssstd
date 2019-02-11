pub mod snek;


#[cfg(test)]
mod tests {
    use crate::snek::Snek;

    #[test]
    fn it_works() {
        assert_eq!(Snek::<i32>::new(), Vec::new());
    }
}
