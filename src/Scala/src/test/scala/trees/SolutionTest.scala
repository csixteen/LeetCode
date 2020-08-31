import org.scalatest.funsuite.AnyFunSuite
import trees.{TreeNode, Solution}


class SolutionTest extends AnyFunSuite {
  test("Solution.inorderTraversal empty tree") {
    assert(Solution.inorderTraversal(null) === List())
  }
}
