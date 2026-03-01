package com.spruce.app

import android.app.Activity
import android.os.Bundle
import android.view.Surface
import android.view.SurfaceHolder
import android.view.SurfaceView
import android.view.WindowManager
import android.util.Log
import androidx.appcompat.app.AppCompatActivity

/**
 * Main Activity for Spruce Platform Android Application
 * 
 * This activity creates a native surface and initializes the Rust-based
 * UI renderer for Vue 3.6 Vapor Mode applications.
 */
class MainActivity : AppCompatActivity(), SurfaceHolder.Callback {
    companion object {
        private const val TAG = "SpruceMainActivity"
        
        // Load the native Rust library
        init {
            try {
                System.loadLibrary("spruce_core")
                Log.d(TAG, "✅ Native library loaded successfully")
            } catch (e: UnsatisfiedLinkError) {
                Log.e(TAG, "❌ Failed to load native library: ${e.message}")
            }
        }
    }
    
    private lateinit var surfaceView: SurfaceView
    private var nativeAppHandle: Long = 0
    
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        
        Log.d(TAG, "🚀 Spruce MainActivity onCreate")
        
        // Keep screen on for better performance
        window.addFlags(WindowManager.LayoutParams.FLAG_KEEP_SCREEN_ON)
        
        // Initialize native Rust application
        if (!initializeNativeApp()) {
            Log.e(TAG, "❌ Failed to initialize native app")
            finish()
            return
        }
        
        // Create surface view for native rendering
        surfaceView = SurfaceView(this)
        surfaceView.holder.addCallback(this)
        setContentView(surfaceView)
        
        // Trigger native lifecycle event
        nativeOnCreate()
    }
    
    override fun onStart() {
        super.onStart()
        Log.d(TAG, "▶️ Spruce MainActivity onStart")
        nativeOnStart()
    }
    
    override fun onResume() {
        super.onResume()
        Log.d(TAG, "⏯️ Spruce MainActivity onResume")
        nativeOnResume()
    }
    
    override fun onPause() {
        super.onPause()
        Log.d(TAG, "⏸️ Spruce MainActivity onPause")
        nativeOnPause()
    }
    
    override fun onStop() {
        super.onStop()
        Log.d(TAG, "⏹️ Spruce MainActivity onStop")
        nativeOnStop()
    }
    
    override fun onDestroy() {
        super.onDestroy()
        Log.d(TAG, "💥 Spruce MainActivity onDestroy")
        nativeOnDestroy()
        cleanupNativeApp()
    }
    
    // Surface callback methods
    override fun surfaceCreated(holder: SurfaceHolder) {
        Log.d(TAG, "🎨 Surface created")
    }
    
    override fun surfaceChanged(holder: SurfaceHolder, format: Int, width: Int, height: Int) {
        Log.d(TAG, "🔄 Surface changed: ${width}x${height}, format: $format")
        
        val surface = holder.surface
        if (surface.isValid) {
            nativeInitSurface(surface, width, height)
        }
    }
    
    override fun surfaceDestroyed(holder: SurfaceHolder) {
        Log.d(TAG, "🧹 Surface destroyed")
        nativeDestroySurface()
    }
    
    // Native method declarations
    private external fun initializeNativeApp(): Boolean
    private external fun cleanupNativeApp()
    
    // Lifecycle native methods
    private external fun nativeOnCreate()
    private external fun nativeOnStart()
    private external fun nativeOnResume() 
    private external fun nativeOnPause()
    private external fun nativeOnStop()
    private external fun nativeOnDestroy()
    
    // Surface native methods
    private external fun nativeInitSurface(surface: Surface, width: Int, height: Int)
    private external fun nativeDestroySurface()
    
    // Utility method for getting device info from native side
    fun getDeviceInfo(): String {
        return nativeGetDeviceInfo()
    }
    
    private external fun nativeGetDeviceInfo(): String
}