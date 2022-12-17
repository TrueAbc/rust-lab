
use core::arch::asm;

const SSIZE: isize = 48; // 48 byte 大小的stack空间


#[derive(Debug, Default)]
#[repr(C)]
struct ThreadContext {
    rsp : u64, // stack pointer
}

fn main() {
    let mut ctx = ThreadContext::default();
    let mut stack = vec![0_u8; SSIZE as usize];
    unsafe {
        let stack_bottom = stack.as_mut_ptr().offset(SSIZE);
        
        // index 32 是第一个16byte偏移的地址, 向下舍入到最近的16位偏移的地址
        let sb_aligned = (stack_bottom as usize & !15) as *mut u8; // 16 byte 对齐
        // stack增长的方向
        std::ptr::write(sb_aligned.offset(-16) as *mut u64, hello as u64);
        ctx.rsp = sb_aligned.offset(-16) as u64;
        gt_switch(&mut ctx);
    }
}

fn hello() -> ! {
    println!("I love waking up on a new stack");
    loop{}
}

// 将new代表的地址传入到rsp, 这里表示使用任意的通用寄存器存储变量
// rsp 寄存器是stack的下一个可用地址
unsafe fn gt_switch(new : *const ThreadContext) {
    asm!(
        "mov rsp, [{0} + 0x00]", // 0x00 offset of var in {0}
        // move what's at the + 0x00 offset from the memory location 
        // that {compiler_chosen_general_purpose_register}points to, to the rspregister
        "ret",
        in(reg) new,
    )
}