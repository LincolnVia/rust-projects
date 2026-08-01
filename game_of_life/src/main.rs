use minifb::{Error::WindowCreate, *};

const WINDOW_HEIGHT: u32 = 800;
const WINDOW_WIDTH: u32 = 800;
const BUFFER_WIDTH: usize = 256;
const BUFFER_HEIGHT: usize = 256;

struct Vec2 {
    x: i32,
    y: i32,
}

fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}

fn vec2_to_buffer(pos: Vec2) -> u32 {
    (pos.y * BUFFER_WIDTH as i32 + pos.x) as u32
}
fn buffer_to_vec2(index: u32) -> Vec2 {
    Vec2 {
        x: (index % BUFFER_WIDTH as u32) as i32,
        y: (index / BUFFER_HEIGHT as u32) as i32,
    }
}
fn clear_screen(buffer: &mut [u32]) {
    for cell in buffer.iter_mut().enumerate() {
        *cell.1 = 0;
    }
}

fn get_cell_status(buffer: &[u32], pos: &Vec2) -> u32 {
    let index: usize = (pos.y * BUFFER_WIDTH as i32 + pos.x) as usize;
    if index > buffer.len() - 1 || index <= 0 {
        return 0;
    }

    if buffer[index] > 0 { 1 } else { 0 }
}

fn get_neighbor_count(buffer: &[u32], center: &Vec2) -> u32 {
    let mut count = 0;

    for x in -1..=1 {
        for y in -1..=1 {
            if x != 0 || y != 0 {
                count += get_cell_status(
                    &buffer,
                    &Vec2 {
                        x: center.x + x,
                        y: center.y + y,
                    },
                );
            }
        }
    }
    count
}

fn update_cell_state(buffer: &Vec<u32>, pos: Vec2) -> bool {
    let alive_neighbors: u32 = get_neighbor_count(buffer, &pos);

    let mut alive: bool = false;
    if get_cell_status(buffer, &pos) > 0 {
        alive = true
    }

    match (alive_neighbors, alive) {
        (2 | 3, true) => alive = true,
        (3, false) => alive = true,
        _ => alive = false,
    }

    alive
}

fn setup_board(buffer: &mut Vec<u32>) {
    for i in buffer.iter_mut() {
        if rand::random_range(0..=24) >= 21 {
            *i = from_u8_rgb(0, 200, 100);
        } else {
            *i = from_u8_rgb(0, 0, 0)
        }
    }
}

fn update_board(back_buffer: &mut Vec<u32>, front_buffer: &mut Vec<u32>) {
    for index in 0..back_buffer.len() {
        let position = buffer_to_vec2(index as u32);
        let status = update_cell_state(front_buffer, position);

        if status {
            let r: u8 = rand::random_range(0..=255);
            let g: u8 = rand::random_range(0..=255);
            let b: u8 = rand::random_range(0..=255);

            back_buffer[index] = from_u8_rgb(r, g, b);
        }
    }
}

fn main() {
    let mut window = Window::new(
        "Game Of Life",
        WINDOW_WIDTH as usize,
        WINDOW_HEIGHT as usize,
        WindowOptions {
            resize: true,
            ..WindowOptions::default()
        },
    )
    .expect("Unable to open Window");
    window.set_background_color(0, 0, 0);
    window.set_target_fps(60);

    let black = from_u8_rgb(0, 0, 0);
    let white = from_u8_rgb(255, 255, 255);
    let mut back_buffer: Vec<u32> = vec![black; BUFFER_WIDTH * BUFFER_HEIGHT];
    let mut front_buffer: Vec<u32> = vec![black; BUFFER_WIDTH * BUFFER_HEIGHT];

    let buffer_size = BUFFER_WIDTH * BUFFER_HEIGHT;

    setup_board(&mut front_buffer);

    while window.is_open() {
        if window.is_key_pressed(Key::R, KeyRepeat::No) {
            setup_board(&mut front_buffer);
        }

        update_board(&mut back_buffer, &mut front_buffer);

        std::mem::swap(&mut front_buffer, &mut back_buffer);
        clear_screen(&mut back_buffer);
        window.update_with_buffer(&front_buffer, BUFFER_WIDTH, BUFFER_HEIGHT);
    }
}
