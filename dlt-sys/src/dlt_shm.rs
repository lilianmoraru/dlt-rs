use libc::{ c_int, c_uint, c_char, c_uchar };
use dlt_common::DltBuffer;
use dlt_types::DltReturnValue;

/// Shared memory key.
/// Must be the same for server and client
pub const DLT_SHM_KEY: c_uint = 11771;

/// Default size of shared memory.
/// Size is extended during creation to fit segment size.
/// Client retrieves real size from shm buffer
pub const DLT_SHM_SIZE: c_uint = 100000;

/// ID of the used semaphore.
/// Used for synchronisation of write and read access of multiple clients and server.
/// Must be the same for server and client
pub const DLT_SHM_SEM: c_uint = 22771;

pub const DLT_SHM_HEAD: &'static [u8; 4] = b"SHM\x00";

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DltShm {
    /// ID of shared memory
    pub shmid: c_int,
    /// ID of semaphore
    pub semid: c_int,
    pub buffer: DltBuffer
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct DltShmBlockHead {
    pub head: [c_char; 4],
    pub status: c_uchar,
    pub size: c_int
}

extern {
    /// Initialise the shared memory on the client side.
    /// This function must be called before using further shm functions.
    ///
    /// `buf` - pointer to shm structure
    ///
    /// `key` - the identifier of the shm, must be the same for server and client
    ///
    /// ### Returns
    /// Negative value if there was an error
    pub fn dlt_shm_init_client(buf: *mut DltShm, key: c_int) -> DltReturnValue;

    /// Initialise the shared memory on the server side.
    /// This function must be called before using further shm functions.
    ///
    /// `buf` - pointer to shm structure
    ///
    /// `key` - the identifier of the shm, must be the same for server and client
    ///
    /// `size` - the requested size of the shm
    ///
    /// ### Returns
    /// Negative value if there was an error
    pub fn dlt_shm_init_server(buf: *mut DltShm, key: c_int, size: c_int) -> DltReturnValue;

    /// Push data from client onto the shm.
    ///
    /// `data[1-3]` - pointer to [1-3] data block to be written, null if not used
    ///
    /// `size[1-3]` - size in bytes of [1-3] data block to be written, 0 if not used
    ///
    /// ### Returns
    /// Negative value if there was an error
    pub fn dlt_shm_push(buf: *mut DltShm, data1: *const c_uchar, size1: c_uint, data2: *const c_uchar, size2: c_uint, data3: *const c_uchar, size3: c_uint) -> c_int;

    /// Pull data from shm.
    /// This function should be called from client.
    /// Data is deleted from shm after this call.
    ///
    /// `buf` - pointer to shm structure
    ///
    /// `data` - pointer to buffer where data is to be written
    ///
    /// `size` - maximum size to be written into buffer
    ///
    /// ### Returns
    /// Negative value if there was an error
    pub fn dlt_shm_pull(buf: *mut DltShm, data: *mut c_uchar, size: c_int) -> c_int;

    /// Copy message from shm.
    /// This function should be called from server.
    /// Data is not deleted from shm after this call.
    ///
    /// `buf` - pointer to shm structure
    ///
    /// `data` - pointer to buffer where data is to be written
    ///
    /// `size` - maximum size to be written into buffer
    ///
    /// ### Returns
    /// Negative value if there was an error
    pub fn dlt_shm_copy(buf: *mut DltShm, data: *mut c_uchar, size: c_int) -> c_int;

    /// Delete message from shm.
    /// This function should be called from server.
    /// This function should be called after each succesful copy.
    ///
    /// `buf` - pointer to shm structure
    ///
    /// ### Returns
    /// Negative value if there was an error
    pub fn dlt_shm_remove(buf: *mut DltShm) -> c_int;

    /// Print information about shm.
    ///
    /// `buf` - pointer to shm structure
    pub fn dlt_shm_info(buf: *mut DltShm);

    /// Print status about shm.
    ///
    /// `buf` - pointer to shm structure
    pub fn dlt_shm_status(buf: *mut DltShm);

    /// Deinitialise the shared memory on the client side.
    ///
    /// `buf` - pointer to shm structure
    ///
    /// ### Returns
    /// Negative value if there was an error
    pub fn dlt_shm_free_client(buf: *mut DltShm) -> DltReturnValue;

    /// Returns the used size of the shm.
    ///
    /// `buf` - pointer to shm structure
    ///
    /// ### Returns
    /// Size of the shared memory
    pub fn dlt_shm_get_total_size(buf: *mut DltShm) -> c_int;

    /// Returns the used size in the shm.
    ///
    /// `buf` - pointer to shm structure
    ///
    /// ### Returns
    /// Size of the shared memory
    pub fn dlt_shm_get_used_size(buf: *mut DltShm) -> c_int;

    /// Return the number of messages in the shm.
    ///
    /// `buf` - pointer to shm structure
    ///
    /// ### Returns
    /// Size of the shared memory
    pub fn dlt_shm_get_message_count(buf: *mut DltShm) -> c_int;

    /// Reset pointers and counters when shm corrupted.
    ///
    /// `buf` - pointer to shm structure
    ///
    /// ### Returns
    /// Size of the shared memory
    pub fn dlt_shm_reset(buf: *mut DltShm) -> c_int;

    /// Recover to find next valid message.
    ///
    /// `buf` - pointer to shm structure
    ///
    /// ### Returns
    /// Size of the shared memory
    pub fn dlt_shm_recover(buf: *mut DltShm) -> c_int;

    /// Deinitialise the shared memory on the server side.
    ///
    /// `buf` - pointer to shm structure
    ///
    /// ### Returns
    /// Negative value if there was an error
    pub fn dlt_shm_free_server(buf: *mut DltShm) -> c_int;
}

