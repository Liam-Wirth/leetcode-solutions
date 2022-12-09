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
    //NOTE: to self, I was stuck cause I sent Calls++ to the method, and the thing wouldn't work, but it worked when I sent calls+1 to the method, not sure why that is but now I know to never use ++ when I want to increment the variable in a method call.
    invertTreeRec(left.left,right.right,calls+1);
    invertTreeRec(left.right,right.left,calls+1); 
}
}
