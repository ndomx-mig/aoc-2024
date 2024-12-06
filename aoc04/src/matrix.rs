pub trait Matrix {
    type Elem;

    fn rows(&self) -> usize;
    fn cols(&self) -> usize;
    fn extract(&self, row: usize, col: usize) -> Option<Self::Elem>;
    fn compare_element(&self, row: usize, col: usize, comp: Self::Elem) -> bool;
    fn get_window(&self, row: usize, col: usize) -> Option<[[Self::Elem; 3]; 3]>;
    fn window_diagonals(&self, row: usize, col: usize) -> Option<[Self::Elem; 4]>;

    fn max_row_index(&self) -> usize {
        return self.rows() - 1;
    }

    fn max_col_index(&self) -> usize {
        return self.cols() - 1;
    }
}

impl Matrix for Vec<String> {
    type Elem = char;

    fn rows(&self) -> usize {
        return self.len();
    }

    fn cols(&self) -> usize {
        return self[0].len();
    }

    fn extract(&self, row: usize, col: usize) -> Option<Self::Elem> {
        return self[row].chars().nth(col);
    }

    fn compare_element(&self, row: usize, col: usize, comp: Self::Elem) -> bool {
        return match self[row].chars().nth(col) {
            Some(c) => comp == c,
            None => false,
        };
    }

    fn get_window(&self, row: usize, col: usize) -> Option<[[Self::Elem; 3]; 3]> {
        if row == 0 || row >= self.max_row_index() {
            return None;
        }

        if col == 0 || col >= self.max_col_index() {
            return None;
        }

        let mut window = [['0'; 3]; 3];
        for i in 0..3 {
            for j in 0..3 {
                let row_idx = (row + i - 1) as usize;
                let col_idx = (col + j - 1) as usize;

                window[i][j] = self[row_idx].chars().nth(col_idx).unwrap();
            }
        }

        return Some(window);
    }

    fn window_diagonals(&self, row: usize, col: usize) -> Option<[Self::Elem; 4]> {
        if row == 0 || row >= self.max_row_index() {
            return None;
        }

        if col == 0 || col >= self.max_col_index() {
            return None;
        }

        return Some([
            self[row - 1].chars().nth(col - 1).unwrap(),
            self[row - 1].chars().nth(col + 1).unwrap(),
            self[row + 1].chars().nth(col - 1).unwrap(),
            self[row + 1].chars().nth(col + 1).unwrap(),
        ]);
    }
}
