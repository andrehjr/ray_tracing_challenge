use crate::matrix;
use crate::tuple::Tuple;
use std::ops::Mul;

#[derive(Debug)]
pub struct Matrix {
    pub matrix: Vec<Vec<f64>>,
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

pub fn view_transform(from : Tuple, to : Tuple, up : Tuple) -> Matrix {
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

    fn mul(self, other: &matrix::Matrix) -> Matrix {
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

    fn mul(self, other: &matrix::Matrix) -> Self {
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
