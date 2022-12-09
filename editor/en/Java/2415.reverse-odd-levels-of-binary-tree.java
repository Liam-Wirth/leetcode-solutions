/**
 * Definition for a binary tree node.
 * public class TreeNode {
 *     int val;
 *     TreeNode left;
 *     TreeNode right;
 *     TreeNode() {}
 *     TreeNode(int val) { this.val = val; }
 *     TreeNode(int val, TreeNode left, TreeNode right) {
 *         this.val = val;
 *         this.left = left;
 *         this.right = right;
 *     }
 * }
 */
class Solution {
    public TreeNode reverseOddLevels(TreeNode root) {
       invertTreeRec(root.left,root.right,1);
       return root;
    }
private void invertTreeRec(TreeNode left, TreeNode right, int calls){
    
    if(left == null || right == null) return;
    
    if(calls % 2 == 1 ){
    int temp = left.val;
    left.val = right.val;
    right.val = temp;
    }
    invertTreeRec(left.left,right.right,calls+1);
    invertTreeRec(left.right,right.left,calls+1); 
}
}
