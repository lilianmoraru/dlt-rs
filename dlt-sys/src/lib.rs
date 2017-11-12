#![allow(non_camel_case_types, non_snake_case)]

extern crate libc;

pub mod dlt_types;
pub mod dlt_common;
pub mod dlt_shm;
pub mod dlt_user;

pub use dlt_types::*;
pub use dlt_common::*;
pub use dlt_shm::*;
pub use dlt_user::*;

#[test]
fn hello_from_rust() {
    use std::ffi::CString;
    use std::ptr;

    let dlt_major_version = CString::new("2").unwrap();
    let dlt_minor_version = CString::new("17").unwrap();
    unsafe {
        // DLT_REGISTER_APP
        dlt_check_library_version(dlt_major_version.as_ptr(),
                                  dlt_minor_version.as_ptr());
        dlt_register_app(CString::new("RLOG").unwrap().as_ptr(),
                         CString::new("Rust logging").unwrap().as_ptr());

        // DLT_DECLARE_CONTEXT
        let mut testing: DltContext = DltContext {
            contextID: [0; 4],
            log_level_pos: 0,
            log_level_ptr: ptr::null_mut(),
            trace_status_ptr: ptr::null_mut(),
            mcnt: 0
        };

        // DLT_REGISTER_CONTEXT
        dlt_register_context(&mut testing,
                             CString::new("TEST").unwrap().as_ptr(),
                             CString::new("Rusty test").unwrap().as_ptr());

        // DLT_LOG
        if dlt_user_is_logLevel_enabled(&mut testing, DltLogLevelType::DLT_LOG_INFO) == DltReturnValue::DLT_RETURN_TRUE {
            let mut log_local = DltContextData {
                handle: ptr::null_mut(),
                buffer: [0; DLT_USER_BUF_MAX_SIZE],
                size: 0,
                log_level: 0,
                trace_status: 0,
                args_num: 0,
                context_description: ptr::null_mut()
            };

            let dlt_local: libc::c_int = dlt_user_log_write_start(&mut testing, &mut log_local, DltLogLevelType::DLT_LOG_INFO) as i32;
            if dlt_local > 0 {
                // DLT_CSTRING
                dlt_user_log_write_constant_string(&mut log_local,
                                                   CString::new("Hello from Rust").unwrap().as_ptr());
                dlt_user_log_write_finish(&mut log_local);
            }
        }

        // DLT_UNREGISTER_CONTEXT
        dlt_unregister_context(&mut testing);

        // DLT_UNREGISTER_APP
        dlt_unregister_app();
    }
}
