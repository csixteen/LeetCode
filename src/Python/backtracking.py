#!/usr/bin/env python3
# coding: utf-8

from typing import List
import itertools


class Solution:
    def letterCombinations(self, digits: List[str]) -> List[str]:
        """ https://leetcode.com/problems/letter-combinations-of-a-phone-number/ """
        if not len(digits):
            return []

        keyboard = {
            '2': ['a', 'b', 'c'],
            '3': ['d', 'e', 'f'],
            '4': ['g', 'h', 'i'],
            '5': ['j', 'k', 'l'],
            '6': ['m', 'n', 'o'],
            '7': ['p', 'q', 'r', 's'],
            '8': ['t', 'u', 'v'],
            '9': ['w', 'x', 'y', 'z'],
        }

        tuples = list(itertools.product(*[keyboard[d] for d in digits]))

        return [''.join(x) for x in tuples]
