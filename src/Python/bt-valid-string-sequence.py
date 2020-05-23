#!/usr/bin/env python3
# coding: utf-8
# https://leetcode.com/explore/challenge/card/30-day-leetcoding-challenge/532/week-5/3315/

from typing import List
import unittest


class TreeNode:
    def __init__(self, val, left=None, right=None):
        self.val = val
        self.left = left
        self.right = right

    def __repr__(self):
        return f"val: {self.val}, ({self.left}), ({self.right})"


class Solution:
    def isValidSequence(self, root: TreeNode, arr: List[int]) -> bool:
        def _isValidSeq(node: TreeNode, i: int) -> bool:
            if all([
                i == len(arr) - 1,
                node.val == arr[i],
                not node.left,
                not node.right,
            ]):
                return True
            elif node and i < len(arr) and node.val != arr[i]:
                return False
            else:
                left = _isValidSeq(node.left, i+1) \
                        if (node.left and i < len(arr) - 1) else False
                right = _isValidSeq(node.right, i+1) \
                        if (node.right and i < len(arr) - 1) else False

                return left or right

        return _isValidSeq(root, 0)


class TestSolution(unittest.TestCase):
    def setUp(self):
        tree = TreeNode(
            0,
            TreeNode(1,
                TreeNode(0, right=TreeNode(1)),
                TreeNode(1, TreeNode(0), TreeNode(0))),
            TreeNode(0, TreeNode(0)))

        self.input_expected = [
            (
                {"root": tree, "arr": [0, 1, 0, 1]},
                True,
            ),
            (
                {"root": tree, "arr": [0, 0, 1]},
                False,
            ),
            (
                {"root": tree, "arr": [0, 1, 1]},
                False,
            ),
        ]

    def test_isValidSequence(self):
        s = Solution()

        for _input, expected in self.input_expected:
            self.assertEqual(expected, s.isValidSequence(**_input))


if __name__ == "__main__":
    unittest.main(verbosity=2)
