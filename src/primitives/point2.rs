use nalgebra::{SMatrix, SMatrixView, Vector2};

use super::Parametric;

#[derive(Debug)]
pub struct Point2 {
    data: Vector2<f64>,
    gradient: Vector2<f64>,
}

impl Point2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            data: Vector2::new(x, y),
            gradient: Vector2::zeros(),
        }
    }

    pub fn x(&self) -> f64 {
        self.data.x
    }

    pub fn y(&self) -> f64 {
        self.data.y
    }

    pub fn data(&self) -> Vector2<f64> {
        self.data
    }

    pub fn gradient(&self) -> SMatrix<f64, 2, 2> {
        SMatrix::<f64, 2, 2>::from_row_slice(&[1.0, 0.0, 0.0, 1.0])
    }

    pub fn set_x(&mut self, x: f64) {
        self.data.x = x;
    }

    pub fn set_y(&mut self, y: f64) {
        self.data.y = y;
    }

    pub fn add_to_gradient(&mut self, gradient: SMatrixView<f64, 1, 2>) {
        // Panic if nan or inf is encountered
        assert!(gradient.iter().all(|x| x.is_finite()));
        self.gradient += gradient.transpose();
    }
}

impl Parametric for Point2 {
    fn references(&self) -> Vec<std::rc::Rc<std::cell::RefCell<dyn Parametric>>> {
        vec![]
    }

    fn zero_gradient(&mut self) {
        self.gradient = Vector2::zeros();
    }

    fn step(&mut self, step_size: f64) {
        self.data -= step_size * self.gradient;
    }
}
