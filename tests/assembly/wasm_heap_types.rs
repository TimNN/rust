// only-wasm32
// assembly-output: emit-asm

// compile-flags: --edition=2021 -Zmerge-functions=disabled -Aunused-imports -Aimproper_ctypes_definitions
//-Cno-prepopulate-passes

// -Dimproper_ctypes_definitions

#![crate_type = "cdylib"]
#![feature(wasm_heap_types_v0, wasm_heap_types_v1)]

use core::ffi::wasm::{
    self, global, table, Extern, ExternRef, Global, HeapRef, IsHeapRef,
    IsHeapRefWithNullInitAllowed, Table,
};

// CHECK-LABEL: @nullable_heapref
// CHECK-SAME: ptr addrspace(10) nocapture %0
#[no_mangle]
pub extern "C" fn nullable_heapref(_: HeapRef<Extern>) {}

// CHECK-LABEL: @externref
// CHECK-SAME: ptr addrspace(10) nocapture %0
#[no_mangle]
pub extern "C" fn externref(_: ExternRef) {}

#[repr(transparent)]
pub struct Foo(HeapRef<Extern>);

unsafe impl IsHeapRefWithNullInitAllowed for Foo {}

global!(MyGlobal: Foo);
table!(MyTable: Foo);

global!(pub Alpha: Foo);
global!(pub Beta: Foo);

// CHECK-LABEL: @global_set
// CHECK-SAME: foobar
#[no_mangle]
pub extern "C" fn global_set(v: Foo) {
    MyGlobal.set(v)
}

#[no_mangle]
pub extern "C" fn global_get() -> Foo {
    MyGlobal.get()
}

// CHECK-LABEL: @table_set
// CHECK-SAME: foobar
#[no_mangle]
pub extern "C" fn table_set(i: u32, v: Foo) {
    MyTable.set(i, v)
}

#[no_mangle]
pub extern "C" fn table_get(i: u32) -> Foo {
    MyTable.get(i)
}

// C!HECK-LABEL: @raw_extern
// C!HECK-SAME: ptr addrspace(10) nocapture %0
// C!HECK-SAME: ptr addrspace(10) nocapture %1
// #[no_mangle]
// pub extern "C" fn raw_extern(_: *const Extern, _: *mut Extern) {}

// C!HECK-LABEL: @option_extern
// C!HECK-SAME: ptr addrspace(10) nocapture %0
// #[no_mangle]
// pub extern "C" fn option_extern(_: Option<&Extern>) {}

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
