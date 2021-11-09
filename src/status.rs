use std::{error::Error, ffi::CStr, fmt::Display};

use notmuch_sys::{notmuch_status_t, notmuch_status_to_string};

#[derive(Debug, PartialEq)]
pub enum Status {
    /// No error occurred.
    Success,

    /// Out of memory.
    OutOfMemory,

    /// An attempt was made to write to a database opened in read-only mode.
    ReadOnlyDatabase,

    /// A Xapian exception occurred.
    XapianException,

    /// An error occurred trying to read or write to a file (this could be file not found,
    /// permission denied, etc.)
    FileError,

    /// A file was presented that doesn't appear to be an email message.
    FileNotEmail,

    /// A file contains a message ID that is identical to a message already in the database.
    DuplicateMessageId,

    /// The user erroneously passed a NULL pointer to a notmuch function.
    NullPointer,

    /// A tag value is too long (exceeds NOTMUCH_TAG_MAX).
    TagTooLong,

    /// The notmuch_message_thaw function has been called more times than notmuch_message_freeze.
    UnbalancedFreezeThaw,

    /// notmuch_database_end_atomic has been called more times than notmuch_database_begin_atomic.
    UnbalancedAtomic,

    /// The operation is not supported.
    UnsupportedOperation,

    /// The operation requires a database upgrade.
    UpgradeRequired,

    /// There is a problem with the proposed path, e.g. a relative path passed to a function
    /// expecting an absolute path.
    PathError,

    /// The requested operation was ignored. Depending on the function, this may not be an actual
    /// error.
    Ignored,

    /// One of the arguments violates the preconditions for the function, in a way not covered by a
    /// more specific argument.
    IllegalArgument,

    /// A MIME object claimed to have cryptographic protection which notmuch tried to handle, but
    /// the protocol was not specified in an intelligible way.
    MalformedCryptoProtocol,

    /// Notmuch attempted to do crypto processing, but could not initialize the engine needed to do
    /// so.
    FailedCryptoContextCreation,

    /// A MIME object claimed to have cryptographic protection, and notmuch attempted to process
    /// it, but the specific protocol was something that notmuch doesn't know how to handle.
    UnknownCryptoProtocol,

    /// Unable to load a config file
    NoConfig,

    /// Unable to load a database
    NoDatabase,

    /// Database exists, so not (re)-created
    DatabaseExists,

    /// Not an actual status value. Just a way to find out how many valid status values there are.
    LastStatus,
}

impl Error for Status {}

impl Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = unsafe {
            let nm_msg = notmuch_status_to_string(self.into());
            CStr::from_ptr(nm_msg).to_str().ok()
        }
        .unwrap();

        write!(f, "{}", msg)
    }
}

impl From<notmuch_status_t> for Status {
    fn from(st: notmuch_status_t) -> Self {
        match st {
            notmuch_sys::_notmuch_status_NOTMUCH_STATUS_SUCCESS => Self::Success,
            notmuch_sys::_notmuch_status_NOTMUCH_STATUS_OUT_OF_MEMORY => Self::OutOfMemory,
            notmuch_sys::_notmuch_status_NOTMUCH_STATUS_READ_ONLY_DATABASE => {
                Self::ReadOnlyDatabase
            }
            notmuch_sys::_notmuch_status_NOTMUCH_STATUS_XAPIAN_EXCEPTION => Self::XapianException,
            notmuch_sys::_notmuch_status_NOTMUCH_STATUS_FILE_ERROR => Self::FileError,
            notmuch_sys::_notmuch_status_NOTMUCH_STATUS_FILE_NOT_EMAIL => Self::FileNotEmail,
            notmuch_sys::_notmuch_status_NOTMUCH_STATUS_DUPLICATE_MESSAGE_ID => {
                Self::DuplicateMessageId
            }
            notmuch_sys::_notmuch_status_NOTMUCH_STATUS_NULL_POINTER => Self::NullPointer,
            notmuch_sys::_notmuch_status_NOTMUCH_STATUS_TAG_TOO_LONG => Self::TagTooLong,
            notmuch_sys::_notmuch_status_NOTMUCH_STATUS_UNBALANCED_FREEZE_THAW => {
                Self::UnbalancedFreezeThaw
            }
            notmuch_sys::_notmuch_status_NOTMUCH_STATUS_UNBALANCED_ATOMIC => Self::UnbalancedAtomic,
            notmuch_sys::_notmuch_status_NOTMUCH_STATUS_UNSUPPORTED_OPERATION => {
                Self::UnsupportedOperation
            }
            notmuch_sys::_notmuch_status_NOTMUCH_STATUS_UPGRADE_REQUIRED => Self::UpgradeRequired,
            notmuch_sys::_notmuch_status_NOTMUCH_STATUS_PATH_ERROR => Self::PathError,
            notmuch_sys::_notmuch_status_NOTMUCH_STATUS_IGNORED => Self::Ignored,
            notmuch_sys::_notmuch_status_NOTMUCH_STATUS_ILLEGAL_ARGUMENT => Self::IllegalArgument,
            notmuch_sys::_notmuch_status_NOTMUCH_STATUS_MALFORMED_CRYPTO_PROTOCOL => {
                Self::MalformedCryptoProtocol
            }
            notmuch_sys::_notmuch_status_NOTMUCH_STATUS_FAILED_CRYPTO_CONTEXT_CREATION => {
                Self::FailedCryptoContextCreation
            }
            notmuch_sys::_notmuch_status_NOTMUCH_STATUS_UNKNOWN_CRYPTO_PROTOCOL => {
                Self::UnknownCryptoProtocol
            }
            notmuch_sys::_notmuch_status_NOTMUCH_STATUS_NO_CONFIG => Self::NoConfig,
            notmuch_sys::_notmuch_status_NOTMUCH_STATUS_NO_DATABASE => Self::NoDatabase,
            notmuch_sys::_notmuch_status_NOTMUCH_STATUS_DATABASE_EXISTS => Self::DatabaseExists,
            notmuch_sys::_notmuch_status_NOTMUCH_STATUS_LAST_STATUS => Self::LastStatus,
            _ => Self::LastStatus,
        }
    }
}

