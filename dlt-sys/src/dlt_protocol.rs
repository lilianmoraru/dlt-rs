#![allow(non_snake_case)]

use libc::c_int;

// Definitions of the htyp parameter
/// Use extended header
pub const DLT_HTYP_UEH:  c_int = 0x01;
/// MSB first
pub const DLT_HTYP_MSBF: c_int = 0x02;
/// With ECU ID
pub const DLT_HTYP_WEID: c_int = 0x04;
/// With session ID
pub const DLT_HTYP_WSID: c_int = 0x08;
/// With timestamp
pub const DLT_HTYP_WTMS: c_int = 0x10;
/// Version number, 0x1
pub const DLT_HTYP_VERS: c_int = 0xe0;

pub const DLT_HTYP_PROTOCOL_VERSION1: c_int = 1 << 5;

#[inline]
pub fn DLT_IS_HTYP_UEH(htyp:  c_int) -> c_int {
    htyp & DLT_HTYP_UEH
}

#[inline]
pub fn DLT_IS_HTYP_MSBF(htyp: c_int) -> c_int {
    htyp & DLT_HTYP_MSBF
}

#[inline]
pub fn DLT_IS_HTYP_WEID(htyp: c_int) -> c_int {
    htyp & DLT_HTYP_WEID
}

#[inline]
pub fn DLT_IS_HTYP_WSID(htyp: c_int) -> c_int {
    htyp & DLT_HTYP_WSID
}

#[inline]
pub fn DLT_IS_HTYP_WTMS(htyp: c_int) -> c_int {
    htyp & DLT_HTYP_WTMS
}

// Definitions of msin parameter
/// Verbose
pub const DLT_MSIN_VERB: c_int = 0x01;
/// Message type
pub const DLT_MSIN_MSTP: c_int = 0x0e;
/// Message type info
pub const DLT_MSIN_MTIN: c_int = 0xf0;

/// Shift right offset to get mstp value
pub const DLT_MSIN_MSTP_SHIFT: c_int = 1;
/// Shift right offset to get mtin value
pub const DLT_MSIN_MTIN_SHIFT: c_int = 4;

#[inline]
pub fn DLT_IS_MSIN_VERB(msin:  c_int) -> c_int {
    msin & DLT_MSIN_VERB
}

#[inline]
pub fn DLT_GET_MSIN_MSTP(msin: c_int) -> c_int {
    (msin & DLT_MSIN_MSTP) >> DLT_MSIN_MSTP_SHIFT
}

#[inline]
pub fn DLT_GET_MSIN_MTIN(msin: c_int) -> c_int {
    (msin & DLT_MSIN_MTIN) >> DLT_MSIN_MTIN_SHIFT
}

// Definitions of mstp parameter
/// Log message type
pub const DLT_TYPE_LOG:       c_int = 0x00;
/// Application trace message type
pub const DLT_TYPE_APP_TRACE: c_int = 0x01;
/// Network trace message type
pub const DLT_TYPE_NW_TRACE:  c_int = 0x02;
/// Control message type
pub const DLT_TYPE_CONTROL:   c_int = 0x03;

// Definitions of msti parameter
/// Tracing of a variable
pub const DLT_TRACE_VARIABLE:     c_int = 0x01;
/// Tracing of function calls
pub const DLT_TRACE_FUNCTION_IN:  c_int = 0x02;
/// Tracing of function return values
pub const DLT_TRACE_FUNCTION_OUT: c_int = 0x03;
/// Tracing of states of a state machine
pub const DLT_TRACE_STATE:        c_int = 0x04;
/// Tracing of virtual function bus
pub const DLT_TRACE_VFB:          c_int = 0x05;

// Definitions of msci parameter
/// Request message
pub const DLT_CONTROL_REQUEST:  c_int = 0x01;
/// Response to request message
pub const DLT_CONTROL_RESPONSE: c_int = 0x02;
/// Keep-alive message
pub const DLT_CONTROL_TIME:     c_int = 0x03;

