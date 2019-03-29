#![allow(non_camel_case_types)]

use crate::ptr::{Array, WasmPtr};
use wasmer_runtime_core::types::{ValueError, ValueType};

pub type __wasi_advice_t = u8;
pub const __WASI_ADVICE_DONTNEED: u8 = 0;
pub const __WASI_ADVICE_NOREUSE: u8 = 1;
pub const __WASI_ADVICE_NORMAL: u8 = 2;
pub const __WASI_ADVICE_RANDOM: u8 = 3;
pub const __WASI_ADVICE_SEQUENTIAL: u8 = 4;
pub const __WASI_ADVICE_WILLNEED: u8 = 5;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct __wasi_ciovec_t {
    pub buf: WasmPtr<u8, Array>,
    pub buf_len: u32,
}

pub type __wasi_clockid_t = u32;
pub const __WASI_CLOCK_MONOTONIC: u32 = 0;
pub const __WASI_CLOCK_PROCESS_CPUTIME_ID: u32 = 1;
pub const __WASI_CLOCK_REALTIME: u32 = 2;
pub const __WASI_CLOCK_THREAD_CPUTIME_ID: u32 = 3;

pub type __wasi_device_t = u64;

pub type __wasi_dircookie_t = u64;
pub const __WASI_DIRCOOKIE_START: u64 = 0;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct __wasi_dirent_t {
    pub d_next: __wasi_dircookie_t,
    pub d_ino: __wasi_inode_t,
    pub _namlen: u32,
    pub d_type: __wasi_filetype_t,
}

