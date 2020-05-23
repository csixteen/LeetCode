#!/usr/bin/env python3
# coding: utf-8
# https://leetcode.com/problems/diameter-of-binary-tree/

import unittest


class TreeNode:
    def __init__(self, x, left=None, right=None):
        self.val = x
        self.left = left
        self.right = right


class Solution:
    def diameterOfBinaryTree(self, root: TreeNode) -> int:
        self.answer = 0

        def helper(node: TreeNode):
            if not node:
                return 0

            left = helper(node.left)
            right = helper(node.right)
            self.answer = max(self.answer, left + right)

            return max(left, right) + 1

        helper(root)

        return self.answer


class TestSolution(unittest.TestCase):
    def setUp(self):
        self.input_expected = [
            (
                {
                    "root": TreeNode(1, left=TreeNode(4))
                },
                1,
            ),
            (
                {
                    "root": TreeNode(1, right=TreeNode(2)),
                },
                1,
            ),
            (
                {
                    "root": TreeNode(1,
                                TreeNode(2, TreeNode(4), TreeNode(5)),
                                TreeNode(3)),
                },
                3,
            ),
            (
                {
                    "root": TreeNode(1, TreeNode(4), TreeNode(2)),
                },
                2,
            ),
            (
                {
                    "root": TreeNode(1,
                                TreeNode(4),
                                TreeNode(2,
                                    TreeNode(3,
                                        TreeNode(6, left=TreeNode(10)),
                                        TreeNode(7)),
                                    TreeNode(5,
                                        TreeNode(8),
                                        TreeNode(9, right=TreeNode(11))))),
                },
                6,
            ),
        ]

    def test_diameterOfBinaryTree(self):
        s = Solution()

        for _input, expected in self.input_expected:
            self.assertEqual(expected, s.diameterOfBinaryTree(**_input))


if __name__ == "__main__":
    unittest.main(verbosity=2)
