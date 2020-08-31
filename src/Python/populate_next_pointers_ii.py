#!/usr/bin/env python3
# coding: utf-8
import unittest
from collections import deque


class TreeNode:
    def __init__(self, x):
        self.val = x
        self.left = None
        self.right = None
        self.next = None


class Solution:
    def connect(self, root):
        if not root:
            return None

        queue = deque([(root, 0)])
        while len(queue) > 0:
            node,l = queue.popleft()
            if queue and queue[0][1] == l:
                node.next = queue[0][0]
            else:
                node.next = None

            if node.left:
                queue.append((node.left, l+1))
            if node.right:
                queue.append((node.right, l+1))

        return root


class TestSolution(unittest.TestCase):
    def setUp(self):
        self.test_cases = []

    def test_connect(self):
        for i, e in self.test_cases:
            self.assertEqual(Solution.connect(i), e)


if __name__ == '__main__':
    unittest.main(verbosity=2)

