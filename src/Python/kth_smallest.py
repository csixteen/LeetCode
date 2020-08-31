#!/usr/bin/env python3
# coding: utf-8
import unittest


class TreeNode:
    def __init__(self, x):
        self.val = x
        self.left = None
        self.right = None


class Solution:
    def kthSmallest(self, root, k):
        stack = [root]
        counter = 0
        while stack:
            node = stack.pop()
            if not node.left and not node.right:
                counter += 1
                if counter == k:
                    return node.val
            else:
                if node.right:
                    stack.append(node.right)
                    node.right = None
                stack.append(node)
                if node.left:
                    stack.append(node.left)
                    node.left = None


class TestSolution(unittest.TestCase):
    def setUp(self):
        self.test_cases = []

    def test_kthSmallest(self):
        s = Solution()
        for i, e in self.test_cases:
            self.assertEqual(s.kthSmallest(*i), e)


if __name__ == '__main__':
    unittest.main(verbosity=2)

