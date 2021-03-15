#!/usr/bin/env python3

import unittest

from strings import Solution


class TestStrings(unittest.TestCase):
    def setUp(self):
        self.s = Solution()

    def test_countBinarySubstrings(self):
        self.assertEqual(6, self.s.countBinarySubstrings("00110011"))
        self.assertEqual(4, self.s.countBinarySubstrings("10101"))


if __name__ == "__main__":
    unittest.main(verbosity=2)