impl From<Status> for notmuch_status_t {
    fn from(st: Status) -> Self {
        match st {
            Status::Success => notmuch_sys::_notmuch_status_NOTMUCH_STATUS_SUCCESS,
            Status::OutOfMemory => notmuch_sys::_notmuch_status_NOTMUCH_STATUS_OUT_OF_MEMORY,
            Status::ReadOnlyDatabase => {
                notmuch_sys::_notmuch_status_NOTMUCH_STATUS_READ_ONLY_DATABASE
            }
            Status::XapianException => notmuch_sys::_notmuch_status_NOTMUCH_STATUS_XAPIAN_EXCEPTION,
            Status::FileError => notmuch_sys::_notmuch_status_NOTMUCH_STATUS_FILE_ERROR,
            Status::FileNotEmail => notmuch_sys::_notmuch_status_NOTMUCH_STATUS_FILE_NOT_EMAIL,
            Status::DuplicateMessageId => {
                notmuch_sys::_notmuch_status_NOTMUCH_STATUS_DUPLICATE_MESSAGE_ID
            }
            Status::NullPointer => notmuch_sys::_notmuch_status_NOTMUCH_STATUS_NULL_POINTER,
            Status::TagTooLong => notmuch_sys::_notmuch_status_NOTMUCH_STATUS_TAG_TOO_LONG,
            Status::UnbalancedFreezeThaw => {
                notmuch_sys::_notmuch_status_NOTMUCH_STATUS_UNBALANCED_FREEZE_THAW
            }
            Status::UnbalancedAtomic => {
                notmuch_sys::_notmuch_status_NOTMUCH_STATUS_UNBALANCED_ATOMIC
            }
            Status::UnsupportedOperation => {
                notmuch_sys::_notmuch_status_NOTMUCH_STATUS_UNSUPPORTED_OPERATION
            }
            Status::UpgradeRequired => notmuch_sys::_notmuch_status_NOTMUCH_STATUS_UPGRADE_REQUIRED,
            Status::PathError => notmuch_sys::_notmuch_status_NOTMUCH_STATUS_PATH_ERROR,
            Status::Ignored => notmuch_sys::_notmuch_status_NOTMUCH_STATUS_IGNORED,
            Status::IllegalArgument => notmuch_sys::_notmuch_status_NOTMUCH_STATUS_ILLEGAL_ARGUMENT,
            Status::MalformedCryptoProtocol => {
                notmuch_sys::_notmuch_status_NOTMUCH_STATUS_MALFORMED_CRYPTO_PROTOCOL
            }
            Status::FailedCryptoContextCreation => {
                notmuch_sys::_notmuch_status_NOTMUCH_STATUS_FAILED_CRYPTO_CONTEXT_CREATION
            }
            Status::UnknownCryptoProtocol => {
                notmuch_sys::_notmuch_status_NOTMUCH_STATUS_UNKNOWN_CRYPTO_PROTOCOL
            }
            Status::NoConfig => notmuch_sys::_notmuch_status_NOTMUCH_STATUS_NO_CONFIG,
            Status::NoDatabase => notmuch_sys::_notmuch_status_NOTMUCH_STATUS_NO_DATABASE,
            Status::DatabaseExists => notmuch_sys::_notmuch_status_NOTMUCH_STATUS_DATABASE_EXISTS,
            Status::LastStatus => notmuch_sys::_notmuch_status_NOTMUCH_STATUS_LAST_STATUS,
        }
    }
}

impl From<&Status> for notmuch_status_t {
    fn from(st: &Status) -> Self {
        st.into()
    }
}
