class Solution(object):
    def reverse(self, x):
        """
        :type x: int
        :rtype: int
        """
        pre = "-" if x < 0 else ""
        x = int(pre + str(abs(x))[::-1])
        return 0 if not -2**31 <= x <= 2**31 else x
