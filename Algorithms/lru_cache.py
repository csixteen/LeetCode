#!/usr/bin/env python3
# coding: utf-8
# https://leetcode.com/problems/lru-cache/

import unittest


class ListItem:
    def __init__(self, k: int, v: int):
        self.key = k
        self.value = v
        self._prev = None
        self._next = None


class LRUCache:
    def __init__(self, capacity: int):
        self.capacity = capacity
        self.cache: Dict[int, ListItem] = {}
        self.head = None
        self.tail = None

    def _discard_item(self, item: ListItem):
        if self.tail is not None and self.tail.key == item.key:
            self.tail = item._next

        if self.head is not None and self.head.key == item.key:
            self.head = item._prev

        if item._prev is not None:
            item._prev._next = item._next

        if item._next is not None:
            item._next._prev = item._prev

    def get(self, key: int) -> int:
        if key not in self.cache:
            return -1

        # We're in the head, no need to do anything else
        if self.head.key == key:
            return self.head.value

        curr = self.cache[key]
        ret = curr.value

        # We're in the tail
        if self.tail.key == key:
            self.tail = self.tail._next
            self.tail._prev = None

        if curr._prev is not None:
            curr._prev._next, curr._next._prev = curr._next, curr._prev

        curr._prev = self.head
        curr._next = None
        self.head._next = curr
        self.head = curr

        return ret

    def put(self, key: int, value: int):
        if key in self.cache:
            self._discard_item(self.cache[key])
            del self.cache[key]
            self.capacity += 1

        item = ListItem(key, value)
        self.cache[key] = item

        if self.head is not None:
            self.head._next = item
        item._prev = self.head
        self.head = item

        if self.tail is None:
            self.tail = item

        if self.capacity == 0:
            key_to_delete = self.tail.key
            self.tail = self.tail._next
            self.tail._prev = None

            del self.cache[key_to_delete]
        else:
            self.capacity -= 1


class TestLRUCache(unittest.TestCase):
    def test_lru_cache(self):
        lru = LRUCache(2)

        lru.put(1, 1)
        lru.put(2, 2)
        self.assertEqual(1, lru.get(1))

        lru.put(3, 3)  # evicts 2
        self.assertEqual(-1, lru.get(2))

        lru.put(4, 4)  # evicts 1
        self.assertEqual(-1, lru.get(1))
        self.assertEqual(3, lru.get(3))
        self.assertEqual(4, lru.get(4))

    def test_lru_cache_size1(self):
        lru = LRUCache(1)

        lru.put(1, 1)
        self.assertEqual(1, lru.get(1))

        lru.put(2, 2)
        self.assertEqual(-1, lru.get(1))
        self.assertEqual(2, lru.get(2))

    def test_lru_cache_failed_test(self):
        lru = LRUCache(2)

        lru.put(2, 1)
        lru.put(2, 2)
        self.assertEqual(2, lru.get(2))

        lru.put(1, 1)
        lru.put(4, 1)
        self.assertEqual(-1, lru.get(2))

    def test_lru_cache_wrong_answer(self):
        lru = LRUCache(2)

        lru.put(2, 1)
        lru.put(1, 1)
        lru.put(2, 3)
        lru.put(4, 1)

        self.assertEqual(-1, lru.get(1))
        self.assertEqual(3, lru.get(2))


if __name__ == "__main__":
    unittest.main(verbosity=2)
