use std::ops::Index;

#[derive(Debug, Clone)]
struct Dim {
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
struct Matrix {
    data: Vec<f64>,
    shape: Dim,
}

impl Matrix {
    fn new(data: &[f64], shape: &[usize; 2]) -> Self {
        let shape = Dim::new(shape);
        assert_eq!(shape.total_elements(), data.len(), "The data does not fit the shape");
        Matrix {
            data: data.to_vec(),
            shape,
        }
    }

    fn zeros(shape: &[usize; 2]) -> Self {
        let shape = Dim::new(shape);
        Matrix {
            data: vec![0.0; shape.total_elements()],
            shape,
        }
    }

    fn dim(&self) -> Dim {
        self.shape.clone()
    }
}

fn dot(lhs: &Matrix, rhs: &Matrix) -> Matrix {
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

fn simple_dot(lhs: &Matrix, rhs: &Matrix) -> Matrix {
    let new_shape = lhs.shape.dot(&rhs.shape).unwrap();
    let mut out_data: Vec<f64> = Vec::with_capacity(2*2);

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

fn main() {
    let m = Matrix::new(&[
        1., 2., 3.,
        4., 5., 6.,
    ], &[2, 3]);
    let m2 = Matrix::new(&[
        1., 2.,
        3., 4.,
        5., 6.,
    ], &[3, 2]);
    let out = dot(&m, &m2);
    let out2 = simple_dot(&m, &m2);

    println!("{:?}", out);
    println!("{:?}", out2);
}
