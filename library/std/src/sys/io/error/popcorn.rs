use crate::{fmt, io};

pub fn errno() -> i32 {
    0
}

pub fn is_interrupted(_errno: i32) -> bool {
    false
}

pub fn decode_error_kind(errno: i32) -> io::ErrorKind {
    decode_error_code(errno).0
}

pub fn format_error(errno: i32, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.write_str(decode_error_code(errno).1)
}

fn decode_error_code(code: i32) -> (io::ErrorKind, &'static str) {
    use io::ErrorKind::*;
    match code {
        0 => /* Error::InvalidPointer */ (InvalidInput, "pointer passed was invalid"),
        1 => /* Error::InvalidUtf8 */ (InvalidData, "string data was malformed UTF-8"),
        2 => /* Error::UnsupportedProtocol */ (Unsupported, "server or handle does not support requested protocol or method"),
        3 => /* Error::UnknownProtocol */ (Unsupported, "unknown protocol or method"),
        4 => /* Error::EndpointNotFound */ (NotFound, "could not find requested endpoint"),
        5 => /* Error::NameInUse */ (AlreadyExists, "requested endpoint is already in use"),
        6 => /* Error::InvalidHandle */ (InvalidInput, "invalid handle"),
        7 => /* Error::Overflow */ (Other, "failed to allocate numeric identifier due to overflow"),
        8 => /* Error::InvalidName */ (InvalidData, "requested endpoint path contained invalid characters"),
        9 => /* Error::DeadServer */ (Other, "server has disconnected"),
        10 => /* Error::InvalidReturn */ (InvalidInput, "invalid return type for method (likely non-numeric return for memory out)"),
        11 => /* Error::Invalid */ (Other, "invalid"),
        12 => /* Error::AllocationFailure */ (OutOfMemory, "failed to allocate memory"),
		13 => /* Error::InvalidArg */ (InvalidInput, "invalid numeric argument passed"),
		14 => /* Error::AsyncUnsupported */ (Unsupported, "syscall only supported synchronously"),
        15 => /* Error::FutureCompat */ (InvalidInput, "argument value reserved for future use"),
		16 => /* Error::EoF */ (UnexpectedEof, "unexpected end-of-file"),
		17 => /* Error::ProtocolOverlap */ (InvalidData, "combined handles have overlap in supported protocols"),
        _ => /* Error::Invalid */ (Other, "unknown"),
    }
}
