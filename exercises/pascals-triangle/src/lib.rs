pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle {
            rows: create_triangle(row_count),
        }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.clone()
    }
}

fn create_triangle(row_count: u32) -> Vec<Vec<u32>> {
    (0..row_count).map(|x| create_row(x)).collect()
}

fn create_row(row_length: u32) -> Vec<u32> {
    let mut row: Vec<u32> = vec![1];

    for i in 1..=row_length {
        if let Some(&last) = row.last() {
            row.push(last * (row_length + 1 - i) / i)
        }
    }

    row
}
