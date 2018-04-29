#!/usr/bin/env python3
# coding: utf-8
# https://leetcode.com/problems/intersection-of-two-linked-lists/
class Solution(object):
    def getIntersectionNode(self, headA, headB):
        """
        :type headA: ListNode
        :type headB: ListNode
        :rtype: ListNode
        """
        if not headA or not headB:
            return None

        set_a = set()
        while headA:
            set_a.add(id(headA))
            headA = headA.next
        while headB:
            i = id(headB)
            if i in set_a:
                return headB
            headB = headB.next

