use std::fmt;
use std::ops::Add;

#[derive(PartialEq, Debug)]
pub struct Matrix {
    mat: Vec<Vec<f64>>,
}

pub struct Column {
    col: Vec<f64>,
}

impl fmt::Display for Column {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut fmt_str = String::new();
        let newline = '\n';

        let col = &self.col;
        for elem in col {
            let elem_str = elem.to_string();
            fmt_str.push_str(&elem_str);
            fmt_str.push(newline);
        }
        write!(f, "{}", fmt_str)
    }
}

impl Column {
    pub fn new<T: Into<f64> + std::fmt::Debug>(elems: Vec<T>) -> Column {
        // let column = Column { col: elems };
        // cast each element of elems vector to a float
        let mut f_elems = Vec::new();
        for elem in elems {
            f_elems.push(elem.into());
        }
        let column = Column { col: f_elems };
        column
    }

    // TODO why is this here I forgor
    pub fn to_vector(&self) -> Vec<f64> {
        let mut vectr = Vec::new();
        for elem in &self.col {
            vectr.push(*elem);
        }
        vectr
    }

    // calculate dot product of two column vectors
    pub fn dot(&self, other: &Column) -> f64 {
        if &self.col.len() == &other.col.len() {
            let mut sum = 0.0;
            // iterate through each col
            for i in 0..self.col.len() {
                // and multiply the elements
                sum += &self.col[i] * &other.col[i]
            }
            sum
        } else {
            panic!("Both vectors should be of the same size!");
        }
    }
}

// __str__ display format
impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut fmt_str = String::new();
        let spacer = "    ";
        let rows = &self.mat;
        for row in rows {
            for elem in row {
                let mut elem_str = elem.to_string();
                elem_str.push_str(&spacer);
                fmt_str.push_str(&elem_str);
            }
            fmt_str.push_str("\n");
        }
        write!(f, "{}", fmt_str)
    }
}

impl Add for Matrix {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if self.ncol() != other.ncol() || self.nrow() != other.nrow() {
            panic!("matrices must have similar dimensions to be added together!");
        } else {
            let mut sum = Vec::new();
            for i in 0..self.ncol() {
                let mut new_row = Vec::new();
                for j in 0..self.nrow() {
                    new_row.push(self.mat[i][j] + other.mat[i][j]);
                }
                sum.push(new_row);
            }

            Self { mat: sum }
        }
    }
}

impl Matrix {
    pub fn new<T: Into<f64> + std::fmt::Debug>(rows: Vec<Vec<T>>) -> Matrix {
        // build matrix from nested vectors
        let mut matrix = Vec::new();

        let size = rows[0].len(); // enforce ncol by first row entered
        for row in rows {
            // first ensure each row vector is the proper length
            // TODO: better error handling
            if size != row.len() {
                panic!("Row vector {:?} must have {} elements", row, size);
            }
            // matrix.push(row);
            // cast each element to a float before creating matrix object
            let mut floatrow = Vec::new();
            for element in row {
                floatrow.push(element.into());
            }
            matrix.push(floatrow);
        }

        let mat = Matrix { mat: matrix };
        mat
    }

    pub fn nrow(&self) -> usize {
        self.mat.len()
    }

    pub fn ncol(&self) -> usize {
        self.mat[0].len()
    }

    pub fn element(&self, i: i64, j: i64) -> f64 {
        self.mat[(i) as usize][(j) as usize]
    }

    pub fn is_square(&self) -> bool {
        self.nrow() == self.ncol()
    }

    pub fn to_columns(&self) -> Vec<Column> {
        let mut columns = Vec::new();
        for i in 0..self.ncol() {
            let mut col = Vec::new();
            for j in 0..self.nrow() {
                col.push(self.element(j as i64, i as i64));
            }
            columns.push(Column::new(col));
        }
        columns
    }

    pub fn transpose(&self) -> Matrix {
        let cols = &self.to_columns();
        let mut vec_cols = Vec::new();

        for column in cols {
            vec_cols.push(column.to_vector());
        }
        Matrix::new(vec_cols)
    }

    pub fn clone(&self) -> Matrix {
        let mut copy_vec = Vec::new();
        for row in &self.mat {
            let mut row_copy = Vec::new();
            for element in row {
                row_copy.push(*element);
            }
            copy_vec.push(row_copy);
        }
        Matrix::new(copy_vec)
    }

    pub fn determinant(&self) -> f64 {
        if self.is_square() {
            // one-dimensional matrix
            if self.ncol() == 1 {
                self.element(0, 0)

            // two-by-two matrix
            } else if self.ncol() == 2 {
                let det = (self.element(0, 0) * self.element(1, 1))
                    - (self.element(1, 0) * self.element(0, 1));
                det
            // everything else
            } else {
                // half of LU Decomposition
                let copy = &mut self.clone(); // mutable copy for row reduction
                for i in 0..&self.ncol() - 1 {
                    // TODO: this is ugly. prolly refactor element() to accept
                    // generic integers rather than i64
                    let diagonal = copy.element(i as i64, i as i64);
                    for j in i + 1..copy.ncol() {
                        let combination = copy.element(j as i64, i as i64) / diagonal;
                        for k in i..copy.ncol() {
                            // copy.element(j as i64, k as i64) -=
                            //     combination * (copy.element(i as i64, k as i64));
                            copy.mat[j][k] -= combination * copy.element(i as i64, k as i64);
                        }
                    }
                }
                // product of diagonals from U yields determinant of original matrix
                let mut product = 1.0;
                for i in 0..copy.ncol() {
                    product *= copy.element(i as i64, i as i64);
                }
                product
            }
        } else {
            panic!("determinant does not exist for non-square matrices!");
        }
    }
}
