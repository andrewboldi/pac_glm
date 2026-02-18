//! GPU context management for wgpu rendering

use wgpu::{Instance, Surface, Device, Queue, SurfaceConfiguration};
use std::sync::Arc;

/// GPU context containing all necessary wgpu handles for rendering
pub struct RenderContext {
    /// WGPU instance for GPU enumeration
    pub instance: Instance,
    /// Surface for presenting rendered frames
    pub surface: Surface<'static>,
    /// GPU device for command submission
    pub device: Device,
    /// Command queue for submitting work to the GPU
    pub queue: Queue,
    /// Surface configuration (format, size, present mode)
    pub config: SurfaceConfiguration,
}

impl RenderContext {
    /// Create a new render context with the given window
    /// 
    /// # Arguments
    /// * `window` - The window to create a surface for
    pub async fn new(window: Arc<winit::window::Window>) -> Self {
        // Create wgpu instance
        let instance = Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            ..Default::default()
        });

        // Create surface
        let surface = instance.create_surface(window.clone())
            .expect("Failed to create surface");

        // Request adapter (GPU)
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .expect("Failed to find an appropriate adapter");

        // Get surface capabilities
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats.iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        // Get window size
        let size = window.inner_size();

        // Configure surface
        let config = SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };

        // Request device and queue
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
                    label: Some("Render Device"),
                    memory_hints: wgpu::MemoryHints::default(),
                },
                None,
            )
            .await
            .expect("Failed to create device");

        // Configure the surface
        surface.configure(&device, &config);

        Self {
            instance,
            surface,
            device,
            queue,
            config,
        }
    }

    /// Resize the surface to match the new window size
    /// 
    /// # Arguments
    /// * `new_width` - New width in pixels
    /// * `new_height` - New height in pixels
    pub fn resize(&mut self, new_width: u32, new_height: u32) {
        if new_width == 0 || new_height == 0 {
            return;
        }
        self.config.width = new_width;
        self.config.height = new_height;
        self.surface.configure(&self.device, &self.config);
    }

    /// Get the current surface texture for rendering
    pub fn get_current_texture(&self) -> Result<wgpu::SurfaceTexture, wgpu::SurfaceError> {
        self.surface.get_current_texture()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_context_exists() {
        // Just verify the struct compiles
        // Actual initialization requires an async runtime and window
        let _: Option<RenderContext> = None;
    }
}
