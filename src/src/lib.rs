
mod dsl;

use android_logger::FilterBuilder;
use stretch::style::*;
use tiny_skia::*;
use jni::JNIEnv;
use jni::objects::JClass;
use jni::sys::jbyteArray;
use log::warn;
use log::LevelFilter;
use android_logger::Config;

use dsl::UIComponent::{Button, View};


// Import View and Button from your DSL module


#[no_mangle]
pub extern "system" fn Java_com_example_myapplication_MainActivity_renderSkia<'local>(
    env: JNIEnv<'local>,
    class: JClass<'local>,
    screen_width: i32,  // Parameters for screen dimensions
    screen_height: i32,
) -> jbyteArray {

    
    // Initialize logger
    // android_logger::init_once(
    //     Config::default()
    //         .with_max_level(LevelFilter::Trace)
    //         .with_tag("mytag")
    //         .with_filter(FilterBuilder::new().parse("debug,hello::crate=trace").build()),
    // );
    

    // Create a blank pixmap to draw on
    let mut pixmap = Pixmap::new(screen_width as u32, screen_height as u32).unwrap();

    // Call the render_ui function to render the UI components
    render_ui(&mut pixmap, screen_width, screen_height);

    // Get the raw pixel data from the pixmap
    let pixels = pixmap.data();

    // Create a new Java byte array to pass pixel data back to Android
    let byte_array = env.byte_array_from_slice(pixels).unwrap();

    // Return the byte array to Java
    **byte_array
}

fn render_ui(pixmap: &mut Pixmap, screen_width: i32, screen_height: i32) {
    let mut stretch = stretch::node::Stretch::new();

    // Create a View node
    let view_node = stretch.new_node(
        Style {
            size: stretch::geometry::Size {
                width: Dimension::Points(screen_width as f32),  // View's width
                height: Dimension::Points(screen_height as f32), // View's height
            },
            ..Default::default()
        },
        vec![], // No children for the View yet
    ).unwrap();

    // Create the first Button node
    let button1_node = stretch.new_node(
        Style {
            size: stretch::geometry::Size {
                width: Dimension::Points(150.0),   // Button1's width
                height: Dimension::Points(50.0),   // Button1's height
            },
            ..Default::default()
        },
        vec![], // No children for Button1
    ).unwrap();

    // Create the second Button node
    let button2_node = stretch.new_node(
        Style {
            size: stretch::geometry::Size {
                width: Dimension::Points(300.0),   // Button2's width
                height: Dimension::Points(300.0),   // Button2's height
            },
            ..Default::default()
        },
        vec![], // No children for Button2
    ).unwrap();

    // You could add these buttons as children of the View if needed
    let container_node = stretch.new_node(
        Style {
            size: stretch::geometry::Size {
                width: Dimension::Points(screen_width as f32),  // Container width
                height: Dimension::Points(screen_height as f32), // Container height
            },
            justify_content: JustifyContent::SpaceBetween, // Layout the buttons with space between them
            ..Default::default()
        },
        vec![button1_node, button2_node], // Add buttons as children
    ).unwrap();

    // Compute the layout
    stretch.compute_layout(container_node, stretch::geometry::Size::undefined()).unwrap();

    // Get layouts for the View and Buttons
    let container_layout = stretch.layout(container_node).unwrap();
    let button1_layout = stretch.layout(button1_node).unwrap();
    let button2_layout = stretch.layout(button2_node).unwrap();

    // Render the View (optional, if needed)
    let view = View::new(screen_width as u32, screen_height as u32, Color::from_rgba8(100, 100, 255, 255));
    view.render(pixmap, container_layout, screen_width as f32, screen_height as f32);

    // Render the first Button
    let button1 = Button::new("Button 1".to_string(), 150, 50, Color::from_rgba8(0, 150, 0, 255));
    button1.render(pixmap, button1_layout, screen_width as f32, screen_height as f32);

    // Render the second Button
    let button2 = Button::new("Button 2".to_string(), 150, 50, Color::from_rgba8(255, 0, 0, 255));
    button2.render(pixmap, button2_layout, screen_width as f32, screen_height as f32);
}




// fn render_ui(pixmap: &mut Pixmap, screen_width: i32, screen_height: i32) {
//     // Create and render a View (container)
//     let view = View::new(screen_width as u32, screen_height as u32, Color::from_rgba8(100, 100, 255, 255));
//     view.render(pixmap);

//     // Create and render a Button
//     let button = Button::new("Click Me".to_string(), 150, 50, Color::from_rgba8(0, 150, 0, 255));
//     button.render(pixmap);

//     // You can add more UI components or modify their position, size, etc.
// }

// mod dsl;

