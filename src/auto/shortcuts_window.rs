// This file was generated by gir (6bcd52a) from gir-files (1069259)
// DO NOT EDIT

use Bin;
use Container;
use Widget;
use Window;
use ffi;
use glib;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct ShortcutsWindow(Object<ffi::GtkShortcutsWindow>): Window, Bin, Container, Widget;

    match fn {
        get_type => || ffi::gtk_shortcuts_window_get_type(),
    }
}

pub trait ShortcutsWindowExt {
    fn get_property_section_name(&self) -> Option<String>;

    fn set_property_section_name(&self, section_name: Option<&str>);

    fn get_property_view_name(&self) -> Option<String>;

    fn set_property_view_name(&self, view_name: Option<&str>);

    fn connect_close<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_search<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_section_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;

    fn connect_property_view_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<ShortcutsWindow> + IsA<glib::object::Object>> ShortcutsWindowExt for O {
    fn get_property_section_name(&self) -> Option<String> {
        let mut value = Value::from(None::<&str>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "section-name".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    fn set_property_section_name(&self, section_name: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "section-name".to_glib_none().0, Value::from(section_name).to_glib_none().0);
        }
    }

    fn get_property_view_name(&self) -> Option<String> {
        let mut value = Value::from(None::<&str>);
        unsafe {
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "view-name".to_glib_none().0, value.to_glib_none_mut().0);
        }
        value.get()
    }

    fn set_property_view_name(&self, view_name: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "view-name".to_glib_none().0, Value::from(view_name).to_glib_none().0);
        }
    }

    fn connect_close<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "close",
                transmute(close_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_search<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "search",
                transmute(search_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_section_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::section-name",
                transmute(notify_section_name_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_view_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::view-name",
                transmute(notify_view_name_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn close_trampoline<P>(this: *mut ffi::GtkShortcutsWindow, f: glib_ffi::gpointer)
where P: IsA<ShortcutsWindow> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ShortcutsWindow::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn search_trampoline<P>(this: *mut ffi::GtkShortcutsWindow, f: glib_ffi::gpointer)
where P: IsA<ShortcutsWindow> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ShortcutsWindow::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn notify_section_name_trampoline<P>(this: *mut ffi::GtkShortcutsWindow, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ShortcutsWindow> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ShortcutsWindow::from_glib_none(this).downcast_unchecked())
}

unsafe extern "C" fn notify_view_name_trampoline<P>(this: *mut ffi::GtkShortcutsWindow, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<ShortcutsWindow> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&ShortcutsWindow::from_glib_none(this).downcast_unchecked())
}
