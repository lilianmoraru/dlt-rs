/// Definitions of DLT return values
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DltReturnValue {
    DLT_RETURN_LOGGING_DISABLED = -7,
    DLT_RETURN_USER_BUFFER_FULL = -6,
    DLT_RETURN_WRONG_PARAMETER  = -5,
    DLT_RETURN_BUFFER_FULL      = -4,
    DLT_RETURN_PIPE_FULL        = -3,
    DLT_RETURN_PIPE_ERROR       = -2,
    DLT_RETURN_ERROR            = -1,
    DLT_RETURN_OK               = 0,
    DLT_RETURN_TRUE             = 1
}

/// Definitions of DLT log level
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DltLogLevelType {
    /// Default log level
    DLT_LOG_DEFAULT = -1,
    /// Log level off
    DLT_LOG_OFF     = 0,
    /// Fatal system error
    DLT_LOG_FATAL   = 1,
    /// Error with impact to correct functionality
    DLT_LOG_ERROR   = 2,
    /// Warning, correct behaviour could not be ensured
    DLT_LOG_WARN    = 3,
    /// Informational
    DLT_LOG_INFO    = 4,
    /// Debug
    DLT_LOG_DEBUG   = 5,
    /// Highest grade of information
    DLT_LOG_VERBOSE = 6,
    /// Maximum value, used for range checking
    DLT_LOG_MAX     = 7
}

/// Definitions of DLT Format
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DltFormatType {
    /// No special format
    DLT_FORMAT_DEFAULT = 0,
    /// Hex 8
    DLT_FORMAT_HEX8    = 1,
    /// Hex 16
    DLT_FORMAT_HEX16   = 2,
    /// Hex 32
    DLT_FORMAT_HEX32   = 3,
    /// Hex 64
    DLT_FORMAT_HEX64   = 4,
    /// Binary 8
    DLT_FORMAT_BIN8    = 5,
    /// Binary 16
    DLT_FORMAT_BIN16   = 6,
    /// Maximum value, used for range checking
    DLT_FORMAT_MAX     = 7
}

/// Definitions of DLT trace status
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DltTraceStatusType {
    /// Default trace status
    DLT_TRACE_STATUS_DEFAULT = -1,
    /// Trace status: Off
    DLT_TRACE_STATUS_OFF     = 0,
    /// Trace status: On
    DLT_TRACE_STATUS_ON      = 1,
    /// Maximum value, used for range checking
    DLT_TRACE_STATUS_MAX     = 2
}

/// Definitions for dlt_user_trace_network/DLT_TRACE_NETWORK()
/// as defined in the DLT protocol
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DltNetworkTraceType {
    /// Interprocess communication
    DLT_NW_TRACE_IPC           = 1,
    /// Controller Area Network Bus
    DLT_NW_TRACE_CAN           = 2,
    /// Flexray Bus
    DLT_NW_TRACE_FLEXRAY       = 3,
    /// Media Oriented System Transport Bus
    DLT_NW_TRACE_MOST          = 4,
    DLT_NW_TRACE_RESERVED0     = 5,
    DLT_NW_TRACE_RESERVED1     = 6,
    DLT_NW_TRACE_RESERVED2     = 7,
    DLT_NW_TRACE_USER_DEFINED0 = 8,
    DLT_NW_TRACE_USER_DEFINED1 = 9,
    DLT_NW_TRACE_USER_DEFINED2 = 10,
    DLT_NW_TRACE_USER_DEFINED3 = 11,
    DLT_NW_TRACE_USER_DEFINED4 = 12,
    DLT_NW_TRACE_USER_DEFINED5 = 13,
    DLT_NW_TRACE_USER_DEFINED6 = 14,
    DLT_NW_TRACE_USER_DEFINED7 = 15,
    /// Maximum value, used for range checking
    DLT_NW_TRACE_MAX           = 16
}

/// This are the log modes
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DltUserLogMode {
    DLT_USER_MODE_UNDEFINED = -1,
    DLT_USER_MODE_OFF       = 0,
    DLT_USER_MODE_EXTERNAL  = 1,
    DLT_USER_MODE_INTERNAL  = 2,
    DLT_USER_MODE_BOTH      = 3,
    /// Maximum value, used for range checking
    DLT_USER_MODE_MAX       = 4
}
