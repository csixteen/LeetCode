#!/usr/bin/env python3
# coding: utf-8
# https://leetcode.com/problems/best-time-to-buy-and-sell-stock-ii/
import unittest


class Solution(object):
    def maxProfit2(self, prices):
        max_profit = 0
        return sum([prices[i]-prices[i-1] for i in range(1, len(prices)) \
                    if prices[i] > prices[i-1]])

    def maxProfit(self, prices):
        """
        :type prices: list[int]
        :rtype: int
        """
        max_profit = 0
        min_price = float('inf')
        for i, price in enumerate(prices):
            if price < min_price:
                min_price = price
            elif price - min_price > max_profit:
                max_profit = price - min_price
                m = self.maxProfit(prices[i:])
                max_profit = max(max_profit, m + max_profit)

        return max_profit


class TestSolution(unittest.TestCase):
    def setUp(self):
        self.input_expected = [
            ([7,1,5,3,6,4], 7),
            ([1,2,3,4,5], 4),
            ([7,6,4,3,1], 0),
            ([2,1,4,5,2,9,7], 11),
            ([1, 10, 11, 20, 21, 30], 29)
        ]

    def test_maxProfit(self):
        s = Solution()
        for i, e in self.input_expected:
            self.assertEqual(s.maxProfit(i), e)


if __name__ == '__main__':
    unittest.main()

