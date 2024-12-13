use sdl2::pixels::Color;

pub enum Style {
    Bold,
    Dim,
    Italic,
    Underline,
    Blink,
    Invert,
    Hidden,
    Strikethrough,
}

pub struct StyledText {
    pub fg_color: Color,
    pub text: String,
    pub style: Style,
    pub bg_color: Color,
}
