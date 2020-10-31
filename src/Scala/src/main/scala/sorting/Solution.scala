package sorting

import scala.collection.mutable

object Solution {

  //---------------------------------------------------------------

  // https://leetcode.com/problems/sort-colors/
  def sortColors(nums: Array[Int]): Unit = {
    def swap(a: Int, b: Int): Unit = {
      val tmp = nums(a)
      nums(a) = nums(b)
      nums(b) = tmp
    }

    def go(left: Int, middle: Int, right: Int): Unit = {
      if (middle > right) return
      else
        nums(middle) match {
          case 0 => swap(left, middle); go(left+1, middle+1, right)
          case 1 => go(left, middle+1, right)
          case 2 => swap(middle, right); go(left, middle, right-1)
        }
    }

    go(0, 0, nums.length-1)
  }


  //---------------------------------------------------------------

  // https://leetcode.com/problems/top-k-frequent-elements/
  def topKFrequent(nums: Array[Int], k: Int): Array[Int] = {
    nums
      .groupBy(x => x)
      .values
      .toArray
      .sortWith{ case (a,b) => a.length.compareTo(b.length) > 0 }
      .map(_.head)
      .take(k)
  }


  //---------------------------------------------------------------

  // https://leetcode.com/problems/kth-largest-element-in-an-array/
  def findKthLargestNaive(nums: Array[Int], k: Int): Int = {
    nums.sortWith((a,b) => a.compareTo(b) > 0)(k-1)
  }

  def findKthLargestMinHeap(nums: Array[Int], k: Int): Int = {
    mutable.PriorityQueue[Int]().addAll(nums).dequeueAll(k-1)
  }


  //---------------------------------------------------------------

  // https://leetcode.com/problems/find-peak-element/
  def findPeakElementLinear(nums: Array[Int]): Int = {
    (0 until nums.length-1).takeWhile(i => nums(i) < nums(i+1)).lastOption.getOrElse(-1) + 1
  }

  def findPeakElementLog(nums: Array[Int]): Int = {
    def go(left: Int, right: Int): Int = {
      if (left == right) left
      else {
        val mid = left + (right - left) / 2
        if (nums(mid) > nums(mid+1)) go(left, mid)
        else go(mid+1, right)
      }
    }

    go(0, nums.length-1)
  }


  //---------------------------------------------------------------

  // https://leetcode.com/problems/merge-intervals/
  def mergeIntervals(intervals: Array[Array[Int]]): Array[Array[Int]] = {
    def go(ints: Array[Array[Int]]): Array[Array[Int]] = {
      if (ints.length < 2) ints
      else
        ints match {
          case Array(Array(x1, x2), Array(y1, y2), xs@_*) if (x2 >= y1) =>
            go(Array(x1, x2.max(y2)) +: xs.toArray)
          case Array(x, xs@_*) => x +: go(xs.toArray)
        }
    }

    go(intervals.sortBy(_(0)))
  }

  def mergeIntervalsFold(intervals: Array[Array[Int]]): Array[Array[Int]] = {
    intervals.sortBy(_(0)).foldLeft(Array[Array[Int]]())((acc, a) => {
        if (acc.isEmpty) Array(a)
        else if(a(0) <= acc.last(1)) acc.init.appended(Array(acc.last(0), acc.last(1).max(a(1))))
        else acc.appended(a)
    })
  }


  //---------------------------------------------------------------

  // https://leetcode.com/problems/search-in-rotated-sorted-array/
  // The original method is called `search`, which is a poor choice of name
  def searchInSortedArray(nums: Array[Int], target: Int): Int = {
    def go(start: Int, end: Int): Int = {
      if (start > end) -1
      else {
        val mid = start + (end - start) / 2
        (nums(mid).compareTo(target), nums(mid).compareTo(nums(start))) match {
          case (0, _) => mid
          case (_, -1) =>
            if (target <= nums(end) && target > nums(mid)) go(mid+1, end)
            else go(start, mid-1)
          case _ =>
            if (target >= nums(start) && target < nums(mid)) go(start, mid-1)
            else go(mid+1, end)
        }
      }
    }

    go(0, nums.length-1)
  }


  //---------------------------------------------------------------

  // https://leetcode.com/problems/meeting-rooms-ii/
  /*
   * The idea behind this solution is to associate an increment value to
   * the start and to the end of an interval. When the interval starts,
   * the increment value is positive (meaning we need another meeting room).
   * The the interval ends, the increment value is negative, meaning that
   * we've vacated a meeting room. If we have multiple intervals overlapping,
   * we'll increment multiple times before we start decrementing. With that
   * in mind, we can use scanLeft, which will return a collection containing
   * cumulative results of applying the function going from left to right.
   * The minimum number of meeting rooms is the max of these cumulative
   * results.
   */
  def minMeetingRooms(intervals: Array[Array[Int]]): Int = {
    intervals.
      flatMap(i => List((i(0), 1), (i(1), -1))).
      sorted.
      scanLeft(0)(_ + _._2).
      max
  }
}
