
use stretch::node::Node;
use stretch::geometry::Size;
use stretch::style::*;
use tiny_skia::{Color, Paint, Pixmap};

pub struct Button {
    node: Node,
    label: String,
    width: u32,
    height: u32,
    color: Color,
}

impl Button {
    pub fn new(stretch: &mut stretch::Stretch, label: String, width: u32, height: u32, color: Color) -> Self {
        let node = stretch.new_node(
            Style {
                size: Size {
                    width: Dimension::Points(width as f32),
                    height: Dimension::Points(height as f32),
                },
                ..Default::default()
            },
            vec![],
        ).unwrap();
        Button { node, label, width, height, color }
    }

    pub fn render(&self, pixmap: &mut Pixmap, layout: &stretch::result::Layout) {
        let mut paint = Paint::default();
        paint.set_color(self.color);
        pixmap.fill_rect(
            tiny_skia::Rect::from_ltrb(
                layout.location.x as f32,
                layout.location.y as f32,
                (layout.location.x + layout.size.width) as f32,
                (layout.location.y + layout.size.height) as f32,
            ).unwrap(),
            &paint,
            tiny_skia::Transform::identity(),
            None,
        );
        // You can add text rendering logic for the button label here
    }
}
