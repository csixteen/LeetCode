#!/usr/bin/env python3
# coding: utf-8
from typing import List
import unittest


class TreeNode:
    def __init__(self, x):
        self.val = x
        self.left = None
        self.right = None


class Solution:
    def buildTree(self, inorder: List[int], postorder: List[int]) -> TreeNode:
        if not inorder and not postorder:
            return None

        root = TreeNode(postorder[-1])
        i = inorder.index(postorder[-1])
        root.left = self.buildTree(inorder[:i], postorder[:i])
        root.right = self.buildTree(inorder[i+1:], postorder[i:-1])

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

