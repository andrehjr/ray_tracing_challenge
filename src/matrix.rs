use crate::tuple::Tuple;
use std::ops::Mul;

#[derive(Debug, Clone)]
pub struct Matrix {
    pub matrix: Vec<Vec<f64>>,
}

impl Matrix {
    pub fn init(matrix: Vec<Vec<f64>>) -> Matrix {
        Matrix { matrix }
    }

    pub fn identity(size: usize) -> Matrix {
        let mut matrix = vec![vec![0.0; size]; size];

        for pos in 0..size {
            matrix[pos][pos] = 1.0
        }

        Matrix { matrix }
    }

    pub fn at(&self, x: usize, y: usize) -> f64 {
        self.matrix[x][y]
    }
}

impl Mul<Tuple> for Matrix {
    type Output = Tuple;

    fn mul(self, tuple: Self::Output) -> Self::Output {
        let matrix = Self::init(vec![
            vec![tuple.x],
            vec![tuple.y],
            vec![tuple.z],
            vec![tuple.w],
        ]);
        let result = self.clone() * matrix;

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
