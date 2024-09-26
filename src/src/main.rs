use taffy::prelude::*;
use tiny_skia::{Pixmap, Paint, Transform, Rect as SkiaRect};
use minifb::{Key, Window, WindowOptions};

fn main() -> Result<(), taffy::TaffyError> {
    // Step 1: Create a flexible layout with TaffyTree
    let mut taffy: TaffyTree<()> = TaffyTree::new();

    // Create three children with different flex-grow properties
    let child1 = taffy.new_leaf(Style {
        flex_grow: 1.0,
        ..Default::default()
    })?;
    
    let child2 = taffy.new_leaf(Style {
        flex_grow: 1.0, // This child takes twice the space of child1
        ..Default::default()
    })?;
    
    let child3 = taffy.new_leaf(Style {
        flex_grow: 1.0,
        ..Default::default()
    })?;

    // Parent node which arranges children horizontally (like a flexbox)
    let parent = taffy.new_with_children(
        Style {
            size: Size { width: Dimension::Length(400.0), height: Dimension::Length(200.0) }, // Fixed size parent
            flex_direction: FlexDirection::Column, // Pass FlexDirection directly
            ..Default::default()
        },
        &[child1, child2, child3],
    )?;

    // Compute the layout of all nodes
    taffy.compute_layout(
        parent,
        Size { width: AvailableSpace::Definite(400.0), height: AvailableSpace::Definite(200.0) },
    )?;

    // Get the layout for each node
    let parent_layout = taffy.layout(parent)?;
    let child1_layout = taffy.layout(child1)?;
    let child2_layout = taffy.layout(child2)?;
    let child3_layout = taffy.layout(child3)?;

    // Step 2: Use tiny-skia to render the layout
    let mut pixmap = Pixmap::new(400, 200).unwrap(); // Create a Pixmap of 400x200 pixels

    // Paint for the children
    let mut paint1 = Paint::default();
    paint1.set_color_rgba8(50, 127, 150, 200);

    let mut paint2 = Paint::default();
    paint2.set_color_rgba8(220, 140, 75, 180);

    let mut paint3 = Paint::default();
    paint3.set_color_rgba8(100, 200, 150, 200);

    // Draw the children using computed layouts
    let child1_rect = SkiaRect::from_xywh(
        child1_layout.location.x as f32,
        child1_layout.location.y as f32,
        child1_layout.size.width as f32,
        child1_layout.size.height as f32,
    ).unwrap();
    pixmap.fill_rect(child1_rect, &paint1, Transform::identity(), None);

    let child2_rect = SkiaRect::from_xywh(
        child2_layout.location.x as f32,
        child2_layout.location.y as f32,
        child2_layout.size.width as f32,
        child2_layout.size.height as f32,
    ).unwrap();
    pixmap.fill_rect(child2_rect, &paint2, Transform::identity(), None);

    let child3_rect = SkiaRect::from_xywh(
        child3_layout.location.x as f32,
        child3_layout.location.y as f32,
        child3_layout.size.width as f32,
        child3_layout.size.height as f32,
    ).unwrap();
    pixmap.fill_rect(child3_rect, &paint3, Transform::identity(), None);

    let buffer: Vec<u32> = pixmap.data().chunks(4).map(|rgba| {
        (rgba[0] as u32) << 16 | (rgba[1] as u32) << 8 | (rgba[2] as u32) // Convert to RGB
    }).collect();

    // Create a window to display the image
    let mut window = Window::new("tiny-skia example", 400, 200, WindowOptions::default())
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });

    // Display the image in the window
    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer, 400, 200).unwrap();
    }

    // Save the image as a PNG file (uncomment if you want to save)
    // pixmap.save_png("flex_layout.png").unwrap();

    Ok(())
}



// use taffy::prelude::*;
// use tiny_skia::{Pixmap, Paint, Transform, Rect as SkiaRect}; // Alias tiny-skia's Rect

// fn main() -> Result<(), taffy::TaffyError> {
//     // Step 1: Taffy layout
//     let mut taffy: TaffyTree<()> = TaffyTree::new();

//     let child = taffy.new_leaf(Style {
//         size: Size { width: Dimension::Percent(0.5), height: Dimension::Auto },
//         ..Default::default()
//     })?;

//     let node = taffy.new_with_children(
//         Style {
//             size: Size { width: Dimension::Length(100.0), height: Dimension::Length(100.0) },
//             justify_content: Some(JustifyContent::Center),
//             ..Default::default()
//         },
//         &[child],
//     )?;

//     taffy.compute_layout(
//         node,
//         Size { height: AvailableSpace::Definite(100.0), width: AvailableSpace::Definite(100.0) },
//     )?;
    
//     let node_layout = taffy.layout(node)?;
//     let child_layout = taffy.layout(child)?;

//     // Step 2: tiny-skia rendering
//     let mut pixmap = Pixmap::new(200, 200).unwrap(); // Create a Pixmap of 200x200 pixels

//     // Paint for the parent node
//     let mut parent_paint = Paint::default();
//     parent_paint.set_color_rgba8(50, 127, 150, 200);

//     // Paint for the child node
//     let mut child_paint = Paint::default();
//     child_paint.set_color_rgba8(220, 140, 75, 180);

//     // Draw the parent node as a rectangle
//     let parent_rect = SkiaRect::from_xywh(
//         node_layout.location.x as f32,
//         node_layout.location.y as f32,
//         node_layout.size.width as f32,
//         node_layout.size.height as f32,
//     ).unwrap();
//     pixmap.fill_rect(parent_rect, &parent_paint, Transform::identity(), None);

//     // Draw the child node as a smaller rectangle inside the parent
//     let child_rect = SkiaRect::from_xywh(
//         child_layout.location.x as f32,
//         child_layout.location.y as f32,
//         child_layout.size.width as f32,
//         child_layout.size.height as f32,
//     ).unwrap();
//     pixmap.fill_rect(child_rect, &child_paint, Transform::identity(), None);

//     // Save the image as a PNG file
//     pixmap.save_png("layout_image.png").unwrap();

//     Ok(())
// }
