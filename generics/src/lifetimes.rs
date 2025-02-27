
pub trait RefIterator<'a> {
    type Item;
    fn next(&'a mut self) -> Option<&'a Self::Item>;
}

pub struct Matrix {
    matrix: Vec<Vec<u32>>,
    pos: usize,
}

impl Matrix {
    pub fn new(input: &Vec<Vec<u32>>) -> Self {
        let matrix: Vec<Vec<u32>> = input.clone();
        let pos = 0usize;
        Self {
            matrix,
            pos,
        }
    }
}

impl<'a> RefIterator<'a> for Matrix {
    type Item = Vec<u32>;

    fn next(&'a mut self) -> Option<&'a Self::Item> {
        if self.pos < self.matrix.len() {
            self.pos = self.pos+1;
            Some(&self.matrix[self.pos-1])
        } else {
            None
        }
    }
}

pub fn sum_row(row: &Vec<u32>) -> u32 {
    let mut sum = 0;
    let _ = row.iter().map(|x| {sum = sum + x}).count();
    sum
}