pub const DLT_MSIN_CONTROL_REQUEST: c_int = (DLT_TYPE_CONTROL << DLT_MSIN_MSTP_SHIFT)
                                             | (DLT_CONTROL_REQUEST << DLT_MSIN_MTIN_SHIFT);

pub const DLT_MSIN_CONTROL_RESPONSE: c_int = (DLT_TYPE_CONTROL << DLT_MSIN_MSTP_SHIFT)
                                              | (DLT_CONTROL_RESPONSE << DLT_MSIN_MTIN_SHIFT);

pub const DLT_MSIN_CONTROL_TIME: c_int = (DLT_TYPE_CONTROL << DLT_MSIN_MSTP_SHIFT)
                                          | (DLT_CONTROL_TIME << DLT_MSIN_MTIN_SHIFT);

// Definitions of types of arguments in payload
/// Length of standard data: 1 = 8bit, 2 = 16bit, 3 = 32 bit, 4 = 64 bit, 5 = 128 bit
pub const DLT_TYPE_INFO_TYLE: c_int = 0x0000000f;
/// Boolean data
pub const DLT_TYPE_INFO_BOOL: c_int = 0x00000010;
/// Signed integret data
pub const DLT_TYPE_INFO_SINT: c_int = 0x00000020;
/// Unsigned integer data
pub const DLT_TYPE_INFO_UINT: c_int = 0x00000040;
/// Float data
pub const DLT_TYPE_INFO_FLOA: c_int = 0x00000080;
/// Array of standard types
pub const DLT_TYPE_INFO_ARAY: c_int = 0x00000100;
/// String
pub const DLT_TYPE_INFO_STRG: c_int = 0x00000200;
/// Raw data
pub const DLT_TYPE_INFO_RAWD: c_int = 0x00000400;
/// Set, if additional information to a variable is available
pub const DLT_TYPE_INFO_VARI: c_int = 0x00000800;
/// Set, if quantization and offset are added
pub const DLT_TYPE_INFO_FIXP: c_int = 0x00001000;
/// Set, if additional trace information is added
pub const DLT_TYPE_INFO_TRAI: c_int = 0x00002000;
/// Struct
pub const DLT_TYPE_INFO_STRU: c_int = 0x00004000;
/// Coding of the type string: 0 = ASCII, 1 = UTF-8
pub const DLT_TYPE_INFO_SCOD: c_int = 0x00038000;

pub const DLT_TYLE_8BIT:   c_int = 0x00000001;
pub const DLT_TYLE_16BIT:  c_int = 0x00000002;
pub const DLT_TYLE_32BIT:  c_int = 0x00000003;
pub const DLT_TYLE_64BIT:  c_int = 0x00000004;
pub const DLT_TYLE_128BIT: c_int = 0x00000005;

pub const DLT_SCOD_ASCII: c_int = 0x00000000;
pub const DLT_SCOD_UTF8:  c_int = 0x00008000;
pub const DLT_SCOD_HEX:   c_int = 0x00010000;
pub const DLT_SCOD_BIN:   c_int = 0x00018000;

