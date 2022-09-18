mod line;
mod util;
use image::{RgbImage};

use line::Point;


// #[derive(Clone)]
// pub struct ConvexPolygon<'a> {
//     pub points: &'a[Point]
// }

// impl ConvexPolygon<'_>{
//     pub fn draw(&self, img: &mut RgbImage, color: &[u8; 3]) {
//         let n = self.points.len();
//         // vec2 bbox[2] = find_bounding_box(points); 
//         // for (each pixel in the bounding box) { 
//         //     if (inside(points, pixel)) { 
//         //         put_pixel(pixel); 
//         //     } 
//     // }
        
//     }
// }


// pub fn draw_convex_polygons(vertices: Vec<i32>, img: &mut RgbImage, color: &[u8; 3]) {
    
    
// }


fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use util::save_file;
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
        let z = Line::new(40, 40, 200, 40);
        z.draw(&mut img, &blue);
        let a = Line::new(300, 300, 300, 20);
        a.draw(&mut img, &red);
        save_file(String::from("test1.tga"), img);
    }

    #[test]
    fn origin_finding() {
        let mut img = RgbImage::new(801, 801);
        let green = [0, 255, 0];
        let red = [255, 0, 0];
        let blue = [0, 0, 255];
        let x = Line::new(0, 0, 400, 400);
        x.draw(&mut img, &red);
        let y = Line::new(800, 0, 400, 400);
        y.draw(&mut img, &blue);
        let z = Line::new(0, 800, 400, 400);
        z.draw(&mut img, &green);
        save_file(String::from("test2.tga"), img);
    }
}

