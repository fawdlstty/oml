use crate::{OmlExpr, OmlValue};
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_double, c_int, c_longlong, c_void};

trait AsCInt {
    fn as_cint(&self) -> c_int;
}

impl AsCInt for bool {
    fn as_cint(&self) -> c_int {
        if *self {
            1
        } else {
            0
        }
    }
}

/// Try parse string and get oml-expr pointer
#[no_mangle]
pub extern "C" fn oml_expr_from_str(
    psrc: *const c_char,
    ppexpr: *mut *mut c_void,
    pperr: *mut *const c_char,
) -> c_int {
    let src = unsafe { CStr::from_ptr(psrc).to_str().unwrap_or("") };
    match OmlExpr::from_str(src) {
        Ok(root) => {
            unsafe { *ppexpr = Box::leak(Box::new(root)) as *mut OmlExpr as *mut c_void };
            unsafe { *pperr = std::ptr::null_mut() };
            true.as_cint()
        }
        Err(err) => {
            unsafe { *ppexpr = std::ptr::null_mut() };
            unsafe { *pperr = CString::new(err).unwrap().into_raw() };
            false.as_cint()
        }
    }
}

#[no_mangle]
pub extern "C" fn oml_expr_evalute(
    pexpr: *mut c_void,
    ppath: *const c_char,
    ppval: *mut *mut c_void,
    pperr: *mut *const c_char,
) -> c_int {
    let expr = unsafe { Box::from_raw(pexpr as *mut OmlExpr) };
    let path = unsafe { CStr::from_ptr(ppath).to_str().unwrap_or("") };
    let ret = match expr[path].evalute() {
        Ok(root) => {
            unsafe { *ppval = Box::leak(Box::new(root)) as *mut OmlValue as *mut c_void };
            unsafe { *pperr = std::ptr::null_mut() };
            true
        }
        Err(err) => {
            unsafe { *ppval = std::ptr::null_mut() };
            unsafe { *pperr = CString::new(err).unwrap().into_raw() };
            false
        }
    };
    Box::leak(expr);
    ret.as_cint()
}

#[no_mangle]
pub extern "C" fn oml_value_is_none(pval: *mut c_void, ppath: *const c_char) -> c_int {
    let val = unsafe { Box::from_raw(pval as *mut OmlValue) };
    let path = unsafe { CStr::from_ptr(ppath).to_str().unwrap_or("") };
    let ret = val[path].is_none();
    Box::leak(val);
    ret.as_cint()
}

#[no_mangle]
pub extern "C" fn oml_value_is_bool(pval: *mut c_void, ppath: *const c_char) -> c_int {
    let val = unsafe { Box::from_raw(pval as *mut OmlValue) };
    let path = unsafe { CStr::from_ptr(ppath).to_str().unwrap_or("") };
    let ret = val[path].is_bool();
    Box::leak(val);
    ret.as_cint()
}

#[no_mangle]
pub extern "C" fn oml_value_as_bool(pval: *mut c_void, ppath: *const c_char) -> c_int {
    let val = unsafe { Box::from_raw(pval as *mut OmlValue) };
    let path = unsafe { CStr::from_ptr(ppath).to_str().unwrap_or("") };
    let ret = val[path].as_bool().unwrap_or(false);
    Box::leak(val);
    ret.as_cint()
}

#[no_mangle]
pub extern "C" fn oml_value_is_int(pval: *mut c_void, ppath: *const c_char) -> c_int {
    let val = unsafe { Box::from_raw(pval as *mut OmlValue) };
    let path = unsafe { CStr::from_ptr(ppath).to_str().unwrap_or("") };
    let ret = val[path].is_int();
    Box::leak(val);
    ret.as_cint()
}

#[no_mangle]
pub extern "C" fn oml_value_as_int(pval: *mut c_void, ppath: *const c_char) -> c_longlong {
    let val = unsafe { Box::from_raw(pval as *mut OmlValue) };
    let path = unsafe { CStr::from_ptr(ppath).to_str().unwrap_or("") };
    let ret = val[path].as_int().unwrap_or(0);
    Box::leak(val);
    ret
}

#[no_mangle]
pub extern "C" fn oml_value_is_float(pval: *mut c_void, ppath: *const c_char) -> c_int {
    let val = unsafe { Box::from_raw(pval as *mut OmlValue) };
    let path = unsafe { CStr::from_ptr(ppath).to_str().unwrap_or("") };
    let ret = val[path].is_float();
    Box::leak(val);
    ret.as_cint()
}

