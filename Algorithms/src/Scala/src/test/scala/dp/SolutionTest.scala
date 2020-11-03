package dp.test


import org.scalatest.funsuite.AnyFunSuite
import dp.Solution._


class SolutionTest extends AnyFunSuite {
  test("Solution.canJump") {
    assert(canJump(Array(2, 3, 1, 1, 4)))
    assert(!canJump(Array(3, 2, 1, 0, 4)))
  }

  test("Solution.uniquePaths") {
    assert(uniquePaths(3, 7) == 28)
    assert(uniquePaths(3, 2) == 3)
    assert(uniquePaths(7, 3) == 28)
    assert(uniquePaths(3, 3) == 6)
  }
}
