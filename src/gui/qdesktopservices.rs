// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;
use super::qurl::QUrl;
use super::qobject::QObject;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: static void QDesktopServices::unsetUrlHandler(const QString & scheme);
  fn _ZN16QDesktopServices15unsetUrlHandlerERK7QString(arg0: *mut c_void);
  // proto: static bool QDesktopServices::openUrl(const QUrl & url);
  fn _ZN16QDesktopServices7openUrlERK4QUrl(arg0: *mut c_void) -> c_char;
  // proto: static void QDesktopServices::setUrlHandler(const QString & scheme, QObject * receiver, const char * method);
  fn _ZN16QDesktopServices13setUrlHandlerERK7QStringP7QObjectPKc(arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_char);
}

// body block begin
// class sizeof(QDesktopServices)=1
pub struct QDesktopServices {
  pub qclsinst: *mut c_void,
}

  // proto: static void QDesktopServices::unsetUrlHandler(const QString & scheme);
impl /*struct*/ QDesktopServices {
  pub fn unsetUrlHandler_s<RetType, T: QDesktopServices_unsetUrlHandler_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.unsetUrlHandler_s();
    // return 1;
  }
}

pub trait QDesktopServices_unsetUrlHandler_s<RetType> {
  fn unsetUrlHandler_s(self ) -> RetType;
}

  // proto: static void QDesktopServices::unsetUrlHandler(const QString & scheme);
impl<'a> /*trait*/ QDesktopServices_unsetUrlHandler_s<()> for (QString) {
  fn unsetUrlHandler_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDesktopServices15unsetUrlHandlerERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QDesktopServices15unsetUrlHandlerERK7QString(arg0)};
    // return 1;
  }
}

  // proto: static bool QDesktopServices::openUrl(const QUrl & url);
impl /*struct*/ QDesktopServices {
  pub fn openUrl_s<RetType, T: QDesktopServices_openUrl_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.openUrl_s();
    // return 1;
  }
}

pub trait QDesktopServices_openUrl_s<RetType> {
  fn openUrl_s(self ) -> RetType;
}

  // proto: static bool QDesktopServices::openUrl(const QUrl & url);
impl<'a> /*trait*/ QDesktopServices_openUrl_s<i8> for (QUrl) {
  fn openUrl_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDesktopServices7openUrlERK4QUrl()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN16QDesktopServices7openUrlERK4QUrl(arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto: static void QDesktopServices::setUrlHandler(const QString & scheme, QObject * receiver, const char * method);
impl /*struct*/ QDesktopServices {
  pub fn setUrlHandler_s<RetType, T: QDesktopServices_setUrlHandler_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setUrlHandler_s();
    // return 1;
  }
}

pub trait QDesktopServices_setUrlHandler_s<RetType> {
  fn setUrlHandler_s(self ) -> RetType;
}

  // proto: static void QDesktopServices::setUrlHandler(const QString & scheme, QObject * receiver, const char * method);
impl<'a> /*trait*/ QDesktopServices_setUrlHandler_s<()> for (QString, QObject, &'a  String) {
  fn setUrlHandler_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDesktopServices13setUrlHandlerERK7QStringP7QObjectPKc()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.as_ptr()  as *mut c_char;
     unsafe {_ZN16QDesktopServices13setUrlHandlerERK7QStringP7QObjectPKc(arg0, arg1, arg2)};
    // return 1;
  }
}
