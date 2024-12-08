// Generated by `wit-bindgen` 0.21.0. DO NOT EDIT!
// Options used:
#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn _export_run_cabi<T: Guest>() -> i32 {
    let result0 = T::run();
    match result0 {
        true => 1,
        false => 0,
    }
}
pub trait Guest {
    fn run() -> bool;
}
#[doc(hidden)]

macro_rules! __export_world_file_write_read_delete_cabi{
  ($ty:ident with_types_in $($path_to_types:tt)*) => (const _: () = {

    #[export_name = "run"]
    unsafe extern "C" fn export_run() -> i32 {
      $($path_to_types)*::_export_run_cabi::<$ty>()
    }
  };);
}
#[doc(hidden)]
pub(crate) use __export_world_file_write_read_delete_cabi;

/// Generates `#[no_mangle]` functions to export the specified type as the
/// root implementation of all generated traits.
///
/// For more information see the documentation of `wit_bindgen::generate!`.
///
/// ```rust
/// # macro_rules! export{ ($($t:tt)*) => (); }
/// # trait Guest {}
/// struct MyType;
///
/// impl Guest for MyType {
///     // ...
/// }
///
/// export!(MyType);
/// ```
#[allow(unused_macros)]
#[doc(hidden)]

macro_rules! __export_file_write_read_delete_impl {
  ($ty:ident) => (self::export!($ty with_types_in self););
  ($ty:ident with_types_in $($path_to_types_root:tt)*) => (
  $($path_to_types_root)*::__export_world_file_write_read_delete_cabi!($ty with_types_in $($path_to_types_root)*);
  )
}
#[doc(inline)]
pub(crate) use __export_file_write_read_delete_impl as export;

#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.21.0:file-write-read-delete:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 195] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x077\x01A\x02\x01A\x02\x01\
@\0\0\x7f\x04\0\x03run\x01\0\x04\x01\x1fgolem:it/file-write-read-delete\x04\0\x0b\
\x1c\x01\0\x16file-write-read-delete\x03\0\0\0G\x09producers\x01\x0cprocessed-by\
\x02\x0dwit-component\x070.201.0\x10wit-bindgen-rust\x060.21.0";

#[inline(never)]
#[doc(hidden)]
#[cfg(target_arch = "wasm32")]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}