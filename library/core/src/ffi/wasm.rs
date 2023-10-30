#![unstable(feature = "wasm_heap_types", issue = "none")]
#![allow(missing_debug_implementations, missing_docs)]

use crate::marker::PhantomData;

// FIXME: "error[E0198]: negative impls cannot be unsafe" seems wrong for
// these traits. Potential workaround: for every public marker, create a
// private mirror trait.

/// Marker trait applied to types that live on the WebAssembly heap.
#[cfg_attr(not(bootstrap), lang = "wasm_heap_ty")]
#[unstable(
    feature = "wasm_heap_types_internals",
    issue = "none",
    reason = "permanently unstable implementation detail"
)]
unsafe auto trait WasmHeapType {}

/// Marker trait applied to pointers or references into the WebAssembly
/// heap.
#[cfg_attr(not(bootstrap), lang = "wasm_heap_ref")]
pub unsafe auto trait WasmHeapRef {}

unsafe impl<T: WasmHeapType + ?Sized> WasmHeapRef for *const T {}
unsafe impl<T: WasmHeapType + ?Sized> WasmHeapRef for *mut T {}
unsafe impl<T: WasmHeapType + ?Sized> WasmHeapRef for &T {}
unsafe impl<T: WasmHeapType + ?Sized> WasmHeapRef for &mut T {}

// These marker traits primarily affect a type's layout computation, so
// should not see through `PhantomData`.
impl<T: ?Sized> !WasmHeapType for PhantomData<T> {}
impl<T: ?Sized> !WasmHeapRef for PhantomData<T> {}

extern "C" {
    /// The `extern` WebAssembly heap type.
    ///
    /// See [ExternRef].
    #[cfg_attr(not(bootstrap), lang = "wasm_extern_ty")]
    pub type Extern;
}

unsafe impl WasmHeapType for Extern {}

/// The [`externref`] WebAssembly type.
///
/// A nullable reference to [Extern].
///
/// [`externref`]: https://webassembly.github.io/spec/core/syntax/types.html#syntax-reftype
pub struct ExternRef(*const Extern);

// FIXME: Consider `extern` support for `Global` and `Table`.

// FIXME: Consider an `Index` implementation for `Table`.

/// Represents a WebAssembly [global].
///
/// Every reference to a global must be *compile time constant*. In Rust,
/// this means that every global is a separate type, defined with the
/// [global!] macro.
///
/// [global]: https://webassembly.github.io/spec/core/syntax/modules.html#syntax-global
pub trait Global<T: WasmHeapRef> {
    fn instance() -> Self;

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
pub trait Table<T: WasmHeapRef> {
    fn instance() -> Self;

    fn get(&self, idx: u32) -> T;

    fn set(&self, idx: u32, val: T);
}

#[doc(hidden)]
#[unstable(
    feature = "wasm_heap_types_internals",
    issue = "none",
    reason = "permanently unstable implementation detail"
)]
pub mod internals {
    use super::WasmHeapRef;
    use crate::marker::PhantomData;

    /// The type of the `static` containing the actual global.
    #[cfg_attr(not(bootstrap), lang = "wasm_global_ty")]
    pub struct GlobalImpl<T: WasmHeapRef>(PhantomData<T>);

    impl<T: WasmHeapRef> GlobalImpl<T> {
        pub const fn new() -> Self {
            GlobalImpl(PhantomData)
        }
    }

    /// The type of the `static` containing the actual table.
    #[cfg_attr(not(bootstrap), lang = "wasm_table_ty")]
    pub struct TableImpl<T: WasmHeapRef>(PhantomData<T>);

    impl<T: WasmHeapRef> TableImpl<T> {
        pub const fn new() -> Self {
            TableImpl(PhantomData)
        }
    }

    // The Wasm runtime is expected to synchronize access globals and tables if
    // necessary.
    unsafe impl<T: WasmHeapRef> Sync for GlobalImpl<T> {}
    unsafe impl<T: WasmHeapRef> Sync for TableImpl<T> {}
}

#[macro_export]
#[allow_internal_unstable(wasm_heap_types_internals)]
macro_rules! global {
    ($vis:vis $name:ident: $Ty:ty) => {
        static GLOBAL: $crate::ffi::wasm::internals::GlobalImpl<$Ty> =
            $crate::ffi::wasm::internals::GlobalImpl::new();

        $vis struct $name;

        impl $crate::ffi::wasm::Global<$Ty> for $name {
            #[inline]
            fn instance() -> Self { let _ = &GLOBAL; todo!(); }

            #[inline]
            fn get(&self) -> $Ty { let _ = &GLOBAL; todo!(); }

            #[inline]
            fn set(&self, _val: $Ty) { let _ = &GLOBAL; todo!(); }
        }
    }
}

#[macro_export]
#[allow_internal_unstable(wasm_heap_types_internals)]
macro_rules! table {
    ($vis:vis $name:ident: $Ty:ty) => {
        static TABLE: $crate::ffi::wasm::internals::TableImpl<$Ty> =
            $crate::ffi::wasm::internals::TableImpl::new();

        $vis struct $name;

        impl $crate::ffi::wasm::Table<$Ty> for $name {
            #[inline]
            fn instance() -> Self { let _ = &GLOBAL; todo!(); }

            #[inline]
            fn get(&self, _idx: u32) -> $Ty { let _ = &TABLE; todo!(); }

            #[inline]
            fn set(&self, _idx: u32, _val: $Ty) { let _ = &TABLE; todo!(); }
        }
    }
}
