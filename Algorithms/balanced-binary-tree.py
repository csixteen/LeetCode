#!/usr/bin/env python3
# coding: utf-8
# https://leetcode.com/problems/balanced-binary-tree/
import unittest


class TreeNode(object):
    def __init__(self, x, l=None, r=None):
        self.val = x
        self.left = l
        self.right = r


class Solution(object):
    def treeHeight(self, root):
        """
        :type root: TreeNode
        :rtype: int
        """
        return self._treeHeightRecursive(root, 0)

    def _treeHeightRecursive(self, node, acc):
        """
        :type node: TreeNode
        :type acc: int
        :rtype: int
        """
        if node is None:
            return acc
        return max(
            self._treeHeightRecursive(node.left, acc+1),
            self._treeHeightRecursive(node.right, acc+1)
        )

    def isBalanced(self, root):
        """
        :type root: TreeNode
        :rtype: bool
        """
        if not root:
            return True
        return abs(
            self.treeHeight(root.left) - self.treeHeight(root.right)
        ) <= 1 and \
            self.isBalanced(root.left) and \
            self.isBalanced(root.right)


class TestSolution(unittest.TestCase):
    def test_isBalanced(self):
        s = Solution()
        t = TreeNode(
            3,
            TreeNode(9),
            TreeNode(20, TreeNode(15), TreeNode(7))
        )
        self.assertTrue(s.isBalanced(t))

        t = TreeNode(1)
        self.assertTrue(s.isBalanced(t))

        t = TreeNode(
            1,
            TreeNode(
                2,
                TreeNode(3, TreeNode(4), TreeNode(4)),
                TreeNode(3)
            ),
            TreeNode(2)
        )
        self.assertFalse(s.isBalanced(t))

        t = TreeNode(1, r=TreeNode(2, r=TreeNode(3)))
        self.assertFalse(s.isBalanced(t))

        t = TreeNode(
            1,
            TreeNode(2, l=TreeNode(3, l=TreeNode(4))),
            TreeNode(2, r=TreeNode(3, r=TreeNode(4)))
        )
        self.assertFalse(s.isBalanced(t))


if __name__ == '__main__':
    unittest.main()

