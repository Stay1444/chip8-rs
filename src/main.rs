mod chip8;

use macroquad::prelude::*;

#[macroquad::main("BasicShapes")]
async fn main() {

    let mut vm = chip8::VM::new();

    vm.load_program_from_file(&std::path::Path::new("./roms/test_opcode.ch8"), 200);

    vm.program_counter = 200;

    loop {
        clear_background(LIGHTGRAY);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);
        let frametime = get_frame_time();
        draw_text(&format!("Hello {}", frametime).as_str(), 20.0, 20.0, 30.0, DARKGRAY);

        vm.tick().unwrap();
        
        next_frame().await
    }
}