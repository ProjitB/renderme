use image::{RgbImage, Rgb};

#[derive(Copy, Clone)]
pub struct Point { // Screen Point
    pub x: u32,
    pub y: u32,
}

#[derive(Copy, Clone)]
pub struct Line {
    pub p0: Point,
    pub p1: Point,
}

impl Line {
    pub fn draw_under_45_degrees<F: FnMut(u32, u32)>(x0: u32, x1: u32, y0: u32, y1: u32, dx: i32, dy: i32, mut imgput: F) {
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
        let dx: i32 = (self.p0.x as i32 - self.p1.x as i32).abs();
        let dy: i32 = (self.p0.y as i32 - self.p1.y as i32).abs();
        if dx > dy {
            Line::draw_under_45_degrees(self.p0.x, self.p1.x, self.p0.y, self.p1.y, dx, dy, |x: u32, y: u32| {img.put_pixel(x, y, Rgb(*color))});
        } else { // Swap stuff around so it's like a rotation
            Line::draw_under_45_degrees(self.p0.y, self.p1.y, self.p0.x, self.p1.x, dy, dx, |x: u32, y: u32| {img.put_pixel(y, x, Rgb(*color))});
        }
 
    }

    pub fn new(x0: u32, y0: u32, x1: u32, y1: u32) -> Line {
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