// Definitions of DLT services
/// Service ID: Set log level
pub const DLT_SERVICE_ID_SET_LOG_LEVEL:                   c_int = 0x01;
/// Service ID: Set trace status
pub const DLT_SERVICE_ID_SET_TRACE_STATUS:                c_int = 0x02;
/// Service ID: Get log info
pub const DLT_SERVICE_ID_GET_LOG_INFO:                    c_int = 0x03;
/// Service ID: Get dafault log level
pub const DLT_SERVICE_ID_GET_DEFAULT_LOG_LEVEL:           c_int = 0x04;
/// Service ID: Store configuration
pub const DLT_SERVICE_ID_STORE_CONFIG:                    c_int = 0x05;
/// Service ID: Reset to factory defaults
pub const DLT_SERVICE_ID_RESET_TO_FACTORY_DEFAULT:        c_int = 0x06;
/// Service ID: Set communication interface status
pub const DLT_SERVICE_ID_SET_COM_INTERFACE_STATUS:        c_int = 0x07;
/// Service ID: Set communication interface maximum bandwidth
pub const DLT_SERVICE_ID_SET_COM_INTERFACE_MAX_BANDWIDTH: c_int = 0x08;
/// Service ID: Set verbose mode
pub const DLT_SERVICE_ID_SET_VERBOSE_MODE:                c_int = 0x09;
/// Service ID: Set message filtering
pub const DLT_SERVICE_ID_SET_MESSAGE_FILTERING:           c_int = 0x0A;
/// Service ID: Set timing packets
pub const DLT_SERVICE_ID_SET_TIMING_PACKETS:              c_int = 0x0B;
/// Service ID: Get local time
pub const DLT_SERVICE_ID_GET_LOCAL_TIME:                  c_int = 0x0C;
/// Service ID: Use ECU id
pub const DLT_SERVICE_ID_USE_ECU_ID:                      c_int = 0x0D;
/// Service ID: Use session id
pub const DLT_SERVICE_ID_USE_SESSION_ID:                  c_int = 0x0E;
/// Service ID: Use timestamp
pub const DLT_SERVICE_ID_USE_TIMESTAMP:                   c_int = 0x0F;
/// Service ID: Use extended header
pub const DLT_SERVICE_ID_USE_EXTENDED_HEADER:             c_int = 0x10;
/// Service ID: Set default log level
pub const DLT_SERVICE_ID_SET_DEFAULT_LOG_LEVEL:           c_int = 0x11;
/// Service ID: Set default trace status
pub const DLT_SERVICE_ID_SET_DEFAULT_TRACE_STATUS:        c_int = 0x12;
/// Service ID: Get software version
pub const DLT_SERVICE_ID_GET_SOFTWARE_VERSION:            c_int = 0x13;
/// Service ID: Message buffer overflow
pub const DLT_SERVICE_ID_MESSAGE_BUFFER_OVERFLOW:         c_int = 0x14;
/// Service ID: Last entry to avoid any further modifications in dependent code
pub const DLT_SERVICE_ID_LAST_ENTRY:                      c_int = 0x15;
/// Service ID: Message unregister context
pub const DLT_SERVICE_ID_UNREGISTER_CONTEXT:              c_int = 0xf01;
/// Service ID: Message connection info
pub const DLT_SERVICE_ID_CONNECTION_INFO:                 c_int = 0xf02;
/// Service ID: Timezone
pub const DLT_SERVICE_ID_TIMEZONE:                        c_int = 0xf03;
/// Service ID: Marker
pub const DLT_SERVICE_ID_MARKER:                          c_int = 0xf04;
/// Service ID: Offline log storage
pub const DLT_SERVICE_ID_OFFLINE_LOGSTORAGE:              c_int = 0xf05;
/// Service ID: (Dis)Connect passive Node
pub const DLT_SERVICE_ID_PASSIVE_NODE_CONNECT:            c_int = 0xf0E;
/// Service ID: Passive Node status information
pub const DLT_SERVICE_ID_PASSIVE_NODE_CONNECTION_STATUS:  c_int = 0xf0F;
/// Service ID: set all log level
pub const DLT_SERVICE_ID_SET_ALL_LOG_LEVEL:               c_int = 0xf10;
/// Service ID: Message Injection (minimal ID)
pub const DLT_SERVICE_ID_CALLSW_CINJECTION:               c_int = 0xFFF;

// Definitions of DLT service response status
/// Control message response: OK
pub const DLT_SERVICE_RESPONSE_OK:            c_int = 0x00;
/// Control message response: Not supported
pub const DLT_SERVICE_RESPONSE_NOT_SUPPORTED: c_int = 0x01;
/// Control message response: Error
pub const DLT_SERVICE_RESPONSE_ERROR:         c_int = 0x02;

// Definitions of DLT service connection state
/// Client is disconnected
pub const DLT_CONNECTION_STATUS_DISCONNECTED: c_int = 0x01;
/// Client is connected
pub const DLT_CONNECTION_STATUS_CONNECTED:    c_int = 0x02;