use libc::{ c_int, c_uint, c_long, size_t, c_char, c_uchar, c_void, speed_t, FILE, PATH_MAX };

use dlt_types::DltReturnValue;

/// The size of a DLT ID
pub const DLT_ID_SIZE:    usize = 4;

/// Maximum number of filters
pub const DLT_FILTER_MAX: usize = 30;

extern {
    /// The definition of the serial header containing the characters "DLS" + 0x01.
    #[link_name = "dltSerialHeader"]
    pub static mut dltSerialHeader: [c_char; DLT_ID_SIZE];

    /// The definition of the serial header containing the characters "DLS" + 0x01 as char.
    #[link_name = "dltSerialHeaderChar"]
    pub static mut dltSerialHeaderChar: [c_char; DLT_ID_SIZE];

    /// The common base-path of the dlt-daemon-fifo and application-generated fifos
    #[link_name = "dltFifoBaseDir"]
    pub static mut dltFifoBaseDir: [c_char; (PATH_MAX + 1) as usize];
}

/// The structure of the DLT file storage header.
/// This header is used before each stored DLT message
#[cfg_attr(not(target_env = "msvc"), repr(packed))]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DltStorageHeader {
    /// This pattern should be DLT0x01
    pub pattern: [c_char; DLT_ID_SIZE],
    /// Seconds since epoch(1.1.1970)
    pub seconds: u32,
    /// Microseconds
    pub microseconds: i32,
    /// The ECU id is added, if it is not already in the DLT message itself
    pub ecu: [c_char; DLT_ID_SIZE]
}
// Not tested yet on Windows
#[cfg(not(target_env = "msvc"))] const SIZE_OF_DLT_STORAGE_HEADER: usize = 16;

/// The structure of the DLT standard header.
/// This header is used in each DLT message
#[cfg_attr(not(target_env = "msvc"), repr(packed))]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DltStandardHeader {
    /// This parameter contains several informations
    pub htyp: u8,
    /// The message counter is increased with each sent DLT message
    pub mcnt: u8,
    /// Length of the complete message, without storage header
    pub len: u16
}
#[cfg(not(target_env = "msvc"))] const SIZE_OF_DLT_STANDARD_HEADER: usize = 4;

/// The structure of the DLT extra header parameters.
/// Each parameter is sent only if enabled in htyp
#[cfg_attr(not(target_env = "msvc"), repr(packed))]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DltStandardHeaderExtra {
    /// ECU id
    pub ecu: [c_char; DLT_ID_SIZE],
    /// Session number
    pub seid: u32,
    /// Timestamp since system start in 0.1 milliseconds
    pub tmsp: u32
}
#[cfg(not(target_env = "msvc"))] const SIZE_OF_DLT_STANDARD_HEADER_EXTRA: usize = 12;

/// The structure of the DLT extended header.
/// This header is only sent if enabled in htyp parameter
#[cfg_attr(not(target_env = "msvc"), repr(packed))]
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DltExtendedHeader {
    /// Message info
    pub msin: u8,
    /// Number of arguments
    pub noar: u8,
    /// Application id
    pub apid: [c_char; DLT_ID_SIZE],
    /// Context id
    pub ctid: [c_char; DLT_ID_SIZE]
}
#[cfg(not(target_env = "msvc"))] const SIZE_OF_DLT_EXTENDED_HEADER: usize = 10;

/// The structure to organise the DLT messages
/// This structure is used by the corresponding functions
#[repr(C)]
pub struct DltMessage {
    // Flags
    pub found_serialheader: i8,

    // Offsets
    pub resync_offset: i32,

    // Size parameters
    /// Size of complete header including storage header
    pub headersize: i32,
    /// Size of complete payload
    pub datasize: i32,

    // Buffer for current loaded message
    /// Buffer for loading complete header
    pub headerbuffer: [i8; SIZE_OF_DLT_STORAGE_HEADER + SIZE_OF_DLT_STANDARD_HEADER
        + SIZE_OF_DLT_STANDARD_HEADER_EXTRA + SIZE_OF_DLT_EXTENDED_HEADER],
    /// Buffer for loading payload
    pub databuffer: *mut u8,
    pub databuffersize: i32
}

