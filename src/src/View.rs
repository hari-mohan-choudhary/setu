
use stretch::node::Node;
use stretch::geometry::Size;
use stretch::style::*;
use tiny_skia::{Color, Paint, Pixmap};

pub struct View {
    node: Node,
    width: u32,
    height: u32,
    color: Color,
}

impl View {
    pub fn render(&self, pixmap: &mut Pixmap, layout: &stretch::result::Layout, parent_width: f32, parent_height: f32) {
        let width = dimension_to_f32(Dimension::Points(layout.size.width), parent_width);
        let height = dimension_to_f32(Dimension::Points(layout.size.height), parent_height);
        
        let mut paint = Paint::default();
        paint.set_color(self.color);
        pixmap.fill_rect(
            tiny_skia::Rect::from_ltrb(
                layout.location.x as f32,
                layout.location.y as f32,
                layout.location.x as f32 + width,
                layout.location.y as f32 + height,
            ).unwrap(),
            &paint,
            tiny_skia::Transform::identity(),
            None,
        );
    }
}

fn dimension_to_f32(dimension: Dimension, parent_size: f32) -> f32 {
    match dimension {
        Dimension::Points(val) => val,
        Dimension::Percent(val) => val * parent_size,
        Dimension::Auto | Dimension::Undefined => 0.0, // You can define default behavior for Auto/Undefined here
    }
}



// pub mod UIComponent {
//     use tiny_skia::{Pixmap, Paint, Color};

//     pub struct View {
//         width: u32,
//         height: u32,
//         color: Color,
//     }

//     impl View {
//         pub fn new(width: u32, height: u32, color: Color) -> Self {
//             View { width, height, color }
//         }

//         pub fn render(&self, pixmap: &mut Pixmap) {
//             let mut paint = Paint::default();
//             paint.set_color(self.color);
//             pixmap.fill_rect(tiny_skia::Rect::from_ltrb(0.0, 0.0, self.width as f32, self.height as f32).unwrap(), &paint, tiny_skia::Transform::identity(), None);
//         }
//     }

//     pub struct Button {
//         label: String,
//         width: u32,
//         height: u32,
//         color: Color,
//     }

//     impl Button {
//         pub fn new(label: String, width: u32, height: u32, color: Color) -> Self {
//             Button { label, width, height, color }
//         }

//         pub fn render(&self, pixmap: &mut Pixmap) {
//             let mut paint = Paint::default();
//             paint.set_color(self.color);
//             pixmap.fill_rect(tiny_skia::Rect::from_ltrb(10.0, 10.0, self.width as f32, self.height as f32).unwrap(), &paint, tiny_skia::Transform::identity(), None);
//             // You can add text rendering logic here for the button label
//         }
//     }

//     pub struct Text {
//         content: String,
//         font_size: f32,
//         color: Color,
//     }

//     impl Text {
//         pub fn new(content: String, font_size: f32, color: Color) -> Self {
//             Text { content, font_size, color }
//         }

//         pub fn render(&self, pixmap: &mut Pixmap) {
//             // Render logic for text goes here
//             // tiny-skia does not support text rendering directly,
//             // you may need a separate library or method to draw text
//         }
//     }
// }
