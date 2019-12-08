#!/usr/bin/env python3
# coding: utf-8
from collections import deque
from typing import List
import unittest


class TreeNode:
        def __init__(self, x: int, l: TreeNode = None, r: TreeNode = None):
        self.val = x
        self.left = None
        self.right = None


class Solution:
    def zigzagLevelOrder(self, root: TreeNode) -> List[List[int]]:
        if not root:
            return []

        res = []
        queue = deque([(root, 0)])
        while len(queue) > 0:
            node, depth = queue.popleft()
            if len(res) <= depth:
                res.append([])
            if depth % 2 == 0:
                res[depth].append(node.val)
            else:
                res[depth].insert(0, node.val)

            if node.left:
                queue.append((node.left, depth + 1))
            if node.right:
                queue.append((node.right, depth + 1))

        return res


class TestSolution(unittest.TestCase):
    def setUp(self):
        self.test_cases = [
            (
                TreeNode(
                    x=3,
                    l=TreeNode(x=9),
                    r=TreeNode(
                        x=20,
                        l=TreeNode(x=15),
                        r=TreeNode(x=7)
                    )
                ),
                [
                    [3],
                    [20, 9],
                    [15, 7]
                ]
            ),
            (None, [])
        ]

    def test_zigzagLevelOrder(self):
        s = Solution()
        for i, e in self.test_cases:
            self.assertEqual(s.zigzagLevelOrder(i), e)


if __name__ == '__main__':
    unittest.main(verbosity=2)

