#!/usr/bin/env python3
# coding: utf-8
# https://leetcode.com/problems/maximum-depth-of-binary-tree
import unittest


class TreeNode(object):
    def __init__(self, x, l=None, r=None):
        self.val = x
        self.left = l
        self.right = r


class Solution(object):
    def maxDepth(self, root):
        """
        :type root: TreeNode
        :rtype: int
        """
        return self._maxDepthRecursive(root, 0)

    def _maxDepthRecursive(self, node, acc):
        if node is None:
            return acc
        return max(
            self._maxDepthRecursive(node.left, acc+1),
            self._maxDepthRecursive(node.right, acc+1)
        )


class TestSolution(unittest.TestCase):
    def test_maxDepth(self):
        s = Solution()
        t = TreeNode(
            3,
            TreeNode(9),
            TreeNode(
                20,
                TreeNode(15),
                TreeNode(7)
            )
        )
        self.assertEqual(s.maxDepth(t), 3)

        t = TreeNode(3)
        self.assertEqual(s.maxDepth(t), 1)

        t = TreeNode(
            3,
            TreeNode(
                9,
                TreeNode(1),
                TreeNode(2)
            ),
            TreeNode(
                15,
                TreeNode(19),
                TreeNode(24)
            )
        )
        self.assertEqual(s.maxDepth(t), 3)

        t = TreeNode(
            3,
            TreeNode(
                4,
                TreeNode(
                    5,
                    TreeNode(6)
                )
            )
        )
        self.assertEqual(s.maxDepth(t), 4)


if __name__ == '__main__':
    unittest.main()

