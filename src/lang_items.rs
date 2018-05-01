#[cfg_attr(any(target_arch = "powerpc", target_arch = "wasm32"), lang = "panic_fmt")]
#[no_mangle]
pub extern "C" fn panic_fmt() -> ! {
    loop {}
}
