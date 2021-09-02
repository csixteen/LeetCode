#!/usr/bin/env python3
# coding: utf-8

import unittest

from greedy import Solution


class TestGreedy(unittest.TestCase):
    def setUp(self):
        self.s = Solution()

    def test_maximumUnits(self):
        self.assertEqual(8, self.s.maximumUnits([[1,3],[2,2],[3,1]], 4))
        self.assertEqual(91, self.s.maximumUnits([[5,10],[2,5],[4,7],[3,9]], 10))


if __name__ == "__main__":
    unittest.main(verbosity=2)
