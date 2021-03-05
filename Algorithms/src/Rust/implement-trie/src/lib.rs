// https://leetcode.com/problems/implement-trie-prefix-tree/

#![allow(dead_code)]

use std::collections::HashMap;


#[derive(Default)]
struct Trie {
    end: bool,
    children: HashMap<char, Trie>,
}

impl Trie {
    fn new() -> Self {
        Default::default()
    }

    fn insert(&mut self, word: String) {
        let mut curr = self;

        for c in word.chars() {
            curr = curr.children.entry(c).or_insert(Trie::new());
        }

        curr.end = true;
    }

    fn search(&self, word: String) -> bool {
        self.find(word).map_or(false, |t| t.end)
    }

    fn starts_with(&self, prefix: String) -> bool {
        self.find(prefix).is_some()
    }

    fn find(&self, word: String) -> Option<&Trie> {
        let mut curr = self;

        for c in word.chars() {
            curr = curr.children.get(&c)?;
        }

        Some(curr)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trie() {
        let mut t = Trie::new();

        t.insert(String::from("apple"));
        assert!(t.search(String::from("apple")));
        assert!(!t.search(String::from("app")));
        assert!(t.starts_with(String::from("app")));

        t.insert(String::from("app"));
        assert!(t.search(String::from("app")));
        assert!(t.search(String::from("apple")));
        assert!(t.starts_with(String::from("app")));

        t.insert(String::from("application"));
        assert!(!t.search(String::from("applications")));
        assert!(t.search(String::from("app")));
        assert!(t.search(String::from("apple")));
        assert!(t.search(String::from("application")));
        assert!(!t.search(String::from("applica")));
        assert!(t.starts_with(String::from("applica")));

        t.insert(String::from("trie"));
        assert!(t.search(String::from("trie")));
        assert!(t.search(String::from("app")));
        assert!(t.search(String::from("apple")));
    }
}
