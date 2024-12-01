#[allow(dead_code)]
pub mod exports {
    #[allow(dead_code)]
    pub mod wasi {
        #[allow(dead_code)]
        pub mod cli {
            #[allow(dead_code, clippy::all)]
            pub mod run {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_run_cabi<T: Guest>() -> i32 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let result0 = T::run();
                    let result1 = match result0 {
                        Ok(_) => 0i32,
                        Err(_) => 1i32,
                    };
                    result1
                }
                pub trait Guest {
                    /// Run the program.
                    fn run() -> Result<(), ()>;
                }
                #[doc(hidden)]
                macro_rules! __export_wasi_cli_run_0_2_2_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name = "wasi:cli/run@0.2.2#run"] unsafe
                        extern "C" fn export_run() -> i32 { $($path_to_types)*::
                        _export_run_cabi::<$ty > () } };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_wasi_cli_run_0_2_2_cabi;
            }
        }
    }
}
mod _rt {
    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
    }
}
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
macro_rules! __export_command_impl {
    ($ty:ident) => {
        self::export!($ty with_types_in self);
    };
    ($ty:ident with_types_in $($path_to_types_root:tt)*) => {
        $($path_to_types_root)*::
        exports::wasi::cli::run::__export_wasi_cli_run_0_2_2_cabi!($ty with_types_in
        $($path_to_types_root)*:: exports::wasi::cli::run);
    };
}
#[doc(inline)]
pub(crate) use __export_command_impl as export;
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.31.0:component:nana:command:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 201] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07L\x01A\x02\x01A\x02\x01\
B\x03\x01j\0\0\x01@\0\0\0\x04\0\x03run\x01\x01\x04\x01\x12wasi:cli/run@0.2.2\x05\
\0\x04\x01\x16component:nana/command\x04\0\x0b\x0d\x01\0\x07command\x03\0\0\0G\x09\
producers\x01\x0cprocessed-by\x02\x0dwit-component\x070.216.0\x10wit-bindgen-rus\
t\x060.31.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
