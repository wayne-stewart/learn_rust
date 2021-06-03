
#[macro_use]
extern crate lazy_static;

extern "C" {
    pub fn jslog(s: usize);
}

use std::sync::Mutex;
use std::cell::RefCell;

#[derive(Copy,Clone)]
enum Cell {
    Alive,
    Dead,

    // these are transitional values
    Dying,      // still alive for the purposes of cell calculations
    Reviving    // still dead for the purposes of cell calculations
}

impl Cell {
    pub fn is_alive(&self) -> usize {
        match self {
            Cell::Alive => 1,
            Cell::Dying => 1,
            _ => 0
        }
    }
}

struct GameState {
    canvas_width: usize,
    canvas_height: usize,
    canvas_data: RefCell<Vec<u8>>,
    cell_size: f32,
    cell_row_count: usize,
    cell_col_count: usize,
    cells: RefCell<Vec<Cell>>,
    cells_save: RefCell<Vec<Cell>>,
    buffer: RefCell<Vec<u8>>,
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
            cells_save: RefCell::new(Vec::with_capacity(0)),
            buffer: RefCell::new(Vec::with_capacity(8192)),
            mouse_x: 0,
            mouse_y: 0,
            mouse_x_max: 0,
            mouse_y_max: 0
        }
    }

    pub fn find_cell_index_mouse_hover(&self) -> usize {
        let cell_hover_x = (self.mouse_x as f32 / self.cell_size) as usize;
        let cell_hover_y = (self.mouse_y as f32 / self.cell_size) as usize;
        return cell_hover_y * self.cell_col_count + cell_hover_x;
    }

    pub fn toggle_cell_mouse_hover(&mut self) {
        let i = self.find_cell_index_mouse_hover();
        self.toggle_cell(i);
    }

    pub fn toggle_cell_xy(&mut self, x: usize, y: usize) {
        self.toggle_cell(y * self.cell_col_count + x);
    }

    pub fn toggle_cell(&mut self, index: usize) {
        let mut cells = self.cells.borrow_mut();
        match cells[index] {
            Cell::Alive => { cells[index] = Cell::Dead; },
            Cell::Dead => { cells[index] = Cell::Alive; },
            _ => { }
        }
    }

    pub fn save(&mut self) {
        let cells = self.cells.borrow();
        let mut saved = self.cells_save.borrow_mut();
        for i in 0..cells.len() {
            saved[i] = cells[i];
        }
    }

    pub fn restore(&mut self) {
        let mut cells = self.cells.borrow_mut();
        let saved = self.cells_save.borrow();
        for i in 0..cells.len() {
            cells[i] = saved[i];
        }
    }

    pub fn kill_all(&mut self) {
        let mut cells = self.cells.borrow_mut();
        for i in 0..cells.len() {
            cells[i] = Cell::Dead;
        }
    }

    pub fn is_alive(&self, x: usize, y: usize) -> bool {
        let cells = self.cells.borrow();
        match cells[y * self.cell_col_count + x].is_alive() {
            1 => true,
            _ => false
        }
    }

    pub fn die(&mut self, x: usize, y: usize) {
        let mut cells = self.cells.borrow_mut();
        cells[y * self.cell_col_count + x] = Cell::Dying;
    }

    pub fn revive(&mut self, x: usize, y: usize) {
        let mut cells = self.cells.borrow_mut();
        cells[y * self.cell_col_count + x] = Cell::Reviving;
    }

    pub fn count_living_neighbours(&self, x: usize, y: usize) -> usize {
        let x = x as i32;
        let y =  y as i32;
        let cells = self.cells.borrow();
        let stride = self.cell_col_count;
        let xlen = self.cell_col_count as i32;
        let ylen = self.cell_row_count as i32;
        let pairs: [(i32,i32); 8] = [
            (x-1, y-1),
            (x,   y-1),
            (x+1, y-1),
            (x-1, y),
            (x+1, y),
            (x-1, y+1),
            (x,   y+1),
            (x+1, y+1)
            ];
        let mut count = 0;
        for coord in pairs.iter() {
            if coord.0 >= 0 && coord.0 < xlen &&
               coord.1 >= 0 && coord.1 < ylen {
                   count += cells[coord.1 as usize * stride + coord.0 as usize].is_alive();
               }
        }
        count
    }

    pub fn tick(&mut self) {
        // each alive cell with 1 or 0 alive neighbors dies
        // each alive cell with 4+ alive neighbors dies
        // each dead cell with 3 alive neighbors lives

        for y in 0..self.cell_row_count {
            for x in 0..self.cell_col_count {
                let nc = self.count_living_neighbours(x, y);
                let living = self.is_alive(x, y);
                if living && (nc < 2 || nc > 3) {
                    self.die(x, y);
                }
                else if !living && nc == 3 {
                    self.revive(x, y);
                }
            }
        }

        // finalize alive or dead
        let mut cells = self.cells.borrow_mut();
        for i in 0..cells.len() {
            cells[i] = match cells[i] {
                Cell::Dying    => Cell::Dead,
                Cell::Reviving => Cell::Alive,
                Cell::Alive    => Cell::Alive,
                Cell::Dead     => Cell::Dead
            };
        }
    }
}

