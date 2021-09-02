// https://leetcode.com/problems/assign-cookies/

use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct Solution;

impl Solution {
    pub fn find_content_children(g: Vec<i32>, s: Vec<i32>) -> i32 {
        fn go(
            mut xs: BinaryHeap<Reverse<&i32>>,
            mut ys: BinaryHeap<Reverse<&i32>>,
            x: Option<Reverse<&i32>>,
            y: Option<Reverse<&i32>>,
            acc: i32
        ) -> i32 {
            match (x, y) {
                (None, _) => acc,
                (_, None) => acc,
                (Some(Reverse(x1)), Some(Reverse(y1))) => {
                    if x1 <= y1 {
                        let new_x = xs.pop();
                        let new_y = ys.pop();
                        go(xs, ys, new_x, new_y, acc+1)
                    } else {
                        let new_y = ys.pop();
                        go(xs, ys, x, new_y, acc)
                    }
                },
            }
        }

        let mut g_sorted = BinaryHeap::new();
        let mut s_sorted = BinaryHeap::new();

        g.iter().for_each(|i| { g_sorted.push(Reverse(i)); });
        s.iter().for_each(|i| { s_sorted.push(Reverse(i)); });

        let x = g_sorted.pop();
        let y = s_sorted.pop();

        go(g_sorted, s_sorted, x, y, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        assert_eq!(1, Solution::find_content_children(vec![1, 2, 3], vec![1, 1]));
    }

    #[test]
    fn example2() {
        assert_eq!(2, Solution::find_content_children(vec![1, 2], vec![1, 2, 3]));
    }
}
