#!/usr/bin/env python3
# coding: utf-8
from collections import deque
import unittest


class BSTIterator:
    def __init__(self, root):
        self.root = root
        self.elements = deque()
        if root:
            stack = [root]
            while stack:
                node = stack.pop()
                if not node.left and not node.right:
                    self.elements.append(node.val)
                else:
                    if node.right:
                        stack.append(node.right)
                        node.right = None
                    stack.append(node)
                    if node.left:
                        stack.append(node.left)
                        node.left = None

    def next(self):
        return self.elements.popleft()

    def hasNext(self):
        return len(self.elements) > 0

