pub trait Matrix {
    type Elem;

    fn rows(&self) -> usize;
    fn cols(&self) -> usize;
    fn extract(&self, row: usize, col: usize) -> Option<Self::Elem>;
}

impl Matrix for Vec<&str> {
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
}
