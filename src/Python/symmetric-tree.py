#!/usr/bin/env python3
# coding: utf-8
# https://leetcode.com/problems/symmetric-tree
import unittest


class TreeNode(object):
    def __init__(self, x, l=None, r=None):
        self.val = x
        self.left = l
        self.right = r


class Solution(object):
    def isSymmetric(self, root):
        """
        :type root: TreeNode
        :rtype: bool
        """
        if root is None:
            return True

        return self._isSymmetricRecursive(root.left, root.right)

    def _isSymmetricRecursive(self, left, right):
        if left is None or right is None:
            return left is None and right is None
        return left.val == right.val and \
            self._isSymmetricRecursive(left.left, right.right) and \
            self._isSymmetricRecursive(left.right, right.left)


class TestSolution(unittest.TestCase):
    def test_isSymmetric(self):
        s = Solution()
        t = TreeNode(
            1,
            TreeNode(2, TreeNode(3), TreeNode(4)),
            TreeNode(2, TreeNode(4), TreeNode(3))
        )
        self.assertTrue(s.isSymmetric(t))

        t = TreeNode(
            1,
            TreeNode(2, r=TreeNode(3)),
            TreeNode(2, r=TreeNode(3))
        )
        self.assertFalse(s.isSymmetric(t))

        t = TreeNode(
            1,
            TreeNode(2, l=TreeNode(3)),
            TreeNode(2, r=TreeNode(3))
        )
        self.assertTrue(s.isSymmetric(t))

        t = TreeNode(1)
        self.assertTrue(s.isSymmetric(t))

        t = TreeNode(
            2,
            TreeNode(
                3,
                TreeNode(4),
                TreeNode(
                    5,
                    TreeNode(8),
                    TreeNode(9)
                )
            ),
            TreeNode(
                3,
                TreeNode(
                    5
                ),
                TreeNode(
                    4,
                    TreeNode(9),
                    TreeNode(8)
                )
            )
        )
        self.assertFalse(s.isSymmetric(t))


if __name__ == '__main__':
    unittest.main()

