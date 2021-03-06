// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Adjustment;
use Buildable;
use Orientable;
use Orientation;
use Range;
use Widget;
use glib::object::Cast;
use glib::object::IsA;
use glib::translate::*;
use gtk_sys;
use std::fmt;

glib_wrapper! {
    pub struct Scrollbar(Object<gtk_sys::GtkScrollbar, gtk_sys::GtkScrollbarClass, ScrollbarClass>) @extends Range, Widget, @implements Buildable, Orientable;

    match fn {
        get_type => || gtk_sys::gtk_scrollbar_get_type(),
    }
}

impl Scrollbar {
    pub fn new<P: IsA<Adjustment>>(orientation: Orientation, adjustment: Option<&P>) -> Scrollbar {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(gtk_sys::gtk_scrollbar_new(orientation.to_glib(), adjustment.map(|p| p.as_ref()).to_glib_none().0)).unsafe_cast()
        }
    }
}

pub const NONE_SCROLLBAR: Option<&Scrollbar> = None;

impl fmt::Display for Scrollbar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Scrollbar")
    }
}