#[no_mangle]
pub extern "C" fn oml_value_as_float(pval: *mut c_void, ppath: *const c_char) -> c_double {
    let val = unsafe { Box::from_raw(pval as *mut OmlValue) };
    let path = unsafe { CStr::from_ptr(ppath).to_str().unwrap_or("") };
    let ret = val[path].as_float().unwrap_or(0.0);
    Box::leak(val);
    ret
}

#[no_mangle]
pub extern "C" fn oml_value_is_str(pval: *mut c_void, ppath: *const c_char) -> c_int {
    let val = unsafe { Box::from_raw(pval as *mut OmlValue) };
    let path = unsafe { CStr::from_ptr(ppath).to_str().unwrap_or("") };
    let ret = val[path].is_str();
    Box::leak(val);
    ret.as_cint()
}

#[no_mangle]
pub extern "C" fn oml_value_as_str(pval: *mut c_void, ppath: *const c_char) -> *const c_char {
    let val = unsafe { Box::from_raw(pval as *mut OmlValue) };
    let path = unsafe { CStr::from_ptr(ppath).to_str().unwrap_or("") };
    let ret = val[path].as_str();
    let ret = CString::new(ret).unwrap().into_raw();
    Box::leak(val);
    ret
}

#[no_mangle]
pub extern "C" fn oml_value_is_array(pval: *mut c_void, ppath: *const c_char) -> c_int {
    let val = unsafe { Box::from_raw(pval as *mut OmlValue) };
    let path = unsafe { CStr::from_ptr(ppath).to_str().unwrap_or("") };
    let ret = val[path].is_array();
    Box::leak(val);
    ret.as_cint()
}

#[no_mangle]
pub extern "C" fn oml_value_get_array_length(pval: *mut c_void, ppath: *const c_char) -> c_int {
    let val = unsafe { Box::from_raw(pval as *mut OmlValue) };
    let path = unsafe { CStr::from_ptr(ppath).to_str().unwrap_or("") };
    let ret = val[path].as_array().map(|arr| arr.len()).unwrap_or(0);
    Box::leak(val);
    ret as c_int
}

#[no_mangle]
pub extern "C" fn oml_value_is_map(pval: *mut c_void, ppath: *const c_char) -> c_int {
    let val = unsafe { Box::from_raw(pval as *mut OmlValue) };
    let path = unsafe { CStr::from_ptr(ppath).to_str().unwrap_or("") };
    let ret = val[path].is_map();
    Box::leak(val);
    ret.as_cint()
}

#[no_mangle]
pub extern "C" fn oml_value_get_map_length(pval: *mut c_void, ppath: *const c_char) -> c_int {
    let val = unsafe { Box::from_raw(pval as *mut OmlValue) };
    let path = unsafe { CStr::from_ptr(ppath).to_str().unwrap_or("") };
    let ret = val[path].as_map().map(|arr| arr.len()).unwrap_or(0);
    Box::leak(val);
    ret as c_int
}

#[no_mangle]
pub extern "C" fn oml_value_get_map_key(
    pval: *mut c_void,
    ppath: *const c_char,
    index: c_int,
) -> *const c_char {
    let val = unsafe { Box::from_raw(pval as *mut OmlValue) };
    let path = unsafe { CStr::from_ptr(ppath).to_str().unwrap_or("") };
    let ret = match val[path].as_map() {
        Some(map) => {
            let mut keys: Vec<_> = map.keys().collect();
            keys.sort();
            match keys.get(index as usize) {
                Some(ret) => CString::new((*ret).clone()).unwrap().into_raw(),
                None => std::ptr::null(),
            }
        }
        None => std::ptr::null(),
    };
    Box::leak(val);
    ret
}

#[no_mangle]
pub extern "C" fn oml_release_expr(pexpr: *const c_void) {
    if !pexpr.is_null() {
        _ = unsafe { Box::from_raw(pexpr as *mut OmlExpr) };
    }
}

#[no_mangle]
pub extern "C" fn oml_release_value(pval: *const c_void) {
    if !pval.is_null() {
        _ = unsafe { Box::from_raw(pval as *mut OmlValue) };
    }
}

#[no_mangle]
pub extern "C" fn oml_release_str(pstr: *const c_char) {
    if !pstr.is_null() {
        _ = unsafe { CString::from_raw(pstr as *mut c_char) };
    }
}
