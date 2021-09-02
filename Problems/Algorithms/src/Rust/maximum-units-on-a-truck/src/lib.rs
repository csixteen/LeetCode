// https://leetcode.com/problems/maximum-units-on-a-truck/

struct Solution;

impl Solution {
    pub fn maximum_units(mut box_types: Vec<Vec<i32>>, truck_size: i32) -> i32 {
        box_types.sort_by(|a, b| b[1].partial_cmp(&a[1]).unwrap());
        let mut truck_size = truck_size;
        let mut total = 0;

        for _box in box_types.iter() {
            let d = truck_size.min(_box[0]);
            total += d * _box[1];
            truck_size -= d;
            if truck_size == 0 {
                break;
            }
        }

        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_maximum_units() {
        assert_eq!(
            Solution::maximum_units(vec![vec![1, 3], vec![2, 2], vec![3, 1]], 4),
            8
        );
        assert_eq!(
            Solution::maximum_units(vec![vec![5, 10], vec![2, 5], vec![4, 7], vec![3, 9]], 10),
            91
        );
    }
}
