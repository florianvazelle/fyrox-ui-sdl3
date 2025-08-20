//! Minimal renderer backend that converts Fyrox-UI `DrawingContext` to SDL3 GPU draw calls.

use crate::utils::create_texture;
use fyrox_ui::draw::Command;
use fyrox_ui::draw::CommandTexture;
use fyrox_ui::draw::DrawingContext;
use sdl3::gpu::*;
use sdl3::video::Window;
use sdl3_sys::gpu::SDL_GPUViewport;
use std::mem::offset_of;

/// GPU resources for the UI render pass.
pub struct UiRenderer {
    pub pipeline: GraphicsPipeline,
    sampler_linear: Sampler,
    // CPU → GPU font atlas pages (one texture per page).
    font_pages: Vec<Texture<'static>>,
    // Cached white 1×1 for fallback.
    white_tex: Texture<'static>,
}

impl UiRenderer {
    pub fn new(device: &Device, window: &Window) -> Self {
        let format = device.get_swapchain_texture_format(window);

        let vert = device
            .create_shader()
            .with_code(
                ShaderFormat::SPIRV,
                include_bytes!(concat!(env!("OUT_DIR"), "/ui.vert.spv")),
                ShaderStage::Vertex,
            )
            .with_uniform_buffers(1)
            .with_entrypoint(c"main")
            .build()
            .unwrap();

        let frag = device
            .create_shader()
            .with_code(
                ShaderFormat::SPIRV,
                include_bytes!(concat!(env!("OUT_DIR"), "/ui.frag.spv")),
                ShaderStage::Fragment,
            )
            .with_samplers(1)
            .with_uniform_buffers(1)
            .with_entrypoint(c"main")
            .build()
            .unwrap();

        let pipeline = device
            .create_graphics_pipeline()
            .with_vertex_shader(&vert)
            .with_fragment_shader(&frag)
            .with_vertex_input_state(
                VertexInputState::new()
                    .with_vertex_buffer_descriptions(&[VertexBufferDescription::new()
                        .with_slot(0)
                        .with_pitch(std::mem::size_of::<fyrox_ui::draw::Vertex>() as u32)
                        .with_input_rate(VertexInputRate::Vertex)
                        .with_instance_step_rate(0)])
                    .with_vertex_attributes(&[
                        VertexAttribute::new()
                            .with_format(VertexElementFormat::Float2)
                            .with_location(0)
                            .with_buffer_slot(0)
                            .with_offset(offset_of!(fyrox_ui::draw::Vertex, pos) as u32),
                        VertexAttribute::new()
                            .with_format(VertexElementFormat::Float2)
                            .with_location(1)
                            .with_buffer_slot(0)
                            .with_offset(offset_of!(fyrox_ui::draw::Vertex, tex_coord) as u32),
                        VertexAttribute::new()
                            .with_format(VertexElementFormat::Ubyte4Norm)
                            .with_location(2)
                            .with_buffer_slot(0)
                            .with_offset(offset_of!(fyrox_ui::draw::Vertex, color) as u32),
                    ]),
            )
            .with_rasterizer_state(
                RasterizerState::new()
                    .with_fill_mode(FillMode::Fill)
                    .with_front_face(FrontFace::Clockwise), // Disable culling for UI geometry
            )
            .with_primitive_type(PrimitiveType::TriangleList)
            .with_target_info(
                GraphicsPipelineTargetInfo::new().with_color_target_descriptions(&[ColorTargetDescription::new()
                    .with_format(format)
                    .with_blend_state(
                        ColorTargetBlendState::new()
                            .with_color_blend_op(BlendOp::Add)
                            .with_src_color_blendfactor(BlendFactor::SrcAlpha)
                            .with_dst_color_blendfactor(BlendFactor::OneMinusSrcAlpha)
                            .with_alpha_blend_op(BlendOp::Add)
                            .with_src_alpha_blendfactor(BlendFactor::One)
                            .with_dst_alpha_blendfactor(BlendFactor::OneMinusSrcAlpha)
                            .with_enable_blend(true),
                    )]),
            )
            .build()
            .unwrap();

        let sampler_linear = device
            .create_sampler(
                SamplerCreateInfo::new()
                    .with_min_filter(Filter::Linear)
                    .with_mag_filter(Filter::Linear)
                    .with_mipmap_mode(SamplerMipmapMode::Linear)
                    .with_address_mode_u(SamplerAddressMode::ClampToEdge)
                    .with_address_mode_v(SamplerAddressMode::ClampToEdge)
                    .with_address_mode_w(SamplerAddressMode::ClampToEdge),
            )
            .unwrap();

        // 1×1 white fallback
        let white_tex = {
            let copy_cmds = device.acquire_command_buffer().unwrap();
            let copy_pass = device.begin_copy_pass(&copy_cmds).unwrap();
            let tex = create_texture(device, &copy_pass, &[255], 1, 1).unwrap();
            device.end_copy_pass(copy_pass);
            copy_cmds.submit().unwrap();
            tex
        };

        Self {
            pipeline,
            sampler_linear,
            font_pages: Vec::new(),
            white_tex,
        }
    }

