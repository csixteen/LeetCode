#!/usr/bin/env python3
# coding: utf-8
import unittest


class Solution:
    def combinations2(self, n, k):
        def combs(lst, k):
            if k == 0:
                return [[]]
            elif len(lst) == 0:
                return []
            else:
                with_h = list(map(lambda x: [lst[0]] + x, combs(lst[1:], k - 1)))
                without_h = combs(lst[1:], k)
                return with_h + without_h

        return combs(list(range(1, n+1)), k)

    def combinations(self, n, k):
        def DFS(A, B, curr, pos, acc, tmp):
            if pos > B:
                acc.add(tuple(tmp))
            else:
                for i in range(curr + 1, A - (B - pos) + 2):
                    DFS(A, B, i, pos+1, acc, tmp + [curr])

        res = set()
        for i in range(1, n - k + 2):
            DFS(n, k, i, 1, res, [])

        return [list(x) for x in res]


class TestSolution(unittest.TestCase):
    def setUp(self):
        self.test_cases = [
            ((4, 2), [[1, 2], [1, 3], [1, 4], [2, 3], [2, 4], [3, 4]])
        ]

    def test_combinations(self):
        s = Solution()
        for i, e in self.test_cases:
            self.assertEqual(sorted(s.combinations(*i)), e)


if __name__ == '__main__':
    unittest.main(verbosity=2)

