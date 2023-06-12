use std::env;
use std::fs::*;
use std::io::Read;

use simple::*;

const EMPTY_BYTE: u8 = 0xFF;
const BLOCK_SIZE: usize = 32;

const WIDTH: u32 = 128;
const HEIGHT: u32 = 128;
const SCALE: u32 = 8;

const SCROLL_SPEED: u32 = 4;

const EMPTY_COLOUR: [u8; 3] = [0, 0, 0];
const FILLED_COLOUR: [u8; 3] = [255, 255, 255];

fn main() {
    
    let path = env::args().nth(1).unwrap();

    let file_size = metadata(path.clone()).unwrap().len() as usize;

    let mut file = File::open(path).unwrap();

    let mut block = [0; BLOCK_SIZE];

    let img: Vec<bool> = (0..file_size / BLOCK_SIZE).map(|_block| {
        file.read_exact(&mut block).unwrap();
        block.iter().all(
            |byte| *byte == EMPTY_BYTE
        )
    }).collect();

    let mut app = simple::Window::new("yeah", (WIDTH * SCALE) as u16, (HEIGHT * SCALE) as u16);

    let mut scroll: u32 = 0;

    while app.next_frame() {

        if app.is_key_down(Key::Down) {
            scroll = (scroll + SCROLL_SPEED).min(img.len() as u32 / WIDTH - HEIGHT);

        } else if app.is_key_down(Key::Up) {
            scroll = scroll.saturating_sub(SCROLL_SPEED);
        
        } else {
            while app.has_event() {

                match app.next_event() {
                    Event::Keyboard {is_down: true, key: Key::Left} => {

                        let top_left = img[(scroll * WIDTH) as usize];

                        for idx in (0..scroll * WIDTH).rev() {

                            if top_left != img[idx as usize] {
                                scroll = idx / WIDTH;
                                break;
                            }
                        }
                    },
                    Event::Keyboard {is_down: true, key: Key::Right} => {

                        let top_left = img[(scroll * WIDTH) as usize];

                        for idx in ((scroll + 1) * WIDTH)..(img.len() as u32) {

                            if top_left != img[idx as usize] {
                                scroll = idx / WIDTH;
                                break;
                            }
                        }
                    },
                    _ => {}
                }
            }
        }

        app.set_color(EMPTY_COLOUR[0], EMPTY_COLOUR[1], EMPTY_COLOUR[2], 255);
        app.fill_rect(Rect::new(0, 0, WIDTH * SCALE, HEIGHT * SCALE));

        app.set_color(FILLED_COLOUR[0], FILLED_COLOUR[1], FILLED_COLOUR[2], 255);

        for (n, block) in img.iter().enumerate() {

            let y = (n as u32 / WIDTH).wrapping_sub(scroll);

            if y > HEIGHT {
                continue;
            }

            if *block {

                app.fill_rect(Rect::new(
                    ((n as u32 % WIDTH) * SCALE) as i32,
                    (y * SCALE) as i32,
                    SCALE,
                    SCALE
                ));
            }
        }

        let (mouse_x, mouse_y) = app.mouse_position();

        if mouse_x < 0 || mouse_y < 0 {
            continue;
        }

        let mouse_x_pos = mouse_x as u32 / SCALE;
        let mouse_y_pos = mouse_y as u32 / SCALE;

        if mouse_x_pos >= WIDTH || mouse_y_pos >= HEIGHT {
            continue;
        }

        app.print(format!("{:#010x}", ((mouse_x_pos + WIDTH * (mouse_y_pos + scroll)) * BLOCK_SIZE as u32)).as_str(), 0, 0);

    }
}