// SPDX-License-Identifier: GPL-3.0-or-later
use crate::status::Status;
use crate::{db::Database, message::Messages};
use notmuch_sys::{
    notmuch_exclude_t, notmuch_exclude_t_NOTMUCH_EXCLUDE_ALL,
    notmuch_exclude_t_NOTMUCH_EXCLUDE_FALSE, notmuch_exclude_t_NOTMUCH_EXCLUDE_FLAG,
    notmuch_exclude_t_NOTMUCH_EXCLUDE_TRUE, notmuch_query_add_tag_exclude, notmuch_query_create,
    notmuch_query_destroy, notmuch_query_search_messages, notmuch_query_set_omit_excluded,
    notmuch_query_t,
};
use std::{ffi::CString, marker::PhantomData};

pub struct Query<'a> {
    query: *mut notmuch_query_t,
    _covariant: PhantomData<&'a ()>,
}

impl<'a> Query<'_> {
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

    /// Create a new query for a `database`
    pub fn create(db: &'a Database, query: &str) -> Query<'a> {
        let query = CString::new(query).expect("Unable to convert query to C string");

        unsafe {
            let query_ptr = notmuch_query_create(db.into(), query.as_ptr());
            Query::from(query_ptr).unwrap()
        }
    }

    pub fn from(ptr: *mut notmuch_query_t) -> Option<Self> {
        // TODO(austin-ray): See if there's a Rust trait similar to From, but returns an Option.
        if ptr.is_null() {
            return None;
        }

        Some(Query {
            query: ptr,
            _covariant: PhantomData,
        })
    }

    /// Execute a query for messages.
    pub fn search_messages(&self) -> Result<Messages, Status> {
        let mut msgs = std::ptr::null_mut();

        let st = unsafe { notmuch_query_search_messages(self.query, &mut msgs) }.into();

        match st {
            Status::Success => Ok(Messages::from(msgs)),
            _ => Err(st),
        }
    }

    /// Specify whether to omit excluded results or simply flag them.
    pub fn set_omit_excluded(&self, exclude: Exclude) {
        unsafe { notmuch_query_set_omit_excluded(self.query, exclude.into()) }
    }
}

impl Drop for Query<'_> {
    fn drop(&mut self) {
        unsafe {
            notmuch_query_destroy(self.query);
        }
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
