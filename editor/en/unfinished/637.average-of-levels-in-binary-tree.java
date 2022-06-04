package unfinished;
import java.util.Arrays;

import javax.swing.tree.TreeNode;

/*
 * @lc app=leetcode id=637 lang=java
 *
 * [637] Average of Levels in Binary Tree
 */

// @lc code=start
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
    private int childCount;

    public List<Double> averageOfLevels(TreeNode root) {
         childCount = root.getChildCount();
         double[] sol = new double[childCount+1];
    //     double temp;
    //     for (int i = 0; i <childCount +1; i++) {
    //     TreeNode root2 = root;
    //     temp+= root2.val;
    //     short counter;
    //     counter++;
    //    while(root2.left != null) {
    //             temp+=root2.left.val;
    //             counter++;
    //             root2=root2.left;
    //    }    
    //    root2=root;
    //     while(root2.right != null) {
    //             temp+=root2.right.val; 
    //             counter++;
    //             root2 = root2.right;
    //     }
    //     sol[i] =  temp/counter;
    //     root = root.getChildAt(i+1);
    //     }
    //     return Arrays.asList(sol);
    //    }
    
    Enumeration<TreeNode> treenum =root.children();
    
    do {

        
    } while (condition); 

    }
// @lc code=end

