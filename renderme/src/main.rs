use image::{RgbImage, Rgb};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_compilation() {
        let mut img = RgbImage::new(800, 800);
        let green = [0, 255, 0];
        let red = [255, 0, 0];
        let blue = [0, 0, 255];
        // line(1, 1, 300, 400, &mut img, &red);
        let x = Line::new(200, 12, 300, 400);
        x.draw(&mut img, &red);
        let y = Line::new(40, 400, 200, 12);
        y.draw(&mut img, &green);
        save_file(String::from("test1.tga"), img);
    }
}

#[derive(Copy, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Copy, Clone)]
pub struct Line {
    pub p0: Point,
    pub p1: Point,
}

impl Line {
    pub fn draw_under_45_degrees<F: FnMut(u32, u32)>(x0: i32, x1: i32, y0: i32, y1: i32, dx: i32, dy: i32, mut imgput: F) {
        let (lx, ly, rx, ry) = if x0 > x1 { // Going left to right
            (x1, y1, x0, y0) } else {(x0, y0, x1, y1)};
        let mut y = ly; // compiler will anyway replace y with y0
        let mut error = 0;
        for x in lx..rx {
            imgput(x as u32, y as u32);
            error += 2*dy;
            if error > dx {
                match ry>ly{
                    true => y+=1,
                    false => y-=1,
                }
                error -= 2*dx;
            }
        }
    }

    pub fn draw(&self, img: &mut RgbImage, color: &[u8; 3]) {
        let dx: i32 = (self.p0.x-self.p1.x).abs();
        let dy: i32 = (self.p0.y-self.p1.y).abs();
        if dx > dy {
            Line::draw_under_45_degrees(self.p0.x, self.p1.x, self.p0.y, self.p1.y, dx, dy, |x: u32, y: u32| {img.put_pixel(x, y, Rgb(*color))});
        } else {
            Line::draw_under_45_degrees(self.p0.y, self.p1.y, self.p0.x, self.p1.x, dy, dx, |x: u32, y: u32| {img.put_pixel(y, x, Rgb(*color))});
        }
 
    }

    pub fn new(x0: i32, y0: i32, x1: i32, y1: i32) -> Line {
        if x0 < x1 {
            Line {
                p0: Point{x:x0, y:y0},
                p1: Point{x:x1, y:y1},
            }
        } else if x0 > x1 {
            Line {
                p0: Point{x:x1, y:y1},
                p1: Point{x:x0, y:y0},
            }
        } else {
            if y0 > y1 {
                Line {
                    p0: Point{x:x1, y:y1},
                    p1: Point{x:x0, y:y0},
                }
            } else {
                Line {
                    p0: Point{x:x0, y:y0},
                    p1: Point{x:x1, y:y1},
                }

            }
        }
    }
}

pub fn save_file(filename: String, img: RgbImage) {
    img.save(filename).unwrap()
}

fn main() {
    println!("Hello, world!");
}
