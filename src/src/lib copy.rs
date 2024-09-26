

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



// use tiny_skia::{Pixmap, Paint, PathBuilder, Transform, Color};
// use jni::JNIEnv;
// use jni::objects::JClass;
// use jni::sys::jbyteArray;

// #[no_mangle]
// pub extern "system" fn Java_com_example_myapplication_MainActivity_renderSkia<'local>(
//     env: JNIEnv<'local>,
//     class: JClass<'local>,
// ) -> jbyteArray {
//     let width = 300;
//     let height = 300;

//     // Create a blank pixmap to draw on
//     let mut pixmap = Pixmap::new(width, height).unwrap();

//     // Example: Draw a simple circle
//     let mut paint = Paint::default();
//     paint.set_color(Color::from_rgba8(0, 255, 255, 255));

//     let mut path_builder = PathBuilder::new();
//     path_builder.push_circle(150.0, 150.0, 100.0);
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


// // // This is the interface to the JVM that we'll call the majority of our
// // // methods on.
// // use jni::JNIEnv;

// // // These objects are what you should use as arguments to your native
// // // function. They carry extra lifetime information to prevent them escaping
// // // this context and getting used after being GC'd.
// // use jni::objects::{JClass, JString};

// // // This is just a pointer. We'll be returning it from our function. We
// // // can't return one of the objects with lifetime information because the
// // // lifetime checker won't let us.
// // use jni::sys::jstring;

// // // This keeps Rust from "mangling" the name and making it unique for this
// // // crate.
// // #[no_mangle]
// // pub extern "system" fn Java_com_example_myapplication_MainActivity_hello<'local>(mut env: JNIEnv<'local>,
// // // This is the class that owns our static method. It's not going to be used,
// // // but still must be present to match the expected signature of a static
// // // native method.
// //                                                      class: JClass<'local>,
// //                                                      input: JString<'local>)
// //                                                      -> jstring {
// //     // First, we have to get the string out of Java. Check out the `strings`
// //     // module for more info on how this works.
// //     let input: String =
// //         env.get_string(&input).expect("Couldn't get java string!").into();

// //     // Then we have to create a new Java string to return. Again, more info
// //     // in the `strings` module.
// //     let output = env.new_string(format!("Hari Hello, {}!", input))
// //         .expect("Couldn't create java string!");

// //     // Finally, extract the raw pointer to return.
// //     output.into_raw()
// // }



// // use tiny_skia::{Pixmap, Paint, PathBuilder, FillRule, Transform};


// // #[no_mangle]
// // pub extern "C" fn Java_com_example_myapplication_MainActivity_renderImage() -> *const u8 {
// //     let mut pixmap = Pixmap::new(500, 500).unwrap();
// //     let mut paint = Paint::default();
// //     paint.set_color_rgba8(255, 0, 0, 255); // Red

// //     let mut pb = PathBuilder::new();
// //     pb.push_circle(250.0, 250.0, 200.0); // Circle in the center
// //     let path = pb.finish().unwrap();

// //     pixmap.fill_path(&path, &paint, FillRule::Winding, Transform::identity(), None);

// //     pixmap.data().as_ptr() // Return pointer to the image buffer
// // }


// // use tiny_skia::{Pixmap, Paint, PathBuilder, FillRule, Transform};

// // #[no_mangle]
// // pub extern "C" fn Java_com_example_myapplication_MainActivity_renderImage(ptr_len: *mut usize) -> *const u8 {
// //     let mut pixmap = Pixmap::new(500, 500).unwrap();

// //     let mut paint = Paint::default();
// //     paint.set_color_rgba8(255, 0, 0, 255);  // Red

// //     let mut pb = PathBuilder::new();
// //     pb.push_circle(250.0, 250.0, 200.0);  // Circle in the center
// //     let path = pb.finish().unwrap();

// //     pixmap.fill_path(&path, &paint, FillRule::Winding, Transform::identity(), None);

// //     let data = pixmap.data();
    
// //     unsafe {
// //         *ptr_len = data.len();  // Set the length in the output parameter
// //     }

// //     data.as_ptr()  // Return pointer to the image buffer
// // }



// // #[no_mangle]
// // pub extern "C" fn render_image() -> *const u8 {
// //     let mut pixmap = Pixmap::new(500, 500).unwrap();

// //     let mut paint = Paint::default();
// //     paint.set_color_rgba8(255, 0, 0, 255);  // Red

// //     let mut pb = PathBuilder::new();
// //     pb.push_circle(250.0, 250.0, 200.0);  // Circle in the center
// //     let path = pb.finish().unwrap();

// //     pixmap.fill_path(&path, &paint, FillRule::Winding, Transform::identity(), None);

// //     pixmap.data().as_ptr()  // Return pointer to the image buffer
// // }

// // pub fn add(left: usize, right: usize) -> usize {
// //     left + right
// // }

// // #[cfg(test)]
// // mod tests {
// //     use super::*;

// //     #[test]
// //     fn it_works() {
// //         let result = add(2, 2);
// //         assert_eq!(result, 4);
// //     }
// // }
