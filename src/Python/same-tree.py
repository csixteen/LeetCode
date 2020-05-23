#!/usr/bin/env python3
# coding: utf-8
# https://leetcode.com/problems/same-tree
import unittest


class TreeNode(object):
    def __init__(self, x):
        self.val = x
        self.left = None
        self.right = None


class Solution(object):
    def isSameTree(self, p, q):
        """
        :type p: TreeNode
        :type q: TreeNode
        :rtype: bool
        """
        if p is None and q is None:
            return True
        elif p is not None and q is not None and p.val == q.val:
            return self.isSameTree(p.left, q.left) and \
                self.isSameTree(p.right, q.right)
        else:
            return False


class TestSolution(unittest.TestCase):
    def test_isSameTree(self):
        s = Solution()

        t1 = TreeNode(1)
        t1.left = TreeNode(2)
        t1.right = TreeNode(3)
        t2 = TreeNode(1)
        t2.left = TreeNode(2)
        t2.right = TreeNode(3)
        self.assertTrue(s.isSameTree(t1, t2))

        t1 = TreeNode(1)
        t1.left = TreeNode(2)
        t2 = TreeNode(1)
        t2.right = TreeNode(2)
        self.assertFalse(s.isSameTree(t1, t2))

        t1 = TreeNode(1)
        t1.left = TreeNode(2)
        t1.right = TreeNode(1)
        t2 = TreeNode(1)
        t2.left = TreeNode(1)
        t2.right = TreeNode(2)
        self.assertFalse(s.isSameTree(t1, t2))


if __name__ == '__main__':
    unittest.main()

