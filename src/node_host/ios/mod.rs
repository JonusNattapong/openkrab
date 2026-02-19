//! iOS Native Module for node_host
//!
//! This module provides Swift-based native functionality for iOS devices:
//! - Camera snap (photo capture)
//! - Screen recording
//! - Location services
//! - Notifications
//!
//! Usage from Rust:
//! ```json
//! {
//!   "action": "camera.snap",
//!   "payload": {"node_id": "ios-1", "camera": "back", "quality": "high"}
//! }
//! ```

use serde::{Deserialize, Serialize};

/// iOS camera configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IosCameraConfig {
    /// Camera position: "front" or "back"
    pub camera: String,
    /// Image quality: "low", "medium", "high"
    pub quality: String,
    /// Enable flash
    pub flash: bool,
}

impl Default for IosCameraConfig {
    fn default() -> Self {
        Self {
            camera: "back".to_string(),
            quality: "high".to_string(),
            flash: false,
        }
    }
}

/// iOS screen recording configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IosScreenRecordConfig {
    /// Enable audio recording
    pub audio: bool,
    /// Video quality: "low", "medium", "high"
    pub quality: String,
    /// Recording duration in seconds
    pub duration_secs: u32,
}

impl Default for IosScreenRecordConfig {
    fn default() -> Self {
        Self {
            audio: false,
            quality: "high".to_string(),
            duration_secs: 60,
        }
    }
}

/// iOS location configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IosLocationConfig {
    /// Accuracy level: "high", "balanced", "low"
    pub accuracy: String,
    /// Timeout in milliseconds
    pub timeout_ms: u32,
}

impl Default for IosLocationConfig {
    fn default() -> Self {
        Self {
            accuracy: "high".to_string(),
            timeout_ms: 10000,
        }
    }
}

/// iOS notification configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IosNotificationConfig {
    /// Notification title
    pub title: String,
    /// Notification body
    pub body: String,
    /// Play sound
    pub sound: bool,
    /// Badge number
    pub badge: Option<u32>,
}

impl Default for IosNotificationConfig {
    fn default() -> Self {
        Self {
            title: "Notification".to_string(),
            body: String::new(),
            sound: true,
            badge: None,
        }
    }
}

/// Swift code generation for iOS native handlers
pub mod swift {
    /// Generate Swift code for camera snap handler
    pub fn camera_handler_swift() -> String {
        r#"import AVFoundation
import UIKit

class KrabKrabCameraHandler: NSObject {
    
    /// Capture a photo from the device camera
    /// - Parameters:
    ///   - camera: "front" or "back"
    ///   - quality: "low", "medium", "high"
    ///   - flash: Enable flash
    ///   - completion: Callback with image data or error
    func capturePhoto(
        camera: String,
        quality: String,
        flash: Bool,
        completion: @escaping (Result<Data, Error>) -> Void
    ) {
        guard UIImagePickerController.isSourceTypeAvailable(.camera) else {
            completion(.failure(CameraError.cameraUnavailable))
            return
        }
        
        // Note: In production, implement custom camera capture
        // This is a placeholder for the native implementation
        completion(.failure(CameraError.notImplemented))
    }
}

enum CameraError: Error {
    case cameraUnavailable
    case notImplemented
    case captureFailure(String)
}

// MARK: - JavaScript Bridge
@objc(KrabKrabCameraBridge)
class KrabKrabCameraBridge: NSObject {
    
    @objc
    func capturePhoto(
        _ nodeId: String,
        camera: String,
        quality: String,
        flash: Bool,
        completion: @escaping (String) -> Void
    ) {
        let handler = KrabKrabCameraHandler()
        handler.capturePhoto(camera: camera, quality: quality, flash: flash) { result in
            switch result {
            case .success(let data):
                let response: [String: Any] = [
                    "ok": true,
                    "node_id": nodeId,
                    "camera": camera,
                    "quality": quality,
                    "flash": flash,
                    "data": data.base64EncodedString()
                ]
                if let jsonData = try? JSONSerialization.data(withJSONObject: response),
                   let jsonString = String(data: jsonData, encoding: .utf8) {
                    completion(jsonString)
                }
            case .failure(let error):
                let response: [String: Any] = [
                    "ok": false,
                    "node_id": nodeId,
                    "error": error.localizedDescription
                ]
                if let jsonData = try? JSONSerialization.data(withJSONObject: response),
                   let jsonString = String(data: jsonData, encoding: .utf8) {
                    completion(jsonString)
                }
            }
        }
    }
}
"#
        .to_string()
    }

