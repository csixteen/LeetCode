class Solution(object):
    def findDuplicates(self, nums):
        dups, numset = [], set()
        for e in nums:
            if e in numset:
                dups.append(e)
            else:
                numset.add(e)
        return dups
