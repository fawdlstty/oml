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
pub extern "C" fn oml_expr_set_none(pexpr: *mut c_void, ppath: *const c_char) {
    let mut expr = unsafe { Box::from_raw(pexpr as *mut OmlExpr) };
    let path = unsafe { CStr::from_ptr(ppath).to_str().unwrap_or("") };
    expr[path] = OmlExpr::None;
}

#[no_mangle]
pub extern "C" fn oml_expr_set_bool(pexpr: *mut c_void, ppath: *const c_char, value: c_int) {
    let mut expr = unsafe { Box::from_raw(pexpr as *mut OmlExpr) };
    let path = unsafe { CStr::from_ptr(ppath).to_str().unwrap_or("") };
    expr[path] = OmlExpr::Value(OmlValue::Bool(value != 0));
}

#[no_mangle]
pub extern "C" fn oml_expr_set_int(pexpr: *mut c_void, ppath: *const c_char, value: c_longlong) {
    let mut expr = unsafe { Box::from_raw(pexpr as *mut OmlExpr) };
    let path = unsafe { CStr::from_ptr(ppath).to_str().unwrap_or("") };
    expr[path] = OmlExpr::Value(OmlValue::Int64(value));
}

#[no_mangle]
pub extern "C" fn oml_expr_set_float(pexpr: *mut c_void, ppath: *const c_char, value: c_double) {
    let mut expr = unsafe { Box::from_raw(pexpr as *mut OmlExpr) };
    let path = unsafe { CStr::from_ptr(ppath).to_str().unwrap_or("") };
    expr[path] = OmlExpr::Value(OmlValue::Float64(value));
}

#[no_mangle]
pub extern "C" fn oml_expr_set_string(
    pexpr: *mut c_void,
    ppath: *const c_char,
    pvalue: *const c_char,
) {
    let mut expr = unsafe { Box::from_raw(pexpr as *mut OmlExpr) };
    let path = unsafe { CStr::from_ptr(ppath).to_str().unwrap_or("") };
    let value = unsafe { CStr::from_ptr(pvalue).to_str().unwrap_or("") }.to_string();
    expr[path] = OmlExpr::Value(OmlValue::String(value));
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
    let ret = val
        .get_with_path(path)
        .map(|a| a.is_none())
        .unwrap_or(false);
    Box::leak(val);
    ret.as_cint()
}

#[no_mangle]
pub extern "C" fn oml_value_is_bool(pval: *mut c_void, ppath: *const c_char) -> c_int {
    let val = unsafe { Box::from_raw(pval as *mut OmlValue) };
    let path = unsafe { CStr::from_ptr(ppath).to_str().unwrap_or("") };
    let ret = val
        .get_with_path(path)
        .map(|a| a.is_bool())
        .unwrap_or(false);
    Box::leak(val);
    ret.as_cint()
}

#[no_mangle]
pub extern "C" fn oml_value_as_bool(pval: *mut c_void, ppath: *const c_char) -> c_int {
    let val = unsafe { Box::from_raw(pval as *mut OmlValue) };
    let path = unsafe { CStr::from_ptr(ppath).to_str().unwrap_or("") };
    let ret = val
        .get_with_path(path)
        .map(|a| a.as_bool())
        .flatten()
        .unwrap_or(false);
    Box::leak(val);
    ret.as_cint()
}

#[no_mangle]
pub extern "C" fn oml_value_is_int(pval: *mut c_void, ppath: *const c_char) -> c_int {
    let val = unsafe { Box::from_raw(pval as *mut OmlValue) };
    let path = unsafe { CStr::from_ptr(ppath).to_str().unwrap_or("") };
    let ret = val.get_with_path(path).map(|a| a.is_int()).unwrap_or(false);
    Box::leak(val);
    ret.as_cint()
}

#[no_mangle]
pub extern "C" fn oml_value_as_int(pval: *mut c_void, ppath: *const c_char) -> c_longlong {
    let val = unsafe { Box::from_raw(pval as *mut OmlValue) };
    let path = unsafe { CStr::from_ptr(ppath).to_str().unwrap_or("") };
    let ret = val
        .get_with_path(path)
        .map(|a| a.as_int())
        .flatten()
        .unwrap_or(-1);
    Box::leak(val);
    ret
}

#[no_mangle]
pub extern "C" fn oml_value_is_float(pval: *mut c_void, ppath: *const c_char) -> c_int {
    let val = unsafe { Box::from_raw(pval as *mut OmlValue) };
    let path = unsafe { CStr::from_ptr(ppath).to_str().unwrap_or("") };
    let ret = val
        .get_with_path(path)
        .map(|a| a.is_float())
        .unwrap_or(false);
    Box::leak(val);
    ret.as_cint()
}

#[no_mangle]
pub extern "C" fn oml_value_as_float(pval: *mut c_void, ppath: *const c_char) -> c_double {
    let val = unsafe { Box::from_raw(pval as *mut OmlValue) };
    let path = unsafe { CStr::from_ptr(ppath).to_str().unwrap_or("") };
    let ret = val
        .get_with_path(path)
        .map(|a| a.as_float())
        .flatten()
        .unwrap_or(f64::NAN);
    Box::leak(val);
    ret
}

#[no_mangle]
pub extern "C" fn oml_value_is_str(pval: *mut c_void, ppath: *const c_char) -> c_int {
    let val = unsafe { Box::from_raw(pval as *mut OmlValue) };
    let path = unsafe { CStr::from_ptr(ppath).to_str().unwrap_or("") };
    let ret = val.get_with_path(path).map(|a| a.is_str()).unwrap_or(false);
    Box::leak(val);
    ret.as_cint()
}

