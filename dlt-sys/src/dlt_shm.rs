use libc::{ c_int, c_uint, c_uchar };
use dlt_common::DltBuffer;
use dlt_types::DltReturnValue;

#[repr(C)]
pub struct DltShm {
    /// Id of shared memory
    pub shmid: c_int,
    /// Id of semaphore
    pub semid: c_int,
    pub buffer: DltBuffer
}

extern {
    fn dlt_shm_init_client(buf: *mut DltShm, key: c_int) -> DltReturnValue;
    fn dlt_shm_init_server(buf: *mut DltShm, key: c_int, size: c_int) -> DltReturnValue;
    fn dlt_shm_push(buf: *mut DltShm, data1: *const c_uchar, size1: c_uint, data2: *const c_uchar, size2: c_uint, data3: *const c_uchar, size3: c_uint) -> c_int;
    fn dlt_shm_pull(buf: *mut DltShm, data: *mut c_uchar, size: c_int) -> c_int;
    fn dlt_shm_copy(buf: *mut DltShm, data: *mut c_uchar, size: c_int) -> c_int;
    fn dlt_shm_remove(buf: *mut DltShm) -> c_int;
    fn dlt_shm_info(buf: *mut DltShm);
    fn dlt_shm_status(buf: *mut DltShm);
    fn dlt_shm_free_client(buf: *mut DltShm) -> DltReturnValue;
    fn dlt_shm_get_total_size(buf: *mut DltShm) -> c_int;
    fn dlt_shm_get_used_size(buf: *mut DltShm) -> c_int;
    fn dlt_shm_get_message_count(buf: *mut DltShm) -> c_int;
    fn dlt_shm_reset(buf: *mut DltShm) -> c_int;
    fn dlt_shm_recover(buf: *mut DltShm) -> c_int;
    fn dlt_shm_free_server(buf: *mut DltShm) -> c_int;
}