// use taffy::prelude::*;  // Make sure this is the correct import
// use tiny_skia::*;
// use jni::JNIEnv;
// use jni::objects::JClass;
// use jni::sys::jbyteArray;
// use log::{warn, LevelFilter};
// use android_logger::Config;

// // use dsl::UIComponent::{View, Button};

// #[no_mangle]
// pub extern "system" fn Java_com_example_myapplication_MainActivity_renderSkia<'local>(
//         env: JNIEnv<'local>,
//     class: JClass<'local>,
//     screen_width: i32,
//     screen_height: i32,
// ) -> jbyteArray {
//     let mut pixmap = Pixmap::new(screen_width as u32, screen_height as u32).unwrap();
//     render_ui(&mut pixmap);
//     let pixels = pixmap.data();
//     env.byte_array_from_slice(pixels).unwrap()
// }

// fn render_ui(pixmap: &mut Pixmap) {
//     // Initialize Taffy layout engine
//     let mut taffy = Taffy::new();

//     // Create a root layout node (a container)
//     let root_node = taffy
//         .new_leaf(Style {
//             size: Size {
//                 width: Dimension::Points(400.0),
//                 height: Dimension::Points(800.0),
//             },
//             ..Default::default()
//         })
//         .unwrap();

//     // Add layout for your button
//     let button_node = taffy
//         .new_leaf(Style {
//             size: Size {
//                 width: Dimension::Points(150.0),
//                 height: Dimension::Points(50.0),
//             },
//             ..Default::default()
//         })
//         .unwrap();

//     // Perform layout calculations
//     taffy
//         .compute_layout(root_node, Size::UNDEFINED)
//         .unwrap();

//     // Render the components using tiny_skia
//     let view = View::new(400, 800, Color::from_rgba8(100, 100, 255, 255));
//     view.render(pixmap);

//     let button = Button::new("Click Me".to_string(), 150, 50, Color::from_rgba8(0, 150, 0, 255));
//     button.render(pixmap);
// }


// mod dsl;

// use android_logger::FilterBuilder;
// use tiny_skia::*;
// use jni::JNIEnv;
// use jni::objects::JClass;
// use jni::sys::jbyteArray;
// use log::warn;
// use log::LevelFilter;
// use android_logger::Config;
// use taffy::prelude::*;

// // Import View and Button from your DSL module
// use dsl::UIComponent::{View, Button};

// #[no_mangle]
// pub extern "system" fn Java_com_example_myapplication_MainActivity_renderSkia<'local>(
//     env: JNIEnv<'local>,
//     class: JClass<'local>,
//     screen_width: i32,  // Parameters for screen dimensions
//     screen_height: i32,
// ) -> jbyteArray {

    
//     // Initialize logger
//     // android_logger::init_once(
//     //     Config::default()
//     //         .with_max_level(LevelFilter::Trace)
//     //         .with_tag("mytag")
//     //         .with_filter(FilterBuilder::new().parse("debug,hello::crate=trace").build()),
//     // );
    
//     // // Warn log for debug
//     // warn!("Rendering started, screen width: {}", screen_width);

//     // Create a blank pixmap to draw on
//     let mut pixmap = Pixmap::new(screen_width as u32, screen_height as u32).unwrap();

//     // Call the render_ui function to render the UI components
//     render_ui(&mut pixmap, screen_width, screen_height);

//     // Get the raw pixel data from the pixmap
//     let pixels = pixmap.data();

//     // Create a new Java byte array to pass pixel data back to Android
//     let byte_array = env.byte_array_from_slice(pixels).unwrap();

//     // Return the byte array to Java
//     **byte_array
// }

// fn render_ui(pixmap: &mut Pixmap, screen_width: i32, screen_height: i32) {
//     // Create and render a View (container)
//     let view = View::new(screen_width as u32, screen_height as u32, Color::from_rgba8(100, 100, 255, 255));
//     view.render(pixmap);

//     // Create and render a Button
//     let button = Button::new("Click Me".to_string(), 150, 50, Color::from_rgba8(0, 150, 0, 255));
//     button.render(pixmap);

//     // You can add more UI components or modify their position, size, etc.
// }




// use android_logger::FilterBuilder;
// use tiny_skia::*;
// use jni::JNIEnv;
// use jni::objects::JClass;
// use jni::sys::jbyteArray;
// use log::warn;
// use log::LevelFilter;
// use android_logger::Config;


// #[no_mangle]
// pub extern "system" fn Java_com_example_myapplication_MainActivity_renderSkia<'local>(
//     env: JNIEnv<'local>,
//     class: JClass<'local>,
//     screen_width: i32,  // Add parameters for screen dimensions
//     screen_height: i32,
// ) -> jbyteArray {
//     // Initialize Android logger
//     android_logger::init_once(
//         Config::default()
//             .with_max_level(LevelFilter::Trace)
//             .with_tag("mytag")
//             .with_filter(FilterBuilder::new().parse("debug,hello::crate=trace").build()),
//     );
    
