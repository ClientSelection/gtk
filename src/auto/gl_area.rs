// This file was generated by gir (ce03df6) from gir-files (71d73f0)
// DO NOT EDIT

#[cfg(feature = "v3_16")]
use Error;
use Widget;
use ffi;
#[cfg(feature = "v3_16")]
use gdk;
#[cfg(feature = "v3_16")]
use gdk_ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
#[cfg(feature = "v3_16")]
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
#[cfg(feature = "v3_16")]
use libc;
#[cfg(feature = "v3_16")]
use signal::Inhibit;
#[cfg(feature = "v3_16")]
use std::boxed::Box as Box_;
use std::mem;
#[cfg(feature = "v3_16")]
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct GLArea(Object<ffi::GtkGLArea>): Widget;

    match fn {
        get_type => || ffi::gtk_gl_area_get_type(),
    }
}

impl GLArea {
    #[cfg(feature = "v3_16")]
    pub fn new() -> GLArea {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_full(ffi::gtk_gl_area_new()).downcast_unchecked()
        }
    }
}

pub trait GLAreaExt {
    #[cfg(feature = "v3_16")]
    fn attach_buffers(&self);

    #[cfg(feature = "v3_16")]
    fn get_auto_render(&self) -> bool;

    #[cfg(feature = "v3_16")]
    fn get_context(&self) -> Option<gdk::GLContext>;

    #[cfg(feature = "v3_16")]
    fn get_error(&self) -> Option<Error>;

    #[cfg(feature = "v3_16")]
    fn get_has_alpha(&self) -> bool;

    #[cfg(feature = "v3_16")]
    fn get_has_depth_buffer(&self) -> bool;

    #[cfg(feature = "v3_16")]
    fn get_has_stencil_buffer(&self) -> bool;

    #[cfg(feature = "v3_16")]
    fn get_required_version(&self) -> (i32, i32);

    #[cfg(feature = "v3_22")]
    fn get_use_es(&self) -> bool;

    #[cfg(feature = "v3_16")]
    fn make_current(&self);

    #[cfg(feature = "v3_16")]
    fn queue_render(&self);

    #[cfg(feature = "v3_16")]
    fn set_auto_render(&self, auto_render: bool);

    #[cfg(feature = "v3_16")]
    fn set_error<'a, P: Into<Option<&'a Error>>>(&self, error: P);

    #[cfg(feature = "v3_16")]
    fn set_has_alpha(&self, has_alpha: bool);

    #[cfg(feature = "v3_16")]
    fn set_has_depth_buffer(&self, has_depth_buffer: bool);

    #[cfg(feature = "v3_16")]
    fn set_has_stencil_buffer(&self, has_stencil_buffer: bool);

    #[cfg(feature = "v3_16")]
    fn set_required_version(&self, major: i32, minor: i32);

    #[cfg(feature = "v3_22")]
    fn set_use_es(&self, use_es: bool);

    #[cfg(feature = "v3_16")]
    fn connect_create_context<F: Fn(&Self) -> gdk::GLContext + 'static>(&self, f: F) -> u64;

    #[cfg(feature = "v3_16")]
    fn connect_render<F: Fn(&Self, &gdk::GLContext) -> Inhibit + 'static>(&self, f: F) -> u64;

