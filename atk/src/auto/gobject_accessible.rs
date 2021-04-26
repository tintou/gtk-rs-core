// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Object;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    pub struct GObjectAccessible(Object<ffi::AtkGObjectAccessible, ffi::AtkGObjectAccessibleClass>) @extends Object;

    match fn {
        type_ => || ffi::atk_gobject_accessible_get_type(),
    }
}

impl GObjectAccessible {
    #[doc(alias = "atk_gobject_accessible_for_object")]
    pub fn for_object<P: IsA<glib::Object>>(obj: &P) -> Option<Object> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::atk_gobject_accessible_for_object(
                obj.as_ref().to_glib_none().0,
            ))
        }
    }
}

pub const NONE_GOBJECT_ACCESSIBLE: Option<&GObjectAccessible> = None;

pub trait GObjectAccessibleExt: 'static {
    #[doc(alias = "atk_gobject_accessible_get_object")]
    #[doc(alias = "get_object")]
    fn object(&self) -> Option<glib::Object>;
}

impl<O: IsA<GObjectAccessible>> GObjectAccessibleExt for O {
    fn object(&self) -> Option<glib::Object> {
        unsafe {
            from_glib_none(ffi::atk_gobject_accessible_get_object(
                self.as_ref().to_glib_none().0,
            ))
        }
    }
}

impl fmt::Display for GObjectAccessible {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("GObjectAccessible")
    }
}
