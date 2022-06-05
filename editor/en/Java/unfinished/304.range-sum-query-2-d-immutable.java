package unfinished;
/*
 * @lc app=leetcode id=304 lang=java
 *
 * [304] Range Sum Query 2D - Immutable
 */

// @lc code=start
 class NumMatrix {
          private int[][] matrix;
          private int sum;
          public NumMatrix(int[][] matrix) {
              this.matrix = matrix;
              sum = 0;
          }

          public int sumRegion(int row1, int col1, int row2, int col2) {
              for (int a = row1; a <= row2 ; a++) {
                  for (int b = col1; b <= col2 ; b++) {
                      sum+=matrix[a][b];
                  }
              }
              return sum;
          }
        }      

/**
 * Your NumMatrix object will be instantiated and called as such:
 * NumMatrix obj = new NumMatrix(matrix);
 * int param_1 = obj.sumRegion(row1,col1,row2,col2);
 */
// @lc code=end

