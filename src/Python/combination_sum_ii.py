#!/usr/bin/env python3
# coding: utf-8
import unittest


class Solution:
    def combinationSumII(self, A, t):
        def DFS(arr, target, res, acc, current_sum, start):
            if current_sum == target:
                res.append(acc)
            elif current_sum > target:
                return
            else:
                i = start
                while i < len(arr):
                    DFS(arr, target, res, acc + [arr[i]], current_sum + arr[i], i + 1)
                    while i < len(arr) - 1 and arr[i] == arr[i + 1]:
                        i += 1
                    i += 1

        res = []
        DFS(sorted(A), t, res, [], 0, 0)

        return res


class TestSolution(unittest.TestCase):
    def setUp(self):
        self.test_cases = [
            (
                ([10,1,2,7,6,1,5], 8),
                [
                    [1, 7],
                    [1, 2, 5],
                    [2, 6],
                    [1, 1, 6]
                ]
            ),
            (
                ([2, 5, 2, 1, 2], 5),
                [
                    [1, 2, 2],
                    [5]
                ]
            )
        ]

    def test_combinationSumII(self):
        s = Solution()
        for i, e in self.test_cases:
            self.assertEqual(s.combinationSumII(*i), sorted(e))


if __name__ == '__main__':
    unittest.main(verbosity=2)