lazy_static! {
    static ref GAME_STATE: Mutex<GameState> = Mutex::new(GameState::new());
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
    let mut canvas_data = state.canvas_data.borrow_mut();
    let cells = state.cells.borrow();
    let width = state.canvas_width;
    let height = state.canvas_height;
    let cell_size = state.cell_size;
    let cell_size_usize = cell_size as usize;
    let row_count = state.cell_row_count;
    let col_count = state.cell_col_count;
    let mouse_x = state.mouse_x;
    let mouse_y = state.mouse_y;

    // draw grey background
    render_fill_rect(&mut canvas_data, width, height, Color::GREY, 0, 0, width, height);

    // draw the current cell the mouse is hovering over blue
    let cell_hover_x = (mouse_x as f32 / cell_size) as usize;
    let cell_hover_y = (mouse_y as f32 / cell_size) as usize;
    let cell_hover_x = (cell_hover_x as f32 * cell_size) as usize;
    let cell_hover_y = (cell_hover_y as f32 * cell_size) as usize;
    render_fill_rect(&mut canvas_data, width, height, Color::BLUE, cell_hover_x + 1, cell_hover_y + 1, cell_size_usize, cell_size_usize);

    // draw all the Alive cells yellow
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

    // draw the horizontal grid lines
    for y in 0..row_count {
        render_fill_rect(&mut canvas_data, width, height, Color::BLACK, 0, ((y+1) as f32 * cell_size) as usize, width, 1);
    }

    // draw the vertical grid lines
    for x in 0..col_count {
        render_fill_rect(&mut canvas_data, width, height, Color::BLACK, ((x+1) as f32 * cell_size) as usize, 0, 1, height);
    }
}

#[no_mangle]
pub extern "C" fn init_rgba_canvas(width: usize, height: usize) {
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

    state.mouse_x_max = (state.cell_col_count as f32 * state.cell_size) as usize - 2;
    state.mouse_y_max = (state.cell_row_count as f32 * state.cell_size) as usize - 2;

    state.cells = RefCell::new((0..(state.cell_col_count * state.cell_row_count))
        .map(|_x| Cell::Dead)
        .collect());
    
    state.cells_save = RefCell::new((0..(state.cell_col_count * state.cell_row_count))
        .map(|_x| Cell::Dead)
        .collect());

    state.canvas_data = RefCell::new((0..(width * height * 4))
        .map(|_x| 0xFF)
        .collect());
}

#[no_mangle]
pub extern "C" fn get_canvas_ptr() -> *const u8 {
    let state = &mut GAME_STATE.lock().unwrap();
    let ptr = state.canvas_data.borrow().as_ptr(); 
    ptr
}

#[no_mangle]
pub extern "C" fn get_buffer_ptr() -> *const u8 {
    let state = &mut GAME_STATE.lock().unwrap();
    let ptr = state.buffer.borrow().as_ptr(); 
    ptr
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
    state.toggle_cell_mouse_hover();
}

#[no_mangle]
pub fn clear() {
    let state = &mut GAME_STATE.lock().unwrap();
    state.kill_all();
}

#[no_mangle]
pub fn tick() { 
    let state = &mut GAME_STATE.lock().unwrap();
    state.tick();
}

#[no_mangle]
pub fn save() {
    let state = &mut GAME_STATE.lock().unwrap();
    state.save();
}

#[no_mangle]
pub fn restore() {
    let state = &mut GAME_STATE.lock().unwrap();
    state.restore();
}

#[no_mangle]
pub unsafe fn export() -> usize {
    let state = &mut GAME_STATE.lock().unwrap();
    let mut buffer = state.buffer.borrow_mut();

    // use the general purpose buffer to communicate with javsacript
    // alias the buffer into a new Vec<i32> so we can write 
    // coordinates into it.
    //buffer.truncate(0);
    let buf_ptr = buffer.as_mut_ptr();
    let mut buf_i32: Vec<i32> = Vec::from_raw_parts(buf_ptr as *mut i32, 0, buffer.capacity() / 4);

    for y in 0.. state.cell_row_count {
        for x in 0..state.cell_col_count {
            if state.is_alive(x,y) {
                buf_i32.push(x as i32);
                buf_i32.push(y as i32);
            }
        }
    }
    
    // return the length so javascript knows how much to read
    // from the general purpose buffer
    // forget the temporary buf_i32 instance so Rust doesn't free
    // the memory when the function goes out of scope.
    let len = buf_i32.len();
    std::mem::forget(buf_i32);
    len
}

#[no_mangle]
pub unsafe fn import(len: usize) { 
    let state = &mut GAME_STATE.lock().unwrap();
    state.kill_all();
    let mut coords: Vec<(usize,usize)> = vec![];

    let mut buffer = state.buffer.borrow_mut();
    let buf_ptr = buffer.as_mut_ptr();
    let buf_i32: Vec<i32> = Vec::from_raw_parts(buf_ptr as *mut i32, len, buffer.capacity() / 4);

    for i in 0..(len/2) {
        let i = i * 2;
        let x = buf_i32[i];
        let y = buf_i32[i+1];
        coords.push((x as usize, y as usize));
    }
    // forget to prevent the vector destructor from running since it is an alias
    std::mem::forget(buf_i32);
    // drop to let rust know we are done with this immutable reference so we can mutate later
    std::mem::drop(buffer);

    // find out how "big" the pattern is and center it in the available space
    let mut x_max = 0;
    let mut x_min = usize::max_value();
    let mut y_max = 0;
    let mut y_min = usize::max_value();
    for coord in coords.iter() {
        x_max = std::cmp::max(coord.0, x_max);
        x_min = std::cmp::min(coord.0, x_min);
        y_max = std::cmp::max(coord.1, y_max);
        y_min = std::cmp::min(coord.1, y_min);
    }
    let x_offset = state.cell_col_count / 2 - (x_max - x_min) / 2;
    let y_offset = state.cell_row_count / 2 - (y_max - y_min) / 2;
    for coord in coords.iter() {
        state.toggle_cell_xy(coord.0 - x_min + x_offset, coord.1 - y_min + y_offset);
    }
}

