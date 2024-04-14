use crate::Solution;
impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let mut row_max = matrix.len();
        let mut col_max = matrix[0].len();
        if row_max == 0 {
            return 0;
        }

        let mut out = 0;

        for row in 0..row_max {
            for col in 0..col_max {
                let mut edge = col_max;
                let mut area = 0;
                for i in row..row_max {
                    for j in col..edge {
                        if matrix[i][j] == '1' {
                            continue;
                        } else {
                            edge = j;
                            break;
                        }
                    }
                    area = area.max((edge - col) * (i - row + 1));
                }
                out = out.max(area);
            }
        }
        out as i32
    }
}
