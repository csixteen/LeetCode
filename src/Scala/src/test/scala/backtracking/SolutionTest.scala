package backtracking.test

import org.scalatest.funsuite.AnyFunSuite
import backtracking.Solution

class SolutionTest extends AnyFunSuite {
  test("Solution.letterCombinations") {
    assert(
      Solution.letterCombinations("23") == 
        List("ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf")
    )
  }

  test("Solution.generateParenthesis") {
    assert(Solution.generateParenthesis(1) == List("()"))
    assert(Solution.generateParenthesis(3) ==
      List("((()))", "(()())", "(())()", "()(())", "()()()"))
  }

  test("Solution.permute") {
    assert(Solution.permute(Array(1, 2, 3)) ==
      List(
        List(1, 2, 3),
        List(1, 3, 2),
        List(2, 1, 3),
        List(2, 3, 1),
        List(3, 1, 2),
        List(3, 2, 1)))
  }
}
