#!/usr/bin/env python3
# coding: utf-8
import unittest


class TreeNode(object):
    def __init__(self, x, l=None, r=None):
        self.val = x
        self.left = l
        self.right = r


class Solution(object):
    def levelOrderBottom(self, root):
        """
        :type root: TreeNode
        :rtype: list[list[int]]
        """
        acc = []
        self._levelOrderBottomRecursive(root, acc, 1)
        return acc[::-1]

    def _levelOrderBottomRecursive(self, node, acc, level):
        if node is not None:
            if len(acc) < level:
                acc.append([node.val])
            else:
                acc[level-1].append(node.val)
            self._levelOrderBottomRecursive(node.left, acc, level+1)
            self._levelOrderBottomRecursive(node.right, acc, level+1)


class TestSolution(unittest.TestCase):
    def test_levelOrderBottom(self):
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
        self.assertEqual(
            s.levelOrderBottom(t),
            [[15,7], [9,20], [3]]
        )

        t = TreeNode(
            3,
            TreeNode(
                9,
                TreeNode(1),
                TreeNode(4)
            ),
            TreeNode(
                20,
                TreeNode(15),
                TreeNode(7)
            )
        )
        self.assertEqual(
            s.levelOrderBottom(t),
            [[1,4,15,7],[9,20],[3]]
        )


if __name__ == '__main__':
    unittest.main()

