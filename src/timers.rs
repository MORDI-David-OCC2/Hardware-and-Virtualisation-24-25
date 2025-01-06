//! Timer-related features
//! 
//! This is not available on every architecture.

// Delay function (busy wait)
#[cfg(target_arch = "arm")]
fn delay_ms(count: u32) {
    for _ in 0..count*8000 {
        // Simple busy-wait loop
        unsafe { core::arch::asm!("nop") }
    }
}