#!/usr/bin/env python3
# coding: utf-8
import unittest


class Solution:
    def invertTree(self, root):
        if not root:
            return root
        
        root.left, root.right = self.invertTree(root.right), self.invertTree(root.left)
        return root

class TestSolution(unittest.TestCase):
    def setUp(self):
        self.test_cases = []

    def test_invertTree(self):
        s = Solution()
        for i, e in self.test_cases:
            self.assertEqual(s.invertTree(i), e)


if __name__ == '__main__':
    unittest.main(verbosity=2)

