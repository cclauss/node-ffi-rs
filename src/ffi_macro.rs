macro_rules! match_args_len {
    ($args_len:ident, $func_args_type:expr, $js_function:expr, $env:expr, $($num:literal => $closure:ident, $($arg:ident),*),*) => {
        match $args_len {
            $(
                $num => {
                    let lambda = |$($arg: *mut c_void),*| {
                        let arg_arr = [$($arg),*];
                        let value: Vec<JsUnknown> = (0..$num)
                            .map(|index| {
                                let c_param = arg_arr[index as usize];
                                let arg_type = ($func_args_type).get_element::<JsUnknown>(index).unwrap();
                                let param = get_js_function_call_value($env, arg_type, c_param);
                                param
                            })
                            .collect();
                        $js_function.call(None, &value).unwrap();
                    };
                    let closure = Box::into_raw(Box::new($closure::new(&lambda)));
                    return std::mem::transmute((*closure).code_ptr());
                }
            )*
            _ => {
                std::ptr::null_mut() as *mut c_void
            },
        }
    };
}
