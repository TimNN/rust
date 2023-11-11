// only-wasm32

// compile-flags: --crate-type=lib --edition=2021
// check-fail

#![feature(wasm_heap_types_v0, wasm_heap_types_v1)]

use core::ffi::wasm::nullability_marker::*;
use core::ffi::wasm::{ExternRef, IsHeapRef};

union UnionNotStruct {
    a: ExternRef,
}

impl IsHeapRef for UnionNotStruct {
    //~^ ERROR `IsHeapRef` may only be implemented for `struct` types

    type Nullability = Nullable;
}

enum EnumNotStruct {
    Alpha(ExternRef),
}

impl IsHeapRef for EnumNotStruct {
    //~^ ERROR `IsHeapRef` may only be implemented for `struct` types

    type Nullability = Nullable;
}

struct NotTransparent(ExternRef);

impl IsHeapRef for NotTransparent {
    //~^ ERROR `IsHeapRef` may only be implemented for `#[repr(transparent)]` types

    type Nullability = Nullable;
}

#[repr(transparent)]
struct OnlyZst((), ());

impl IsHeapRef for OnlyZst {
    //~^ ERROR `IsHeapRef` implemented for type that does not contain a `IsHeapRef` field

    type Nullability = Nullable;
}

#[repr(transparent)]
struct FieldNotHeapRef((), *const ());

impl IsHeapRef for FieldNotHeapRef {
    //~^ ERROR Field 1 (*const ()) does not implement `IsHeapRef`

    type Nullability = Nullable;
}

#[repr(transparent)]
struct GenericNotOkay<T>(T);

impl<T> IsHeapRef for GenericNotOkay<T> {
    //~^ ERROR Field 0 (T) does not implement `IsHeapRef`.

    type Nullability = Nullable;
}

#[repr(transparent)]
struct GenericNullabilityMismatch<T>(T);

impl<T: IsHeapRef> IsHeapRef for GenericNullabilityMismatch<T> {
    //~^ ERROR Nullability mismatch.

    type Nullability = Nullable;
}
