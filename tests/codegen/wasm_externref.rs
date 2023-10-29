// only-wasm32
// compile-flags: --edition=2021

#![crate_type = "cdylib"]
#![feature(wasm_externref, wasm_ffi)]

use core::ffi::wasm::ExternRef;

#[used]
pub static FOO: [ExternRef; 0] = [];

extern "C" {
    fn by_ref(x: &ExternRef);
}

// CHECK-LABEL: @passthrough
// CHECK-NEXT: foobar
#[no_mangle]
pub extern "C" fn passthrough(x: ExternRef) -> ExternRef {
    // unsafe { by_ref(&x) };

    x
}

#[no_mangle]
pub extern "C" fn fnptr(x: fn(u32) -> u32) -> fn(u32) -> u32 {
    x
}

#[no_mangle]
pub extern "C" fn call(v: u32, x: fn(u32) -> u32) -> u32 {
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
