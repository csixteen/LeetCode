#!/usr/bin/env python3

from __future__ import annotations


class Solution:
    def countBinarySubstrings(self, s: str) -> int:
        prev, groups = "x", []
        for d in s:
            if d == prev:
                groups[-1] += 1
            else:
                groups.append(1)

            prev = d

        return sum([min(groups[i], groups[i+1]) for i in range(len(groups)-1)])
