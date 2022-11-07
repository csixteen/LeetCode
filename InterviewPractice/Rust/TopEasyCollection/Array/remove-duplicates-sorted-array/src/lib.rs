struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let l = nums.len();
        let (mut slow, mut fast) = (0, 1);
        while slow < l && fast < l {
            if nums[slow] != nums[fast] {
                slow += 1;
                nums[slow] = nums[fast];
            }
            fast += 1;
        }
        slow as i32 + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example1() {
        let mut input = vec![1, 1, 2];
        assert_eq!(2, Solution::remove_duplicates(&mut input));
        assert_eq!(1, input[0]);
        assert_eq!(2, input[1]);
    }

    #[test]
    fn example2() {
        let mut input = vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        assert_eq!(5, Solution::remove_duplicates(&mut input));
        for i in 0..5 {
            assert_eq!(i as i32, input[i]);
        }
    }

    #[test]
    fn example3() {
        let mut input = vec![1, 1, 1, 1, 1];
        assert_eq!(1, Solution::remove_duplicates(&mut input));
    }

    #[test]
    fn example4() {
        let mut input = vec![0];
        assert_eq!(1, Solution::remove_duplicates(&mut input));
    }
}
