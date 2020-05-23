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

        level = 0
        i = 0
        queue = deque([root])
        while len(queue) > 0:
            node = queue.popleft()
            if queue and i < 2**level - 1:
                node.next = queue[0]
            else:
                node.next = None

            i += 1
            if i == 2**level:
                i = 0
                level += 1

            if node.left:
                queue.append(node.left)
            if node.right:
                queue.append(node.right)

        return root


class TestSolution(unittest.TestCase):
    def setUp(self):
        self.test_cases = []

    def test_connect(self):
        for i, e in self.test_cases:
            self.assertEqual(Solution.connect(i), e)


if __name__ == '__main__':
    unittest.main(verbosity=2)

