  

   class MinimumAreaRectangleIi {
      public static void main(String[] args) {
          Solution solution = new MinimumAreaRectangleIi().new Solution();
      }
      //leetcode submit region begin(Prohibit modification and deletion)


      //find 2 points, and then use pythagoras?

      class Solution {
          // @PARAM points need to be inserted as an ordered pair
          // this is a function that should be able to return distance between 2 points
          public double distance(int[] p1, int[] p2) {
              //p1[0] = x p1[1] = y etc
              int deltaY = p1[1] - p2[1];
              int deltaX = p1[0] - p2[0];
              return (double) deltaY / deltaX;
          }

          public double minAreaFreeRect(int[][] points) {

              for (int row = 0; row < points[0].length; row++) {
                  for (int col = 0; col < points.length; col++) {
                      System.out.println(points[row][col]);
                  }

              }
              return 0;
          }

      }

//leetcode submit region end(Prohibit modification and deletion)
  }