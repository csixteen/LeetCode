import org.scalatest.funsuite.AnyFunSuite


class TreesTest extends AnyFunSuite {
  test("Solution.inorderTraversal") {
    assert(Solution.inorderTraversal(null) === List())
  }
}
