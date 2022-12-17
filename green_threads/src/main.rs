#![feature(naked_functions)]

use std::arch::asm;

const DEFAULT_STACK_SIZE : usize = 1024 * 1024 * 2;
const MAX_THREADS : usize = 4;
static mut RUNTIME : usize = 8;

fn main() {
    
}