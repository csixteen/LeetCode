package array


object Solution {
  // https://leetcode.com/problems/fibonacci-number/
  val fibs: LazyList[Int] = 0 #:: 1 #:: fibs.lazyZip(fibs.tail).map(_ + _)

  def fib(N: Int): Int = fibs.drop(N).head
}
