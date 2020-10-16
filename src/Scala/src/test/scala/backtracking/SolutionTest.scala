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
}
