
#[macro_use]
extern crate lazy_static;

extern "C" {
    pub fn jslog(s: usize);
}

use std::sync::Mutex;
use std::cell::RefCell;

enum Cell {
    Alive,
    Dead
}

// struct Canvas {
//     width: usize,
//     height: usize,
//     data: Vec<u8>
// }

struct GameState {
    canvas_width: usize,
    canvas_height: usize,
    canvas_data: RefCell<Vec<u8>>,
    cell_size: f32,
    cell_row_count: usize,
    cell_col_count: usize,
    cells: RefCell<Vec<Cell>>,
    mouse_x: usize,
    mouse_y: usize,
    mouse_x_max: usize,
    mouse_y_max: usize
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            canvas_width: 0,
            canvas_height: 0,
            canvas_data: RefCell::new(Vec::with_capacity(0)),
            cell_size: 0.0,
            cell_row_count: 0,
            cell_col_count: 0,
            cells: RefCell::new(Vec::with_capacity(0)),
            mouse_x: 0,
            mouse_y: 0,
            mouse_x_max: 0,
            mouse_y_max: 0
        }
    }

    pub fn find_cell_index(&self) -> usize {
        let cell_hover_x = (self.mouse_x as f32 / self.cell_size) as usize;
        let cell_hover_y = (self.mouse_y as f32 / self.cell_size) as usize;
        return cell_hover_y * self.cell_col_count + cell_hover_x;
    }

    pub fn toggle_cell(&mut self) {
        let i = self.find_cell_index();
        let mut cells = self.cells.borrow_mut();
        match cells[i] {
            Cell::Alive => { cells[i] = Cell::Dead; },
            Cell::Dead => { cells[i] = Cell::Alive; }
        }
    }
}

lazy_static! {
    static ref GAME_STATE: Mutex<GameState> = Mutex::new(GameState::new());
}

pub fn tick() -> i32 { 
    4002
}

#[derive(Clone)]
struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}
impl Color {

    pub const fn from_rgba_u32(c: u32) -> Color {
        Color {
            r: ((c & 0xFF000000) >> 24) as u8,
            g: ((c & 0x00FF0000) >> 16) as u8,
            b: ((c & 0x0000FF00) >> 8) as u8,
            a: (c & 0x000000FF) as u8
        }
    }

    pub const BLACK: Color = Color::from_rgba_u32(0x000000FF);
    //pub const RED: Color = Color::from_rgba_u32(0xFF0000FF);
    //pub const GREEN: Color = Color::from_rgba_u32(0x00FF00FF);
    pub const YELLOW: Color = Color::from_rgba_u32(0xFFFF00FF);
    pub const BLUE: Color = Color::from_rgba_u32(0x0000FFFF);
    pub const GREY: Color = Color::from_rgba_u32(0xCCCCCCFF);
}

