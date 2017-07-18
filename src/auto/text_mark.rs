// This file was generated by gir (ce03df6) from gir-files (71d73f0)
// DO NOT EDIT

use TextBuffer;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct TextMark(Object<ffi::GtkTextMark>);

    match fn {
        get_type => || ffi::gtk_text_mark_get_type(),
    }
}

impl TextMark {
    pub fn new<'a, P: Into<Option<&'a str>>>(name: P, left_gravity: bool) -> TextMark {
        assert_initialized_main_thread!();
        let name = name.into();
        let name = name.to_glib_none();
        unsafe {
            from_glib_full(ffi::gtk_text_mark_new(name.0, left_gravity.to_glib()))
        }
    }
}

pub trait TextMarkExt {
    fn get_buffer(&self) -> Option<TextBuffer>;

    fn get_deleted(&self) -> bool;

    fn get_left_gravity(&self) -> bool;

    fn get_name(&self) -> Option<String>;

    fn get_visible(&self) -> bool;

    fn set_visible(&self, setting: bool);
}

impl<O: IsA<TextMark>> TextMarkExt for O {
    fn get_buffer(&self) -> Option<TextBuffer> {
        unsafe {
            from_glib_none(ffi::gtk_text_mark_get_buffer(self.to_glib_none().0))
        }
    }

    fn get_deleted(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_mark_get_deleted(self.to_glib_none().0))
        }
    }

    fn get_left_gravity(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_mark_get_left_gravity(self.to_glib_none().0))
        }
    }

    fn get_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_text_mark_get_name(self.to_glib_none().0))
        }
    }

    fn get_visible(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_text_mark_get_visible(self.to_glib_none().0))
        }
    }

    fn set_visible(&self, setting: bool) {
        unsafe {
            ffi::gtk_text_mark_set_visible(self.to_glib_none().0, setting.to_glib());
        }
    }
}
