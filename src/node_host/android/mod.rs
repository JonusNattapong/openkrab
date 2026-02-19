//! Android Native Module for node_host
//!
//! This module provides Kotlin-based native functionality for Android devices:
//! - Camera snap (photo capture)
//! - Screen recording
//! - Location services
//! - Notifications
//!
//! Usage from Rust:
//! ```json
//! {
//!   "action": "camera.snap",
//!   "payload": {"node_id": "android-1", "camera": "back", "quality": "high"}
//! }
//! ```

use serde::{Deserialize, Serialize};

/// Android camera configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AndroidCameraConfig {
    /// Camera position: "front" or "back"
    pub camera: String,
    /// Image quality: "low", "medium", "high"
    pub quality: String,
    /// Enable flash
    pub flash: bool,
}

impl Default for AndroidCameraConfig {
    fn default() -> Self {
        Self {
            camera: "back".to_string(),
            quality: "high".to_string(),
            flash: false,
        }
    }
}

/// Android screen recording configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AndroidScreenRecordConfig {
    /// Enable audio recording
    pub audio: bool,
    /// Video quality: "low", "medium", "high"
    pub quality: String,
    /// Recording duration in seconds
    pub duration_secs: u32,
}

impl Default for AndroidScreenRecordConfig {
    fn default() -> Self {
        Self {
            audio: false,
            quality: "high".to_string(),
            duration_secs: 60,
        }
    }
}

/// Android location configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AndroidLocationConfig {
    /// Accuracy level: "high", "balanced", "low"
    pub accuracy: String,
    /// Timeout in milliseconds
    pub timeout_ms: u32,
}

impl Default for AndroidLocationConfig {
    fn default() -> Self {
        Self {
            accuracy: "high".to_string(),
            timeout_ms: 10000,
        }
    }
}

/// Android notification configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AndroidNotificationConfig {
    /// Notification title
    pub title: String,
    /// Notification body
    pub body: String,
    /// Play sound
    pub sound: bool,
    /// Badge number
    pub badge: Option<u32>,
}

impl Default for AndroidNotificationConfig {
    fn default() -> Self {
        Self {
            title: "Notification".to_string(),
            body: String::new(),
            sound: true,
            badge: None,
        }
    }
}

/// Kotlin code generation for Android native handlers
pub mod kotlin {
    /// Generate Kotlin code for camera snap handler
    pub fn camera_handler_kotlin() -> String {
        r#"package com.openkrab.nodehost.camera

import android.Manifest
import android.app.Activity
import android.content.Context
import android.content.pm.PackageManager
import android.graphics.ImageFormat
import android.hardware.camera2.*
import android.media.Image
import android.media.ImageReader
import android.os.Handler
import android.os.HandlerThread
import android.util.Size
import androidx.core.app.ActivityCompat
import androidx.core.content.ContextCompat
import org.json.JSONObject
import java.io.ByteArrayOutputStream

class KrabKrabCameraHandler(private val context: Context) {
    
    private var cameraDevice: CameraDevice? = null
    private var captureSession: CameraCaptureSession? = null
    private var imageReader: ImageReader? = null
    private var backgroundThread: HandlerThread? = null
    private var backgroundHandler: Handler? = null
    
    /**
     * Capture a photo from the device camera
     * @param camera "front" or "back"
     * @param quality "low", "medium", "high"
     * @param flash Enable flash
     * @param callback Callback with JSON result
     */
    fun capturePhoto(
        camera: String,
        quality: String,
        flash: Boolean,
        callback: (Result<JSONObject>) -> Unit
    ) {
        // Note: In production, implement Camera2 API capture
        // This is a placeholder for the native implementation
        callback(Result.failure(Exception("Not implemented - requires native Android implementation")))
    }
    
    private fun getCameraId(facing: String): String {
        val manager = context.getSystemService(Context.CAMERA_SERVICE) as CameraManager
        for (id in manager.cameraIdList) {
            val characteristics = manager.getCameraCharacteristics(id)
            val facingInt = if (facing == "front") {
                CameraCharacteristics.LENS_FACING_FRONT
            } else {
                CameraCharacteristics.LENS_FACING_BACK
            }
            if (characteristics.get(CameraCharacteristics.LENS_FACING) == facingInt) {
                return id
            }
        }
        return manager.cameraIdList[0]
    }
    
    private fun getCaptureSize(quality: String): Size {
        return when (quality) {
            "low" -> Size(640, 480)
            "medium" -> Size(1280, 720)
            "high" -> Size(1920, 1080)
            else -> Size(1920, 1080)
        }
    }
}

class CameraCallback : CameraCaptureSession.CaptureCallback() {
    override fun onCaptureCompleted(
        session: CameraCaptureSession,
        request: CaptureRequest,
        result: TotalCaptureResult
    ) {
        super.onCaptureCompleted(session, request, result)
        // Handle capture completion
    }
}
"#.to_string()
    }

