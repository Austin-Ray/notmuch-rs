use crate::{status::Status, tags::Tags};
use notmuch_sys::{
    notmuch_message_add_tag, notmuch_message_freeze, notmuch_message_get_message_id,
    notmuch_message_get_tags, notmuch_message_remove_tag, notmuch_message_t, notmuch_message_thaw,
    notmuch_messages_destroy, notmuch_messages_get, notmuch_messages_move_to_next,
    notmuch_messages_t, notmuch_messages_valid,
};
use std::{
    ffi::{CStr, CString},
    marker::PhantomData,
};

pub struct Message<'a> {
    msg: *mut notmuch_message_t,
    _covariant: PhantomData<&'a ()>,
}

impl<'a> Message<'a> {
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

    /// Returns a `Tags` iterator for all tags associated with this message.
    pub fn get_tags(&self) -> Option<Tags> {
        let tags_ptr = unsafe { notmuch_message_get_tags(self.msg) };
        Tags::from(tags_ptr)
    }

    /// Get message id from notmuch.
    pub fn message_id(&self) -> String {
        let raw_str = unsafe { notmuch_message_get_message_id(self.msg) };

        unsafe { CStr::from_ptr(raw_str) }
            .to_str()
            .unwrap()
            .to_string()
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

impl From<*mut notmuch_message_t> for Message<'_> {
    fn from(ptr: *mut notmuch_message_t) -> Self {
        Message {
            msg: ptr,
            _covariant: PhantomData,
        }
    }
}

/// Container for all messages corresponding to a `Query`.
///
/// Since `libnotmuch` lazy loads messages, this struct is a container for `MessagesIter`. It must
/// be converted into an `Iterator` before use. It's `.into_iter()` may be called multiple times.
///
/// # Example
///
/// ```ignore
/// let msgs = some_query.search_messages()?;
///
/// for msg in msgs.into_inner() {
///     println!("message id: {}", msg.message_id());
/// }
/// ```
pub struct Messages<'a> {
    ptr: *mut notmuch_messages_t,
    _covariant: PhantomData<&'a ()>,
}

impl Drop for Messages<'_> {
    fn drop(&mut self) {
        unsafe {
            notmuch_messages_destroy(self.ptr);
        }
    }
}

impl From<*mut notmuch_messages_t> for Messages<'_> {
    fn from(ptr: *mut notmuch_messages_t) -> Self {
        Messages {
            ptr,
            _covariant: PhantomData,
        }
    }
}

impl<'a> IntoIterator for &Messages<'a> {
    type Item = Message<'a>;
    type IntoIter = MessagesIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        MessagesIter::from_ptr(self.ptr)
    }
}

// NOTE: This intentionally does not implement `Drop`. The pointer is owned by a `Messages` struct.
pub struct MessagesIter<'a> {
    ptr: *mut notmuch_messages_t,
    _covariant: PhantomData<&'a ()>,
}

impl MessagesIter<'_> {
    fn from_ptr(ptr: *mut notmuch_messages_t) -> Self {
        MessagesIter {
            ptr,
            _covariant: PhantomData,
        }
    }
}

impl<'a> Iterator for MessagesIter<'a> {
    type Item = Message<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let is_valid = unsafe { notmuch_messages_valid(self.ptr) != 0 };

        if !is_valid {
            return None;
        }

        let cur_ptr = unsafe { notmuch_messages_get(self.ptr) };
        let cur = Message::from(cur_ptr);

        // Advance the pointer
        unsafe { notmuch_messages_move_to_next(self.ptr) }

        Some(cur)
    }
}
