// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Bin;
use Buildable;
use Container;
use Entry;
use Widget;
use gdk;
use glib::StaticType;
use glib::Value;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_sys;
use gobject_sys;
use gtk_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct SearchBar(Object<gtk_sys::GtkSearchBar, gtk_sys::GtkSearchBarClass, SearchBarClass>) @extends Bin, Container, Widget, @implements Buildable;

    match fn {
        get_type => || gtk_sys::gtk_search_bar_get_type(),
    }
}

impl SearchBar {
    pub fn new() -> SearchBar {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(gtk_sys::gtk_search_bar_new()).unsafe_cast()
        }
    }
}

impl Default for SearchBar {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_SEARCH_BAR: Option<&SearchBar> = None;

pub trait SearchBarExt: 'static {
    fn connect_entry<P: IsA<Entry>>(&self, entry: &P);

    fn get_search_mode(&self) -> bool;

    fn get_show_close_button(&self) -> bool;

    fn handle_event(&self, event: &gdk::Event) -> bool;

    fn set_search_mode(&self, search_mode: bool);

    fn set_show_close_button(&self, visible: bool);

    fn get_property_search_mode_enabled(&self) -> bool;

    fn set_property_search_mode_enabled(&self, search_mode_enabled: bool);

    fn connect_property_search_mode_enabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_show_close_button_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<SearchBar>> SearchBarExt for O {
    fn connect_entry<P: IsA<Entry>>(&self, entry: &P) {
        unsafe {
            gtk_sys::gtk_search_bar_connect_entry(self.as_ref().to_glib_none().0, entry.as_ref().to_glib_none().0);
        }
    }

    fn get_search_mode(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_search_bar_get_search_mode(self.as_ref().to_glib_none().0))
        }
    }

    fn get_show_close_button(&self) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_search_bar_get_show_close_button(self.as_ref().to_glib_none().0))
        }
    }

    fn handle_event(&self, event: &gdk::Event) -> bool {
        unsafe {
            from_glib(gtk_sys::gtk_search_bar_handle_event(self.as_ref().to_glib_none().0, mut_override(event.to_glib_none().0)))
        }
    }

    fn set_search_mode(&self, search_mode: bool) {
        unsafe {
            gtk_sys::gtk_search_bar_set_search_mode(self.as_ref().to_glib_none().0, search_mode.to_glib());
        }
    }

    fn set_show_close_button(&self, visible: bool) {
        unsafe {
            gtk_sys::gtk_search_bar_set_show_close_button(self.as_ref().to_glib_none().0, visible.to_glib());
        }
    }

    fn get_property_search_mode_enabled(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_sys::g_object_get_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"search-mode-enabled\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_search_mode_enabled(&self, search_mode_enabled: bool) {
        unsafe {
            gobject_sys::g_object_set_property(self.to_glib_none().0 as *mut gobject_sys::GObject, b"search-mode-enabled\0".as_ptr() as *const _, Value::from(&search_mode_enabled).to_glib_none().0);
        }
    }

    fn connect_property_search_mode_enabled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::search-mode-enabled\0".as_ptr() as *const _,
                Some(transmute(notify_search_mode_enabled_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_show_close_button_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::show-close-button\0".as_ptr() as *const _,
                Some(transmute(notify_show_close_button_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn notify_search_mode_enabled_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkSearchBar, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<SearchBar> {
    let f: &F = &*(f as *const F);
    f(&SearchBar::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_show_close_button_trampoline<P, F: Fn(&P) + 'static>(this: *mut gtk_sys::GtkSearchBar, _param_spec: glib_sys::gpointer, f: glib_sys::gpointer)
where P: IsA<SearchBar> {
    let f: &F = &*(f as *const F);
    f(&SearchBar::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for SearchBar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SearchBar")
    }
}
