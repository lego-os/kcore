use core::fmt::Display;
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Errno {
    /// Operation not permitted
    EPERM = 1,
    /// No such file or directory
    ENOENT = 2,
    /// No such process
    ESRCH = 3,
    /// Interrupted system call
    EINTR = 4,
    /// Input/output  error
    EIO = 5,
    /// No such device or address
    ENXIO = 6,
    /// Argument list too long
    E2BIG = 7,
    /// Exec format error
    ENOEXEC = 8,
    /// Bad file descriptor
    EBADF = 9,
    /// No child processes
    ECHILD = 10,
    /// Resource deadlock avoided
    EDEADLK = 11,
    /// Cannot allocate memory
    ENOMEM = 12,
    /// Permission denied
    EACCES = 13,
    /// Bad address
    EFAULT = 14,
    /// Block device required
    ENOTBLK = 15,
    /// Device or resource busy
    EBUSY = 16,
    /// File exists
    EEXIST = 17,
    /// Invalid cross-device link
    EXDEV = 18,
    /// No such device
    ENODEV = 19,
    /// Not a directory
    ENOTDIR = 20,
    /// Is a directory
    EISDIR = 21,
    /// Invalid argument
    EINVAL = 22,
    /// Too many open files
    EMFILE = 23,
    /// File table overflow
    ENFILE = 24,
    /// Inappropriate ioctl for device
    ENOTTY = 25,
    /// Text file busy
    ETXTBSY = 26,
    /// File too large
    EFBIG = 27,
    /// No space left on device
    ENOSPC = 28,
    /// Illegal seek
    ESPIPE = 29,
    /// Read-only file system
    EROFS = 30,
    /// Too many links
    EMLINK = 31,
    /// Broken pipe
    EPIPE = 32,
    /// Numerical argument out of domain
    EDOM = 33,
    /// Numerical result out of range
    ERANGE = 34,
    /// Resource temporarily unavailable
    EAGAIN = 35,
    /// Operation would block
    EWOULDBLOCK = 36,
    /// Operation now in progress
    EINPROGRESS = 37,
    /// Operation already in progress
    EALREADY = 38,
    /// Socket operation on non-socket
    ENOTSOCK = 39,
    /// Message too long
    EMSGSIZE = 40,
    /// Protocol wrong type for socket
    EPROTOTYPE = 41,
    /// Protocol not available
    ENOPROTOOPT = 42,
    /// Protocol not supported
    EPROTONOSUPPORT = 43,
    /// Socket type not supported
    ESOCKTNOSUPPORT = 44,
    /// Operation not supported
    EOPNOTSUPP = 45,
    /// Protocol family not supported
    EPFNOSUPPORT = 46,
    /// Address family not supported by protocol
    EAFNOSUPPORT = 47,
    /// Address already in use
    EADDRINUSE = 48,
    /// Cannot assign requested address
    EADDRNOTAVAIL = 49,
    /// Network is down
    ENETDOWN = 50,
    /// Network is unreachable
    ENETUNREACH = 51,
    /// Network dropped connection on reset
    ENETRESET = 52,
    /// Software caused connection abort
    ECONNABORTED = 53,
    /// Connection reset by peer
    ECONNRESET = 54,
    /// No buffer space available
    ENOBUFS = 55,
    /// Transport endpoint is already connected
    EISCONN = 56,
    /// Transport endpoint is not connected
    ENOTCONN = 57,
    /// Destination address required
    EDESTADDRREQ = 58,
    /// Cannot  send  after  transport  endpoint  shutdown
    ESHUTDOWN = 59,
    /// Too many references: cannot splice
    ETOOMANYREFS = 60,
    /// Connection timed out
    ETIMEDOUT = 61,
    /// Connection refused
    ECONNREFUSED = 62,
    /// Too many levels of symbolic links
    ELOOP = 63,
    /// File name too long
    ENAMETOOLONG = 64,
    /// Host is down
    EHOSTDOWN = 65,
    /// No route  to host
    EHOSTUNREACH = 66,
    /// Directory not empty
    ENOTEMPTY = 67,
    /// Too many processes
    EPROCLIM = 68,
    /// Too many users
    EUSERS = 69,
    /// Disk quota exceeded
    EDQUOT = 70,
    /// Stale file handle
    ESTALE = 71,
    /// Object is remote
    EREMOTE = 72,
    /// RPC struct is bad
    EBADRPC = 73,
    /// RPC version wrong
    ERPCMISMATCH = 74,
    /// RPC program not available
    EPROGUNAVAIL = 75,
    /// RPC program version wrong
    EPROGMISMATCH = 76,
    /// RPC bad procedure for program
    EPROCUNAVAIL = 77,
    /// No locks available
    ENOLCK = 78,
    /// Inappropriate file type or format
    EFTYPE = 79,
    /// Authentication error
    EAUTH = 80,
    /// Need authenticator
    ENEEDAUTH = 81,
    /// Function not implemented
    ENOSYS = 82,
    /// Cannot exec a shared library directly
    ELIBEXEC = 83,
    // Not supported
    ENOTSUP = 84,
    /// Invalid or incomplete multibyte or wide character
    EILSEQ = 85,
    /// Inappropriate operation for background process
    EBACKGROUND = 86,
    /// Translator died
    EDIED = 87,
    /// The experienced user will know what is wrong
    ED = 88,
    /// You  really  blew  it  this  time
    EGREGIOUS = 89,
    /// Computer bought the farm
    EIEIO = 90,
    /// Gratuitous error
    EGRATUITOUS = 91,
    /// Bad message
    EBADMSG = 92,
    /// Identifier  removed
    EIDRM = 93,
    /// Multihop  attempted
    EMULTIHOP = 94,
    /// No  data  available
    ENODATA = 95,
    /// Link has been severed
    ENOLINK = 96,
    /// No message of desired type
    ENOMSG = 97,
    /// Out  of streams resources
    ENOSR = 98,
    /// Device not a stream
    ENOSTR = 99,
    /// Value too large for defined data type
    EOVERFLOW = 100,
    /// Protocol error
    EPROTO = 101,
    /// Timer expired
    ETIME = 102,
    /// Operation canceled
    ECANCELED = 103,
    ///Owner  died
    EOWNERDEAD = 104,
    /// State not recoverable
    ENOTRECOVERABLE = 105,
    /// Interrupted system call should be restarted
    ERESTART = 106,
    /// Channel number out of range
    ECHRNG = 107,
    /// Level 2 not synchronized
    EL2NSYNC = 108,
    /// Level 3 halted
    EL3HLT = 109,
    /// Level 3 reset
    EL3RST = 110,
    /// Link number out of range
    ELNRNG = 111,
    /// Protocol driver not attached
    EUNATCH = 112,
    /// No CSI structure available
    ENOCSI = 113,
    /// Level 2 halted
    EL2HLT = 114,
    /// Invalid exchange
    EBADE = 115,
    /// Invalid request descriptor
    EBADR = 116,
    /// Exchange full
    EXFULL = 117,
    /// No anode
    ENOANO = 118,
    /// Invalid request code
    EBADRQC = 119,
    /// Invalid slot
    EBADSLT = 120,
    /// File locking deadlock error
    EDEADLOCK = 121,
    /// Bad font file format
    EBFONT = 122,
    /// Machine is not on the network
    ENONET = 123,
    /// Package not installed
    ENOPKG = 124,
    /// Advertise error
    EADV = 125,
    /// Srmount error
    ESRMNT = 126,
    /// Communication error on send
    ECOMM = 127,
    /// RFS specific error
    EDOTDOT = 128,
    /// Name not unique on network
    ENOTUNIQ = 129,
    /// File descriptor in bad state
    EBADFD = 130,
    /// Remote address changed
    EREMCHG = 131,
    /// Can not access a needed shared library
    ELIBACC = 132,
    /// Accessing a corrupted shared library
    ELIBBAD = 133,
    /// .lib  section  in  a.out  corrupted
    ELIBSCN = 134,
    /// Attempting to link in too many shared libraries
    ELIBMAX = 135,
    /// Streams pipe error
    ESTRPIPE = 136,
    /// Structure needs cleaning
    EUCLEAN = 137,
    /// Not a XENIX named type file
    ENOTNAM = 138,
    /// No XENIX semaphores available
    ENAVAIL = 139,
    /// Is a named type file
    EISNAM = 140,
    /// Remote I/O error
    EREMOTEIO = 141,
    /// No medium found
    ENOMEDIUM = 142,
    /// Wrong medium type
    EMEDIUMTYPE = 143,
    /// Required key not available
    ENOKEY = 144,
    /// Key has expired
    EKEYEXPIRED = 145,
    /// Key has been revoked
    EKEYREVOKED = 146,
    /// Key was rejected by service
    EKEYREJECTED = 147,
    /// Operation not possible due to RF-kill
    ERFKILL = 148,
    /// Memory page has hardware error
    EHWPOISON = 149,
}

