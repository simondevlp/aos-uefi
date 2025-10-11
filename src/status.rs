use core::fmt::Display;

#[repr(transparent)]
#[derive(PartialEq, Eq, Clone, Copy)]
pub struct Status(pub usize);

impl Display for Status {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                Self::SUCCESS => "SUCCESS",
                Self::LOAD_ERROR => "LOAD_ERROR",
                Self::INVALID_PARAMETER => "INVALID_PARAMETER",
                Self::UNSUPPORTED => "UNSUPPORTED",
                Self::BAD_BUFFER_SIZE => "BAD_BUFFER_SIZE",
                Self::BUFFER_TOO_SMALL => "BUFFER_TOO_SMALL",
                Self::NOT_READY => "NOT_READY",
                Self::DEVICE_ERROR => "DEVICE_ERROR",
                Self::WRITE_PROTECTED => "WRITE_PROTECTED",
                Self::OUT_OF_RESOURCES => "OUT_OF_RESOURCES",
                Self::VOLUME_CORRUPTED => "VOLUME_CORRUPTED",
                Self::VOLUME_FULL => "VOLUME_FULL",
                Self::NO_MEDIA => "NO_MEDIA",
                Self::MEDIA_CHANGED => "MEDIA_CHANGED",
                Self::NOT_FOUND => "NOT_FOUND",
                Self::ACCESS_DENIED => "ACCESS_DENIED",
                Self::NO_RESPONSE => "NO_RESPONSE",
                Self::NO_MAPPING => "NO_MAPPING",
                Self::TIMEOUT => "TIMEOUT",
                Self::NOT_STARTED => "NOT_STARTED",
                Self::ALREADY_STARTED => "ALREADY_STARTED",
                Self::ABORTED => "ABORTED",
                Self::ICMP_ERROR => "ICMP_ERROR",
                Self::TFTP_ERROR => "TFTP_ERROR",
                Self::PROTOCOL_ERROR => "PROTOCOL_ERROR",
                Self::INCOMPATIBLE_VERSION => "INCOMPATIBLE_VERSION",
                Self::SECURITY_VIOLATION => "SECURITY_VIOLATION",
                Self::CRC_ERROR => "CRC_ERROR",
                Self::END_OF_MEDIA => "END_OF_MEDIA",
                Self::END_OF_FILE => "END_OF_FILE",
                Self::INVALID_LANGUAGE => "INVALID_LANGUAGE",
                Self::COMPROMISED_DATA => "COMPROMISED_DATA",
                Self::IP_ADDRESS_CONFLICT => "IP_ADDRESS_CONFLICT",
                Self::HTTP_ERROR => "HTTP_ERROR",
                Self::WARN_UNKNOWN_GLYPH => "WARN_UNKNOWN_GLYPH",
                Self::WARN_DELETE_FAILURE => "WARN_DELETE_FAILURE",
                Self::WARN_WRITE_FAILURE => "WARN_WRITE_FAILURE",
                Self::WARN_BUFFER_TOO_SMALL => "WARN_BUFFER_TOO_SMALL",
                Self::WARN_STALE_DATA => "WARN_STALE_DATA",
                Self::WARN_FILE_SYSTEM => "WARN_FILE_SYSTEM",
                Self::WARN_RESET_REQUIRED => "WARN_RESET_REQUIRED",
                _ => "UNKNOWN_STATUS",
            }
        )
    }
}

impl Status {
    const HIGH_BIT_SET: usize = 1 << (usize::BITS - 1);
    const fn error(v: usize) -> Self {
        Self(Self::HIGH_BIT_SET | v)
    }

    pub const fn is_error(&self) -> bool {
        self.0 & Self::HIGH_BIT_SET == Self::HIGH_BIT_SET
    }

    pub const SUCCESS: Self = Self(0);

    // error
    pub const LOAD_ERROR: Self = Self::error(1);
    pub const INVALID_PARAMETER: Self = Self::error(2);
    pub const UNSUPPORTED: Self = Self::error(3);
    pub const BAD_BUFFER_SIZE: Self = Self::error(4);
    pub const BUFFER_TOO_SMALL: Self = Self::error(5);
    pub const NOT_READY: Self = Self::error(6);
    pub const DEVICE_ERROR: Self = Self::error(7);
    pub const WRITE_PROTECTED: Self = Self::error(8);
    pub const OUT_OF_RESOURCES: Self = Self::error(9);
    pub const VOLUME_CORRUPTED: Self = Self::error(10);
    pub const VOLUME_FULL: Self = Self::error(11);
    pub const NO_MEDIA: Self = Self::error(12);
    pub const MEDIA_CHANGED: Self = Self::error(13);
    pub const NOT_FOUND: Self = Self::error(14);
    pub const ACCESS_DENIED: Self = Self::error(15);
    pub const NO_RESPONSE: Self = Self::error(16);
    pub const NO_MAPPING: Self = Self::error(17);
    pub const TIMEOUT: Self = Self::error(18);
    pub const NOT_STARTED: Self = Self::error(19);
    pub const ALREADY_STARTED: Self = Self::error(20);
    pub const ABORTED: Self = Self::error(21);
    pub const ICMP_ERROR: Self = Self::error(22);
    pub const TFTP_ERROR: Self = Self::error(23);
    pub const PROTOCOL_ERROR: Self = Self::error(24);
    pub const INCOMPATIBLE_VERSION: Self = Self::error(25);
    pub const SECURITY_VIOLATION: Self = Self::error(26);
    pub const CRC_ERROR: Self = Self::error(27);
    pub const END_OF_MEDIA: Self = Self::error(28);
    pub const END_OF_FILE: Self = Self::error(31);
    pub const INVALID_LANGUAGE: Self = Self::error(32);
    pub const COMPROMISED_DATA: Self = Self::error(33);
    pub const IP_ADDRESS_CONFLICT: Self = Self::error(34);
    pub const HTTP_ERROR: Self = Self::error(35);

    // warning
    pub const WARN_UNKNOWN_GLYPH: Self = Self(1);
    pub const WARN_DELETE_FAILURE: Self = Self(2);
    pub const WARN_WRITE_FAILURE: Self = Self(3);
    pub const WARN_BUFFER_TOO_SMALL: Self = Self(4);
    pub const WARN_STALE_DATA: Self = Self(5);
    pub const WARN_FILE_SYSTEM: Self = Self(6);
    pub const WARN_RESET_REQUIRED: Self = Self(7);
}
