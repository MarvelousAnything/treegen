use image::{RgbImage, Rgb};
use imageproc::point::Point;
use palette::Srgb;

use crate::rendering::canvas::Canvas;

impl Canvas<Srgb<u8>> for RgbImage {
    fn initialize(&mut self) {
    }

    fn set_pixel(&mut self, point: nalgebra::Point2<usize>, color: Srgb<u8>) {
        let pixel = Rgb([color.red, color.green, color.blue]);
        self.put_pixel(point.x as u32, point.y as u32, pixel);
    }

    fn draw_filled_polygon(&mut self, poly: &[nalgebra::Point2<f64>], color: Srgb<u8>) {
        let binding = poly.iter().map(|p| Point::new(p.x as i32, p.y as i32)).collect::<Vec<_>>();
        let poly = binding.as_slice();
        let color = Rgb([color.red, color.green, color.blue]);
        imageproc::drawing::draw_polygon_mut(self, poly, color);
    }

}
