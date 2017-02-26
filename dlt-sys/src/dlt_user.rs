use libc::{ c_int, c_uint, size_t, c_char, c_uchar, c_void, pthread_t, mqd_t };

use dlt_types::{ DltReturnValue, DltLogLevelType, DltFormatType, DltNetworkTraceType, DltUserLogMode, DltTraceStatusType };
use dlt_common::{ DltBuffer, DltReceiver, DLT_ID_SIZE };

/// Maximum size of each user buffer, also used for injection buffer
pub const DLT_USER_BUF_MAX_SIZE: usize = 1390;
/// Size of resend buffer; Max DLT message size is 1390 bytes plus some extra header space
pub const DLT_USER_RESENDBUF_MAX_SIZE: usize = DLT_USER_BUF_MAX_SIZE + 100;

/// This structure is used for every context used in an application
#[repr(C)]
pub struct DltContext {
    /// Context id
    pub contextID: [c_char; DLT_ID_SIZE],
    /// Offset in user-application context field
    pub log_level_pos: i32,
    /// Pointer to the log level
    pub log_level_ptr: *mut i8,
    /// Pointer to the trace status
    pub trace_status_ptr: *mut i8,
    /// Message counter
    pub mcnt: u8
}

/// This structure is used for context data used in an application
#[repr(C)]
pub struct DltContextData {
    /// Pointer to DltContext
    pub handle: *mut DltContext,
    /// Buffer for building log message
    pub buffer: [c_uchar; DLT_USER_BUF_MAX_SIZE],
    /// Payload size
    pub size: i32,
    /// Log level
    pub log_level: i32,
    /// Trace status
    pub trace_status: i32,
    /// Number of arguments for extended header
    pub args_num: i32,
    /// Description of context
    pub context_description: *mut c_char
}

#[repr(C)]
pub struct DltUserInjectionCallback {
    pub service_id: u32,
    pub injection_callback: extern fn(service_id: u32, data: *mut c_void, length: u32) -> c_int
}

#[repr(C)]
pub struct DltUserLogLevelChangedCallback {
    /// Context ID
    pub contextID: [c_char; DLT_ID_SIZE],
    /// Log level
    pub log_level: i8,
    /// Trace status
    pub trace_status: i8,
    pub log_level_changed_callback: extern fn(context_id: [c_char; DLT_ID_SIZE], log_level: u8, trace_status: u8)
}

/// This structure is used in a table managing all contexts and the corresponding log levels in an application
#[repr(C)]
pub struct dlt_ll_ts_type {
    /// Context ID
    pub contextID: [c_char; DLT_ID_SIZE],
    /// Log level
    pub log_level: i8,
    /// Pointer to the log level
    pub log_level_ptr: *mut i8,
    /// Trace status
    pub trace_status: i8,
    /// Pointer to the trace status
    pub trace_status_ptr: *mut i8,
    /// Description of context
    pub context_description: *mut c_char,
    /// Table with pointers to injection functions and service ids
    pub injection_table: *mut DltUserInjectionCallback,
    pub nrcallbacks: u32,

    /// Log level changed callback
    pub log_level_changed_callback: extern fn(context_id: [c_char; DLT_ID_SIZE], log_level: u8, trace_status: u8)
}

/// This structure holds initial log-level for given appId:ctxId pair
#[repr(C)]
pub struct dlt_env_ll_item {
    pub appId: [c_char; DLT_ID_SIZE],
    pub ctxId: [c_char; DLT_ID_SIZE],
    pub ll:    i8
}

/// This structure holds all initial log-levels given via environment variable DLT_INITIAL_LOG_LEVEL
#[repr(C)]
pub struct dlt_env_ll_set {
    pub item: *mut dlt_env_ll_item,
    pub array_size: size_t,
    pub num_elem: size_t
}

/// This structure is used once for one application.
#[repr(C)]
pub struct DltUser {
    pub ecuID: [c_char; DLT_ID_SIZE],
    pub appID: [c_char; DLT_ID_SIZE],
    pub dlt_log_handle: c_int,
    pub dlt_user_handle: c_int,
    pub dlt_segmented_queue_read_handle: mqd_t,
    pub dlt_segmented_queue_write_handle: mqd_t,
    pub dlt_segmented_nwt_handle: pthread_t,

    pub dlt_is_file: i8,

    pub dlt_ll_ts: *mut dlt_ll_ts_type,
    pub dlt_ll_ts_max_num_entries: u32,
    pub dlt_ll_ts_num_entries: u32,

    pub overflow: i8,
    pub overflow_counter: u32,

    pub application_description: *mut c_char,

    pub receiver: DltReceiver,

