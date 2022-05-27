//! Unobox the contents of a Julia value.
//!
//! A [`Value`] contains a pointer to some data owned by Julia. The layout of this data depends on
//! its [`DataType`]. It's often possible to provide a type defined in Rust that matches the
//! layout of the data in Julia. For example, if the `DataType` is `Int8`, the pointer points to
//! an `i8`. Extracting the contents of a `Value` is called unboxing.
//!
//! The [`Unbox`] trait defined in this module usually dereferences the pointer. There are a few
//! exceptions to this rule. In particular, unboxing a `char` or a `bool` results in a [`Char`] or
//! a [`Bool`] respectively. The reason is that while using invalid `Char`s and `Bool`s is an
//! error in Julia, it's undefined behavior to create them in Rust. Similarly, strings in Julia
//! should be UTF-8 encoded, but to account for the possibility that the contents are invalid the
//! implementation of `Unbox` returns a `String` if the contents are valid and a `Vec<u8>`
//! otherwise.
//!
//! Unlike [`IntoJulia`], the `Unbox` trait is not limited to bits-types. The only requirement is
//! that the layout of the types in both languages match. Types that can be unboxed include those
//! with pointer fields, type parameters, and bits unions. When wrappers are generated with
//! JlrsReflect.jl [`Unbox`] is always derived.
//!
//! [`Cast`]: crate::convert::cast::Cast
//! [`Bool`]: crate::wrappers::inline::bool::Bool
//! [`Char`]: crate::wrappers::inline::char::Char
//! [`DataType`]: crate::wrappers::ptr::datatype::DataType
//! [`IntoJulia`]: crate::convert::into_julia::IntoJulia

use crate::wrappers::ptr::value::Value;
use jl_sys::{
    jl_unbox_float32, jl_unbox_float64, jl_unbox_int16, jl_unbox_int32, jl_unbox_int64,
    jl_unbox_int8, jl_unbox_uint16, jl_unbox_uint32, jl_unbox_uint64, jl_unbox_uint8,
    jl_unbox_voidpointer,
};
use std::ffi::c_void;

use super::into_julia::IntoJulia;

/// A trait implemented by types that can be extracted from a Julia value in combination with
/// [`Value::unbox`] and [`Value::unbox_unchecked`]. This trait can be derived, it's recommended
/// to use JlrsReflect.jl to ensure it's implemented correctly. All wrappers generated by
/// JlrsReflect.jl will implement this trait and [`ValidLayout`], which checks if the conversion
/// is valid at runtime.
///
/// If you do choose to implement it manually, you only need to provide the associated `Output`
/// type if the type matches the layout of the data in Julia. The default implementation of
/// `unbox` dereferences the value as `&Self::Output` and clones it. If this implementation is
/// incorrect it can be overridden.
///
/// [`Value::unbox`]: crate::wrappers::ptr::value::Value::unbox
/// [`Value::unbox_unchecked`]: crate::wrappers::ptr::value::Value::unbox_unchecked
/// [`ValidLayout`]: crate::layout::valid_layout::ValidLayout
pub unsafe trait Unbox {
    type Output: Sized + Clone;

    /// Unbox the value as `Self::Output`.
    ///
    /// Safety: The default implementation assumes that `Self::Output` is the correct layout for
    /// the data that `value` points to.
    #[inline(always)]
    unsafe fn unbox(value: Value) -> Self::Output {
        value.data_ptr().cast::<Self::Output>().as_ref().clone()
    }
}

macro_rules! impl_unboxer {
    ($type:ty, $unboxer:expr) => {
        unsafe impl Unbox for $type {
            type Output = Self;
            #[inline(always)]
            unsafe fn unbox(value: Value) -> $type {
                $unboxer(
                    <Value as crate::wrappers::ptr::private::WrapperPriv>::unwrap(
                        value,
                        $crate::private::Private,
                    ),
                ) as _
            }
        }
    };
}

impl_unboxer!(u8, jl_unbox_uint8);
impl_unboxer!(u16, jl_unbox_uint16);
impl_unboxer!(u32, jl_unbox_uint32);
impl_unboxer!(u64, jl_unbox_uint64);
impl_unboxer!(i8, jl_unbox_int8);
impl_unboxer!(i16, jl_unbox_int16);
impl_unboxer!(i32, jl_unbox_int32);
impl_unboxer!(i64, jl_unbox_int64);
impl_unboxer!(f32, jl_unbox_float32);
impl_unboxer!(f64, jl_unbox_float64);
impl_unboxer!(*mut c_void, jl_unbox_voidpointer);

#[cfg(not(target_pointer_width = "64"))]
impl_unboxer!(usize, jl_unbox_uint32);

#[cfg(not(target_pointer_width = "64"))]
impl_unboxer!(isize, jl_unbox_int32);

#[cfg(target_pointer_width = "64")]
impl_unboxer!(usize, jl_unbox_uint64);

#[cfg(target_pointer_width = "64")]
impl_unboxer!(isize, jl_unbox_int64);

unsafe impl<T: IntoJulia> Unbox for *mut T {
    type Output = Self;
}
