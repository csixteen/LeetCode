#!/usr/bin/env python3
# coding: utf-8
import unittest


class Solution:
    def combinationSum(self, candidates, target):
        def DFS(A, t, res, acc, current_sum, start):
            if start >= len(A) or current_sum > t:
                return
            elif current_sum == target:
                res.append(acc)
            
            i = start
            while i < len(A):
                DFS(A, t, res, acc + [A[i]], current_sum + A[i], i)
                i += 1
                

        res = []
        DFS(candidates, target, res, [], 0, 0)

        return res


class TestSolution(unittest.TestCase):
    def setUp(self):
        self.test_cases = [
            (([2, 3, 6, 7], 7), [[7], [2, 2, 3]]),
            (([2, 3, 5], 8), [[2, 2, 2, 2], [2, 3, 3], [3, 5]])
        ]

    def test_combinationSum(self):
        s = Solution()
        for i, e in self.test_cases:
            self.assertEqual(sorted(s.combinationSum(*i)), sorted(e))


if __name__ == '__main__':
    unittest.main(verbosity=2)

