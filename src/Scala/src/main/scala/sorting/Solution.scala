package sorting

import scala.collection.mutable

object Solution {
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

  // https://leetcode.com/problems/kth-largest-element-in-an-array/
  def findKthLargestNaive(nums: Array[Int], k: Int): Int = {
    nums.sortWith((a,b) => a.compareTo(b) > 0)(k-1)
  }

  def findKthLargestMinHeap(nums: Array[Int], k: Int): Int = {
    mutable.PriorityQueue[Int]().addAll(nums).dequeueAll(k-1)
  }
}
