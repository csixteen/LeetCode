package dp

import scala.collection.mutable.HashMap


object Solution {
  // https://leetcode.com/problems/jump-game/
  def canJump(nums: Array[Int]): Boolean = {
    @annotation.tailrec
    def go(i: Int, maxJump: Int): Option[Int] = {
      if (i >= nums.length-1) Some(maxJump)
      else if(i > maxJump) None
      else go(i+1, maxJump.max(i+nums(i)))
    }

    go(0, 0) match {
      case Some(maxJump) => nums.length-1 <= maxJump
      case None => false
    }
  }


  //-------------------------------------------------

  // https://leetcode.com/problems/unique-paths/
  def uniquePaths(m: Int, n: Int): Int = {
    (0 until n-1).foldLeft(List.fill(m)(1))(
      (acc, _) => acc.scanLeft(0)(_+_).tail
    ).last
  }

  //-------------------------------------------------

  // https://leetcode.com/problems/interleaving-string/
  def isInterleave(s1: String, s2: String, s3: String): Boolean = {
    def go(i: Int, j: Int, k: Int, dp: HashMap[(Int,Int),Boolean]): Boolean = {
      (i == s1.length, j == s2.length) match {
        case (true, _) => s2.substring(j) == s3.substring(k)
        case (_, true) => s1.substring(i) == s3.substring(k)
        case _         =>
          dp.getOrElseUpdate((i, j),
            ((s3(k) == s1(i)) && go(i+1, j, k+1, dp)) ||
            ((s3(k) == s2(j)) && go(i, j+1, k+1, dp)))
      }
    }

    (s1.length + s2.length == s3.length) && go(0, 0, 0, new HashMap[(Int,Int),Boolean]())
  }
}
