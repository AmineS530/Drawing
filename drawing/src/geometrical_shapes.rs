use rand::Rng;
use raster::{Color, Image};
use std::sync::Mutex;


// for no duplicate colors
static USED_COLORS: Mutex<Vec<Color>> = Mutex::new(Vec::new()); // Mutex to protect access to used colors

/* Traits */
pub trait Drawable {
    fn draw(&self, image: &mut raster::Image);

    fn color() -> Color {
        let mut rng = rand::thread_rng();

        loop {
            let r = rng.gen_range(0..=255);
            let g = rng.gen_range(0..=255);
            let b = rng.gen_range(0..=255);

            let c = Color { r, g, b, a: 255 };

            let mut used = USED_COLORS.lock().unwrap(); // accès protégé
            if !used.iter().any(|col| col.r == c.r && col.g == c.g && col.b == c.b) {
                used.push(c.clone());
                return c;
            }
        }
    }
}
pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

/* Implementations */
/********************* Point ********************/
#[derive(Clone, Debug)]
pub struct Point(pub i32, pub i32); // point (x, y)

impl Point {
    // Create a new point
    pub fn new(a: i32, b: i32) -> Self {
        Self(a, b)
    }

    // Create a random point in a given width and height
    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0..width);
        let y = rng.gen_range(0..height);

        Self(x, y)
    }
}

impl Drawable for Point {
    fn draw(&self, image: &mut Image) {
        let color = Self::color();
        if self.0 >= 0 && self.0 < image.width && self.1 >= 0 && self.1 < image.height {
            image.display(self.0, self.1, color);
        }
    }

   
}

/********************* Line ********************/
#[derive(Clone, Debug)]
pub struct Line {            // line from point A to point B
    pub first_p: Point,
    pub sec_p: Point,
    pub color: Color,
}


impl Line {
    // Create a new line from two points
    pub fn new(a: &Point, b: &Point) -> Self {
        Self {
            first_p: a.clone(),
            sec_p: b.clone(),
            color: Self::color(),
        }
    }

    // Create a random line using two random points
    pub fn random(width: i32, height: i32) -> Self {
        let p1 = Point::random(width, height);
        let p2 = Point::random(width, height);
        Self::new(&p1, &p2)
    }
}

impl Drawable for Line {
    fn draw(&self, image: &mut Image) {
        let color = self.color.clone();
        let mut x0 = self.first_p.0;
        let mut y0 = self.first_p.1;
        let x1 = self.sec_p.0;
        let y1 = self.sec_p.1;

        let dx = (x1 - x0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let dy = -(y1 - y0).abs();
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;

        loop {
            if x0 >= 0 && x0 < image.width && y0 >= 0 && y0 < image.height {
                image.display(x0, y0, color.clone());
            }
            if x0 == x1 && y0 == y1 {
                break;
            }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x0 += sx;
            }
            if e2 <= dx {
                err += dx;
                y0 += sy;
            }
        }
    }

  
}


/********************* Triangle ********************/
#[derive(Clone, Debug)]
pub struct Triangle {
    pub first_p: Point,
    pub sec_p: Point,
    pub third_p: Point,
    pub color: Color,
}

impl Triangle {
    pub fn new(a: &Point, b: &Point, c: &Point) -> Self {
        Self {
            first_p: a.clone(),
            sec_p: b.clone(),
            third_p: c.clone(),
            color: Self::color(),
        }
    }
}

impl Drawable for Triangle {
    fn draw(&self, image: &mut Image) {
        let mut line1 = Line::new(&self.first_p, &self.sec_p);
        let mut line2 = Line::new(&self.sec_p, &self.third_p);
        let mut line3 = Line::new(&self.third_p, &self.first_p);

        line1.color = self.color.clone();
        line2.color = self.color.clone();
        line3.color = self.color.clone();
        line1.draw(image);
        line2.draw(image);
        line3.draw(image);
    }

}

/********************* Rectangle ********************/
#[derive(Clone, Debug)]
pub struct Rectangle {
    pub first_p: Point,
    pub sec_p: Point,
    pub color: Color,
}

impl Rectangle {
    pub fn new(a: &Point, b: &Point) -> Self {
        Self {
            first_p: a.clone(),
            sec_p: b.clone(),
            color: Self::color(),
        }
    }
}