impl Display for Errno {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::EPERM => write!(f, "ERROR[{}] Operation not permitted", *self as u32),
            Self::ENOENT => write!(f, "ERROR[{}] No such file or directory", *self as u32),
            Self::ESRCH => write!(f, "ERROR[{}] No such process", *self as u32),
            Self::EINTR => write!(f, "ERROR[{}] Interrupted system call", *self as u32),
            Self::EIO => write!(f, "ERROR[{}] Input/output  error", *self as u32),
            Self::ENXIO => write!(f, "ERROR[{}] No such device or address", *self as u32),
            Self::E2BIG => write!(f, "ERROR[{}] Argument list too long", *self as u32),
            Self::ENOEXEC => write!(f, "ERROR[{}] Exec format error", *self as u32),
            Self::EBADF => write!(f, "ERROR[{}] Bad file descriptor", *self as u32),
            Self::ECHILD => write!(f, "ERROR[{}] No child processes", *self as u32),
            Self::EDEADLK => write!(f, "ERROR[{}] Resource deadlock avoided", *self as u32),
            Self::ENOMEM => write!(f, "ERROR[{}] Cannot allocate memory", *self as u32),
            Self::EACCES => write!(f, "ERROR[{}] Permission denied", *self as u32),
            Self::EFAULT => write!(f, "ERROR[{}] Bad address", *self as u32),
            Self::ENOTBLK => write!(f, "ERROR[{}] Block device required", *self as u32),
            Self::EBUSY => write!(f, "ERROR[{}] Device or resource busy", *self as u32),
            Self::EEXIST => write!(f, "ERROR[{}] File exists", *self as u32),
            Self::EXDEV => write!(f, "ERROR[{}] Invalid cross-device link", *self as u32),
            Self::ENODEV => write!(f, "ERROR[{}] No such device", *self as u32),
            Self::ENOTDIR => write!(f, "ERROR[{}] Not a directory", *self as u32),
            Self::EISDIR => write!(f, "ERROR[{}] Is a directory", *self as u32),
            Self::EINVAL => write!(f, "ERROR[{}] Invalid argument", *self as u32),
            Self::EMFILE => write!(f, "ERROR[{}] Too many open files", *self as u32),
            Self::ENFILE => write!(f, "ERROR[{}] File table overflow", *self as u32),
            Self::ENOTTY => write!(f, "ERROR[{}] Inappropriate ioctl for device", *self as u32),
            Self::ETXTBSY => write!(f, "ERROR[{}] Text file busy", *self as u32),
            Self::EFBIG => write!(f, "ERROR[{}] File too large", *self as u32),
            Self::ENOSPC => write!(f, "ERROR[{}] No space left on device", *self as u32),
            Self::ESPIPE => write!(f, "ERROR[{}] Illegal seek", *self as u32),
            Self::EROFS => write!(f, "ERROR[{}] Read-only file system", *self as u32),
            Self::EMLINK => write!(f, "ERROR[{}] Too many links", *self as u32),
            Self::EPIPE => write!(f, "ERROR[{}] Broken pipe", *self as u32),
            Self::EDOM => write!(
                f,
                "ERROR[{}] Numerical argument out of domain",
                *self as u32
            ),
            Self::ERANGE => write!(f, "ERROR[{}] Numerical result out of range", *self as u32),
            Self::EAGAIN => write!(
                f,
                "ERROR[{}] Resource temporarily unavailable",
                *self as u32
            ),
            Self::EWOULDBLOCK => write!(f, "ERROR[{}] Operation would block", *self as u32),
            Self::EINPROGRESS => write!(f, "ERROR[{}] Operation now in progress", *self as u32),
            Self::EALREADY => write!(f, "ERROR[{}] Operation already in progress", *self as u32),
            Self::ENOTSOCK => write!(f, "ERROR[{}] Socket operation on non-socket", *self as u32),
            Self::EMSGSIZE => write!(f, "ERROR[{}] Message too long", *self as u32),
            Self::EPROTOTYPE => write!(f, "ERROR[{}] Protocol wrong type for socket", *self as u32),
            Self::ENOPROTOOPT => write!(f, "ERROR[{}] Protocol not available", *self as u32),
            Self::EPROTONOSUPPORT => write!(f, "ERROR[{}] Protocol not supported", *self as u32),
            Self::ESOCKTNOSUPPORT => write!(f, "ERROR[{}] Socket type not supported", *self as u32),
            Self::EOPNOTSUPP => write!(f, "ERROR[{}] Operation not supported", *self as u32),
            Self::EPFNOSUPPORT => {
                write!(f, "ERROR[{}] Protocol family not supported", *self as u32)
            }
            Self::EAFNOSUPPORT => write!(
                f,
                "ERROR[{}] Address family not supported by protocol",
                *self as u32
            ),
            Self::EADDRINUSE => write!(f, "ERROR[{}] Address already in use", *self as u32),
            Self::EADDRNOTAVAIL => {
                write!(f, "ERROR[{}] Cannot assign requested address", *self as u32)
            }
            Self::ENETDOWN => write!(f, "ERROR[{}] Network is down", *self as u32),
            Self::ENETUNREACH => write!(f, "ERROR[{}] Network is unreachable", *self as u32),
            Self::ENETRESET => write!(
                f,
                "ERROR[{}] Network dropped connection on reset",
                *self as u32
            ),
            Self::ECONNABORTED => write!(
                f,
                "ERROR[{}] Software caused connection abort",
                *self as u32
            ),
            Self::ECONNRESET => write!(f, "ERROR[{}] Connection reset by peer", *self as u32),
            Self::ENOBUFS => write!(f, "ERROR[{}] No buffer space available", *self as u32),
            Self::EISCONN => write!(
                f,
                "ERROR[{}] Transport endpoint is already connected",
                *self as u32
            ),
            Self::ENOTCONN => write!(
                f,
                "ERROR[{}] Transport endpoint is not connected",
                *self as u32
            ),
            Self::EDESTADDRREQ => write!(f, "ERROR[{}] Destination address required", *self as u32),
            Self::ESHUTDOWN => write!(
                f,
                "ERROR[{}] Cannot  send  after  transport  endpoint  shutdown",
                *self as u32
            ),
            Self::ETOOMANYREFS => write!(
                f,
                "ERROR[{}] Too many references: cannot splice",
                *self as u32
            ),
            Self::ETIMEDOUT => write!(f, "ERROR[{}] Connection timed out", *self as u32),
            Self::ECONNREFUSED => write!(f, "ERROR[{}] Connection refused", *self as u32),
            Self::ELOOP => write!(
                f,
                "ERROR[{}] Too many levels of symbolic links",
                *self as u32
            ),
            Self::ENAMETOOLONG => write!(f, "ERROR[{}] File name too long", *self as u32),
            Self::EHOSTDOWN => write!(f, "ERROR[{}] Host is down", *self as u32),
            Self::EHOSTUNREACH => write!(f, "ERROR[{}] No route  to host", *self as u32),
            Self::ENOTEMPTY => write!(f, "ERROR[{}] Directory not empty", *self as u32),
            Self::EPROCLIM => write!(f, "ERROR[{}] Too many processes", *self as u32),
            Self::EUSERS => write!(f, "ERROR[{}] Too many users", *self as u32),
            Self::EDQUOT => write!(f, "ERROR[{}] Disk quota exceeded", *self as u32),
            Self::ESTALE => write!(f, "ERROR[{}] Stale file handle", *self as u32),
            Self::EREMOTE => write!(f, "ERROR[{}] Object is remote", *self as u32),
            Self::EBADRPC => write!(f, "ERROR[{}] RPC struct is bad", *self as u32),
            Self::ERPCMISMATCH => write!(f, "ERROR[{}] RPC version wrong", *self as u32),
            Self::EPROGUNAVAIL => write!(f, "ERROR[{}] RPC program not available", *self as u32),
            Self::EPROGMISMATCH => write!(f, "ERROR[{}] RPC program version wrong", *self as u32),
            Self::EPROCUNAVAIL => {
                write!(f, "ERROR[{}] RPC bad procedure for program", *self as u32)
            }
            Self::ENOLCK => write!(f, "ERROR[{}] No locks available", *self as u32),
            Self::EFTYPE => write!(
                f,
                "ERROR[{}] Inappropriate file type or format",
                *self as u32
            ),
            Self::EAUTH => write!(f, "ERROR[{}] Authentication error", *self as u32),
            Self::ENEEDAUTH => write!(f, "ERROR[{}] Need authenticator", *self as u32),
            Self::ENOSYS => write!(f, "ERROR[{}] Function not implemented", *self as u32),
            Self::ELIBEXEC => write!(
                f,
                "ERROR[{}] Cannot exec a shared library directly",
                *self as u32
            ),
            Self::ENOTSUP => write!(f, "ERROR[{}] // Not supported", *self as u32),
            Self::EILSEQ => write!(
                f,
                "ERROR[{}] Invalid or incomplete multibyte or wide character",
                *self as u32
            ),
            Self::EBACKGROUND => write!(
                f,
                "ERROR[{}] Inappropriate operation for background process",
                *self as u32
            ),
            Self::EDIED => write!(f, "ERROR[{}] Translator died", *self as u32),
            Self::ED => write!(
                f,
                "ERROR[{}] The experienced user will know what is wrong",
                *self as u32
            ),
            Self::EGREGIOUS => write!(
                f,
                "ERROR[{}] You  really  blew  it  this  time",
                *self as u32
            ),
            Self::EIEIO => write!(f, "ERROR[{}] Computer bought the farm", *self as u32),
            Self::EGRATUITOUS => write!(f, "ERROR[{}] Gratuitous error", *self as u32),
            Self::EBADMSG => write!(f, "ERROR[{}] Bad message", *self as u32),
            Self::EIDRM => write!(f, "ERROR[{}] Identifier  removed", *self as u32),
            Self::EMULTIHOP => write!(f, "ERROR[{}] Multihop  attempted", *self as u32),
            Self::ENODATA => write!(f, "ERROR[{}] No  data  available", *self as u32),
            Self::ENOLINK => write!(f, "ERROR[{}] Link has been severed", *self as u32),
            Self::ENOMSG => write!(f, "ERROR[{}] No message of desired type", *self as u32),
            Self::ENOSR => write!(f, "ERROR[{}] Out  of streams resources", *self as u32),
            Self::ENOSTR => write!(f, "ERROR[{}] Device not a stream", *self as u32),
            Self::EOVERFLOW => write!(
                f,
                "ERROR[{}] Value too large for defined data type",
                *self as u32
            ),
            Self::EPROTO => write!(f, "ERROR[{}] Protocol error", *self as u32),
            Self::ETIME => write!(f, "ERROR[{}] Timer expired", *self as u32),
            Self::ECANCELED => write!(f, "ERROR[{}] Operation canceled", *self as u32),
            Self::EOWNERDEAD => write!(f, "ERROR[{}] ///Owner  died", *self as u32),
            Self::ENOTRECOVERABLE => write!(f, "ERROR[{}] State not recoverable", *self as u32),
            Self::ERESTART => write!(
                f,
                "ERROR[{}] Interrupted system call should be restarted",
                *self as u32
            ),
            Self::ECHRNG => write!(f, "ERROR[{}] Channel number out of range", *self as u32),
            Self::EL2NSYNC => write!(f, "ERROR[{}] Level 2 not synchronized", *self as u32),
            Self::EL3HLT => write!(f, "ERROR[{}] Level 3 halted", *self as u32),
            Self::EL3RST => write!(f, "ERROR[{}] Level 3 reset", *self as u32),
            Self::ELNRNG => write!(f, "ERROR[{}] Link number out of range", *self as u32),
            Self::EUNATCH => write!(f, "ERROR[{}] Protocol driver not attached", *self as u32),
            Self::ENOCSI => write!(f, "ERROR[{}] No CSI structure available", *self as u32),
            Self::EL2HLT => write!(f, "ERROR[{}] Level 2 halted", *self as u32),
            Self::EBADE => write!(f, "ERROR[{}] Invalid exchange", *self as u32),
            Self::EBADR => write!(f, "ERROR[{}] Invalid request descriptor", *self as u32),
            Self::EXFULL => write!(f, "ERROR[{}] Exchange full", *self as u32),
            Self::ENOANO => write!(f, "ERROR[{}] No anode", *self as u32),
            Self::EBADRQC => write!(f, "ERROR[{}] Invalid request code", *self as u32),
            Self::EBADSLT => write!(f, "ERROR[{}] Invalid slot", *self as u32),
            Self::EDEADLOCK => write!(f, "ERROR[{}] File locking deadlock error", *self as u32),
            Self::EBFONT => write!(f, "ERROR[{}] Bad font file format", *self as u32),
            Self::ENONET => write!(f, "ERROR[{}] Machine is not on the network", *self as u32),
            Self::ENOPKG => write!(f, "ERROR[{}] Package not installed", *self as u32),
            Self::EADV => write!(f, "ERROR[{}] Advertise error", *self as u32),
            Self::ESRMNT => write!(f, "ERROR[{}] Srmount error", *self as u32),
            Self::ECOMM => write!(f, "ERROR[{}] Communication error on send", *self as u32),
            Self::EDOTDOT => write!(f, "ERROR[{}] RFS specific error", *self as u32),
            Self::ENOTUNIQ => write!(f, "ERROR[{}] Name not unique on network", *self as u32),
            Self::EBADFD => write!(f, "ERROR[{}] File descriptor in bad state", *self as u32),
            Self::EREMCHG => write!(f, "ERROR[{}] Remote address changed", *self as u32),
            Self::ELIBACC => write!(
                f,
                "ERROR[{}] Can not access a needed shared library",
                *self as u32
            ),
            Self::ELIBBAD => write!(
                f,
                "ERROR[{}] Accessing a corrupted shared library",
                *self as u32
            ),
            Self::ELIBSCN => write!(
                f,
                "ERROR[{}] .lib  section  in  a.out  corrupted",
                *self as u32
            ),
            Self::ELIBMAX => write!(
                f,
                "ERROR[{}] Attempting to link in too many shared libraries",
                *self as u32
            ),
            Self::ESTRPIPE => write!(f, "ERROR[{}] Streams pipe error", *self as u32),
            Self::EUCLEAN => write!(f, "ERROR[{}] Structure needs cleaning", *self as u32),
            Self::ENOTNAM => write!(f, "ERROR[{}] Not a XENIX named type file", *self as u32),
            Self::ENAVAIL => write!(f, "ERROR[{}] No XENIX semaphores available", *self as u32),
            Self::EISNAM => write!(f, "ERROR[{}] Is a named type file", *self as u32),
            Self::EREMOTEIO => write!(f, "ERROR[{}] Remote I/O error", *self as u32),
            Self::ENOMEDIUM => write!(f, "ERROR[{}] No medium found", *self as u32),
            Self::EMEDIUMTYPE => write!(f, "ERROR[{}] Wrong medium type", *self as u32),
            Self::ENOKEY => write!(f, "ERROR[{}] Required key not available", *self as u32),
            Self::EKEYEXPIRED => write!(f, "ERROR[{}] Key has expired", *self as u32),
            Self::EKEYREVOKED => write!(f, "ERROR[{}] Key has been revoked", *self as u32),
            Self::EKEYREJECTED => write!(f, "ERROR[{}] Key was rejected by service", *self as u32),
            Self::ERFKILL => write!(
                f,
                "ERROR[{}] Operation not possible due to RF-kill",
                *self as u32
            ),
            Self::EHWPOISON => write!(f, "ERROR[{}] Memory page has hardware error", *self as u32),
        }
    }
}
