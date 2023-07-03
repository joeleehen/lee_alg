use std::fmt;

pub struct Matrix {
    mat: Vec<Vec<u64>>,
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

impl Matrix {
    pub fn new(rows: Vec<Vec<u64>>) -> Matrix {
        // build matrix from nested vectors
        let mut matrix = Vec::new();

        let size = rows[0].len();    // enforce ncol by first row entered
        for row in rows {
            // first ensure each row vector is the proper length
            // TODO: better error handling
            if size != row.len() {
                panic!("Row vector {:?} must have {} elements", row, size);
            }
            matrix.push(row);
        }

        let mat = Matrix {
            mat: matrix,
        };
        mat
    }

    pub fn nrow(&self) -> usize {
        self.mat.len()
    }

    pub fn ncol(&self) -> usize {
        self.mat[0].len()
    }

    pub fn element(&self, i: i64, j: i64) -> u64 {
        self.mat[(i) as usize][(j) as usize]
    }

    pub fn is_square(&self) -> bool {
        self.nrow() == self.ncol()
    }

    pub fn to_columns(&self) -> Vec<Vec<u64>> {
        let mut columns = Vec::new();
        for i in 0..self.ncol() {
            let mut col = Vec::new();
            for j in 0..self.nrow() {
                col.push(self.element(j as i64, i as i64));
            }
            columns.push(col);
        }
        columns
    }
}
