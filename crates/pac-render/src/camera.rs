//! Camera system for PAC game engine
//!
//! Provides perspective projection, view matrices, and camera controllers
//! for orbit and fly camera modes.

use glam::{Mat4, Quat, Vec2, Vec3};
use pac_math::Transform;

/// A camera with perspective projection and transform
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Camera {
    /// Camera transform (position, rotation, scale)
    pub transform: Transform,
    /// Vertical field of view in radians
    pub fov_y: f32,
    /// Near clipping plane
    pub near: f32,
    /// Far clipping plane
    pub far: f32,
    /// Aspect ratio (width / height)
    pub aspect_ratio: f32,
}

impl Camera {
    /// Default field of view (45 degrees in radians)
    pub const DEFAULT_FOV: f32 = std::f32::consts::FRAC_PI_4;
    /// Default near clipping plane
    pub const DEFAULT_NEAR: f32 = 0.1;
    /// Default far clipping plane
    pub const DEFAULT_FAR: f32 = 1000.0;
    /// Default aspect ratio
    pub const DEFAULT_ASPECT: f32 = 16.0 / 9.0;

    /// Creates a new camera with default settings
    pub fn new() -> Self {
        Self {
            transform: Transform::IDENTITY,
            fov_y: Self::DEFAULT_FOV,
            near: Self::DEFAULT_NEAR,
            far: Self::DEFAULT_FAR,
            aspect_ratio: Self::DEFAULT_ASPECT,
        }
    }

    /// Creates a camera from a transform
    pub fn from_transform(transform: Transform) -> Self {
        Self {
            transform,
            ..Default::default()
        }
    }

    /// Sets the field of view (in radians)
    pub fn with_fov(mut self, fov_y: f32) -> Self {
        self.fov_y = fov_y;
        self
    }

    /// Sets the clipping planes
    pub fn with_clipping_planes(mut self, near: f32, far: f32) -> Self {
        self.near = near;
        self.far = far;
        self
    }

    /// Sets the aspect ratio
    pub fn with_aspect_ratio(mut self, aspect_ratio: f32) -> Self {
        self.aspect_ratio = aspect_ratio;
        self
    }

    /// Returns the perspective projection matrix
    pub fn projection_matrix(&self) -> Mat4 {
        Mat4::perspective_rh_gl(self.fov_y, self.aspect_ratio, self.near, self.far)
    }

    /// Returns the view matrix (inverse of transform's model matrix)
    pub fn view_matrix(&self) -> Mat4 {
        let position = self.transform.position;
        let rotation = self.transform.rotation;

        // View matrix is inverse of camera transform
        // For a camera, we need to translate opposite to position and rotate opposite to rotation
        let rotation_matrix = Mat4::from_quat(rotation.inverse());
        let translation_matrix = Mat4::from_translation(-position);

        rotation_matrix * translation_matrix
    }

    /// Returns the combined view-projection matrix
    pub fn view_projection_matrix(&self) -> Mat4 {
        self.projection_matrix() * self.view_matrix()
    }

    /// Returns the camera's forward direction
    pub fn forward(&self) -> Vec3 {
        self.transform.forward()
    }

    /// Returns the camera's right direction
    pub fn right(&self) -> Vec3 {
        self.transform.right()
    }

    /// Returns the camera's up direction
    pub fn up(&self) -> Vec3 {
        self.transform.up()
    }

    /// Moves the camera by the given delta
    pub fn translate(&mut self, delta: Vec3) {
        self.transform.translate(delta);
    }

    /// Rotates the camera by the given quaternion
    pub fn rotate(&mut self, rotation: Quat) {
        self.transform.rotate_by(rotation);
    }

    /// Makes the camera look at a target point
    pub fn look_at(&mut self, target: Vec3, up: Vec3) {
        self.transform.look_at(target, up);
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self::new()
    }
}

/// Orbit camera controller - camera orbits around a target point
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct OrbitCamera {
    /// The camera being controlled
    pub camera: Camera,
    /// Target point to orbit around
    pub target: Vec3,
    /// Distance from target
    pub distance: f32,
    /// Minimum distance
    pub min_distance: f32,
    /// Maximum distance
    pub max_distance: f32,
    /// Horizontal rotation angle in radians (yaw)
    pub yaw: f32,
    /// Vertical rotation angle in radians (pitch)
    pub pitch: f32,
    /// Minimum pitch angle
    pub min_pitch: f32,
    /// Maximum pitch angle
    pub max_pitch: f32,
    /// Sensitivity for rotation
    pub rotation_sensitivity: f32,
    /// Sensitivity for zoom
    pub zoom_sensitivity: f32,
}

