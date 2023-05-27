mod chip8;

use std::{sync::{Mutex, Arc}, time::Duration, env, process::ExitCode};

use macroquad::prelude::*;

const SCREEN_MARGIN: usize = 15;
const PIXEL_MARGIN: usize = 0;

#[macroquad::main("BasicShapes")]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: chip8 ./path/to/rom");
        return;
    }

    let vm = Arc::new(Mutex::new(chip8::VM::new()));

    vm.lock().unwrap().mem_copy(&chip8::FONT_DATA, 0);

    vm.lock().unwrap().load_program_from_file(&std::path::Path::new(&args[1]), 0x200);

    vm.lock().unwrap().program_counter = 0x200;


    {
        let vm_clone = vm.clone();
        std::thread::spawn(move || {
            tick(vm_clone);
        });
    }


    loop {
        {
            let mut vm_lock = vm.lock().unwrap();
            clear_background(BLACK);
    
            let display_width: f32;
            let display_height: f32;
    
            let aspect_ratio: f32 = chip8::DISPLAY_HEIGHT as f32 / chip8::DISPLAY_HEIGHT as f32;
    
            if screen_width() as f32 / screen_height() as f32 > aspect_ratio {
                display_width = screen_width() as f32 * aspect_ratio;
                display_height = screen_height() as f32;
            } else {
                display_width = screen_width() as f32;
                display_height = screen_width() as f32 / aspect_ratio;
            }
    
            let left_margin = (screen_width() - display_width) / 2.0f32 - SCREEN_MARGIN as f32;
            let top_margin = (screen_height() - display_height) / 2.0f32 - SCREEN_MARGIN as f32;
    
            for y in 0..chip8::DISPLAY_HEIGHT {
                for x in 0..chip8::DISPLAY_WIDTH {
                    let color: Color;
                    if vm_lock.display.get(x, y) {
                        color = color_u8!(200,200,200,255);
                    } else {
                        color = color_u8!(20,20,20,255);
                    }
    
                    let pixel_x = left_margin + SCREEN_MARGIN as f32 + PIXEL_MARGIN as f32 + (display_width / chip8::DISPLAY_WIDTH as f32) * x as f32;
                    let pixel_y = top_margin + SCREEN_MARGIN as f32 + PIXEL_MARGIN as f32 + (display_height / chip8::DISPLAY_HEIGHT as f32) * y as f32;
                    let pixel_w = display_width / chip8::DISPLAY_WIDTH as f32 - 2f32 * PIXEL_MARGIN as f32;
                    let pixel_h = display_height / chip8::DISPLAY_HEIGHT as f32 - 2f32 * PIXEL_MARGIN as f32;
                    draw_rectangle(pixel_x, pixel_y, pixel_w, pixel_h, color)
                }
            }
            vm_lock.tick().unwrap();

            vm_lock.keyboard.keys[0x1] = is_key_down(KeyCode::Key1);
            vm_lock.keyboard.keys[0x2] = is_key_down(KeyCode::Key2);
            vm_lock.keyboard.keys[0x3] = is_key_down(KeyCode::Key3);
            vm_lock.keyboard.keys[0xC] = is_key_down(KeyCode::Key4);
            vm_lock.keyboard.keys[0x4] = is_key_down(KeyCode::Q);
            vm_lock.keyboard.keys[0x5] = is_key_down(KeyCode::W);
            vm_lock.keyboard.keys[0x6] = is_key_down(KeyCode::E);
            vm_lock.keyboard.keys[0xD] = is_key_down(KeyCode::R);
            vm_lock.keyboard.keys[0x7] = is_key_down(KeyCode::A);
            vm_lock.keyboard.keys[0x8] = is_key_down(KeyCode::S);
            vm_lock.keyboard.keys[0x9] = is_key_down(KeyCode::D);
            vm_lock.keyboard.keys[0xE] = is_key_down(KeyCode::F);
            vm_lock.keyboard.keys[0xA] = is_key_down(KeyCode::Z);
            vm_lock.keyboard.keys[0x0] = is_key_down(KeyCode::X);
            vm_lock.keyboard.keys[0xB] = is_key_down(KeyCode::C);
            vm_lock.keyboard.keys[0xF] = is_key_down(KeyCode::V);

            if vm_lock.delay_timer > 0 {
                vm_lock.delay_timer -= 1;
            }

            if vm_lock.sound_timer > 0 {
                vm_lock.sound_timer -= 1;
            }
        }
        next_frame().await
    }

    fn tick(vm: Arc<Mutex<chip8::VM>>) {
        loop {
            std::thread::sleep(Duration::from_millis(2));
            vm.lock().unwrap().tick().unwrap();
        }
    }
}