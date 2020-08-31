#!/usr/bin/env python3
# coding: utf-8
# https://leetcode.com/problems/linked-list-cycle/
class Solution(object):
    def hasCycle(self, head):
        """
        :type head: ListNode
        :rtype: bool
        """
        while head and not hasattr(head, 'visited'):
            head.visited = True
            head = head.next
        return head != None

