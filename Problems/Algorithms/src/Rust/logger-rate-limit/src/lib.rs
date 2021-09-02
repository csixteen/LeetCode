// https://leetcode.com/problems/logger-rate-limiter/

#![allow(dead_code)]

use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Default)]
struct Logger {
    bucket: RefCell<HashMap<String,i32>>,
}

impl Logger {
    fn new() -> Self {
        Default::default()
    }

    fn should_print_message(&self, timestamp: i32, message: String) -> bool {
        if timestamp - self.bucket.borrow().get(&message).unwrap_or(&-10) < 10 {
            false
        } else {
            self.bucket.borrow_mut().insert(message, timestamp);
            true
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        let obj = Logger::new();

        assert!(obj.should_print_message(0, String::from("a")));
        assert!(obj.should_print_message(1, String::from("b")));
        assert!(obj.should_print_message(0, String::from("c")));
        assert!(!obj.should_print_message(1, String::from("a")));
        assert!(obj.should_print_message(11, String::from("b")));
        assert!(!obj.should_print_message(9, String::from("c")));
    }
}
