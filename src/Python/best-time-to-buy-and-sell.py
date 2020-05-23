#!/usr/bin/env python3
# coding: utf-8
# https://leetcode.com/problems/best-time-to-buy-and-sell-stock/
import unittest


class Solution(object):
    def maxProfit(self, prices):
        """
        :type prices: list[int]
        :rtyle: int
        """
        max_profit = 0
        min_price = float('inf')
        for i in prices:
            if i < min_price:
                min_price = i
            elif i - min_price > max_profit:
                max_profit = i - min_price
        return max_profit


class TestSolution(unittest.TestCase):
    def setUp(self):
        self.input_expected = [
            ([7, 1, 5, 3, 6, 4], 5),
            ([7, 6, 4, 3, 1], 0)
        ]

    def test_maxProfit(self):
        s = Solution()
        for i, e in self.input_expected:
            self.assertEqual(s.maxProfit(i), e)


if __name__ == '__main__':
    unittest.main()

