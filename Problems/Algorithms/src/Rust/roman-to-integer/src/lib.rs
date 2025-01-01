pub struct Solution;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Symbol {
    I = 1,
    V = 5,
    X = 10,
    L = 50,
    C = 100,
    D = 500,
    M = 1000,
}

impl Symbol {
    #[inline]
    pub fn is_next_order(&self) -> bool {
        matches!(*self, Symbol::I | Symbol::X | Symbol::C)
    }

    #[inline]
    pub fn next_order(&self) -> Option<Self> {
        match *self {
            Symbol::I => None,
            Symbol::V | Symbol::X => Some(Symbol::I),
            Symbol::L | Symbol::C => Some(Symbol::X),
            Symbol::D | Symbol::M => Some(Symbol::C),
        }
    }
}

impl From<char> for Symbol {
    #[inline]
    fn from(value: char) -> Self {
        match value {
            'I' => Self::I,
            'V' => Self::V,
            'X' => Self::X,
            'L' => Self::L,
            'C' => Self::C,
            'D' => Self::D,
            'M' => Self::M,
            _ => unreachable!(),
        }
    }
}

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut res = 0;
        let symbols: Vec<_> = s.chars().map(Symbol::from).collect();
        let mut i = 0_usize;

        while i < symbols.len() {
            if i < symbols.len() - 1
                && symbols[i].is_next_order()
                && Some(symbols[i]) == symbols[i + 1].next_order()
            {
                res += (symbols[i + 1] as i32) - (symbols[i] as i32);
                i += 2;
            } else {
                res += symbols[i] as i32;
                i += 1;
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case1() {
        assert_eq!(3, Solution::roman_to_int("III".to_owned()));
    }

    #[test]
    fn test_case2() {
        assert_eq!(58, Solution::roman_to_int("LVIII".to_owned()));
    }

    #[test]
    fn test_case3() {
        assert_eq!(1994, Solution::roman_to_int("MCMXCIV".to_owned()));
    }
}
