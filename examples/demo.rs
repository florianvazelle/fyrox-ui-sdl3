use fyrox_ui_sdl3::FyroxUiSdl;
use sdl3::{event::Event, gpu::*, pixels};

use fyrox_ui::Thickness;
use fyrox_ui::button::ButtonMessage;
use fyrox_ui::style::StyledProperty;
use fyrox_ui::tab_control::TabDefinition;
use fyrox_ui::{
    UserInterface,
    border::BorderBuilder,
    brush::Brush,
    button::ButtonBuilder,
    check_box::CheckBoxBuilder,
    core::color::Color,
    dropdown_list::DropdownListBuilder,
    expander::ExpanderBuilder,
    grid::{Column, GridBuilder, Row},
    image::ImageBuilder,
    list_view::ListViewBuilder,
    menu::{MenuBuilder, MenuItemBuilder},
    numeric::NumericUpDownBuilder,
    progress_bar::ProgressBarBuilder,
    range::RangeEditorBuilder,
    scroll_viewer::ScrollViewerBuilder,
    stack_panel::StackPanelBuilder,
    tab_control::TabControlBuilder,
    text::TextBuilder,
    text_box::TextBoxBuilder,
    tree::TreeBuilder,
    widget::WidgetBuilder,
};

pub struct DemoUi;