    #[cfg(feature = "v3_16")]
    fn connect_resize<F: Fn(&Self, i32, i32) + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<GLArea> + IsA<glib::object::Object>> GLAreaExt for O {
    #[cfg(feature = "v3_16")]
    fn attach_buffers(&self) {
        unsafe {
            ffi::gtk_gl_area_attach_buffers(self.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_16")]
    fn get_auto_render(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_gl_area_get_auto_render(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_16")]
    fn get_context(&self) -> Option<gdk::GLContext> {
        unsafe {
            from_glib_none(ffi::gtk_gl_area_get_context(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_16")]
    fn get_error(&self) -> Option<Error> {
        unsafe {
            from_glib_none(ffi::gtk_gl_area_get_error(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_16")]
    fn get_has_alpha(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_gl_area_get_has_alpha(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_16")]
    fn get_has_depth_buffer(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_gl_area_get_has_depth_buffer(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_16")]
    fn get_has_stencil_buffer(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_gl_area_get_has_stencil_buffer(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_16")]
    fn get_required_version(&self) -> (i32, i32) {
        unsafe {
            let mut major = mem::uninitialized();
            let mut minor = mem::uninitialized();
            ffi::gtk_gl_area_get_required_version(self.to_glib_none().0, &mut major, &mut minor);
            (major, minor)
        }
    }

    #[cfg(feature = "v3_22")]
    fn get_use_es(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_gl_area_get_use_es(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_16")]
    fn make_current(&self) {
        unsafe {
            ffi::gtk_gl_area_make_current(self.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_16")]
    fn queue_render(&self) {
        unsafe {
            ffi::gtk_gl_area_queue_render(self.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_16")]
    fn set_auto_render(&self, auto_render: bool) {
        unsafe {
            ffi::gtk_gl_area_set_auto_render(self.to_glib_none().0, auto_render.to_glib());
        }
    }

    #[cfg(feature = "v3_16")]
    fn set_error<'a, P: Into<Option<&'a Error>>>(&self, error: P) {
        let error = error.into();
        let error = error.to_glib_none();
        unsafe {
            ffi::gtk_gl_area_set_error(self.to_glib_none().0, error.0);
        }
    }

    #[cfg(feature = "v3_16")]
    fn set_has_alpha(&self, has_alpha: bool) {
        unsafe {
            ffi::gtk_gl_area_set_has_alpha(self.to_glib_none().0, has_alpha.to_glib());
        }
    }

    #[cfg(feature = "v3_16")]
    fn set_has_depth_buffer(&self, has_depth_buffer: bool) {
        unsafe {
            ffi::gtk_gl_area_set_has_depth_buffer(self.to_glib_none().0, has_depth_buffer.to_glib());
        }
    }

    #[cfg(feature = "v3_16")]
    fn set_has_stencil_buffer(&self, has_stencil_buffer: bool) {
        unsafe {
            ffi::gtk_gl_area_set_has_stencil_buffer(self.to_glib_none().0, has_stencil_buffer.to_glib());
        }
    }

    #[cfg(feature = "v3_16")]
    fn set_required_version(&self, major: i32, minor: i32) {
        unsafe {
            ffi::gtk_gl_area_set_required_version(self.to_glib_none().0, major, minor);
        }
    }

    #[cfg(feature = "v3_22")]
    fn set_use_es(&self, use_es: bool) {
        unsafe {
            ffi::gtk_gl_area_set_use_es(self.to_glib_none().0, use_es.to_glib());
        }
    }

    #[cfg(feature = "v3_16")]
    fn connect_create_context<F: Fn(&Self) -> gdk::GLContext + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) -> gdk::GLContext + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "create-context",
                transmute(create_context_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(feature = "v3_16")]
    fn connect_render<F: Fn(&Self, &gdk::GLContext) -> Inhibit + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &gdk::GLContext) -> Inhibit + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "render",
                transmute(render_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(feature = "v3_16")]
    fn connect_resize<F: Fn(&Self, i32, i32) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self, i32, i32) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "resize",
                transmute(resize_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

#[cfg(feature = "v3_16")]
unsafe extern "C" fn create_context_trampoline<P>(this: *mut ffi::GtkGLArea, f: glib_ffi::gpointer) -> *mut gdk_ffi::GdkGLContext
where P: IsA<GLArea> {
    callback_guard!();
    let f: &Box_<Fn(&P) -> gdk::GLContext + 'static> = transmute(f);
    f(&GLArea::from_glib_none(this).downcast_unchecked()).to_glib_full()
}

#[cfg(feature = "v3_16")]
unsafe extern "C" fn render_trampoline<P>(this: *mut ffi::GtkGLArea, context: *mut gdk_ffi::GdkGLContext, f: glib_ffi::gpointer) -> glib_ffi::gboolean
where P: IsA<GLArea> {
    callback_guard!();
    let f: &Box_<Fn(&P, &gdk::GLContext) -> Inhibit + 'static> = transmute(f);
    f(&GLArea::from_glib_none(this).downcast_unchecked(), &from_glib_none(context)).to_glib()
}

#[cfg(feature = "v3_16")]
unsafe extern "C" fn resize_trampoline<P>(this: *mut ffi::GtkGLArea, width: libc::c_int, height: libc::c_int, f: glib_ffi::gpointer)
where P: IsA<GLArea> {
    callback_guard!();
    let f: &Box_<Fn(&P, i32, i32) + 'static> = transmute(f);
    f(&GLArea::from_glib_none(this).downcast_unchecked(), width, height)
}
