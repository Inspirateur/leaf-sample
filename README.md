# leaf-sample
A Rust crate to iterate indefinitely over well spread points, similar to some leaf arrangement pattern in nature.

## Applications
Ideal for generating a variable amount of distinct colors.

|                        | Grid sample       | Poisson Disk  |  Leaf sample  |
| :---                   |     :---:      |     :---:     |     :---:     | 
| Iterate indefinitely   | ❌             | ❌           |  ✅           |
| Random Looking         | ❌             | ✅           |  ❌           |
| Fast                   | ✅             | ❌           |  ✅           |

## Usage
```rust
use leaf_spread::leaf_sample;
use palette::Hsv;

fn main() {
  for [hue, value] in leaf_sample!(0..360, 0.6..1.0).take(10) {
    let color = Hsv::new(hue, 0.8, value);
    // use the color however
  }
}
```

## How ?
Golden ratio shenaningans from http://extremelearning.com.au/unreasonable-effectiveness-of-quasirandom-sequences/
