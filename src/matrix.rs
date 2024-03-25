use crate::tuple::Tuple;
use std::ops::Mul;

#[derive(Debug)]
pub struct Matrix {
    pub matrix: Vec<Vec<f64>>,
}

#[macro_export]
macro_rules! matrix {
    () => {
        {
            Matrix::init(vec![])
        }
    };
    ($( $( $x: expr ),*);*) => {
        {
            let arrays = [ $( [ $($x),* ] ),* ];

            let data: Vec<_> = arrays.iter()
                .map(|row| row.to_vec())
                .collect();

            Matrix::init(data)
        }
    }
}

impl Matrix {
    pub fn init(matrix: Vec<Vec<f64>>) -> Self {
        Self { matrix }
    }

    pub fn identity(size: usize) -> Self {
        let mut matrix = vec![vec![0.0; size]; size];

        for (pos, item) in matrix.iter_mut().enumerate().take(size) {
            item[pos] = 1.0
        }

        Self { matrix }
    }

    pub fn at(&self, x: usize, y: usize) -> f64 {
        self.matrix[x][y]
    }

    pub fn translation(&self, x: f64, y: f64, z: f64) -> Self {
        let transform = matrix![ 1.0, 0.0, 0.0, x;
                                 0.0, 1.0, 0.0, y;
                                 0.0, 0.0, 1.0, z;
                                 0.0, 0.0, 0.0, 1.0];
        transform * self
    }

    pub fn rotation_x(&self, x: f64) -> Self {
        let transform = matrix![ 1.0, 0.0, 0.0, 0.0;
                                 0.0, x.cos(), -x.sin(), 0.0;
                                 0.0, x.sin(), x.cos(), 0.0;
                                 0.0, 0.0, 0.0, 1.0];
        transform * self
    }

    pub fn rotation_y(&self, x: f64) -> Self {
        let transform = matrix![ x.cos(), 0.0, x.sin(), 0.0;
                                 0.0, 1.0, 0.0, 0.0;
                                 -x.sin(), 0.0, x.cos(), 0.0;
                                 0.0, 0.0, 0.0, 1.0];
        transform * self
    }

    pub fn rotation_z(&self, x: f64) -> Self {
        let transform = matrix![ x.cos(), -x.sin(), 0.0, 0.0;
                                 x.sin(), x.cos(), 0.0, 0.0;
                                 0.0, 0.0, 1.0, 0.0;
                                 0.0, 0.0, 0.0, 1.0];
        transform * self
    }

    pub fn scaling(&self, x: f64, y: f64, z: f64) -> Self {
        let transform = matrix![   x, 0.0, 0.0, 0.0;
                                 0.0,   y, 0.0, 0.0;
                                 0.0, 0.0,   z, 0.0;
                                 0.0, 0.0, 0.0, 1.0];
        transform * self
    }

    pub fn determinant(&self) -> f64 {
        if self.matrix.len() == 2 && self.matrix[0].len() == 2 {
            (self.matrix[0][0] * self.matrix[1][1]) - (self.matrix[0][1] * self.matrix[1][0])
        } else {
            let first_line = &self.matrix[0];
            first_line
                .into_iter()
                .enumerate()
                .fold(0.0, |acc, (y, item)| acc + (item * self.cofactor(0, y)))
        }
    }

    pub fn submatrix(&self, line: usize, column: usize) -> Self {
        let lines = self.matrix.len();
        let mut submatrix = vec![];

        for x in 0..lines {
            if x != line {
                let current_line = &self.matrix[x];
                submatrix.push(
                    current_line
                        .into_iter()
                        .enumerate()
                        .filter_map(
                            |(index, item)| if index != column { Some(*item) } else { None },
                        )
                        .collect::<Vec<f64>>(),
                );
            }
        }

        Self { matrix: submatrix }
    }

    pub fn minor(&self, line: usize, column: usize) -> f64 {
        self.submatrix(line, column).determinant()
    }

    pub fn cofactor(&self, line: usize, column: usize) -> f64 {
        let minor = self.minor(line, column);

        if (line + column) % 2 == 0 {
            minor
        } else {
            minor * -1.0
        }
    }

    pub fn inverse(&self) -> Self {
        let mut inversed = vec![];
        let determinant = self.determinant();
        let lines = self.matrix.len();

        for x in 0..lines {
            let current_line = &self.matrix[x];
            inversed.push(
                current_line
                    .into_iter()
                    .enumerate()
                    .map(|(y, _item)| self.cofactor(x, y) / determinant)
                    .collect::<Vec<f64>>(),
            );
        }

        Self { matrix: inversed }.transpose()
    }

