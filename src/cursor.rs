pub struct Cursor {
    pub x: i32,
    pub y: i32,
}

impl Cursor {
    pub fn new() -> Cursor {
        Cursor {
            x: 0,
            y: 0,
        }
    }
}
