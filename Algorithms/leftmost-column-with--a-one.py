#!/usr/bin/env python3
# coding: utf-8
# https://leetcode.com/explore/challenge/card/30-day-leetcoding-challenge/530/week-3/3306/

from typing import List


class BinaryMatrix:
    """ This is an interactive challenge, which means that
    LeetCode provides the implementation of BinaryMatrix
    and we should only use the available API.
    """
    def get(self, x: int, y: int) -> int:
        return 0

    def dimensions(self) -> List[int]:
        return []


class Solution:
    def leftMostColumnWithOne(self, binaryMatrix: BinaryMatrix) -> int:
        def bin_search(row: int, start: int, end: int) -> int:
            lo, hi = start, end

            while lo <= hi:
                mid = lo + (hi - lo) // 2

                val = binaryMatrix.get(row, mid)
                if val == 1:
                    if mid == start or binaryMatrix.get(row, mid - 1) == 0:
                        return mid
                    else:
                        hi = mid - 1
                else:
                    lo = mid + 1

            return -1

        rows, cols = binaryMatrix.dimensions()

        first = cols + 1
        lo, hi = 0, cols - 1

        for row in range(rows):
            col = bin_search(row, lo, hi)
            if col != -1 and col < first:
                first = col
                hi = col

        return first if first != cols + 1 else -1

