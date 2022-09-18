use image::{RgbImage};

mod line;

use line::Point as Point;


#[derive(Clone)]
pub struct ConvexPolygon<'a> {
    pub points: &'a[Point]
}



pub fn save_file(filename: String, img: RgbImage) {
    img.save(filename).unwrap()
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use line::Line as Line;
    use super::*;

    #[test]
    fn line_compilation() {
        let mut img = RgbImage::new(800, 800);
        let green = [0, 255, 0];
        let red = [255, 0, 0];
        let blue = [0, 0, 255];
        let x = Line::new(200, 12, 300, 400);
        x.draw(&mut img, &red);
        let y = Line::new(40, 400, 200, 12);
        y.draw(&mut img, &green);
        save_file(String::from("test1.tga"), img);
    }
}