fn render_fill_rect(canvas_data: &mut Vec<u8>, canvas_width: usize, canvas_height: usize, color: Color, x: usize, y: usize, width: usize, height: usize) {
    let max_x = canvas_width - 1;
    let max_y = canvas_height - 1;
    if x <= max_x && y <= max_y {
        let width = if x + width > max_x { max_x - x } else { width };
        let height = if y + height > max_y { max_y - y } else { height };
        for _y in y..(y+height) {
            for _x in x..(x+width) {
                let i = (_y * canvas_width + _x) * 4;
                canvas_data[i] = color.r;
                canvas_data[i+1] = color.g;
                canvas_data[i+2] = color.b;
                canvas_data[i+3] = color.a;
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn render() {
    let state = &mut GAME_STATE.lock().unwrap();
    let width = state.canvas_width;
    let height = state.canvas_height;
    let cell_size = state.cell_size;
    let cell_size_usize = cell_size as usize;
    let row_count = state.cell_row_count;
    let col_count = state.cell_col_count;
    let mouse_x = state.mouse_x;
    let mouse_y = state.mouse_y;
    let mut canvas_data = state.canvas_data.borrow_mut();
    render_fill_rect(&mut canvas_data, width, height, Color::GREY, 0, 0, width, height);
    // render_fill_rect(state, Color::BLACK, 0, 0, width, 2);
    // render_fill_rect(state, Color::BLACK, 0, 0, 2, height);
    // render_fill_rect(state, Color::BLACK, width - 2, 0, 2, height);
    // render_fill_rect(state, Color::BLACK, 0, height - 3, width, 2);

    let cell_hover_x = (mouse_x as f32 / cell_size) as usize;
    let cell_hover_y = (mouse_y as f32 / cell_size) as usize;
    let cell_hover_x = (cell_hover_x as f32 * cell_size) as usize;
    let cell_hover_y = (cell_hover_y as f32 * cell_size) as usize;
    render_fill_rect(&mut canvas_data, width, height, Color::BLUE, cell_hover_x + 1, cell_hover_y + 1, cell_size_usize, cell_size_usize);

    let cells = state.cells.borrow();
    for y in 0..row_count {
        for x in 0..col_count {
            let cell_index = y * col_count + x;
            match cells[cell_index] {
                Cell::Alive => { 
                    let y = (y as f32 * cell_size) as usize;
                    let x = (x as f32 * cell_size) as usize;
                    render_fill_rect(&mut canvas_data, width, height, Color::YELLOW, x + 1, y + 1, cell_size_usize, cell_size_usize);
                },
                _ => { }
            }
        }
    }

    for y in 0..row_count {
        render_fill_rect(&mut canvas_data, width, height, Color::BLACK, 0, ((y+1) as f32 * cell_size) as usize, width, 1);
    }

    for x in 0..col_count {
        render_fill_rect(&mut canvas_data, width, height, Color::BLACK, ((x+1) as f32 * cell_size) as usize, 0, 1, height);
    }
}

#[no_mangle]
pub extern "C" fn init_rgba_canvas(width: usize, height: usize) -> *const u8 {
    let state = &mut GAME_STATE.lock().unwrap();
    state.canvas_width = width;
    state.canvas_height = height;
    state.canvas_data = RefCell::new(Vec::with_capacity(width * height * 4));
    if width > height {
        state.cell_size = width as f32 / 100.0;
        state.cell_col_count = 100;
        state.cell_row_count = (height as f32 / state.cell_size) as usize;
    } else {
        state.cell_size = height as f32 / 100.0;
        state.cell_row_count = 100;
        state.cell_col_count = (width as f32 / state.cell_size) as usize;
    }
    //state.cells = Vec::with_capacity(state.cell_col_count * state.cell_row_count);
    state.cells = RefCell::new((0..(state.cell_col_count * state.cell_row_count))
        .map(|_x| Cell::Dead)
        .collect());
    state.mouse_x_max = (state.cell_col_count as f32 * state.cell_size) as usize - 2;
    state.mouse_y_max = (state.cell_row_count as f32 * state.cell_size) as usize - 2;

    // init all pixels to white
    let mut canvas_data = state.canvas_data.borrow_mut();
    for _y in 0..height {
        for _x in 0..width {
            canvas_data.push(0xFF);
            canvas_data.push(0xFF);
            canvas_data.push(0xFF);
            canvas_data.push(0xFF);
        }
    }

    canvas_data.as_ptr()
}

#[no_mangle]
pub fn alloc(len: usize) -> *mut u8 {
    let mut buf = Vec::with_capacity(len);
    let ptr = buf.as_mut_ptr();
    // stop the Vec destructor from being called
    // when buf goes out of scope at end of function
    std::mem::forget(buf);
    return ptr;
}

#[no_mangle]
pub fn onmousemove(x: usize, y: usize) { 
    let state = &mut GAME_STATE.lock().unwrap();
    state.mouse_x = std::cmp::min(std::cmp::max(x, 2), state.mouse_x_max);
    state.mouse_y = std::cmp::min(std::cmp::max(y, 2), state.mouse_y_max);
}

#[no_mangle]
pub fn onclick(x: usize, y: usize) {
    let state = &mut GAME_STATE.lock().unwrap();
    state.mouse_x = std::cmp::min(std::cmp::max(x, 2), state.mouse_x_max);
    state.mouse_y = std::cmp::min(std::cmp::max(y, 2), state.mouse_y_max);
    state.toggle_cell();
}