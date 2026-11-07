use crate::shape::Shape;

pub struct Square {
    pub side: u32,
}

impl Shape for Square {
    type 面积 = u32;

    fn perimeter(&self) -> u32 {
        self.side * 4
    }

    fn 面积(&self) -> u32 {
        self.side * self.side
    }
}
