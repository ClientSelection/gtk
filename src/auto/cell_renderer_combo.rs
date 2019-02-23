// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use CellRenderer;
use CellRendererText;
use TreeIter;
use TreeModel;
use TreePath;
use ffi;
use glib::StaticType;
use glib::Value;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect_raw;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use libc;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib_wrapper! {
    pub struct CellRendererCombo(Object<ffi::GtkCellRendererCombo, ffi::GtkCellRendererComboClass, CellRendererComboClass>) @extends CellRendererText, CellRenderer;

    match fn {
        get_type => || ffi::gtk_cell_renderer_combo_get_type(),
    }
}

impl CellRendererCombo {
    pub fn new() -> CellRendererCombo {
        assert_initialized_main_thread!();
        unsafe {
            CellRenderer::from_glib_none(ffi::gtk_cell_renderer_combo_new()).unsafe_cast()
        }
    }
}

impl Default for CellRendererCombo {
    fn default() -> Self {
        Self::new()
    }
}

pub const NONE_CELL_RENDERER_COMBO: Option<&CellRendererCombo> = None;

pub trait CellRendererComboExt: 'static {
    fn get_property_has_entry(&self) -> bool;

    fn set_property_has_entry(&self, has_entry: bool);

    fn get_property_model(&self) -> Option<TreeModel>;

    fn set_property_model(&self, model: Option<&TreeModel>);

    fn get_property_text_column(&self) -> i32;

    fn set_property_text_column(&self, text_column: i32);

    fn connect_changed<F: Fn(&Self, TreePath, &TreeIter) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_has_entry_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_model_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_text_column_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<CellRendererCombo>> CellRendererComboExt for O {
    fn get_property_has_entry(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"has-entry\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_has_entry(&self, has_entry: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"has-entry\0".as_ptr() as *const _, Value::from(&has_entry).to_glib_none().0);
        }
    }

    fn get_property_model(&self) -> Option<TreeModel> {
        unsafe {
            let mut value = Value::from_type(<TreeModel as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"model\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn set_property_model(&self, model: Option<&TreeModel>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"model\0".as_ptr() as *const _, Value::from(model).to_glib_none().0);
        }
    }

    fn get_property_text_column(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"text-column\0".as_ptr() as *const _, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn set_property_text_column(&self, text_column: i32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0 as *mut gobject_ffi::GObject, b"text-column\0".as_ptr() as *const _, Value::from(&text_column).to_glib_none().0);
        }
    }

    fn connect_changed<F: Fn(&Self, TreePath, &TreeIter) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"changed\0".as_ptr() as *const _,
                Some(transmute(changed_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_has_entry_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::has-entry\0".as_ptr() as *const _,
                Some(transmute(notify_has_entry_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_model_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::model\0".as_ptr() as *const _,
                Some(transmute(notify_model_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }

    fn connect_property_text_column_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"notify::text-column\0".as_ptr() as *const _,
                Some(transmute(notify_text_column_trampoline::<Self, F> as usize)), Box_::into_raw(f))
        }
    }
}

unsafe extern "C" fn changed_trampoline<P, F: Fn(&P, TreePath, &TreeIter) + 'static>(this: *mut ffi::GtkCellRendererCombo, path_string: *mut libc::c_char, new_iter: *mut ffi::GtkTreeIter, f: glib_ffi::gpointer)
where P: IsA<CellRendererCombo> {
    let f: &F = &*(f as *const F);
    let path = from_glib_full(ffi::gtk_tree_path_new_from_string(path_string));
    f(&CellRendererCombo::from_glib_borrow(this).unsafe_cast(), path, &from_glib_borrow(new_iter))
}

unsafe extern "C" fn notify_has_entry_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkCellRendererCombo, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<CellRendererCombo> {
    let f: &F = &*(f as *const F);
    f(&CellRendererCombo::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_model_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkCellRendererCombo, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<CellRendererCombo> {
    let f: &F = &*(f as *const F);
    f(&CellRendererCombo::from_glib_borrow(this).unsafe_cast())
}

unsafe extern "C" fn notify_text_column_trampoline<P, F: Fn(&P) + 'static>(this: *mut ffi::GtkCellRendererCombo, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<CellRendererCombo> {
    let f: &F = &*(f as *const F);
    f(&CellRendererCombo::from_glib_borrow(this).unsafe_cast())
}

impl fmt::Display for CellRendererCombo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "CellRendererCombo")
    }
}
