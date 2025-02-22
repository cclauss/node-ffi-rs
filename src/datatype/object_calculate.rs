use crate::define::RsArgsValue;
use crate::utils::dataprocess::get_js_external_wrap_data;
use indexmap::IndexMap;
use libc::{c_ulonglong, c_void};
use napi::{Env, Result};
use std::alloc::{alloc, Layout};
use std::ffi::CString;
use std::ffi::{c_char, c_double, c_int, c_longlong, c_uchar};

pub fn get_size_align<T: Sized>() -> (usize, usize) {
  (std::mem::size_of::<T>(), std::mem::align_of::<T>())
}

macro_rules! calculate_layout_for {
  ($variant:ident, $type:ty) => {
    fn $variant(size: usize, align: usize, offset: usize) -> (usize, usize, usize) {
      let (type_size, type_align) = get_size_align::<$type>();
      let align = align.max(type_align);
      let padding = (type_align - (offset % type_align)) % type_align;
      let size = size + padding + type_size;
      let offset = offset + padding + type_size;
      (size, align, offset)
    }
  };
}
calculate_layout_for!(calculate_u8, c_uchar);
calculate_layout_for!(calculate_i32, c_int);
calculate_layout_for!(calculate_i64, c_longlong);
calculate_layout_for!(calculate_double, c_double);
calculate_layout_for!(calculate_boolean, bool);
calculate_layout_for!(calculate_void, ());
calculate_layout_for!(calculate_string, *const c_char);
calculate_layout_for!(calculate_pointer, *const c_void);

pub fn calculate_struct_size(map: &IndexMap<String, RsArgsValue>) -> (usize, usize) {
  let (mut size, align, _) =
    map.iter().fold(
      (0, 0, 0),
      |(size, align, offset), (_, field_val)| match field_val {
        RsArgsValue::U8(_) => calculate_u8(size, align, offset),
        RsArgsValue::I32(_) => calculate_i32(size, align, offset),
        RsArgsValue::I64(_) | RsArgsValue::U64(_) => calculate_i64(size, align, offset),
        RsArgsValue::Double(_) => calculate_double(size, align, offset),
        RsArgsValue::String(_) => calculate_string(size, align, offset),
        RsArgsValue::Boolean(_) => calculate_boolean(size, align, offset),
        RsArgsValue::Void(_) => calculate_void(size, align, offset),
        RsArgsValue::Object(_)
        | RsArgsValue::StringArray(_)
        | RsArgsValue::DoubleArray(_)
        | RsArgsValue::I32Array(_)
        | RsArgsValue::U8Array(_, _)
        | RsArgsValue::External(_) => calculate_pointer(size, align, offset),
        RsArgsValue::Function(_, _) => {
          panic!("{:?} calculate_layout error", field_val)
        }
      },
    );
  let padding = if align > 0 && size % align != 0 {
    align - (size % align)
  } else {
    0
  };
  size += padding;
  (size, align)
}

