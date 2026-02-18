//! Input handling for PAC game engine
//!
//! Provides keyboard and mouse state tracking with event handling.

use std::collections::HashSet;
use winit::{
    event::{ElementState, KeyEvent, MouseButton, WindowEvent},
    keyboard::{KeyCode, PhysicalKey},
};

/// Tracks the current state of keyboard and mouse input
#[derive(Debug, Clone, Default)]
pub struct InputState {
    /// Set of currently pressed keys
    keys_pressed: HashSet<KeyCode>,
    /// Set of keys that were pressed this frame (for single-trigger detection)
    keys_just_pressed: HashSet<KeyCode>,
    /// Set of keys that were released this frame
    keys_just_released: HashSet<KeyCode>,
    /// Current mouse position in screen coordinates
    mouse_position: Option<(f64, f64)>,
    /// Previous mouse position for delta calculation
    previous_mouse_position: Option<(f64, f64)>,
    /// Set of currently pressed mouse buttons
    mouse_buttons_pressed: HashSet<MouseButton>,
    /// Set of mouse buttons pressed this frame
    mouse_buttons_just_pressed: HashSet<MouseButton>,
    /// Set of mouse buttons released this frame
    mouse_buttons_just_released: HashSet<MouseButton>,
    /// Mouse wheel delta for this frame
    mouse_wheel_delta: (f32, f32),
    /// Cursor is currently within window bounds
    cursor_in_window: bool,
}

impl InputState {
    /// Creates a new empty input state
    pub fn new() -> Self {
        Self::default()
    }

    /// Updates the input state at the beginning of each frame
    /// Clears the "just" states so they only last one frame
    pub fn update(&mut self) {
        self.keys_just_pressed.clear();
        self.keys_just_released.clear();
        self.mouse_buttons_just_pressed.clear();
        self.mouse_buttons_just_released.clear();
        self.mouse_wheel_delta = (0.0, 0.0);
        self.previous_mouse_position = self.mouse_position;
    }

    /// Processes a window event and updates input state accordingly
    pub fn handle_event(&mut self, event: &WindowEvent) {
        match event {
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(key_code),
                        state,
                        ..
                    },
                ..
            } => {
                self.handle_key(*key_code, *state);
            }
            WindowEvent::CursorMoved { position, .. } => {
                self.mouse_position = Some((position.x, position.y));
            }
            WindowEvent::CursorEntered { .. } => {
                self.cursor_in_window = true;
            }
            WindowEvent::CursorLeft { .. } => {
                self.cursor_in_window = false;
                self.mouse_position = None;
            }
            WindowEvent::MouseInput { button, state, .. } => {
                self.handle_mouse_button(*button, *state);
            }
            WindowEvent::MouseWheel { delta, .. } => {
                self.handle_mouse_wheel(*delta);
            }
            _ => {}
        }
    }

    /// Handles a key event
    fn handle_key(&mut self, key_code: KeyCode, state: ElementState) {
        match state {
            ElementState::Pressed => {
                if !self.keys_pressed.contains(&key_code) {
                    self.keys_just_pressed.insert(key_code);
                }
                self.keys_pressed.insert(key_code);
            }
            ElementState::Released => {
                self.keys_pressed.remove(&key_code);
                self.keys_just_released.insert(key_code);
            }
        }
    }

    /// Handles a mouse button event
    fn handle_mouse_button(&mut self, button: MouseButton, state: ElementState) {
        match state {
            ElementState::Pressed => {
                if !self.mouse_buttons_pressed.contains(&button) {
                    self.mouse_buttons_just_pressed.insert(button);
                }
                self.mouse_buttons_pressed.insert(button);
            }
            ElementState::Released => {
                self.mouse_buttons_pressed.remove(&button);
                self.mouse_buttons_just_released.insert(button);
            }
        }
    }

    /// Handles mouse wheel events
    fn handle_mouse_wheel(&mut self, delta: winit::event::MouseScrollDelta) {
        match delta {
            winit::event::MouseScrollDelta::LineDelta(x, y) => {
                self.mouse_wheel_delta.0 += x;
                self.mouse_wheel_delta.1 += y;
            }
            winit::event::MouseScrollDelta::PixelDelta(pos) => {
                // Convert pixel delta to line delta approximation
                const PIXELS_PER_LINE: f64 = 50.0;
                self.mouse_wheel_delta.0 += (pos.x / PIXELS_PER_LINE) as f32;
                self.mouse_wheel_delta.1 += (pos.y / PIXELS_PER_LINE) as f32;
            }
        }
    }

    /// Returns true if the key is currently being held down
    pub fn is_key_pressed(&self, key_code: KeyCode) -> bool {
        self.keys_pressed.contains(&key_code)
    }

    /// Returns true if the key was pressed this frame (single trigger)
    pub fn is_key_just_pressed(&self, key_code: KeyCode) -> bool {
        self.keys_just_pressed.contains(&key_code)
    }

    /// Returns true if the key was released this frame
    pub fn is_key_just_released(&self, key_code: KeyCode) -> bool {
        self.keys_just_released.contains(&key_code)
    }

    /// Returns true if the mouse button is currently being held down
    pub fn is_mouse_button_pressed(&self, button: MouseButton) -> bool {
        self.mouse_buttons_pressed.contains(&button)
    }

    /// Returns true if the mouse button was pressed this frame (single trigger)
    pub fn is_mouse_button_just_pressed(&self, button: MouseButton) -> bool {
        self.mouse_buttons_just_pressed.contains(&button)
    }

    /// Returns true if the mouse button was released this frame
    pub fn is_mouse_button_just_released(&self, button: MouseButton) -> bool {
        self.mouse_buttons_just_released.contains(&button)
    }

    /// Returns the current mouse position in screen coordinates
    pub fn mouse_position(&self) -> Option<(f64, f64)> {
        self.mouse_position
    }

    /// Returns the mouse delta (movement) since the last frame
    pub fn mouse_delta(&self) -> Option<(f64, f64)> {
        match (self.previous_mouse_position, self.mouse_position) {
            (Some(prev), Some(curr)) => Some((curr.0 - prev.0, curr.1 - prev.1)),
            _ => None,
        }
    }

    /// Returns the mouse wheel delta for this frame
    pub fn mouse_wheel_delta(&self) -> (f32, f32) {
        self.mouse_wheel_delta
    }

    /// Returns true if the cursor is within the window bounds
    pub fn is_cursor_in_window(&self) -> bool {
        self.cursor_in_window
    }

    /// Returns the set of all currently pressed keys
    pub fn pressed_keys(&self) -> &HashSet<KeyCode> {
        &self.keys_pressed
    }

    /// Returns the set of all currently pressed mouse buttons
    pub fn pressed_mouse_buttons(&self) -> &HashSet<MouseButton> {
        &self.mouse_buttons_pressed
    }
}

