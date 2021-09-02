package trees


class TreeNode(_value: Int = 0, _left: TreeNode = null, _right: TreeNode = null) {
  var value: Int = _value
  var left: TreeNode = _left
  var right: TreeNode = _right
}


object Solution {
  // https://leetcode.com/problems/binary-tree-inorder-traversal/
  def inorderTraversal(root: TreeNode): List[Int] = {
    root match {
      case null => List()
      case _ => inorderTraversal(root.left) ::: List(root.value) ::: inorderTraversal(root.right)
    }
  }

  // https://leetcode.com/problems/binary-tree-preorder-traversal/
  def preorderTraversal(root: TreeNode): List[Int] = {
    root match {
      case null => List()
      case _ => List(root.value) ::: preorderTraversal(root.left) ::: preorderTraversal(root.right)
    }
  }

  // https://leetcode.com/problems/same-tree/
  def isSameTree(p: TreeNode, q: TreeNode): Boolean = {
    (p, q) match {
      case (null, null) => true
      case (null, _) => false
      case (_, null) => false
      case _ => {
        if (p.value != q.value) false
        else isSameTree(p.left, q.left) && isSameTree(p.right, q.right)
      }
    }
  }

  // https://leetcode.com/problems/kth-smallest-element-in-a-bst/
  def kthSmallest(root: TreeNode, k: Int): Int = {
    def inorder(node: TreeNode): List[Int] = {
      node match {
        case null => Nil
        case _ => {
          inorder(node.left) ::: (node.value :: inorder(node.right))
        }
      }
    }

    inorder(root).take(k).last
  }

  // https://leetcode.com/problems/unique-binary-search-trees-ii/
  def generateTrees(n: Int): List[TreeNode] = {
    def _genTrees(xs: List[Int]): List[TreeNode] = {
      xs match {
        case List() => List(null)
        case List(x) => List(new TreeNode(x))
        case _ =>
          for {
            i <- xs.indices.toList
            left <- _genTrees(xs.take(i))
            right <- _genTrees(xs.drop(i+1))
          } yield new TreeNode(xs(i), left, right)
      }
    }

    _genTrees((1 to n).toList)
  }
}
