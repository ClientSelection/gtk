// This file was generated by gir (ce03df6) from gir-files (71d73f0)
// DO NOT EDIT

use Container;
use StackTransitionType;
use Widget;
use ffi;
use glib;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct Stack(Object<ffi::GtkStack>): Container, Widget;

    match fn {
        get_type => || ffi::gtk_stack_get_type(),
    }
}

impl Stack {
    #[cfg(feature = "v3_10")]
    pub fn new() -> Stack {
        assert_initialized_main_thread!();
        unsafe {
            Widget::from_glib_none(ffi::gtk_stack_new()).downcast_unchecked()
        }
    }
}

pub trait StackExt {
    #[cfg(feature = "v3_10")]
    fn add_named<P: IsA<Widget>>(&self, child: &P, name: &str);

    #[cfg(feature = "v3_10")]
    fn add_titled<P: IsA<Widget>>(&self, child: &P, name: &str, title: &str);

    #[cfg(feature = "v3_12")]
    fn get_child_by_name(&self, name: &str) -> Option<Widget>;

    #[cfg(feature = "v3_16")]
    fn get_hhomogeneous(&self) -> bool;

    #[cfg(feature = "v3_10")]
    fn get_homogeneous(&self) -> bool;

    #[cfg(feature = "v3_18")]
    fn get_interpolate_size(&self) -> bool;

    #[cfg(feature = "v3_10")]
    fn get_transition_duration(&self) -> u32;

    #[cfg(feature = "v3_12")]
    fn get_transition_running(&self) -> bool;

    #[cfg(feature = "v3_10")]
    fn get_transition_type(&self) -> StackTransitionType;

    #[cfg(feature = "v3_16")]
    fn get_vhomogeneous(&self) -> bool;

    #[cfg(feature = "v3_10")]
    fn get_visible_child(&self) -> Option<Widget>;

    #[cfg(feature = "v3_10")]
    fn get_visible_child_name(&self) -> Option<String>;

    #[cfg(feature = "v3_16")]
    fn set_hhomogeneous(&self, hhomogeneous: bool);

    #[cfg(feature = "v3_10")]
    fn set_homogeneous(&self, homogeneous: bool);

    #[cfg(feature = "v3_18")]
    fn set_interpolate_size(&self, interpolate_size: bool);

    #[cfg(feature = "v3_10")]
    fn set_transition_duration(&self, duration: u32);

    #[cfg(feature = "v3_10")]
    fn set_transition_type(&self, transition: StackTransitionType);

    #[cfg(feature = "v3_16")]
    fn set_vhomogeneous(&self, vhomogeneous: bool);

    #[cfg(feature = "v3_10")]
    fn set_visible_child<P: IsA<Widget>>(&self, child: &P);

    #[cfg(feature = "v3_10")]
    fn set_visible_child_full(&self, name: &str, transition: StackTransitionType);

    #[cfg(feature = "v3_10")]
    fn set_visible_child_name(&self, name: &str);

    fn get_property_homogeneous(&self) -> bool;

    fn set_property_homogeneous(&self, homogeneous: bool);

    fn get_property_interpolate_size(&self) -> bool;

    fn set_property_interpolate_size(&self, interpolate_size: bool);

    fn get_property_transition_duration(&self) -> u32;

    fn set_property_transition_duration(&self, transition_duration: u32);

    fn get_property_transition_running(&self) -> bool;

    fn get_property_transition_type(&self) -> StackTransitionType;

    fn set_property_transition_type(&self, transition_type: StackTransitionType);

    fn get_property_visible_child(&self) -> Option<Widget>;

    fn set_property_visible_child<P: IsA<Widget> + IsA<glib::object::Object> + glib::value::SetValueOptional>(&self, visible_child: Option<&P>);

    fn get_property_visible_child_name(&self) -> Option<String>;

    fn set_property_visible_child_name(&self, visible_child_name: Option<&str>);

    fn get_child_icon_name<T: IsA<Widget>>(&self, item: &T) -> Option<String>;

    fn set_child_icon_name<'a, P: Into<Option<&'a str>>, T: IsA<Widget>>(&self, item: &T, icon_name: P);

    fn get_child_name<T: IsA<Widget>>(&self, item: &T) -> Option<String>;

    fn set_child_name<'a, P: Into<Option<&'a str>>, T: IsA<Widget>>(&self, item: &T, name: P);

    fn get_child_needs_attention<T: IsA<Widget>>(&self, item: &T) -> bool;

    fn set_child_needs_attention<T: IsA<Widget>>(&self, item: &T, needs_attention: bool);

