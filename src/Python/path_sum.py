#!/usr/bin/env python3
# coding: utf-8
import unittest


class TreeNode:
    def __init__(self, x):
        self.val = x
        self.left = None
        self.right = None


class Solution:
    def hasPathSum2(self, root, s):
        if not root:
            return False
        elif root.val == s and not root.left and not root.right:
            return True
        else: 
            if self.hasPathSum2(root.left, s - root.val):
                return True
            if self.hasPathSum2(root.right, s - root.val):
                return True

    def hasPathSum(self, root, s):
        def DFS(node, target, acc):
            if acc + node.val == target and not node.left and not node.right:
                return True
            else:
                left = DFS(node.left, target, acc + node.val) if node.left else False
                right = DFS(node.right, target, acc + node.val) if node.right else False
                return left or right

        return False if not root else DFS(root, s, 0)


class TestSolution(unittest.TestCase):
    def setUp(self):
        self.test_cases = []

    def test_hasPathSum(self):
        s = Solution()
        for i, e in self.test_cases:
            self.assertEqual(s.hasPathSum(i), e)


if __name__ == '__main__':
    unittest.main(verbosity=2)

