#!/usr/bin/env python3
# coding: utf-8
import unittest


class TreeNode(object):
    def __init__(self, x, l=None, r=None):
        self.val = x
        self.left = l
        self.right = r

    def __str__(self):
        return '{} - {} - {}'.format(self.val, self.left, self.right)


class Solution(object):
    def sortedArrayToBST(self, nums):
        """
        :type nums: list[int]
        :rtype: TreeNode
        """
        if not nums:
            return None
        elif len(nums) == 1:
            return TreeNode(nums[0])
        middle = len(nums) // 2
        t = TreeNode(nums[middle])
        t.left = self.sortedArrayToBST(nums[:middle])
        t.right = self.sortedArrayToBST(nums[middle+1:])
        return t


if __name__ == '__main__':
    unittest.main()

