#![allow(unused)]

use std::ops::{Index, IndexMut};

pub struct Grid<T> {
    data: Box<[T]>,
    rows: usize,
    cols: usize,
    stride: usize,
}

impl<T> Grid<T>
where
    T: Default,
{
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            data: (0..rows * cols)
                .map(|_| T::default())
                .collect::<Vec<_>>()
                .into_boxed_slice(),
            rows,
            cols,
            stride: cols,
        }
    }

    pub fn resize(self, rows: usize, cols: usize) -> Self {
        assert!(rows <= self.rows);
        assert!(cols <= self.cols);
        Self { rows, cols, ..self }
    }

    pub fn rows(&self) -> usize { self.rows }
    pub fn cols(&self) -> usize { self.cols }
}

impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, (r, c): (usize, usize)) -> &Self::Output {
        assert!((0..self.rows).contains(&r));
        assert!((0..self.cols).contains(&c));
        &self.data[r * self.stride + c]
    }
}

impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, (r, c): (usize, usize)) -> &mut Self::Output {
        assert!((0..self.rows).contains(&r));
        assert!((0..self.cols).contains(&c));
        &mut self.data[r * self.stride + c]
    }
}