#[no_mangle]
pub extern "C" fn oml_value_as_str(pval: *mut c_void, ppath: *const c_char) -> *const c_char {
    let val = unsafe { Box::from_raw(pval as *mut OmlValue) };
    let path = unsafe { CStr::from_ptr(ppath).to_str().unwrap_or("") };
    let ret = val
        .get_with_path(path)
        .map(|a| a.as_str())
        .unwrap_or("".to_string());
    let ret = CString::new(ret).unwrap().into_raw();
    Box::leak(val);
    ret
}

#[no_mangle]
pub extern "C" fn oml_value_is_array(pval: *mut c_void, ppath: *const c_char) -> c_int {
    let val = unsafe { Box::from_raw(pval as *mut OmlValue) };
    let path = unsafe { CStr::from_ptr(ppath).to_str().unwrap_or("") };
    let ret = val
        .get_with_path(path)
        .map(|a| a.is_array())
        .unwrap_or(false);
    Box::leak(val);
    ret.as_cint()
}

#[no_mangle]
pub extern "C" fn oml_value_get_array_length(pval: *mut c_void, ppath: *const c_char) -> c_int {
    let val = unsafe { Box::from_raw(pval as *mut OmlValue) };
    let path = unsafe { CStr::from_ptr(ppath).to_str().unwrap_or("") };
    let ret = val
        .get_with_path(path)
        .map(|a| a.as_array().map(|arr| arr.len()))
        .flatten()
        .unwrap_or(0);
    Box::leak(val);
    ret as c_int
}

#[no_mangle]
pub extern "C" fn oml_value_is_map(pval: *mut c_void, ppath: *const c_char) -> c_int {
    let val = unsafe { Box::from_raw(pval as *mut OmlValue) };
    let path = unsafe { CStr::from_ptr(ppath).to_str().unwrap_or("") };
    let ret = val.get_with_path(path).map(|a| a.is_map()).unwrap_or(false);
    Box::leak(val);
    ret.as_cint()
}

#[no_mangle]
pub extern "C" fn oml_value_get_map_length(pval: *mut c_void, ppath: *const c_char) -> c_int {
    let val = unsafe { Box::from_raw(pval as *mut OmlValue) };
    let path = unsafe { CStr::from_ptr(ppath).to_str().unwrap_or("") };
    let ret = val
        .get_with_path(path)
        .map(|a| a.as_map().map(|map| map.len()))
        .flatten()
        .unwrap_or(0);
    Box::leak(val);
    ret as c_int
}

#[no_mangle]
pub extern "C" fn oml_value_get_keys(pval: *mut c_void, ppath: *const c_char) -> *const c_char {
    let val = unsafe { Box::from_raw(pval as *mut OmlValue) };
    let path = unsafe { CStr::from_ptr(ppath).to_str().unwrap_or("") };
    let ret = val
        .get_with_path(path)
        .map(|a| {
            a.as_map().map(|map| {
                let mut keys: Vec<_> = map.keys().map(|a| &a[..]).collect();
                keys.sort();
                CString::new(keys.join("#")).unwrap().into_raw() as *const i8
            })
        })
        .flatten()
        .unwrap_or(std::ptr::null());
    Box::leak(val);
    ret
}

#[no_mangle]
pub extern "C" fn oml_value_set_none(pval: *mut c_void, ppath: *const c_char) -> c_int {
    let mut val = unsafe { Box::from_raw(pval as *mut OmlValue) };
    let path = unsafe { CStr::from_ptr(ppath).to_str().unwrap_or("") };
    val.get_with_path_mut(path)
        .map(|a| *a = OmlValue::None)
        .is_some()
        .as_cint()
}

#[no_mangle]
pub extern "C" fn oml_value_set_bool(pval: *mut c_void, ppath: *const c_char, value: c_int) {
    let mut val = unsafe { Box::from_raw(pval as *mut OmlValue) };
    let path = unsafe { CStr::from_ptr(ppath).to_str().unwrap_or("") };
    val.get_with_path_mut(path)
        .map(|a| *a = OmlValue::Bool(value != 0));
}

#[no_mangle]
pub extern "C" fn oml_value_set_int(pval: *mut c_void, ppath: *const c_char, value: c_longlong) {
    let mut val = unsafe { Box::from_raw(pval as *mut OmlValue) };
    let path = unsafe { CStr::from_ptr(ppath).to_str().unwrap_or("") };
    val.get_with_path_mut(path)
        .map(|a| *a = OmlValue::Int64(value));
}

#[no_mangle]
pub extern "C" fn oml_value_set_float(pval: *mut c_void, ppath: *const c_char, value: c_double) {
    let mut val = unsafe { Box::from_raw(pval as *mut OmlValue) };
    let path = unsafe { CStr::from_ptr(ppath).to_str().unwrap_or("") };
    val.get_with_path_mut(path)
        .map(|a| *a = OmlValue::Float64(value));
}

#[no_mangle]
pub extern "C" fn oml_value_set_string(
    pval: *mut c_void,
    ppath: *const c_char,
    pvalue: *const c_char,
) -> c_int {
    let mut val = unsafe { Box::from_raw(pval as *mut OmlValue) };
    let path = unsafe { CStr::from_ptr(ppath).to_str().unwrap_or("") };
    let value = unsafe { CStr::from_ptr(pvalue).to_str().unwrap_or("") }.to_string();
    val.get_with_path_mut(path)
        .map(|a| *a = OmlValue::String(value))
        .is_some()
        .as_cint()
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
