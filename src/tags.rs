use notmuch_sys::{
    notmuch_tags_get, notmuch_tags_move_to_next, notmuch_tags_t, notmuch_tags_valid,
};
use std::ffi::CStr;

pub struct Tags {
    tags: *mut notmuch_tags_t,
}

impl Tags {
    /// Creates a `Tags` iterator from a given raw `ptr`
    ///
    /// If `ptr` is NULL, this returns None.
    pub fn from(ptr: *mut notmuch_tags_t) -> Option<Self> {
        if ptr.is_null() {
            return None;
        }

        Some(Tags { tags: ptr })
    }
}

impl Iterator for Tags {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if unsafe { notmuch_tags_valid(self.tags) } == 0 {
            return None;
        }

        let tag = unsafe {
            let raw_str = notmuch_tags_get(self.tags);
            CStr::from_ptr(raw_str)
        }
        .to_str()
        .ok()?
        .to_string()
        .to_string();

        unsafe { notmuch_tags_move_to_next(self.tags) }

        Some(tag)
    }
}
