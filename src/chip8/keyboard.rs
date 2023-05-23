pub struct Keyboard {
    pub keys: [bool; 16]
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard {
            keys: [false; 16]
        }
    }
}