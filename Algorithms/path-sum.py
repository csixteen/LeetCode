#!/usr/bin/env python3
# coding: utf-8
# https://leetcode.com/problems/path-sum/
import unittest


class TreeNode(object):
    def __init__(self, x, l=None, r=None):
        self.val = x
        self.left = l
        self.right = r


class Solution(object):
    def hasPathSum(self, root, s):
        """
        :type root: TreeNode
        :type s: int
        :rtype: bool
        """
        if not root:
            return False
        else:
            sums = []
            if not root.left and root.right:
                self._hasPathSumRecursive(root.right, root.val, sums)
            elif root.left and not root.right:
                self._hasPathSumRecursive(root.left, root.val, sums)
            else:
                self._hasPathSumRecursive(root, 0, sums)
            return s in sums

    def _hasPathSumRecursive(self, node, acc, sums):
        acc += node.val
        if not node.left and not node.right:
            sums.append(acc)
        else:
            if node.left:
                self._hasPathSumRecursive(node.left, acc, sums)
            if node.right:
                self._hasPathSumRecursive(node.right, acc, sums)


class TestSolution(unittest.TestCase):
    def test_hasPathSum(self):
        s = Solution()
        t = TreeNode(
            5,
            TreeNode(
                4,
                l=TreeNode(
                    11,
                    TreeNode(7),
                    TreeNode(2)
                )
            ),
            TreeNode(
                8,
                TreeNode(13),
                TreeNode(4, r=TreeNode(1))
            )
        )
        self.assertTrue(s.hasPathSum(t, 22))

        t = TreeNode(1)
        self.assertFalse(s.hasPathSum(t, 22))

        t = TreeNode(
            2,
            TreeNode(3),
            TreeNode(4)
        )
        self.assertFalse(s.hasPathSum(t, 22))

        t = TreeNode(
            19,
            TreeNode(3, TreeNode(4), TreeNode(7)),
            TreeNode(4, TreeNode(10), TreeNode(5))
        )
        self.assertFalse(s.hasPathSum(t, 22))

        t = TreeNode(1, l=TreeNode(2))
        self.assertFalse(s.hasPathSum(t, 1))

        t = TreeNode(
            1,
            TreeNode(
                2,
                l=TreeNode(
                    3,
                    l=TreeNode(
                        4,
                        l=TreeNode(5)
                    )
                )
            )
        )
        self.assertFalse(s.hasPathSum(t, 6))


if __name__ == '__main__':
    unittest.main()

