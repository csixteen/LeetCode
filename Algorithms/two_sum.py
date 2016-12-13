from operator import itemgetter

class Solution(object):
    def two_sum(self, nums, target):
        nums = sorted(list(enumerate(nums)), key=itemgetter(1))
        n = len(nums)-1
        while nums[0][1] + nums[n][1] > target:
            n -= 1

        for i in range(0, n):
            for j in range(n, -1, -1):
                if nums[i][1] + nums[j][1] == target:
                    return [nums[i][0], nums[j][0]]



import unittest

class SolutionTest(unittest.TestCase):
    def test_two_sum(self):
        nums1 = [2, 7, 11, 15]
        nums2 = [101, 23, 45, 2, 66, 14]
        nums3 = [999, 1, 132, 57, 192, 8, 54]
        target1 = 9
        target2 = 68
        target3 = 55
        
        self.assertEqual(Solution().two_sum(nums1, target1), [0, 1])
        self.assertEqual(Solution().two_sum(nums2, target2), [3, 4])
        self.assertEqual(Solution().two_sum(nums3, target3), [1, 6])

if __name__ == "__main__":
    unittest.main()
