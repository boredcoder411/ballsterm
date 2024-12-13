use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::render::Texture;
use sdl2::ttf::FontStyle;
use std::sync::mpsc;
use std::time::Duration;
use crate::styled_text::{Style, StyledText};

pub fn window_begin(receiver: mpsc::Receiver<StyledText>) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let ttf_context = sdl2::ttf::init().unwrap(); // Initialize the TTF context

    let window = video_subsystem.window("rust-sdl2 demo", 800, 600)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().build().unwrap();
    let texture_creator = canvas.texture_creator();

    // Load a TTF font
    let font_path = "nerd_font.ttf";
    let mut font = ttf_context.load_font(font_path, 32).unwrap(); // Font size 32

    let mut texture: Option<Texture> = None;
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;

    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();

        // Check for new messages and update the texture if needed
        if let Ok(new_styled_text) = receiver.try_recv() {
            // Apply styles
            font.set_style(match new_styled_text.style {
                Style::Bold => FontStyle::BOLD,
                Style::Italic => FontStyle::ITALIC,
                Style::Underline => FontStyle::UNDERLINE,
                Style::Strikethrough => FontStyle::STRIKETHROUGH,
                _ => FontStyle::NORMAL,
            });

            let surface = font.render(&new_styled_text.text)
                .blended(new_styled_text.fg_color) // Use the specified foreground color
                .unwrap();
            texture = Some(texture_creator.create_texture_from_surface(&surface).unwrap());

            // Set background color (clearing canvas will use it)
            canvas.set_draw_color(new_styled_text.bg_color);
        }

        // Draw the current text texture if available
        if let Some(ref text_texture) = texture {
            let text_width = 400; // Hardcoded width for simplicity; adjust as needed
            let text_height = 32; // Font size
            let text_rect = Rect::new((800 - text_width as i32) / 2, (600 - text_height as i32) / 2, text_width, text_height);
            canvas.copy(text_texture, None, text_rect).unwrap();
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                },
                _ => {}
            }
        }

        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

