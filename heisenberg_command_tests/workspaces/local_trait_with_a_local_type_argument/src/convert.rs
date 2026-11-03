pub trait Convert<TSource> {
    fn convert(source: TSource) -> Self;
}
