package array

import scala.collection.mutable.Queue

object Solution {
  // https://leetcode.com/explore/learn/card/fun-with-arrays/521/introduction/3237/
  def findNumbers(nums: Array[Int]): Int = {
    @annotation.tailrec
    def numDigits(n: Int, acc: Int): Int = {
      if (n == 0) acc
      else numDigits(n / 10, acc+1)
    }

    nums.filter(numDigits(_, 0) % 2 == 0).length
  }

  // https://leetcode.com/explore/learn/card/fun-with-arrays/521/introduction/3240/
  def sortedSquares(nums: Array[Int]): Array[Int] = {
    nums.map(x => x*x).sorted
  }

  // https://leetcode.com/explore/learn/card/fun-with-arrays/525/inserting-items-into-an-array/3245/
  def duplicateZeros(arr: Array[Int]): Unit = {
    def go(i: Int, q: Queue[Int]): Unit = {
      if (i < arr.length) {
        if (arr(i) == 0) q.enqueue(0, 0)
        else q.enqueue(arr(i))
        arr(i) = q.dequeue()
        go(i+1, q)
      }
    }

    go(0, new Queue[Int]())
  }
}
