#![cfg(target_os="android")]
#![allow(non_snake_case)]

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


