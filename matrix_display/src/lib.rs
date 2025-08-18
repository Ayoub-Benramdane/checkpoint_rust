pub struct Matrix(pub Vec<Vec<i32>>);

impl Matrix {
    pub fn new(slice: &[&[i32]]) -> Self {
        let mut table = Vec::new();
        for row in slice {
            table.push(row.to_vec());
        }
        Self(table)
    }
}

use std::fmt;

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut cnt = 0;
        for c in &self.0 {
            cnt += 1;
            let mut count = 0;
            write!(f, "(");
            for num in c {
                count += 1;
                write!(f, "{}", num);
                if count < c.len() {
                    write!(f, " ");
                }
            }
            write!(f, ")");
            if cnt < self.0.len() {
                writeln!(f);
            }
        }
        Ok(())
    }
}