/// Action-based input mapping for game actions
#[derive(Debug, Clone, Default)]
pub struct InputMap {
    /// Maps action names to their keyboard bindings
    keyboard_bindings: std::collections::HashMap<String, Vec<KeyCode>>,
    /// Maps action names to their mouse button bindings
    mouse_bindings: std::collections::HashMap<String, Vec<MouseButton>>,
}

impl InputMap {
    /// Creates a new empty input map
    pub fn new() -> Self {
        Self::default()
    }

    /// Binds a key to an action
    pub fn bind_key(&mut self, action: impl Into<String>, key: KeyCode) {
        self.keyboard_bindings
            .entry(action.into())
            .or_default()
            .push(key);
    }

    /// Binds a mouse button to an action
    pub fn bind_mouse_button(&mut self, action: impl Into<String>, button: MouseButton) {
        self.mouse_bindings
            .entry(action.into())
            .or_default()
            .push(button);
    }

    /// Returns true if the action is currently active
    pub fn is_action_active(&self, action: &str, input_state: &InputState) -> bool {
        // Check keyboard bindings
        if let Some(keys) = self.keyboard_bindings.get(action) {
            if keys.iter().any(|key| input_state.is_key_pressed(*key)) {
                return true;
            }
        }

        // Check mouse bindings
        if let Some(buttons) = self.mouse_bindings.get(action) {
            if buttons
                .iter()
                .any(|btn| input_state.is_mouse_button_pressed(*btn))
            {
                return true;
            }
        }

        false
    }

    /// Returns true if the action was just activated this frame
    pub fn is_action_just_pressed(&self, action: &str, input_state: &InputState) -> bool {
        // Check keyboard bindings
        if let Some(keys) = self.keyboard_bindings.get(action) {
            if keys.iter().any(|key| input_state.is_key_just_pressed(*key)) {
                return true;
            }
        }

        // Check mouse bindings
        if let Some(buttons) = self.mouse_bindings.get(action) {
            if buttons
                .iter()
                .any(|btn| input_state.is_mouse_button_just_pressed(*btn))
            {
                return true;
            }
        }

        false
    }

    /// Removes all bindings for an action
    pub fn unbind_action(&mut self, action: &str) {
        self.keyboard_bindings.remove(action);
        self.mouse_bindings.remove(action);
    }

    /// Clears all bindings
    pub fn clear_bindings(&mut self) {
        self.keyboard_bindings.clear();
        self.mouse_bindings.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use winit::keyboard::KeyCode;

    #[test]
    fn test_key_press_and_release() {
        let mut input = InputState::new();

        // Simulate pressing a key
        input.handle_key(KeyCode::KeyW, ElementState::Pressed);
        assert!(input.is_key_pressed(KeyCode::KeyW));
        assert!(input.is_key_just_pressed(KeyCode::KeyW));

        // Update clears "just" states
        input.update();
        assert!(input.is_key_pressed(KeyCode::KeyW));
        assert!(!input.is_key_just_pressed(KeyCode::KeyW));

        // Simulate releasing the key
        input.handle_key(KeyCode::KeyW, ElementState::Released);
        assert!(!input.is_key_pressed(KeyCode::KeyW));
        assert!(input.is_key_just_released(KeyCode::KeyW));

        // Update clears "just" states
        input.update();
        assert!(!input.is_key_just_released(KeyCode::KeyW));
    }

    #[test]
    fn test_input_map() {
        let mut input_map = InputMap::new();
        let mut input_state = InputState::new();

        input_map.bind_key("move_forward", KeyCode::KeyW);
        input_map.bind_key("move_forward", KeyCode::ArrowUp);

        // Should not be active initially
        assert!(!input_map.is_action_active("move_forward", &input_state));

        // Press the key
        input_state.handle_key(KeyCode::KeyW, ElementState::Pressed);
        assert!(input_map.is_action_active("move_forward", &input_state));
        assert!(input_map.is_action_just_pressed("move_forward", &input_state));
    }

    #[test]
    fn test_mouse_button_tracking() {
        let mut input = InputState::new();

        input.handle_mouse_button(MouseButton::Left, ElementState::Pressed);
        assert!(input.is_mouse_button_pressed(MouseButton::Left));
        assert!(input.is_mouse_button_just_pressed(MouseButton::Left));

        input.update();
        assert!(!input.is_mouse_button_just_pressed(MouseButton::Left));

        input.handle_mouse_button(MouseButton::Left, ElementState::Released);
        assert!(!input.is_mouse_button_pressed(MouseButton::Left));
        assert!(input.is_mouse_button_just_released(MouseButton::Left));
    }
}
