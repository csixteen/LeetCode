// https://leetcode.com/problems/design-hashmap/

struct MyHashMap {
    arr: [i32; 1_000_001]
}

impl MyHashMap {
    fn new() -> Self {
        MyHashMap{ arr: [-1; 1_000_001] }
    }

    fn put(&mut self, key: i32, value: i32) {
        self.arr[key as usize] = value;
    }

    fn get(&self, key: i32) -> i32 {
        self.arr[key as usize]
    }

    fn remove(&mut self, key: i32) {
        self.arr[key as usize] = -1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_hashmap() {
        let mut obj = MyHashMap::new();

        obj.put(1, 1); // The map is now [[1,1]]
        obj.put(2, 2); // The map is now [[1,1], [2,2]]

        assert_eq!(1, obj.get(1));
        assert_eq!(-1, obj.get(3));

        obj.put(2, 1);

        assert_eq!(1, obj.get(2));

        obj.remove(2);

        assert_eq!(-1, obj.get(2));
    }
}