pub type __wasi_errno_t = u16;
pub const __WASI_ESUCCESS: u16 = 0;
pub const __WASI_E2BIG: u16 = 1;
pub const __WASI_EACCES: u16 = 2;
pub const __WASI_EADDRINUSE: u16 = 3;
pub const __WASI_EADDRNOTAVAIL: u16 = 4;
pub const __WASI_EAFNOSUPPORT: u16 = 5;
pub const __WASI_EAGAIN: u16 = 6;
pub const __WASI_EALREADY: u16 = 7;
pub const __WASI_EBADF: u16 = 8;
pub const __WASI_EBADMSG: u16 = 9;
pub const __WASI_EBUSY: u16 = 10;
pub const __WASI_ECANCELED: u16 = 11;
pub const __WASI_ECHILD: u16 = 12;
pub const __WASI_ECONNABORTED: u16 = 13;
pub const __WASI_ECONNREFUSED: u16 = 14;
pub const __WASI_ECONNRESET: u16 = 15;
pub const __WASI_EDEADLK: u16 = 16;
pub const __WASI_EDESTADDRREQ: u16 = 17;
pub const __WASI_EDOM: u16 = 18;
pub const __WASI_EDQUOT: u16 = 19;
pub const __WASI_EEXIST: u16 = 20;
pub const __WASI_EFAULT: u16 = 21;
pub const __WASI_EFBIG: u16 = 22;
pub const __WASI_EHOSTUNREACH: u16 = 23;
pub const __WASI_EIDRM: u16 = 24;
pub const __WASI_EILSEQ: u16 = 25;
pub const __WASI_EINPROGRESS: u16 = 26;
pub const __WASI_EINTR: u16 = 27;
pub const __WASI_EINVAL: u16 = 28;
pub const __WASI_EIO: u16 = 29;
pub const __WASI_EISCONN: u16 = 30;
pub const __WASI_EISDIR: u16 = 31;
pub const __WASI_ELOOP: u16 = 32;
pub const __WASI_EMFILE: u16 = 33;
pub const __WASI_EMLINK: u16 = 34;
pub const __WASI_EMSGSIZE: u16 = 35;
pub const __WASI_EMULTIHOP: u16 = 36;
pub const __WASI_ENAMETOOLONG: u16 = 37;
pub const __WASI_ENETDOWN: u16 = 38;
pub const __WASI_ENETRESET: u16 = 39;
pub const __WASI_ENETUNREACH: u16 = 40;
pub const __WASI_ENFILE: u16 = 41;
pub const __WASI_ENOBUFS: u16 = 42;
pub const __WASI_ENODEV: u16 = 43;
pub const __WASI_ENOENT: u16 = 44;
pub const __WASI_ENOEXEC: u16 = 45;
pub const __WASI_ENOLCK: u16 = 46;
pub const __WASI_ENOLINK: u16 = 47;
pub const __WASI_ENOMEM: u16 = 48;
pub const __WASI_ENOMSG: u16 = 49;
pub const __WASI_ENOPROTOOPT: u16 = 50;
pub const __WASI_ENOSPC: u16 = 51;
pub const __WASI_ENOSYS: u16 = 52;
pub const __WASI_ENOTCONN: u16 = 53;
pub const __WASI_ENOTDIR: u16 = 54;
pub const __WASI_ENOTEMPTY: u16 = 55;
pub const __WASI_ENOTRECOVERABLE: u16 = 56;
pub const __WASI_ENOTSOCK: u16 = 57;
pub const __WASI_ENOTSUP: u16 = 58;
pub const __WASI_ENOTTY: u16 = 59;
pub const __WASI_ENXIO: u16 = 60;
pub const __WASI_EOVERFLOW: u16 = 61;
pub const __WASI_EOWNERDEAD: u16 = 62;
pub const __WASI_EPERM: u16 = 63;
pub const __WASI_EPIPE: u16 = 64;
pub const __WASI_EPROTO: u16 = 65;
pub const __WASI_EPROTONOSUPPORT: u16 = 66;
pub const __WASI_EPROTOTYPE: u16 = 67;
pub const __WASI_ERANGE: u16 = 68;
pub const __WASI_EROFS: u16 = 69;
pub const __WASI_ESPIPE: u16 = 70;
pub const __WASI_ESRCH: u16 = 71;
pub const __WASI_ESTALE: u16 = 72;
pub const __WASI_ETIMEDOUT: u16 = 73;
pub const __WASI_ETXTBSY: u16 = 74;
pub const __WASI_EXDEV: u16 = 75;
pub const __WASI_ENOTCAPABLE: u16 = 76;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct __wasi_event_fd_readwrite_t {
    pub nbytes: __wasi_filesize_t,
    pub flags: __wasi_eventrwflags_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union __wasi_event_u {
    fd_readwrite: __wasi_event_fd_readwrite_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct __wasi_event_t {
    pub userdata: __wasi_userdata_t,
    pub error: __wasi_errno_t,
    pub type_: __wasi_eventtype_t,
    pub u: __wasi_event_u,
}

pub type __wasi_eventrwflags_t = u16;
pub const __WASI_EVENT_FD_READWRITE_HANGUP: u16 = 1 << 0;

pub type __wasi_eventtype_t = u8;
pub const __WASI_EVENTTYPE_CLOCK: u8 = 0;
pub const __WASI_EVENTTYPE_FD_READ: u8 = 1;
pub const __WASI_EVENTTYPE_FD_WRITE: u8 = 2;

pub type __wasi_exitcode_t = u32;

pub type __wasi_fd_t = u32;
pub const __WASI_STDIN_FILENO: u32 = 0;
pub const __WASI_STDOUT_FILENO: u32 = 1;
pub const __WASI_STDERR_FILENO: u32 = 2;

pub type __wasi_fdflags_t = u16;
pub const __WASI_FDFLAG_APPEND: u16 = 1 << 0;
pub const __WASI_FDFLAG_DSYNC: u16 = 1 << 1;
pub const __WASI_FDFLAG_NONBLOCK: u16 = 1 << 2;
pub const __WASI_FDFLAG_RSYNC: u16 = 1 << 3;
pub const __WASI_FDFLAG_SYNC: u16 = 1 << 4;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct __wasi_fdstat_t {
    pub fs_filetype: __wasi_filetype_t,
    pub fs_flags: __wasi_fdflags_t,
    pub fs_rights_base: __wasi_rights_t,
    pub fs_rights_inheriting: __wasi_rights_t,
}

pub type __wasi_filedelta_t = i64;

pub type __wasi_filesize_t = u64;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct __wasi_filestat_t {
    pub st_dev: __wasi_device_t,
    pub st_ino: __wasi_inode_t,
    pub st_filetype: __wasi_filetype_t,
    pub st_nlink: __wasi_linkcount_t,
    pub st_size: __wasi_filesize_t,
    pub st_atim: __wasi_timestamp_t,
    pub st_mtim: __wasi_timestamp_t,
    pub st_ctim: __wasi_timestamp_t,
}

pub type __wasi_filetype_t = u8;
pub const __WASI_FILETYPE_UNKNOWN: u8 = 0;
pub const __WASI_FILETYPE_BLOCK_DEVICE: u8 = 1;
pub const __WASI_FILETYPE_CHARACTER_DEVICE: u8 = 2;
pub const __WASI_FILETYPE_DIRECTORY: u8 = 3;
pub const __WASI_FILETYPE_REGULAR_FILE: u8 = 4;
pub const __WASI_FILETYPE_SOCKET_DGRAM: u8 = 5;
pub const __WASI_FILETYPE_SOCKET_STREAM: u8 = 6;
pub const __WASI_FILETYPE_SYMBOLIC_LINK: u8 = 7;

pub type __wasi_fstflags_t = u16;
pub const __WASI_FILESTAT_SET_ATIM: u16 = 1 << 0;
pub const __WASI_FILESTAT_SET_ATIM_NOW: u16 = 1 << 1;
pub const __WASI_FILESTAT_SET_MTIM: u16 = 1 << 2;
pub const __WASI_FILESTAT_SET_MTIM_NOW: u16 = 1 << 3;

pub type __wasi_inode_t = u64;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct __wasi_iovec_t {
    pub buf: WasmPtr<u8, Array>,
    pub buf_len: u32,
}

pub type __wasi_linkcount_t = u32;

pub type __wasi_lookupflags_t = u32;
pub const __WASI_LOOKUP_SYMLINK_FOLLOW: u32 = 1 << 0;

pub type __wasi_oflags_t = u16;
pub const __WASI_O_CREAT: u16 = 1 << 0;
pub const __WASI_O_DIRECTORY: u16 = 1 << 1;
pub const __WASI_O_EXCL: u16 = 1 << 2;
pub const __WASI_O_TRUNC: u16 = 1 << 3;

pub type __wasi_riflags_t = u16;
pub const __WASI_SOCK_RECV_PEEK: u16 = 1 << 0;
pub const __WASI_SOCK_RECV_WAITALL: u16 = 1 << 1;

pub type __wasi_rights_t = u64;
pub const __WASI_RIGHT_FD_DATASYNC: u64 = 1 << 0;
pub const __WASI_RIGHT_FD_READ: u64 = 1 << 1;
pub const __WASI_RIGHT_FD_SEEK: u64 = 1 << 2;
pub const __WASI_RIGHT_FD_FDSTAT_SET_FLAGS: u64 = 1 << 3;
pub const __WASI_RIGHT_FD_SYNC: u64 = 1 << 4;
pub const __WASI_RIGHT_FD_TELL: u64 = 1 << 5;
pub const __WASI_RIGHT_FD_WRITE: u64 = 1 << 6;
pub const __WASI_RIGHT_FD_ADVISE: u64 = 1 << 7;
pub const __WASI_RIGHT_FD_ALLOCATE: u64 = 1 << 8;
pub const __WASI_RIGHT_PATH_CREATE_DIRECTORY: u64 = 1 << 9;
pub const __WASI_RIGHT_PATH_CREATE_FILE: u64 = 1 << 10;
pub const __WASI_RIGHT_PATH_LINK_SOURCE: u64 = 1 << 11;
pub const __WASI_RIGHT_PATH_LINK_TARGET: u64 = 1 << 12;
pub const __WASI_RIGHT_PATH_OPEN: u64 = 1 << 13;
pub const __WASI_RIGHT_FD_READDIR: u64 = 1 << 14;
pub const __WASI_RIGHT_PATH_READLINK: u64 = 1 << 15;
pub const __WASI_RIGHT_PATH_RENAME_SOURCE: u64 = 1 << 16;
pub const __WASI_RIGHT_PATH_RENAME_TARGET: u64 = 1 << 17;
pub const __WASI_RIGHT_PATH_FILESTAT_GET: u64 = 1 << 18;
pub const __WASI_RIGHT_PATH_FILESTAT_SET_SIZE: u64 = 1 << 19;
pub const __WASI_RIGHT_PATH_FILESTAT_SET_TIMES: u64 = 1 << 20;
pub const __WASI_RIGHT_FD_FILESTAT_GET: u64 = 1 << 21;
pub const __WASI_RIGHT_FD_FILESTAT_SET_SIZE: u64 = 1 << 22;
pub const __WASI_RIGHT_FD_FILESTAT_SET_TIMES: u64 = 1 << 23;
pub const __WASI_RIGHT_PATH_SYMLINK: u64 = 1 << 24;
pub const __WASI_RIGHT_PATH_UNLINK_FILE: u64 = 1 << 25;
pub const __WASI_RIGHT_PATH_REMOVE_DIRECTORY: u64 = 1 << 26;
pub const __WASI_RIGHT_POLL_FD_READWRITE: u64 = 1 << 27;
pub const __WASI_RIGHT_SOCK_SHUTDOWN: u64 = 1 << 28;

pub type __wasi_roflags_t = u16;
pub const __WASI_SOCK_RECV_DATA_TRUNCATED: u16 = 1 << 0;

pub type __wasi_sdflags_t = u8;
pub const __WASI_SHUT_RD: u8 = 1 << 0;
pub const __WASI_SHUT_WR: u8 = 1 << 1;

pub type __wasi_siflags_t = u16;

pub type __wasi_signal_t = u8;
pub const __WASI_SIGABRT: u8 = 0;
pub const __WASI_SIGALRM: u8 = 1;
pub const __WASI_SIGBUS: u8 = 2;
pub const __WASI_SIGCHLD: u8 = 3;
pub const __WASI_SIGCONT: u8 = 4;
pub const __WASI_SIGFPE: u8 = 5;
pub const __WASI_SIGHUP: u8 = 6;
pub const __WASI_SIGILL: u8 = 7;
pub const __WASI_SIGINT: u8 = 8;
pub const __WASI_SIGKILL: u8 = 9;
pub const __WASI_SIGPIPE: u8 = 10;
pub const __WASI_SIGQUIT: u8 = 11;
pub const __WASI_SIGSEGV: u8 = 12;
pub const __WASI_SIGSTOP: u8 = 13;
pub const __WASI_SIGSYS: u8 = 14;
pub const __WASI_SIGTERM: u8 = 15;
pub const __WASI_SIGTRAP: u8 = 16;
pub const __WASI_SIGTSTP: u8 = 17;
pub const __WASI_SIGTTIN: u8 = 18;
pub const __WASI_SIGTTOU: u8 = 19;
pub const __WASI_SIGURG: u8 = 20;
pub const __WASI_SIGUSR1: u8 = 21;
pub const __WASI_SIGUSR2: u8 = 22;
pub const __WASI_SIGVTALRM: u8 = 23;
pub const __WASI_SIGXCPU: u8 = 24;
pub const __WASI_SIGXFSZ: u8 = 25;

pub type __wasi_subclockflags_t = u16;
pub const __WASI_SUBSCRIPTION_CLOCK_ABSTIME: u16 = 1 << 0;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct __wasi_subscription_clock_t {
    pub userdata: __wasi_userdata_t,
    pub clock_id: __wasi_clockid_t,
    pub timeout: __wasi_timestamp_t,
    pub precision: __wasi_timestamp_t,
    pub flags: __wasi_subclockflags_t,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct __wasi_subscription_fs_readwrite_t {
    pub fd: __wasi_fd_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub union __wasi_subscription_u {
    clock: __wasi_subscription_clock_t,
    fd_readwrite: __wasi_subscription_fs_readwrite_t,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct __wasi_subscription_t {
    pub userdata: __wasi_userdata_t,
    pub type_: __wasi_eventtype_t,
    pub u: __wasi_subscription_u,
}

pub type __wasi_timestamp_t = u64;

pub type __wasi_userdata_t = u64;

pub type __wasi_whence_t = u8;
pub const __WASI_WHENCE_CUR: u8 = 0;
pub const __WASI_WHENCE_END: u8 = 1;
pub const __WASI_WHENCE_SET: u8 = 2;
