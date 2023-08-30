use nalgebra::{Point2, Vector2};

pub trait Canvas<ColorType>
where
    ColorType: Copy,
{
    fn initialize(&mut self);
    fn set_pixel(&mut self, point: Point2<usize>, color: ColorType);
    fn draw_line(&mut self, p1: Point2<f64>, p2: Point2<f64>, color: ColorType) {
        let mut x1 = p1.x;
        let mut y1 = p1.y;
        let x2 = p2.x;
        let y2 = p2.y;

        let dx = (x2 - x1).abs();
        let dy = (y2 - y1).abs();

        let sx = if x1 < x2 { 1.0 } else { -1.0 };
        let sy = if y1 < y2 { 1.0 } else { -1.0 };

        let mut err = dx - dy;

        loop {
            self.set_pixel(Point2::new(x1 as usize, y1 as usize), color);

            if x1 == x2 && y1 == y2 {
                break;
            }

            let e2 = 2.0 * err;

            if e2 > -dy {
                err -= dy;
                x1 += sx;
            }

            if e2 < dx {
                err += dx;
                y1 += sy;
            }
        }
    }
    fn draw_rotated_rectangle(&mut self, p1: Point2<f64>, p2: Point2<f64>, thickness: f64, color: ColorType) {
        let direction = p2 - p1;
        let direction = direction.normalize();
        let normal = Vector2::new(-direction.y, direction.x);
        let half_thickness = thickness / 2.0;

        let corner1 = p1 + normal * half_thickness;
        let corner2 = p1 - normal * half_thickness;
        let corner3 = p2 - normal * half_thickness;
        let corner4 = p2 + normal * half_thickness;

        self.draw_filled_polygon(&[corner1, corner2, corner3, corner4], color);

    }

    fn draw_filled_polygon(&mut self, poly: &[Point2<f64>], color: ColorType);
}
