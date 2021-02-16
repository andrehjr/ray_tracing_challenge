use raytracer::*;

#[test]
fn test_basic_matrix() {
    let matrix = Matrix::init(vec![vec![3.0, 1.0], vec![2.0, 7.0]]);

    assert_eq!(matrix.at(0, 1), 1.0);
    assert_eq!(matrix.at(1, 1), 7.0);
    assert_eq!(matrix.at(0, 0), 3.0);
    assert_eq!(matrix.at(1, 0), 2.0);
}

#[test]
fn test_matrix_multiply() {
    // 0, 0 ->  1.0 * -2.0 + 2.0 * 3.0 + 3.0 * 4.0 + 4.0 * 1.0
    //
    let matrix_a = Matrix::init(vec![
        vec![1.0, 2.0, 3.0, 4.0],
        vec![5.0, 6.0, 7.0, 8.0],
        vec![9.0, 8.0, 7.0, 6.0],
        vec![5.0, 4.0, 3.0, 2.0],
    ]);

    let matrix_b = Matrix::init(vec![
        vec![-2.0, 1.0, 2.0, 3.0],
        vec![3.0, 2.0, 1.0, -1.0],
        vec![4.0, 3.0, 6.0, 5.0],
        vec![1.0, 2.0, 7.0, 8.0],
    ]);

    let matrix_result = Matrix::init(vec![
        vec![20.0, 22.0, 50.0, 48.0],
        vec![44.0, 54.0, 114.0, 108.0],
        vec![40.0, 58.0, 110.0, 102.0],
        vec![16.0, 26.0, 46.0, 42.0],
    ]);

    assert_eq!((matrix_a * matrix_b).matrix, matrix_result.matrix);
}

#[test]
fn test_matrix_multiple_tuple() {
    let matrix_a = Matrix::init(vec![
        vec![1.0, 2.0, 3.0, 4.0],
        vec![2.0, 4.0, 4.0, 2.0],
        vec![8.0, 6.0, 4.0, 1.0],
        vec![0.0, 0.0, 0.0, 1.0],
    ]);

    let tuple = Tuple {
        x: 1.0,
        y: 2.0,
        z: 3.0,
        w: 1.0,
    };

    let result = Tuple {
        x: 18.0,
        y: 24.0,
        z: 33.0,
        w: 1.0,
    };

    assert_eq!(matrix_a * tuple, result);
}

#[test]
fn test_matrix_identity() {
    let matrix_a = Matrix::init(vec![
        vec![1.0, 0.0, 0.0, 0.0],
        vec![0.0, 1.0, 0.0, 0.0],
        vec![0.0, 0.0, 1.0, 0.0],
        vec![0.0, 0.0, 0.0, 1.0],
    ]);

    let identity = Matrix::identity(4);

    assert_eq!(matrix_a, identity);
}

#[test]
fn test_matrix_identity_multiply() {
    let matrix_a = Matrix::init(vec![
        vec![1.0, 2.0, 3.0, 4.0],
        vec![2.0, 4.0, 4.0, 2.0],
        vec![8.0, 6.0, 4.0, 1.0],
        vec![0.0, 0.0, 0.0, 1.0],
    ]);

    let identity = Matrix::identity(4);
    let result = identity * matrix_a.clone();
    assert_eq!(matrix_a, result);
}
