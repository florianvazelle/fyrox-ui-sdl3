<div align="center">

# `fyrox-ui-sdl3`

**Rust library that integrates FyroxUI with SDL3.**

[![Crates.io](https://img.shields.io/crates/v/fyrox-ui-sdl3.svg)](https://crates.io/crates/fyrox-ui-sdl3)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/florianvazelle/fyrox-ui-sdl3/nix.yml)
[![API Docs](https://docs.rs/fyrox-ui-sdl3/badge.svg)](https://docs.rs/fyrox-ui-sdl3)
[![dependency status](https://deps.rs/repo/github/florianvazelle/fyrox-ui-sdl3/status.svg)](https://deps.rs/repo/github/florianvazelle/fyrox-ui-sdl3)
![GitHub License](https://img.shields.io/github/license/florianvazelle/fyrox-ui-sdl3)

</div>

## Features

This crate provides an SDL3 backend platform and renderer for fyrox-ui.

- The backend platform handles window/input device events,
- The rendering backend use the SDL3 GPU API, and can be use as a render pass.

## Full demo

```rust
use fyrox_ui_sdl3::FyroxUiSdl;
use sdl3::{event::Event, gpu::*, pixels};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // initialize SDL and its video subsystem
    let sdl = sdl3::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    // create a new window
    let window = video_subsystem
        .window("Hello fyrox UI!", 1280, 720)
        .position_centered()
        .resizable()
        .build()
        .unwrap();

    let device = Device::new(ShaderFormat::SPIRV, true)
        .unwrap()
        .with_window(&window)
        .unwrap();

    // create platform and renderer
    let mut fyrox_ui = FyroxUiSdl::new(&device, &window);

    // start main loop
    let mut event_pump = sdl.event_pump().unwrap();

    'main: loop {
        for event in event_pump.poll_iter() {
            // pass all events to imgui platform
            fyrox_ui.handle_event(&event);

            if let Event::Quit { .. } = event {
                break 'main;
            }
        }

        fyrox_ui.update(1.0 / 60.0);

        let mut command_buffer = device.acquire_command_buffer()?;

        if let Ok(swapchain) = command_buffer.wait_and_acquire_swapchain_texture(&window) {
            let color_targets = [ColorTargetInfo::default()
                .with_texture(&swapchain)
                .with_load_op(LoadOp::CLEAR)
                .with_store_op(StoreOp::STORE)
                .with_clear_color(pixels::Color::RGB(128, 128, 128))];

            fyrox_ui.render(&device, &window, &event_pump, &mut command_buffer, &color_targets);

            command_buffer.submit()?;
        } else {
            println!("Swapchain unavailable, cancel work");
            command_buffer.cancel();
        }
    }

    Ok(())
}
```

## Development

The project use [`just`](https://just.systems/man/en/) as command runner.

To check all available recipes, run:
```
just
```

To run formatters:
```
just fmt
```