    /// Generate Swift code for screen recording handler
    pub fn screen_record_handler_swift() -> String {
        r#"import ReplayKit
import UIKit

class KrabKrabScreenRecordHandler: NSObject {
    
    private let recorder = RPRecorderFactory.shared()
    private var isRecording = false
    
    /// Start screen recording
    /// - Parameters:
    ///   - audio: Enable audio recording
    ///   - quality: "low", "medium", "high"
    ///   - duration: Recording duration in seconds
    ///   - completion: Callback with recording result
    func startRecording(
        audio: Bool,
        quality: String,
        duration: UInt,
        completion: @escaping (Result<String, Error>) -> Void
    ) {
        guard !isRecording else {
            completion(.failure(ScreenRecordError.alreadyRecording))
            return
        }
        
        // Note: In production, implement ReplayKit screen recording
        // This is a placeholder for the native implementation
        completion(.failure(ScreenRecordError.notImplemented))
    }
    
    /// Stop screen recording
    func stopRecording(completion: @escaping (Result<URL, Error>) -> Void) {
        guard isRecording else {
            completion(.failure(ScreenRecordError.notRecording))
            return
        }
        isRecording = false
        // Return recorded video URL
        completion(.failure(ScreenRecordError.notImplemented))
    }
}

enum ScreenRecordError: Error {
    case alreadyRecording
    case notRecording
    case notImplemented
    case recordingFailed(String)
}

// MARK: - JavaScript Bridge
@objc(KrabKrabScreenRecordBridge)
class KrabKrabScreenRecordBridge: NSObject {
    
    @objc
    func startRecording(
        _ nodeId: String,
        audio: Bool,
        quality: String,
        durationSecs: UInt,
        completion: @escaping (String) -> Void
    ) {
        let handler = KrabKrabScreenRecordHandler()
        handler.startRecording(audio: audio, quality: quality, duration: durationSecs) { result in
            switch result {
            case .success(let recordingId):
                let response: [String: Any] = [
                    "ok": true,
                    "node_id": nodeId,
                    "audio": audio,
                    "quality": quality,
                    "duration_secs": durationSecs,
                    "recording_id": recordingId
                ]
                if let jsonData = try? JSONSerialization.data(withJSONObject: response),
                   let jsonString = String(data: jsonData, encoding: .utf8) {
                    completion(jsonString)
                }
            case .failure(let error):
                let response: [String: Any] = [
                    "ok": false,
                    "node_id": nodeId,
                    "error": error.localizedDescription
                ]
                if let jsonData = try? JSONSerialization.data(withJSONObject: response),
                   let jsonString = String(data: jsonData, encoding: .utf8) {
                    completion(jsonString)
                }
            }
        }
    }
}
"#
        .to_string()
    }

    /// Generate Swift code for location handler
    pub fn location_handler_swift() -> String {
        r#"import CoreLocation

class KrabKrabLocationHandler: NSObject, CLLocationManagerDelegate {
    
    private var locationManager: CLLocationManager?
    private var locationCompletion: ((Result<CLLocation, Error>) -> Void)?
    
    /// Get current location
    /// - Parameters:
    ///   - accuracy: "high", "balanced", "low"
    ///   - timeout: Timeout in milliseconds
    ///   - completion: Callback with location or error
    func getLocation(
        accuracy: String,
        timeout: UInt,
        completion: @escaping (Result<CLLocation, Error>) -> Void
    ) {
        locationCompletion = completion
        locationManager = CLLocationManager()
        locationManager?.delegate = self
        
        // Set accuracy based on parameter
        switch accuracy {
        case "high":
            locationManager?.desiredAccuracy = kCLLocationAccuracyBest
        case "balanced":
            locationManager?.desiredAccuracy = kCLLocationAccuracyHundredMeters
        case "low":
            locationManager?.desiredAccuracy = kCLLocationAccuracyKilometer
        default:
            locationManager?.desiredAccuracy = kCLLocationAccuracyBest
        }
        
        let status = locationManager?.authorizationStatus ?? .notDetermined
        
        switch status {
        case .authorizedWhenInUse, .authorizedAlways:
            locationManager?.requestLocation()
        case .notDetermined:
            locationManager?.requestWhenInUseAuthorization()
        case .denied, .restricted:
            completion(.failure(LocationError.permissionDenied))
        @unknown default:
            completion(.failure(LocationError.unknown))
        }
    }
    
    // MARK: - CLLocationManagerDelegate
    
    func locationManager(_ manager: CLLocationManager, didUpdateLocations locations: [CLLocation]) {
        if let location = locations.first {
            locationCompletion?(.success(location))
        }
        locationCompletion = nil
    }
    
    func locationManager(_ manager: CLLocationManager, didFailWithError error: Error) {
        locationCompletion?(.failure(error))
        locationCompletion = nil
    }
}

enum LocationError: Error {
    case permissionDenied
    case locationUnavailable
    case timeout
    case unknown
}

// MARK: - JavaScript Bridge
@objc(KrabKrabLocationBridge)
class KrabKrabLocationBridge: NSObject {
    
    @objc
    func getLocation(
        _ nodeId: String,
        accuracy: String,
        timeoutMs: UInt,
        completion: @escaping (String) -> Void
    ) {
        let handler = KrabKrabLocationHandler()
        handler.getLocation(accuracy: accuracy, timeout: timeoutMs) { result in
            switch result {
            case .success(let location):
                let response: [String: Any] = [
                    "ok": true,
                    "node_id": nodeId,
                    "accuracy": accuracy,
                    "timeout_ms": timeoutMs,
                    "latitude": location.coordinate.latitude,
                    "longitude": location.coordinate.longitude,
                    "altitude": location.altitude,
                    "accuracy_meters": location.horizontalAccuracy
                ]
                if let jsonData = try? JSONSerialization.data(withJSONObject: response),
                   let jsonString = String(data: jsonData, encoding: .utf8) {
                    completion(jsonString)
                }
            case .failure(let error):
                let response: [String: Any] = [
                    "ok": false,
                    "node_id": nodeId,
                    "error": error.localizedDescription
                ]
                if let jsonData = try? JSONSerialization.data(withJSONObject: response),
                   let jsonString = String(data: jsonData, encoding: .utf8) {
                    completion(jsonString)
                }
            }
        }
    }
}
"#
        .to_string()
    }

