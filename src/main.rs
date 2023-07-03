use leeAlg::Matrix;

fn main() {
    let test = Matrix::new(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9], vec![10, 11, 12]]);

    println!("{}", test);
    if test.is_square() {
        println!("is square");
    } else {
        println!("not square");
    }

    println!();
    println!("number of rows: {}", test.nrow());
    println!("number of columns: {}", test.ncol());

    println!();
    println!("extracting columns of matrix");
    println!("{:?}", test.to_columns());
}
