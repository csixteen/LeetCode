#!/usr/bin/env python3
# coding: utf-8
import unittest


class Solution:
    def gray(self, N):
        def gray_rec(X, c, acc):
            if c == X:
                return acc
            else:
                return gray_rec(X, c + 1, list(map(lambda e: '0' + e, acc)) + list(map(lambda e: '1' + e, reversed(acc))))

        if N == 0:
            return [0]

        return list(map(lambda x: int(x, 2), gray_rec(N, 1, ['0', '1'])))


class TestSolution(unittest.TestCase):
    def setUp(self):
        self.test_cases = [
            (0, [0]),
            (1, [0, 1]),
            (2, [0, 1, 3, 2])
        ]

    def test_gray(self):
        s = Solution()
        for i, e in self.test_cases:
            self.assertEqual(s.gray(i), e)


if __name__ == '__main__':
    unittest.main(verbosity=2)

