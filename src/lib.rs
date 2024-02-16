/// Add the first and second parameters
///
/// # Examples
///
/// ```
/// let left_arg = 5;
/// let right_arg = 10;
/// let answer = rust_mylib::add(left_arg, right_arg);
///
/// assert_eq!(15, answer);
/// ```
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[macro_export]
macro_rules! my_macro {
    () => {
        println!("This is my macro!");
    };
}