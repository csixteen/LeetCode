// https://leetcode.com/problems/flood-fill/

#![allow(dead_code)]

struct Solution {}

impl Solution {
    pub fn flood_fill(image: Vec<Vec<i32>>, sr: i32, sc: i32, new_color: i32) -> Vec<Vec<i32>> {
        fn dfs(img: &mut Vec<Vec<i32>>, i: i32, j: i32, ic: i32, nc: i32) {
            if i >= 0 && 
                j >= 0 && 
                i < img.len() as i32 && 
                j < img[0].len() as i32 &&
                img[i as usize][j as usize] == ic {

                img[i as usize][j as usize] = nc;

                dfs(img, i-1, j, ic, nc);
                dfs(img, i+1, j, ic, nc);
                dfs(img, i, j-1, ic, nc);
                dfs(img, i, j+1, ic, nc);
            }
        }

        let mut ret = image.clone();
        let initial_color = ret[sr as usize][sc as usize];

        if initial_color != new_color {
            dfs(&mut ret, sr, sc, initial_color, new_color);
        }

        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        image: Vec<Vec<i32>>,
        sr: i32,
        sc: i32,
        new_color: i32,
        expected: Vec<Vec<i32>>,
    }

    #[test]
    fn test_flood_fill() {
        let test_cases = [
            TestCase {
                image: vec![vec![1, 1, 1], vec![1, 1, 0], vec![1, 0, 1]],
                sr: 1,
                sc: 1,
                new_color: 2,
                expected: vec![vec![2, 2, 2], vec![2, 2, 0], vec![2, 0, 1]],
            },
            TestCase {
                image: vec![vec![0, 0, 0], vec![0, 1, 1]],
                sr: 1,
                sc: 1,
                new_color: 1,
                expected: vec![vec![0, 0, 0], vec![0, 1, 1]],
            },
        ];

        for case in test_cases.iter() {
            assert_eq!(
                case.expected,
                Solution::flood_fill(
                    case.image.clone(),
                    case.sr,
                    case.sc,
                    case.new_color,
                ),
            );
        }
    }
}
