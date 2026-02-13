use std::ops::{Add, Sub, Mul, Index, IndexMut};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Matrix3 {
    data: [[f64; 3]; 3],
}

impl Matrix3 {
    #[inline]
    pub fn new(data: [[f64; 3]; 3]) -> Self {
        Self { data }
    }

    #[inline]
    pub fn identity() -> Self {
        let mut data = [[0.0; 3]; 3];
        data[0][0] = 1.0;
        data[1][1] = 1.0;
        data[2][2] = 1.0;
        Self { data }
    }

    #[inline]
    pub fn zero() -> Self {
        Self { data: [[0.0; 3]; 3] }
    }

    #[inline]
    pub fn get(&self, row: usize, col: usize) -> f64 {
        self.data[row][col]
    }

    #[inline]
    pub fn set(&mut self, row: usize, col: usize, value: f64) {
        self.data[row][col] = value;
    }

    #[inline]
    pub fn determinant(&self) -> f64 {
        let a = self.data[0][0];
        let b = self.data[0][1];
        let c = self.data[0][2];
        let d = self.data[1][0];
        let e = self.data[1][1];
        let f = self.data[1][2];
        let g = self.data[2][0];
        let h = self.data[2][1];
        let i = self.data[2][2];

        a * (e * i - f * h) - b * (d * i - f * g) + c * (d * h - e * g)
    }

    #[inline]
    pub fn transpose(&self) -> Self {
        let mut result = Self::zero();
        for i in 0..3 {
            for j in 0..3 {
                result.data[i][j] = self.data[j][i];
            }
        }
        result
    }

    #[inline]
    pub fn multiply(&self, other: &Self) -> Self {
        let mut result = Self::zero();
        for i in 0..3 {
            for j in 0..3 {
                let mut sum = 0.0;
                for k in 0..3 {
                    sum = sum + self.data[i][k] * other.data[k][j];
                }
                result.data[i][j] = sum;
            }
        }
        result
    }

    #[inline]
    pub fn multiply_vector(&self, v: &super::Vector2) -> super::Vector2 {
        let x = self.data[0][0] * v.x + self.data[0][1] * v.y + self.data[0][2];
        let y = self.data[1][0] * v.x + self.data[1][1] * v.y + self.data[1][2];
        super::Vector2::new(x, y)
    }
}

impl Add for Matrix3 {
    type Output = Self;
    #[inline]
    fn add(self, other: Self) -> Self {
        let mut data = [[0.0; 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                data[i][j] = self.data[i][j] + other.data[i][j];
            }
        }
        Self { data }
    }
}

impl Sub for Matrix3 {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        let mut data = [[0.0; 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                data[i][j] = self.data[i][j] - other.data[i][j];
            }
        }
        Self { data }
    }
}

impl Mul<f64> for Matrix3 {
    type Output = Self;
    #[inline]
    fn mul(self, scalar: f64) -> Self {
        let mut data = [[0.0; 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                data[i][j] = self.data[i][j] * scalar;
            }
        }
        Self { data }
    }
}

impl Index<(usize, usize)> for Matrix3 {
    type Output = f64;
    #[inline]
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.0][index.1]
    }
}

impl IndexMut<(usize, usize)> for Matrix3 {
    #[inline]
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.data[index.0][index.1]
    }
}

impl fmt::Display for Matrix3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "[{}, {}, {}]", self.data[0][0], self.data[0][1], self.data[0][2])?;
        writeln!(f, "[{}, {}, {}]", self.data[1][0], self.data[1][1], self.data[1][2])?;
        write!(f, "[{}, {}, {}]", self.data[2][0], self.data[2][1], self.data[2][2])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix3_identity() {
        let identity = Matrix3::identity();
        assert_eq!(identity.get(0, 0), 1.0);
        assert_eq!(identity.get(1, 1), 1.0);
        assert_eq!(identity.get(2, 2), 1.0);
    }

    #[test]
    fn test_matrix3_determinant() {
        let m = Matrix3::identity();
        assert_eq!(m.determinant(), 1.0);
    }
}
