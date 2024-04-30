#[macro_use]
extern crate napi_derive;

use napi::bindgen_prelude::*;

use napi::{Either, Env, JsNumber, JsUnknown};

use libc::c_char;
use libc::malloc;
use libffi_sys::{
  ffi_abi_FFI_DEFAULT_ABI, ffi_call, ffi_cif, ffi_prep_cif, ffi_type, ffi_type_pointer,
  ffi_type_sint32,
};
use libloading::{Library, Symbol};
use std::ffi::c_void;
use std::ffi::CString;

#[napi]
pub enum RetType {
  String,
  I32,
}

pub enum RetTypeStruct {
  String(String),
  I32(i32),
}

#[napi]
pub enum ParamsType {
  String,
  I32,
}

#[napi(object)]
struct FFIParams {
  pub library: String,
  pub func_name: String,
  pub ret_type: RetType,
  pub params_type: Vec<ParamsType>,
  pub params_value: Vec<JsUnknown>,
}

#[napi]
fn load(env: Env, params: FFIParams) -> Either<String, i32> {
  let FFIParams {
    library,
    func_name,
    ret_type,
    params_type,
    params_value,
  } = params;

  unsafe {
    let lib = Library::new(library).unwrap();
    let func: Symbol<unsafe extern "C" fn()> = lib.get(func_name.as_str().as_bytes()).unwrap();

    let (mut arg_types, arg_values): (Vec<*mut ffi_type>, Vec<RetTypeStruct>) = params_type
      .iter()
      .zip(params_value.into_iter())
      .map(|(param, value)| match param {
        ParamsType::I32 => {
          let arg_type = &mut ffi_type_sint32 as *mut ffi_type;
          let arg_val: i32 = value.coerce_to_number().unwrap().try_into().unwrap();
          (arg_type, RetTypeStruct::I32(arg_val))
        }
        ParamsType::String => {
          let arg_type = &mut ffi_type_pointer as *mut ffi_type;
          let arg_val: String = value
            .coerce_to_string()
            .unwrap()
            .into_utf8()
            .unwrap()
            .try_into()
            .unwrap();
          (arg_type, RetTypeStruct::String(arg_val))
        }
      })
      .unzip();
    let mut arg_values: Vec<*mut c_void> = arg_values
      .into_iter()
      .map(|val| match val {
        RetTypeStruct::I32(val) => {
          let c_num = Box::new(val);
          Box::into_raw(c_num) as *mut c_void
        }
        RetTypeStruct::String(val) => {
          let c_string = Box::new(CString::new(val).unwrap());
          Box::into_raw(c_string) as *mut c_void
        }
      })
      .collect();

    let r_type: *mut ffi_type = match ret_type {
      RetType::I32 => &mut ffi_type_sint32 as *mut ffi_type,
      RetType::String => &mut ffi_type_pointer as *mut ffi_type,
    };

    let mut cif = ffi_cif {
      abi: ffi_abi_FFI_DEFAULT_ABI,
      nargs: params_type.len() as u32,
      arg_types: arg_types.as_mut_ptr(),
      rtype: r_type,
      bytes: 0,
      flags: 0,
      #[cfg(all(target_arch = "aarch64", target_vendor = "apple"))]
      aarch64_nfixedargs: 0,
    };

    ffi_prep_cif(
      &mut cif as *mut ffi_cif,
      ffi_abi_FFI_DEFAULT_ABI,
      params_type.len() as u32,
      r_type,
      arg_types.as_mut_ptr(),
    );

    match ret_type {
      RetType::String => {
        let mut result: *mut c_char = malloc(std::mem::size_of::<*mut c_char>()) as *mut c_char;
        ffi_call(
          &mut cif,
          Some(*func),
          &mut result as *mut *mut c_char as *mut c_void,
          arg_values.as_mut_ptr(),
        );

        let result_str = CString::from_raw(result).into_string().unwrap();
        Either::A(result_str)
      }
      RetType::I32 => {
        let mut result: i32 = 0;
        ffi_call(
          &mut cif,
          Some(*func),
          &mut result as *mut i32 as *mut c_void,
          arg_values.as_mut_ptr(),
        );
        Either::B(result)
      }
    }
  }
}