    pub fn transpose(&self) -> Self {
        let lines = self.matrix.len();
        let columns = self.matrix[0].len();
        let mut matrix = vec![vec![0.0; columns]; lines];

        for x in 0..lines {
            for (y, item) in matrix.iter_mut().enumerate().take(columns) {
                item[x] = self.at(x, y)
            }
        }

        Self { matrix }
    }
}

pub fn view_transform(from: Tuple, to: Tuple, up: Tuple) -> Matrix {
    let forward = (to - from).norm();
    let upn = up.norm();
    let left = forward.cross_product(upn);
    let true_up = left.cross_product(forward);

    let orientation = matrix![ left.x, left.y, left.z, 0.0;
                               true_up.x, true_up.y, true_up.z, 0.0;
                               -forward.x, -forward.y, -forward.z, 0.0;
                               0.0, 0.0, 0.0, 1.0];

    return orientation * Matrix::identity(4).translation(-from.x, -from.y, -from.z);
}

pub const EPSILON: f64 = 0.001;

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        if self.matrix.is_empty() && other.matrix.is_empty() {
            true
        } else if self.matrix.len() != other.matrix.len()
            || self.matrix[0].len() != other.matrix[0].len()
        {
            false
        } else {
            let lines = self.matrix.len();
            let columns = self.matrix[0].len();

            for x in 0..lines {
                for y in 0..columns {
                    if (self.at(x, y) - other.at(x, y)).abs() > EPSILON {
                        return false;
                    }
                }
            }
            true
        }
    }
}

impl<'a> Mul<Tuple> for &'a Matrix {
    type Output = Tuple;

    fn mul(self, tuple: Tuple) -> Self::Output {
        let matrix = matrix![tuple.x; tuple.y ; tuple.z; tuple.w ];
        let result = self * &matrix;

        let x = result.matrix[0][0];
        let y = result.matrix[1][0];
        let z = result.matrix[2][0];
        let w = result.matrix[3][0];

        Tuple { x, y, z, w }
    }
}

impl Mul<Tuple> for Matrix {
    type Output = Tuple;

    fn mul(self, tuple: Self::Output) -> Self::Output {
        let matrix = matrix![tuple.x; tuple.y ; tuple.z; tuple.w ];
        let result = self * &matrix;

        let x = result.matrix[0][0];
        let y = result.matrix[1][0];
        let z = result.matrix[2][0];
        let w = result.matrix[3][0];

        Self::Output { x, y, z, w }
    }
}

impl<'a> Mul<&Matrix> for &Matrix {
    type Output = Matrix;

    fn mul(self, other: &Matrix) -> Matrix {
        let lines = self.matrix.len();
        let columns = other.matrix[0].len();

        let mut matrix = vec![vec![0.0; columns]; lines];

        for x in 0..lines {
            for y in 0..columns {
                let mut sum = 0.0;
                for pos in 0..lines {
                    let item = self.matrix[x][pos];
                    let item_b = other.matrix[pos][y];
                    sum += item * item_b;
                }
                matrix[x][y] = sum
            }
        }

        Matrix { matrix }
    }
}

impl Mul<&Matrix> for Matrix {
    type Output = Self;

    fn mul(self, other: &Matrix) -> Self {
        let lines = self.matrix.len();
        let columns = other.matrix[0].len();

        let mut matrix = vec![vec![0.0; columns]; lines];

        for x in 0..lines {
            for y in 0..columns {
                let mut sum = 0.0;
                for pos in 0..lines {
                    let item = self.matrix[x][pos];
                    let item_b = other.matrix[pos][y];
                    sum += item * item_b;
                }
                matrix[x][y] = sum
            }
        }

        Self { matrix }
    }
}

impl Mul for Matrix {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let lines = self.matrix.len();
        let columns = other.matrix[0].len();

        let mut matrix = vec![vec![0.0; columns]; lines];

        for x in 0..lines {
            for y in 0..columns {
                let mut sum = 0.0;
                for pos in 0..lines {
                    let item = self.matrix[x][pos];
                    let item_b = other.matrix[pos][y];
                    sum += item * item_b;
                }
                matrix[x][y] = sum
            }
        }

