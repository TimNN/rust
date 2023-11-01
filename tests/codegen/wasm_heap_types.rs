// only-wasm32
// compile-flags: -Dimproper_ctypes_definitions --edition=2021

#![crate_type = "cdylib"]
#![feature(wasm_heap_types_v0, wasm_heap_types_v1)]

use core::ffi::wasm::{Extern, ExternRef};

// CHECK-LABEL: @externref
// CHECK-SAME: ptr addrspace(10) nocapture %0
#[no_mangle]
pub extern "C" fn externref(_: ExternRef) {}

// CHECK-LABEL: @raw_extern
// CHECK-SAME: ptr addrspace(10) nocapture %0
// CHECK-SAME: ptr addrspace(10) nocapture %1
#[no_mangle]
pub extern "C" fn raw_extern(_: *const Extern, _: *mut Extern) {}

// CHECK-LABEL: @option_extern
// CHECK-SAME: ptr addrspace(10) nocapture %0
#[no_mangle]
pub extern "C" fn option_extern(_: Option<&Extern>) {}

// #[used]
// pub static FOO: [ExternRef; 0] = [];

// extern "C" {
//     fn by_ref(x: &ExternRef);
// }

// #[no_mangle]
// pub extern "C" fn consume(x: &ExternRef) {}

// #[no_mangle]
// pub extern "C" fn fnptr(x: fn(u32) -> u32) -> fn(u32) -> u32 {
//     x
// }

// #[no_mangle]
// pub extern "C" fn call(v: u32, x: fn(u32) -> u32) -> u32 {
//     x(v)
// }

// pub static FOOS: u32 = 43;

// #[no_mangle]
// pub extern "C" fn staticcc() {
//     // let x = 42;
//     // let y = &x;
//     let z = &FOOS;

//     unsafe { consume(z) };
// }

// extern "C" {
//     fn consume(_: &u32);
// }
