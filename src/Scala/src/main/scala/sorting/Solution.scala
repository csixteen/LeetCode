package sorting

object Solution {
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
}
