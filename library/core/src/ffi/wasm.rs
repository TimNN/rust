#![unstable(feature = "wasm_heap_types_v0", issue = "none")]
#![allow(missing_debug_implementations, missing_docs)]

/// Identifies types that exist (only) on the WebAssembly heap.
///
/// FIXME: impl-restricted
// #[lang = "wasm_heap_type"]
#[unstable(feature = "wasm_heap_types_v1", issue = "none")]
// FIXME: Add an appropriate `rustc_on_unimplemented` attribute.
pub trait HeapType {}

/// Identifies types that are represented as references to the WebAssembly heap.
///
/// FIXME: impl-restricted
#[lang = "wasm_heap_ref"]
#[unstable(feature = "wasm_heap_types_v1", issue = "none")]
// FIXME: Add an appropriate `rustc_on_unimplemented` attribute.
pub trait HeapRef {
    #[lang = "wasm_heap_ref_nullability"]
    type Nullability;
}

impl<T: HeapRef<Nullability = nullability_marker::NonNull>> HeapRef for Option<T> {
    type Nullability = nullability_marker::Nullable;
}

impl<T: HeapType + ?Sized> HeapRef for &T {
    type Nullability = nullability_marker::NonNull;
}

impl<T: HeapType + ?Sized> HeapRef for &mut T {
    type Nullability = nullability_marker::NonNull;
}

impl<T: HeapType + ?Sized> HeapRef for *const T {
    type Nullability = nullability_marker::Nullable;
}

impl<T: HeapType + ?Sized> HeapRef for *mut T {
    type Nullability = nullability_marker::Nullable;
}

#[unstable(feature = "wasm_heap_types_v1", issue = "none")]
pub mod nullability_marker {
    #[lang = "wasm_nullability_marker_non_null"]
    pub enum NonNull {}

    #[lang = "wasm_nullability_marker_nullable"]
    pub enum Nullable {}
}

extern "C" {
    /// The `extern` WebAssembly heap type.
    ///
    /// See [ExternRef].
    #[lang = "wasm_extern_ty"]
    #[unstable(feature = "wasm_heap_types_v1", issue = "none")]
    pub type Extern;
}

impl HeapType for Extern {}

/// The [`externref`] WebAssembly type.
///
/// A nullable reference to [Extern].
///
/// [`externref`]: https://webassembly.github.io/spec/core/syntax/types.html#syntax-reftype
#[repr(transparent)]
pub struct ExternRef(Option<&'static Extern>);

impl HeapRef for ExternRef {
    type Nullability = nullability_marker::Nullable;
}
