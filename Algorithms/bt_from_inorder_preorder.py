#!/usr/bin/env python3
# coding: utf-8
import unittest


class TreeNode:
    def __init__(self, x):
        self.val = x
        self.left = None
        self.right = None


class Solution:
    def buildTree(self, preorder: List[int], inorder: List[int]) -> TreeNode:
        if not preorder and not inorder:
            return None

        root = TreeNode(preorder[0])
        i = inorder.index(preorder[0])
        root.left = self.buildTree(preorder[1:i+1], inorder[:i])
        root.right = self.buildTree(preorder[i+1:], inorder[i+1:])

        return root


class TestSolution(unittest.TestCase):
    def setUp(self):
        self.test_cases = []

    def test_buildTree(self):
        s = Solution()
        for i, e in self.test_cases:
            self.assertEqual(s.buildTree(i), e)


if __name__ == '__main__':
    unittest.main(verbosity=2)