    /// Render Fyrox-UI `DrawingContext` to the current color target(s).
    ///
    /// `color_targets` must be the same swapchain target you used for the rest of your frame.
    pub fn render(
        &self,
        device: &sdl3::gpu::Device,
        window: &sdl3::video::Window,
        command_buffer: &mut CommandBuffer,
        color_targets: &[ColorTargetInfo],
        drawing: &DrawingContext,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (width, height) = window.size();
        let vertices_len = drawing.get_vertices().len();

        // Skip rendering if there's nothing to draw
        if width == 0 || height == 0 || vertices_len == 0 {
            return Ok(());
        }

        let (vbuf, vidx) = {
            let copy_commands = device.acquire_command_buffer()?;
            let transfer_buffer = device
                .create_transfer_buffer()
                .with_size(std::mem::size_of_val(drawing.get_vertices()) as u32)
                .with_usage(sdl3::gpu::TransferBufferUsage::UPLOAD)
                .build()?;
            let copy_pass = device.begin_copy_pass(&copy_commands)?;
            let vbuf = crate::utils::create_buffer_with_data(
                device,
                &transfer_buffer,
                &copy_pass,
                BufferUsageFlags::VERTEX,
                drawing.get_vertices(),
            )?;
            let indices: Vec<u32> = drawing
                .get_triangles()
                .iter()
                .flat_map(|tri| tri.indices()) // flattens &[u32] into u32
                .copied() // &[u32] → u32
                .collect();
            let vidx = crate::utils::create_buffer_with_data(
                device,
                &transfer_buffer,
                &copy_pass,
                BufferUsageFlags::INDEX,
                &indices,
            )?;
            device.end_copy_pass(copy_pass);
            copy_commands.submit()?;
            (vbuf, vidx)
        };

        let render_pass = device.begin_render_pass(command_buffer, color_targets, None)?;
        render_pass.bind_graphics_pipeline(&self.pipeline);

        // Set viewport and projection matrix
        device.set_viewport(
            &render_pass,
            SDL_GPUViewport {
                x: 0.0,
                y: 0.0,
                w: width as f32,
                h: height as f32,
                min_depth: 0.0,
                max_depth: 1.0,
            },
        );

        // Push orthographic projection matrix
        let matrix = [
            [2.0 / width as f32, 0.0, 0.0, 0.0],
            [0.0, 2.0 / -(height as f32), 0.0, 0.0],
            [0.0, 0.0, -1.0, 0.0],
            [-1.0, 1.0, 0.0, 1.0],
        ];
        command_buffer.push_vertex_uniform_data(0, &matrix);

        render_pass.bind_vertex_buffers(0, &[BufferBinding::new().with_buffer(&vbuf).with_offset(0)]);
        render_pass.bind_index_buffer(
            &BufferBinding::new().with_buffer(&vidx).with_offset(0),
            IndexElementSize::_32BIT,
        );

        // Draw every command
        for cmd_ui in drawing.get_commands() {
            self.draw_command(device, &render_pass, command_buffer, cmd_ui, (width, height))?;
        }

        // render_pass.draw_primitives(vertices_len, 1, 0, 0);

        device.end_render_pass(render_pass);

        Ok(())
    }

    /// After `ui.draw(&mut drawing)`, call this to ensure glyph pages are on the GPU.
    // pub fn sync_font_pages(
    //     &mut self,
    //     device: &Device,
    //     font: &FontResource,
    // ) -> Result<(), Box<dyn std::error::Error>> {
    //     // We rebuild the list each frame for simplicity. You could keep a map (font_uuid, height, page_idx) → Texture.
    //     self.font_pages.clear();

    //     // We need mutable access to clear `modified` flags after upload.
    //     if let Some(mut font_data) = font.state().data() {
    //         for atlas in font_data.atlases.values_mut() {
    //             for page in &mut atlas.pages {
    //                 // Create/update a texture for the page if needed.
    //                 if page.texture.is_none() {
    //                     let w = font_data.page_size as u32;
    //                     let h = font_data.page_size as u32;

    //                     // Expand font's 8-bit alpha page → RGBA (white * alpha)
    //                     let mut rgba = Vec::with_capacity((w * h * 4) as usize);
    //                     for &a in &page.pixels {
    //                         rgba.extend_from_slice(&[255, 255, 255, a]);
    //                     }

    //                     let tex = {
    //                         let copy_cmds = device.acquire_command_buffer().unwrap();
    //                         let copy_pass = device.begin_copy_pass(&copy_cmds).unwrap();
    //                         let tex = create_texture(device, &copy_pass, &rgba, w, h).unwrap();
    //                         device.end_copy_pass(copy_pass);
    //                         copy_cmds.submit().unwrap();
    //                         tex
    //                     };

    //                     self.font_pages.push(tex);

    //                     page.modified = false;
    //                 }
    //             }
    //         }
    //     }

    //     Ok(())
    // }

