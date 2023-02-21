mod leaf_sample;
use leaf_sample::LeafSample;
use itertools::izip;

#[macro_export]
macro_rules! leaf_sample {
    ( $( $x:expr ),* ) => {
        izip!($(LeafSample::new($x)),*)
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        for (hue, value) in leaf_sample!(0..360, 0.6..1.0).take(10) {
            println!("{hue}, {value}");
        }
    }
}