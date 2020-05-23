#!/usr/bin/env python3
# coding: utf-8
from collections import defaultdict
from typing import List
import unittest


class Solution:
    @staticmethod
    def subdomainVisits(cpdomains: List[str]) -> List[str]:
        ret = defaultdict(int)
        for visits in cpdomains:
            count, domain = visits.split(" ")
            parts = domain.split(".")
            for i in range(len(parts)):
                ret[".".join(parts[i:])] += int(count)

        return ["{} {}".format(v, k) for k, v in ret.items()]


class TestSolution(unittest.TestCase):
    def setUp(self):
        self.test_cases = [
            (
                ["9001 discuss.leetcode.com"],
                ["9001 discuss.leetcode.com", "9001 leetcode.com", "9001 com"]
            ),
            (
                ["900 google.mail.com", "50 yahoo.com", "1 intel.mail.com", "5 wiki.org"],
                ["901 mail.com","50 yahoo.com","900 google.mail.com","5 wiki.org","5 org","1 intel.mail.com","951 com"]
            )
        ]

    def test_subdomainVisits(self):
        for i, e in self.test_cases:
            self.assertSetEqual(set(Solution.subdomainVisits(i)) ^ set(e), set())


if __name__ == '__main__':
    unittest.main()