    fn draw_command(
        &self,
        device: &Device,
        pass: &RenderPass,
        cmd: &mut CommandBuffer,
        cmd_ui: &Command,
        (fb_w, fb_h): (u32, u32),
    ) -> Result<(), Box<dyn std::error::Error>> {
        // --- Scissor from clip bounds (simple clipping path)
        let mut clip = cmd_ui.clip_bounds;
        clip.position.x = clip.position.x.floor();
        clip.position.y = clip.position.y.floor();
        clip.size.x = clip.size.x.ceil();
        clip.size.y = clip.size.y.ceil();

        let sc_x = clip.position.x as i32;
        let sc_y = clip.position.y as i32;
        let sc_w = clip.size.x.max(0.0) as u32;
        let sc_h = clip.size.y.max(0.0) as u32;

        unsafe {
            // SDL uses top-left origin for GPU scissor.
            // If yours is bottom-left, flip Y accordingly. With SDL3 it’s top-left.
            let rect = sdl3::rect::Rect::new(sc_x, sc_y, sc_w, sc_h);
            sdl3_sys::gpu::SDL_SetGPUScissor(pass.raw(), rect.raw());
        }

        // --- Pack fyrox_widgetData
        //
        // This mirrors what Fyrox writes:
        //  - worldViewProjection (we put this in vertex set=0 already)
        //  - brush data: solid/gradient colors+stops, gradient points, resolution, bounds, flags, opacity
        // To keep it short, we do the minimal subset most UI uses: solid brush + is_font flag + opacity.
        // If you need gradients, extend this to match your fragment shader’s UBO layout.

        #[repr(C)]
        struct WidgetData {
            // vec4 solid_color
            solid: [f32; 4],
            // vec2 resolution
            resolution: [f32; 2],
            // vec2 bounds_min
            bounds_min: [f32; 2],
            // vec2 bounds_max
            bounds_max: [f32; 2],
            // float is_font_texture (0.0/1.0)
            is_font: f32,
            // float opacity
            opacity: f32,
            // padding to 16B multiples
            _pad: [f32; 2],
        }

        let solid = match cmd_ui.brush {
            fyrox_ui::brush::Brush::Solid(c) => [
                c.r as f32 / 255.0,
                c.g as f32 / 255.0,
                c.b as f32 / 255.0,
                c.a as f32 / 255.0,
            ],
            _ => [1.0, 1.0, 1.0, 1.0], // extend for gradients if needed
        };

        let bounds_min = [cmd_ui.bounds.position.x, cmd_ui.bounds.position.y];
        let br = cmd_ui.bounds.right_bottom_corner();
        let bounds_max = [br.x, br.y];

        let is_font = matches!(cmd_ui.texture, CommandTexture::Font { .. }) as i32 as f32;

        let widget = WidgetData {
            solid,
            resolution: [fb_w as f32, fb_h as f32],
            bounds_min,
            bounds_max,
            is_font,
            opacity: cmd_ui.opacity,
            _pad: [0.0, 0.0],
        };

        cmd.push_fragment_uniform_data(0, &widget);

        // --- Bind texture
        let (tex, samp) = match &cmd_ui.texture {
            CommandTexture::Font {
                font,
                page_index,
                height,
            } => {
                let mut texture = None;

                if let Some(font) = font.state().data() {
                    let page_size = font.page_size() as u32;
                    if let Some(page) = font
                        .atlases
                        .get_mut(height)
                        .and_then(|atlas| atlas.pages.get_mut(*page_index))
                    {
                        // if page.texture.is_none() || page.modified {
                        texture = {
                            let copy_cmds = device.acquire_command_buffer().unwrap();
                            let copy_pass = device.begin_copy_pass(&copy_cmds).unwrap();
                            let tex = create_texture(device, &copy_pass, &page.pixels, page_size, page_size).unwrap();
                            device.end_copy_pass(copy_pass);
                            copy_cmds.submit().unwrap();
                            Some(tex)
                        };
                        // }
                    }
                }

                (&texture.unwrap_or(self.white_tex.clone()), &self.sampler_linear)

                // Map page index to an uploaded SDL texture.
                // let tex = self
                //     .font_pages
                //     .get(*page_index)
                //     .unwrap_or(&self.white_tex);
                // (tex, &self.sampler_linear)
            }
            CommandTexture::Texture(tex_res) => {
                // If you have your own texture cache for UI, plug it here.
                // For now, use white fallback.
                (&self.white_tex, &self.sampler_linear)
            }
            _ => (&self.white_tex, &self.sampler_linear),
        };

        let binding = TextureSamplerBinding::new().with_texture(tex).with_sampler(samp);
        pass.bind_fragment_samplers(0, &[binding]);

        // --- Draw triangles range
        let start = cmd_ui.triangles.start as u32;
        let count = (cmd_ui.triangles.end - cmd_ui.triangles.start) as u32;

        pass.draw_indexed_primitives(count * 3, 1, 0, (start * 3) as i32, 0);

        Ok(())
    }
}
