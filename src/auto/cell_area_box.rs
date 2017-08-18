// This file was generated by gir (6bcd52a) from gir-files (1069259)
// DO NOT EDIT

use CellArea;
use CellLayout;
use CellRenderer;
use Orientable;
use ffi;
use glib;
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
    pub struct CellAreaBox(Object<ffi::GtkCellAreaBox>): CellArea, CellLayout, Orientable;

    match fn {
        get_type => || ffi::gtk_cell_area_box_get_type(),
    }
}

impl CellAreaBox {
    pub fn new() -> CellAreaBox {
        assert_initialized_main_thread!();
        unsafe {
            CellArea::from_glib_none(ffi::gtk_cell_area_box_new()).downcast_unchecked()
        }
    }
}

impl Default for CellAreaBox {
    fn default() -> Self {
        Self::new()
    }
}

pub trait CellAreaBoxExt {
    fn get_spacing(&self) -> i32;

    fn pack_end<P: IsA<CellRenderer>>(&self, renderer: &P, expand: bool, align: bool, fixed: bool);

    fn pack_start<P: IsA<CellRenderer>>(&self, renderer: &P, expand: bool, align: bool, fixed: bool);

    fn set_spacing(&self, spacing: i32);

    fn connect_property_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64;
}

impl<O: IsA<CellAreaBox> + IsA<glib::object::Object>> CellAreaBoxExt for O {
    fn get_spacing(&self) -> i32 {
        unsafe {
            ffi::gtk_cell_area_box_get_spacing(self.to_glib_none().0)
        }
    }

    fn pack_end<P: IsA<CellRenderer>>(&self, renderer: &P, expand: bool, align: bool, fixed: bool) {
        unsafe {
            ffi::gtk_cell_area_box_pack_end(self.to_glib_none().0, renderer.to_glib_none().0, expand.to_glib(), align.to_glib(), fixed.to_glib());
        }
    }

    fn pack_start<P: IsA<CellRenderer>>(&self, renderer: &P, expand: bool, align: bool, fixed: bool) {
        unsafe {
            ffi::gtk_cell_area_box_pack_start(self.to_glib_none().0, renderer.to_glib_none().0, expand.to_glib(), align.to_glib(), fixed.to_glib());
        }
    }

    fn set_spacing(&self, spacing: i32) {
        unsafe {
            ffi::gtk_cell_area_box_set_spacing(self.to_glib_none().0, spacing);
        }
    }

    fn connect_property_spacing_notify<F: Fn(&Self) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::spacing",
                transmute(notify_spacing_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_spacing_trampoline<P>(this: *mut ffi::GtkCellAreaBox, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<CellAreaBox> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&CellAreaBox::from_glib_none(this).downcast_unchecked())
}
