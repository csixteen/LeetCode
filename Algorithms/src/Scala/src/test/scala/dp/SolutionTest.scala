package dp.test


import org.scalatest.funsuite.AnyFunSuite
import dp.Solution._


class SolutionTest extends AnyFunSuite {
  test("Solution.canJump") {
    assert(canJump(Array(2, 3, 1, 1, 4)))
    assert(!canJump(Array(3, 2, 1, 0, 4)))
  }
}
