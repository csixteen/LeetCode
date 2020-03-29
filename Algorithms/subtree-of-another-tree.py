#!/usr/bin/env python3
# coding: utf-8
# https://leetcode.com/problems/subtree-of-another-tree/

import unittest


class TreeNode:
    def __init__(self, x, left=None, right=None):
        self.val = x
        self.left = left
        self.right = right


class Solution:
    def _is_same_tree(self, s: TreeNode, t: TreeNode) -> bool:
        if not s and not t:
            return True
        if not s or not t:
            return False
        if s.val != t.val:
            return False

        return (
            self._is_same_tree(s.left, t.left) and 
            self._is_same_tree(s.right, t.right)
        )

    def isSubtree(self, s: TreeNode, t: TreeNode) -> bool:
        if not s and not t:
            return True

        if not s or not t:
            return False

        return (
            self._is_same_tree(s, t) or
            self.isSubtree(s.left, t) or
            self.isSubtree(s.right, t)
        )

class TestSolution(unittest.TestCase):
    def setUp(self):
        self.input_expected = [
            (
                {
                    "s": TreeNode(3,
                            TreeNode(4, TreeNode(1), TreeNode(2)),
                            TreeNode(5)),
                    "t": TreeNode(4, TreeNode(1), TreeNode(2)),
                },
                True
            ),
            (
                {
                    "s": TreeNode(3,
                            TreeNode(4, TreeNode(1), TreeNode(2, TreeNode(0), None)),
                            TreeNode(5)),
                    "t": TreeNode(4, TreeNode(1), TreeNode(2)),
                },
                False
            ),
        ]

    def test_isSubtree(self):
        s = Solution()

        for _input, expected in self.input_expected:
            self.assertEqual(expected, s.isSubtree(**_input))


if __name__ == "__main__":
    unittest.main(verbosity=2)
