use libc::{ c_int, c_uint, c_long, size_t, c_char, c_uchar, c_void, speed_t, FILE, PATH_MAX };

use dlt_types::DltReturnValue;

/// The size of a DLT ID
pub const DLT_ID_SIZE:    usize = 4;

/// Maximum number of filters
pub const DLT_FILTER_MAX: usize = 30;

extern "C" {
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

extern "C" {
    /// Helper function to print a byte array in hex.
    ///
    /// `ptr` - pointer to the byte array
    ///
    /// `size` - number of bytes to be printed
    pub fn dlt_print_hex(ptr: *mut u8, size: c_int);

    /// Helper function to print a byte array in hex into a string.
    ///
    /// `text` - pointer to a ASCII string, in which the text is written
    ///
    /// `textlength` - maximal size of text buffer
    ///
    /// `ptr` - pointer to the byte array
    ///
    /// `size` - number of bytes to be printed
    ///
    /// ### Returns
    /// Negative value if there was an error
    pub fn dlt_print_hex_string(text: *mut c_char, textlength: c_int, ptr: *mut u8, size: c_int) -> DltReturnValue;

    /// Helper function to print a byte array in hex and ascii into a string.
    ///
    /// `text` - pointer to an ASCII string, in which the text is written
    ///
    /// `textlength` - maximal size of text buffer
    ///
    /// `ptr` - pointer to the byte array
    ///
    /// `size` - number of bytes to be printed
    ///
    /// `html` - output is html? 0 - false, 1 - true
    ///
    /// ### Returns
    /// Negative value if there was an error
    pub fn dlt_print_mixed_string(text: *mut c_char, textlength: c_int, ptr: *mut u8, size: c_int, html: c_int) -> DltReturnValue;

    /// Helper function to print a byte array in ascii into a string.
    ///
    /// `text` - pointer to an ASCII string, in which the text is written
    ///
    /// `textlength` - maximal size of text buffer
    ///
    /// `ptr` - pointer to the byte array
    ///
    /// `size` - number of bytes to be printed
    ///
    /// ### Returns
    /// Negative value if there was an error
    pub fn dlt_print_char_string(text: *mut *mut c_char, textlength: c_int, ptr: *mut u8, size: c_int) -> DltReturnValue;

    /// Helper function to print an ID.
    ///
    /// `text` - pointer to ASCII string where to write the ID
    ///
    /// `id` - four byte char array as used in DLT messages as IDs
    pub fn dlt_print_id(text: *mut c_char, id: *const c_char);

    /// Helper function to set an ID parameter.
    ///
    /// `id` - four byte char array as used in DLT messages as IDs
    ///
    /// `text` - string to be copied into char array
    pub fn dlt_set_id(id: *mut c_char, text: *const c_char);

    /// Helper function to remove not nice to print characters, e.g. NULL or carage return.
    ///
    /// `text` - pointer to string to be claned
    ///
    /// `length` - length of string excluding terminating zero
    pub fn dlt_clean_string(text: *mut c_char, length: c_int);

    /// Initialise the filter list.
    /// This function must be called before using further dlt filter.
    ///
    /// `filter` - pointer to structure of organising DLT filter
    ///
    /// `verbose` - if set to true, verbose information is printed out
    ///
    /// ### Returns
    /// Negative value if there was an error
    pub fn dlt_filter_init(filter: *mut DltFilter, verbose: c_int) -> DltReturnValue;

    /// Free the used memory by the organising structure of filter.
    ///
    /// `filter` - pointer to structure of organising DLT filter
    ///
    /// `verbose` - if set to true, verbose information is printed out
    ///
    /// ### Returns
    /// Negative value if there was an error
    pub fn dlt_filter_free(filter: *mut DltFilter, verbose: c_int) -> DltReturnValue;

    /// Load filter list from file.
    ///
    /// `filter` - pointer to structure of organising DLT filter
    ///
    /// `filename` - filename to load filter from
    ///
    /// `verbose` - if set to true, verbose information is printed out
    ///
    /// ### Returns
    /// Negative value if there was an error
    pub fn dlt_filter_load(filter: *mut DltFilter, filename: *const c_char, verbose: c_int) -> DltReturnValue;

    /// Save filter list to file.
    ///
    /// `filter` - pointer to structure of organising DLT filter
    ///
    /// `filename` - filename to load filters from
    ///
    /// `verbose` - if set to true, verbose information is printed out
    ///
    /// ### Returns
    /// Negative value if there was an error
    pub fn dlt_filter_save(filter: *mut DltFilter, filename: *const c_char, verbose: c_int) -> DltReturnValue;

    /// Find index of filter in filter list.
    ///
    /// `filter` - pointer to structure of organising DLT filter
    ///
    /// `apid` - application ID to be found in filter list
    ///
    /// `ctid` - context ID to be found in filter list
    ///
    /// `verbose` - if set to true, verbose information is printed out
    ///
    /// ### Returns
    /// Negative value if there was an error(or not found), else return index of filter
    pub fn dlt_filter_find(filter: *mut DltFilter, apid: *const c_char, ctid: *const c_char, verbose: c_int) -> c_int;

    /// Add new filter to filter list.
    ///
    /// `filter` - pointer to structure of organising DLT filter
    ///
    /// `apid` - application ID to be added to filter list (must always be set)
    ///
    /// `ctid` - context ID to be added to filter list(empty equals don't care)
    ///
    /// `verbose` - if set to true, verbose information is printed out
    ///
    /// ### Returns
    /// Negative value if there was an error
    pub fn dlt_filter_add(filter: *mut DltFilter, apid: *const c_char, ctid: *const c_char, verbose: c_int) -> DltReturnValue;

    /// Delete filter from filter list.
    ///
    /// `filter` - pointer to structure of organising DLT filter
    ///
    /// `apid` - application ID to be deleted from filter list
    ///
    /// `ctid` - context ID to be deleted from filter list
    ///
    /// `verbose` - if set to true, verbose information is printed out
    ///
    /// ### Returns
    /// Negative value if there was an error
    pub fn dlt_filter_delete(filter: *mut DltFilter, apid: *const c_char, ctid: *const c_char, verbose: c_int) -> DltReturnValue;

    /// Initialise the structure used to access a DLT message.
    /// This function must be called before using further dlt_message functions.
    ///
    /// `msg` - pointer to structure of organising access to DLT messages
    ///
    /// `verbose` - if set to true, verbose information is printed out
    ///
    /// ### Returns
    /// Negative value if there was an error
    pub fn dlt_message_init(msg: *mut DltMessage, verbose: c_int) -> DltReturnValue;

    /// Free the used memory by the organising structure of file.
    ///
    /// `msg` - pointer to structure of organising access to DLT messages
    ///
    /// `verbose` - if set to true, verbose information is printed out
    ///
    /// ### Returns
    /// Negative value if there was an error
    pub fn dlt_message_free(msg: *mut DltMessage, verbose: c_int) -> DltReturnValue;

    /// Print Header into an ASCII string.
    /// This function calls dlt_message_header_flags() with flags=DLT_HEADER_SHOW_ALL
    ///
    /// `msg` - pointer to structure of organising access to DLT messages
    ///
    /// `text` - pointer to an ASCII string, in which the header is written
    ///
    /// `textlength` - maximal size of text buffer
    ///
    /// `verbose` - if set to true, verbose information is printed out
    ///
    /// ### Returns
    /// Negative value if there was an error
    pub fn dlt_message_header(msg: *mut DltMessage, text: *mut c_char, textlength: c_int, verbose: c_int) -> DltReturnValue;

    /// Print Header into an ASCII string, selective.
    ///
    /// `msg` - pointer to structure of organising access to DLT messages
    ///
    /// `text` - pointer to an ASCII string, in which the header is written
    ///
    /// `textlength` - maximal size of text buffer
    ///
    /// `flags` - select, bit-field to select, what should be printed(DLT_HEADER_SHOW_...)
    ///
    /// `verbose` - if set to true, verbose information is printed out
    ///
    /// ### Returns
    /// Negative value if there was an error
    pub fn dlt_message_header_flags(msg: *mut DltMessage, text: *mut c_char, textlength: c_int, flags: c_int, verbose: c_int) -> DltReturnValue;

    /// Print Payload into an ASCII string.
    ///
    /// `msg` - pointer to structure of organising access to DLT messages
    ///
    /// `text` - pointer to an ASCII string, in which the header is written
    ///
    /// `textlength` - maximal size of text buffer
    ///
    /// `_type` - 1 = payload as hex, 2 = payload as ASCII
    ///
    /// `verbose` - if set to true, verbose information is printed out
    ///
    /// ### Returns
    /// Negative value if there was an error
    pub fn dlt_message_payload(msg: *mut DltMessage, text: *mut c_char, textlength: c_int, _type: c_int, verbose: c_int) -> DltReturnValue;

    /// Check if message is filtered or not. All filters are applied(logical OR).
    ///
    /// `msg` - pointer to structure of organising access to DLT messages
    ///
    /// `filter` - pointer to filter
    ///
    /// `verbose` - if set to true, verbose information is printed out
    ///
    /// ### Returns
    /// `1` = filter matches, `0` = filter does not match, negative value if there was an error
    pub fn dlt_message_filter_check(msg: *mut DltMessage, filter: *mut DltFilter, verbose: c_int) -> DltReturnValue;

    /// Read message from memory buffer.
    /// Message in buffer has no storage header.
    ///
    /// `msg` - pointer to structure of organising access to DLT messages
    ///
    /// `buffer` - pointer to memory buffer
    ///
    /// `length` - length of message in buffer
    ///
    /// `resync` - if set to true, resync to serial header is enforced
    ///
    /// `verbose` - if set to true, verbose information is printed out
    ///
    /// ### Returns
    /// Negative value if there was an error
    pub fn dlt_message_read(msg: *mut DltMessage, buffer: *mut u8, length: c_uint, resync: c_int, verbose: c_int) -> c_int;

    /// Get standard header extra parameters
    ///
    /// `msg` - pointer to structure of organising access to DLT messages
    ///
    /// `verbose` - if set to true, verbose information is printed out
    ///
    /// ### Returns
    /// Negative value if there was an error
    pub fn dlt_message_get_extraparameters(msg: *mut DltMessage, verbose: c_int) -> DltReturnValue;

    /// Set standard header extra parameters
    ///
    /// `msg` -pointer to structure of organising access to DLT messages
    ///
    /// `verbose` - if set to true, verbose information is printed out
    ///
    /// ### Returns
    /// Negative value if there was an error
    pub fn dlt_message_set_extraparameters(msg: *mut DltMessage, verbose: c_int) -> DltReturnValue;

    /// Initialise the structure used to access a DLT file.
    /// This function must be called before using further dlt_file functions.
    ///
    /// `file` - pointer to structure of organising access to DLT file
    ///
    /// `verbose` - if set to true, verbose information is printed out
    ///
    /// ### Returns
    /// Negative value if there was an error
    pub fn dlt_file_init(file: *mut DltFile, verbose: c_int) -> DltReturnValue;

    /// Set a list of filters.
    /// This function should be called before loading a DLT file, if filter shoudl be used.
    /// A filter list is an array of filters. Several filters are combined logically by an OR operation.
    /// **The filter list is not copied - take care to keep the list in memory**.
    ///
    /// `file` - pointer to structure of organising access to DLT file
    ///
    /// `filter` - pointer to filter list array
    ///
    /// `verbose` - if set to true, verbose information is printed out
    ///
    /// ### Returns
    /// Negative value if there was an error
    pub fn dlt_file_set_filter(file: *mut DltFile, filter: *mut DltFilter, verbose: c_int) -> DltReturnValue;

    /// Initialising loading a DLT file.
    ///
    /// `file` - pointer to structure of organising access to DLT file
    ///
    /// `filename` - filename of DLT file
    ///
    /// `verbose` - if set to true, verbose information is printed out
    ///
    /// ### Returns
    /// Negative value if there was an error
    pub fn dlt_file_open(file: *mut DltFile, filename: *const c_char, verbose: c_int) -> DltReturnValue;

    /// Find next message in the DLT file and parse them.
    /// This function finds the next message in the DLT file.
    /// If a filter is set, the filter list is used.
    ///
    /// `file` - pointer to structure of organising access to DLT file
    ///
    /// `verbose` - if set to true, verbose information is printed out
    ///
    /// ### Returns
    /// `0` = message does not match filter, `1` = message was read, negative value if there was an error
    pub fn dlt_file_read(file: *mut DltFile, verbose: c_int) -> DltReturnValue;

    /// Find next message in the DLT file in RAW format(without storage header) and parse them.
    /// This function finds the next message in the DLT file.
    /// If a filter is set, the filter list is used.
    ///
    /// `file` - pointer to structure of organising access to DLT file
    ///
    /// `resync` - Resync to serial header when set to true
    ///
    /// `verbose` - if set to true, verbose information is printed out
    ///
    /// ### Returns
    /// `0` = message does not match filter, `1` = message was read, negative value if there was an error
    pub fn dlt_file_read_raw(file: *mut DltFile, resync: c_int, verbose: c_int) -> DltReturnValue;

    /// Closing loading a DLT file.
    ///
    /// `file` - pointer to strucutre of organising access to DLT file
    ///
    /// `verbose` - if set to true, verbose information is printed out
    ///
    /// ### Returns
    /// Negative value if there was an error
    pub fn dlt_file_close(file: *mut DltFile, verbose: c_int) -> DltReturnValue;

    /// Load standard header of a message from file
    ///
    /// `file` - pointer to structure of organising access to DLT file
    ///
    /// `verbose` - if set to true, verbose information is printed out
    ///
    /// ### Returns
    /// Negative value if there was an error
    pub fn dlt_file_read_header(file: *mut DltFile, verbose: c_int) -> DltReturnValue;

    /// Load standard header of a message from file in RAW format(without storage header)
    ///
    /// `file` - pointer to strucutre of organisning access to DLT file
    ///
    /// `resync` - Resync to serial header when set to true
    ///
    /// `verbose` - if set to true, verbose information is printed out
    ///
    /// ### Returns
    /// Negative value if there was an error
    pub fn dlt_file_read_header_raw(file: *mut DltFile, resync: c_int, verbose: c_int) -> DltReturnValue;

    /// Load, if available in message, extra standard header field and
    /// extended header of a message from file
    /// (**dlt_file_read_header() must have been called before this call!**)
    ///
    /// `file` - pointer to strucutre of organising access to DLT file
    ///
    /// `verbose` - if set to true, verbose information is printed out
    ///
    /// ### Returns
    /// Negative value if there was an error
    pub fn dlt_file_read_header_extended(file: *mut DltFile, verbose: c_int) -> DltReturnValue;

    /// Load payload of a message from file
    /// (**dlt_file_read_header() must have been called before this call!**)
    ///
    /// `file` - pointer to structure of organising access to DLT file
    ///
    /// `verbose` - if set to true, verbose information is printed out
    ///
    /// ### Returns
    /// Negative value if there was an error
    pub fn dlt_file_read_data(file: *mut DltFile, verbose: c_int) -> DltReturnValue;

    /// Load header and payload of a message selected by the index.
    /// If filter are set, index is based on the filtered list.
    ///
    /// `file` - pointer to structure of organising access to DLT file
    ///
    /// `index` - position of message in the files beginning from zero
    ///
    /// `verbose` - if set to true, verbose information is printed out
    ///
    /// ### Returns
    /// Number of messages loaded, negative value if there was an error
    pub fn dlt_file_message(file: *mut DltFile, index: c_int, verbose: c_int) -> DltReturnValue;

    /// Free the used memory by the organising structure of file.
    ///
    /// `file` - pointer to structure of organising access to DLT file
    ///
    /// `verbose` - if set to true, verbose information is printed out
    ///
    /// ### Returns
    /// Negative value if there was an error
    pub fn dlt_file_free(file: *mut DltFile, verbose: c_int) -> DltReturnValue;

    /// Set internal logging filename if mode 2
    ///
    /// `filename` - the filename
    pub fn dlt_log_set_filename(filename: *const c_char);

    /// Set internal logging level
    ///
    /// `level` - the level
    pub fn dlt_log_set_level(level: c_int);

    /// Initialize(external) logging facility
    ///
    /// `mode` - positive, `0` = log to stdout, `1` = log to syslog, `2` = log to file
    pub fn dlt_log_init(mode: c_int);

    /// Log ASCII string with null-termination to (external) logging facility
    ///
    /// `prio` - priority(see syslog() call)
    ///
    /// `s` - pointer to an ASCII string with null-termination
    ///
    /// ### Returns
    /// Negative value if there was an error
    pub fn dlt_log(prio: c_int, s: *mut c_char) -> DltReturnValue;

    /// Log with variable arguments to (external) logging facility (like printf)
    ///
    /// `prio` - priority (see syslog() call)
    ///
    /// `format` - format string for log message
    ///
    /// ### Returns
    /// Negative value if there was an error
    pub fn dlt_vlog(prio: c_int, format: *const c_char, ...) -> DltReturnValue;

    /// Log size bytes with variable arguments to (external) logging facility (similar to snprintf)
    ///
    /// `prio` - priority (see syslog() call)
    ///
    /// `size` - number of bytes to log
    ///
    /// `format` - format string for log message
    ///
    /// ### Returns
    /// Negative value if there was an error
    pub fn dlt_vnlog(prio: c_int, size: size_t, format: *const c_char, ...) -> DltReturnValue;

    /// De-Initialize (external) logging facility
    pub fn dlt_log_free();

    /// Initialising a DLT receiver structure
    ///
    /// `receiver` - pointer to DLT receiver structure
    ///
    /// `_fd` - handle to file/socket/fifo, from which the data should be received
    ///
    /// `_buffersize` - size of the data buffer for storing the received data
    ///
    /// ### Returns
    /// Negative value if there was an error
    pub fn dlt_receiver_init(receiver: *mut DltReceiver, _fd: c_int, _buffersize: c_int) -> DltReturnValue;

    /// De-Initialize a DLT receiver structure
    ///
    /// `receiver` - pointer to DLT receiver structure
    ///
    /// ### Returns
    /// Negative value if there was an error
    pub fn dlt_receiver_free(receiver: *mut DltReceiver) -> DltReturnValue;

    /// Receive data from socket using the DLT receiver structure
    ///
    /// `receiver` - pointer to DLT receiver structure
    ///
    /// ### Returns
    /// Number of received bytes or negative value if there was an error
    pub fn dlt_receiver_receive_socket(receiver: *mut DltReceiver) -> c_int;

    /// Receive data from file/FIFO using the DLT receiver structure
    ///
    /// `receiver` - pointer to DLT receiver structure
    ///
    /// ### Returns
    /// Number of received bytes or negative value if there was an error
    pub fn dlt_receiver_receive_fd(receiver: *mut DltReceiver) -> c_int;

    /// Remove a specific size of bytes from the received data
    ///
    /// `receiver` - pointer to DLT reciever structure
    ///
    /// `size` - amount of bytes to be removed
    ///
    /// ### Returns
    /// Negative value if there was an error
    pub fn dlt_receiver_remove(receiver: *mut DltReceiver, size: c_int) -> DltReturnValue;

    /// Move data from last recieve call to front of receive buffer
    ///
    /// `receiver` - pointer to DLT reciever structure
    ///
    /// ### Returns
    /// Negative value if there was an error
    pub fn dlt_receiver_move_to_begin(receiver: *mut DltReceiver) -> DltReturnValue;

    /// Check whether to_get amount of data is available in receiver and
    /// copy it to dest. Skip the DltUserHeader if skip_header is set to 1.
    ///
    /// `receiver` - pointer to DLT receiver structure
    ///
    /// `dest` - pointer to the destination buffer
    ///
    /// `to_get` - size of the data to copy in dest
    ///
    /// `skip_header` - whether the DltUserHeader must be skipped
    pub fn dlt_receiver_check_and_get(receiver: *mut DltReceiver, dest: *mut c_void, to_get: c_uint, skip_header: c_uint) -> c_int;

    /// Fill out storage header of a DLT message
    ///
    /// `storageheader` - pointer to storage header to a DLT message
    ///
    /// `ecu` - name of ecu to be set in storage header
    ///
    /// ### Returns
    /// Negative value if there was an error
    pub fn dlt_set_storageheader(storageheader: *mut DltStorageHeader, ecu: *const c_char) -> DltReturnValue;

    /// Check if a storage header contains its marker
    ///
    /// `storageheader` - pointer to storage header of a DLT message
    ///
    /// ### Returns
    /// `0` - no, `1` - yes, negative value if there was an error
    pub fn dlt_check_storageheader(storageheader: *mut DltStorageHeader) -> DltReturnValue;

    /// Initialize static ringbuffer with a size of size.
    /// Initialise as server. Init counters.
    /// Memory is already allocated.
    ///
    /// `buf` - pointer to ringbuffer structure
    ///
    /// `ptr` - ptr to ringbuffer memory
    ///
    /// `size` - maximum size of buffer in bytes
    ///
    /// ### Returns
    /// Negative value if there was an error
    pub fn dlt_buffer_init_static_server(buf: *mut DltBuffer, ptr: *const c_uchar, size: u32) -> DltReturnValue;

    /// Initialize static ringbuffer with a size of size.
    /// Initialize as a client. Do not change counters.
    /// Memory is already allocated.
    ///
    /// `buf` - pointer to ringbuffer structure
    ///
    /// `ptr` - ptr to ringbuffer memory
    ///
    /// `size` - maximum size of buffer in bytes
    ///
    /// ### Returns
    /// Negative value if there was an error
    pub fn dlt_buffer_init_static_client(buf: *mut DltBuffer, ptr: *const c_uchar, size: u32) -> DltReturnValue;

    /// Initialize dynamic ringbuffer with a size of size.
    /// Initialize as a client. Do not change counters.
    /// Memory will be allocated starting with min_size.
    /// If more memory is needed, size is increased with step_size.
    /// The maximum size is max_size.
    ///
    /// `buf` - pointer to ringbuffer structure
    ///
    /// `min_size` - minimum size of buffer in bytes
    ///
    /// `max_size` - maximum size of buffer in bytes
    ///
    /// `step_size` - size of which ringbuffer is increased
    ///
    /// ### Returns
    /// Negative  value if there was an error
    pub fn dlt_buffer_init_dynamic(buf: *mut DltBuffer, min_size: u32, max_size: u32, step_size: u32) -> DltReturnValue;

    /// Deinitialize usage of the static ringbuffer
    ///
    /// `buf` - pointer to the ringbuffer structure
    ///
    /// ### Returns
    /// Negative value if there was an error
    pub fn dlt_buffer_free_static(buf: *mut DltBuffer) -> DltReturnValue;

    /// Release and free memory used by the dynamic ringbuffer
    ///
    /// `buf` - pointer to the ringbuffer structure
    ///
    /// ### Returns
    /// Negative value if there was an error
    pub fn dlt_buffer_free_dynamic(buf: *mut DltBuffer) -> DltReturnValue;

    /// Write one entry to the ringbuffer
    ///
    /// `buf` - pointer to the ringbuffer structure
    ///
    /// `data` - pointer to data to be written into the ringbuffer
    ///
    /// `size` - size of data in bytes to be written into the ringbuffer
    ///
    /// ### Returns
    /// Negative value if there was an error
    pub fn dlt_buffer_push(buf: *mut DltBuffer, data: *const c_uchar, size: c_uint) -> DltReturnValue;

    /// Write up to three entries to ringbuffer.
    /// Entries are joined to one block.
    ///
    /// `dlt` - pointer to the ringbuffer structure
    ///
    /// `data[1-3]` - pointer to the data to be written into the ringbuffer
    ///
    /// `size[1-3]` - size of the data in bytes to be written into the ringbuffer
    ///
    /// ### Returns
    /// Negative value if there was an error
    pub fn dlt_buffer_push3(buf: *mut DltBuffer, data1: *const c_uchar, size1: c_uint, data2: *const c_uchar, size2: c_uint, data3: *const c_uchar, size3: c_uint) -> DltReturnValue;

    /// Read one entry from ringbuffer.
    /// Remove it from ringbuffer.
    ///
    /// `buf` - pointer to the ringbuffer structure
    ///
    /// `data` - pointer to data read from ringbuffer
    ///
    /// `max_size` - max size of data in bytes from ringbuffer
    ///
    /// ### Returns
    /// Size of read data, zero if no data available, negative value if there was an error
    pub fn dlt_buffer_pull(buf: *mut DltBuffer, data: *mut c_uchar, max_size: c_int) -> c_int;

    /// Read one entry from ringbuffer.
    /// Do not remove it from ringbuffer.
    ///
    /// `buf` - pointer to the ringbuffer structure
    ///
    /// `data` - pointer to data read from ringbuffer
    ///
    /// `max_size` - max size of read data in bytes from ringbuffer
    ///
    /// ### Returns
    /// Size of read data, zero if no data available, negative value if there was an error
    pub fn dlt_buffer_copy(buf: *mut DltBuffer, data: *mut c_uchar, max_size: c_int) -> c_int;

    /// Remove entry from ringbuffer.
    ///
    /// `buf` - pointer to the ringbuffer structure
    ///
    /// ### Returns
    /// Size of read data, zero if no data available, negative value if there was an error
    pub fn dlt_buffer_remove(buf: *mut DltBuffer) -> c_int;

    /// Print information about buffer and log to internal DLT log.
    ///
    /// `buf` - pointer to the ringbuffer structure
    pub fn dlt_buffer_info(buf: *mut DltBuffer);

    /// Print status of buffer and log to internal DLT log.
    ///
    /// `buf` - pointer to the ringbuffer structure
    pub fn dlt_buffer_status(buf: *mut DltBuffer);

    /// Get total size in bytes of ringbuffer.
    /// If buffer is dynamic, max size is returned.
    ///
    /// `buf` - pointer to the ringbuffer structure
    ///
    /// ### Returns
    /// Total size of buffer
    pub fn dlt_buffer_get_total_size(buf: *mut DltBuffer) -> u32;

    /// Get used size in bytes of ringbuffer.
    ///
    /// `buf` - pointer to the ringbuffer structure
    ///
    /// ### Returns
    /// Used size of buffer
    pub fn dlt_buffer_get_used_size(buf: *mut DltBuffer) -> c_int;

    /// Get number of entries in the ringbuffer.
    ///
    /// `buf` - pointer to the ringbuffer structure
    ///
    /// ### Returns
    /// Number of entries
    pub fn dlt_buffer_get_message_count(buf: *mut DltBuffer) -> c_int;

    // non-Windows specific functions can be found lower in the "non_windows" module

    // DLT internal functions
    /// Common port of initialization
    ///
    /// ### Returns
    /// Negative value if there was an error
    pub fn dlt_init_common() -> DltReturnValue;

    /// Return the uptime of the system in 0.1 ms resolution
    ///
    /// ### Returns
    /// `0` - if there was an error
    pub fn dlt_uptime() -> u32;

    /// Print header of a DLT message
    ///
    /// `message` - pointer to structure of organising access to DLT messages
    ///
    /// `text` - pointer to an ASCII string, in which the header is written
    ///
    /// `size` - maximum size of the text buffer
    ///
    /// `verbose` - if set to `true`, verbose information is printed out
    ///
    /// ### Returns
    /// Negative value if there was an error
    pub fn dlt_message_print_header(message: *mut DltMessage, text: *mut c_char, size: u32, verbose: c_int) -> DltReturnValue;

    /// Print payload of a DLT message as Hex-Output
    ///
    /// `message` - pointer to structure of organising access to DLT messages
    ///
    /// `text` - pointer to an ASCII string, in which the output is written
    ///
    /// `size` - maximum size of the text buffer
    ///
    /// `verbose` - if set to `true`, verbose information is printed out
    ///
    /// ### Returns
    /// Negative value if there was an error
    pub fn dlt_message_print_hex(message: *mut DltMessage, text: *mut c_char, size: u32, verbose: c_int) -> DltReturnValue;

    /// Print payload of a DLT message as ASCII-Output
    ///
    /// `message` - pointer to structure of organising access to DLT messages
    ///
    /// `text` - pointer to an ASCII string, in which the output is written
    ///
    /// `size` - maximum size of the text buffer
    ///
    /// `verbose` - if set to `true`, verbose information is printed out
    ///
    /// ### Returns
    /// Negative value if there was an error
    pub fn dlt_message_print_ascii(message: *mut DltMessage, text: *mut c_char, size: u32, verbose: c_int) -> DltReturnValue;

    /// Print payload of a DLT message as Mixed-Output(Hex and ASCII), for plain text output
    ///
    /// `message` - pointer to structure of organising access to DLT messages
    ///
    /// `text` - pointer to an ASCII string, in which the output is written
    ///
    /// `size` - maximum size of the buffer
    ///
    /// `verbose` - if set to `true`, verbose information is printed out
    ///
    /// ### Returns
    /// Negative value if there was an error
    pub fn dlt_message_print_mixed_plain(message: *mut DltMessage, text: *mut c_char, size: u32, verbose: c_int) -> DltReturnValue;

    /// Print payload of a DLT message as Mixed-Output(Hex and ASCII), for HTML text output
    ///
    /// `message` - pointer to structure of organising access to DLT messages
    ///
    /// `text` - pointer to an ASCII string, in which the output is written
    ///
    /// `size` - maximum size of the text buffer
    ///
    /// `verbose` - if set to `true`, verbose information is printed out
    ///
    /// ### Returns
    /// Negative value if there was an error
    pub fn dlt_message_print_mixed_html(message: *mut DltMessage, text: *mut c_char, size: u32, verbose: c_int) -> DltReturnValue;

    /// Decode and print an argument of a DLT message
    ///
    /// `msg` - pointer to structure of organising access to DLT messages
    ///
    /// `type_info` - type of argument
    ///
    /// `ptr` - pointer to pointer to data(pointer to data is changed within' this function)
    ///
    /// `datalength` - pointer to datalength(datalength is changed within' this function)
    ///
    /// `text` - pointer to an ASCII string, in which the output is written
    ///
    /// `textlength` - maximum size of the text buffer
    ///
    /// `byteLength` - if argument is a string, and this value is `0` or greater, this value will be taken as string length
    ///
    /// `verbose` - if set to `true`, verbose information is printed out
    ///
    /// ### Returns
    /// Negative value if there was an error
    pub fn dlt_message_argument_print(msg: *mut DltMessage, type_info: u32, ptr: *mut *mut u8, datalength: *mut i32, text: *mut c_char, textlength: c_int, byteLength: c_int, verbose: c_int) -> DltReturnValue;

    /// Check environment variables
    pub fn dlt_check_envvar();

    /// Create the specified path, recursive if necessary.
    /// Behaves like calling `mkdir -p <dir>` in the terminal
    pub fn dlt_mkdir_recursive(dir: *const c_char) -> c_int;
}

#[cfg(not(target_os = "windows"))]
mod non_windows {
    use super::*;
    extern "C" {
        /// Helper function: Setup serial connection
        ///
        /// `fd` - file descriptor of serial tty device
        ///
        /// `speed` - serial line speed, as defined in termios.h
        ///
        /// ### Returns
        /// Negative value if there was an error
        pub fn dlt_setup_serial(fd: c_int, speed: speed_t) -> DltReturnValue;

        /// Helper function: Convert serial line baudrate(as number) to line speed(as defined in termios.h)
        ///
        /// `baudrate` - serial line baudrate(as number)
        ///
        /// ### Returns
        /// Serial line speed, as defined in termios.h
        pub fn dlt_convert_serial_speed(baudrate: c_int) -> speed_t;

        /// Print DLT version and DLT git version to buffer
        ///
        /// `buf` - pointer to buffer
        ///
        /// `size` - size of buffer
        pub fn dlt_get_version(buf: *mut c_char, size: size_t);

        /// Print DLT major version to buffer
        ///
        /// `buf` - pointer to buffer
        ///
        /// `size` - size of buffer
        pub fn dlt_get_major_version(buf: *mut c_char, size: size_t);

        /// Print DLT minor version to buffer
        ///
        /// `buf` - pointer to buffer
        ///
        /// `size` - size of buffer
        pub fn dlt_get_minor_version(buf: *mut c_char, size: size_t);
    }
}

#[cfg(not(target_os = "windows"))]
pub use self::non_windows::*;
