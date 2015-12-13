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
  // proto: void QDesktopServices::unsetUrlHandler(const QString & scheme);
  fn _ZN16QDesktopServices15unsetUrlHandlerERK7QString(arg0: *const c_void) -> i32;
  // proto: bool QDesktopServices::openUrl(const QUrl & url);
  fn _ZN16QDesktopServices7openUrlERK4QUrl(arg0: *const c_void) -> i32;
  // proto: void QDesktopServices::setUrlHandler(const QString & scheme, QObject * receiver, const char * method);
  fn _ZN16QDesktopServices13setUrlHandlerERK7QStringP7QObjectPKc(arg0: *const c_void, arg1: *mut c_void, arg2: *const c_char) -> i32;
}

// body block begin
// class sizeof(QDesktopServices)=1
pub struct QDesktopServices {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QDesktopServices {
  pub fn unsetUrlHandler<T: QDesktopServices_unsetUrlHandler>(&mut self, value: T) -> i32 {
    value.unsetUrlHandler(self);
    return 1;
  }
}

pub trait QDesktopServices_unsetUrlHandler {
  fn unsetUrlHandler(self, this: &mut QDesktopServices) -> i32;
}

// proto: void QDesktopServices::unsetUrlHandler(const QString & scheme);
impl<'a> /*trait*/ QDesktopServices_unsetUrlHandler for (&'a  QString) {
  fn unsetUrlHandler(self, this: &mut QDesktopServices) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDesktopServices15unsetUrlHandlerERK7QString()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN16QDesktopServices15unsetUrlHandlerERK7QString(arg0)};
    return 1;
  }
}

impl /*struct*/ QDesktopServices {
  pub fn openUrl<T: QDesktopServices_openUrl>(&mut self, value: T) -> i32 {
    value.openUrl(self);
    return 1;
  }
}

pub trait QDesktopServices_openUrl {
  fn openUrl(self, this: &mut QDesktopServices) -> i32;
}

// proto: bool QDesktopServices::openUrl(const QUrl & url);
impl<'a> /*trait*/ QDesktopServices_openUrl for (&'a  QUrl) {
  fn openUrl(self, this: &mut QDesktopServices) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDesktopServices7openUrlERK4QUrl()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN16QDesktopServices7openUrlERK4QUrl(arg0)};
    return 1;
  }
}

impl /*struct*/ QDesktopServices {
  pub fn setUrlHandler<T: QDesktopServices_setUrlHandler>(&mut self, value: T) -> i32 {
    value.setUrlHandler(self);
    return 1;
  }
}

pub trait QDesktopServices_setUrlHandler {
  fn setUrlHandler(self, this: &mut QDesktopServices) -> i32;
}

// proto: void QDesktopServices::setUrlHandler(const QString & scheme, QObject * receiver, const char * method);
impl<'a> /*trait*/ QDesktopServices_setUrlHandler for (&'a  QString, &'a mut QObject, &'a  String) {
  fn setUrlHandler(self, this: &mut QDesktopServices) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QDesktopServices13setUrlHandlerERK7QStringP7QObjectPKc()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.as_ptr()  as *const c_char;
    unsafe {_ZN16QDesktopServices13setUrlHandlerERK7QStringP7QObjectPKc(arg0, arg1, arg2)};
    return 1;
  }
}