/// Structure to store filter parameters.
/// ID are maximal four characters. Unused values are filled with zeros.
/// If every value as filter is valid, the id should be empty by having only zero values.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DltFilter {
    /// Application id
    pub apid: [[c_char; DLT_ID_SIZE]; DLT_FILTER_MAX],
    /// Context id
    pub ctid: [[c_char; DLT_ID_SIZE]; DLT_FILTER_MAX],
    /// Number of filters
    pub counter: c_int
}

/// The structure to organise the access to DLT files.
/// This structure is used by the corresponding functions.
#[repr(C)]
pub struct DltFile {
    // File handle and index for fast access
    pub handle: *mut FILE,
    pub index:  *mut c_long,

    // Size parameters
    /// Number of messages in DLT file with filter
    pub counter: i32,
    /// Number of messages in DLT file without filter
    pub counter_total: i32,
    /// Current index to message parsed in DLT file starting at 0
    pub position: i32,
    /// Length of the file
    pub file_length: c_long,
    /// Current position in the file
    pub file_position: c_long,

    // Error counters
    /// Number of incomplete DLT messages found during file parsing
    pub error_messages: i32,

    // Filter parameters
    /// Pointer to filter list. Zero if no filter is set
    pub filter: *mut DltFilter,
    /// Number of filter set
    pub filter_counter: i32,

    // Current loaded message
    /// The message
    pub msg: DltMessage
}

/// The structure is used to organise the receiving of data
/// including buffer handling.
/// This structure is used by the corresponding functions.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DltReceiver {
    /// Bytes received in last receive call
    pub lastBytesRcvd: i32,
    /// Received bytes
    pub bytesRcvd: i32,
    /// Total number of received bytes
    pub totalBytesRcvd: i32,
    /// Pointer to receiver buffer
    pub buffer: *mut c_char,
    /// Pointer to position within' the receiver buffer
    pub buf: *mut c_char,
    /// Connection handle
    pub fd: c_int,
    /// Size of receiver buffer
    pub buffersize: i32
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DltBuffer {
    /// Pointer to beginning of shared memory
    pub shm: *mut c_uchar,
    /// Size of data area in shared memory
    pub size: c_int,
    /// Pointer to data area in shared memory
    pub mem: *mut c_uchar,

    /// Minimum size of buffer
    pub min_size: u32,
    /// Maximum size of buffer
    pub max_size: u32,
    /// Step size of buffer
    pub step_size: u32
}

