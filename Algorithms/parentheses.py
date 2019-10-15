#!/usr/bin/env python3
# coding: utf-8
import unittest


class Solution:
    def generate(self, N):
        def DFS(acc, s, left, right):
            if left == 0 and right == 0:
                acc.append(s)
            else:
                if left > 0:
                    DFS(acc, s + '(', left-1, right)
                if right > left:
                    DFS(acc, s + ')', left, right-1)

        res = []
        DFS(res, '', N, N)
        return res


class TestSolution(unittest.TestCase):
    def setUp(self):
        self.test_cases = [
            (1, ['()']),
            (2, ['(())', '()()']),
            (3, ['((()))', '(()())', '(())()', '()(())', '()()()'])
        ]

    def test_generate(self):
        s = Solution()
        for i, e in self.test_cases:
            self.assertEqual(s.generate(i), e)


if __name__ == '__main__':
    unittest.main(verbosity=2)

