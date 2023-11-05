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
#[rustc_deny_explicit_impl(implement_via_object = false)]
// FIXME: Add an appropriate `rustc_on_unimplemented` attribute.
// DESIGN/FIXME: I went back-and-forth on this a _lot_: Should this trait
// be automatically implemented by the compiler? Or manually for every type?
// For a manual implementation, the compiler would need to ensure that it is
// "correct", and that would require tracking nullability information in the
// trait (because `Option<HeapRef>` is only valid for a non-null `HeapRef`). Two
// major advantages of a manual implementation are that it shows up better in
// rustdoc, and that it ensures that types have to explicitly opt-in to
// containing `HeapRef`s.
pub trait IsHeapRef {}

// FIXME: Consider `extern` support for `Global` and `Table`.

/// Represents a WebAssembly [global].
///
/// Every reference to a global must be *compile time constant*. In Rust,
/// this means that every global is a separate type, defined with the
/// [global!] macro.
///
/// [global]: https://webassembly.github.io/spec/core/syntax/modules.html#syntax-global
pub trait Global<T: IsHeapRef> {
    fn get(&self) -> T;

    fn set(&self, val: T);
}

/// Represents a WebAssembly [table].
///
/// Every reference to a table must be *compile time constant*. In Rust,
/// this means that every table is a separate type, defined with the
/// [table!] macro.
///
/// [table]: https://webassembly.github.io/spec/core/syntax/modules.html#syntax-table
pub trait Table<T: IsHeapRef> {
    fn get(&self, idx: u32) -> T;

    fn set(&self, idx: u32, val: T);
}

#[unstable(feature = "wasm_heap_types_v1", issue = "none")]
// FIXME: Add an appropriate `rustc_on_unimplemented` attribute.
// FIXME: Consider including nullability information in `IsHeapRef`, to make
// this trait slightly safer. (Or get rid of it entirely, if we can support an
// explicit initializer).
pub unsafe trait IsHeapRefWithNullInitAllowed: IsHeapRef {}

/// Perma-unstable internals, primarily for use via macros.
#[doc(hidden)]
#[unstable(feature = "wasm_heap_types_internals", issue = "none")]
pub mod internals {
    use super::{IsHeapRef, IsHeapRefWithNullInitAllowed};
    use crate::cell::UnsafeCell;
    use crate::marker::PhantomData;

    /// The type of the `static` containing the actual global.
    #[lang = "wasm_global_ty"]
    // This contains an `UnsafeCell` to ensure it is considered mutable by
    // codegen. (Though it doesn't seem to make a difference to LLVM).
    pub struct GlobalImpl<T: IsHeapRef>(UnsafeCell<PhantomData<T>>);

    impl<T: IsHeapRefWithNullInitAllowed> GlobalImpl<T> {
        pub const fn new() -> Self {
            GlobalImpl(UnsafeCell::new(PhantomData))
        }
    }

    /// The type of the `static` containing the actual table.
    #[lang = "wasm_table_ty"]
    // See `GlobalImpl` for why this contains an `UnsafeCell`.
    pub struct TableImpl<T: IsHeapRef>(UnsafeCell<PhantomData<T>>);

    impl<T: IsHeapRef> TableImpl<T> {
        pub const fn new() -> Self {
            TableImpl(UnsafeCell::new(PhantomData))
        }
    }

    extern "rust-intrinsic" {
        pub fn wasm_global_get<T: IsHeapRef>(tbl: &GlobalImpl<T>) -> T;
        pub fn wasm_global_set<T: IsHeapRef>(tbl: &GlobalImpl<T>, val: T);
        pub fn wasm_table_get<T: IsHeapRef>(tbl: &TableImpl<T>, idx: u32) -> T;
        pub fn wasm_table_set<T: IsHeapRef>(tbl: &TableImpl<T>, idx: u32, val: T);
    }

    // The Wasm runtime is expected to synchronize access to globals and tables
    // if necessary.
    unsafe impl<T: IsHeapRef> Sync for GlobalImpl<T> {}
    unsafe impl<T: IsHeapRef> Sync for TableImpl<T> {}
}

#[macro_export]
#[allow_internal_unstable(wasm_heap_types_internals)]
macro_rules! global {
    ($vis:vis $name:ident: $Ty:ty) => {
        $vis struct $name;

        // Items (including `static`s) in macro_rules! are not hygienic, so hide
        // the `static` so it doesn't generate name colissions.
        const _: () = {
            static GLOBAL: $crate::ffi::wasm::internals::GlobalImpl<$Ty> =
                $crate::ffi::wasm::internals::GlobalImpl::new();

            impl $crate::ffi::wasm::Global<$Ty> for $name {

                #[inline]
                fn get(&self) -> $Ty {
                    unsafe {
                        $crate::ffi::wasm::internals::wasm_global_get(&GLOBAL)
                    }
                }

                #[inline]
                fn set(&self, val: $Ty) {
                    unsafe {
                        $crate::ffi::wasm::internals::wasm_global_set(&GLOBAL, val)
                    }
                }
            }
        };
    }
}

#[macro_export]
#[allow_internal_unstable(wasm_heap_types_internals)]
macro_rules! table {
    ($vis:vis $name:ident: $Ty:ty) => {
        $vis struct $name;

        // Items (including `static`s) in macro_rules! are not hygienic, so hide
        // the `static` so it doesn't generate name colissions.
        const _: () = {
            static TABLE: $crate::ffi::wasm::internals::TableImpl<$Ty> =
                $crate::ffi::wasm::internals::TableImpl::new();

            impl $crate::ffi::wasm::Table<$Ty> for $name {

                #[inline]
                fn get(&self, idx: u32) -> $Ty {
                    unsafe {
                        $crate::ffi::wasm::internals::wasm_table_get(&TABLE, idx)
                    }
                }

                #[inline]
                fn set(&self, idx: u32, val: $Ty) {
                    unsafe {
                        $crate::ffi::wasm::internals::wasm_table_set(&TABLE, idx, val)
                    }
                }
            }
        };
    }
}

pub use {global, table};

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
