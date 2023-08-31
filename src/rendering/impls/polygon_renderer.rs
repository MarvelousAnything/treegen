use palette::Srgb;

use crate::rendering::renderer::Renderer;

pub struct PolygonRenderer;

impl Renderer<Srgb<u8>> for PolygonRenderer {
    fn initialize(&mut self, _canvas: &mut dyn crate::rendering::canvas::Canvas<Srgb<u8>>) {}

    fn render_node(
        &self,
        canvas: &mut dyn crate::rendering::canvas::Canvas<Srgb<u8>>,
        node: &crate::skeleton::node::Node,
    ) {
        canvas.draw_rotated_rectangle(
            node.point,
            node.next_point(),
            node.thickness,
            Self::get_color_for_node(node),
        )
    }

    fn get_color_for_node(node: &crate::skeleton::node::Node) -> Srgb<u8> {
        if let Some(color) = node.color {
            color
        } else {
            Srgb::new(
                (node.angle.abs() * 255.0).clamp(10.0, 255.0) as u8,
                0,
                (node.angle.abs() * 255.0).clamp(10.0, 255.0) as u8,
            )
        }
    }
}