impl OrbitCamera {
    /// Default minimum distance
    pub const DEFAULT_MIN_DISTANCE: f32 = 0.1;
    /// Default maximum distance
    pub const DEFAULT_MAX_DISTANCE: f32 = 1000.0;
    /// Default minimum pitch (in radians, -85 degrees)
    pub const DEFAULT_MIN_PITCH: f32 = -1.48;
    /// Default maximum pitch (in radians, 85 degrees)
    pub const DEFAULT_MAX_PITCH: f32 = 1.48;
    /// Default rotation sensitivity
    pub const DEFAULT_ROTATION_SENSITIVITY: f32 = 0.005;
    /// Default zoom sensitivity
    pub const DEFAULT_ZOOM_SENSITIVITY: f32 = 0.1;

    /// Creates a new orbit camera
    pub fn new() -> Self {
        let mut camera = Self {
            camera: Camera::new(),
            target: Vec3::ZERO,
            distance: 10.0,
            min_distance: Self::DEFAULT_MIN_DISTANCE,
            max_distance: Self::DEFAULT_MAX_DISTANCE,
            yaw: 0.0,
            pitch: 0.0,
            min_pitch: Self::DEFAULT_MIN_PITCH,
            max_pitch: Self::DEFAULT_MAX_PITCH,
            rotation_sensitivity: Self::DEFAULT_ROTATION_SENSITIVITY,
            zoom_sensitivity: Self::DEFAULT_ZOOM_SENSITIVITY,
        };
        camera.update_position();
        camera
    }

    /// Sets the target point to orbit around
    pub fn with_target(mut self, target: Vec3) -> Self {
        self.target = target;
        self.update_position();
        self
    }

    /// Sets the orbit distance
    pub fn with_distance(mut self, distance: f32) -> Self {
        self.distance = distance.clamp(self.min_distance, self.max_distance);
        self.update_position();
        self
    }

    /// Sets the distance limits
    pub fn with_distance_limits(mut self, min: f32, max: f32) -> Self {
        self.min_distance = min;
        self.max_distance = max;
        self.distance = self.distance.clamp(min, max);
        self.update_position();
        self
    }

    /// Sets the pitch limits
    pub fn with_pitch_limits(mut self, min: f32, max: f32) -> Self {
        self.min_pitch = min;
        self.max_pitch = max;
        self.pitch = self.pitch.clamp(min, max);
        self.update_position();
        self
    }

    /// Updates the camera position based on orbit parameters
    fn update_position(&mut self) {
        // Calculate position on sphere around target
        let cos_pitch = self.pitch.cos();
        let sin_pitch = self.pitch.sin();
        let cos_yaw = self.yaw.cos();
        let sin_yaw = self.yaw.sin();

        let offset = Vec3::new(
            self.distance * cos_pitch * sin_yaw,
            self.distance * sin_pitch,
            self.distance * cos_pitch * cos_yaw,
        );

        self.camera.transform.position = self.target + offset;
        self.camera.look_at(self.target, Vec3::Y);
    }

    /// Rotates the camera by delta yaw and pitch
    pub fn rotate(&mut self, delta_yaw: f32, delta_pitch: f32) {
        self.yaw += delta_yaw * self.rotation_sensitivity;
        self.pitch += delta_pitch * self.rotation_sensitivity;
        self.pitch = self.pitch.clamp(self.min_pitch, self.max_pitch);
        self.update_position();
    }

    /// Rotates the camera from mouse movement
    pub fn rotate_from_mouse(&mut self, delta: Vec2) {
        self.rotate(delta.x, -delta.y);
    }

    /// Zooms the camera by changing distance
    pub fn zoom(&mut self, delta: f32) {
        let zoom_factor = 1.0 + delta * self.zoom_sensitivity;
        self.distance = (self.distance * zoom_factor).clamp(self.min_distance, self.max_distance);
        self.update_position();
    }

    /// Pans the camera target (and camera position) by the given delta
    pub fn pan(&mut self, delta_right: f32, delta_up: f32) {
        let right = self.camera.right();
        let up = self.camera.up();

        let pan_delta = right * delta_right + up * delta_up;
        self.target += pan_delta;
        self.update_position();
    }

