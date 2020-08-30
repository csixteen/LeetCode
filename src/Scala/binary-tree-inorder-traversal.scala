// https://leetcode.com/problems/binary-tree-inorder-traversal/

class TreeNode(_value: Int = 0, _left: TreeNode = null, _right: TreeNode = null) {
  var value: Int = _value
  var left: TreeNode = _left
  var right: TreeNode = _right
}


object Solution {
  def inorderTraversal(root: TreeNode): List[Int] = {
    root match {
      case null => List()
      case _ => inorderTraversal(root.left) ::: List(root.value) ::: inorderTraversal(root.right)
    }
  }
}
