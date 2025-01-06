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
}
