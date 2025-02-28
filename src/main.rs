// to do implement basic matrix functionality



struct Matrix{
    rows: usize,
    cols: usize,
}

impl Matrix{
    fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
        }
    }
}



fn main() {

    let a = Matrix::new(2, 2);
    println!("{}   {} ", a.rows,a.cols);
}



