#!/usr/bin/env python3
# coding: utf-8

import unittest

from backtracking import Solution


class TestBacktracking(unittest.TestCase):
    def test_letterCombinations(self):
        s = Solution()

        self.assertEqual(
            ['ad', 'ae', 'af', 'bd', 'be', 'bf', 'cd', 'ce', 'cf'],
            s.letterCombinations('23'),
        )


if __name__ == '__main__':
    unittest.main(verbosity=2)
