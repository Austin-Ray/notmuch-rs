// SPDX-License-Identifier: GPL-3.0-or-later
use crate::status::Status;
use notmuch_sys::{
    notmuch_database_destroy, notmuch_database_mode_t,
    notmuch_database_mode_t_NOTMUCH_DATABASE_MODE_READ_ONLY,
    notmuch_database_mode_t_NOTMUCH_DATABASE_MODE_READ_WRITE, notmuch_database_open,
    notmuch_database_t,
};
use std::{ffi::CString, path::Path};

/// Should database be opened in read-only or read-write mode.
pub enum OpenMode {
    /// Open database for reading only.
    ReadOnly,
    /// Open database for reading and writing.
    ReadWrite,
}

impl From<OpenMode> for notmuch_database_mode_t {
    fn from(mode: OpenMode) -> Self {
        match mode {
            OpenMode::ReadOnly => notmuch_database_mode_t_NOTMUCH_DATABASE_MODE_READ_ONLY,
            OpenMode::ReadWrite => notmuch_database_mode_t_NOTMUCH_DATABASE_MODE_READ_WRITE,
        }
    }
}

/// Notmuch database struct.
pub struct Database {
    ptr: *mut notmuch_database_t,
}

impl Database {
    /// Open a database at the provided `path` in either read-only or read-write mode.
    ///
    /// This is equivalent to opening the database without a configuration file.
    ///
    /// Any errors reported by `libnotmuch` will be printed to `stderr`.
    pub fn open(path: &Path, mode: OpenMode) -> Result<Self, Status> {
        let path = CString::new(path.as_os_str().to_str().unwrap())
            .expect("Unable to convert path to C string");
        let mut db = std::ptr::null_mut();

        let st = unsafe { notmuch_database_open(path.as_ptr(), mode.into(), &mut db) }.into();

        match st {
            Status::Success => Ok(Database::from(db)),
            _ => Err(st),
        }
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        unsafe {
            notmuch_database_destroy(self.ptr);
        }
    }
}

impl From<&Database> for *mut notmuch_database_t {
    fn from(db: &Database) -> Self {
        db.ptr
    }
}

impl From<*mut notmuch_database_t> for Database {
    fn from(ptr: *mut notmuch_database_t) -> Self {
        Database { ptr }
    }
}
