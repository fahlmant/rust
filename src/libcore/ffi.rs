// Copyright 2018 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![stable(feature = "", since = "1.30.0")]

#![allow(non_camel_case_types)]

//! Utilities related to FFI bindings.

use ::fmt;

/// Equivalent to C's `void` type when used as a [pointer].
///
/// In essence, `*const c_void` is equivalent to C's `const void*`
/// and `*mut c_void` is equivalent to C's `void*`. That said, this is
/// *not* the same as C's `void` return type, which is Rust's `()` type.
///
/// Ideally, this type would be equivalent to [`!`], but currently it may
/// be more ideal to use `c_void` for FFI purposes.
///
/// [`!`]: ../../std/primitive.never.html
/// [pointer]: ../../std/primitive.pointer.html
// NB: For LLVM to recognize the void pointer type and by extension
//     functions like malloc(), we need to have it represented as i8* in
//     LLVM bitcode. The enum used here ensures this and prevents misuse
//     of the "raw" type by only having private variants.. We need two
//     variants, because the compiler complains about the repr attribute
//     otherwise.
#[repr(u8)]
#[stable(feature = "raw_os", since = "1.1.0")]
pub enum c_void {
    #[unstable(feature = "c_void_variant", reason = "should not have to exist",
               issue = "0")]
    #[doc(hidden)] __variant1,
    #[unstable(feature = "c_void_variant", reason = "should not have to exist",
               issue = "0")]
    #[doc(hidden)] __variant2,
}

#[stable(feature = "std_debug", since = "1.16.0")]
impl fmt::Debug for c_void {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.pad("c_void")
    }
}

#[cfg(any(all(not(target_arch = "aarch64"), not(target_arch = "powerpc"),
              not(target_arch = "x86_64"), not(stage0)),
          windows))]
#[unstable(feature = "c_variadic",
           reason = "the `c_variadic` feature has not been properly tested on \
                     all supported platforms",
  issue = "27745")]
/// Basic implementation of a `va_list`.
extern {
    type VaListImpl;
}


#[cfg(any(all(not(target_arch = "aarch64"), not(target_arch = "powerpc"),
              not(target_arch = "x86_64"), not(stage0)),
          windows))]
impl fmt::Debug for VaListImpl {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "va_list* {:p}", self)
    }
}

#[cfg(all(target_arch = "aarch64", not(windows), not(stage0)))]
#[repr(C)]
#[derive(Debug)]
#[unstable(feature = "c_variadic",
           reason = "the `c_variadic` feature has not been properly tested on \
                     all supported platforms",
  issue = "27745")]
/// AArch64 ABI implementation of a `va_list`. See the
/// [Aarch64 Procedure Call Standard] for more details.
///
/// [AArch64 Procedure Call Standard]:
/// http://infocenter.arm.com/help/topic/com.arm.doc.ihi0055b/IHI0055B_aapcs64.pdf
struct VaListImpl {
    stack: *mut (),
    gr_top: *mut (),
    vr_top: *mut (),
    gr_offs: i32,
    vr_offs: i32,
}

#[cfg(all(target_arch = "powerpc", not(windows), not(stage0)))]
#[repr(C)]
#[derive(Debug)]
#[unstable(feature = "c_variadic",
           reason = "the `c_variadic` feature has not been properly tested on \
                     all supported platforms",
  issue = "27745")]
/// PowerPC ABI implementation of a `va_list`.
struct VaListImpl {
    gpr: u8,
    fpr: u8,
    reserved: u16,
    overflow_arg_area: *mut (),
    reg_save_area: *mut (),
}

#[cfg(all(target_arch = "x86_64", not(windows), not(stage0)))]
#[repr(C)]
#[derive(Debug)]
#[unstable(feature = "c_variadic",
           reason = "the `c_variadic` feature has not been properly tested on \
                     all supported platforms",
  issue = "27745")]
/// x86_64 ABI implementation of a `va_list`.
struct VaListImpl {
    gp_offset: i32,
    fp_offset: i32,
    overflow_arg_area: *mut (),
    reg_save_area: *mut (),
}

/// A wrapper for a `va_list`
#[cfg(not(stage0))]
#[lang = "va_list"]
#[derive(Debug)]
#[unstable(feature = "c_variadic",
           reason = "the `c_variadic` feature has not been properly tested on \
                     all supported platforms",
  issue = "27745")]
#[repr(transparent)]
pub struct VaList<'a>(&'a mut VaListImpl);

