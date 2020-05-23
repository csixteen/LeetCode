#!/usr/bin/env python3
# coding: utf-8
from collections import deque
import unittest


class TreeNode:
    def __init__(self, x):
        self.val = x
        self.left = None
        self.right = None


class Solution:
    def minDepth(self, root):
        if not root:
            return 0

        queue = deque()
        queue.append((root, 1))
        while len(queue) > 0:
            node, depth = queue.popleft()
            if not node.left and not node.right:
                return depth

            if node.left:
                queue.append((node.left, depth + 1))
            if node.right:
                queue.append((node.right, depth + 1))


class TestSolution(unittest.TestCase):
    def setUp(self):
        self.test_cases = []

    def test_minDepth(self):
        for i, e in self.test_cases:
            self.assertEqual(Solution.minDepth(i), e)


if __name__ == '__main__':
    unittest.main(verbosity=2)

