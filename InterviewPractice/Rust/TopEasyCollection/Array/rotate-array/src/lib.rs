struct Solution;

impl Solution {
    pub fn rotate2(nums: &mut Vec<i32>, k: i32) {
        let k = (k as usize) % nums.len();
        &nums.reverse();
        &nums[..k].reverse();
        &nums[k..].reverse();
    }

    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let l = nums.len();
        let k = (k as usize) % l;

        // Reverse
        (0..l / 2).for_each(|i| nums.swap(i, l-i-1));

        // Reverse first part
        (0..k / 2).for_each(|i| nums.swap(i, k-i-1));

        // Reverse second bit
        (0..(l - k) / 2).for_each(|i| nums.swap(k + i, l-i-1));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let mut v = vec![1, 2, 3, 4, 5, 6, 7];
        Solution::rotate(&mut v, 3);
        assert_eq!(v, vec![5, 6, 7, 1, 2, 3, 4]);
    }

    #[test]
    fn example2() {
        let mut v = vec![-1, -100, 3, 99];
        Solution::rotate(&mut v, 2);
        assert_eq!(v, vec![3, 99, -1, -100]);
    }

    #[test]
    fn example3() {
        let mut v = vec![1];
        Solution::rotate(&mut v, 3);
        assert_eq!(v, vec![1]);
    }
}
