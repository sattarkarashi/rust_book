fn main() {
    struct DataProcessor {
        vector: Vec<f64>,
        matrix: Vec<Vec<f64>>,
    }

    impl DataProcessor {
        fn new() -> Self {
            DataProcessor {
                vector: Vec::new(),
                matrix: Vec::new(),
            }
        }

        fn add_vector(&mut self, vec: Vec<f64>) {
            self.vector = vec;
        }

        fn add_matrix(&mut self, mat: Vec<Vec<f64>>) {
            self.matrix = mat;
        }

        fn process_vector(&self) -> f64 {
            self.vector.iter().sum()
        }

        fn process_matrix(&self) -> Vec<f64> {
            self.matrix.iter()
                .map(|row| row.iter().sum())
                .collect()
        }
    }

    impl DataProcessor {
        fn add_vectors(&self, other: &Vec<f64>) -> Vec<f64> {
            self.vector.iter()
                .zip(other.iter())
                .map(|(a, b)| a + b)
                .collect()
        }

        fn multiply_vectors(&self, other: &Vec<f64>) -> Vec<f64> {
            self.vector.iter()
                .zip(other.iter())
                .map(|(a, b)| a * b)
                .collect()
        }

        fn dot_product(&self, other: &Vec<f64>) -> f64 {
            self.multiply_vectors(other).iter().sum()
        }

        fn scale_vector(&self, scalar: f64) -> Vec<f64> {
            self.vector.iter()
                .map(|x| x * scalar)
                .collect()
        }
    }

    let mut dp = DataProcessor::new();
    dp.add_vector(vec![1.0, 2.0, 3.0]);
}
