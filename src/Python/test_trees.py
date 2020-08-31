#!/usr/bin/env python3
# coding: utf-8

import unittest

from trees import Solution, TreeNode


class TestTrees(unittest.TestCase):
    def setUp(self):
        self.s = Solution()

    def test_levelOrderBottom(self):
        self.assertEqual(
            self.s.levelOrderBottom(
                TreeNode(
                    3,
                    TreeNode(9),
                    TreeNode(20, TreeNode(15), TreeNode(7)),
                ),
            ),
            [[15, 7], [9, 20], [3]],
        )

        self.assertEqual(
            self.s.levelOrderBottom(
                TreeNode(
                    3,
                    TreeNode(9, TreeNode(1), TreeNode(4)),
                    TreeNode(20, TreeNode(15), TreeNode(7)),
                ),
            ),
            [[1, 4, 15, 7], [9, 20], [3]],
        )

    def test_isValidSequence(self):
        t = TreeNode(
            0,
            TreeNode(
                1,
                TreeNode(0, right=TreeNode(1)),
                TreeNode(1, TreeNode(0), TreeNode(0)),
            ),
            TreeNode(0, TreeNode(0)),
        )

        self.assertTrue(self.s.isValidSequence(t, [0, 1, 0, 1]))
        self.assertFalse(self.s.isValidSequence(t, [0, 0, 1]))
        self.assertFalse(self.s.isValidSequence(t, [0, 1, 1]))

    def test_isBalanced(self):
        self.assertTrue(
            self.s.isBalanced(
                TreeNode(
                    3,
                    TreeNode(9),
                    TreeNode(20, TreeNode(15), TreeNode(7)),
                ),
            )
        )
        
        self.assertTrue(self.s.isBalanced(TreeNode(1)))

        self.assertFalse(
            self.s.isBalanced(
                TreeNode(
                    1,
                    TreeNode(
                        2,
                        TreeNode(3, TreeNode(4), TreeNode(4)),
                        TreeNode(3),
                    ),
                    TreeNode(2),
                ),
            ),
        )

        self.assertFalse(
            self.s.isBalanced(
                TreeNode(1, right=TreeNode(2, right=TreeNode(3))),
            ),
        )

        self.assertFalse(
            self.s.isBalanced(
                TreeNode(
                    1,
                    TreeNode(2, left=TreeNode(3, left=TreeNode(4))),
                    TreeNode(2, right=TreeNode(3, right=TreeNode(4))),
                ),
            ),
        )

    def test_zigzagLevelOrder(self):
        self.assertEqual(
            [[3], [20, 9], [15, 7]],
            self.s.zigzagLevelOrder(
                TreeNode(
                    3,
                    TreeNode(9),
                    TreeNode(20, TreeNode(15), TreeNode(7)),
                ),
            ),
        )

    def test_buildFromInorderPostorder(self):
        self.assertEqual(
            TreeNode(
                3,
                TreeNode(9),
                TreeNode(20, TreeNode(15), TreeNode(7)),
            ),
            self.s.buildFromInorderPostorder(
                [9, 3, 15, 20, 7], [9, 15, 7, 20, 3],
            ),
        )


if __name__ == "__main__":
    unittest.main(verbosity=2)
