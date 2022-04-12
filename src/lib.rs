#![no_std]
#![cfg_attr(test, no_main)]

#[cfg(test)]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
  loop {}
}