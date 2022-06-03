  
  package com.liam.leetcode;

  import java.util.Arrays;

  public class RangeSumQuery2dImmutable{
      public static void main(String[] args) {
           Solution solution = new RangeSumQuery2dImmutable().new Solution();
      }
      //leetcode submit region begin(Prohibit modification and deletion)
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
//leetcode submit region end(Prohibit modification and deletion)

  }