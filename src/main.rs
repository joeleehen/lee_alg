use lee_alg::*;

fn main() {
    let test = Matrix::new(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9], vec![10, 11, 12]]);

    println!("extracting columns of matrix");
    for column in test.to_columns() {
        println!("{}", column);
    }
}
