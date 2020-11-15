package array.test


import org.scalatest.funsuite.AnyFunSuite
import array.Solution._


class SolutionTest extends AnyFunSuite {
  test("Solution.fib") {
    assert(fib(2) == 1)
    assert(fib(3) == 2)
    assert(fib(4) == 3)
  }
}
