use std::{array::from_fn, ops::Range};

fn golden(d: usize) -> f64 {
    // Compute the generalised golden ratio
    // => the unique positive root of x^(d+1) = x + 1
    let p = 1. / (d as f64 + 1.);
    let mut res = 1.;
    for _ in 0..50 {
        res = (res + 1f64).powf(p);
    }
    return res;
}

pub(crate) struct GoldenSerie<const D: usize = 1> {
    vals: [f64; D],
    i: usize,
    ampls: [f64; D],
    shifts: [f64; D],
}

impl<const D: usize> GoldenSerie<D> {
    pub fn new<T: Into<f64> + Copy>(bounds: [Range<T>; D]) -> Self {
        GoldenSerie {
            vals: from_fn(|i| 1. / golden(i + 1).powi(i as i32 + 1)),
            i: 1,
            ampls: bounds
                .clone()
                .map(|bound| bound.end.into() - bound.start.into()),
            shifts: bounds.map(|bound| bound.start.into()),
        }
    }
}

impl<const D: usize> Iterator for GoldenSerie<D> {
    type Item = [f32; D];

    fn next(&mut self) -> Option<Self::Item> {
        let res = from_fn(|i| {
            ((self.i as f64 * self.vals[i]).fract() * self.ampls[i] + self.shifts[i]) as f32
        });
        self.i += 1;
        Some(res)
    }
}
