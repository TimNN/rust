#![unstable(feature = "wasm_heap_types_v0", issue = "none")]
#![allow(missing_debug_implementations, missing_docs)]

use crate::marker::PhantomData;

/// Identifies types that exist (only) on the WebAssembly heap.
///
/// Only the standard library or macros from the standard library, may
/// implement this trait. Implementing this trait for a type indicates that the
/// Rust compiler knows how to represent that type on the WebAssembly heap.
/// Implementing this trait incorrectly may lead to compiler errors or panics.
///
/// The representation of a type on the WebAssembly heap is opaque to the Rust
/// compiler, which restricts how they can be used.
///
/// Use the [HeapRef] type to reference instances of [HeapTypeRepr] types on the
/// WebAssembly heap.
#[lang = "wasm_heap_type_repr"]
#[unstable(feature = "wasm_heap_types_v1", issue = "none")]
// FIXME: Do we need the `'static` bound? Should we keep it or get rid of it?
// FIXME: Add an appropriate `rustc_on_unimplemented` attribute.
pub trait HeapTypeRepr: 'static {
    /// Perma-unstable item to prevent (stable) implementations outside of the
    /// standard library and its macros.
    #[doc(hidden)]
    #[unstable(feature = "wasm_heap_types_internals", issue = "none")]
    const _INTERNAL: ();
}

/// A non-null reference to the WebAssembly heap.
///
/// The representation of references to the WebAssembly heap is opaque to the
/// Rust compiler, which restricts to how they can be used:
///
/// * They cannot be stored in linear memory.
/// * It is not possible to create pointers or references to a [HeapRef].
/// * A [HeapRef] cannot be used as the type of a field in a union, a
///   non-transparent struct, or an enum other than [Option].
///
/// If this type is used as the type of an [Option], then it is considered a
/// _nullable_ reference to the WebAssembly heap going forward. A nullable
/// reference to the WebAssembly heap **cannot** be stored in an [Option].
///
/// If this type is used as the field of a transparent struct (or an [Option]),
/// all the restrictions above apply to that struct as well.
#[lang = "wasm_heap_ref_ty"]
#[unstable(feature = "wasm_heap_types_v1", issue = "none")]
pub struct HeapRef<T: HeapTypeRepr + ?Sized>(PhantomData<T>);

impl<T: HeapTypeRepr + ?Sized> Copy for HeapRef<T> {}

impl<T: HeapTypeRepr + ?Sized> Clone for HeapRef<T> {
    fn clone(&self) -> Self {
        unreachable!();
    }
}

#[lang = "wasm_is_heap_ref"]
#[unstable(feature = "wasm_heap_types_v1", issue = "none")]
// FIXME: Add an appropriate `rustc_on_unimplemented` attribute.
// DESIGN/FIXME: I went back-and-forth on this a _lot_: Should this trait
// be automatically implemented by the compiler? Or manually for every type?
// For a manual implementation, the compiler would need to ensure that it is
// "correct", and that would require tracking nullability information in the
// trait (because `Option<HeapRef>` is only valid for a non-null `HeapRef`). Two
// major advantages of a manual implementation are that it shows up better in
// rustdoc, and that it ensures that types have to explicitly opt-in to
// containing `HeapRef`s.
pub trait IsHeapRef {
    type Nullability;
}

#[unstable(feature = "wasm_heap_types_v1", issue = "none")]
pub mod nullability_marker {
    pub enum NonNull {}
    pub enum Nullable {}
}

impl<T: HeapTypeRepr + ?Sized> IsHeapRef for HeapRef<T> {
    type Nullability = nullability_marker::NonNull;
}

impl<T: IsHeapRef<Nullability = nullability_marker::NonNull>> IsHeapRef for Option<T> {
    type Nullability = nullability_marker::Nullable;
}

extern "C" {
    /// The `extern` WebAssembly heap type.
    ///
    /// See [ExternRef].
    #[lang = "wasm_extern_ty"]
    #[unstable(feature = "wasm_heap_types_v1", issue = "none")]
    pub type Extern;
}

impl HeapTypeRepr for Extern {
    #[doc(hidden)]
    const _INTERNAL: () = ();
}

/// The [`externref`] WebAssembly type.
///
/// A nullable reference to [Extern].
///
/// [`externref`]: https://webassembly.github.io/spec/core/syntax/types.html#syntax-reftype
#[repr(transparent)]
pub struct ExternRef(Option<HeapRef<Extern>>);

impl IsHeapRef for ExternRef {
    type Nullability = nullability_marker::Nullable;
}
