enum Shape {
    Circle,
}

use Shape::Circle;

pub fn variant() -> bool {
    matches!(Shape::Circle, Circle)
}