impl Drawable for Rectangle {
    fn draw(&self, image: &mut Image) {
        let a = Point::new(self.first_p.0, self.first_p.1);
        let b = Point::new(self.sec_p.0, self.first_p.1);
        let c = Point::new(self.sec_p.0, self.sec_p.1);
        let d = Point::new(self.first_p.0, self.sec_p.1);

        let points = [&a, &b, &c, &d];
        for i in 0..points.len() {
            let start = points[i];
            let end = points[(i + 1) % points.len()];
            let mut line = Line::new(start, end);
            line.color = self.color.clone();
            line.draw(image);
        }
    }
}

/********************* Circle ********************/
#[derive(Clone, Debug)]
pub struct Circle {
    pub center: Point,
    pub radius: i32,
    pub color: Color,
}

impl Circle {
    pub fn new(center: Point, radius: i32) -> Self {
        Self { center, radius, color: Self::color() }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::thread_rng();
        Self::new(
            Point::random(width, height),
            rng.gen_range(0..width.min(height) / 2),
        )
    }
}

impl Drawable for Circle {
    fn draw(&self, image: &mut Image) {
        let cx = self.center.0;
        let cy = self.center.1;
        let r = self.radius;
        let color = self.color.clone();

        // Using Bresenham's circle algorithm
        let mut x = 0;
        let mut y = r;
        let mut d = 3 - 2 * r;

        while y >= x {
            // Draw eight symmetric points
            if cx + x >= 0 && cx + x < image.width && cy + y >= 0 && cy + y < image.height {
                image.display(cx + x, cy + y, color.clone());
            }
            if cx + x >= 0 && cx + x < image.width && cy - y >= 0 && cy - y < image.height {
                image.display(cx + x, cy - y, color.clone());
            }
            if cx - x >= 0 && cx - x < image.width && cy + y >= 0 && cy + y < image.height {
                image.display(cx - x, cy + y, color.clone());
            }
            if cx - x >= 0 && cx - x < image.width && cy - y >= 0 && cy - y < image.height {
                image.display(cx - x, cy - y, color.clone());
            }
            if cx + y >= 0 && cx + y < image.width && cy + x >= 0 && cy + x < image.height {
                image.display(cx + y, cy + x, color.clone());
            }
            if cx + y >= 0 && cx + y < image.width && cy - x >= 0 && cy - x < image.height {
                image.display(cx + y, cy - x, color.clone());
            }
            if cx - y >= 0 && cx - y < image.width && cy + x >= 0 && cy + x < image.height {
                image.display(cx - y, cy + x, color.clone());
            }
            if cx - y >= 0 && cx - y < image.width && cy - x >= 0 && cy - x < image.height {
                image.display(cx - y, cy - x, color.clone());
            }

            if d <= 0 {
                d += 4 * x + 6;
            } else {
                d += 4 * (x - y) + 10;
                y -= 1;
            }
            x += 1;
        }
    }

}

/********************* Pentagon ********************/
#[derive(Clone, Debug)]
pub struct Pentagon {
    pub center: Point,
    pub radius: i32,
    pub color: Color,
}

impl Pentagon {
    pub fn new(center: Point, radius: i32) -> Self {
        Self {
            center,
            radius,
            color: Self::color(),
        }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::thread_rng();
        Self::new(
            Point::random(width, height),
            rng.gen_range(50..width.min(height) / 6),
        )
    }
}

impl Drawable for Pentagon {
    fn draw(&self, image: &mut Image) {
        let center_x = self.center.0;
        let center_y = self.center.1;
        let radius = self.radius as f64;
        let color = self.color.clone();

        // Calculate pentagon points
        let mut points = Vec::new();
        for i in 0..5 {
            let angle = (i as f64) * 2.0 * std::f64::consts::PI / 5.0 - std::f64::consts::PI / 2.0;
            let x = (center_x as f64 + radius * angle.cos()) as i32;
            let y = (center_y as f64 + radius * angle.sin()) as i32;
            points.push(Point::new(x, y));
        }

        // Draw the pentagon using lines
        for i in 0..5 {
            let mut line = Line::new(&points[i], &points[(i + 1) % 5]);
            line.color = color.clone();
            line.draw(image);
        }
    }
}
