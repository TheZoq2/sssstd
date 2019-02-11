
#[macro_export]
macro_rules! snek {
    ($($a:expr),*) => {
        vec!($($a),*)
    }
}
