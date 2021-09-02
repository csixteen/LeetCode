package array.test


import org.scalatest.funsuite.AnyFunSuite
import array.Solution._


class SolutionTest extends AnyFunSuite {
  test("Find Numbers with Even Number of Digits") {
    assert(findNumbers(Array(12,345,2,6,7896)) == 2)
    assert(findNumbers(Array(555,901,482,1771)) == 1)
  }
}
