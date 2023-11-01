#![unstable(feature = "wasm_heap_types_v0", issue = "none")]
#![allow(missing_debug_implementations, missing_docs)]

use crate::marker::PhantomData;
use crate::ptr;

#[lang = "wasm_heap_repr_nullable"]
pub struct Nullable<T: ?Sized>(PhantomData<T>);
#[lang = "wasm_heap_repr_nonnull"]
pub struct NonNull<T: ?Sized>(PhantomData<T>);
#[lang = "wasm_heap_repr_direct"]
pub struct Direct<T: ?Sized>(PhantomData<T>);

pub unsafe trait IsNonNull {}
unsafe impl<T: ?Sized> IsNonNull for NonNull<T> {}

pub unsafe trait IsNonNullOrDirect {}
unsafe impl<T: ?Sized> IsNonNullOrDirect for NonNull<T> {}
unsafe impl<T: ?Sized> IsNonNullOrDirect for Direct<T> {}

pub unsafe trait IsNonNullOrNullable {}
unsafe impl<T: ?Sized> IsNonNullOrNullable for NonNull<T> {}
unsafe impl<T: ?Sized> IsNonNullOrNullable for Nullable<T> {}

pub unsafe trait WasmHeapTypeRepr {
    type Raw: ?Sized;
}

unsafe impl<T: ?Sized> WasmHeapTypeRepr for Nullable<T> {
    type Raw = T;
}
unsafe impl<T: ?Sized> WasmHeapTypeRepr for NonNull<T> {
    type Raw = T;
}
unsafe impl<T: ?Sized> WasmHeapTypeRepr for Direct<T> {
    type Raw = T;
}

pub unsafe trait WasmHeapTypeDescriptor {
    #[lang = "wasm_heap_type_repr"]
    type Repr: WasmHeapTypeRepr;
}

unsafe impl<T: WasmHeapTypeDescriptor + ?Sized> WasmHeapTypeDescriptor for &T {
    type Repr = NonNull<<T::Repr as WasmHeapTypeRepr>::Raw>;
}

unsafe impl<T: WasmHeapTypeDescriptor + ?Sized> WasmHeapTypeDescriptor for &mut T {
    type Repr = NonNull<<T::Repr as WasmHeapTypeRepr>::Raw>;
}

unsafe impl<T: WasmHeapTypeDescriptor + ?Sized> WasmHeapTypeDescriptor for ptr::NonNull<T> {
    type Repr = NonNull<<T::Repr as WasmHeapTypeRepr>::Raw>;
}

unsafe impl<T: WasmHeapTypeDescriptor + ?Sized> WasmHeapTypeDescriptor for *const T
where
    T::Repr: IsNonNullOrDirect,
{
    type Repr = Nullable<<T::Repr as WasmHeapTypeRepr>::Raw>;
}

unsafe impl<T: WasmHeapTypeDescriptor + ?Sized> WasmHeapTypeDescriptor for *mut T
where
    T::Repr: IsNonNullOrDirect,
{
    type Repr = Nullable<<T::Repr as WasmHeapTypeRepr>::Raw>;
}

unsafe impl<T: WasmHeapTypeDescriptor> WasmHeapTypeDescriptor for Option<T>
where
    T::Repr: IsNonNull,
{
    type Repr = Nullable<<T::Repr as WasmHeapTypeRepr>::Raw>;
}

/// Trait that identifies pointers or references into the WebAssembly heap.
#[lang = "wasm_heap_ref"]
pub unsafe trait WasmHeapRef: WasmHeapTypeDescriptor {}

unsafe impl<T: WasmHeapTypeDescriptor> WasmHeapRef for T where T::Repr: IsNonNullOrNullable {}

extern "C" {
    /// The `extern` WebAssembly heap type.
    ///
    /// See [ExternRef].
    #[lang = "wasm_extern_ty"]
    #[unstable(feature = "wasm_heap_types_v1", issue = "none")]
    pub type Extern;
}

unsafe impl WasmHeapTypeDescriptor for Extern {
    type Repr = Direct<Extern>;
}

/// The [`externref`] WebAssembly type.
///
/// A nullable reference to [Extern].
///
/// [`externref`]: https://webassembly.github.io/spec/core/syntax/types.html#syntax-reftype
#[repr(transparent)]
pub struct ExternRef(*const Extern);

unsafe impl WasmHeapTypeDescriptor for ExternRef {
    type Repr = Nullable<Extern>;
}

// FIXME: Consider `extern` support for `Global` and `Table`.

/// Represents a WebAssembly [global].
///
/// Every reference to a global must be *compile time constant*. In Rust,
/// this means that every global is a separate type, defined with the
/// [global!] macro.
///
/// [global]: https://webassembly.github.io/spec/core/syntax/modules.html#syntax-global
pub trait Global<T: WasmHeapRef> {
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
    fn get(&self, idx: u32) -> T;

    fn set(&self, idx: u32, val: T);
}

#[doc(hidden)]
#[unstable(
    feature = "wasm_heap_types_internals",
    issue = "none",
    reason = "permanently unstable implementation details"
)]
pub mod internals {
    use super::WasmHeapRef;
    use crate::marker::PhantomData;

    /// The type of the `static` containing the actual global.
    #[lang = "wasm_global_ty"]
    pub struct GlobalImpl<T: WasmHeapRef>(PhantomData<T>);

    impl<T: WasmHeapRef> GlobalImpl<T> {
        pub const fn new() -> Self {
            GlobalImpl(PhantomData)
        }
    }

    /// The type of the `static` containing the actual table.
    #[lang = "wasm_table_ty"]
    pub struct TableImpl<T: WasmHeapRef>(PhantomData<T>);

    impl<T: WasmHeapRef> TableImpl<T> {
        pub const fn new() -> Self {
            TableImpl(PhantomData)
        }
    }

    // The Wasm runtime is expected to synchronize access to globals and tables
    // if necessary.
    unsafe impl<T: WasmHeapRef> Sync for GlobalImpl<T> {}
    unsafe impl<T: WasmHeapRef> Sync for TableImpl<T> {}

    pub trait NullabilityMarker {}

    pub struct NonNull;
    pub struct Nullable;

    impl NullabilityMarker for NonNull {}
    impl NullabilityMarker for Nullable {}
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
