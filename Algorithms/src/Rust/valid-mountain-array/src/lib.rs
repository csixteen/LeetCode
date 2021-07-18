// https://leetcode.com/problems/valid-mountain-array/

struct Solution;

impl Solution {
    pub fn valid_mountain_array(arr: Vec<i32>) -> bool {
        let mut i = 1;
        while i < arr.len() && arr[i] > arr[i-1] {
            i += 1;
        }

        if i == 1 || i == arr.len() { return false }
        
        while i < arr.len() && arr[i] < arr[i-1] {
            i += 1;
        }

        i == arr.len()
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        assert!(!Solution::valid_mountain_array(vec![2, 1]));
    }

    #[test]
    fn example2() {
        assert!(!Solution::valid_mountain_array(vec![3, 5, 5]));
    }

    #[test]
    fn example3() {
        assert!(Solution::valid_mountain_array(vec![0, 3, 2, 1]));
    }
}
