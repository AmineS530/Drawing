use rand::Rng;
use raster::{Color, Image};

/* Types declarations */
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Point(pub i32, pub i32);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Line {
    pub first_p: Point,
    pub sec_p: Point,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Triangle {
    pub first_p: Point,
    pub sec_p: Point,
    pub third_p: Point,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Rectangle {
   pub first_p: Point,
   pub sec_p: Point,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Circle {
    pub center: Point,
    pub radius: i32,
}

/* Traits */
pub trait Drawable {

    fn draw(&self, image: &mut raster::Image);

    fn color(&self) -> raster::Color {
        raster::Color::black()
    }
    
}

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: raster::Color);
}

/* Implementations */

impl Point {
    // Create a new point
    pub fn new(a: i32, b: i32) -> Point {
        Point(a, b)
    }

    // Create a random point in a given width and height
    pub fn random(width: i32, height: i32) -> Point {

        let mut rng = rand::thread_rng();
        let x = rng.gen_range(0..width);
        let y = rng.gen_range(0..height);

        Point(x, y)
    }
}

impl Drawable for Point {
    fn draw(&self, image: &mut raster::Image) {
        let color = self.color();
        if self.0 >= 0 && self.0 < image.width && self.1 >= 0 && self.1 < image.height {
            image.set_pixel(self.0, self.1, color).unwrap();
        }
    }

    fn color(&self) -> Color {
        Color::rgb(255, 0, 0)
    }
}

impl Displayable for Point {
    fn display(&mut self, x: i32, y: i32, color: Color) {
        self.0 = x;
        self.1 = y;
    }
}

impl Line {
    // Create a new line from two points
    pub fn new(a: &Point, b: &Point) -> Line {
        Line { 
            first_p: a.clone(), 
            sec_p: b.clone() 
        }
    }

    // Create a random line using two random points
    pub fn random(width: i32, height: i32) -> Line {
        let p1 = Point::random(width, height);
        let p2 = Point::random(width, height);
        Line::new(&p1, &p2)
    }
}

impl Drawable for Line {
    fn draw(&self, image: &mut raster::Image) {
        let color = self.color();

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
                image.set_pixel(x0, y0, color.clone()).unwrap();
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

    fn color(&self) -> Color {
        Color::rgb(0, 0, 255) // blue
    }
}

impl Displayable for Line {
    fn display(&mut self, x: i32, y: i32, color: Color) {

        self.first_p.0 = x;
        self.first_p.1 = y;
        self.sec_p.0 = x + 1;
        self.sec_p.1 = y + 1;

    }
}

impl Triangle {
    pub fn new(a: &Point, b: &Point, c: &Point) -> Triangle {
        Triangle { 
            first_p: a.clone(), 
            sec_p: b.clone(), 
            third_p: c.clone() 
        }
    }
}

impl Drawable for Triangle {
    fn draw(&self, image: &mut raster::Image) {
        let line1 = Line::new(&self.first_p, &self.sec_p);
        let line2 = Line::new(&self.sec_p, &self.third_p);
        let line3 = Line::new(&self.third_p, &self.first_p);

        line1.draw(image);
        line2.draw(image);
        line3.draw(image);
    }

    fn color(&self) -> Color {
        Color::rgb(0, 255, 0) // green
    }
}

impl Displayable for Triangle {
    fn display(&mut self, x: i32, y: i32, color: Color) {
        self.first_p.0 = x;
        self.first_p.1 = y;
        self.sec_p.0 = x + 1;
        self.sec_p.1 = y + 1;
        self.third_p.0 = x + 2;
        self.third_p.1 = y + 2;
    }
}


// Remove the dashes _ when you work on something
/*
impl Circle {
    pub fn random(_width:i32,_height:i32){}
}

impl Rectangle {
    pub fn new(_a:Point,_b:Point){}
}

impl Drawable for Circle {
    fn draw(&self, _image: &mut raster::Image) {}
}

impl Drawable for Rectangle {
    fn draw(&self, _image: &mut raster::Image) {}
}

impl Displayable for Circle {}
impl Displayable for Rectangle {}
*/