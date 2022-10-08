//! # Add_One
//!
//! `add_one` is a collection of utilities to make performing certain
//! calculations more convenient.


/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = add_one::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add_one(2);
        assert_eq!(result, 3);
    }
}


// Re-exports
pub use self::module::inner_enum;
pub use self::module::inner_function;

pub mod module {
    pub fn inner_function {
        // -snip-
    }
    pub enum inner_enum {
        A,
        B
    }
}