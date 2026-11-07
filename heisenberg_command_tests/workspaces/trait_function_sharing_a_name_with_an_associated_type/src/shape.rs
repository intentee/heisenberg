pub trait Shape {
    type 面积;

    fn perimeter(&self) -> u32;

    fn 面积(&self) -> Self::面积;
}
