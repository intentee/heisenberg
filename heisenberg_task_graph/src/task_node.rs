pub struct TaskNode<TValue> {
    pub dependencies: Vec<usize>,
    pub value: TValue,
}
