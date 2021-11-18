package trees.test

import org.scalatest.funsuite.AnyFunSuite
import trees.{TreeNode, Solution}

class SolutionTest extends AnyFunSuite {
  test("Solution.inorderTraversal empty tree") {
    assert(Solution.inorderTraversal(null) === List())
  }

  test("Solution.inorderTraversal tree not empty") {
    assert(
      Solution.inorderTraversal(
        new TreeNode(1, _right = new TreeNode(2, _left = new TreeNode(3)))
      ) === List(1, 3, 2)
    )
  }

  test("Solution.preorderTraversal empty tree") {
    assert(Solution.preorderTraversal(null) == List())
  }

  test("Solution.preorderTraversal tree not empty") {
    assert(
      Solution.preorderTraversal(
        new TreeNode(1, _right = new TreeNode(2, _left = new TreeNode(3)))
      ) === List(1, 2, 3)
    )
  }

  test("Solution.isSameTree same trees") {
    assert(
      Solution.isSameTree(
        new TreeNode(1, new TreeNode(2), new TreeNode(3)),
        new TreeNode(1, new TreeNode(2), new TreeNode(3))
      )
    )
  }

  test("Solution.isSameTree different trees 1") {
    assert(
      !Solution.isSameTree(
        new TreeNode(1, _left = new TreeNode(2)),
        new TreeNode(1, _right = new TreeNode(2))
      )
    )
  }

  test("Solution.isSameTree different trees 2") {
    assert(
      !Solution.isSameTree(
        new TreeNode(1, new TreeNode(2), new TreeNode(1)),
        new TreeNode(1, new TreeNode(1), new TreeNode(2))
      )
    )
  }

  test("Solution.kthSmallest") {
    assert(
      Solution.kthSmallest(
        new TreeNode(
          3,
          _left = new TreeNode(1, _right = new TreeNode(2)),
          _right = new TreeNode(4)
        ),
        1
      ) == 1
    )

    assert(
      Solution.kthSmallest(
        new TreeNode(
          5,
          _left = new TreeNode(
            3,
            _left = new TreeNode(2, _left = new TreeNode(1)),
            _right = new TreeNode(4)
          ),
          _right = new TreeNode(6)
        ),
        3
      ) == 3
    )
  }

  test("Solution.generateTrees") {
    assert(
      (
        Solution.generateTrees(3),
        List(
          new TreeNode(
            1,
            _right = new TreeNode(2, _right = new TreeNode(3))
          ),
          new TreeNode(
            1,
            _right = new TreeNode(3, _left = new TreeNode(2))
          ),
          new TreeNode(
            2,
            _left = new TreeNode(1),
            _right = new TreeNode(3)
          ),
          new TreeNode(
            3,
            _left = new TreeNode(1, _right = new TreeNode(2))
          ),
          new TreeNode(
            3,
            _left = new TreeNode(2, _left = new TreeNode(1))
          )
        )
      ).zipped.map(Solution.isSameTree(_, _)).foldLeft(true)(_ && _)
    )
  }

  test("Solution._inorderMap") {
    assert(
      Solution._inorderMap(
        new TreeNode(1, _right = new TreeNode(2, _left = new TreeNode(2))),
        Map[Int, Int]()
      ) == Map(1 -> 1, 2 -> 2)
    )

    assert(
      Solution._inorderMap(
        new TreeNode(
          1,
          new TreeNode(1),
          new TreeNode(
            2,
            new TreeNode(2),
            new TreeNode(3)
          )
        ),
        Map[Int, Int]()
      ) == Map(1 -> 2, 2 -> 2, 3 -> 1)
    )
  }

  test("Solution.findMode") {
    assert(
      Solution.findMode(
        new TreeNode(1, _right = new TreeNode(2, _left = new TreeNode(2)))
      ) === Array(2)
    )

    assert(
      Solution.findMode(new TreeNode(0)) === Array(0)
    )

    assert(
      Solution.findMode(
        new TreeNode(
          1,
          new TreeNode(1),
          new TreeNode(
            2,
            new TreeNode(2),
            new TreeNode(3)
          )
        )
      ) === Array(1, 2)
    )
  }
}
