#!/usr/bin/env python3
# coding: utf-8
import unittest


class TreeNode:
    def __init__(self, x):
        self.val = x
        self.left = None
        self.right = None


class Solution:
    def preorderTraversal(self, root):
        if not root:
            return []

        res = []
        stack = [root]
        while stack:
            node = stack.pop()
            res.append(node.val)
            if node.right:
                stack.append(node.right)
            if node.left:
                stack.append(node.left)

        return res


class TestSolution(unittest.TestCase):
    def setUp(self):
        self.test_cases = []

    def test_preorderTraversal(self):
        for i, e in self.test_cases:
            self.assertEqual(Solution.preorderTraversal(i), e)


if __name__ == '__main__':
    unittest.main(verbosity=2)

