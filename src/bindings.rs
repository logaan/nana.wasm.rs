#[allow(dead_code)]
pub mod component {
    #[allow(dead_code)]
    pub mod nana {
        #[allow(dead_code, clippy::all)]
        pub mod greeter {
            #[used]
            #[doc(hidden)]
            static __FORCE_SECTION_REF: fn() = super::super::super::__link_custom_section_describing_imports;
            use super::super::super::_rt;
            #[allow(unused_unsafe, clippy::all)]
            pub fn greet(name: &str) -> _rt::String {
                unsafe {
                    #[repr(align(4))]
                    struct RetArea([::core::mem::MaybeUninit<u8>; 8]);
                    let mut ret_area = RetArea([::core::mem::MaybeUninit::uninit(); 8]);
                    let vec0 = name;
                    let ptr0 = vec0.as_ptr().cast::<u8>();
                    let len0 = vec0.len();
                    let ptr1 = ret_area.0.as_mut_ptr().cast::<u8>();
                    #[cfg(target_arch = "wasm32")]
                    #[link(wasm_import_module = "component:nana/greeter")]
                    extern "C" {
                        #[link_name = "greet"]
                        fn wit_import(_: *mut u8, _: usize, _: *mut u8);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    fn wit_import(_: *mut u8, _: usize, _: *mut u8) {
                        unreachable!()
                    }
                    wit_import(ptr0.cast_mut(), len0, ptr1);
                    let l2 = *ptr1.add(0).cast::<*mut u8>();
                    let l3 = *ptr1.add(4).cast::<usize>();
                    let len4 = l3;
                    let bytes4 = _rt::Vec::from_raw_parts(l2.cast(), len4, len4);
                    _rt::string_lift(bytes4)
                }
            }
        }
    }
}
#[allow(dead_code)]
pub mod exports {
    #[allow(dead_code)]
    pub mod component {
        #[allow(dead_code)]
        pub mod nana {
            #[allow(dead_code, clippy::all)]
            pub mod greeter {
                #[used]
                #[doc(hidden)]
                static __FORCE_SECTION_REF: fn() = super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_greet_cabi<T: Guest>(
                    arg0: *mut u8,
                    arg1: usize,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")] _rt::run_ctors_once();
                    let len0 = arg1;
                    let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
                    let result1 = T::greet(_rt::string_lift(bytes0));
                    let ptr2 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let vec3 = (result1.into_bytes()).into_boxed_slice();
                    let ptr3 = vec3.as_ptr().cast::<u8>();
                    let len3 = vec3.len();
                    ::core::mem::forget(vec3);
                    *ptr2.add(4).cast::<usize>() = len3;
                    *ptr2.add(0).cast::<*mut u8>() = ptr3.cast_mut();
                    ptr2
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_greet<T: Guest>(arg0: *mut u8) {
                    let l0 = *arg0.add(0).cast::<*mut u8>();
                    let l1 = *arg0.add(4).cast::<usize>();
                    _rt::cabi_dealloc(l0, l1, 1);
                }
                pub trait Guest {
                    fn greet(name: _rt::String) -> _rt::String;
                }
                #[doc(hidden)]
                macro_rules! __export_component_nana_greeter_cabi {
                    ($ty:ident with_types_in $($path_to_types:tt)*) => {
                        const _ : () = { #[export_name = "component:nana/greeter#greet"]
                        unsafe extern "C" fn export_greet(arg0 : * mut u8, arg1 : usize,)
                        -> * mut u8 { $($path_to_types)*:: _export_greet_cabi::<$ty >
                        (arg0, arg1) } #[export_name =
                        "cabi_post_component:nana/greeter#greet"] unsafe extern "C" fn
                        _post_return_greet(arg0 : * mut u8,) { $($path_to_types)*::
                        __post_return_greet::<$ty > (arg0) } };
                    };
                }
                #[doc(hidden)]
                pub(crate) use __export_component_nana_greeter_cabi;
                #[repr(align(4))]
                struct _RetArea([::core::mem::MaybeUninit<u8>; 8]);
                static mut _RET_AREA: _RetArea = _RetArea(
                    [::core::mem::MaybeUninit::uninit(); 8],
                );
            }
        }
    }
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
    pub use alloc_crate::string::String;
    pub use alloc_crate::vec::Vec;
    pub unsafe fn string_lift(bytes: Vec<u8>) -> String {
        if cfg!(debug_assertions) {
            String::from_utf8(bytes).unwrap()
        } else {
            String::from_utf8_unchecked(bytes)
        }
    }
    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
    }
    pub unsafe fn cabi_dealloc(ptr: *mut u8, size: usize, align: usize) {
        if size == 0 {
            return;
        }
        let layout = alloc::Layout::from_size_align_unchecked(size, align);
        alloc::dealloc(ptr, layout);
    }
    extern crate alloc as alloc_crate;
    pub use alloc_crate::alloc;
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
        $($path_to_types_root)*:: exports::wasi::cli::run); $($path_to_types_root)*::
        exports::component::nana::greeter::__export_component_nana_greeter_cabi!($ty
        with_types_in $($path_to_types_root)*:: exports::component::nana::greeter);
    };
}
#[doc(inline)]
pub(crate) use __export_command_impl as export;
#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.31.0:component:nana:command:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 304] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\xb2\x01\x01A\x02\x01\
A\x06\x01B\x02\x01@\x01\x04names\0s\x04\0\x05greet\x01\0\x03\x01\x16component:na\
na/greeter\x05\0\x01B\x03\x01j\0\0\x01@\0\0\0\x04\0\x03run\x01\x01\x04\x01\x12wa\
si:cli/run@0.2.2\x05\x01\x01B\x02\x01@\x01\x04names\0s\x04\0\x05greet\x01\0\x04\x01\
\x16component:nana/greeter\x05\x02\x04\x01\x16component:nana/command\x04\0\x0b\x0d\
\x01\0\x07command\x03\0\0\0G\x09producers\x01\x0cprocessed-by\x02\x0dwit-compone\
nt\x070.216.0\x10wit-bindgen-rust\x060.31.0";
#[inline(never)]
#[doc(hidden)]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
