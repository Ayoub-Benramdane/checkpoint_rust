pub fn matrix_determinant(matrix: [[isize; 3]; 3]) -> isize {
    let a = matrix[0][0] * (matrix[1][1] * matrix[2][2] - matrix[1][2] * matrix[2][1]);
    let b = matrix[0][1] * (matrix[0][1] * matrix[2][2] - matrix[0][2] * matrix[2][1]);
    let c = matrix[0][2] * (matrix[0][1] * matrix[1][2] - matrix[1][1] * matrix[1][2]);
    a - b + c
}