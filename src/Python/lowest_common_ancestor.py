#!/usr/bin/env python3
# coding: utf-8
# https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-tree/

import unittest


class Solution:
    def lowestCommonAncestor(self, root, p, q):
        if not root:
            return None

        if root.val == p.val or root.val == q.val:
            return root

        left = self.lowestCommonAncestor(root.left, p, q)
        right = self.lowestCommonAncestor(root.right, p, q)

        if left and right:
            return root
        elif left:
            return left
        else:
            return right


class TestSolution(unittest.TestCase):
    def setUp(self):
        self.test_cases = []

    def test_lowestCommonAncestor(self):
        s = Solution()
        for i, e in self.test_cases:
            self.assertEqual(s.lowestCommonAncestor(*i), e)


if __name__ == '__main__':
    unittest.main(verbosity=2)

