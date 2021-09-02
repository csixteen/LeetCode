// https://leetcode.com/problems/unique-binary-search-trees/submissions/

struct Solution;

impl Solution {
    pub fn num_trees(n: i32) -> i32 {
        fn go(n: usize, cache: &mut Vec<usize>) -> usize {
            if cache[n] != 0 { cache[n] }
            else {
                let res = (1..=n).map(|i| go(i-1, cache) * go(n-i, cache)).sum::<usize>();
                cache[n] = res;
                res
            }
        }

        let mut cache = vec![0_usize; n as usize + 1];
        cache[0] = 1;
        cache[1] = 1;
        go(n as usize, &mut cache) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        assert_eq!(5, Solution::num_trees(3));
    }

    #[test]
    fn example2() {
        assert_eq!(2, Solution::num_trees(2));
    }

    #[test]
    fn example3() {
        assert_eq!(1, Solution::num_trees(1));
    }
}
