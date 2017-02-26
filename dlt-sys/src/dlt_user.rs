use libc::{ c_int, size_t, c_char, c_uchar, c_void, pthread_t, mqd_t };

use dlt_types::DltReturnValue;
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
    pub injection_callback: fn(service_id: u32, data: *mut c_void, length: u32) -> c_int
}

#[repr(C)]
pub struct DltUserLogLevelChangedCallback {
    /// Context ID
    pub contextID: [c_char; DLT_ID_SIZE],
    /// Log level
    pub log_level: i8,
    /// Trace status
    pub trace_status: i8,
    pub log_level_changed_callback: fn(context_id: [c_char; DLT_ID_SIZE], log_level: u8, trace_status: u8)
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
    pub log_level_changed_callback: fn(context_id: [c_char; DLT_ID_SIZE], log_level: u8, trace_status: u8)
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
//DltReturnValue dlt_user_log_write_start(DltContext *handle, DltContextData *log, DltLogLevelType loglevel);
//DltReturnValue dlt_user_log_write_start_id(DltContext *handle, DltContextData *log, DltLogLevelType loglevel, uint32_t messageid);
//DltReturnValue dlt_user_log_write_finish(DltContextData *log);
//
//DltReturnValue dlt_user_log_write_bool(DltContextData *log, uint8_t data);
//
//DltReturnValue dlt_user_log_write_float32(DltContextData *log, float32_t data);
//DltReturnValue dlt_user_log_write_float64(DltContextData *log, double data);
//
//DltReturnValue dlt_user_log_write_uint(DltContextData *log, unsigned int data);
//DltReturnValue dlt_user_log_write_uint8(DltContextData *log, uint8_t data);
//DltReturnValue dlt_user_log_write_uint16(DltContextData *log, uint16_t data);
//DltReturnValue dlt_user_log_write_uint32(DltContextData *log, uint32_t data);
//DltReturnValue dlt_user_log_write_uint64(DltContextData *log, uint64_t data);
//
//DltReturnValue dlt_user_log_write_uint8_formatted(DltContextData *log, uint8_t data, DltFormatType type);
//DltReturnValue dlt_user_log_write_uint16_formatted(DltContextData *log, uint16_t data, DltFormatType type);
//DltReturnValue dlt_user_log_write_uint32_formatted(DltContextData *log, uint32_t data, DltFormatType type);
//DltReturnValue dlt_user_log_write_uint64_formatted(DltContextData *log, uint64_t data, DltFormatType type);
//
//DltReturnValue dlt_user_log_write_ptr(DltContextData *log, void *data);
//DltReturnValue dlt_user_log_write_int(DltContextData *log, int data);
//DltReturnValue dlt_user_log_write_int8(DltContextData *log, int8_t data);
//DltReturnValue dlt_user_log_write_int16(DltContextData *log, int16_t data);
//DltReturnValue dlt_user_log_write_int32(DltContextData *log, int32_t data);
//DltReturnValue dlt_user_log_write_int64(DltContextData *log, int64_t data);
//
//DltReturnValue dlt_user_log_write_string( DltContextData *log, const char *text);
//DltReturnValue dlt_user_log_write_constant_string( DltContextData *log, const char *text);
//DltReturnValue dlt_user_log_write_utf8_string(DltContextData *log, const char *text);
//DltReturnValue dlt_user_log_write_raw(DltContextData *log,void *data,uint16_t length);
//DltReturnValue dlt_user_log_write_raw_formatted(DltContextData *log,void *data,uint16_t length,DltFormatType type);
//DltReturnValue dlt_user_trace_network(DltContext *handle, DltNetworkTraceType nw_trace_type, uint16_t header_len, void *header, uint16_t payload_len, void *payload);
//DltReturnValue dlt_user_trace_network_truncated(DltContext *handle, DltNetworkTraceType nw_trace_type, uint16_t header_len, void *header, uint16_t payload_len, void *payload, int allow_truncate);
//DltReturnValue dlt_user_trace_network_segmented(DltContext *handle, DltNetworkTraceType nw_trace_type, uint16_t header_len, void *header, uint16_t payload_len, void *payload);
//
//// The following API functions define a high level function interface for DLT
//DltReturnValue dlt_init();
//DltReturnValue dlt_init_file(const char *name);
//DltReturnValue dlt_free();
//DltReturnValue dlt_check_library_version(const char * user_major_version, const char * user_minor_version);
//DltReturnValue dlt_register_app(const char *appid, const char * description);
//DltReturnValue dlt_unregister_app(void);
//DltReturnValue dlt_register_context(DltContext *handle, const char *contextid, const char * description);
//DltReturnValue dlt_register_context_ll_ts(DltContext *handle, const char *contextid, const char * description, int loglevel, int tracestatus);
//DltReturnValue dlt_unregister_context(DltContext *handle);
//int dlt_set_resend_timeout_atexit(uint32_t timeout_in_milliseconds);
//DltReturnValue dlt_set_log_mode(DltUserLogMode mode);
//int dlt_get_log_state();
//DltReturnValue dlt_register_injection_callback(DltContext *handle, uint32_t service_id,
//int (*dlt_injection_callback)(uint32_t service_id, void *data, uint32_t length));
//DltReturnValue dlt_register_log_level_changed_callback(DltContext *handle,
//void (*dlt_log_level_changed_callback)(char context_id[DLT_ID_SIZE],uint8_t log_level, uint8_t trace_status));
//DltReturnValue dlt_verbose_mode(void);
//DltReturnValue dlt_user_check_library_version(const char *user_major_version,const char *user_minor_version);
//DltReturnValue dlt_nonverbose_mode(void);
//DltReturnValue dlt_use_extended_header_for_non_verbose(int8_t use_extende_header_for_non_verbose);
//DltReturnValue dlt_with_session_id(int8_t with_session_id);
//DltReturnValue dlt_with_timestamp(int8_t with_timestamp);
//DltReturnValue dlt_with_ecu_id(int8_t with_ecu_id);
//DltReturnValue dlt_set_application_ll_ts_limit(DltLogLevelType loglevel, DltTraceStatusType tracestatus);
//int dlt_env_adjust_ll_from_env(dlt_env_ll_set const * const ll_set, char const * const apid, char const * const ctid, int const ll);
//int dlt_env_extract_ll_set(char ** const env, dlt_env_ll_set * const ll_set);
//void dlt_env_free_ll_set(dlt_env_ll_set * const ll_set);
//DltReturnValue dlt_enable_local_print(void);
//DltReturnValue dlt_disable_local_print(void);
//DltReturnValue dlt_log_string(DltContext *handle,DltLogLevelType loglevel, const char *text);
//DltReturnValue dlt_log_string_int(DltContext *handle,DltLogLevelType loglevel, const char *text, int data);
//DltReturnValue dlt_log_string_uint(DltContext *handle,DltLogLevelType loglevel, const char *text, unsigned int data);
//DltReturnValue dlt_log_int(DltContext *handle,DltLogLevelType loglevel, int data);
//DltReturnValue dlt_log_uint(DltContext *handle,DltLogLevelType loglevel, unsigned int data);
//DltReturnValue dlt_log_raw(DltContext *handle,DltLogLevelType loglevel, void *data,uint16_t length);
//DltReturnValue dlt_log_marker();
//DltReturnValue dlt_forward_msg(void *msgdata,size_t size);
//DltReturnValue dlt_user_check_buffer(int *total_size, int *used_size);
//int dlt_user_atexit_blow_out_user_buffer(void);
//DltReturnValue dlt_user_log_resend_buffer(void);

// #ifdef DLT_TEST_ENABLE
//void dlt_user_test_corrupt_user_header(int enable);
//void dlt_user_test_corrupt_message_size(int enable,int16_t size);
// #endif
}