    /// Generate Kotlin code for screen recording handler
    pub fn screen_record_handler_kotlin() -> String {
        r#"package com.openkrab.nodehost.screenrecord

import android.app.Activity
import android.content.Context
import android.content.Intent
import android.media.projection.MediaProjection
import android.media.projection.MediaProjectionManager
import android.os.Handler
import android.os.HandlerThread
import java.io.File

class KrabKrabScreenRecordHandler(private val context: Context) {
    
    private var mediaProjection: MediaProjection? = null
    private var isRecording = false
    private var outputFile: File? = null
    
    /**
     * Start screen recording
     * @param audio Enable audio recording
     * @param quality "low", "medium", "high"
     * @param duration Recording duration in seconds
     * @param callback Callback with JSON result
     */
    fun startRecording(
        audio: Boolean,
        quality: String,
        duration: Int,
        callback: (Result<JSONObject>) -> Unit
    ) {
        if (isRecording) {
            callback(Result.failure(Exception("Already recording")))
            return
        }
        
        // Note: In production, implement MediaProjection screen recording
        // This is a placeholder for the native implementation
        callback(Result.failure(Exception("Not implemented - requires native Android implementation")))
    }
    
    /**
     * Stop screen recording
     * @param callback Callback with file URL
     */
    fun stopRecording(callback: (Result<File>) -> Unit) {
        if (!isRecording) {
            callback(Result.failure(Exception("Not recording")))
            return
        }
        
        isRecording = false
        mediaProjection?.stop()
        
        // Return recorded video file
        outputFile?.let {
            callback(Result.success(it))
        } ?: callback(Result.failure(Exception("No output file")))
    }
    
    private fun getVideoQuality(quality: String): Int {
        return when (quality) {
            "low" -> 500000
            "medium" -> 2000000
            "high" -> 5000000
            else -> 2000000
        }
    }
}
"#.to_string()
    }

    /// Generate Kotlin code for location handler
    pub fn location_handler_kotlin() -> String {
        r#"package com.openkrab.nodehost.location

import android.Manifest
import android.content.Context
import android.content.pm.PackageManager
import android.location.Location
import android.os.Looper
import androidx.core.app.ActivityCompat
import androidx.core.content.ContextCompat
import com.google.android.gms.location.*
import org.json.JSONObject

class KrabKrabLocationHandler(private val context: Context) {
    
    private val fusedLocationClient: FusedLocationProviderClient =
        LocationServices.getFusedLocationProviderClient(context)
    
    private var locationCallback: LocationCallback? = null
    
    /**
     * Get current location
     * @param accuracy "high", "balanced", "low"
     * @param timeout Timeout in milliseconds
     * @param callback Callback with JSON result
     */
    fun getLocation(
        accuracy: String,
        timeout: Long,
        callback: (Result<Location>) -> Unit
    ) {
        if (ContextCompat.checkSelfPermission(
                context,
                Manifest.permission.ACCESS_FINE_LOCATION
            ) != PackageManager.PERMISSION_GRANTED
        ) {
            callback(Result.failure(SecurityException("Location permission denied")))
            return
        }
        
        val priority = when (accuracy) {
            "high" -> Priority.PRIORITY_HIGH_ACCURACY
            "balanced" -> Priority.PRIORITY_BALANCED_POWER_ACCURACY
            "low" -> Priority.PRIORITY_LOW_POWER
            else -> Priority.PRIORITY_HIGH_ACCURACY
        }
        
        val locationRequest = LocationRequest.Builder(timeout)
            .setPriority(priority)
            .build()
        
        locationCallback = object : LocationCallback() {
            override fun onLocationResult(result: LocationResult) {
                result.lastLocation?.let { location ->
                    callback(Result.success(location))
                } ?: callback(Result.failure(Exception("No location available")))
                fusedLocationClient.removeLocationUpdates(this)
            }
        }
        
        try {
            fusedLocationClient.requestLocationUpdates(
                locationRequest,
                locationCallback!!,
                Looper.getMainLooper()
            )
        } catch (e: SecurityException) {
            callback(Result.failure(e))
        }
        
        // Timeout handling would need to be implemented
    }
    
    /**
     * Get last known location (faster but may be stale)
     */
    fun getLastLocation(callback: (Result<Location>) -> Unit) {
        if (ContextCompat.checkSelfPermission(
                context,
                Manifest.permission.ACCESS_FINE_LOCATION
            ) != PackageManager.PERMISSION_GRANTED
        ) {
            callback(Result.failure(SecurityException("Location permission denied")))
            return
        }
        
        try {
            fusedLocationClient.lastLocation
                .addOnSuccessListener { location ->
                    location?.let {
                        callback(Result.success(it))
                    } ?: callback(Result.failure(Exception("No last location")))
                }
                .addOnFailureListener { e ->
                    callback(Result.failure(e))
                }
        } catch (e: SecurityException) {
            callback(Result.failure(e))
        }
    }
    
    fun removeUpdates() {
        locationCallback?.let {
            fusedLocationClient.removeLocationUpdates(it)
        }
    }
}
"#
        .to_string()
    }

