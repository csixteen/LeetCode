// https://leetcode.com/problems/simplify-path/

struct Solution;

impl Solution {
    pub fn simplify_path(path: String) -> String {
        "/".to_string() + &path.split('/').fold(Vec::new(), |mut acc, part| {
            if part == ".." { acc.pop(); }
            else if part != "." && part != "" { acc.push(part); }
            acc
        }).join("/")
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example1() {
        assert_eq!(
            "/home".to_string(),
            Solution::simplify_path("/home/".to_string()),
        );
    }

    #[test]
    fn example2() {
        assert_eq!(
            "/".to_string(),
            Solution::simplify_path("/../".to_string()),
        );
    }

    #[test]
    fn example3() {
        assert_eq!(
            "/home/foo".to_string(),
            Solution::simplify_path("/home//foo/".to_string()),
        );
    }

    #[test]
    fn example4() {
        assert_eq!(
            "/c".to_string(),
            Solution::simplify_path("/a/./b/../../c/".to_string()),
        );
    }
}
