use std::ops::Range;

pub struct LeafSample {
    i: usize,
    ampl: f64,
    shift: f64
}

impl LeafSample {
    pub fn new<T: Into<f64> + Copy>(bounds: Range<T>) -> Self {
        LeafSample { 
            i: 0, 
            ampl: bounds.end.into()-bounds.start.into(),
            shift: bounds.start.into() 
        }
    }
}

impl Iterator for LeafSample {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let dec = (self.i as f64*(1.0_f64 + 5.0_f64.sqrt())/2.0_f64).fract();
        let res = (dec*self.ampl + self.shift) as f32;
        self.i += 1;
        Some(res)
    }
}