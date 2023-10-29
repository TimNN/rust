// only-wasm32
// assembly-output: emit-asm
// compile-flags: --edition=2021

#![crate_type = "cdylib"]
#![feature(wasm_externref, wasm_ffi)]
#![feature(wasm_abi)]

use core::ffi::wasm::ExternRef;

extern "C" {
    fn by_ref(x: &ExternRef);
}

// CHECK-LABEL: @passthrough
// CHECK-NEXT: foobar
#[no_mangle]
pub extern "C" fn passthrough(x: ExternRef, y: u32) -> ExternRef {
    // unsafe { by_ref(&x) };
    return x;
}

fn add(x: u32) -> u32 {
    x + 1
}

#[no_mangle]
pub extern "C" fn fnptr(_x: extern "wasm" fn(u32) -> u32) -> fn(u32) -> u32 {
    add
}

#[no_mangle]
pub extern "C" fn call(v: u32, x: extern "wasm" fn(u32) -> u32) -> u32 {
    x(v)
}

pub static FOOS: u32 = 43;

#[no_mangle]
pub extern "C" fn staticcc() {
    // let x = 42;
    // let y = &x;
    let z = &FOOS;

    unsafe { consume(z) };
}

extern "C" {
    fn consume(_: &u32);
}
