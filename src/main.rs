use toyml_rs::{dot, Matrix, simple_dot, transpose, transpose_dot};

fn main() {
    let m = Matrix::new(&[
        1., 2., 3.,
        4., 5., 6.,
    ], &[2, 3]);
    let m2 = Matrix::new(&[
        7., 8.,
        9., 10.,
        11., 12.,
    ], &[3, 2]);
    let out = dot(&m, &m2);
    let out2 = transpose_dot(&m, &m2);

    println!("{:?}", out);
    println!("{:?}", out2);
}
