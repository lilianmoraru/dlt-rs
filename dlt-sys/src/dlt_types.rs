use libc::{ c_int };

pub static UINT16_MAX: u16 = 65535;

/// Definitions of DLT return values
#[repr(C)]
pub enum DltReturnValue {
    DLT_RETURN_LOGGING_DISABLED,
    DLT_RETURN_USER_BUFFER_FULL,
    DLT_RETURN_WRONG_PARAMETER,
    DLT_RETURN_BUFFER_FULL,
    DLT_RETURN_PIPE_FULL,
    DLT_RETURN_PIPE_ERROR,
    DLT_RETURN_ERROR,
    DLT_RETURN_OK,
    DLT_RETURN_TRUE
}

/// Definitions of DLT log level
#[repr(C)]
pub enum DltLogLevelType {
    /// Default log level
    DLT_LOG_DEFAULT,
    /// Log level off
    DLT_LOG_OFF,
    /// Fatal system error
    DLT_LOG_FATAL,
    /// Error with impact to correct functionality
    DLT_LOG_ERROR,
    /// Warning, correct behaviour could not be ensured
    DLT_LOG_WARN,
    /// Informational
    DLT_LOG_INFO,
    /// Debug
    DLT_LOG_DEBUG,
    /// Highest grade of information
    DLT_LOG_VERBOSE,
    /// Maximum value, used for range checking
    DLT_LOG_MAX,
}

/// Definitions of DLT Format
#[repr(C)]
pub enum DltFormatType {
    /// No special format
    DLT_FORMAT_DEFAULT,
    /// Hex 8
    DLT_FORMAT_HEX8,
    /// Hex 16
    DLT_FORMAT_HEX16,
    /// Hex 32
    DLT_FORMAT_HEX32,
    /// Hex 64
    DLT_FORMAT_HEX64,
    /// Binary 8
    DLT_FORMAT_BIN8,
    /// Binary 16
    DLT_FORMAT_BIN16,
    /// Maximum value, used for range checking
    DLT_FORMAT_MAX
}

/// Definitions of DLT trace status
#[repr(C)]
pub enum DltTraceStatusType {
    /// Default trace status
    DLT_TRACE_STATUS_DEFAULT,
    /// Trace status: Off
    DLT_TRACE_STATUS_OFF,
    /// Trace status: On
    DLT_TRACE_STATUS_ON,
    /// Maximum value, used for range checking
    DLT_TRACE_STATUS_MAX
}

/// Definitions for dlt_user_trace_network/DLT_TRACE_NETWORK()
/// as defined in the DLT protocol
#[repr(C)]
pub enum DltNetworkTraceType {
    /// Interprocess communication
    DLT_NW_TRACE_IPC,
    /// Controller Area Network Bus
    DLT_NW_TRACE_CAN,
    /// Flexray Bus
    DLT_NW_TRACE_FLEXRAY,
    /// Media Oriented System Transport Bus
    DLT_NW_TRACE_MOST,
    DLT_NW_TRACE_RESERVED0,
    DLT_NW_TRACE_RESERVED1,
    DLT_NW_TRACE_RESERVED2,
    DLT_NW_TRACE_USER_DEFINED0,
    DLT_NW_TRACE_USER_DEFINED1,
    DLT_NW_TRACE_USER_DEFINED2,
    DLT_NW_TRACE_USER_DEFINED3,
    DLT_NW_TRACE_USER_DEFINED4,
    DLT_NW_TRACE_USER_DEFINED5,
    DLT_NW_TRACE_USER_DEFINED6,
    DLT_NW_TRACE_USER_DEFINED7,
    /// Maximum value, used for range checking
    DLT_NW_TRACE_MAX
}

/// This are the log modes
#[repr(C)]
pub enum DltUserLogMode {
    DLT_USER_MODE_UNDEFINED,
    DLT_USER_MODE_OFF,
    DLT_USER_MODE_EXTERNAL,
    DLT_USER_MODE_INTERNAL,
    DLT_USER_MODE_BOTH,
    /// Maximum value, used for range checking
    DLT_USER_MODE_MAX
}

//-----------------------------------------------------------
impl From<DltReturnValue> for c_int {
    fn from(return_value: DltReturnValue) -> Self {
        match return_value {
            DltReturnValue::DLT_RETURN_LOGGING_DISABLED => -7,
            DltReturnValue::DLT_RETURN_USER_BUFFER_FULL => -6,
            DltReturnValue::DLT_RETURN_WRONG_PARAMETER  => -5,
            DltReturnValue::DLT_RETURN_BUFFER_FULL      => -4,
            DltReturnValue::DLT_RETURN_PIPE_FULL        => -3,
            DltReturnValue::DLT_RETURN_PIPE_ERROR       => -2,
            DltReturnValue::DLT_RETURN_ERROR            => -1,
            DltReturnValue::DLT_RETURN_OK               => 0,
            DltReturnValue::DLT_RETURN_TRUE             => 1
        }
    }
}

