// only-wasm32

// compile-flags: --crate-type=lib --edition=2021
// check-pass

#![feature(wasm_heap_types_v0, wasm_heap_types_v1)]

use core::ffi::wasm::nullability_marker::*;
use core::ffi::wasm::{ExternRef, IsHeapRef};

#[repr(transparent)]
struct GenericOkay<T>(T);

impl<T: IsHeapRef> IsHeapRef for GenericOkay<T> {
    type Nullability = T::Nullability;
}

#[repr(transparent)]
struct GenericExplicitNullability<T>(T);

impl<T: IsHeapRef<Nullability = Nullable>> IsHeapRef for GenericExplicitNullability<T> {
    type Nullability = Nullable;
}

#[repr(transparent)]
struct ExplicitNullability(ExternRef);

impl IsHeapRef for ExplicitNullability {
    type Nullability = Nullable;
}

#[repr(transparent)]
struct NullabilityProjection(ExternRef);

impl IsHeapRef for NullabilityProjection {
    type Nullability = <ExternRef as IsHeapRef>::Nullability;
}
