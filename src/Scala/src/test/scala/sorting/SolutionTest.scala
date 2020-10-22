package sorting.test

import org.scalatest.funsuite.AnyFunSuite
import sorting.Solution

class SolutionTest extends AnyFunSuite {
  test("Solution.sortColors") {
    List(
      (Array(2,0,2,1,1,0), Array(0,0,1,1,2,2)),
      (Array(2,0,1), Array(0,1,2)),
      (Array(0), Array(0)),
      (Array(1), Array(1)),
    ).foreach{
      case (input, expected) => {
        Solution.sortColors(input)
        assert(input === expected)
      }
    }
  }
}
