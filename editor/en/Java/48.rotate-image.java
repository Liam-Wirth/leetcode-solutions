class Solution {
    //NOTE: must modify the matrix in place, no allocation of another matrix
    public Solution(){

    }
    public void rotate(int[][] matrix) {
        int size = matrix.length;
        int holder;
        for (int i = 0; i < size; i++) {
            //NOTE: J is kept at a size less than i because I is already being compared to the size of the array, and if i == j, the operation would be performed on the square later, as it would get swapped with a later element that has a coordinate in which j<i 
            for (int j = 0; j < i; j++) {
                holder = matrix[i][j];
                matrix[i][j]=matrix[j][i];
                matrix[j][i]=holder;
            }
            
        }
        //now I mirror the array
        for(int i = 0; i<size;i++){
            //NOTE: On this interior loop, it only needs to be less than the size/2 because otherwise, I would perform the swap twice, effectively making the part of the code do nothing.
            for (int j = 0; j < size/2; j++) {
                holder = matrix[i][j];
                matrix[i][j] = matrix[i][size-j-1];
                matrix[i][size-j-1]=holder;
            }
        }
    }
}

class test {
    public static void main(String[] args) {
        Solution sol = new Solution();
        int[][] mat = {{1,2,3},{4,5,6},{7,8,9}};
        for (int i = 0; i < mat.length; i++) {
            System.out.println();
            for (int j = 0; j < mat[0].length; j++) {
               System.out.print(mat[i][j]); 
            }
            
        }
        sol.rotate(mat);
        for (int i = 0; i < mat.length; i++) {
            System.out.println();
            for (int j = 0; j < mat[0].length; j++) {
               System.out.print(mat[i][j]); 
            }
            
        }

    }
}
