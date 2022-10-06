// To sequentially run tests: cargo test -- --test-threads=1

pub fn add_two(num: usize) -> usize {
    num + 2
}

pub fn less_than_10(num: i32) -> bool {
    num < 10
}

pub fn greeting(name: &str) -> String {
    println!("Print Statement"); // can be shown using cargo test -- --show-output
    format!("Hello {}!", name)
}

pub fn painc_fn() {
    println!("Print Statement");
    panic!("This function will always panic");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add_two(2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_less_than() {
        assert!(less_than_10(5));
        assert!(!less_than_10(15));
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }

    #[test]
    #[should_panic]
    fn should_panic() {
        painc_fn();
    }

    #[test]
    fn it_works2() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    #[ignore]
    fn ignored_test() {
        // will be ignored in normal cargo test
        // can run only ignored ones with cargo test -- --ignored
    }
}