    /// Generate Kotlin code for notification handler
    pub fn notification_handler_kotlin() -> String {
        r#"package com.openkrab.nodehost.notification

import android.app.NotificationChannel
import android.app.NotificationManager
import android.app.PendingIntent
import android.content.Context
import android.content.Intent
import android.os.Build
import androidx.core.app.NotificationCompat
import androidx.core.app.NotificationManagerCompat
import org.json.JSONObject

class KrabKrabNotificationHandler(private val context: Context) {
    
    companion object {
        const val CHANNEL_ID = "krabkrab_notifications"
        const val CHANNEL_NAME = "KrabKrab Notifications"
    }
    
    init {
        createNotificationChannel()
    }
    
    /**
     * Send a local notification
     * @param title Notification title
     * @param body Notification body
     * @param sound Play sound
     * @param badge Badge number (optional)
     * @param callback Callback with JSON result
     */
    fun sendNotification(
        title: String,
        body: String,
        sound: Boolean,
        badge: Int?,
        callback: (Result<JSONObject>) -> Unit
    ) {
        try {
            val notificationId = (System.currentTimeMillis() % Int.MAX_VALUE).toInt()
            
            val builder = NotificationCompat.Builder(context, CHANNEL_ID)
                .setSmallIcon(android.R.drawable.ic_dialog_info)
                .setContentTitle(title)
                .setContentText(body)
                .setPriority(NotificationCompat.PRIORITY_DEFAULT)
                .setAutoCancel(true)
            
            if (sound) {
                builder.setDefaults(NotificationCompat.DEFAULT_SOUND)
            }
            
            badge?.let {
                builder.setNumber(it)
            }
            
            with(NotificationManagerCompat.from(context)) {
                try {
                    notify(notificationId, builder.build())
                    
                    val result = JSONObject().apply {
                        put("ok", true)
                        put("notification_id", notificationId)
                        put("title", title)
                        put("body", body)
                    }
                    callback(Result.success(result))
                } catch (e: SecurityException) {
                    callback(Result.failure(SecurityException("Notification permission denied")))
                }
            }
        } catch (e: Exception) {
            callback(Result.failure(e))
        }
    }
    
    /**
     * Create notification channel (required for Android 8.0+)
     */
    private fun createNotificationChannel() {
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            val importance = NotificationManager.IMPORTANCE_DEFAULT
            val channel = NotificationChannel(CHANNEL_ID, CHANNEL_NAME, importance).apply {
                description = "Notifications from KrabKrab agent"
            }
            
            val notificationManager = 
                context.getSystemService(Context.NOTIFICATION_SERVICE) as NotificationManager
            notificationManager.createNotificationChannel(channel)
        }
    }
    
    /**
     * Cancel a notification
     */
    fun cancelNotification(notificationId: Int) {
        NotificationManagerCompat.from(context).cancel(notificationId)
    }
    
    /**
     * Cancel all notifications
     */
    fun cancelAllNotifications() {
        NotificationManagerCompat.from(context).cancelAll()
    }
}
"#
        .to_string()
    }
}

/// Export all Kotlin code as a combined string
pub fn generate_all_kotlin() -> String {
    format!(
        "{}\n\n{}\n\n{}\n\n{}",
        kotlin::camera_handler_kotlin(),
        kotlin::screen_record_handler_kotlin(),
        kotlin::location_handler_kotlin(),
        kotlin::notification_handler_kotlin()
    )
}
