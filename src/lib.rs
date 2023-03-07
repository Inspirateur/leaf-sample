use std::ops::Range;
mod leaf_sample;
use leaf_sample::GoldenSerie;

pub fn range_into<T: Into<f64> + Copy>(range: Range<T>) -> Range<f64> {
    return Range {
        start: range.start.into(),
        end: range.end.into(),
    };
}

// Waiting for https://github.com/rust-lang/rust/issues/83527
macro_rules! count {
    () => { 0 };
    ($a:expr) => { 1 };
    ($odd:expr, $($a:expr, $b:expr),*) => {
        (count!($($a)*) << 1) | 1
    };
    ($($a:expr, $even:expr)*) => {
        (count!($($a)*) << 1)
    };
}

#[macro_export]
macro_rules! leaf_sample {
    ( $( $x:expr ),* ) => {
        GoldenSerie::new([$(range_into($x)),*])
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn macro_works() {
        for [hue, value] in leaf_sample!(0..360, 0.6..1.0).take(10) {
            println!("{hue}, {value}");
        }
    }
}