impl From<DltLogLevelType> for c_int {
    fn from(log_level: DltLogLevelType) -> Self {
        match log_level {
            DltLogLevelType::DLT_LOG_DEFAULT => -1,
            DltLogLevelType::DLT_LOG_OFF     => 0,
            DltLogLevelType::DLT_LOG_FATAL   => 1,
            DltLogLevelType::DLT_LOG_ERROR   => 2,
            DltLogLevelType::DLT_LOG_WARN    => 3,
            DltLogLevelType::DLT_LOG_INFO    => 4,
            DltLogLevelType::DLT_LOG_DEBUG   => 5,
            DltLogLevelType::DLT_LOG_VERBOSE => 6,
            DltLogLevelType::DLT_LOG_MAX     => 7
        }
    }
}

impl From<DltFormatType> for c_int {
    fn from(format: DltFormatType) -> Self {
        match format {
            DltFormatType::DLT_FORMAT_DEFAULT => 0,
            DltFormatType::DLT_FORMAT_HEX8    => 1,
            DltFormatType::DLT_FORMAT_HEX16   => 2,
            DltFormatType::DLT_FORMAT_HEX32   => 3,
            DltFormatType::DLT_FORMAT_HEX64   => 4,
            DltFormatType::DLT_FORMAT_BIN8    => 5,
            DltFormatType::DLT_FORMAT_BIN16   => 6,
            DltFormatType::DLT_FORMAT_MAX     => 7
        }
    }
}

impl From<DltTraceStatusType> for c_int {
    fn from(trace_status: DltTraceStatusType) -> Self {
        match trace_status {
            DltTraceStatusType::DLT_TRACE_STATUS_DEFAULT => -1,
            DltTraceStatusType::DLT_TRACE_STATUS_OFF     => 0,
            DltTraceStatusType::DLT_TRACE_STATUS_ON      => 1,
            DltTraceStatusType::DLT_TRACE_STATUS_MAX     => 2
        }
    }
}

impl From<DltNetworkTraceType> for c_int {
    fn from(network_trace_type: DltNetworkTraceType) -> Self {
        match network_trace_type {
            DltNetworkTraceType::DLT_NW_TRACE_IPC           => 1,
            DltNetworkTraceType::DLT_NW_TRACE_CAN           => 2,
            DltNetworkTraceType::DLT_NW_TRACE_FLEXRAY       => 3,
            DltNetworkTraceType::DLT_NW_TRACE_MOST          => 4,
            DltNetworkTraceType::DLT_NW_TRACE_RESERVED0     => 5,
            DltNetworkTraceType::DLT_NW_TRACE_RESERVED1     => 6,
            DltNetworkTraceType::DLT_NW_TRACE_RESERVED2     => 7,
            DltNetworkTraceType::DLT_NW_TRACE_USER_DEFINED0 => 8,
            DltNetworkTraceType::DLT_NW_TRACE_USER_DEFINED1 => 9,
            DltNetworkTraceType::DLT_NW_TRACE_USER_DEFINED2 => 10,
            DltNetworkTraceType::DLT_NW_TRACE_USER_DEFINED3 => 11,
            DltNetworkTraceType::DLT_NW_TRACE_USER_DEFINED4 => 12,
            DltNetworkTraceType::DLT_NW_TRACE_USER_DEFINED5 => 13,
            DltNetworkTraceType::DLT_NW_TRACE_USER_DEFINED6 => 14,
            DltNetworkTraceType::DLT_NW_TRACE_USER_DEFINED7 => 15,
            DltNetworkTraceType::DLT_NW_TRACE_MAX           => 16
        }
    }
}

impl From<DltUserLogMode> for c_int {
    fn from(log_mode: DltUserLogMode) -> Self {
        match log_mode {
            DltUserLogMode::DLT_USER_MODE_UNDEFINED => -1,
            DltUserLogMode::DLT_USER_MODE_OFF       => 0,
            DltUserLogMode::DLT_USER_MODE_EXTERNAL  => 1,
            DltUserLogMode::DLT_USER_MODE_INTERNAL  => 2,
            DltUserLogMode::DLT_USER_MODE_BOTH      => 3,
            DltUserLogMode::DLT_USER_MODE_MAX       => 4
        }
    }
}