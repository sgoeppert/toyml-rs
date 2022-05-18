
use std::ops::Index;

#[derive(Debug, Clone)]
pub struct Dim {
    shape: [usize; 2],
}

impl Dim {
    fn new(shape: &[usize; 2]) -> Self {
        assert!(shape.len() > 1, "The shape must have at least 2 dimensions. For a column vector use [N, 1] and for a row vector [1, N]");
        Dim {
            shape: shape.clone()
        }
    }

    fn total_elements(&self) -> usize {
        self.shape.iter().product()
    }

    fn dot(&self, rhs: &Dim) -> Option<Dim> {
        if self[1] != rhs[0] {
            None
        } else {
            Some(Dim {
                shape: [self[0], rhs[1]]
            })
        }
    }
}

impl Index<usize> for Dim {
    type Output = usize;

    fn index(&self, index: usize) -> &Self::Output {
        &self.shape[index]
    }
}

#[derive(Debug)]
pub struct Matrix {
    data: Vec<f64>,
    shape: Dim,
}

impl Matrix {
    pub fn new(data: &[f64], shape: &[usize; 2]) -> Self {
        let shape = Dim::new(shape);
        assert_eq!(shape.total_elements(), data.len(), "The data does not fit the shape");
        Matrix {
            data: data.to_vec(),
            shape,
        }
    }

    pub fn zeros(shape: &[usize; 2]) -> Self {
        let shape = Dim::new(shape);
        Matrix {
            data: vec![0.0; shape.total_elements()],
            shape,
        }
    }

    pub fn dim(&self) -> Dim {
        self.shape.clone()
    }
}

/// Transpose a matrix
pub fn transpose(mat: &Matrix) -> Matrix {
    let new_shape = Dim::new(&[mat.shape[1], mat.shape[0]]);

    let mut transposed = vec![0.; new_shape.total_elements()];
    for y in 0..mat.shape[0] {
        for x in 0..mat.shape[1] {
            let idx = x + mat.shape[1] * y;
            let new_idx = y + mat.shape[0] * x;
            transposed[new_idx] = mat.data[idx];
        }
    }

    Matrix {
        shape: new_shape,
        data: transposed
    }
}

pub fn dot(lhs: &Matrix, rhs: &Matrix) -> Matrix {
    let new_shape = lhs.shape.dot(&rhs.shape).unwrap();
    let mut out_data: Vec<f64> = Vec::with_capacity(new_shape.total_elements());

    let rows = new_shape[0];
    let cols = new_shape[1];
    let items = lhs.shape[1];

    for row in 0..rows {
        for col in 0..cols {
            let mut sum = 0.;
            for idx in 0..items {
                let l_idx = row * items + idx;
                let r_idx = idx * cols + col;
                sum += lhs.data[l_idx] * rhs.data[r_idx];
            }
            out_data.push(sum);
        }
    }

    Matrix {
        data: out_data,
        shape: new_shape
    }
}

/// Calculate the dot product between to matrices by first transposing the right matrix.
///
/// The goal is to achieve a cache friendlier implementation for large matrices. So far it doesn't
/// seem to be better than `dot()`.
pub fn transpose_dot(lhs: &Matrix, rhs: &Matrix) -> Matrix {
    let new_shape = lhs.shape.dot(&rhs.shape).unwrap();
    let mut out_data: Vec<f64> = Vec::with_capacity(new_shape.total_elements());

    let t_rhs = transpose(&rhs);
    let rows = new_shape[0];
    let cols = new_shape[1];
    let items = lhs.shape[1];

    for row in 0..rows {
        for col in 0..cols {
            let mut sum = 0.;
            for idx in 0..items {
                let l_idx = row * items + idx;
                let r_idx = col * items + idx;
                sum += lhs.data[l_idx] * t_rhs.data[r_idx];
            }
            out_data.push(sum);
        }
    }

    Matrix {
        data: out_data,
        shape: new_shape
    }
}

/// Calculate the dot product using an iterator based approach.
///
/// Very slow and having to collect
/// into a `Vec<Vec<f64>>` is terrible. Maybe there is a better way with a custom iterator instead
/// of using `[T].chunks()` or `.map()`
pub fn slow_dot(lhs: &Matrix, rhs: &Matrix) -> Matrix {
    let new_shape = lhs.shape.dot(&rhs.shape).unwrap();

    let mut out_data = Vec::with_capacity(new_shape.total_elements());

    let m1_rows = lhs.data.chunks(lhs.shape[1]);
    let m2_cols: Vec<Vec<f64>> = (0..rhs.shape[1])
        .into_iter()
        .map(|c| {
            rhs.data
                .iter()
                .skip(c)
                .step_by(rhs.shape[1])
                .map(|v| *v)
                .collect()
        })
        .collect();

    for row in m1_rows {
        for col in &m2_cols {
            out_data.push(row.iter().zip(col.iter()).map(|(a, b)| a*b).sum())
        }
    }

    Matrix {
        data: out_data,
        shape: new_shape,
    }
}

#[cfg(test)]
mod tests {
    use test::{Bencher, black_box};
    use super::*;

    #[bench]
    fn bench_dot(b: &mut Bencher) {
        let m = Matrix::new(&[
            1., 2., 3.,7.,10., 20.,0.3, 0.5,
            4., 5., 6.,7.,10., 20.,0.3, 0.5,
            4., 5., 6.,7.,10., 20.,0.3, 0.5,
            4., 5., 6.,7.,10., 20.,0.3, 0.5,
            4., 5., 9.,7.,10., 11.,0.3, 0.5,
            4., 5., 6.,7.,10., 20.,0.3, 0.5,
        ], &[6, 8]);
        let m2 = Matrix::new(&[
            1., 2.,1., 2.,5., 6.,1., 2.,0.1, 0.3, 0.6,
            3., 4.,1., 2.,5., 6.,1., 2.,0.1, 0.3, 0.6,
            5., 6.,1., 2.,5., 6.,1., 2.,0.1, 0.3, 0.6,
            5., 6.,1., 2.,5., 6.,1., 2.,0.1, 0.3, 0.6,
            5., 6.,1., 2.,5., 6.,1., 2.,0.1, 0.3, 0.6,
            5., 6.,1., 2.,5., 6.,1., 2.,0.1, 0.3, 0.6,
            5., 6.,1., 2.,5., 6.,1., 2.,0.1, 0.3, 0.66,
            5., 6.,1., 2.,5., 6.,1., 2.,0.1, 0.3, 0.66,
        ], &[8, 11]);

        let bb_m = black_box(&m);
        let bb_m2 = black_box(&m2);

        b.iter(|| {
            for _ in 1..100 {
                dot(&bb_m, &bb_m2);
            }
        })
    }

}