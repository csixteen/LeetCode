// https://leetcode.com/problems/insert-delete-getrandom-o1/

#![allow(dead_code)]

use rand::{thread_rng, Rng};

use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Default)]
struct RandomizedSet {
    map: RefCell<HashMap<i32, usize>>,
    vec: RefCell<Vec<i32>>,
}

impl RandomizedSet {
    fn new() -> Self {
        Default::default()
    }

    fn insert(&self, val: i32) -> bool {
        let contains = self.map.borrow().contains_key(&val);

        match contains {
            true => false,
            false => {
                self.map.borrow_mut().insert(val, self.vec.borrow().len());
                self.vec.borrow_mut().push(val);

                true
            },
        }
    }

    fn remove(&self, val: i32) -> bool {
        let contains = self.map.borrow().contains_key(&val);

        match contains {
            false => false,
            true => {
                let len = self.vec.borrow().len();
                let index = *self.map.borrow().get(&val).unwrap();
                let last_elem = self.vec.borrow()[len - 1];
                self.vec.borrow_mut()[index] = last_elem;
                self.map.borrow_mut().insert(last_elem, index);
                self.vec.borrow_mut().remove(len-1);
                self.map.borrow_mut().remove(&val);

                true
            }
        }
    }

    fn get_random(&self) -> i32 {
        let mut rng = thread_rng();
        let hi = self.vec.borrow().len();

        self.vec.borrow()[rng.gen_range(0, hi)]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_randomized_set() {
        let obj = RandomizedSet::new();

        assert!(obj.insert(1));
        assert!(obj.remove(1));
        assert!(!obj.remove(2));
        assert!(obj.insert(2));
        assert!(!obj.insert(2));
        assert_eq!(2, obj.get_random());
    }

    #[test]
    fn test_randomized_set2() {
        let obj = RandomizedSet::new();

        assert!(obj.insert(0));
        assert!(obj.insert(1));
        assert!(obj.remove(0));
        assert!(obj.insert(2));
        assert!(obj.remove(1));
        assert_eq!(2, obj.get_random());
    }

    #[test]
    fn test_randomized_set3() {
        let obj = RandomizedSet::new();

        assert!(!obj.remove(0));
        assert!(!obj.remove(0));
        assert!(obj.insert(0));
        assert_eq!(0, obj.get_random());
        assert!(obj.remove(0));
        assert!(obj.insert(0));
    }
}
