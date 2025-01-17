use std::{cell::RefCell, rc::Rc};

use nalgebra::SMatrix;

use crate::{
    constraints::Constraint,
    primitives::line::Line,
};

// This is a sketch constraint that makes the end point of an arc coincident with a point.
#[derive(Debug)]
pub struct VerticalLine {
    line: Rc<RefCell<Line>>,
}

impl VerticalLine {
    pub fn new(line: Rc<RefCell<Line>>) -> Self {
        Self { line }
    }

    pub fn line(&self) -> Rc<RefCell<Line>> {
        self.line.clone()
    }

    pub fn set_line(&mut self, line: Rc<RefCell<Line>>) {
        self.line = line;
    }
}

impl Constraint for VerticalLine {
    fn references(&self) -> Vec<Rc<RefCell<dyn crate::primitives::Parametric>>> {
        vec![self.line.clone()]
    }

    fn loss_value(&self) -> f64 {
        let start = self.line.borrow().start().borrow().data();
        let end = self.line.borrow().end().borrow().data();
        let dx = end.x - start.x;
        0.5 * dx * dx
    }

    fn update_gradient(&mut self) {
        let start = self.line.borrow().start().borrow().data();
        let end = self.line.borrow().end().borrow().data();
        let dx = end.x - start.x;

        let gradient_constraint = SMatrix::<f64, 1, 2>::from_row_slice(&[dx, 0.0]);

        let grad_start = self.line.borrow().start_gradient();
        let grad_end = self.line.borrow().end_gradient();

        self.line
            .borrow_mut()
            .add_to_gradient((-gradient_constraint * grad_start).as_view());
        self.line
            .borrow_mut()
            .add_to_gradient((gradient_constraint * grad_end).as_view());
    }
}

// Run some tests
#[cfg(test)]
mod tests {
    use std::{cell::RefCell, rc::Rc};

    use crate::{
        constraints::lines::vertical_line::VerticalLine,
        primitives::{line::Line, point2::Point2},
        sketch::Sketch,
    };

    #[test]
    fn test_vertical_line() {
        let mut sketch = Sketch::new();

        let line_start = Rc::new(RefCell::new(Point2::new(3.0, 4.0)));
        let line_end = Rc::new(RefCell::new(Point2::new(5.0, 6.0)));
        let line = Rc::new(RefCell::new(Line::new(
            line_start.clone(),
            line_end.clone(),
        )));
        sketch.add_primitive(line_start.clone());
        sketch.add_primitive(line_end.clone());
        sketch.add_primitive(line.clone());

        let constr1 = VerticalLine::new(line.clone());
        sketch.add_constraint(Rc::new(RefCell::new(constr1)));

        sketch.solve(0.001, 100000);

        println!("line: {:?}", line.as_ref().borrow());

        assert!(
            (line.as_ref().borrow().end().borrow().data().x - line.as_ref().borrow().start().borrow().data().x)
                .abs()
                < 1e-6
        );
    }
}
