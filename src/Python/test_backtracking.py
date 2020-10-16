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

    def test_minPathSum(self):
        s = Solution()

        self.assertEqual(
            7,
            s.minPathSum([[1, 3, 1], [1, 5, 1], [4, 2, 1]]),
        )


if __name__ == '__main__':
    unittest.main(verbosity=2)
