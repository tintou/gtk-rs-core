// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use Object;
use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct Selection(Object<ffi::AtkSelection, ffi::AtkSelectionIface>);

    match fn {
        get_type => || ffi::atk_selection_get_type(),
    }
}

pub trait SelectionExt {
    fn add_selection(&self, i: i32) -> bool;

    fn clear_selection(&self) -> bool;

    fn get_selection_count(&self) -> i32;

    fn is_child_selected(&self, i: i32) -> bool;

    fn ref_selection(&self, i: i32) -> Option<Object>;

    fn remove_selection(&self, i: i32) -> bool;

    fn select_all_selection(&self) -> bool;

    fn connect_selection_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Selection> + IsA<glib::object::Object>> SelectionExt for O {
    fn add_selection(&self, i: i32) -> bool {
        unsafe {
            from_glib(ffi::atk_selection_add_selection(self.to_glib_none().0, i))
        }
    }

    fn clear_selection(&self) -> bool {
        unsafe {
            from_glib(ffi::atk_selection_clear_selection(self.to_glib_none().0))
        }
    }

    fn get_selection_count(&self) -> i32 {
        unsafe {
            ffi::atk_selection_get_selection_count(self.to_glib_none().0)
        }
    }

    fn is_child_selected(&self, i: i32) -> bool {
        unsafe {
            from_glib(ffi::atk_selection_is_child_selected(self.to_glib_none().0, i))
        }
    }

    fn ref_selection(&self, i: i32) -> Option<Object> {
        unsafe {
            from_glib_full(ffi::atk_selection_ref_selection(self.to_glib_none().0, i))
        }
    }

    fn remove_selection(&self, i: i32) -> bool {
        unsafe {
            from_glib(ffi::atk_selection_remove_selection(self.to_glib_none().0, i))
        }
    }

    fn select_all_selection(&self) -> bool {
        unsafe {
            from_glib(ffi::atk_selection_select_all_selection(self.to_glib_none().0))
        }
    }

    fn connect_selection_changed<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "selection-changed",
                transmute(selection_changed_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn selection_changed_trampoline<P>(this: *mut ffi::AtkSelection, f: glib_ffi::gpointer)
where P: IsA<Selection> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Selection::from_glib_borrow(this).downcast_unchecked())
}
