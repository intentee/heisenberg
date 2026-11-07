macro_rules! pair {
    ($first:ident, $second:ident) => {
        pub struct $first;
        pub struct $second;
    };
}

pair!(Pair, Unpaired);
