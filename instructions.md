# Drawing Shapes in Rust

## Instructions

The purpose of this exercise is to create an image like the example below:

![example](./preview.png)

You will need to do the following:

Create the following **traits**:

- **`Drawable`**  
  Contains the methods:
  - `draw`
  - `color`

- **`Displayable`**  
  Contains the method:
  - `display`

Define them according to the way they are called in the `main.rs` function.

---

In order to compile and run `main.rs`, you'll need to define some **structures**.  

Each structure must provide an associated function `new`:

- **Point** → created from two `i32` values.  
- **Line** → created from references to two different points.  
- **Triangle** → created from references to three different points.  
- **Rectangle** → created from references to two different points.  
- **Circle** → created from a reference to a point (center) and an `i32` radius.  

⚡ Each shape must be drawn in a different color.  

> 💡 Don’t forget to add the required dependencies in your `Cargo.toml`.

---

## Bonus

Optionally, you can also implement:

- **Pentagon**
- **Cube**

---

## Usage

```rust
mod geometrical_shapes;

use geometrical_shapes as gs;
use gs::{Displayable, Drawable};
use raster::{Color, Image};

fn main() {
    let mut image = Image::blank(1000, 1000);

    gs::Line::random(image.width, image.height).draw(&mut image);

    gs::Point::random(image.width, image.height).draw(&mut image);

    let rectangle = gs::Rectangle::new(&gs::Point::new(150, 300), &gs::Point::new(50, 60));
    rectangle.draw(&mut image);

    let triangle = gs::Triangle::new(
        &gs::Point::new(500, 500),
        &gs::Point::new(250, 700),
        &gs::Point::new(700, 800),
    );
    triangle.draw(&mut image);

    for _ in 1..50 {
        gs::Circle::random(image.width, image.height).draw(&mut image);
    }

    raster::save(&image, "image.png").unwrap();
}

impl Displayable for Image {
    fn display(&mut self, x: i32, y: i32, color: Color) {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            self.set_pixel(x, y, color).unwrap();
        }
    }
}