pub unsafe fn generate_c_struct(
  env: &Env,
  map: IndexMap<String, RsArgsValue>,
) -> Result<*mut c_void> {
  let (size, align) = calculate_struct_size(&map);
  let layout = if size > 0 {
    Layout::from_size_align(size, align).unwrap()
  } else {
    Layout::new::<i32>()
  };
  let ptr = alloc(layout) as *mut c_void;
  let mut field_ptr = ptr;
  let mut offset = 0;
  for (_, field_val) in map {
    let field_size = match field_val {
      RsArgsValue::U8(number) => {
        let (size, align) = get_size_align::<c_uchar>();
        let padding = (align - (offset % align)) % align;
        field_ptr = field_ptr.offset(padding as isize);
        (field_ptr as *mut c_uchar).write(number);
        offset += size + padding;
        size
      }
      RsArgsValue::I32(number) => {
        let (size, align) = get_size_align::<c_int>();
        let padding = (align - (offset % align)) % align;
        field_ptr = field_ptr.offset(padding as isize);
        (field_ptr as *mut c_int).write(number);
        offset += size + padding;
        size
      }
      RsArgsValue::I64(number) => {
        let (size, align) = get_size_align::<c_longlong>();
        let padding = (align - (offset % align)) % align;
        field_ptr = field_ptr.offset(padding as isize);
        (field_ptr as *mut c_longlong).write(number);
        offset += size + padding;
        size
      }
      RsArgsValue::U64(number) => {
        let (size, align) = get_size_align::<c_ulonglong>();
        let padding = (align - (offset % align)) % align;
        field_ptr = field_ptr.offset(padding as isize);
        (field_ptr as *mut c_ulonglong).write(number);
        offset += size + padding;
        size
      }
      RsArgsValue::Double(double_number) => {
        let (size, align) = get_size_align::<c_double>();
        let padding = (align - (offset % align)) % align;
        field_ptr = field_ptr.offset(padding as isize);
        (field_ptr as *mut c_double).write(double_number);
        offset += size + padding;
        size
      }
      RsArgsValue::Boolean(val) => {
        let (size, align) = get_size_align::<bool>();
        let padding = (align - (offset % align)) % align;
        field_ptr = field_ptr.offset(padding as isize);
        (field_ptr as *mut bool).write(val);
        offset += size + padding;
        size
      }
      RsArgsValue::String(str) => {
        let (size, align) = get_size_align::<*mut c_void>();
        let padding = (align - (offset % align)) % align;
        field_ptr = field_ptr.offset(padding as isize);
        let c_string = CString::new(str).unwrap();
        (field_ptr as *mut *const c_char).write(c_string.as_ptr());
        std::mem::forget(c_string);
        offset += size + padding;
        size
      }
      RsArgsValue::StringArray(str_arr) => {
        let (size, align) = get_size_align::<*mut c_void>();
        let padding = (align - (offset % align)) % align;
        field_ptr = field_ptr.offset(padding as isize);
        let c_char_vec: Vec<*const c_char> = str_arr
          .into_iter()
          .map(|str| {
            let c_string = CString::new(str).unwrap();
            let ptr = c_string.as_ptr();
            std::mem::forget(c_string);
            return ptr;
          })
          .collect();
        (field_ptr as *mut *const *const c_char).write(c_char_vec.as_ptr());
        std::mem::forget(c_char_vec);
        offset += size + padding;
        size
      }
      RsArgsValue::DoubleArray(arr) => {
        let (size, align) = get_size_align::<*mut c_void>();
        let padding = (align - (offset % align)) % align;
        field_ptr = field_ptr.offset(padding as isize);
        (field_ptr as *mut *const c_double).write(arr.as_ptr());
        std::mem::forget(arr);
        offset += size + padding;
        size
      }
      RsArgsValue::I32Array(arr) => {
        let (size, align) = get_size_align::<*mut c_void>();
        let padding = (align - (offset % align)) % align;
        field_ptr = field_ptr.offset(padding as isize);
        (field_ptr as *mut *const c_int).write(arr.as_ptr());
        std::mem::forget(arr);
        offset += size + padding;
        size
      }
      RsArgsValue::U8Array(buffer, arr) => {
        let buffer = buffer.unwrap();
        let (size, align) = get_size_align::<*mut c_void>();
        let padding = (align - (offset % align)) % align;
        field_ptr = field_ptr.offset(padding as isize);
        (field_ptr as *mut *const c_uchar).write(buffer.as_ptr());
        std::mem::forget(buffer);
        offset += size + padding;
        size
      }
      RsArgsValue::Object(val) => {
        let (size, align) = get_size_align::<*mut c_void>();
        let padding = (align - (offset % align)) % align;
        field_ptr = field_ptr.offset(padding as isize);
        let obj_ptr = generate_c_struct(env, val)?;
        (field_ptr as *mut *const c_void).write(obj_ptr);
        offset += size + padding;
        size
      }
      RsArgsValue::External(val) => {
        let (size, align) = get_size_align::<*mut c_void>();
        let padding = (align - (offset % align)) % align;
        field_ptr = field_ptr.offset(padding as isize);
        (field_ptr as *mut *const c_void).write(get_js_external_wrap_data(&env, val)?);
        offset += size + padding;
        size
      }
      RsArgsValue::Void(_) => {
        let (size, align) = get_size_align::<()>();
        let padding = (align - (offset % align)) % align;
        field_ptr = field_ptr.offset(padding as isize);
        (field_ptr as *mut ()).write(());
        offset += size + padding;
        size
      }
      RsArgsValue::Function(_, _) => panic!("write_data error {:?}", field_val),
    };
    field_ptr = field_ptr.offset(field_size as isize);
  }
  Ok(ptr)
}
