use toyml_rs::math::{dot, Matrix};

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

    println!("{:?}", out);
}
