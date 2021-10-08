// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::AsyncResult;
use crate::Cancellable;
use crate::IOStream;
use crate::ProxyAddress;
use glib::object::IsA;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::pin::Pin;
use std::ptr;

glib::wrapper! {
    #[doc(alias = "GProxy")]
    pub struct Proxy(Interface<ffi::GProxy, ffi::GProxyInterface>);

    match fn {
        type_ => || ffi::g_proxy_get_type(),
    }
}

impl Proxy {
    #[doc(alias = "g_proxy_get_default_for_protocol")]
    #[doc(alias = "get_default_for_protocol")]
    pub fn default_for_protocol(protocol: &str) -> Option<Proxy> {
        unsafe {
            from_glib_full(ffi::g_proxy_get_default_for_protocol(
                protocol.to_glib_none().0,
            ))
        }
    }
}

pub const NONE_PROXY: Option<&Proxy> = None;

pub trait ProxyExt: 'static {
    #[doc(alias = "g_proxy_connect")]
    fn connect<P: IsA<IOStream>, Q: IsA<ProxyAddress>, R: IsA<Cancellable>>(
        &self,
        connection: &P,
        proxy_address: &Q,
        cancellable: Option<&R>,
    ) -> Result<IOStream, glib::Error>;

    #[doc(alias = "g_proxy_connect_async")]
    fn connect_async<
        P: IsA<IOStream>,
        Q: IsA<ProxyAddress>,
        R: IsA<Cancellable>,
        S: FnOnce(Result<IOStream, glib::Error>) + Send + 'static,
    >(
        &self,
        connection: &P,
        proxy_address: &Q,
        cancellable: Option<&R>,
        callback: S,
    );

    fn connect_async_future<
        P: IsA<IOStream> + Clone + 'static,
        Q: IsA<ProxyAddress> + Clone + 'static,
    >(
        &self,
        connection: &P,
        proxy_address: &Q,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<IOStream, glib::Error>> + 'static>>;

    #[doc(alias = "g_proxy_supports_hostname")]
    fn supports_hostname(&self) -> bool;
}

impl<O: IsA<Proxy>> ProxyExt for O {
    fn connect<P: IsA<IOStream>, Q: IsA<ProxyAddress>, R: IsA<Cancellable>>(
        &self,
        connection: &P,
        proxy_address: &Q,
        cancellable: Option<&R>,
    ) -> Result<IOStream, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_proxy_connect(
                self.as_ref().to_glib_none().0,
                connection.as_ref().to_glib_none().0,
                proxy_address.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn connect_async<
        P: IsA<IOStream>,
        Q: IsA<ProxyAddress>,
        R: IsA<Cancellable>,
        S: FnOnce(Result<IOStream, glib::Error>) + Send + 'static,
    >(
        &self,
        connection: &P,
        proxy_address: &Q,
        cancellable: Option<&R>,
        callback: S,
    ) {
        let user_data: Box_<S> = Box_::new(callback);
        unsafe extern "C" fn connect_async_trampoline<
            S: FnOnce(Result<IOStream, glib::Error>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret = ffi::g_proxy_connect_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<S> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = connect_async_trampoline::<S>;
        unsafe {
            ffi::g_proxy_connect_async(
                self.as_ref().to_glib_none().0,
                connection.as_ref().to_glib_none().0,
                proxy_address.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn connect_async_future<
        P: IsA<IOStream> + Clone + 'static,
        Q: IsA<ProxyAddress> + Clone + 'static,
    >(
        &self,
        connection: &P,
        proxy_address: &Q,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<IOStream, glib::Error>> + 'static>> {
        let connection = connection.clone();
        let proxy_address = proxy_address.clone();
        Box_::pin(crate::GioFuture::new(
            self,
            move |obj, cancellable, send| {
                obj.connect_async(&connection, &proxy_address, Some(cancellable), move |res| {
                    send.resolve(res);
                });
            },
        ))
    }

    fn supports_hostname(&self) -> bool {
        unsafe {
            from_glib(ffi::g_proxy_supports_hostname(
                self.as_ref().to_glib_none().0,
            ))
        }
    }
}

impl fmt::Display for Proxy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Proxy")
    }
}
