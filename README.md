# ISOtope

The **I**terative **SO**lver for 2D Sketches. This project is a dependency free gradient descent based solver. It works by minimizing the energy of virtual springs that represent the constraints.

- Keeps it simple and stupid. Most PCs are more than powerful enough to solve small sketches in real time. We don't need to overcomplicate things.
- Each Sketch is a list of primitives and constraints.
- Supported primitives
    - [x] Point
    - [x] Line
    - [x] Circle
    - [x] Arc
- Each constraint is a virtual spring and has a function to calculate its energy (or call it loss function) and the gradient. Current constraints are:
    - [x] Distance
    - [x] Angle
    - [x] Horizontal
    - [x] Vertical
    - [x] Coincident
    - [x] Parallel
    - [x] Perpendicular
    - [x] Fix
    - [x] Equal length
    - [x] Concentric (can be done by using same ref for center points for circle and arcs)
    - [ ] Tangent

## Todo

- [ ] For perpendicular and parallel constraint: Normalize directions, such that they do not pull to length 0
- [ ] Gradient descent seems to be to slow for more complex sketches. Use conjugate gradient instead to ensure a constant progress for each constraint.
- [ ] Get the rectangle rotated test case working

## Algorithm

The algorithm is quite simple. First, we create a Sketch that constists of primitives. Primitives can also reference each other. Take for example a line that is defined by two points. If one of the points moves, the line should also move. This is done by creating a reference to the point in the line. The line will then always calculate its position based on the points.

Consider for example a simple sketch consisting of three points $p_a$, $p_b$ and $p_c$, and two lines $l_1$ and $l_2$. The vector $q$ would look like this:

$$
p_a = [x_1, y_1]
$$

$$
p_b = [x_2, y_2]
$$

$$
p_c = [x_3, y_3]
$$

$$
l_1 = \overline{p_a p_b}
$$

$$
l_2 = \overline{p_b p_c}
$$

Now we take all these parameters and put them into a big vector $q$. This vector is the current state of our sketch. 

$$
q = [x_1, y_1, x_2, y_2, x_3, y_3]
$$

This is great, now all we have to do is to find the value for $q$ that satisfies all the constraints. We do this by minimizing the energy of the constraints. The energy of a constraint is a function that takes the current state of the sketch and returns a scalar value. This value is the energy of the constraint. The gradient of the energy is a vector that points in the direction of the steepest ascent. We can use this gradient to update the state of the sketch. 

If you want to have a visual interpretation, think of physical springs. [The energy of a spring is given by](https://en.wikipedia.org/wiki/Hooke%27s_law#Spring_energy)

$$
E = 0.5 * k * x^2
$$

We model all our constraints the same way. Take for example a coincident constraint.

$$
L(x_1, y_1, x_2, y_2) = 0.5 * {(x_1 - x_2)^2 + (y_1 - y_2)^2}
$$

Or a horizontal constraint

$$
L(x_1, y_1, x_2, y_2) = 0.5 * (y_1 - y_2)^2
$$

Some constraints are more complex, like a distance constraint, but in the end all of them are a function that map the current state of the sketch to a scalar value which we want to minimize.

Now all we have to do is a simple gradient descent

$$
q_{t+1} = q_t - \alpha \sum_i \nabla L_i(q_t)
$$

Where $\alpha$ is the step size (typicall 0.001) and $\nabla L_i(q_t)$ is the gradient of the energy of the constraints. If we do this often enough, e.g. 100000 times, we will find a state of the sketch that satisfies all the constraints as much as possible. It ends up in the same phyical state as a spring system would. Even though 100000 times sounds a like a lot, it is not for a modern PC. It just takes a couple of milliseconds.

In case of conflicting constraints, we can also figure out which constraints are the ones that actually cause the conflict. Constraints that are satisfied will have a energy/loss of 0.

$$
L_i(q) = 0
$$

constraints that are in conflict will have a non-zero energy/loss.

$$
L_i(q) > 0
$$

and can be highlighted to the user.

## Usage

Check out the examples folder at [src/examples](src/examples) for more examples.

```rust
pub fn test_rectangle_axis_aligned() {
    let mut sketch = Sketch::new();

    let point_a = Rc::new(RefCell::new(Point2::new(0.0, 0.0)));
    let point_b = Rc::new(RefCell::new(Point2::new(0.0, 0.0)));
    let point_c = Rc::new(RefCell::new(Point2::new(0.0, 0.0)));
    let point_d = Rc::new(RefCell::new(Point2::new(0.0, 0.0)));

    sketch.add_primitive(point_a.clone());
    sketch.add_primitive(point_b.clone());
    sketch.add_primitive(point_c.clone());
    sketch.add_primitive(point_d.clone());

    let line_a = Rc::new(RefCell::new(Line::new(point_a.clone(), point_b.clone())));
    let line_b = Rc::new(RefCell::new(Line::new(point_b.clone(), point_c.clone())));
    let line_c = Rc::new(RefCell::new(Line::new(point_c.clone(), point_d.clone())));
    let line_d = Rc::new(RefCell::new(Line::new(point_d.clone(), point_a.clone())));

    sketch.add_primitive(line_a.clone());
    sketch.add_primitive(line_b.clone());
    sketch.add_primitive(line_c.clone());
    sketch.add_primitive(line_d.clone());

    // Fix point a to origin
    sketch.add_constraint(Rc::new(RefCell::new(FixPoint::new(
        point_a.clone(),
        Vector2::new(0.0, 0.0),
    ))));

    // Constrain line_a and line_c to be horizontal
    sketch.add_constraint(Rc::new(RefCell::new(
        HorizontalLine::new(line_a.clone()),
    )));
    sketch.add_constraint(Rc::new(RefCell::new(
        HorizontalLine::new(line_c.clone()),
    )));

    // Constrain line_b and line_d to be vertical
    sketch.add_constraint(Rc::new(RefCell::new(
        VerticalLine::new(line_b.clone()),
    )));
    sketch.add_constraint(Rc::new(RefCell::new(
        VerticalLine::new(line_d.clone()),
    )));

    // Constrain the length of line_a to 2
    sketch.add_constraint(Rc::new(RefCell::new(
        HorizontalDistanceBetweenPoints::new(point_a.clone(), point_b.clone(), 2.0),
    )));

    // Constrain the length of line_b to 3
    sketch.add_constraint(Rc::new(RefCell::new(
        VerticalDistanceBetweenPoints::new(point_a.clone(), point_d.clone(), 3.0),
    )));

    // Now solve the sketch
    sketch.solve(0.001, 100000);

    println!("point_a: {:?}", point_a.as_ref().borrow());
    println!("point_b: {:?}", point_b.as_ref().borrow());
    println!("point_c: {:?}", point_c.as_ref().borrow());
    println!("point_d: {:?}", point_d.as_ref().borrow());

    assert!(
        (point_a.as_ref().borrow().data() - Vector2::new(0.0, 0.0)).norm() < 0.001
    );
    assert!(
        (point_b.as_ref().borrow().data() - Vector2::new(2.0, 0.0)).norm() < 0.001
    );
    assert!(
        (point_c.as_ref().borrow().data() - Vector2::new(2.0, 3.0)).norm() < 0.001
    );
    assert!(
        (point_d.as_ref().borrow().data() - Vector2::new(0.0, 3.0)).norm() < 0.001
    );
}
```
