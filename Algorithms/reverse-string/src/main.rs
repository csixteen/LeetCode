struct Solution {}

impl Solution {
    pub fn reverse_string(s: &mut Vec<char>) {
        if s.len() == 0 {
            return ();
        }
        
        let mut i = 0;
        let mut j = s.len() - 1;
        
        while i < j {
            s.swap(i, j);
            i += 1;
            j -= 1;
        }
    }
}
