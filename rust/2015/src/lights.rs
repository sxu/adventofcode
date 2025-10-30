#[derive(Debug)]
pub struct Grid<T> {
    height: usize,
    width: usize,
    data: Vec<T>,
}

impl<T: Clone> Grid<T> {
    pub fn new(height: usize, width: usize, init: T) -> Self {
        Self {
            height,
            width,
            data: vec![init; height * width],
        }
    }
}

impl<T> Grid<T> {
    pub fn at(&self, i: usize, j: usize) -> &T {
        &self.data[i * self.width + j]
    }

    pub fn at_mut(&mut self, i: usize, j: usize) -> &mut T {
        &mut self.data[i * self.width + j]
    }

    pub fn apply(
        &mut self,
        upper_left: (usize, usize),
        lower_right: (usize, usize),
        f: fn(&T) -> T,
    ) {
        let (x1, y1) = upper_left;
        let (x2, y2) = lower_right;
        assert!(x1 <= x2 && x2 < self.height);
        assert!(y1 <= y2 && y2 < self.width);
        for i in x1..=x2 {
            for j in y1..=y2 {
                *self.at_mut(i, j) = f(self.at(i, j));
            }
        }
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn data(&self) -> &Vec<T> {
        &self.data
    }
}