extern {
    pub fn dlt_print_hex(ptr: *mut u8, size: c_int);
    pub fn dlt_print_hex_string(text: *mut c_char, textlength: c_int, ptr: *mut u8, size: c_int) -> DltReturnValue;
    pub fn dlt_print_mixed_string(text: *mut c_char, textlength: c_int, ptr: *mut u8, size: c_int, html: c_int) -> DltReturnValue;
    pub fn dlt_print_char_string(text: *mut *mut c_char, textlength: c_int, ptr: *mut u8, size: c_int) -> DltReturnValue;
    pub fn dlt_print_id(text: *mut c_char, id: *const c_char);
    pub fn dlt_set_id(id: *mut c_char, text: *const c_char);
    pub fn dlt_clean_string(text: *mut c_char, length: c_int);
    pub fn dlt_filter_init(filter: *mut DltFilter, verbose: c_int) -> DltReturnValue;
    pub fn dlt_filter_free(filter: *mut DltFilter, verbose: c_int) -> DltReturnValue;
    pub fn dlt_filter_load(filter: *mut DltFilter, filename: *const c_char, verbose: c_int) -> DltReturnValue;
    pub fn dlt_filter_save(filter: *mut DltFilter, filename: *const c_char, verbose: c_int) -> DltReturnValue;
    pub fn dlt_filter_find(filter: *mut DltFilter, apid: *const c_char, ctid: *const c_char, verbose: c_int) -> c_int;
    pub fn dlt_filter_add(filter: *mut DltFilter, apid: *const c_char, ctid: *const c_char, verbose: c_int) -> DltReturnValue;
    pub fn dlt_filter_delete(filter: *mut DltFilter, apid: *const c_char, ctid: *const c_char, verbose: c_int) -> DltReturnValue;
    pub fn dlt_message_init(msg: *mut DltMessage, verbose: c_int) -> DltReturnValue;
    pub fn dlt_message_free(msg: *mut DltMessage, verbose: c_int) -> DltReturnValue;
    pub fn dlt_message_header(msg: *mut DltMessage, text: *mut c_char, textlength: c_int, verbose: c_int) -> DltReturnValue;
    pub fn dlt_message_header_flags(msg: *mut DltMessage, text: *mut c_char, textlength: c_int, flags: c_int, verbose: c_int) -> DltReturnValue;
    pub fn dlt_message_payload(msg: *mut DltMessage, text: *mut c_char, textlength: c_int, _type: c_int, verbose: c_int) -> DltReturnValue;
    pub fn dlt_message_filter_check(msg: *mut DltMessage, filter: *mut DltFilter, verbose: c_int) -> DltReturnValue;
    pub fn dlt_message_read(msg: *mut DltMessage, buffer: *mut u8, length: c_uint, resync: c_int, verbose: c_int) -> c_int;
    pub fn dlt_message_get_extraparameters(msg: *mut DltMessage, verbose: c_int) -> DltReturnValue;
    pub fn dlt_message_set_extraparameters(msg: *mut DltMessage, verbose: c_int) -> DltReturnValue;
    pub fn dlt_file_init(file: *mut DltFile, verbose: c_int) -> DltReturnValue;
    pub fn dlt_file_set_filter(file: *mut DltFile, filter: *mut DltFilter, verbose: c_int) -> DltReturnValue;
    pub fn dlt_file_open(file: *mut DltFile, filename: *const c_char, verbose: c_int) -> DltReturnValue;
    pub fn dlt_file_read(file: *mut DltFile, verbose: c_int) -> DltReturnValue;
    pub fn dlt_file_read_raw(file: *mut DltFile, resync: c_int, verbose: c_int) -> DltReturnValue;
    pub fn dlt_file_close(file: *mut DltFile, verbose: c_int) -> DltReturnValue;
    pub fn dlt_file_read_header(file: *mut DltFile, verbose: c_int) -> DltReturnValue;
    pub fn dlt_file_read_header_raw(file: *mut DltFile, resync: c_int, verbose: c_int) -> DltReturnValue;
    pub fn dlt_file_read_header_extended(file: *mut DltFile, verbose: c_int) -> DltReturnValue;
    pub fn dlt_file_read_data(file: *mut DltFile, verbose: c_int) -> DltReturnValue;
    pub fn dlt_file_message(file: *mut DltFile, index: c_int, verbose: c_int) -> DltReturnValue;
    pub fn dlt_file_free(file: *mut DltFile, verbose: c_int) -> DltReturnValue;
    pub fn dlt_log_set_filename(filename: *const c_char);
    pub fn dlt_log_set_level(level: c_int);
    pub fn dlt_log_init(mode: c_int);
    pub fn dlt_log(prio: c_int, s: *mut c_char) -> DltReturnValue;
    pub fn dlt_vlog(prio: c_int, format: *const c_char, ...) -> DltReturnValue;
    pub fn dlt_vnlog(prio: c_int, size: size_t, format: *const c_char, ...) -> DltReturnValue;
    pub fn dlt_log_free();
    pub fn dlt_receiver_init(receiver: *mut DltReceiver, _fd: c_int, _buffersize: c_int) -> DltReturnValue;
    pub fn dlt_receiver_free(receiver: *mut DltReceiver) -> DltReturnValue;
    pub fn dlt_receiver_receive_socket(receiver: *mut DltReceiver) -> c_int;
    pub fn dlt_receiver_receive_fd(receiver: *mut DltReceiver) -> c_int;
    pub fn dlt_receiver_remove(receiver: *mut DltReceiver, size: c_int) -> DltReturnValue;
    pub fn dlt_receiver_move_to_begin(receiver: *mut DltReceiver) -> DltReturnValue;
    pub fn dlt_receiver_check_and_get(receiver: *mut DltReceiver, dest: *mut c_void, to_get: c_uint, skip_header: c_uint) -> c_int;
    pub fn dlt_set_storageheader(storageheader: *mut DltStorageHeader, ecu: *const c_char) -> DltReturnValue;
    pub fn dlt_check_storageheader(storageheader: *mut DltStorageHeader) -> DltReturnValue;
    pub fn dlt_buffer_init_static_server(buf: *mut DltBuffer, ptr: *const c_uchar, size: u32) -> DltReturnValue;
    pub fn dlt_buffer_init_static_client(buf: *mut DltBuffer, ptr: *const c_uchar, size: u32) -> DltReturnValue;
    pub fn dlt_buffer_init_dynamic(buf: *mut DltBuffer, min_size: u32, max_size: u32, step_size: u32) -> DltReturnValue;
    pub fn dlt_buffer_free_static(buf: *mut DltBuffer) -> DltReturnValue;
    pub fn dlt_buffer_free_dynamic(buf: *mut DltBuffer) -> DltReturnValue;
    pub fn dlt_buffer_push(buf: *mut DltBuffer, data: *const c_uchar, size: c_uint) -> DltReturnValue;
    pub fn dlt_buffer_push3(buf: *mut DltBuffer, data1: *const c_uchar, size1: c_uint, data2: *const c_uchar, size2: c_uint, data3: *const c_uchar, size3: c_uint) -> DltReturnValue;
    pub fn dlt_buffer_pull(buf: *mut DltBuffer, data: *mut c_uchar, max_size: c_int) -> c_int;
    pub fn dlt_buffer_copy(buf: *mut DltBuffer, data: *mut c_uchar, max_size: c_int) -> c_int;
    pub fn dlt_buffer_remove(buf: *mut DltBuffer) -> c_int;
    pub fn dlt_buffer_info(buf: *mut DltBuffer);
    pub fn dlt_buffer_status(buf: *mut DltBuffer);
    pub fn dlt_buffer_get_total_size(buf: *mut DltBuffer) -> u32;
    pub fn dlt_buffer_get_used_size(buf: *mut DltBuffer) -> c_int;
    pub fn dlt_buffer_get_message_count(buf: *mut DltBuffer) -> c_int;
    #[cfg(not(target_os = "windows"))] pub fn dlt_setup_serial(fd: c_int, speed: speed_t) -> DltReturnValue;
    #[cfg(not(target_os = "windows"))] pub fn dlt_convert_serial_speed(baudrate: c_int) -> speed_t;
    #[cfg(not(target_os = "windows"))] pub fn dlt_get_version(buf: *mut c_char, size: size_t);
    #[cfg(not(target_os = "windows"))] pub fn dlt_get_major_version(buf: *mut c_char, size: size_t);
    #[cfg(not(target_os = "windows"))] pub fn dlt_get_minor_version(buf: *mut c_char, size: size_t);
    pub fn dlt_init_common() -> DltReturnValue;
    pub fn dlt_uptime() -> u32;
    pub fn dlt_message_print_header(message: *mut DltMessage, text: *mut c_char, size: u32, verbose: c_int) -> DltReturnValue;
    pub fn dlt_message_print_hex(message: *mut DltMessage, text: *mut c_char, size: u32, verbose: c_int) -> DltReturnValue;
    pub fn dlt_message_print_ascii(message: *mut DltMessage, text: *mut c_char, size: u32, verbose: c_int) -> DltReturnValue;
    pub fn dlt_message_print_mixed_plain(message: *mut DltMessage, text: *mut c_char, size: u32, verbose: c_int) -> DltReturnValue;
    pub fn dlt_message_print_mixed_html(message: *mut DltMessage, text: *mut c_char, size: u32, verbose: c_int) -> DltReturnValue;
    pub fn dlt_message_argument_print(msg: *mut DltMessage, type_info: u32, ptr: *mut *mut u8, datalength: *mut i32, text: *mut c_char, textlength: c_int, byteLength: c_int, verbose: c_int) -> DltReturnValue;
    pub fn dlt_check_envvar();
    pub fn dlt_mkdir_recursive(dir: *const c_char) -> c_int;
}
