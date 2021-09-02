// https://leetcode.com/problems/permutations-ii/

struct Solution;

impl Solution {
    pub fn permute_unique(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        fn go(xs: Vec<i32>) -> Vec<Vec<i32>> {
            if xs.is_empty() { vec![vec![]] }
            else {
                xs.iter().enumerate().fold((None, Vec::new()), |(p, mut acc), (i, &n)| {
                    match p {
                        Some(j) if j == n => (p, acc),
                        _ => {
                            let (left, right) = xs.split_at(i);
                            acc.append(
                                &mut go([left, &right[1..]].concat())
                                    .into_iter()
                                    .map(|perm| [perm, vec![n]].concat())
                                    .collect()
                            );
                            (Some(n), acc)
                        },
                    }
                }).1
            }
        }

        nums.sort_unstable();
        go(nums)
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        assert_eq!(
            vec![vec![1, 1, 2], vec![1, 2, 1], vec![2, 1, 1]],
            Solution::permute_unique(vec![1, 1, 2]),
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            vec![
                vec![1, 2, 3],
                vec![1, 3, 2],
                vec![2, 1, 3],
                vec![2, 3, 1],
                vec![3, 1, 2],
                vec![3, 2, 1],
            ],
            Solution::permute_unique(vec![1, 2, 3]),
        );
    }
}
