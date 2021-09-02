package array

object Solution {
  def findNumbers(nums: Array[Int]): Int = {
    @annotation.tailrec
    def numDigits(n: Int, acc: Int): Int = {
      if (n == 0) acc
      else numDigits(n / 10, acc+1)
    }

    nums.filter(numDigits(_, 0) % 2 == 0).length
  }
}
