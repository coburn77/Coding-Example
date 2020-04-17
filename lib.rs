/*
This module defines a simple API for browser clients to interact with.
*/
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;
use core::sync::atomic::{AtomicU32, Ordering};

const WIDTH: usize = 300;
const HEIGHT: usize = 300;

/* Initializes the buffer to be written to on behalf of the animation algorithms */ 
#[no_mangle]
static mut BUFFER: [u32; WIDTH * HEIGHT] = [0; WIDTH * HEIGHT];

/* This function takes 2 numbers as an arguement and adds 
them together and returns them back to browser client */
#[wasm_bindgen]
pub fn add(x: u32, y: u32) -> u32 {
    x + y
}

/* This function takes a string arguement and reverses and 
returns reversed string back to client */
#[wasm_bindgen]
pub fn reverse(s: String) -> String {
    s.chars().rev().collect::<String>()
}

/* This function is the first example animation, the called 
animation algorithm writes to the defined above buffer. */
#[no_mangle]
pub extern fn render_one() {
    unsafe {
      render_frame_one( &mut BUFFER ) // Unsafe because static
    }
}

/* This function is the second example animation, the called 
animation algorithm writes to the defined above buffer. */
#[no_mangle]
pub extern fn render_two() {
    unsafe {
      render_frame_two( &mut BUFFER )
    }
}

/* Supports fetching thread safe interger */
static FRAME: AtomicU32 = AtomicU32::new( 0 );

/* Performs work of writing frame content to the buffer */
fn render_frame_one( buffer: &mut [u32; WIDTH * HEIGHT] ) {
    let f = FRAME.fetch_add(1, Ordering::Relaxed);
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let v : u32 = (x ^ y) as u32;
            buffer[y * WIDTH + x] = f.wrapping_add( v ) | 0xFF_00_00_00;
        }
    }
}

/* Performs work of writing frame content to the buffer */
fn render_frame_two( buffer: &mut [u32; WIDTH * HEIGHT] ) {
    let f = FRAME.fetch_add(1, Ordering::Relaxed);
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let v : u32 = ( (x as f32).sin() * 255.
                          + (y as f32).sin() * 255. ) as u32;
            buffer[y * WIDTH + x] = f.wrapping_add( v ) | 0xFF_00_00_00;
        }
    }
}
