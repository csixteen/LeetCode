// https://leetcode.com/problems/maximum-sum-circular-subarray/

// The idea behind this solution is to use a tweaked version of the
// Kadane's Algorithm for maximum contiguous subarray. In the original
// version, the maximum subarray is composed of one single interval
// between 0 and N-1 (where N is the length of the array). Using a circular
// array, the maximum subarray can be either one single interval or
// two intervals. 
//
// The trivial cases are still an array with only positive
// integers (in which case the maximum subarray is the entire array)
// or only negative integers (in which case the maximum subarray is made
// of a single element, which is the maximum element in the array).
//
// On the event of having a mixed array, we can make the following observations:
// 1-) The solution will be either a maximum subarray of a single interval,
// in which case the solution is given by Kadane's Algorithm, or
// 2-) The solution will be a maximum subarray of two intervals, in which
// case the solution is the difference between the sum of the entire array
// and Kadane's Algorithm for the MINIMUM contiguous subarray.
// 
// This last case may seem a bit weird at first, but let's see with some examples:
// 
// Example 1
// ==========
// [5, -3, -2,  5] - in this case, the sum of the entire array is 5. On the other hand,
// we can see that we can have a maximum subarray of 2 intervals: [5] + [5]. Meaning,
// we can have a maximum subarray by subtracting the minimum subarray ([-3, -2]) from
// the sum of the entire array.
//
// Example 2
// ==========
// [1, -2, 3, -2] - in this case, the sum of the entire array is 0. The maximum subarray
// that we can calculate is made of a single element ([3]). The minimum subarray is
// also made of a single element ([-2]). In this case, the result we're interested in
// is simply the maximum subarray [3] (which is given by the normal Kadane's algorithm).
//
// Example 3
// =========
// [20, -40, -5, 10] - again, similar to the first example. The sum of the entire array
// is -15. The maximum subarray as per Kadane's algorithm is [20] and the minimum
// subarray as per Kadane's minimum algorithm is [-40, -5]. Again, we can instinctively
// say that the maximum subarray should have two intervals [20] and [10], which is
// the same as subtracting the minimum subarray from the sum of the entire array.
//
// Visually speaking, building a two interval maximum subarray is like this:
// [X, X, Y Y Y Y Y, X, X] where [Y Y Y Y Y] is the minimum subarray. 
//        ---------
// The minimum subarray can also start or end at the edges of the array.

struct Solution {}

impl Solution {
    pub fn max_subarray_sum_circular(a: Vec<i32>) -> i32 {
        let (
            mut max_curr,
            mut max_glob,
            mut min_curr,
            mut min_glob,
        ) = (a[0], a[0], a[0], a[0]);

        let sum: i32 = a.iter().sum();

        for n in a.iter().skip(1) {
            max_curr = *n.max(&(max_curr + n));
            max_glob = max_glob.max(max_curr);

            min_curr = *n.min(&(min_curr + n));
            min_glob = min_glob.min(min_curr);
        }

        if min_glob == sum {
            max_glob
        } else {
            max_glob.max(sum - min_glob)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct TestCase {
        a: Vec<i32>,
        expected: i32,
    }

    #[test]
    fn test_max_subarray_sum_circular() {
        let test_cases = [
            TestCase { a: vec![1, -2, 3, -2], expected: 3 },
            TestCase { a: vec![5, -3, 5], expected: 10 },
            TestCase { a: vec![3, -1, 2, -1], expected: 4 },
            TestCase { a: vec![3, -2, 2, -3], expected: 3 },
            TestCase { a: vec![-2, -3, -1], expected: -1 },
            TestCase { a: vec![-2], expected: -2 },
        ];

        for case in test_cases.iter() {
            assert_eq!(
                case.expected,
                Solution::max_subarray_sum_circular(case.a.clone()),
            );
        }
    }
}