//     warn!("Screen width: {}, Screen height: {}", screen_width, screen_height);

//     let width = screen_width;
//     let height = screen_height;

//     // Create a blank pixmap to draw on
//     let mut pixmap = Pixmap::new(width as u32, height as u32).unwrap();

//     // Paint and PathBuilder for shapes
//     let mut paint = Paint::default();

//     // Set color for the background
//     paint.set_color(Color::from_rgba8(255, 255, 255, 255));  // White background
//     let mut path_builder = PathBuilder::new();
//     path_builder.push_rect(Rect::from_ltrb(0.0, 0.0, width as f32, height as f32).unwrap());
//     let path = path_builder.finish().unwrap();
//     pixmap.fill_path(&path, &paint, tiny_skia::FillRule::Winding, Transform::identity(), None);

//     // 1. Draw a circle
//     paint.set_color(Color::from_rgba8(100, 100, 255, 255));  // Blue circle
//     let circle_center = Point::from_xy(width as f32 / 4.0, height as f32 / 4.0);
//     let circle_radius = 100.0;
//     // pixmap.fill_circle(circle_center.x, circle_center.y, circle_radius, &paint, tiny_skia::FillRule::Winding);

//     // 2. Draw a square
//     paint.set_color(Color::from_rgba8(255, 0, 0, 255));  // Red square
//     let square_size = 150.0;
//     path_builder = PathBuilder::new();
//     path_builder.push_rect(Rect::from_ltrb(width as f32 / 2.0 - square_size / 2.0, 
//                                            height as f32 / 2.0 - square_size / 2.0,
//                                            width as f32 / 2.0 + square_size / 2.0, 
//                                            height as f32 / 2.0 + square_size / 2.0).unwrap());
//     let square_path = path_builder.finish().unwrap();
//     pixmap.fill_path(&square_path, &paint, tiny_skia::FillRule::Winding, Transform::identity(), None);

//     // 3. Draw a rectangle
//     paint.set_color(Color::from_rgba8(0, 255, 0, 255));  // Green rectangle
//     path_builder = PathBuilder::new();
//     path_builder.push_rect(Rect::from_ltrb(50.0, height as f32 - 150.0, width as f32 - 50.0, height as f32 - 50.0).unwrap());
//     let rect_path = path_builder.finish().unwrap();
//     pixmap.fill_path(&rect_path, &paint, tiny_skia::FillRule::Winding, Transform::identity(), None);

//     // Get the raw pixel data from the pixmap
//     let pixels = pixmap.data();

//     // Create a new Java byte array to pass pixel data back to Android
//     let byte_array = env.byte_array_from_slice(pixels).unwrap();

//     // Return the byte array to Java
//     **byte_array
// }


// use android_logger::FilterBuilder;
// use tiny_skia::*;
// use jni::JNIEnv;
// use jni::objects::JClass;
// use jni::sys::jbyteArray;
// use log::warn;
// use log::LevelFilter;
// use android_logger::Config;


// #[no_mangle]
// pub extern "system" fn Java_com_example_myapplication_MainActivity_renderSkia<'local>(
//     env: JNIEnv<'local>,
//     class: JClass<'local>,
//     screen_width: i32,  // Add parameters for screen dimensions
//     screen_height: i32,
// ) -> jbyteArray {
//     android_logger::init_once(
//         Config::default()
//             .with_max_level(LevelFilter::Trace)
//             .with_tag("mytag")
//             .with_filter(FilterBuilder::new().parse("debug,hello::crate=trace").build()),
//     );
//     // android_log::init("MyApp").unwrap();
//     warn!("Nothing more to say {:?}", screen_width);

//     let width = screen_width;
//     let height = screen_height;

//     // Create a blank pixmap to draw on
//     let mut pixmap = Pixmap::new(width as u32, height as u32).unwrap();

//     // Example: Draw a simple circle
//     let mut paint = Paint::default();
//     paint.set_color(Color::from_rgba8(100, 100, 0, 255));

//     let mut path_builder = PathBuilder::new();
//     path_builder.push_rect(Rect::from_ltrb(0.0, 0.0, width as f32, height as f32).unwrap());
//     let path = path_builder.finish().unwrap();

//     pixmap.fill_path(
//         &path,
//         &paint,
//         tiny_skia::FillRule::Winding,
//         Transform::identity(),
//         None,
//     );

//     // Get the raw pixel data from the pixmap
//     let pixels = pixmap.data();

//     // Create a new Java byte array to pass pixel data back to Android
//     let byte_array = env.byte_array_from_slice(pixels).unwrap();

//     // Return the byte array to Java
//     **byte_array
// }

