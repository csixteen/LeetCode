// https://leetcode.com/problems/count-primes/

impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        if n < 2 {
            return 0;
        }
        
        let n = n as usize;
        let mut primes = vec![true; n];

        primes[0] = false;
        primes[1] = false;

        let mut i = 2;

        while i * i <= n {
            if primes[i] {
                for j in (i*2..n).step_by(i) {
                    primes[j] = false;
                }
            }

            i += 1;
        }

        (0..primes.len())
            .filter(|&i| primes[i as usize])
            .count() as i32
    }
}
