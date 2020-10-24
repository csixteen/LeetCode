package sorting.test

import org.scalatest.funsuite.AnyFunSuite
import sorting.Solution._

class SolutionTest extends AnyFunSuite {
  test("Solution.sortColors") {
    List(
      (Array(2,0,2,1,1,0), Array(0,0,1,1,2,2)),
      (Array(2,0,1), Array(0,1,2)),
      (Array(0), Array(0)),
      (Array(1), Array(1)),
    ).foreach{
      case (input, expected) => {
        sortColors(input)
        assert(input === expected)
      }
    }
  }

  test("Solution.topKFrequent") {
    assert(topKFrequent(Array(1, 1, 1, 2, 2, 3), 2) === Array(1, 2))
    assert(topKFrequent(Array(1), 1) === Array(1))
  }

  test("Solution.findKthLargest") {
    assert(findKthLargestNaive(Array(3, 2, 1, 5, 6, 4), 2) == 5)
    assert(findKthLargestMinHeap(Array(3, 2, 1, 5, 6, 4), 2) == 5)
  }
}
