use std::cmp::*;

pub struct Solution;

impl Solution {
    pub fn decrypt_runsum(code: Vec<i32>, k: i32) -> Vec<i32> {
        if k == 0 {
            return vec![0; code.len()];
        }

        let (mut start, mut end) = if k > 0 {
            (1, k as usize)
        } else {
            (code.len() - k.unsigned_abs() as usize, code.len() - 1)
        };
        let mut sum = (start..=end).fold(0, |acc, i| acc + code[i]);
        let mut res = Vec::with_capacity(code.len());

        for _i in &code {
            res.push(sum);
            sum -= code[start % code.len()];
            sum += code[(end + 1) % code.len()];
            start += 1;
            end += 1;
        }

        res
    }

    pub fn decrypt(code: Vec<i32>, k: i32) -> Vec<i32> {
        match k.cmp(&0) {
            Ordering::Less => code
                .iter()
                .enumerate()
                .map(|(i, _)| {
                    code.iter()
                        .rev()
                        .cycle()
                        .skip(code.len() - i)
                        .take(k.unsigned_abs() as usize)
                        .sum::<i32>()
                })
                .collect(),
            Ordering::Equal => vec![0; code.len()],
            Ordering::Greater => code
                .iter()
                .enumerate()
                .map(|(i, _)| {
                    code.iter()
                        .cycle()
                        .skip(i + 1)
                        .take(k as usize)
                        .sum::<i32>()
                })
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_k_greater_than_0() {
        assert_eq!(vec![12, 10, 16, 13], Solution::decrypt(vec![5, 7, 1, 4], 3));
        assert_eq!(
            vec![12, 10, 16, 13],
            Solution::decrypt_runsum(vec![5, 7, 1, 4], 3)
        );
    }

    #[test]
    fn test_k_equals_0() {
        assert_eq!(vec![0, 0, 0, 0], Solution::decrypt(vec![5, 7, 1, 4], 0));
        assert_eq!(
            vec![0, 0, 0, 0],
            Solution::decrypt_runsum(vec![5, 7, 1, 4], 0)
        );
    }

    #[test]
    fn test_k_less_than_0() {
        assert_eq!(vec![12, 5, 6, 13], Solution::decrypt(vec![2, 4, 9, 3], -2));
        assert_eq!(
            vec![12, 5, 6, 13],
            Solution::decrypt_runsum(vec![2, 4, 9, 3], -2)
        );
    }
}
