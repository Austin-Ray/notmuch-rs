use crate::{status::Status, tags::Tags};
use notmuch_sys::{
    notmuch_message_add_tag, notmuch_message_freeze, notmuch_message_get_tags,
    notmuch_message_remove_tag, notmuch_message_t, notmuch_message_thaw,
};
use std::ffi::CString;

pub struct Message {
    msg: *mut notmuch_message_t,
}

impl Message {
    /// Add a given `tag` to the message.
    pub fn add_tag(&self, tag: &str) -> Result<(), Status> {
        let tag = CString::new(tag).unwrap();

        let st = unsafe { notmuch_message_add_tag(self.msg, tag.as_ptr()) }.into();

        match st {
            Status::Success => Ok(()),
            _ => Err(st),
        }
    }

    /// Freeze the current state of `message` within the database.
    pub fn freeze(&self) -> Result<(), Status> {
        let st = unsafe { notmuch_message_freeze(self.msg) }.into();

        match st {
            Status::Success => Ok(()),
            _ => Err(st),
        }
    }

    /// Creates a Rust struct wrapping the raw notmuch pointer if possible.
    pub fn from(ptr: *mut notmuch_message_t) -> Option<Self> {
        if ptr.is_null() {
            return None;
        }

        Some(Message { msg: ptr })
    }

    /// Returns a `Tags` iterator for all tags associated with this message.
    pub fn get_tags(&self) -> Option<Tags> {
        let tags_ptr = unsafe { notmuch_message_get_tags(self.msg) };
        Tags::from(tags_ptr)
    }

    /// Remove a given `tag` from the message.
    pub fn remove_tag(&self, tag: &str) -> Result<(), Status> {
        let tag = CString::new(tag).unwrap();

        let st = unsafe { notmuch_message_remove_tag(self.msg, tag.as_ptr()) }.into();

        match st {
            Status::Success => Ok(()),
            _ => Err(st),
        }
    }

    /// Thaw the current `message`, synchronizing any changes that may have occurred while
    /// `message` was frozen into the notmuch database.
    pub fn thaw(&self) -> Result<(), Status> {
        let st = unsafe { notmuch_message_thaw(self.msg) }.into();

        match st {
            Status::Success => Ok(()),
            _ => Err(st),
        }
    }
}