    /// Updates the camera transform based on current parameters
    pub fn update(&mut self) {
        self.update_position();
    }
}

impl Default for OrbitCamera {
    fn default() -> Self {
        Self::new()
    }
}

/// Fly camera controller - first-person style camera
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FlyCamera {
    /// The camera being controlled
    pub camera: Camera,
    /// Movement speed
    pub speed: f32,
    /// Mouse look sensitivity
    pub sensitivity: f32,
    /// Current yaw angle (horizontal rotation)
    pub yaw: f32,
    /// Current pitch angle (vertical rotation)
    pub pitch: f32,
    /// Minimum pitch angle
    pub min_pitch: f32,
    /// Maximum pitch angle
    pub max_pitch: f32,
}

impl FlyCamera {
    /// Default movement speed
    pub const DEFAULT_SPEED: f32 = 5.0;
    /// Default mouse sensitivity
    pub const DEFAULT_SENSITIVITY: f32 = 0.002;
    /// Default minimum pitch (in radians, -89 degrees)
    pub const DEFAULT_MIN_PITCH: f32 = -1.55;
    /// Default maximum pitch (in radians, 89 degrees)
    pub const DEFAULT_MAX_PITCH: f32 = 1.55;

    /// Creates a new fly camera
    pub fn new() -> Self {
        Self {
            camera: Camera::new(),
            speed: Self::DEFAULT_SPEED,
            sensitivity: Self::DEFAULT_SENSITIVITY,
            yaw: 0.0,
            pitch: 0.0,
            min_pitch: Self::DEFAULT_MIN_PITCH,
            max_pitch: Self::DEFAULT_MAX_PITCH,
        }
    }

    /// Sets the movement speed
    pub fn with_speed(mut self, speed: f32) -> Self {
        self.speed = speed;
        self
    }

    /// Sets the mouse sensitivity
    pub fn with_sensitivity(mut self, sensitivity: f32) -> Self {
        self.sensitivity = sensitivity;
        self
    }

    /// Sets the pitch limits
    pub fn with_pitch_limits(mut self, min: f32, max: f32) -> Self {
        self.min_pitch = min;
        self.max_pitch = max;
        self
    }

    /// Updates camera rotation from mouse movement
    pub fn rotate_from_mouse(&mut self, delta: Vec2) {
        self.yaw += delta.x * self.sensitivity;
        self.pitch += -delta.y * self.sensitivity;
        self.pitch = self.pitch.clamp(self.min_pitch, self.max_pitch);
        self.update_rotation();
    }

    /// Updates the camera rotation from yaw and pitch
    fn update_rotation(&mut self) {
        // Create rotation from yaw (Y axis) and pitch (X axis)
        let yaw_rotation = Quat::from_rotation_y(self.yaw);
        let pitch_rotation = Quat::from_rotation_x(self.pitch);
        self.camera.transform.rotation = yaw_rotation * pitch_rotation;
    }

    /// Moves the camera forward by the given amount
    pub fn move_forward(&mut self, amount: f32) {
        let forward = self.camera.forward();
        self.camera.transform.position += forward * amount * self.speed;
    }

    /// Moves the camera backward by the given amount
    pub fn move_backward(&mut self, amount: f32) {
        self.move_forward(-amount);
    }

    /// Moves the camera right by the given amount
    pub fn move_right(&mut self, amount: f32) {
        let right = self.camera.right();
        self.camera.transform.position += right * amount * self.speed;
    }

    /// Moves the camera left by the given amount
    pub fn move_left(&mut self, amount: f32) {
        self.move_right(-amount);
    }

    /// Moves the camera up by the given amount
    pub fn move_up(&mut self, amount: f32) {
        self.camera.transform.position += Vec3::Y * amount * self.speed;
    }

    /// Moves the camera down by the given amount
    pub fn move_down(&mut self, amount: f32) {
        self.move_up(-amount);
    }

    /// Moves the camera with WASD-style input
    /// forward/back: -1.0 to 1.0 (negative is backward)
    /// right/left: -1.0 to 1.0 (negative is left)
    pub fn move_wasd(&mut self, forward_back: f32, right_left: f32, delta_time: f32) {
        let forward = self.camera.forward();
        let right = self.camera.right();

        let movement = (forward * forward_back + right * right_left).normalize_or_zero();
        self.camera.transform.position += movement * self.speed * delta_time;
    }
}

