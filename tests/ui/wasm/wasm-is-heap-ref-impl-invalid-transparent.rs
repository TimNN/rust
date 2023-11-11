// only-wasm32

// compile-flags: --crate-type=lib --edition=2021
// check-fail

#![feature(wasm_heap_types_v0, wasm_heap_types_v1)]

use core::ffi::wasm::nullability_marker::*;
use core::ffi::wasm::{ExternRef, IsHeapRef};

// Ensure that the `#[repr(transparent)]` error triggers, not the wasm-specific
// one.
#[repr(transparent)]
struct InvalidTransparent(ExternRef, ExternRef);
//~^ ERROR transparent struct needs at most one field with non-trivial

impl IsHeapRef for InvalidTransparent {
    type Nullability = Nullable;
}
