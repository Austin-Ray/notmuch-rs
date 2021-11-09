// SPDX-License-Identifier: GPL-3.0-or-later
use crate::status::Status;
use notmuch_sys::{
    notmuch_exclude_t, notmuch_exclude_t_NOTMUCH_EXCLUDE_ALL,
    notmuch_exclude_t_NOTMUCH_EXCLUDE_FALSE, notmuch_exclude_t_NOTMUCH_EXCLUDE_FLAG,
    notmuch_exclude_t_NOTMUCH_EXCLUDE_TRUE, notmuch_query_add_tag_exclude,
    notmuch_query_set_omit_excluded, notmuch_query_t,
};
use std::ffi::CString;

pub struct Query {
    query: *mut notmuch_query_t,
}

impl Query {
    /// Add a tag that will be excluded from the query results by default.
    ///
    /// This exclusion will be ignored if this tag appears explicitly in the query.
    pub fn add_tag_exclude(&self, tag: &str) -> Result<(), Status> {
        let tag = CString::new(tag).unwrap();

        let st = unsafe { notmuch_query_add_tag_exclude(self.query, tag.as_ptr()) }.into();

        match st {
            Status::Success => Ok(()),
            _ => Err(st),
        }
    }

    pub fn from(ptr: *mut notmuch_query_t) -> Option<Self> {
        // TODO(austin-ray): See if there's a Rust trait similar to From, but returns an Option.
        if ptr.is_null() {
            return None;
        }

        Some(Query { query: ptr })
    }

    /// Specify whether to omit excluded results or simply flag them.
    pub fn set_omit_excluded(&self, exclude: Exclude) {
        unsafe { notmuch_query_set_omit_excluded(self.query, exclude.into()) }
    }
}

pub enum Exclude {
    True,
    False,
    All,
    Flags,
}

impl Into<notmuch_exclude_t> for Exclude {
    fn into(self) -> notmuch_exclude_t {
        match self {
            Self::All => notmuch_exclude_t_NOTMUCH_EXCLUDE_ALL,
            Self::False => notmuch_exclude_t_NOTMUCH_EXCLUDE_FALSE,
            Self::Flags => notmuch_exclude_t_NOTMUCH_EXCLUDE_FLAG,
            Self::True => notmuch_exclude_t_NOTMUCH_EXCLUDE_TRUE,
        }
    }
}