// The VaArgSafe trait needs to be used in public interfaces, however, the trait
// itself must not be allowed to be used outside this module. Allowing users to
// implement the trait for a new type (thereby allowing the va_arg intrinsic to
// be used on a new type) is likely to cause undefined behavior.
//
// FIXME(dlrobertson): In order to use the VaArgSafe trait in a public interface
// but also ensure it cannot be used elsewhere, the trait needs to be public
// within a private module. Once RFC 2145 has been implemented look into
// improving this.
#[cfg(not(stage0))]
mod sealed_trait {
    /// Trait which whitelists the allowed types to be used with [VaList::arg]
    ///
    /// [VaList::va_arg]: struct.VaList.html#method.arg
    #[unstable(feature = "c_variadic",
               reason = "the `c_variadic` feature has not been properly tested on \
                         all supported platforms",
      issue = "27745")]
    pub trait VaArgSafe {}
}

#[cfg(not(stage0))]
macro_rules! impl_va_arg_safe {
    ($($t:ty),+) => {
        $(
            #[unstable(feature = "c_variadic",
                       reason = "the `c_variadic` feature has not been properly tested on \
                                 all supported platforms",
              issue = "27745")]
            impl sealed_trait::VaArgSafe for $t {}
        )+
    }
}

#[cfg(not(stage0))]
impl_va_arg_safe!{i8, i16, i32, i64, usize}
#[cfg(not(stage0))]
impl_va_arg_safe!{u8, u16, u32, u64, isize}
#[cfg(not(stage0))]
impl_va_arg_safe!{f64}

#[cfg(not(stage0))]
#[unstable(feature = "c_variadic",
           reason = "the `c_variadic` feature has not been properly tested on \
                     all supported platforms",
  issue = "27745")]
impl<T> sealed_trait::VaArgSafe for *mut T {}
#[cfg(not(stage0))]
#[unstable(feature = "c_variadic",
           reason = "the `c_variadic` feature has not been properly tested on \
                     all supported platforms",
  issue = "27745")]
impl<T> sealed_trait::VaArgSafe for *const T {}

#[cfg(not(stage0))]
impl<'a> VaList<'a> {
    #[cfg(any(all(not(target_arch = "aarch64"), not(target_arch = "powerpc"),
                  not(target_arch = "x86_64")),
              windows))]
    unsafe fn to_intrinsic_ptr(&mut self) -> *mut i8 {
        &mut self.0 as *mut _ as *mut i8
    }

    #[cfg(all(any(target_arch = "aarch64", target_arch = "powerpc",
                  target_arch = "x86_64"),
              not(windows)))]
    unsafe fn to_intrinsic_ptr(&mut self) -> *mut i8 {
        self.0 as *mut _ as *mut i8
    }

    /// Advance to the next arg.
    #[unstable(feature = "c_variadic",
               reason = "the `c_variadic` feature has not been properly tested on \
                         all supported platforms",
      issue = "27745")]
    pub unsafe fn arg<T: sealed_trait::VaArgSafe>(&mut self) -> T {
        va_arg(self.to_intrinsic_ptr())
    }

    /// Copy the `va_list` at the current location.
    #[unstable(feature = "c_variadic",
               reason = "the `c_variadic` feature has not been properly tested on \
                         all supported platforms",
      issue = "27745")]
    pub unsafe fn copy<F, R>(&mut self, f: F) -> R
            where F: for<'copy> FnOnce(VaList<'copy>) -> R {
        #[cfg(all(any(target_arch = "aarch64", target_arch = "powerpc",
                      target_arch = "x86_64"),
                  not(windows)))]
        let ap_inner = &mut ::mem::uninitialized::<VaListImpl>();
        #[cfg(any(all(not(target_arch = "aarch64"), not(target_arch = "powerpc"),
                      not(target_arch = "x86_64")),
                  windows))]
        let ap_inner: &mut VaListImpl = &mut *(1 as *mut VaListImpl);
        let mut ap = VaList(ap_inner);
        let ap_ptr = ap.to_intrinsic_ptr();
        va_copy(ap_ptr, self.to_intrinsic_ptr());
        let ret = f(ap);
        va_end(ap_ptr);
        ret
    }
}

#[cfg(not(stage0))]
extern "rust-intrinsic" {
    /// Destroy the arglist `ap` after initialization with `va_start` or
    /// `va_copy`.
    fn va_end(ap: *mut i8);

    /// Copy the current location of arglist `src` to the arglist `dst`.
    fn va_copy(dst: *mut i8, src: *const i8);

    /// Loads an argument of type `T` from the `va_list` `ap` and increment the
    /// argument `ap` points to.
    fn va_arg<T: sealed_trait::VaArgSafe>(ap: *mut i8) -> T;
}
