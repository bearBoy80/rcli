use rcli::{multiply, Matrix};

fn main() {
    let a: Matrix<i32> = Matrix::new([1, 2, 3, 4, 5, 6], 2, 3);
    // 1,2;3,4;5,6
    let b: Matrix<i32> = Matrix::new([1, 2, 3, 4, 5, 6], 3, 2);
    let c = multiply(&a, &b).unwrap();
    println!("{:?}", c);
}
