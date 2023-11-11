// only-wasm32
// assembly-output: emit-asm

// compile-flags: --edition=2021 -Zmerge-functions=disabled -Dimproper_ctypes_definitions

#![crate_type = "cdylib"]
#![feature(wasm_heap_types_v0, wasm_heap_types_v1)]

use core::ffi::wasm::{self, Extern, ExternRef, HeapRef, IsHeapRef};

// CHECK-LABEL: @nullable_heapref
// CHECK-SAME: ptr addrspace(10) nocapture %r
#[no_mangle]
pub extern "C" fn nullable_heapref(r: HeapRef<Extern>) {}

// CHECK-LABEL: @externref
// CHECK-SAME: ptr addrspace(10) nocapture %r
#[no_mangle]
pub extern "C" fn externref(r: ExternRef) {}

#[repr(transparent)]
pub struct Foo(HeapRef<Extern>);

impl IsHeapRef for Foo {
    type Nullability = wasm::nullability_marker::Nullable;
}