        Self { matrix }
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    use super::*;
    use crate::matrix;
    use crate::point;
    use crate::vector;
    use std::f64::consts::{FRAC_PI_2, FRAC_PI_4, PI};

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
        let result = identity * &matrix_a;
        assert_eq!(matrix_a, result);
    }

    #[test]
    fn test_matrix_rotation_y() {
        let half_quarter = Matrix::identity(4).rotation_y(FRAC_PI_4);
        let full_quarter = Matrix::identity(4).rotation_y(FRAC_PI_2);

        let p = point!(0.0, 0.0, 1.0);

        assert_eq!(
            half_quarter * p,
            point!(2.0_f64.sqrt() / 2.0, 0.0, 2.0_f64.sqrt() / 2.0)
        );
        assert_eq!(full_quarter * p, point!(1.0, 0.0, 0.0));
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

        let a_x_b = &matrix_a * &matrix_b;

        assert_eq!(matrix_a, a_x_b * matrix_b.inverse());
    }

    proptest! {
        #[test]
        fn test_inverse_matrix_property(vec in prop::collection::vec(prop::collection::vec(-1000.0..1000.0, 4), 4)) {
            let m = Matrix { matrix: vec };
            let inverse = m.inverse();
            assert_eq!(m * inverse, Matrix::identity(4));
        }
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

    #[test]
    fn test_view_transform() {
        let from = point!(0.0, 0.0, 0.0);
        let to = point!(0.0, 0.0, -1.0);
        let up = vector!(0.0, 1.0, 0.0);

        let t = view_transform(from, to, up);

        assert_eq!(t, Matrix::identity(4));
    }

    #[test]
    fn test_view_transform_looking_in_the_positive_z_direction() {
        let from = point!(0.0, 0.0, 0.0);
        let to = point!(0.0, 0.0, 1.0);
        let up = vector!(0.0, 1.0, 0.0);

        let t = view_transform(from, to, up);

        assert_eq!(t, Matrix::identity(4).scaling(-1.0, 1.0, -1.0));
    }

    #[test]
    fn test_view_transform_moves_the_world() {
        let from = point!(0.0, 0.0, 8.0);
        let to = point!(0.0, 0.0, 0.0);
        let up = vector!(0.0, 1.0, 0.0);

        let t = view_transform(from, to, up);

        assert_eq!(t, Matrix::identity(4).translation(0.0, 0.0, -8.0));
    }

    #[test]
    fn test_arbirtrary_view_transformation() {
        let from = point!(1.0, 3.0, 2.0);
        let to = point!(4.0, -2.0, 8.0);
        let up = vector!(1.0, 1.0, 0.0);

        let t = view_transform(from, to, up);

        assert_eq!(
            t,
            matrix![-0.50709, 0.50709, 0.67612, -2.36643;
                           0.76772, 0.60609, 0.12122, -2.82843;
                           -0.35857, 0.59761, -0.71714, 0.00000;
                           0.00000, 0.00000, 0.00000, 1.00000]
        );
    }

    #[test]
    fn test_translation() {
        let rs = point!(-3.0, 4.0, 5.0).translation(5.0, -3.0, 2.0);
        assert_eq!(rs, point!(2.0, 1.0, 7.0));
    }

    // pending inverse matrix tests

    #[test]
    fn test_scaling() {
        let rs = point!(-4.0, 6.0, 8.0).scaling(2.0, 3.0, 4.0);
        assert_eq!(rs, point!(-8.0, 18.0, 32.0));
    }

    #[test]
    fn test_scaling_reflection() {
        let rs = point!(2.0, 3.0, 4.0).scaling(-1.0, 1.0, 1.0);
        assert_eq!(rs, point!(-2.0, 3.0, 4.0));
    }

    // pending inverse matrix tests

    // rotation x

    //      y
    //      |_1.0
    //      |
    // -------------> x
    //	    |
    #[test]
    fn test_rotation_x() {
        let point_a = point!(0.0, 1.0, 0.0);

        assert_eq!(point_a.rotation_x(PI / 4.0), point!(0.0, 0.70710, 0.70710));
        assert_eq!(point_a.rotation_x(PI / 2.0), point!(0.0, 0.0, 1.0));
    }

    // rotation y

    #[test]
    fn test_rotation_y() {
        let point_a = point!(0.0, 0.0, 1.0);

        assert_eq!(point_a.rotation_y(PI / 4.0), point!(0.70710, 0.0, 0.70710));
        assert_eq!(point_a.rotation_y(PI / 2.0), point!(1.0, 0.0, 0.0));
    }

    // rotation z

    #[test]
    fn test_rotation_z() {
        let point_a = point!(0.0, 1.0, 0.0);

        assert_eq!(point_a.rotation_z(PI / 4.0), point!(-0.70710, 0.70710, 0.0));
        assert_eq!(point_a.rotation_z(PI / 2.0), point!(-1.0, 0.0, 0.0));
    }

    // shearing / skew
    #[test]
    fn test_skew() {
        let skewed = point!(2.0, 3.0, 4.0).skew(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        assert_eq!(skewed, point!(5.0, 3.0, 4.0));
    }
}
