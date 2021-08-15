// https://leetcode.com/problems/next-greater-element-i/

use std::collections::{HashMap, VecDeque};

struct Solution;

impl Solution {
    pub fn next_greater_element(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        fn unload_stack(acc: &mut HashMap<i32, i32>, stack: &mut VecDeque<i32>) {
            if !stack.is_empty() {
                acc.insert(stack.pop_front().unwrap(), -1);
                unload_stack(acc, stack);
            }
        }

        fn insert(e: i32, acc: &mut HashMap<i32, i32>, stack: &mut VecDeque<i32>) {
            if stack.is_empty() || e < stack[0] {
                stack.push_front(e);
            } else {
                acc.insert(stack.pop_front().unwrap(), e);
                insert(e, acc, stack);
            }
        }

        fn go(xs: &Vec<i32>, i: usize, acc: &mut HashMap<i32, i32>, stack: &mut VecDeque<i32>) {
            if i < xs.len() {
                insert(xs[i], acc, stack);
                go(xs, i+1, acc, stack);
            }
        }

        let mut mapping = HashMap::new();
        let mut stack = VecDeque::new();
        go(&nums2, 0, &mut mapping, &mut stack);
        unload_stack(&mut mapping, &mut stack);

        nums1.iter().map(|i| mapping.get(i).unwrap()).cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        assert_eq!(
            vec![-1, 3, -1],
            Solution::next_greater_element(vec![4, 1, 2], vec![1, 3, 4, 2]),
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            vec![3, -1],
            Solution::next_greater_element(vec![2, 4], vec![1, 2, 3, 4]),
        );
    }
}
