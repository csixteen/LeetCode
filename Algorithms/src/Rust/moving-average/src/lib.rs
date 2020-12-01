// https://leetcode.com/problems/moving-average-from-data-stream/

use std::collections::VecDeque;

struct MovingAverage {
    size: usize,
    sum: f64,
    window: VecDeque<i32>,
}

impl MovingAverage {
    fn new(size: i32) -> Self {
        MovingAverage{
            size: size as usize,
            sum: 0_f64,
            window: VecDeque::new(),
        }
    }

    fn next(&mut self, val: i32) -> f64 {
        self.sum += val as f64;
        self.window.push_front(val);
        if self.window.len() > self.size {
            self.sum -= self.window.pop_back().unwrap() as f64;
        }

        self.sum / self.window.len() as f64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_moving_average() {
        let mut obj = MovingAverage::new(3);

        assert_eq!(1.0, obj.next(1));
        assert_eq!((1.0+10.0)/2_f64, obj.next(10));
        assert_eq!((1.0+10.0+3.0)/3_f64, obj.next(3));
        assert_eq!((10.0+3.0+5.0)/3_f64, obj.next(5));
    }
}
