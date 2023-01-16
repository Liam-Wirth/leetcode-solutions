 class TreeNode {
      int val;
      TreeNode left;
      TreeNode right;
      TreeNode() {}
      TreeNode(int val) { this.val = val; }
      TreeNode(int val, TreeNode left, TreeNode right) {
          this.val = val;
          this.left = left;
          this.right = right;
  }
 }
/*
 *  Maximum Searches: 3
 *      Searches used so far:
 *          1 How to navigate a binary tree
 *          ended up being a very useful search, because the way leetcode does binary trees is different from what i've seen.
 *          basically a node can only have 2 children, and the order from input goes (Root,Left,Right)
 *              So [1,2,3,null,2,1,2] would be
 *                  1
 *              2       3
 *            []  2    1  2
 *
 *
/*
 * I need to think about how I am going to navigate this tree
 * */
class Solution {
    public int maxDiff = 0;
    public int maxAncestorDiff(TreeNode root) {
        if (root == null){
            return maxDiff;
        }
        recursiveHelper(root,root.val,root.val);
        return maxDiff;
    }

    private void recursiveHelper(TreeNode root, int min, int max){
        maxDiff = Math.max(getDiff(min,max),maxDiff);
        if(root.left != null){
           int lmin = (Math.min(min,root.left.val));
           int lmax = (Math.max(max,root.left.val));
            recursiveHelper(root.left,lmin,lmax);
           
        }
        if(root.right != null){
            int rmin = (Math.min(min,root.right.val));
            int rmax = (Math.max(max,root.right.val));
            recursiveHelper(root.right,rmin,rmax);
        }
    }
    public int getDiff(int a,int b){
        return Math.abs((a-b));
    }
}
class test {
    public static void main(String[] args) {
        Solution sol = new Solution();
        TreeNode root  = new TreeNode();
    }
}
