#!/usr/bin/env python3
# coding: utf-8
import unittest


class Solution(object):
    def addBinary(self, a, b):
        """
        :type a: str
        :type b: str
        :rtype: str
        """
        tmp, carry = 0, 0
        len_a, len_b = len(a), len(b)
        int_a = list(map(int, a))
        int_b = list(map(int, b))
        ret = []
        while not (len_a <= 0 and len_b <= 0):
            if carry > 0:
                tmp = 1
                carry -= 1
            if len_a > 0:
                carry += (int_a[len_a-1] + tmp) // 2
                tmp = (int_a[len_a-1] + tmp) % 2
                len_a -= 1
            if len_b > 0:
                carry += (int_b[len_b-1] + tmp) // 2
                tmp = (int_b[len_b-1] + tmp) % 2
                len_b -= 1

            ret.insert(0, tmp)
            tmp = 0

        if carry > 0:
            ret.insert(0, 1)

        return ''.join(map(str, ret))


class TestSolution(unittest.TestCase):
    def test_addBinary(self):
        s = Solution()
        self.assertEqual(s.addBinary('11', '1'), '100')
        self.assertEqual(s.addBinary('11', '11'), '110')
        self.assertEqual(s.addBinary('111', '11'), '1010')
        self.assertEqual(s.addBinary('111', '111'), '1110')
        self.assertEqual(s.addBinary('1', '1'), '10')
        self.assertEqual(s.addBinary('101', '1'), '110')
        self.assertEqual(s.addBinary('0', '0'), '0')


if __name__ == '__main__':
    unittest.main()

