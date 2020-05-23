#!/usr/bin/env python3
# coding: utf-8
# https://leetcode.com/problems/group-anagrams/

from collections import defaultdict
from typing import List, Tuple
import unittest


class Solution:
    def groupAnagrams(self, strs: List[str]) -> List[List[str]]:
        mp = defaultdict(list)

        for word in strs:
            mp[tuple(sorted(word))].append(word)

        return list(mp.values())


Input = List[str]
Expected = List[List[str]]


class TestSolution(unittest.TestCase):
    def setUp(self):
        self.input_expected: List[Tuple[Input, Expected]] = [
            (
                ["eat", "tea", "tan", "ate", "nat", "bat"],
                [
                    ["ate","eat","tea"],
                    ["nat","tan"],
                    ["bat"],
                ],
            ),
        ]

    def test_groupAnagrams(self):
        s = Solution()

        for _input, expected in self.input_expected:
            self.assertEqual(
                [sorted(x) for x in sorted(
                    s.groupAnagrams(_input),
                    key=lambda x: len(x),
                    reverse=True,
                )],
                expected
            )


if __name__ == "__main__":
    unittest.main(verbosity=2)
