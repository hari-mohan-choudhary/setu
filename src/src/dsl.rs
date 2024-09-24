pub mod UIComponent {
    use stretch::node::Node;
    use stretch::style::*;
    use stretch::geometry::Size;
    use tiny_skia::{Pixmap, Paint, Color};

    fn dimension_to_f32(dimension: Dimension, parent_size: f32) -> f32 {
        match dimension {
            Dimension::Points(value) => value,                      // Use value directly for Points
            Dimension::Percent(percentage) => percentage * parent_size,  // Calculate percent of parent size
            Dimension::Auto | Dimension::Undefined => 0.0,          // Default to 0 for Auto or Undefined
        }
    }

    pub struct View {
        width: u32,
        height: u32,
        color: tiny_skia::Color, // assuming you're using tiny-skia for color
    }
    
    impl View {
        // Define the `new` function for creating a new View
        pub fn new(width: u32, height: u32, color: tiny_skia::Color) -> Self {
            Self {
                width,
                height,
                color,
            }
        }
    
        // Render method as before
        pub fn render(&self, pixmap: &mut tiny_skia::Pixmap, layout: &stretch::result::Layout, parent_width: f32, parent_height: f32) {
            let mut paint = tiny_skia::Paint::default();
            paint.set_color(self.color);
    
            // Render the view based on the layout from Stretch
            pixmap.fill_rect(
                tiny_skia::Rect::from_ltrb(
                    layout.location.x as f32,
                    layout.location.y as f32,
                    layout.location.x as f32 + layout.size.width as f32,
                    layout.location.y as f32 + layout.size.height as f32,
                ).unwrap(),
                &paint,
                tiny_skia::Transform::identity(),
                None,
            );
        }
    }
    

    pub struct Button {
        label: String,
        width: u32,
        height: u32,
        color: tiny_skia::Color,
    }
    
    impl Button {
        pub fn new(label: String, width: u32, height: u32, color: tiny_skia::Color) -> Self {
            Self {
                label,
                width,
                height,
                color,
            }
        }
    
        pub fn render(&self, pixmap: &mut tiny_skia::Pixmap, layout: &stretch::result::Layout, parent_width: f32, parent_height: f32) {
            let mut paint = tiny_skia::Paint::default();
            paint.set_color(self.color);
    
            // Render the button based on the layout from Stretch
            pixmap.fill_rect(
                tiny_skia::Rect::from_ltrb(
                    layout.location.x as f32,
                    layout.location.y as f32,
                    layout.location.x as f32 + layout.size.width as f32,
                    layout.location.y as f32 + layout.size.height as f32,
                ).unwrap(),
                &paint,
                tiny_skia::Transform::identity(),
                None,
            );
    
            // Optionally, render the label text (requires a text rendering system)
        }
    }
    
}
