use lee_alg::*;

#[test]
fn square() {
    let tester = Matrix::new(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);
    let test_notsq = Matrix::new(vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9],
        vec![10, 11, 12],
    ]);
    assert_eq!(true, tester.is_square());
    assert_eq!(false, test_notsq.is_square());
}

#[test]
fn cols() {
    let tester = Matrix::new(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);
    assert_eq!(3, tester.ncol());
}

#[test]
fn rows() {
    let test_notsq = Matrix::new(vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9],
        vec![10, 11, 12],
    ]);
    assert_eq!(4, test_notsq.nrow());
}

#[test]
fn dot_prod() {
    let tester = Matrix::new(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]);
    let col1 = &tester.to_columns()[0];
    let col2 = &tester.to_columns()[1];
    assert_eq!(78, col1.dot(&col2));
}


#[test]
fn transpose() {
    let a = Matrix::new(vec![vec![1, 2, 3], vec![4, 5, 6]]);
    let b = Matrix::new(vec![vec![1, 4], vec![2, 5], vec![ 3, 6]]);
    assert_eq!(a.transpose(), b);
}
