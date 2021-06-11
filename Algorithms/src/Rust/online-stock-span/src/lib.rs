// https://leetcode.com/problems/online-stock-span/

#![allow(dead_code)]

#[derive(Default)]
struct StockSpanner {
    stack: Vec<(i32, i32)>,
}

impl StockSpanner {

    fn new() -> Self {
        Default::default()
    }
    
    fn next(&mut self, price: i32) -> i32 {
        let mut total = 1;

        while !self.stack.is_empty() &&
            self.stack.iter().last().unwrap().0 <= price {
            total += self.stack.pop().unwrap().1;
        }

        self.stack.push((price, total));

        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stock_spanner() {
        let mut s = StockSpanner::new();

        assert_eq!(1, s.next(100));
        assert_eq!(1, s.next(80));
        assert_eq!(1, s.next(60));
        assert_eq!(2, s.next(70));
        assert_eq!(1, s.next(60));
        assert_eq!(4, s.next(75));
        assert_eq!(6, s.next(85));
    }

    #[test]
    fn test_stock_spanner_ascending() {
        let mut s = StockSpanner::new();

        assert_eq!(1, s.next(1));
        assert_eq!(2, s.next(2));
        assert_eq!(3, s.next(3));
        assert_eq!(4, s.next(4));
        assert_eq!(5, s.next(5));
        assert_eq!(6, s.next(6));
    }

    #[test]
    fn test_stock_spanner_descending() {
        let mut s = StockSpanner::new();

        assert_eq!(1, s.next(6));
        assert_eq!(1, s.next(5));
        assert_eq!(1, s.next(4));
        assert_eq!(1, s.next(3));
        assert_eq!(1, s.next(2));
        assert_eq!(1, s.next(1));
    }
}
