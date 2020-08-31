#!/usr/bin/env python3
# coding: utf-8

from __future__ import annotations
import unittest


class TreeNode:
    def __init__(self, val: int, left: TreeNode = None, right: TreeNode = None):
        self.val = val
        self.left = left
        self.right = right

    def __eq__(self, other: TreeNode) -> bool:
        if not other or self.val != other.val:
            return False
        else:
            l = not self.left or self.left == other.left
            r = not self.right or self.right == other.right
            return l and r


class TestTreeNode(unittest.TestCase):
    def test_equal_trees(self):
        self.assertEqual(
            TreeNode(2, TreeNode(1), TreeNode(3)),
            TreeNode(2, TreeNode(1), TreeNode(3)),
        )

    def test_different_trees(self):
        self.assertNotEqual(
            TreeNode(2, TreeNode(1), TreeNode(3)),
            TreeNode(2, TreeNode(2), TreeNode(3)),
        )


if __name__ == "__main__":
    unittest.main(verbosity=2)
