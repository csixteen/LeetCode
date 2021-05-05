// https://leetcode.com/problems/shortest-distance-to-a-character/

struct Solution;

impl Solution {
    pub fn shortest_to_char(s: String, c: char) -> Vec<i32> {
        let len = s.len();
        let mut dist = vec![len as i32; len];
        let mut prev = -(len as i32);

        for (i, ch) in s.char_indices() {
            if ch == c {
                prev = i as i32;
            }
            dist[i] = (i as i32 - prev) as i32;
        }

        prev = len as i32;
        for (i, ch) in s.char_indices().rev() {
            if ch == c {
                prev = i as i32;
            }
            dist[i] = dist[i].min(prev - (i as i32));
        }

        dist
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn test_shortest_to_char() {
        assert_eq!(
            vec![3, 2, 1, 0, 1, 0, 0, 1, 2, 2, 1, 0],
            Solution::shortest_to_char("loveleetcode".to_string(), 'e')
        );

        assert_eq!(
            vec![3, 2, 1, 0],
            Solution::shortest_to_char("aaab".to_string(), 'b')
        );
    }
}
