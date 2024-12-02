pub struct Solution;

impl Solution {
    pub fn valid_word_abbreviation(word: String, abbr: String) -> bool {
        Solution::abbr(word.as_bytes(), abbr.as_bytes())
    }

    fn abbr(word: &[u8], ab: &[u8]) -> bool {
        if word.is_empty() && ab.is_empty() {
            return true;
        }

        if word.is_empty() || ab.is_empty() {
            return false;
        }

        if (ab[0] as char).is_ascii_alphabetic() {
            return (word[0] == ab[0]) && Solution::abbr(&word[1..], &ab[1..]);
        }

        let n = ab
            .iter()
            .take_while(|&c| (*c as char).is_numeric())
            .map(|c| *c as char)
            .collect::<String>();

        // Need to perform this silly check because of the problem's silly requirements
        if n.starts_with('0') {
            return false;
        }

        let len = n.len();
        let n = n.parse::<usize>().expect("Couldn't parse usize");

        if n > word.len() {
            return false;
        }

        Solution::abbr(&word[n..], &ab[len..])
    }
}

#[cfg(test)]
mod test {
    use super::*;

    macro_rules! test_abbr {
        ( $( $word:expr => $abbr:expr ),* ) => {{
             $(
                 assert!(Solution::valid_word_abbreviation($word.into(), $abbr.into()));
             )*
        }};
    }

    macro_rules! test_not_abbr {
        ( $( $word:expr => $abbr:expr ),* ) => {{
             $(
                 assert!(!Solution::valid_word_abbreviation($word.into(), $abbr.into()));
             )*
        }};
    }

    #[test]
    fn test_abbr() {
        test_abbr!(
            "internationalization" => "i12iz4n",
            "substitution" => "s10n",
            "substitution" => "sub4u4",
            "substitution" => "12",
            "substitution" => "su3i1u2on",
            "substitution" => "substitution"
        );
    }

    #[test]
    fn test_not_abbr() {
        test_not_abbr!("apple" => "a2e", "substitution" => "s55n", "substitution" => "s010n", "substitution" => "s0ubstitution");
    }
}
