use tiny_skia::{Pixmap, Paint, PathBuilder, Transform, Color};
use jni::JNIEnv;
use jni::objects::JClass;
use jni::sys::jbyteArray;

#[no_mangle]
pub extern "system" fn Java_com_example_myapplication_MainActivity_renderSkia<'local>(
    env: JNIEnv<'local>,
    class: JClass<'local>,
) -> jbyteArray {
    let width = 300;
    let height = 300;

    // Create a blank pixmap to draw on
    let mut pixmap = Pixmap::new(width, height).unwrap();

    // Example: Draw a simple circle
    let mut paint = Paint::default();
    paint.set_color(Color::from_rgba8(0, 255, 255, 255));

    let mut path_builder = PathBuilder::new();
    path_builder.push_circle(150.0, 150.0, 100.0);
    let path = path_builder.finish().unwrap();

    pixmap.fill_path(
        &path,
        &paint,
        tiny_skia::FillRule::Winding,
        Transform::identity(),
        None,
    );

    // Get the raw pixel data from the pixmap
    let pixels = pixmap.data();

    // Create a new Java byte array to pass pixel data back to Android
    let byte_array = env.byte_array_from_slice(pixels).unwrap();

    // Return the byte array to Java
    **byte_array
}


// // This is the interface to the JVM that we'll call the majority of our
// // methods on.
// use jni::JNIEnv;

// // These objects are what you should use as arguments to your native
// // function. They carry extra lifetime information to prevent them escaping
// // this context and getting used after being GC'd.
// use jni::objects::{JClass, JString};

// // This is just a pointer. We'll be returning it from our function. We
// // can't return one of the objects with lifetime information because the
// // lifetime checker won't let us.
// use jni::sys::jstring;

// // This keeps Rust from "mangling" the name and making it unique for this
// // crate.
// #[no_mangle]
// pub extern "system" fn Java_com_example_myapplication_MainActivity_hello<'local>(mut env: JNIEnv<'local>,
// // This is the class that owns our static method. It's not going to be used,
// // but still must be present to match the expected signature of a static
// // native method.
//                                                      class: JClass<'local>,
//                                                      input: JString<'local>)
//                                                      -> jstring {
//     // First, we have to get the string out of Java. Check out the `strings`
//     // module for more info on how this works.
//     let input: String =
//         env.get_string(&input).expect("Couldn't get java string!").into();

//     // Then we have to create a new Java string to return. Again, more info
//     // in the `strings` module.
//     let output = env.new_string(format!("Hari Hello, {}!", input))
//         .expect("Couldn't create java string!");

//     // Finally, extract the raw pointer to return.
//     output.into_raw()
// }



// use tiny_skia::{Pixmap, Paint, PathBuilder, FillRule, Transform};


// #[no_mangle]
// pub extern "C" fn Java_com_example_myapplication_MainActivity_renderImage() -> *const u8 {
//     let mut pixmap = Pixmap::new(500, 500).unwrap();
//     let mut paint = Paint::default();
//     paint.set_color_rgba8(255, 0, 0, 255); // Red

//     let mut pb = PathBuilder::new();
//     pb.push_circle(250.0, 250.0, 200.0); // Circle in the center
//     let path = pb.finish().unwrap();

//     pixmap.fill_path(&path, &paint, FillRule::Winding, Transform::identity(), None);

//     pixmap.data().as_ptr() // Return pointer to the image buffer
// }


// use tiny_skia::{Pixmap, Paint, PathBuilder, FillRule, Transform};

// #[no_mangle]
// pub extern "C" fn Java_com_example_myapplication_MainActivity_renderImage(ptr_len: *mut usize) -> *const u8 {
//     let mut pixmap = Pixmap::new(500, 500).unwrap();

//     let mut paint = Paint::default();
//     paint.set_color_rgba8(255, 0, 0, 255);  // Red

//     let mut pb = PathBuilder::new();
//     pb.push_circle(250.0, 250.0, 200.0);  // Circle in the center
//     let path = pb.finish().unwrap();

//     pixmap.fill_path(&path, &paint, FillRule::Winding, Transform::identity(), None);

//     let data = pixmap.data();
    
//     unsafe {
//         *ptr_len = data.len();  // Set the length in the output parameter
//     }

//     data.as_ptr()  // Return pointer to the image buffer
// }



// #[no_mangle]
// pub extern "C" fn render_image() -> *const u8 {
//     let mut pixmap = Pixmap::new(500, 500).unwrap();

//     let mut paint = Paint::default();
//     paint.set_color_rgba8(255, 0, 0, 255);  // Red

//     let mut pb = PathBuilder::new();
//     pb.push_circle(250.0, 250.0, 200.0);  // Circle in the center
//     let path = pb.finish().unwrap();

//     pixmap.fill_path(&path, &paint, FillRule::Winding, Transform::identity(), None);

//     pixmap.data().as_ptr()  // Return pointer to the image buffer
// }

// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
