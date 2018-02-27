pub type Matrix = Vec<Vec<f32>>;

pub fn mat_match(mat1: &Matrix, mat2: &Matrix) -> bool {
    mat1[0].iter().count() == mat2.iter().count()
}

pub fn mat_mult(mat1: &Matrix, mat2: &Matrix) -> Matrix {
    unimplemented!()
}
