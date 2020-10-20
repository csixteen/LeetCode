package backtracking

import scala.collection.mutable.ListBuffer

object Solution {
  private def combinations(as: List[String]): List[List[Char]] = {
    as match {
      case Nil => List()
      case List(x) => x.map(List(_)).toList
      case h::t => {
        var withoutHead = combinations(t)
        h.flatMap(x => withoutHead.map(x::_)).toList
      }
    }
  }

  // https://leetcode.com/problems/letter-combinations-of-a-phone-number/
  def letterCombinations(digits: String): List[String] = {
    val pad = Map(
      '2' -> "abc",
      '3' -> "def",
      '4' -> "ghi",
      '5' -> "jkl",
      '6' -> "mno",
      '7' -> "pqrs",
      '8' -> "tuv",
      '9' -> "wxyz"
    )

    combinations(digits.map(pad(_)).toList).map(_.mkString)
  }

  // https://leetcode.com/problems/generate-parentheses/
  def generateParenthesis(n: Int): List[String] = {
    def go(acc: ListBuffer[String], curr: String, left: Int, right: Int): Unit = {
      if (left == 0 && right == 0) acc.append(curr)
      else {
        if (left > 0) go(acc, curr + "(", left-1, right)
        if (right > left) go(acc, curr + ")", left, right-1)
      }
    }

    val acc = ListBuffer[String]()

    go(acc, "", n, n)

    acc.toList
  }

  // https://leetcode.com/problems/permutations/
  def permute(nums: Array[Int]): List[List[Int]] = {
    def go(as: List[Int]): List[List[Int]] = {
      if (as.isEmpty) List(List())
      else as.flatMap(c => {
        val cIdx = as.indexOf(c)
        val without = as.take(cIdx) ::: as.drop(cIdx+1)
        go(without).map(c::_)
      })
    }

    go(nums.toList)
  }

  // https://leetcode.com/problems/subsets/
  def subsets(nums: Array[Int]): List[List[Int]] = {
    @annotation.tailrec
    def go(as: List[Int], acc: List[List[Int]]): List[List[Int]] = {
      as match {
        case Nil => acc
        case h::t => go(t, acc ::: acc.map(_.appended(h)))
      }
    }

    go(nums.toList, List(List()))
  }

  // https://leetcode.com/problems/word-search/
  // The original method is called `exist`, which is a poor choice of name
  def existsWord(board: Array[Array[Char]], word: String): Boolean = {
    def go(from: Int, i: Int, j: Int, visited: Set[(Int, Int)]): Boolean = {
      if (from >= word.length) true
      else if (
        i < 0 ||
        i >= board.length ||
        j < 0 ||
        j >= board(0).length ||
        visited.exists(_ == (i,j)) ||
        word(from) != board(i)(j)
      ) false
      else {
        val newVisited = visited + ((i,j))
        val newFrom = from+1

        go(newFrom, i, j+1, newVisited) ||
        go(newFrom, i, j-1, newVisited) ||
        go(newFrom, i+1, j, newVisited) ||
        go(newFrom, i-1, j, newVisited)
      }
    }

    for {
      i <- (0 to board.length-1)
      j <- (0 to board(0).length-1)
    } yield {
      if (go(0, i, j, Set())) return true
    }

    false
  }
}
