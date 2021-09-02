// https://leetcode.com/problems/can-place-flowers/

struct Solution;

impl Solution {
    pub fn can_place_flowers(mut flowerbed: Vec<i32>, mut n: i32) -> bool {
        let mut i = 0;
        let len = flowerbed.len();

        for i in 0..len {
            if n == 0 { break; }

            if flowerbed[i] == 1 ||
                (i > 0 && flowerbed[i-1] == 1) ||
                (i < len - 1 && flowerbed[i+1] == 1) { 
                continue; 
            }

            flowerbed[i] = 1;
            n -= 1;
        }

        n == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_place_flowers() {
        assert!(Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 1));
        assert!(!Solution::can_place_flowers(vec![1, 0, 0, 0, 1], 2));
    }
}