    pub verbose_mode: i8,
    pub use_extende_header_for_non_verbose: i8,
    pub with_session_id: i8,
    pub with_timestamp: i8,
    pub with_ecu_id: i8,

    pub enable_local_print: i8,
    pub local_print_mode: i8,

    pub log_state: i8,

    pub startup_buffer: DltBuffer,
    pub resend_buffer: [u8; DLT_USER_RESENDBUF_MAX_SIZE],

    pub timeout_at_exit_handler: u32,
    pub initial_ll_set: dlt_env_ll_set,

    // #ifdef DLT_SHM_ENABLE
    // pub dlt_shm: DltShm,
    // #endif

    // #ifdef DLT_TEST_ENABLE
    // pub corrupt_user_header: c_int,
    // pub corrupt_message_size: c_int,
    // pub corrupt_message_size_size: i16,
    // #endif
}

extern {
    fn dlt_user_log_write_start(handle: *mut DltContext, log: *mut DltContextData, loglevel: DltLogLevelType) -> DltReturnValue;
    fn dlt_user_log_write_start_id(handle: *mut DltContext, log: *mut DltContextData, loglevel: DltLogLevelType, messageid: u32) -> DltReturnValue;
    fn dlt_user_log_write_finish(log: *mut DltContextData) -> DltReturnValue;

    fn dlt_user_log_write_bool(log: *mut DltContextData, data: u8) -> DltReturnValue;

    fn dlt_user_log_write_float32(log: *mut DltContextData, data: f32) -> DltReturnValue;
    fn dlt_user_log_write_float64(log: *mut DltContextData, data: f64) -> DltReturnValue;

    fn dlt_user_log_write_uint(log: *mut DltContextData, data: c_uint) -> DltReturnValue;
    fn dlt_user_log_write_uint8(log: *mut DltContextData, data: u8) -> DltReturnValue;
    fn dlt_user_log_write_uint16(log: *mut DltContextData, data: u16) -> DltReturnValue;
    fn dlt_user_log_write_uint32(log: *mut DltContextData, data: u32) -> DltReturnValue;
    fn dlt_user_log_write_uint64(log: *mut DltContextData, data: u64) -> DltReturnValue;

    fn dlt_user_log_write_uint8_formatted(log: *mut DltContextData, data: u8, _type: DltFormatType) -> DltReturnValue;
    fn dlt_user_log_write_uint16_formatted(log: *mut DltContextData, data: u16, _type: DltFormatType) -> DltReturnValue;
    fn dlt_user_log_write_uint32_formatted(log: *mut DltContextData, data: u32, _type: DltFormatType) -> DltReturnValue;
    fn dlt_user_log_write_uint64_formatted(log: *mut DltContextData, data: u64, _type: DltFormatType) -> DltReturnValue;

    fn dlt_user_log_write_ptr(log: *mut DltContextData, data: *mut c_void) -> DltReturnValue;

    fn dlt_user_log_write_int(log: *mut DltContextData, data: c_int) -> DltReturnValue;
    fn dlt_user_log_write_int8(log: *mut DltContextData, data: i8) -> DltReturnValue;
    fn dlt_user_log_write_int16(log: *mut DltContextData, data: i16) -> DltReturnValue;
    fn dlt_user_log_write_int32(log: *mut DltContextData, data: i32) -> DltReturnValue;
    fn dlt_user_log_write_int64(log: *mut DltContextData, data: i64) -> DltReturnValue;

    fn dlt_user_log_write_string(log: *mut DltContextData, text: *const c_char) -> DltReturnValue;
    fn dlt_user_log_write_constant_string(log: *mut DltContextData, text: *const c_char) -> DltReturnValue;
    fn dlt_user_log_write_utf8_string(log: *mut DltContextData, text: *const c_char) -> DltReturnValue;
    fn dlt_user_log_write_raw(log: *mut DltContextData, data: *mut c_void, length: u16) -> DltReturnValue;
    fn dlt_user_log_write_raw_formatted(log: *mut DltContextData, data: *mut c_void, length: u16, _type: DltFormatType) -> DltReturnValue;

    fn dlt_user_trace_network(handle: *mut DltContext, nw_trace_type: DltNetworkTraceType, header_len: u16, header: *mut c_void, payload_len: u16, payload: *mut c_void) -> DltReturnValue;
    fn dlt_user_trace_network_truncated(handle: *mut DltContext, nw_trace_type: DltNetworkTraceType, header_len: u16, header: *mut c_void, payload_len: u16, payload: *mut c_void, allow_truncate: c_int) -> DltReturnValue;
    fn dlt_user_trace_network_segmented(handle: *mut DltContext, nw_trace_type: DltNetworkTraceType, header_len: u16, header: *mut c_void, payload_len: u16, payload: *mut c_void) -> DltReturnValue;

    //// The following API functions define a high level function interface for DLT
    fn dlt_init() -> DltReturnValue;
    fn dlt_init_file(name: *const c_char) -> DltReturnValue;
    fn dlt_free() -> DltReturnValue;
    fn dlt_check_library_version(user_major_version: *const c_char, user_minor_version: *const c_char) -> DltReturnValue;
    fn dlt_register_app(appid: *const c_char, description: *const c_char) -> DltReturnValue;
    fn dlt_unregister_app() -> DltReturnValue;
    fn dlt_register_context(handle: *mut DltContext, contextid: *const c_char, description: *const c_char) -> DltReturnValue;
    fn dlt_register_context_ll_ts(handle: *mut DltContext, contextid: *const c_char, description: *const c_char, loglevel: c_int, tracestatus: c_int) -> DltReturnValue;
    fn dlt_unregister_context(handle: *mut DltContext) -> DltReturnValue;
    fn dlt_set_resend_timeout_atexit(timeout_in_milliseconds: u32) -> c_int;
    fn dlt_set_log_mode(mode: DltUserLogMode) -> DltReturnValue;
    fn dlt_get_log_state() -> c_int;
    fn dlt_register_injection_callback(handle: *mut DltContext, service_id: u32, dlt_injection_callback: extern fn (_service_id: u32, data: *mut c_void, length: u32) -> c_int) -> DltReturnValue;
    fn dlt_register_log_level_changed_callback(handle: *mut DltContext, dlt_log_level_changed_callback: extern fn (context_id: [c_char; DLT_ID_SIZE], log_level: u8, trace_status: u8)) -> DltReturnValue;
    fn dlt_verbose_mode() -> DltReturnValue;
    fn dlt_user_check_library_version(user_major_version: *const c_char, user_minor_version: *const c_char) -> DltReturnValue;
    fn dlt_nonverbose_mode() -> DltReturnValue;
    fn dlt_use_extended_header_for_non_verbose(use_extende_header_for_non_verbose: i8) -> DltReturnValue;
    fn dlt_with_session_id(with_session_id: i8) -> DltReturnValue;
    fn dlt_with_timestamp(with_timestamp: i8) -> DltReturnValue;
    fn dlt_with_ecu_id(with_ecu_id: i8) -> DltReturnValue;
    fn dlt_set_application_ll_ts_limit(loglevel: DltLogLevelType, tracestatus: DltTraceStatusType) -> DltReturnValue;
    fn dlt_env_adjust_ll_from_env(ll_set: *const dlt_env_ll_set, apid: *const c_char, ctid: *const c_char, ll: c_int) -> c_int;
    fn dlt_env_extract_ll_set(env: *mut *mut c_char, ll_set: *mut dlt_env_ll_set) -> c_int;
    fn dlt_env_free_ll_set(ll_set: *mut dlt_env_ll_set);
    fn dlt_enable_local_print() -> DltReturnValue;
    fn dlt_disable_local_print() -> DltReturnValue;
    fn dlt_log_string(handle: *mut DltContext, loglevel: DltLogLevelType, text: *const c_char) -> DltReturnValue;
    fn dlt_log_string_int(handle: *mut DltContext, loglevel: DltLogLevelType, text: *const c_char, data: c_int) -> DltReturnValue;
    fn dlt_log_string_uint(handle: *mut DltContext, loglevel: DltLogLevelType, text: *const c_char, data: c_uint) -> DltReturnValue;
    fn dlt_log_int(handle: *mut DltContext, loglevel: DltLogLevelType, data: c_int) -> DltReturnValue;
    fn dlt_log_uint(handle: *mut DltContext, loglevel: DltLogLevelType, data: c_uint) -> DltReturnValue;
    fn dlt_log_raw(handle: *mut DltContext, loglevel: DltLogLevelType, data: *mut c_void, length: u16) -> DltReturnValue;
    fn dlt_log_marker() -> DltReturnValue;
    fn dlt_forward_msg(msgdata: *mut c_void, size: size_t) -> DltReturnValue;
    fn dlt_user_check_buffer(total_size: *mut c_int, used_size: *mut c_int) -> DltReturnValue;
    fn dlt_user_atexit_blow_out_user_buffer() -> c_int;
    fn dlt_user_log_resend_buffer() -> DltReturnValue;

// #ifdef DLT_TEST_ENABLE
//void dlt_user_test_corrupt_user_header(int enable);
//void dlt_user_test_corrupt_message_size(int enable,int16_t size);
// #endif
}