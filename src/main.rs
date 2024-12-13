use ballsterm::window::window_begin;
use ballsterm::cursor::Cursor;
use ballsterm::styled_text::{Style, StyledText};
use std::thread;
use std::sync::mpsc;
use std::time::Duration;
use sdl2::pixels::Color;

// Example usage
fn main() {
    let (sender, receiver) = mpsc::channel();

    // Spawn the SDL2 window in a separate thread
    let handle = thread::spawn(move || {
        window_begin(receiver);
    });

    // Send styled messages to the window thread
    sender.send(StyledText {
        fg_color: Color::RGB(255, 255, 255),
        bg_color: Color::RGB(0, 0, 0),
        text: "Welcome to Rust SDL2!".to_string(),
        style: Style::Bold,
    }).unwrap();
    
    thread::sleep(Duration::from_secs(2));

    sender.send(StyledText {
        fg_color: Color::RGB(0, 255, 0),
        bg_color: Color::RGB(0, 0, 0),
        text: "Hello from the main thread!".to_string(),
        style: Style::Italic,
    }).unwrap();

    // Allow some time before closing
    thread::sleep(Duration::from_secs(5));
    handle.join().unwrap();
}

