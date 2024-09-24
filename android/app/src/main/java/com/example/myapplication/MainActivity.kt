package com.example.myapplication

import android.os.Bundle
import androidx.activity.ComponentActivity
import androidx.activity.compose.setContent
import androidx.compose.ui.viewinterop.AndroidView


class MainActivity : ComponentActivity() {

    // Load the native Rust library
    init {
        System.loadLibrary("tiny_skia_android")
    }

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        // Set the content to your custom SurfaceView
        setContent {
            AndroidView(factory = { context ->
                SkiaSurfaceView(context)
            })
        }
    }

    companion object {
        // Declare the external Rust function for rendering with tiny-skia
        @JvmStatic
        external fun renderSkia(screenWidth: Int, screenHeight: Int): ByteArray
    }
}

//package com.example.myapplication
//
//import android.graphics.Bitmap
//import android.graphics.BitmapFactory
//import android.os.Bundle
//import androidx.activity.ComponentActivity
//import androidx.activity.compose.setContent
//import androidx.compose.foundation.Image
//import androidx.compose.foundation.layout.fillMaxSize
//import androidx.compose.runtime.Composable
//import androidx.compose.runtime.LaunchedEffect
//import androidx.compose.runtime.mutableStateOf
//import androidx.compose.runtime.remember
//import androidx.compose.ui.Modifier
//import androidx.compose.ui.graphics.asImageBitmap
//import androidx.compose.material3.Text
//import androidx.compose.ui.text.TextStyle
//import androidx.compose.ui.text.font.FontWeight
//import androidx.compose.ui.unit.sp
//import kotlinx.coroutines.Dispatchers
//import kotlinx.coroutines.withContext
//import java.nio.ByteBuffer
//
//class MainActivity : ComponentActivity() {
//
//    // Load the native Rust library (if needed for other functions)
//    init {
//        System.loadLibrary("tiny_skia_android")
//    }
//
//    override fun onCreate(savedInstanceState: Bundle?) {
//        super.onCreate(savedInstanceState)
//
//        // No hello function needed
//        setContent {
//            // Display the image rendered by tiny-skia
//            SkiaImage()
//        }
//    }
//
//    companion object {
//        // Declare the external Rust function for rendering with tiny-skia
//        @JvmStatic
//        external fun renderSkia(): ByteArray
//    }
//}
//
//@Composable
//fun SkiaImage() {
//    val bitmapState = remember { mutableStateOf<Bitmap?>(null) }
//
//    // Load the bitmap asynchronously
//    LaunchedEffect(Unit) {
//        withContext(Dispatchers.IO) {
//            // Call the native Rust function to get pixel data directly
//            val pixels = MainActivity.renderSkia() // Call the method directly
//
//            // Convert byte array to Bitmap
//            val width = 300
//            val height = 300
//            val bmp = Bitmap.createBitmap(width, height, Bitmap.Config.ARGB_8888)
//            bmp.copyPixelsFromBuffer(ByteBuffer.wrap(pixels))
//
//            // Update bitmap state on the main thread
//            withContext(Dispatchers.Main) {
//                bitmapState.value = bmp
//            }
//        }
//    }
//
//    // Display the bitmap if it's available
//    bitmapState.value?.let { bmp ->
//        Image(
//            bitmap = bmp.asImageBitmap(),
//            contentDescription = null,
//            modifier = Modifier.fillMaxSize()
//        )
//    } ?: Text(
//        text = "Loading...",
//        style = TextStyle(fontSize = 24.sp, fontWeight = FontWeight.Bold),
//    )
//}






//package com.example.myapplication
//
//import android.graphics.Bitmap
//import android.graphics.BitmapFactory
//import android.os.Bundle
//import androidx.activity.ComponentActivity
//import androidx.activity.compose.setContent
//import androidx.compose.foundation.Image
//import androidx.compose.foundation.layout.fillMaxSize
//import androidx.compose.runtime.Composable
//import androidx.compose.ui.Modifier
//import androidx.compose.ui.graphics.asImageBitmap
//import androidx.compose.material3.Text
//import androidx.compose.ui.text.TextStyle
//import androidx.compose.ui.text.font.FontWeight
//import androidx.compose.ui.unit.dp
//import androidx.compose.ui.unit.sp
//
//class MainActivity : ComponentActivity() {
//
//    // Declare the external Rust function for the greeting message
//    external fun hello(name: String): String
//
//
//    init {
//        // Load the native Rust library (if needed for other functions)
//        System.loadLibrary("tiny_skia_android")
//    }
//
//    override fun onCreate(savedInstanceState: Bundle?) {
//        super.onCreate(savedInstanceState)
//        val greetingMessage  = hello("Josh")
//        setContent {
//            // Display a greeting message
//            GreetingMessage(message = greetingMessage)
//        }
//    }
//}
//
//@Composable
//fun GreetingMessage(message: String) {
//    Text(
//        text = message,
//        style = TextStyle(fontSize = 24.sp, fontWeight = FontWeight.Bold),
////        modifier = Modifier.padding(16.dp)
//    )
//}
