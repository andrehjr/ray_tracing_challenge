use raytracer::*;

#[test]
fn test_basic_matrix() {
    let matrix = matrix![3.0, 1.0 ; 2.0, 7.0];

    assert_eq!(matrix.at(0, 1), 1.0);
    assert_eq!(matrix.at(1, 1), 7.0);
    assert_eq!(matrix.at(0, 0), 3.0);
    assert_eq!(matrix.at(1, 0), 2.0);
}

#[test]
fn test_matrix_multiply() {
    // 0, 0 ->  1.0 * -2.0 + 2.0 * 3.0 + 3.0 * 4.0 + 4.0 * 1.0
    //
    let matrix_a = matrix![1.0, 2.0, 3.0, 4.0; 
    					   5.0, 6.0, 7.0, 8.0;
    					   9.0, 8.0, 7.0, 6.0;
    					   5.0, 4.0, 3.0, 2.0];

    let matrix_b = matrix![-2.0, 1.0, 2.0, 3.0;
				            3.0, 2.0, 1.0, -1.0;
				            4.0, 3.0, 6.0, 5.0;
				            1.0, 2.0, 7.0, 8.0];

    let matrix_result = matrix![20.0, 22.0, 50.0, 48.0;
						        44.0, 54.0, 114.0, 108.0;
						        40.0, 58.0, 110.0, 102.0;
						        16.0, 26.0, 46.0, 42.0];

    assert_eq!((matrix_a * matrix_b).matrix, matrix_result.matrix);
}

#[test]
fn test_matrix_multiple_tuple() {
    let matrix_a = matrix![
        1.0, 2.0, 3.0, 4.0;
        2.0, 4.0, 4.0, 2.0;
        8.0, 6.0, 4.0, 1.0;
        0.0, 0.0, 0.0, 1.0];

    let tuple = point!(1.0, 2.0, 3.0);
    let result = point!(18.0, 24.0, 33.0);

    assert_eq!(matrix_a * tuple, result);
}

#[test]
fn test_matrix_identity() {
    let matrix_a = matrix![1.0, 0.0, 0.0, 0.0;
                           0.0, 1.0, 0.0, 0.0;
					       0.0, 0.0, 1.0, 0.0;
					       0.0, 0.0, 0.0, 1.0];

    let identity = Matrix::identity(4);

    assert_eq!(matrix_a, identity);
}

#[test]
fn test_matrix_identity_multiply() {
    let matrix_a = matrix![1.0, 2.0, 3.0, 4.0;
				           2.0, 4.0, 4.0, 2.0;
				           8.0, 6.0, 4.0, 1.0;
				           0.0, 0.0, 0.0, 1.0];

    let identity = Matrix::identity(4);
    let result = identity * matrix_a.clone();
    assert_eq!(matrix_a, result);
}

#[test]
fn test_matrix_determinant_2x2() {
    let matrix_a = matrix![1.0, 5.0; -3.0, 2.0];

    assert_eq!(17.0, matrix_a.determinant());
}

#[test]
fn test_matrix_submatrixes() {
    let matrix_a = matrix![ 1.0, 5.0, 0.0;
						   -3.0, 2.0, 7.0;
						    0.0, 6.0, -3.0];

    assert_eq!(matrix![-3.0, 2.0; 0.0, 6.0], matrix_a.submatrix(0, 2));

    let matrix_b = matrix![-6.0, 1.0, 1.0, 6.0;
						   -8.0, 5.0, 8.0, 6.0;
						   -1.0, 0.0, 8.0, 2.0;
						   -7.0, 1.0, -1.0, 1.0];
    assert_eq!(
        matrix![-6.0, 1.0, 6.0; -8.0, 8.0, 6.0; -7.0, -1.0, 1.0],
        matrix_b.submatrix(2, 1)
    );
}

#[test]
fn test_minor_matrix() {
    let matrix_a = matrix![3.0, 5.0, 0.0;
						   2.0,-1.0,-7.0;
						   6.0,-1.0, 5.0];

    assert_eq!(25.0, matrix_a.minor(1, 0));
}

#[test]
fn test_transpose() {
    let matrix_a = matrix![0.0, 9.0, 3.0, 0.0;
						   9.0, 8.0, 0.0, 8.0;
						   1.0, 8.0, 5.0, 3.0;
						   0.0, 0.0, 5.0, 8.0 ];

    let transposed = matrix![0.0, 9.0, 1.0, 0.0;
				 			 9.0, 8.0, 8.0, 0.0;
			 				 3.0, 0.0, 5.0, 5.0;
							 0.0, 8.0, 3.0, 8.0];

    assert_eq!(matrix_a.transpose(), transposed);
    assert_eq!(Matrix::identity(4), Matrix::identity(4).transpose());
}
