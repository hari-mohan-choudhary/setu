package com.example.myapplication

import android.content.Context
import android.graphics.Bitmap
import android.graphics.Canvas
import android.view.SurfaceHolder
import android.view.SurfaceView
import java.nio.ByteBuffer

class SkiaSurfaceView(context: Context) : SurfaceView(context), Runnable, SurfaceHolder.Callback {
    private var thread: Thread? = null
    private var isRunning = false
    private val surfaceHolder: SurfaceHolder = holder

    init {
        // Register the SurfaceHolder callback to get lifecycle events
        surfaceHolder.addCallback(this)
        setWillNotDraw(false) // Allow drawing on this SurfaceView
    }

    override fun surfaceCreated(holder: SurfaceHolder) {
        isRunning = true
        thread = Thread(this)
        thread?.start()
    }

    override fun surfaceChanged(holder: SurfaceHolder, format: Int, width: Int, height: Int) {
        // Handle surface changes if necessary
    }

    override fun surfaceDestroyed(holder: SurfaceHolder) {
        isRunning = false
        try {
            thread?.join() // Wait for the thread to finish
        } catch (e: InterruptedException) {
            e.printStackTrace()
        }
        thread = null
    }

    override fun run() {
        while (isRunning) {
            drawSkia()
        }
    }

    private fun drawSkia() {
        val canvas: Canvas? = surfaceHolder.lockCanvas()
        if (canvas != null) {
            try {
                // Clear the canvas with a background color
                canvas.drawColor(0xFFFFFFFF.toInt()) // White background
val width = resources.displayMetrics.widthPixels
        val height = resources.displayMetrics.heightPixels

                // Call your Rust function to render with tiny-skia and get pixel data
                val pixels = MainActivity.renderSkia(width, height)

                // Here, you would use the pixels to draw to the canvas
                // Example: create a bitmap from pixel data (adjust as needed)
//                val width = 300 // Set your width
//                val height = 300 // Set your height
                val bitmap = Bitmap.createBitmap(width, height, Bitmap.Config.ARGB_8888)
                bitmap.copyPixelsFromBuffer(ByteBuffer.wrap(pixels))

                // Draw the bitmap on the canvas
                canvas.drawBitmap(bitmap, 0f, 0f, null) // Adjust position as needed

            } finally {
                surfaceHolder.unlockCanvasAndPost(canvas)
            }
        }
    }
}
