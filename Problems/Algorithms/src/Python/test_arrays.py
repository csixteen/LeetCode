#!/usr/bin/env python3

import unittest

from arrays import Solution


class TestStrings(unittest.TestCase):
    def setUp(self):
        self.s = Solution()

    def test_sortedSquares(self):
        self.assertEqual([0, 1, 9, 16, 100], self.s.sortedSquares([-4, -1, 0, 3, 10]))
        self.assertEqual([4, 9, 9, 49, 121], self.s.sortedSquares([-7, -3, 2, 3, 11]))
        self.assertEqual([1, 4, 100, 121], self.s.sortedSquares([1, 2, 10, 11]))
        self.assertEqual([1], self.s.sortedSquares([1]))
        self.assertEqual([1, 4, 9, 16, 25], self.s.sortedSquares([-5, -4, -3, -2, -1]))

    def test_findDisappearedNumbers(self):
        self.assertEqual([5, 6], self.s.findDisappearedNumbers([4, 3, 2, 7, 8, 2, 3, 1]))

    def test_maximumProduct(self):
        self.assertEqual(6, self.s.maximumProduct([1, 2, 3]))
        self.assertEqual(24, self.s.maximumProduct([1, 2, 3, 4]))
        self.assertEqual(-6, self.s.maximumProduct([-1, -2, -3]))


if __name__ == "__main__":
    unittest.main(verbosity=2)
