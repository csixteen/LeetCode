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

    def __repr__(self) -> str:
        return f"({self.key}, {self.value})"


class LRUCache:
    def __init__(self, capacity: int):
        self.capacity = capacity
        self.size = 0
        self.cache: Dict[int, ListItem] = {}
        self.head = None
        self.tail = None

    def _add_item(self, item: ListItem):
        """ We always add the item to the head of the list """
        item._prev = self.head
        if self.head is not None:
            self.head._next = item
        self.head = item

        if self.tail is None:
            self.tail = item

    def _delete_item(self, item: ListItem):
        if item._prev is None:
            # item is tail
            self.tail = item._next
        else:
            item._prev._next = item._next

        if item._next is None:
            # item is head
            self.head = item._prev
        else:
            item._next._prev = item._prev

    def _move_to_head(self, item: ListItem):
        self._delete_item(item)
        self._add_item(item)

    def _pop_tail(self) -> ListItem:
        curr = self.tail
        self._delete_item(curr)

        return curr

    def get(self, key: int) -> int:
        item = self.cache.get(key, None)

        if item is None:
            return -1

        self._move_to_head(item)

        return item.value

    def put(self, key: int, value: int):
        item = self.cache.get(key, None)

        if item is None:
            item = ListItem(key, value)

            self.cache[key] = item
            self._add_item(item)
            self.size += 1

            if self.size > self.capacity:
                tail = self._pop_tail()
                del self.cache[tail.key]
                self.size -= 1
        else:
            item.value = value
            self._move_to_head(item)


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
