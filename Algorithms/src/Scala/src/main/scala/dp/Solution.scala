package dp


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
}
