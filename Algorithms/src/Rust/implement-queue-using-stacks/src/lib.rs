// https://leetcode.com/problems/implement-queue-using-stacks/

#[derive(Default)]
struct MyQueue {
    vec1: Vec<i32>,
    vec2: Vec<i32>
}

impl MyQueue {
    fn new() -> Self {
        Default::default()
    }

    fn push(&mut self, x: i32) {
        self.vec1.push(x);
    }

    fn pop(&mut self) -> i32 {
        // The problem statement guarantees that all the calls to
        // pop will be valid, so we don't need to do safety checks here.
        if self.vec2.is_empty() {
            self.vec2 = self.vec1.drain(..).rev().collect();
        }
        self.vec2.pop().unwrap()
    }

    fn peek(&mut self) -> i32 {
        if self.vec2.is_empty() {
            self.vec2 = self.vec1.drain(..).rev().collect();
        }
        let l = self.vec2.len();
        self.vec2[l-1]
    }

    fn empty(&self) -> bool {
        self.vec1.is_empty() && self.vec2.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_queue() {
        let mut obj = MyQueue::new();

        obj.push(1);
        assert!(!obj.empty());
        assert_eq!(1, obj.peek());
        obj.push(2);
        assert_eq!(1, obj.peek());
        assert_eq!(1, obj.pop());
        assert_eq!(2, obj.peek());
        assert!(!obj.empty());
        assert_eq!(2, obj.pop());
        assert!(obj.empty());
    }
}