    /// Generate Swift code for notification handler
    pub fn notification_handler_swift() -> String {
        r#"import UserNotifications
import UIKit

class KrabKrabNotificationHandler: NSObject, UNUserNotificationCenterDelegate {
    
    /// Send a local notification
    /// - Parameters:
    ///   - title: Notification title
    ///   - body: Notification body
    ///   - sound: Play sound
    ///   - badge: Badge number (optional)
    ///   - completion: Callback with result
    func sendNotification(
        title: String,
        body: String,
        sound: Bool,
        badge: UInt?,
        completion: @escaping (Result<Void, Error>) -> Void
    ) {
        let center = UNUserNotificationCenter.current()
        
        // Request permission if needed
        center.requestAuthorization(options: [.alert, .sound, .badge]) { granted, error in
            if let error = error {
                completion(.failure(error))
                return
            }
            
            guard granted else {
                completion(.failure(NotificationError.permissionDenied))
                return
            }
            
            let content = UNMutableNotificationContent()
            content.title = title
            content.body = body
            if sound {
                content.sound = .default
            }
            if let badge = badge {
                content.badge = NSNumber(value: badge)
            }
            
            // Create immediate trigger
            let trigger = UNTimeIntervalNotificationTrigger(timeInterval: 0.1, repeats: false)
            
            let request = UNNotificationRequest(
                identifier: UUID().uuidString,
                content: content,
                trigger: trigger
            )
            
            center.add(request) { error in
                if let error = error {
                    completion(.failure(error))
                } else {
                    completion(.success(()))
                }
            }
        }
    }
}

enum NotificationError: Error {
    case permissionDenied
    case sendFailed(String)
}

// MARK: - JavaScript Bridge
@objc(KrabKrabNotificationBridge)
class KrabKrabNotificationBridge: NSObject {
    
    @objc
    func sendNotification(
        _ nodeId: String,
        title: String,
        body: String,
        sound: Bool,
        badge: NSNumber?,
        completion: @escaping (String) -> Void
    ) {
        let handler = KrabKrabNotificationHandler()
        let badgeValue = badge?.uintValue
        
        handler.sendNotification(
            title: title,
            body: body,
            sound: sound,
            badge: badgeValue
        ) { result in
            switch result {
            case .success:
                let response: [String: Any] = [
                    "ok": true,
                    "node_id": nodeId,
                    "title": title,
                    "body": body,
                    "sound": sound
                ]
                if let jsonData = try? JSONSerialization.data(withJSONObject: response),
                   let jsonString = String(data: jsonData, encoding: .utf8) {
                    completion(jsonString)
                }
            case .failure(let error):
                let response: [String: Any] = [
                    "ok": false,
                    "node_id": nodeId,
                    "error": error.localizedDescription
                ]
                if let jsonData = try? JSONSerialization.data(withJSONObject: response),
                   let jsonString = String(data: jsonData, encoding: .utf8) {
                    completion(jsonString)
                }
            }
        }
    }
}
"#
        .to_string()
    }
}

/// Export all Swift code as a combined string
pub fn generate_all_swift() -> String {
    format!(
        "{}\n\n{}\n\n{}\n\n{}",
        swift::camera_handler_swift(),
        swift::screen_record_handler_swift(),
        swift::location_handler_swift(),
        swift::notification_handler_swift()
    )
}
