
struct Matrix{
    rows: usize,
    cols: usize,
    data: Vec<Vec<f64>>
}



impl Matrix{
    fn new(rows: usize, cols: usize, data: Vec<Vec<f64>>) -> Self {
        Self {
            rows,
            cols,
            data
        }
    }
    
    fn add(&self, m2 : &Self) -> Self {
        let mut res = Matrix::zeros(self.rows , self.cols);

        for i in 0..self.rows{
            for j in 0..self.cols{
                res.data[i][j] += self.data[i][j] + m2.data[i][j];
            }
        }

        res
    }

    fn zeros(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            data: vec![vec![0.0; cols]; rows],
        }
    }

    fn print(&self) {
        for row in &self.data {
            println!("{:?}", row);
        }
    }

}



fn main() {
    
    let a = Matrix::new(2, 2, vec![vec![1.0, 2.0], vec![3.0, 4.0]]);
    let b = Matrix::new(2, 2, vec![vec![5.0, 6.0], vec![7.0, 8.0]]);
    

    let result = a.add(&b);
    println!("Matrix Sum:");
    result.print();

}



