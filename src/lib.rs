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

        for row in rows {
            matrix.push(row);
        }

        let mat = Matrix {
            mat: matrix,
        };
        mat
    }

    pub fn size(&self) -> usize {
        self.mat.len()
    }

    pub fn element(&self, i: i64, j: i64) -> u64 {
        self.mat[(i - 1) as usize][(j - 1) as usize]
    }
}