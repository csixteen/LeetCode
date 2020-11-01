#!/usr/bin/env python3
# coding: utf-8

from __future__ import annotations
from collections import deque
from typing import List
import unittest


class TreeNode:
    def __init__(self, val: int, left: TreeNode = None, right: TreeNode = None):
        self.val = val
        self.left = left
        self.right = right

    def __repr__(self) -> str:
        return f"val: {self.val} ({self.left}) ({self.right})"

    def __eq__(self, other: TreeNode) -> bool:
        if not other or self.val != other.val:
            return False
        else:
            l = not self.left or self.left == other.left
            r = not self.right or self.right == other.right
            return l and r


class NPTreeNode(TreeNode):
    """
    Used here: https://leetcode.com/problems/populating-next-right-pointers-in-each-node/
    """
    def __init__(
        self,
        val: int,
        left: NPTreeNode = None,
        right: NPTreeNode = None,
        _next: NPTreeNode = None,
    ):
        super().__init__(val, left, right)
        self.next = _next


class Solution:
    def levelOrderBottom(self, root: TreeNode) -> List[List[int]]:
        def loop(node: TreeNode, acc: List[List[Int]], level: int):
            if node:
                if len(acc) < level:
                    acc.append([node.val])
                else:
                    acc[level-1].append(node.val)

                loop(node.left, acc, level + 1)
                loop(node.right, acc, level + 1)

        acc = []
        loop(root, acc, 1)
        return acc[::-1]

    def isValidSequence(self, root: TreeNode, arr: List[int]) -> bool:
        """
        https://leetcode.com/explore/challenge/card/30-day-leetcoding-challenge/532/week-5/3315/
        """
        def loop(node: TreeNode, i: int) -> bool:
            if all([
                i == len(arr) -1,
                node.val == arr[i],
                not node.left,
                not node.right,
            ]):
                return True
            elif node and i < len(arr) and node.val != arr[i]:
                return False
            else:
                left = loop(node.left, i+1) \
                        if (node.left and i < len(arr) - 1) else False
                right = loop(node.right, i+1) \
                        if (node.right and i < len(arr) - 1) else False

                return left or right

        return loop(root, 0)

    def __treeHeight(self, root: TreeNode) -> int:
        def loop(node: TreeNode, acc: int) -> int:
            if not node:
                return acc
            return max(
                loop(node.left, acc + 1),
                loop(node.right, acc + 1),
            )

        return loop(root, 0)

    def isBalanced(self, root: TreeNode) -> bool:
        """ https://leetcode.com/problems/balanced-binary-tree/ """
        if not root:
            return True
        return abs(
            self.__treeHeight(root.left) - self.__treeHeight(root.right)
        ) <= 1 and \
            self.isBalanced(root.left) and self.isBalanced(root.right)

    def zigzagLevelOrder(self, root: TreeNode) -> List[List[int]]:
        """ https://leetcode.com/problems/binary-tree-zigzag-level-order-traversal/ """
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

    def buildFromInorderPostorder(self, inorder: List[int], postorder: List[int]) -> TreeNode:
        """
        https://leetcode.com/problems/construct-binary-tree-from-inorder-and-postorder-traversal/
        """
        if not inorder and not postorder:
            return None

        root = TreeNode(postorder[-1])
        i = inorder.index(postorder[-1])
        root.left = self.buildFromInorderPostorder(inorder[:i], postorder[:i])
        root.right = self.buildFromInorderPostorder(inorder[i+1:], postorder[i:-1])

        return root

    def buildFromInorderPreorder(self, preorder: List[int], inorder: List[int]) -> TreeNode:
        """
        https://leetcode.com/problems/construct-binary-tree-from-preorder-and-inorder-traversal/
        """
        if not preorder and not inorder:
            return None

        root = TreeNode(preorder[0])
        i = inorder.index(preorder[0])
        root.left = self.buildFromInorderPreorder(preorder[1:i+1], inorder[:i])
        root.right = self.buildFromInorderPreorder(preorder[i+1:], inorder[i+1:])

        return root

    def preorderTraversal(self, root: TreeNode) -> List[int]:
        """ https://leetcode.com/problems/binary-tree-preorder-traversal/ """
        if not root:
            return []

        res = []
        stack = [root]
        while stack:
            node = stack.pop()
            res.append(node.val)
            if node.right:
                stack.append(node.right)
            if node.left:
                stack.append(node.left)

        return res

    def isSameTree(self, p: TreeNode, q: TreeNode) -> bool:
        """ https://leetcode.com/problems/same-tree/ """
        if not p and not q:
            return True
        elif (not p or not q) or (p.val != q.val):
            return False
        else:
            return (
                self.isSameTree(p.left, q.left) and
                self.isSameTree(p.right, q.right)
            )

    def inorderTraversal(self, root: TreeNode) -> List[int]:
        # TODO: review this one
        if not root:
            return []

        res = []
        stack = [root]
        while stack:
            node = stack.pop()
            
            if not node.left and not node.right:
                res.append(node.val)
            else:
                if node.right:
                    stack.append(node.right)
                    node.right = None
                stack.append(node)
                if node.left:
                    stack.append(node.left)
                    node.left = None

        return res

    def invertTree(self, root: TreeNode) -> TreeNode:
        if not root:
            return None

        root.left = self.invertTree(root.right)
        root.right = self.invertTree(root.left)

        return root

    def isSymmetric(self, root: TreeNode) -> bool:
        """ https://leetcode.com/problems/symmetric-tree """
        def loop(left: TreeNode, right: TreeNode) -> bool:
            if not left or not right:
                return not left and not right

            return (
                left.val == right.val and
                loop(left.left, right.right) and
                loop(left.right, right.left)
            )

        if not root:
            return True

        return loop(root.left, root.right)

    def isSubtree(self, s: TreeNode, t: TreeNode) -> bool:
        """ https://leetcode.com/problems/subtree-of-another-tree/ """
        if not s and not t:
            return True

        if not s or not t:
            return False

        return (
            self.isSameTree(s, t) or
            self.isSubtree(s.left, t) or
            self.isSubtree(s.right, t)
        )

    def populateNextRightPointers(self, root: NPTreeNode) -> NPTreeNode:
        """
        TODO: Review this one. Seems unnecessarily complicated.
        https://leetcode.com/problems/populating-next-right-pointers-in-each-node/
        """
        if not root:
            return None

        level, i = 0, 0
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

    def kthSmallest(self, root: TreeNote, k: int) -> int:
        """ TODO: review """
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


class TestTreeNode(unittest.TestCase):
    def test_equal_trees(self):
        self.assertEqual(
            TreeNode(2, TreeNode(1), TreeNode(3)),
            TreeNode(2, TreeNode(1), TreeNode(3)),
        )

    def test_different_trees(self):
        self.assertNotEqual(
            TreeNode(2, TreeNode(1), TreeNode(3)),
            TreeNode(2, TreeNode(2), TreeNode(3)),
        )


if __name__ == "__main__":
    unittest.main(verbosity=2)
