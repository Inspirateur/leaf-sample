# leaf-spread [WIP]
A Rust crate to iterate indefinitely over well spread points, similar to some leaf arrangement pattern in nature.

## Applications
Ideal for generating distinct colors.

|                        | Gradient       | Poisson Disk  |  Leaf spread  |
| :---                   |     :---:      |     :---:     |     :---:     | 
| Iterate indefinitely   | ❌             | ❌           |  ✅           |
| Random Looking         | ❌             | ✅           |  ❌           |
| Fast                   | ✅             | ❌           |  ✅           |

## Usage
```rust
use leaf_spread::sample_iter;
use palette::Hsv;

fn main() {
  for (hue, value) in sample_iter!(0..360, 0.6..1.0) {
    let color = Hsv::new(hue, 0.8, value);
    // use the color however
  }
```

## How ?
It's Golden ratio shenaningans