impl DemoUi {
    pub fn build(ui: &mut UserInterface) {
        let mut ctx = ui.build_ctx();

        // Root container
        StackPanelBuilder::new(
            WidgetBuilder::new()
                .with_background(StyledProperty::from(Brush::Solid(Color::opaque(30, 30, 30))))
                // Text
                .with_child(
                    TextBuilder::new(WidgetBuilder::new())
                        .with_text("Demo: Text")
                        .build(&mut ctx),
                )
                // Button
                .with_child(
                    ButtonBuilder::new(WidgetBuilder::new())
                        .with_text("Click Me")
                        .build(&mut ctx),
                )
                // CheckBox
                .with_child(
                    CheckBoxBuilder::new(WidgetBuilder::new())
                        // .with_text("Check Me")
                        .build(&mut ctx),
                )
                // TextBox
                .with_child(
                    TextBoxBuilder::new(WidgetBuilder::new())
                        .with_text("Editable text")
                        .build(&mut ctx),
                )
                // ProgressBar
                .with_child(
                    ProgressBarBuilder::new(WidgetBuilder::new())
                        // .with_value(0.5)
                        .build(&mut ctx),
                )
                // NumericUpDown
                .with_child(
                    NumericUpDownBuilder::new(WidgetBuilder::new())
                        .with_min_value(0.0)
                        .with_max_value(10.0)
                        .with_step(1.0)
                        .with_value(5.0)
                        .build(&mut ctx),
                )
                // RangeEditor
                .with_child(
                    RangeEditorBuilder::new(WidgetBuilder::new())
                        // .with_range(0.0..10.0)
                        .with_value(2.0..8.0)
                        .build(&mut ctx),
                )
                // ListView
                .with_child(
                    ListViewBuilder::new(WidgetBuilder::new())
                        .with_items(vec![
                            TextBuilder::new(WidgetBuilder::new())
                                .with_text("Item 1")
                                .build(&mut ctx),
                            TextBuilder::new(WidgetBuilder::new())
                                .with_text("Item 2")
                                .build(&mut ctx),
                        ])
                        .build(&mut ctx),
                )
                // DropdownList
                .with_child(
                    DropdownListBuilder::new(WidgetBuilder::new())
                        .with_items(vec![
                            TextBuilder::new(WidgetBuilder::new())
                                .with_text("Choice A")
                                .build(&mut ctx),
                            TextBuilder::new(WidgetBuilder::new())
                                .with_text("Choice B")
                                .build(&mut ctx),
                        ])
                        .build(&mut ctx),
                )
                // RectEditor
                // .with_child(
                //     RectEditorBuilder::new(WidgetBuilder::new()).build(&mut ctx),
                // )
                // Grid (2x2 example)
                .with_child(
                    GridBuilder::new(WidgetBuilder::new())
                        .add_column(Column::stretch())
                        .add_column(Column::stretch())
                        .add_row(Row::stretch())
                        .add_row(Row::stretch())
                        .build(&mut ctx),
                )
                // ScrollViewer
                .with_child(
                    ScrollViewerBuilder::new(WidgetBuilder::new())
                        .with_content(
                            TextBuilder::new(WidgetBuilder::new())
                                .with_text("Scrollable content")
                                .build(&mut ctx),
                        )
                        .build(&mut ctx),
                )
                // Expander
                .with_child(
                    ExpanderBuilder::new(WidgetBuilder::new())
                        .with_header(
                            TextBuilder::new(WidgetBuilder::new())
                                .with_text("Expander")
                                .build(&mut ctx),
                        )
                        .with_content(
                            TextBuilder::new(WidgetBuilder::new())
                                .with_text("Hidden content")
                                .build(&mut ctx),
                        )
                        .build(&mut ctx),
                )
                // TabControl
                .with_child(
                    TabControlBuilder::new(WidgetBuilder::new())
                        .with_tab(TabDefinition {
                            header: TextBuilder::new(WidgetBuilder::new())
                                .with_text("First")
                                .build(&mut ctx),
                            content: TextBuilder::new(WidgetBuilder::new())
                                .with_text("First tab's contents!")
                                .build(&mut ctx),
                            can_be_closed: true,
                            user_data: None,
                        })
                        .with_tab(TabDefinition {
                            header: TextBuilder::new(WidgetBuilder::new())
                                .with_text("Second")
                                .build(&mut ctx),
                            content: TextBuilder::new(WidgetBuilder::new())
                                .with_text("Second tab's contents!")
                                .build(&mut ctx),
                            can_be_closed: true,
                            user_data: None,
                        })
                        .build(&mut ctx),
                )
                // Tree
                .with_child(
                    TreeBuilder::new(WidgetBuilder::new())
                        .with_items(vec![
                            TextBuilder::new(WidgetBuilder::new())
                                .with_text("Node A")
                                .build(&mut ctx),
                            TextBuilder::new(WidgetBuilder::new())
                                .with_text("Node B")
                                .build(&mut ctx),
                        ])
                        .build(&mut ctx),
                )
                // Menu
                .with_child(
                    MenuBuilder::new(WidgetBuilder::new())
                        .with_items(vec![
                            MenuItemBuilder::new(WidgetBuilder::new())
                                // .with_content(TextBuilder::new(WidgetBuilder::new()).with_text("File").build(&mut ctx))
                                .build(&mut ctx),
                        ])
                        .build(&mut ctx),
                )
                // Image (placeholder)
                .with_child(ImageBuilder::new(WidgetBuilder::new()).build(&mut ctx))
                // Border
                .with_child(
                    BorderBuilder::new(WidgetBuilder::new())
                        .with_stroke_thickness(StyledProperty::from(Thickness::uniform(2.0)))
                        // .with_brush(Brush::Solid(Color::opaque(200, 0, 0)))
                        // .with_content(
                        //     TextBuilder::new(WidgetBuilder::new())
                        //         .with_text("Bordered Text")
                        //         .build(&mut ctx),
                        // )
                        .build(&mut ctx),
                ),
        )
        .build(&mut ctx);
    }
}

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
    DemoUi::build(&mut fyrox_ui.ui);

    // start main loop
    let mut event_pump = sdl.event_pump().unwrap();

    'main: loop {
        for event in event_pump.poll_iter() {
            // pass all events to imgui platform
            fyrox_ui.handle_event(&event, |message| {
                if let Some(ButtonMessage::Click) = message.data::<ButtonMessage>() {
                    println!("Button {:?} clicked!", message.destination());
                }
            });

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

            let _ = fyrox_ui.render(&device, &window, &mut command_buffer, &color_targets);

            command_buffer.submit()?;
        } else {
            println!("Swapchain unavailable, cancel work");
            command_buffer.cancel();
        }
    }

    Ok(())
}
