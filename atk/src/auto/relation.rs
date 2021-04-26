// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Object;
use crate::RelationType;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    pub struct Relation(Object<ffi::AtkRelation, ffi::AtkRelationClass>);

    match fn {
        type_ => || ffi::atk_relation_get_type(),
    }
}

impl Relation {
    #[doc(alias = "atk_relation_new")]
    pub fn new(targets: &[Object], relationship: RelationType) -> Relation {
        assert_initialized_main_thread!();
        let n_targets = targets.len() as i32;
        unsafe {
            from_glib_full(ffi::atk_relation_new(
                targets.to_glib_none().0,
                n_targets,
                relationship.into_glib(),
            ))
        }
    }
}

pub const NONE_RELATION: Option<&Relation> = None;

pub trait RelationExt: 'static {
    #[doc(alias = "atk_relation_add_target")]
    fn add_target<P: IsA<Object>>(&self, target: &P);

    #[doc(alias = "atk_relation_get_relation_type")]
    #[doc(alias = "get_relation_type")]
    fn relation_type(&self) -> RelationType;

    #[doc(alias = "atk_relation_get_target")]
    #[doc(alias = "get_target")]
    fn target(&self) -> Vec<Object>;

    #[doc(alias = "atk_relation_remove_target")]
    fn remove_target<P: IsA<Object>>(&self, target: &P) -> bool;

    #[doc(alias = "relation-type")]
    fn set_relation_type(&self, relation_type: RelationType);

    fn set_target(&self, target: Option<&glib::ValueArray>);

    #[doc(alias = "relation-type")]
    fn connect_relation_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "target")]
    fn connect_target_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Relation>> RelationExt for O {
    fn add_target<P: IsA<Object>>(&self, target: &P) {
        unsafe {
            ffi::atk_relation_add_target(
                self.as_ref().to_glib_none().0,
                target.as_ref().to_glib_none().0,
            );
        }
    }

    fn relation_type(&self) -> RelationType {
        unsafe {
            from_glib(ffi::atk_relation_get_relation_type(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn target(&self) -> Vec<Object> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::atk_relation_get_target(
                self.as_ref().to_glib_none().0,
            ))
        }
    }

    fn remove_target<P: IsA<Object>>(&self, target: &P) -> bool {
        unsafe {
            from_glib(ffi::atk_relation_remove_target(
                self.as_ref().to_glib_none().0,
                target.as_ref().to_glib_none().0,
            ))
        }
    }

    fn set_relation_type(&self, relation_type: RelationType) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"relation-type\0".as_ptr() as *const _,
                relation_type.to_value().to_glib_none().0,
            );
        }
    }

    fn set_target(&self, target: Option<&glib::ValueArray>) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"target\0".as_ptr() as *const _,
                target.to_value().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "relation-type")]
    fn connect_relation_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_relation_type_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::AtkRelation,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Relation>,
        {
            let f: &F = &*(f as *const F);
            f(&Relation::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::relation-type\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_relation_type_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[doc(alias = "target")]
    fn connect_target_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_target_trampoline<P, F: Fn(&P) + 'static>(
            this: *mut ffi::AtkRelation,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) where
            P: IsA<Relation>,
        {
            let f: &F = &*(f as *const F);
            f(&Relation::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::target\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_target_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for Relation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Relation")
    }
}
