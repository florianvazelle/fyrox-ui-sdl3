pub mod platform;
pub mod renderer;
pub mod utils;

use crate::platform::Platform;
use crate::renderer::UiRenderer;
use fyrox_ui::UiUpdateSwitches;
use fyrox_ui::{UserInterface, core::algebra::Vector2, message::UiMessage};
use sdl3::gpu::*;

pub struct FyroxUiSdl {
    pub ui: UserInterface,
    pub renderer: UiRenderer,
    pub platform: Platform,

    pub width: f32,
    pub height: f32,
}

impl FyroxUiSdl {
    pub fn new(device: &sdl3::gpu::Device, window: &sdl3::video::Window) -> Self {
        let ui = UserInterface::new(Vector2::new(window.size().0 as f32, window.size().1 as f32));

        let renderer = UiRenderer::new(device, window);
        let platform = Platform::new();

        Self {
            ui,
            renderer,
            platform,
            width: window.size().0 as f32,
            height: window.size().1 as f32,
        }
    }

    pub fn handle_event<T>(&mut self, event: &sdl3::event::Event, mut event_callback: T)
    where
        T: FnMut(UiMessage),
    {
        self.platform.handle_event(&mut self.ui, event);

        while let Some(message) = self.ui.poll_message() {
            event_callback(message);
        }
    }

    /// Resize UI when window size changes.
    pub fn resize(&mut self, width: f32, height: f32) {
        self.width = width;
        self.height = height;
        self.ui.set_screen_size(Vector2::new(width, height));
    }

    /// Pump a frame of UI logic (animations, layout, message routing).
    pub fn update(&mut self, dt: f32) {
        self.ui.update(
            Vector2::new(self.width, self.height),
            dt,
            &UiUpdateSwitches { node_overrides: None },
        );
    }

    /// Send a generic UI message (you usually route input events to this).
    pub fn send_message(&mut self, message: UiMessage) {
        self.ui.send_message(message);
    }

    pub fn render(
        &mut self,
        device: &sdl3::gpu::Device,
        window: &sdl3::video::Window,
        command_buffer: &mut CommandBuffer,
        color_targets: &[ColorTargetInfo],
    ) -> Result<(), Box<dyn std::error::Error>> {
        let draw_ctx = self.ui.draw();
        self.renderer
            .render(device, window, command_buffer, color_targets, draw_ctx)
    }
}
