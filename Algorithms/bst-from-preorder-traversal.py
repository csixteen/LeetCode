#!/usr/bin/env python3
# coding: utf-8
# https://leetcode.com/problems/construct-binary-search-tree-from-preorder-traversal/

from typing import List
import unittest


class TreeNode:
    def __init__(self, x, left=None, right=None):
        self.val = x
        self.left = left
        self.right = right

    def __repr__(self) -> str:
        return f"val: {self.val}, (left: {self.left}), (right: {self.right})"


class Solution:
    def bstFromPreorder(self, preorder: List[int]) -> TreeNode:
        root = TreeNode(preorder[0])

        # left
        smaller = list(filter(lambda x: x < preorder[0], preorder))
        if len(smaller) > 0:
            root.left = self.bstFromPreorder(smaller)

        # right
        larger = list(filter(lambda x: x > preorder[0], preorder))
        if len(larger) > 0:
            root.right = self.bstFromPreorder(larger)

        return root


class TestSolution(unittest.TestCase):
    def setUp(self):
        self.input_expected = [
            (
                {"preorder": [8,5,1,7,10,12]},
                TreeNode(
                    8,
                    TreeNode(5, TreeNode(1), TreeNode(7)),
                    TreeNode(10, right=TreeNode(12)),
                ),
            ),
            (
                {"preorder": [8, 9, 10, 11]},
                TreeNode(8, right=TreeNode(9, right=TreeNode(10, right=TreeNode(11)))),
            ),
        ]

    def same_tree(self, a: TreeNode, b: TreeNode) -> bool:
        if not a and not b:
            return True
        elif not a or not b:
            return False
        elif a.val != b.val:
            return False
        else:
            return self.same_tree(a.left, b.left) and self.same_tree(a.right, b.right)

    def test_bstFromPreorder(self):
        s = Solution()

        for _input, expected in self.input_expected:
            self.assertTrue(self.same_tree(expected, s.bstFromPreorder(**_input)))


if __name__ == "__main__":
    unittest.main(verbosity=2)
