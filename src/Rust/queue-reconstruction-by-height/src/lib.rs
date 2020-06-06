// https://leetcode.com/problems/queue-reconstruction-by-height/

struct Solution {}

const HEIGHT: usize = 0;
const AHEAD: usize = 1;

impl Solution {
    pub fn reconstruct_queue(mut people: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut ret: Vec<Vec<i32>> = Vec::with_capacity(people.len());

        &people.sort_unstable_by(|a, b| {
            a[HEIGHT].cmp(&b[HEIGHT]).then(a[AHEAD].cmp(&b[AHEAD]).reverse())
        });

        for person in people.iter().rev() {
            ret.insert(person[AHEAD] as usize, person.to_vec());
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example1() {
        assert_eq!(
            vec![
                vec![5, 0],
                vec![7, 0],
                vec![5, 2],
                vec![6, 1],
                vec![4, 4],
                vec![7, 1],
            ],
            Solution::reconstruct_queue(
                vec![
                    vec![7, 0],
                    vec![4, 4],
                    vec![7, 1],
                    vec![5, 0],
                    vec![6, 1],
                    vec![5, 2],
                ],
            ),
        );
    }

    #[test]
    fn test_example2() {
        assert_eq!(
            vec![
                vec![3, 0],
                vec![1, 1],
                vec![7, 0],
                vec![5, 1],
                vec![2, 3],
            ],
            Solution::reconstruct_queue(
                vec![
                    vec![2, 3],
                    vec![3, 0],
                    vec![5, 1],
                    vec![7, 0],
                    vec![1, 1],
                ],
            ),
        );
    }
}
