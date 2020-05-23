#!/usr/bin/env python3
# coding: utf-8
import unittest


class Solution:
    def radix_sort(self, arr):
        """
        Time complexity: O(n)
        Space complexity: O(n)
        """
        position = 1
        max_number = max(arr)

        while position <= max_number:
            queue_list = [list() for _ in range(10)]

            for num in arr:
                digit_number = num // position % 10
                queue_list[digit_number].append(num)

            index = 0
            for numbers in queue_list:
                for num in numbers:
                    arr[index] = num
                    index += 1

            position *= 10

        return arr

    def maximum_gap(self, arr):
        if len(arr) < 2:
            return 0

        self.radix_sort(arr)
        m = 0
        for i in range(len(arr) - 1):
            m = max(m, arr[i + 1] - arr[i])

        return m


class TestSolution(unittest.TestCase):
    def setUp(self):
        self.test_cases = [
            ([3, 6, 9, 1], 3),
            ([10], 0)
        ]

    def test_maximum_gap(self):
        s = Solution()
        for i, e in self.test_cases:
            self.assertEqual(s.maximum_gap(i), e)


if __name__ == '__main__':
    unittest.main(verbosity=2)

