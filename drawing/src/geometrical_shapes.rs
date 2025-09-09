/* Types declarations */
pub struct Point(pub i32,pub i32);
pub struct Line {
    x: Point,
    y: Point,
}

pub struct Triangle {
    x: Point,
    y: Point,
    z: Point,
}

pub struct Rectangle {
   x:Point,
   y:Point,
}

pub struct Circle {
    center:Point,
    radius:i32,
}
/* Traits */
pub trait Drawable {
    //  fn draw (){}
    //  fn color (){}
}

pub trait Displayable {
    //  fn display(){}
}
/* Implementations */
// Remove the dashes _ when you work on something
impl Line {
    pub fn random(_width:i32,_height:i32){}
}
impl Point {
    pub fn new(a:i32,b:i32) -> Point{
        Point(a, b)
    }
    pub fn random(_width:i32,_height:i32){}
}
impl Circle {
    pub fn random(_width:i32,_height:i32){}
}
impl Triangle {
    pub fn new(_a:Point,_b:Point,_c:Point){}
}
impl Rectangle {
    pub fn new(_a:Point,_b:Point){}
}