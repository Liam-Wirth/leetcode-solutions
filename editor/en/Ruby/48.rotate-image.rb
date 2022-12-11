# @param {Integer[][]} matrix
# @return {Void} Do not return anything, modify matrix in-place instead.
    
def rotate(matrix)
   n = matrix.length
   transpose(matrix, n)
   mirror(matrix, n)
end 
 def transpose(matrix, n)
  n.times do |i|
       (i...n).step(1) do |j|
          matrix[i][j], matrix[j][i] = matrix[j][i], matrix[i][j] 
       end
    end
  matrix
 end
 def mirror(matrix,n)
  (0...n).each { |i|
    (0...n/2).each { |j|
      matrix[i][j],matrix[i][n-j-1] = matrix[i][n-j-1],matrix[i][j]
    }
  }
  matrix
 end
