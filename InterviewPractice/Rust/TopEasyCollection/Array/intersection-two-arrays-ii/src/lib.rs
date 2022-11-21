use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let occur1: HashMap<i32, usize> =
            nums1.iter().fold(HashMap::new(), |mut acc, e| {
                acc.entry(*e).and_modify(|c| *c += 1).or_insert(1);
                acc
            });
        let occur2: HashMap<i32, usize> =
            nums2.iter().fold(HashMap::new(), |mut acc, e| {
                acc.entry(*e).and_modify(|c| *c += 1).or_insert(1);
                acc
            });

        occur1.iter().fold(Vec::new(), |mut acc, (k, v)| {
            match occur2.get(k) {
                Some(v2) => {
                    for i in 0..*v.min(v2) {
                        acc.push(*k);
                    }
                    acc
                },
                None     => acc,
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        assert_eq!(vec![2, 2], Solution::intersect(vec![1, 2, 2, 1], vec![2, 2]));
    }

    #[test]
    fn example2() {
        assert_eq!(vec![4, 9], Solution::intersect(vec![4, 9, 5], vec![9, 4, 9, 8, 4]));
    }
}
