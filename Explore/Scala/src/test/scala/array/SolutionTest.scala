package array.test


import org.scalatest.funsuite.AnyFunSuite
import array.Solution._


class SolutionTest extends AnyFunSuite {
  test("Find Numbers with Even Number of Digits") {
    assert(findNumbers(Array(12,345,2,6,7896)) == 2)
    assert(findNumbers(Array(555,901,482,1771)) == 1)
  }

  test("Squares of a Sorted Array") {
    assert(sortedSquares(Array(-4, -1, 0, 3, 10)) === Array(0, 1, 9, 16, 100))
    assert(sortedSquares(Array(-7, -3, 2, 3, 11)) === Array(4, 9, 9, 49, 121))
  }

  test("Duplicate Zeros") {
    var example1 = Array(1, 0, 2, 3, 0, 4, 5, 0)
    duplicateZeros(example1)
    assert(example1 === Array(1, 0, 0, 2, 3, 0, 0, 4))

    var example2 = Array(1, 2, 3)
    duplicateZeros(example2)
    assert(example2 === Array(1, 2, 3))

    var example3 = Array(0, 0, 1)
    duplicateZeros(example3)
    assert(example3 === Array(0, 0, 0))

    var example4 = Array(8, 4, 5, 0, 0, 0, 0, 7)
    duplicateZeros(example4)
    assert(example4 === Array(8, 4, 5, 0, 0, 0, 0, 0))
  }

  test("Remove Element") {
    var example1 = Array(3, 2, 2, 3)
    assert(removeElement(example1, 3) == 2)

    var example2 = Array(0, 1, 2, 2, 3, 0, 4, 2)
    assert(removeElement(example2, 2) == 5)

    var example3 = Array(3, 3)
    assert(removeElement(example3, 3) == 0)

    var example4 = Array(2)
    assert(removeElement(example4, 3) == 1)
  }
}
