import org.scalatest.funsuite.AnyFunSuite
import trees.{TreeNode, Solution}


class SolutionTest extends AnyFunSuite {
  test("Solution.inorderTraversal empty tree") {
    assert(Solution.inorderTraversal(null) === List())
  }

  test("Solution.inorderTraversal tree not empty") {
    assert(Solution.inorderTraversal(
      new TreeNode(1, _right = new TreeNode(2, _left = new TreeNode(3)))
    ) === List(1, 3, 2))
  }

  test("Solution.preorderTraversal empty tree") {
    assert(Solution.preorderTraversal(null) == List())
  }

  test("Solution.preorderTraversal tree not empty") {
    assert(Solution.preorderTraversal(
      new TreeNode(1, _right = new TreeNode(2, _left = new TreeNode(3)))
    ) === List(1, 2, 3))
  }
}