impl Default for FlyCamera {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_camera_default() {
        let camera = Camera::new();
        assert_eq!(camera.fov_y, Camera::DEFAULT_FOV);
        assert_eq!(camera.near, Camera::DEFAULT_NEAR);
        assert_eq!(camera.far, Camera::DEFAULT_FAR);
        assert_eq!(camera.aspect_ratio, Camera::DEFAULT_ASPECT);
    }

    #[test]
    fn test_camera_projection_matrix() {
        let camera = Camera::new();
        let proj = camera.projection_matrix();
        // Projection matrix should not be identity
        assert_ne!(proj, Mat4::IDENTITY);
    }

    #[test]
    fn test_camera_view_matrix() {
        let mut camera = Camera::new();
        camera.transform.position = Vec3::new(0.0, 0.0, 5.0);
        camera.look_at(Vec3::ZERO, Vec3::Y);

        let view = camera.view_matrix();
        // View matrix should not be identity
        assert_ne!(view, Mat4::IDENTITY);
    }

    #[test]
    fn test_orbit_camera_rotation() {
        let mut orbit = OrbitCamera::new();
        let initial_yaw = orbit.yaw;
        let initial_pitch = orbit.pitch;

        orbit.rotate(1.0, 0.5);

        assert_ne!(orbit.yaw, initial_yaw);
        assert_ne!(orbit.pitch, initial_pitch);
    }

    #[test]
    fn test_orbit_camera_zoom() {
        let mut orbit = OrbitCamera::new();
        let initial_distance = orbit.distance;

        orbit.zoom(1.0);

        assert_ne!(orbit.distance, initial_distance);
    }

    #[test]
    fn test_fly_camera_rotation() {
        let mut fly = FlyCamera::new();
        let initial_yaw = fly.yaw;
        let initial_pitch = fly.pitch;

        fly.rotate_from_mouse(Vec2::new(100.0, 50.0));

        assert_ne!(fly.yaw, initial_yaw);
        assert_ne!(fly.pitch, initial_pitch);
    }

    #[test]
    fn test_fly_camera_movement() {
        let mut fly = FlyCamera::new();
        let initial_pos = fly.camera.transform.position;

        fly.move_forward(1.0);

        assert_ne!(fly.camera.transform.position, initial_pos);
    }

    #[test]
    fn test_pitch_clamping() {
        let mut fly = FlyCamera::new();

        // Try to rotate past max pitch
        fly.pitch = fly.max_pitch;
        fly.rotate_from_mouse(Vec2::new(0.0, -1000.0));
        assert!(fly.pitch <= fly.max_pitch);

        // Reset and try to rotate past min pitch
        fly.pitch = fly.min_pitch;
        fly.rotate_from_mouse(Vec2::new(0.0, 1000.0));
        assert!(fly.pitch >= fly.min_pitch);
    }

    #[test]
    fn test_camera_builder_pattern() {
        let camera = Camera::new()
            .with_fov(std::f32::consts::FRAC_PI_3)
            .with_clipping_planes(0.01, 100.0)
            .with_aspect_ratio(4.0 / 3.0);

        assert_eq!(camera.fov_y, std::f32::consts::FRAC_PI_3);
        assert_eq!(camera.near, 0.01);
        assert_eq!(camera.far, 100.0);
        assert_eq!(camera.aspect_ratio, 4.0 / 3.0);
    }

    #[test]
    fn test_orbit_camera_builder() {
        let orbit = OrbitCamera::new()
            .with_target(Vec3::new(1.0, 2.0, 3.0))
            .with_distance(20.0)
            .with_distance_limits(1.0, 50.0)
            .with_pitch_limits(-1.0, 1.0);

        assert_eq!(orbit.target, Vec3::new(1.0, 2.0, 3.0));
        assert_eq!(orbit.distance, 20.0);
        assert_eq!(orbit.min_distance, 1.0);
        assert_eq!(orbit.max_distance, 50.0);
        assert_eq!(orbit.min_pitch, -1.0);
        assert_eq!(orbit.max_pitch, 1.0);
    }

    #[test]
    fn test_fly_camera_builder() {
        let fly = FlyCamera::new()
            .with_speed(10.0)
            .with_sensitivity(0.01)
            .with_pitch_limits(-1.2, 1.2);

        assert_eq!(fly.speed, 10.0);
        assert_eq!(fly.sensitivity, 0.01);
        assert_eq!(fly.min_pitch, -1.2);
        assert_eq!(fly.max_pitch, 1.2);
    }
}
