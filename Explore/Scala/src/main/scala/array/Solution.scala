package array

import scala.collection.immutable.Set
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

  // https://leetcode.com/explore/learn/card/fun-with-arrays/526/deleting-items-from-an-array/3247/
  def removeElement(nums: Array[Int], `val`: Int): Int = {
    def go(i: Int, j: Int): Int = {
      if (j >= nums.length) i
      else
        nums(j) == `val` match {
          case false => nums(i) = nums(j); go(i+1, j+1)
          case true  => go(i, j+1)
        }
    }

    go(0, 0)
  }

  // https://leetcode.com/explore/learn/card/fun-with-arrays/527/searching-for-items-in-an-array/3250/
  def checkIfExist(arr: Array[Int]): Boolean = {
    def go(i: Int, acc: Set[Int]): Boolean = {
      if (i >= arr.length) false
      else if (acc.contains(2*arr(i)) || (arr(i) %2 == 0 && acc.contains(arr(i)/2))) true
      else go(i+1, acc + arr(i))
    }

    go(0, Set())
  }

  // https://leetcode.com/explore/learn/card/fun-with-arrays/511/in-place-operations/3259/
  def replaceElements(arr: Array[Int]): Array[Int] = {
    def go(i: Int, maxSoFar: Int): Unit = {
      if (i >= 0) {
        val prev = arr(i)
        arr(i) = maxSoFar
        go(i-1, maxSoFar.max(prev))
      }
    }

    go(arr.length-1, -1)
    arr
  }

  // https://leetcode.com/explore/learn/card/fun-with-arrays/511/in-place-operations/3258/
  def removeDuplicates(nums: Array[Int]): Int = {
    def go(write: Int, read: Int): Int = {
      if (read >= nums.length) 0
      else
        nums(read) == nums(read-1) match {
          case true  => go(write, read+1)
          case false => nums(write) = nums(read) ; 1 + go(write+1, read+1)
        }
    }

    (if (nums.isEmpty) 0 else 1) + go(1, 1)
  }

  // https://leetcode.com/explore/learn/card/fun-with-arrays/511/in-place-operations/3157/
  def moveZeroes(nums: Array[Int]): Unit = {
    def go(write: Int, read: Int): Int = {
      if (read >= nums.length) write
      else
        nums(read) == 0 match {
          case true  => go(write, read+1)
          case false => nums(write) = nums(read) ; go(write+1, read+1)
        }
    }

    (go(0, 0) until nums.length).foreach(nums(_) = 0)
  }

  // https://leetcode.com/explore/learn/card/fun-with-arrays/511/in-place-operations/3260/
  def sortArrayByParity(nums: Array[Int]): Array[Int] = {
    def swap(i: Int, j: Int): Unit = {
      val tmp = nums(i)
      nums(i) = nums(j)
      nums(j) = tmp
    }

    def go(l: Int, r: Int): Unit = {
      if (l < r)
        (nums(l) % 2 == 0) match {
          case true  => go(l+1, r)
          case false => swap(l, r) ; go(l, r-1)
        }
    }

    go(0, nums.length-1)
    nums
  }

  // https://leetcode.com/explore/learn/card/fun-with-arrays/523/conclusion/3228/
  def heightChecker(heights: Array[Int]): Int = {
    heights.zip(heights.sorted).filter{ case (x,y) => x != y }.length
  }

  // https://leetcode.com/explore/learn/card/fun-with-arrays/523/conclusion/3230/
  def findMaxConsecutiveOnes(nums: Array[Int]): Int = {
    def up(left: Int, right: Int, zeroes: Int, maxSoFar: Int): Int = {
      if (right >= nums.length) maxSoFar
      else
        nums(right) match {
          case 0 => down(left, right, zeroes+1, maxSoFar)
          case 1 => up(left, right+1, zeroes, maxSoFar.max(right+1-left))
        }
    }

    def down(left: Int, right: Int, zeroes: Int, maxSoFar: Int): Int = {
      zeroes > 1 match {
        case true  => down(left+1, right, zeroes-(nums(left)^1), maxSoFar)
        case false => up(left, right+1, zeroes, maxSoFar.max(right+1-left))
      }
    }

    up(0, 0, 0, 0)
  }
}
