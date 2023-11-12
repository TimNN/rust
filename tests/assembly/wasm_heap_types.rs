// only-wasm32
// assembly-output: emit-asm

// compile-flags: --edition=2021 -Zmerge-functions=disabled
// compile-flags: -Dimproper_ctypes_definitions

#![crate_type = "cdylib"]
#![feature(lint_reasons, wasm_heap_types_v0, wasm_heap_types_v1)]

use core::ffi::wasm::{self, Extern, ExternRef, HeapRef, IsHeapRef};

// CHECK-LABEL: @non_null_heapref
// CHECK-SAME: ptr addrspace(10) nocapture %_r
#[expect(improper_ctypes_definitions)]
#[no_mangle]
pub extern "C" fn non_null_heapref(_r: HeapRef<Extern>) {}

// CHECK-LABEL: @externref
// CHECK-SAME: ptr addrspace(10) nocapture %_r
#[no_mangle]
pub extern "C" fn externref(_r: ExternRef) {}

#[repr(transparent)]
pub struct Foo(HeapRef<Extern>);

impl IsHeapRef for Foo {
    type Nullability = wasm::nullability_marker::NonNull;
}

#[cfg(never)]
#[no_mangle]
pub extern "C" fn is_some(r: Option<HeapRef<Extern>>) -> bool {
    match r {
        Some(_) => true,
        None => false,
    }
}
