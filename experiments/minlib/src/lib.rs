#![feature(new_uninit)]
extern crate wee_alloc;

// Use `wee_alloc` as the global allocator.
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[link(wasm_import_module = "env")]
extern "C" {
    fn print(ptr: *const u8, len: usize);
}

const DISPLAY_BUFFER_WIDTH: usize = 40;
const DISPLAY_BUFFER_HEIGHT: usize = 25;
pub const DISPLAY_BUFFER_SIZE: usize = DISPLAY_BUFFER_WIDTH * DISPLAY_BUFFER_HEIGHT;
// pub extern "C" DISPLAY_BUFFER_SIZE2: usize = DISPLAY_BUFFER_WIDTH * DISPLAY_BUFFER_HEIGHT;

pub struct State {
    display_buffer: Box<[u8]>,
    x: i32,
    y: i32,
    vx: i32,
    vy: i32,
}

#[no_mangle]
pub extern "C" fn setup() -> *mut State {
    // Allocate memory for display buffer
    let buffer = (0..DISPLAY_BUFFER_SIZE)
        .map(|_| b' ')
        .collect::<Vec<u8>>()
        .into_boxed_slice();

    // Allocate memory for state
    let state = Box::<State>::new(State {
        display_buffer: buffer,
        x: 0,
        y: 0,
        vx: 1,
        vy: 1,
    });
    print_safe("Initialised state");
    Box::into_raw(state)
}

#[no_mangle]
/// Safe if called no more than once with result from setup
pub unsafe extern "C" fn destroy(state: *mut State) {
    Box::from_raw(state);
}

#[no_mangle]
/// Safe if called once at a time with result from setup() without calling destroy
pub unsafe extern "C" fn draw(state: *mut State) -> *mut u8 {
    let state = state.as_mut().unwrap();
    draw_safe(state);

    state.display_buffer.first_mut().unwrap()
}

fn print_safe(text: &str) {
    unsafe { print(text.as_ptr(), text.len()) }
}

fn ball_index(state: &State) -> usize {
    state.x as usize + state.y as usize * DISPLAY_BUFFER_WIDTH
}

fn draw_safe(state: &mut State) {
    let old_index = ball_index(state);
    if (state.vx > 0 && state.x as usize >= DISPLAY_BUFFER_WIDTH - 1)
        || (state.vx < 0 && state.x <= 0)
    {
        state.vx *= -1;
    }
    if (state.vy > 0 && state.y as usize >= DISPLAY_BUFFER_HEIGHT - 1)
        || (state.vy < 0 && state.y <= 0)
    {
        state.vy *= -1;
    }
    state.x += state.vx;
    state.y += state.vy;
    let new_index = ball_index(state);

    let buffer = &mut *state.display_buffer;

    buffer[old_index] = b' ';
    buffer[new_index] = b'O';
}
