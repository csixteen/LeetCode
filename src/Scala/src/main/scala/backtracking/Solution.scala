package backtracking

object Solution {
  // https://leetcode.com/problems/letter-combinations-of-a-phone-number/
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
}
