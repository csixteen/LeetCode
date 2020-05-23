#!/usr/bin/env python3
# coding: utf-8
import unittest


class Solution:
    def append(self, node, tree):
        if not tree.right:
            tree.right = node
        else:
            tree.right = self.append(node, tree.right)
        return tree

    def flatten(self, root):
        if not root or (not root.left and not root.right):
            return root
        if not root.left:
            root.right = self.flatten(root.right)
            return root
        else:
            root.right = self.flatten(self.append(root.right, root.left))
            root.left = None
            return root


class TestSolution(unittest.TestCase):
    def setUp(self):
        self.test_cases = []

    def test_flatten(self):
        s = Solution()
        for i, e in self.test_cases:
            self.assertEqual(s.flatten(i), e)


if __name__ == '__main__':
    unittest.main(verbosity=2)

