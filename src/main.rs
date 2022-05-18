
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

    fn dim(&self) -> Dim {
        self.shape.clone()
    }
}

fn main() {
    let m = Matrix::new(&[1., 2., 3., 4., 5., 6.,], &[1, 6]);

    println!("{:?}", m);
}