    fn get_child_position<T: IsA<Widget>>(&self, item: &T) -> i32;

    fn set_child_position<T: IsA<Widget>>(&self, item: &T, position: i32);

    fn get_child_title<T: IsA<Widget>>(&self, item: &T) -> Option<String>;

    fn set_child_title<'a, P: Into<Option<&'a str>>, T: IsA<Widget>>(&self, item: &T, title: P);
}

impl<O: IsA<Stack> + IsA<Container> + IsA<glib::object::Object>> StackExt for O {
    #[cfg(feature = "v3_10")]
    fn add_named<P: IsA<Widget>>(&self, child: &P, name: &str) {
        unsafe {
            ffi::gtk_stack_add_named(self.to_glib_none().0, child.to_glib_none().0, name.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_10")]
    fn add_titled<P: IsA<Widget>>(&self, child: &P, name: &str, title: &str) {
        unsafe {
            ffi::gtk_stack_add_titled(self.to_glib_none().0, child.to_glib_none().0, name.to_glib_none().0, title.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_12")]
    fn get_child_by_name(&self, name: &str) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_stack_get_child_by_name(self.to_glib_none().0, name.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_16")]
    fn get_hhomogeneous(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_stack_get_hhomogeneous(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_10")]
    fn get_homogeneous(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_stack_get_homogeneous(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_18")]
    fn get_interpolate_size(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_stack_get_interpolate_size(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_10")]
    fn get_transition_duration(&self) -> u32 {
        unsafe {
            ffi::gtk_stack_get_transition_duration(self.to_glib_none().0)
        }
    }

    #[cfg(feature = "v3_12")]
    fn get_transition_running(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_stack_get_transition_running(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_10")]
    fn get_transition_type(&self) -> StackTransitionType {
        unsafe {
            from_glib(ffi::gtk_stack_get_transition_type(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_16")]
    fn get_vhomogeneous(&self) -> bool {
        unsafe {
            from_glib(ffi::gtk_stack_get_vhomogeneous(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_10")]
    fn get_visible_child(&self) -> Option<Widget> {
        unsafe {
            from_glib_none(ffi::gtk_stack_get_visible_child(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_10")]
    fn get_visible_child_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::gtk_stack_get_visible_child_name(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_16")]
    fn set_hhomogeneous(&self, hhomogeneous: bool) {
        unsafe {
            ffi::gtk_stack_set_hhomogeneous(self.to_glib_none().0, hhomogeneous.to_glib());
        }
    }

    #[cfg(feature = "v3_10")]
    fn set_homogeneous(&self, homogeneous: bool) {
        unsafe {
            ffi::gtk_stack_set_homogeneous(self.to_glib_none().0, homogeneous.to_glib());
        }
    }

    #[cfg(feature = "v3_18")]
    fn set_interpolate_size(&self, interpolate_size: bool) {
        unsafe {
            ffi::gtk_stack_set_interpolate_size(self.to_glib_none().0, interpolate_size.to_glib());
        }
    }

    #[cfg(feature = "v3_10")]
    fn set_transition_duration(&self, duration: u32) {
        unsafe {
            ffi::gtk_stack_set_transition_duration(self.to_glib_none().0, duration);
        }
    }

    #[cfg(feature = "v3_10")]
    fn set_transition_type(&self, transition: StackTransitionType) {
        unsafe {
            ffi::gtk_stack_set_transition_type(self.to_glib_none().0, transition.to_glib());
        }
    }

    #[cfg(feature = "v3_16")]
    fn set_vhomogeneous(&self, vhomogeneous: bool) {
        unsafe {
            ffi::gtk_stack_set_vhomogeneous(self.to_glib_none().0, vhomogeneous.to_glib());
        }
    }

    #[cfg(feature = "v3_10")]
    fn set_visible_child<P: IsA<Widget>>(&self, child: &P) {
        unsafe {
            ffi::gtk_stack_set_visible_child(self.to_glib_none().0, child.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_10")]
    fn set_visible_child_full(&self, name: &str, transition: StackTransitionType) {
        unsafe {
            ffi::gtk_stack_set_visible_child_full(self.to_glib_none().0, name.to_glib_none().0, transition.to_glib());
        }
    }

    #[cfg(feature = "v3_10")]
    fn set_visible_child_name(&self, name: &str) {
        unsafe {
            ffi::gtk_stack_set_visible_child_name(self.to_glib_none().0, name.to_glib_none().0);
        }
    }

    fn get_property_homogeneous(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "homogeneous".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_homogeneous(&self, homogeneous: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "homogeneous".to_glib_none().0, Value::from(&homogeneous).to_glib_none().0);
        }
    }

    fn get_property_interpolate_size(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "interpolate-size".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_interpolate_size(&self, interpolate_size: bool) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "interpolate-size".to_glib_none().0, Value::from(&interpolate_size).to_glib_none().0);
        }
    }

    fn get_property_transition_duration(&self) -> u32 {
        let mut value = Value::from(&0u32);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "transition-duration".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_property_transition_duration(&self, transition_duration: u32) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "transition-duration".to_glib_none().0, Value::from(&transition_duration).to_glib_none().0);
        }
    }

    fn get_property_transition_running(&self) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "transition-running".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn get_property_transition_type(&self) -> StackTransitionType {
        let mut value = Value::from(&0);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "transition-type".to_glib_none().0, value.to_glib_none_mut().0);
            from_glib(transmute(value.get::<i32>().unwrap()))
        }
    }

    fn set_property_transition_type(&self, transition_type: StackTransitionType) {
        let transition_type = transition_type.to_glib() as i32;
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "transition-type".to_glib_none().0, Value::from(&transition_type).to_glib_none().0);
        }
    }

    fn get_property_visible_child(&self) -> Option<Widget> {
        let mut value = Value::from(None::<&Widget>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "visible-child".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    fn set_property_visible_child<P: IsA<Widget> + IsA<glib::object::Object> + glib::value::SetValueOptional>(&self, visible_child: Option<&P>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "visible-child".to_glib_none().0, Value::from(visible_child).to_glib_none().0);
        }
    }

    fn get_property_visible_child_name(&self) -> Option<String> {
        let mut value = Value::from(None::<&str>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "visible-child-name".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    fn set_property_visible_child_name(&self, visible_child_name: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "visible-child-name".to_glib_none().0, Value::from(visible_child_name).to_glib_none().0);
        }
    }

    fn get_child_icon_name<T: IsA<Widget>>(&self, item: &T) -> Option<String> {
        let mut value = Value::from(None::<&str>);
        unsafe {
            ffi::gtk_container_child_get_property(self.to_glib_none().0, item.to_glib_none().0, "icon-name".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    fn set_child_icon_name<'a, P: Into<Option<&'a str>>, T: IsA<Widget>>(&self, item: &T, icon_name: P) {
        let icon_name = icon_name.into();
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0, item.to_glib_none().0, "icon-name".to_glib_none().0, Value::from(icon_name).to_glib_none().0);
        }
    }

    fn get_child_name<T: IsA<Widget>>(&self, item: &T) -> Option<String> {
        let mut value = Value::from(None::<&str>);
        unsafe {
            ffi::gtk_container_child_get_property(self.to_glib_none().0, item.to_glib_none().0, "name".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    fn set_child_name<'a, P: Into<Option<&'a str>>, T: IsA<Widget>>(&self, item: &T, name: P) {
        let name = name.into();
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0, item.to_glib_none().0, "name".to_glib_none().0, Value::from(name).to_glib_none().0);
        }
    }

    fn get_child_needs_attention<T: IsA<Widget>>(&self, item: &T) -> bool {
        let mut value = Value::from(&false);
        unsafe {
            ffi::gtk_container_child_get_property(self.to_glib_none().0, item.to_glib_none().0, "needs-attention".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_child_needs_attention<T: IsA<Widget>>(&self, item: &T, needs_attention: bool) {
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0, item.to_glib_none().0, "needs-attention".to_glib_none().0, Value::from(&needs_attention).to_glib_none().0);
        }
    }

    fn get_child_position<T: IsA<Widget>>(&self, item: &T) -> i32 {
        let mut value = Value::from(&0);
        unsafe {
            ffi::gtk_container_child_get_property(self.to_glib_none().0, item.to_glib_none().0, "position".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get().unwrap()
    }

    fn set_child_position<T: IsA<Widget>>(&self, item: &T, position: i32) {
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0, item.to_glib_none().0, "position".to_glib_none().0, Value::from(&position).to_glib_none().0);
        }
    }

    fn get_child_title<T: IsA<Widget>>(&self, item: &T) -> Option<String> {
        let mut value = Value::from(None::<&str>);
        unsafe {
            ffi::gtk_container_child_get_property(self.to_glib_none().0, item.to_glib_none().0, "title".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    fn set_child_title<'a, P: Into<Option<&'a str>>, T: IsA<Widget>>(&self, item: &T, title: P) {
        let title = title.into();
        unsafe {
            ffi::gtk_container_child_set_property(self.to_glib_none().0, item.to_glib_none().0, "title".to_glib_none().0, Value::from(title).to_glib_none().0);
        }
    }
}
