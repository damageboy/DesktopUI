// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Buildable;
use crate::Widget;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::ToValue;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "GtkCellEditable")]
    pub struct CellEditable(Interface<ffi::GtkCellEditable, ffi::GtkCellEditableIface>) @requires Widget, Buildable;

    match fn {
        type_ => || ffi::gtk_cell_editable_get_type(),
    }
}

pub const NONE_CELL_EDITABLE: Option<&CellEditable> = None;

pub trait CellEditableExt: 'static {
    #[doc(alias = "gtk_cell_editable_editing_done")]
    fn editing_done(&self);

    #[doc(alias = "gtk_cell_editable_remove_widget")]
    fn remove_widget(&self);

    #[doc(alias = "gtk_cell_editable_start_editing")]
    fn start_editing(&self, event: Option<&gdk::Event>);

    #[doc(alias = "editing-canceled")]
    fn is_editing_canceled(&self) -> bool;

    #[doc(alias = "editing-canceled")]
    fn set_editing_canceled(&self, editing_canceled: bool);

    #[doc(alias = "editing-done")]
    fn connect_editing_done<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "remove-widget")]
    fn connect_remove_widget<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "editing-canceled")]
    fn connect_editing_canceled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<CellEditable>> CellEditableExt for O {
    fn editing_done(&self) {
        unsafe {
            ffi::gtk_cell_editable_editing_done(self.as_ref().to_glib_none().0);
        }
    }

    fn remove_widget(&self) {
        unsafe {
            ffi::gtk_cell_editable_remove_widget(self.as_ref().to_glib_none().0);
        }
    }

    fn start_editing(&self, event: Option<&gdk::Event>) {
        unsafe {
            ffi::gtk_cell_editable_start_editing(
                self.as_ref().to_glib_none().0,
                mut_override(event.to_glib_none().0),
            );
        }
    }

    fn is_editing_canceled(&self) -> bool {
        unsafe {
            let mut value = glib::Value::from_type(<bool as StaticType>::static_type());
            glib::gobject_ffi::g_object_get_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"editing-canceled\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `editing-canceled` getter")
        }
    }

    fn set_editing_canceled(&self, editing_canceled: bool) {
        unsafe {
            glib::gobject_ffi::g_object_set_property(
                self.to_glib_none().0 as *mut glib::gobject_ffi::GObject,
                b"editing-canceled\0".as_ptr() as *const _,
                editing_canceled.to_value().to_glib_none().0,
            );
        }
    }

    fn connect_editing_done<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn editing_done_trampoline<P: IsA<CellEditable>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkCellEditable,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(CellEditable::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"editing-done\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    editing_done_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_remove_widget<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn remove_widget_trampoline<P: IsA<CellEditable>, F: Fn(&P) + 'static>(
            this: *mut ffi::GtkCellEditable,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(CellEditable::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"remove-widget\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    remove_widget_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    fn connect_editing_canceled_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_editing_canceled_trampoline<
            P: IsA<CellEditable>,
            F: Fn(&P) + 'static,
        >(
            this: *mut ffi::GtkCellEditable,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(CellEditable::from_glib_borrow(this).unsafe_cast_ref())
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::editing-canceled\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_editing_canceled_trampoline::<Self, F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for CellEditable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("CellEditable")
    }
}
