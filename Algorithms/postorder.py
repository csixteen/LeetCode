#!/usr/bin/env python3
# coding: utf-8
import unittest


class Solution:
    def postorderTraversal(self, root):
        if not root:
            return []

        res = []
        stack = [root]
        while stack:
            node = stack.pop()
            if not node.left and not node.right:
                res.append(node.val)
            else:
                stack.append(node)
                if node.right:
                    stack.append(node.right)
                    node.right = None
                if node.left:
                    stack.append(node.left)
                    node.left = None

        return res


class TestSolution(unittest.TestCase):
    def setUp(self):
        self.test_cases = []

    def test_postorderTraversal(self):
        s = Solution()
        for i, e in self.test_cases:
            self.assertEqual(s.postorderTraversal(i), e)


if __name__ == '__main__':
    unittest.main(verbosity=2)

