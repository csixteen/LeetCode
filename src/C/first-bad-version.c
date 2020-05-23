// https://leetcode.com/problems/first-bad-version/submissions/
// The API isBadVersion is defined for you.
// bool isBadVersion(int version);

int firstBadVersion(int n) {
    int lo = 1;
    int hi = n;

    while (lo <= hi) {
        int mid = lo + (int)((hi - lo) / 2);
        bool ibf = isBadVersion(mid);

        if (ibf && (mid == 0 || !isBadVersion(mid-1))) {
            return mid;
        } else if (ibf && isBadVersion(n-1)) {
            hi = mid - 1;
        } else {
            lo = mid + 1;
        }
    }

    return lo;
}
