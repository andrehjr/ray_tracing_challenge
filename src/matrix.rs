use crate::matrix;
use crate::tuple::Tuple;
use std::ops::Mul;

#[derive(Debug, Clone)]
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

    pub fn determinant(&self) -> f64 {
        if self.matrix.len() == 2 && self.matrix[0].len() == 2 {
            (self.matrix[0][0] * self.matrix[1][1]) - (self.matrix[0][1] * self.matrix[1][0])
        } else {
            0.0
        }
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

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        self.matrix == other.matrix
    }
}

impl Mul<Tuple> for Matrix {
    type Output = Tuple;

    fn mul(self, tuple: Self::Output) -> Self::Output {
        let matrix = matrix![tuple.x; tuple.y ; tuple.z; tuple.w ];
        let result = self * matrix;

        let x = result.matrix[0][0];
        let y = result.matrix[1][0];
        let z = result.matrix[2][0];
        let w = result.matrix[3][0];

        Self::Output { x, y, z, w }
    }
}

impl Mul<Matrix> for Matrix {
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
