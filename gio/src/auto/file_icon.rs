// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::File;
use crate::Icon;
use crate::LoadableIcon;
use glib::object::IsA;
use glib::translate::*;
use std::fmt;

glib::wrapper! {
    pub struct FileIcon(Object<ffi::GFileIcon, ffi::GFileIconClass>) @implements Icon, LoadableIcon;

    match fn {
        type_ => || ffi::g_file_icon_get_type(),
    }
}

impl FileIcon {
    #[doc(alias = "g_file_icon_new")]
    pub fn new<P: IsA<File>>(file: &P) -> FileIcon {
        unsafe { from_glib_full(ffi::g_file_icon_new(file.as_ref().to_glib_none().0)) }
    }

    #[doc(alias = "g_file_icon_get_file")]
    #[doc(alias = "get_file")]
    pub fn file(&self) -> File {
        unsafe { from_glib_none(ffi::g_file_icon_get_file(self.to_glib_none().0)) }
    }
}

impl fmt::Display for FileIcon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("FileIcon")
    }
}
