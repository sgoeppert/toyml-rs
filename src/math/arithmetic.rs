use std::ops::{Add, Sub};
use crate::math::Matrix;

impl Add<f64> for Matrix {
    type Output = Matrix;

    fn add(self, rhs: f64) -> Self::Output {
        Matrix {
            data: self.data.iter().map(|v| v + rhs).collect(),
            shape: self.dim()
        }
    }
}

impl Add<Matrix> for f64 {
    type Output = Matrix;

    fn add(self, rhs: Matrix) -> Self::Output {
        Matrix {
            data: rhs.data.iter().map(|v| v + self).collect(),
            shape: rhs.dim()
        }
    }
}


impl Add<f64> for &Matrix {
    type Output = Matrix;

    fn add(self, rhs: f64) -> Self::Output {
        Matrix {
            data: self.data.iter().map(|v| v + rhs).collect(),
            shape: self.dim()
        }
    }
}

impl Add<&Matrix> for f64 {
    type Output = Matrix;

    fn add(self, rhs: &Matrix) -> Self::Output {
        Matrix {
            data: rhs.data.iter().map(|v| v + self).collect(),
            shape: rhs.dim()
        }
    }
}

impl Sub<f64> for Matrix {
    type Output = Matrix;
    fn sub(self, rhs: f64) -> Self::Output {
        Matrix {
            data: self.data.iter().map(|v| v - rhs).collect(),
            shape: self.dim()
        }

    }
}

impl Sub<f64> for &Matrix {
    type Output = Matrix;
    fn sub(self, rhs: f64) -> Self::Output {
        Matrix {
            data: self.data.iter().map(|v| v - rhs).collect(),
            shape: self.dim()
        }

    }
}

impl Add<Matrix> for Matrix {
    type Output = Matrix;

    fn add(self, rhs: Matrix) -> Self::Output {
        assert_eq!(self.shape, rhs.shape);
        Matrix {
            data: self.data.iter().zip(rhs.data.iter()).map(|(l, r)| l + r).collect(),
            shape: self.dim()
        }
    }
}

impl Add<&Matrix> for &Matrix {
    type Output = Matrix;

    fn add(self, rhs: &Matrix) -> Self::Output {
        assert_eq!(self.shape, rhs.shape);
        Matrix {
            data: self.data.iter().zip(rhs.data.iter()).map(|(l, r)| l + r).collect(),
            shape: self.dim()
        }
    }
}

// TODO: Add other operators (multiply/divide with f64)

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add_float() {
        let mut m = Matrix::zeros(&[3, 3]);
        m = m + 1.;
        m = 2. + m;
        assert_eq!(m.data, vec![3.; 9]);
    }

    #[test]
    fn sub_float() {
        let mut m = Matrix::new(&[0., 1., 2., 3., 4., 5., 6., 7., 8.,], &[3, 3]);
        m = m - 1.;
        assert_eq!(m.data, &[-1., 0., 1., 2., 3., 4., 5., 6., 7.]);
    }

    #[test]
    fn add_matrix() {
        let m1 = Matrix::new(&[0., 1., 2., 3., 4., 5., 6., 7., 8.,], &[3, 3]);
        let m2 = Matrix::new(&[0., 1., 2., 3., 4., 5., 6., 7., 8.,], &[3, 3]);
        let out = m1 + m2;
        assert_eq!(out.data, &[0., 2., 4., 6., 8., 10., 12., 14., 16.,]);
    }

    #[test]
    fn add_matrix_ref() {
        let m1 = Matrix::new(&[0., 1., 2., 3., 4., 5., 6., 7., 8.,], &[3, 3]);
        let m2 = Matrix::new(&[0., 1., 2., 3., 4., 5., 6., 7., 8.,], &[3, 3]);
        let out = &m1 + &m2;
        assert_eq!(out.data, &[0., 2., 4., 6., 8., 10., 12., 14., 16.,]);
    }
    #[test]
    #[should_panic]
    fn panic_when_adding_matrix_of_diff_size() {
        let m1 = Matrix::zeros(&[1,2]);
        let m2 = Matrix::zeros(&[2,3]);
        let _ = m1 + m2;
    }
}