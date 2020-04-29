#!/usr/bin/env python3
# coding: utf-8
# https://leetcode.com/problems/binary-tree-maximum-path-sum/

import unittest


class TreeNode:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right


class Solution:
    def maxPathSum(self, root: TreeNode) -> int:
        def DFS(node: TreeNode) -> int:
            if not node:
                return 0

            left = max(DFS(node.left), 0)
            right = max(DFS(node.right), 0)

            cost = node.val + left + right
            self.max_path_sum = max(self.max_path_sum, cost)

            return node.val + max(left, right)

        self.max_path_sum = float("-inf")
        DFS(root)

        return self.max_path_sum


class TestSolution(unittest.TestCase):
    def setUp(self):
        self.input_expected = [
            (
                {"root": TreeNode(-2, TreeNode(-1))},
                -1,
            ),
            (
                {"root": TreeNode(-3)},
                -3,
            ),
            (
                {"root": TreeNode(1, TreeNode(2), TreeNode(3))},
                6,
            ),
            (
                {
                    "root": TreeNode(
                        -10, 
                        TreeNode(9),
                        TreeNode(20, TreeNode(15), TreeNode(7)),
                    ),
                },
                42,
            ),
            (
                {
                    "root": TreeNode(
                        0,
                        TreeNode(20, TreeNode(-30), TreeNode(5)),
                        TreeNode(17, TreeNode(6), TreeNode(-12)),
                    ),
                },
                48,
            ),
        ]

    def test_maxPathSum(self):
        s = Solution()

        for _input, expected in self.input_expected:
            self.assertEqual(expected, s.maxPathSum(**_input))


if __name__ == "__main__":
    unittest.main(verbosity=2)
