// only-wasm32
// assembly-output: emit-asm

// compile-flags: --edition=2021 -Zmerge-functions=disabled -Dimproper_ctypes_definitions

#![crate_type = "cdylib"]
#![feature(wasm_heap_types_v0, wasm_heap_types_v1)]

use core::ffi::wasm::{
    self, global, table, Extern, ExternRef, Global, HeapRef, IsHeapRef,
    IsHeapRefWithNullInitAllowed, Table,
};

// CHECK-LABEL: @_ZN15wasm_heap_types1_6GLOBAL
// CHECK-SAME: = internal addrspace(1) global ptr addrspace(10) undef

// CHECK-LABEL: @_ZN15wasm_heap_types1_5TABLE
// CHECK-SAME: = internal addrspace(1) global [0 x ptr addrspace(10)] undef

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

unsafe impl IsHeapRefWithNullInitAllowed for Foo {}

global!(MyGlobal: Foo);
table!(MyTable: Foo);

global!(pub Alpha: Foo);
global!(pub Beta: Foo);

// CHECK-LABEL: @global_set
// CHECK-NEXT: start:
// CHECK-NEXT: store ptr addrspace(10) %v, ptr addrspace(1) @_ZN15wasm_heap_types1_6GLOBAL
// CHECK-NEXT: ret void
#[no_mangle]
pub extern "C" fn global_set(v: Foo) {
    MyGlobal.set(v)
}

// CHECK-LABEL: @global_get
// CHECK-NEXT: start:
// CHECK-NEXT: load volatile ptr addrspace(10), ptr addrspace(1) @_ZN15wasm_heap_types1_6GLOBAL
// CHECK-NEXT: ret ptr addrspace(10) %0
#[no_mangle]
pub extern "C" fn global_get() -> Foo {
    MyGlobal.get()
}

// CHECK-LABEL: @table_set
// CHECK-NEXT: start:
// CHECK-NEXT: call void @llvm.wasm.table.set.externref(
// CHECK-SAME: ptr addrspace(1) @_ZN15wasm_heap_types1_5TABLE
// CHECK-SAME: i32 %i, ptr addrspace(10) %v
// CHECK-NEXT: ret void
#[no_mangle]
pub extern "C" fn table_set(i: u32, v: Foo) {
    MyTable.set(i, v)
}

// CHECK-LABEL: @table_get
// CHECK-NEXT: start:
// CHECK-NEXT: call ptr addrspace(10) @llvm.wasm.table.get.externref(
// CHECK-SAME: ptr addrspace(1) @_ZN15wasm_heap_types1_5TABLE
// CHECK-SAME: i32 %i
// CHECK-NEXT: ret ptr addrspace(10) %0
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
