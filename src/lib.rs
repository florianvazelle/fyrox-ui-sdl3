pub mod platform;
pub mod renderer;
pub mod utils;

use crate::platform::Platform;
use crate::renderer::UiRenderer;
use fyrox_ui::UiUpdateSwitches;
use fyrox_ui::brush::Brush;
use fyrox_ui::{
    UserInterface, button::ButtonBuilder, core::algebra::Vector2, grid::GridBuilder, message::UiMessage,
    text::TextBuilder,
};
use fyrox_ui::{core::color::Color, widget::WidgetBuilder};
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
        let mut ui = UserInterface::new(Vector2::new(window.size().0 as f32, window.size().1 as f32));
        Self::build_example_ui(&mut ui);

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

    /// Convenience to add some example controls. Returns root handle of the added panel.
    pub fn build_example_ui(ui: &mut UserInterface) {
        ButtonBuilder::new(
            WidgetBuilder::new()
                .with_width(160.0)
                .with_height(32.0)
                .with_desired_position(Vector2::new(100.0, 200.0)),
        )
        .with_text("Click Me!")
        .build(&mut ui.build_ctx());

        TextBuilder::new(WidgetBuilder::new().with_desired_position(Vector2::new(300.0, 300.0)))
            .with_text("This is some text.")
            .build(&mut ui.build_ctx());

        GridBuilder::new(
            WidgetBuilder::new()
                .with_background(fyrox_ui::style::StyledProperty::new(
                    Brush::Solid(Color::opaque(30, 30, 30)),
                    "tot",
                ))
                .with_width(300.0)
                .with_height(120.0)
                .with_child(
                    TextBuilder::new(
                        WidgetBuilder::new()
                            .with_vertical_alignment(fyrox_ui::VerticalAlignment::Center)
                            .with_horizontal_alignment(fyrox_ui::HorizontalAlignment::Center),
                    )
                    .with_text("Hello Fyrox-UI")
                    .with_font_size(fyrox_ui::style::StyledProperty::new(30.0, "size"))
                    .build(&mut ui.build_ctx()),
                )
                .with_child(
                    ButtonBuilder::new(
                        WidgetBuilder::new()
                            .with_background(fyrox_ui::style::StyledProperty::new(
                                Brush::Solid(Color::opaque(30, 30, 30)),
                                "tot",
                            ))
                            .with_width(300.0)
                            .with_height(120.0),
                    )
                    .with_text("Start")
                    .build(&mut ui.build_ctx()),
                ),
        )
        .add_row(fyrox_ui::grid::Row::stretch())
        .add_column(fyrox_ui::grid::Column::stretch())
        .build(&mut ui.build_ctx());
    }

    pub fn handle_event(&mut self, event: &sdl3::event::Event) {
        self.platform.handle_event(&mut self.ui, event);
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
        event_pump: &sdl3::EventPump,
        command_buffer: &mut CommandBuffer,
        color_targets: &[ColorTargetInfo],
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.platform.prepare_frame(event_pump, &mut self.ui);

        let draw_ctx = self.ui.draw();
        self.renderer
            .render(device, window, command_buffer, color_targets, draw_ctx)
    }
}
