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
fn test_matrix_cofactor() {
    let matrix_a = matrix![3.0, 5.0, 0.0;
						   2.0,-1.0,-7.0;
						   6.0,-1.0, 5.0];

    assert_eq!(-12.0, matrix_a.minor(0, 0));
    assert_eq!(-12.0, matrix_a.cofactor(0, 0));

    assert_eq!(25.0, matrix_a.minor(1, 0));
    assert_eq!(-25.0, matrix_a.cofactor(1, 0));
}

#[test]
fn test_determinant_3x3() {
    let matrix_a = matrix![1.0, 2.0, 6.0;
						   -5.0,8.0,-4.0;
						   2.0,6.0, 4.0];

    assert_eq!(-196.0, matrix_a.determinant());
}

#[test]
fn test_inverse_matrix() {
    let matrix_a = matrix![-5.0,2.0,6.0,-8.0;
							1.0,-5.0,1.0,8.0;
							7.0,7.0,-6.0,-7.0;
							1.0,-3.0,7.0, 4.0];

    assert_eq!(
        matrix![0.21804511278195488, 0.45112781954887216, 0.24060150375939848, -0.045112781954887216;
                       -0.8082706766917294, -1.4567669172932332, -0.44360902255639095, 0.5206766917293233;
                       -0.07894736842105263, -0.2236842105263158, -0.05263157894736842, 0.19736842105263158;
					   -0.5225563909774437, -0.8139097744360902, -0.3007518796992481, 0.30639097744360905],
        matrix_a.inverse()
    );
}

#[test]
fn test_inverse_matrix_more_samples() {
    let matrix_a = matrix![8.0, -5.0, 9.0, 2.0;
							7.0, 5.0, 6.0, 1.0;
							-6.0, 0.0, 9.0, 6.0;
							-3.0, 0.0, -9.0, -4.0];

    assert_eq!(
        matrix![-0.15385, -0.15385, -0.28205, -0.53846;
               -0.07692, 0.12308, 0.02564, 0.03077;
               0.35897, 0.35897, 0.43590, 0.92308;
			   -0.69231, -0.69231, -0.76923, -1.92308],
        matrix_a.inverse()
    );

    let matrix_b = matrix![9.0, 3.0, 0.0, 9.0;
							-5.0, -2.0, -6.0, -3.0;
							-4.0, 9.0, 6.0, 4.0;
							-7.0, 6.0, 6.0, 2.0];

    assert_eq!(
        matrix![ -0.04074, -0.07778,  0.14444, -0.22222;
                 -0.07778,  0.03333,  0.36667, -0.33333;
                 -0.02901, -0.14630, -0.10926,  0.12963;
				  0.17778,  0.06667, -0.26667,  0.33333],
        matrix_b.inverse()
    );
}

#[test]
fn test_inverse_matrix_multiplication() {
    let matrix_a = matrix![3.0, -9.0, 7.0, 3.0;
						   3.0, -8.0, 2.0, -9.0;
						   -4.0, 4.0, 4.0, 1.0;
						   -6.0, 5.0, -1.0, 1.0];

    let matrix_b = matrix![8.0, 2.0, 2.0, 2.0;
							3.0, -1.0, 7.0, 0.0;
							7.0, 0.0, 5.0, 4.0;
							6.0, -2.0, 0.0, 5.0];

    let a_x_b = matrix_a.clone() * matrix_b.clone();

    assert_eq!(matrix_a, a_x_b * matrix_b.inverse());
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
