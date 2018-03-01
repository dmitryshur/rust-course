pub type Matrix = Vec<Vec<f32>>;

pub fn mat_match(mat1: &Matrix, mat2: &Matrix) -> bool {
    mat1[0].iter().count() == mat2.iter().count()
}

pub fn mat_mult(mat1: &Matrix, mat2: &Matrix) -> Matrix {
    let mut new_matrix: Matrix = vec![];
    for first_row in 0..mat1.len() {
        let mut sum_vec = vec![];
        for column_counter in 0..mat2[0].len() {
            let mut sum = 0.;
            for j in 0..mat2.len() {
                sum = sum + mat1[first_row][j] * mat2[j][column_counter];
            }
            sum_vec.push(sum);
        }
        new_matrix.push(sum_vec);
    }
    new_matrix
}
