// auto generated, do not modify.
// created: Thu Dec 24 23:00:39 2015
// src-file: /QtGui/qopenglwindow.h
// dst-file: /src/gui/qopenglwindow.rs
//

// header block begin =>
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;


// <= header block end

// main block begin =>
// <= main block end

// use block begin =>
use super::qpaintdevicewindow::QPaintDeviceWindow; // 773
use std::ops::Deref;
use super::qwindow::QWindow; // 773
use super::qimage::QImage; // 773
use super::qopenglcontext::QOpenGLContext; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]

// #[link(name = "QtInline")]

extern {
  // proto:  bool QOpenGLWindow::isValid();
  fn _ZNK13QOpenGLWindow7isValidEv(qthis: *mut c_void) -> c_char;
  // proto:  QImage QOpenGLWindow::grabFramebuffer();
  fn _ZN13QOpenGLWindow15grabFramebufferEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QOpenGLWindow::QOpenGLWindow(const QOpenGLWindow & );
  fn _ZN13QOpenGLWindowC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QOpenGLWindow::frameSwapped();
  fn _ZN13QOpenGLWindow12frameSwappedEv(qthis: *mut c_void);
  // proto:  QOpenGLContext * QOpenGLWindow::shareContext();
  fn _ZNK13QOpenGLWindow12shareContextEv(qthis: *mut c_void);
  // proto:  void QOpenGLWindow::makeCurrent();
  fn _ZN13QOpenGLWindow11makeCurrentEv(qthis: *mut c_void);
  // proto:  QOpenGLContext * QOpenGLWindow::context();
  fn _ZNK13QOpenGLWindow7contextEv(qthis: *mut c_void);
  // proto:  void QOpenGLWindow::doneCurrent();
  fn _ZN13QOpenGLWindow11doneCurrentEv(qthis: *mut c_void);
  // proto:  GLuint QOpenGLWindow::defaultFramebufferObject();
  fn _ZNK13QOpenGLWindow24defaultFramebufferObjectEv(qthis: *mut c_void);
  // proto:  void QOpenGLWindow::~QOpenGLWindow();
  fn _ZN13QOpenGLWindowD0Ev(qthis: *mut c_void);
  // proto:  const QMetaObject * QOpenGLWindow::metaObject();
  fn _ZNK13QOpenGLWindow10metaObjectEv(qthis: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QOpenGLWindow)=1
pub struct QOpenGLWindow {
  qbase: QPaintDeviceWindow,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QOpenGLWindow {
  pub fn inheritFrom(qthis: *mut c_void) -> QOpenGLWindow {
    return QOpenGLWindow{qbase: QPaintDeviceWindow::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QOpenGLWindow {
  type Target = QPaintDeviceWindow;

  fn deref(&self) -> &QPaintDeviceWindow {
    return & self.qbase;
  }
}
impl AsRef<QPaintDeviceWindow> for QOpenGLWindow {
  fn as_ref(& self) -> & QPaintDeviceWindow {
    return & self.qbase;
  }
}
  // proto:  bool QOpenGLWindow::isValid();
impl /*struct*/ QOpenGLWindow {
  pub fn isValid<RetType, T: QOpenGLWindow_isValid<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QOpenGLWindow_isValid<RetType> {
  fn isValid(self , rsthis: & QOpenGLWindow) -> RetType;
}

  // proto:  bool QOpenGLWindow::isValid();
impl<'a> /*trait*/ QOpenGLWindow_isValid<i8> for () {
  fn isValid(self , rsthis: & QOpenGLWindow) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QOpenGLWindow7isValidEv()};
    let mut ret = unsafe {_ZNK13QOpenGLWindow7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QImage QOpenGLWindow::grabFramebuffer();
impl /*struct*/ QOpenGLWindow {
  pub fn grabFramebuffer<RetType, T: QOpenGLWindow_grabFramebuffer<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.grabFramebuffer(self);
    // return 1;
  }
}

pub trait QOpenGLWindow_grabFramebuffer<RetType> {
  fn grabFramebuffer(self , rsthis: & QOpenGLWindow) -> RetType;
}

  // proto:  QImage QOpenGLWindow::grabFramebuffer();
impl<'a> /*trait*/ QOpenGLWindow_grabFramebuffer<QImage> for () {
  fn grabFramebuffer(self , rsthis: & QOpenGLWindow) -> QImage {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLWindow15grabFramebufferEv()};
    let mut ret = unsafe {_ZN13QOpenGLWindow15grabFramebufferEv(rsthis.qclsinst)};
    let mut ret1 = QImage::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QOpenGLWindow::QOpenGLWindow(const QOpenGLWindow & );
impl /*struct*/ QOpenGLWindow {
  pub fn New<T: QOpenGLWindow_New>(value: T) -> QOpenGLWindow {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLWindow_New {
  fn New(self) -> QOpenGLWindow;
}

  // proto:  void QOpenGLWindow::QOpenGLWindow(const QOpenGLWindow & );
impl<'a> /*trait*/ QOpenGLWindow_New for (&'a QOpenGLWindow) {
  fn New(self) -> QOpenGLWindow {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLWindowC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QOpenGLWindowC1ERKS_(qthis, arg0)};
    let rsthis = QOpenGLWindow{/**/qbase: QPaintDeviceWindow::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QOpenGLWindow::frameSwapped();
impl /*struct*/ QOpenGLWindow {
  pub fn frameSwapped<RetType, T: QOpenGLWindow_frameSwapped<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.frameSwapped(self);
    // return 1;
  }
}

pub trait QOpenGLWindow_frameSwapped<RetType> {
  fn frameSwapped(self , rsthis: & QOpenGLWindow) -> RetType;
}

  // proto:  void QOpenGLWindow::frameSwapped();
impl<'a> /*trait*/ QOpenGLWindow_frameSwapped<()> for () {
  fn frameSwapped(self , rsthis: & QOpenGLWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLWindow12frameSwappedEv()};
     unsafe {_ZN13QOpenGLWindow12frameSwappedEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QOpenGLContext * QOpenGLWindow::shareContext();
impl /*struct*/ QOpenGLWindow {
  pub fn shareContext<RetType, T: QOpenGLWindow_shareContext<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.shareContext(self);
    // return 1;
  }
}

pub trait QOpenGLWindow_shareContext<RetType> {
  fn shareContext(self , rsthis: & QOpenGLWindow) -> RetType;
}

  // proto:  QOpenGLContext * QOpenGLWindow::shareContext();
impl<'a> /*trait*/ QOpenGLWindow_shareContext<()> for () {
  fn shareContext(self , rsthis: & QOpenGLWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QOpenGLWindow12shareContextEv()};
     unsafe {_ZNK13QOpenGLWindow12shareContextEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QOpenGLWindow::makeCurrent();
impl /*struct*/ QOpenGLWindow {
  pub fn makeCurrent<RetType, T: QOpenGLWindow_makeCurrent<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.makeCurrent(self);
    // return 1;
  }
}

pub trait QOpenGLWindow_makeCurrent<RetType> {
  fn makeCurrent(self , rsthis: & QOpenGLWindow) -> RetType;
}

  // proto:  void QOpenGLWindow::makeCurrent();
impl<'a> /*trait*/ QOpenGLWindow_makeCurrent<()> for () {
  fn makeCurrent(self , rsthis: & QOpenGLWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLWindow11makeCurrentEv()};
     unsafe {_ZN13QOpenGLWindow11makeCurrentEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QOpenGLContext * QOpenGLWindow::context();
impl /*struct*/ QOpenGLWindow {
  pub fn context<RetType, T: QOpenGLWindow_context<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.context(self);
    // return 1;
  }
}

pub trait QOpenGLWindow_context<RetType> {
  fn context(self , rsthis: & QOpenGLWindow) -> RetType;
}

  // proto:  QOpenGLContext * QOpenGLWindow::context();
impl<'a> /*trait*/ QOpenGLWindow_context<()> for () {
  fn context(self , rsthis: & QOpenGLWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QOpenGLWindow7contextEv()};
     unsafe {_ZNK13QOpenGLWindow7contextEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QOpenGLWindow::doneCurrent();
impl /*struct*/ QOpenGLWindow {
  pub fn doneCurrent<RetType, T: QOpenGLWindow_doneCurrent<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.doneCurrent(self);
    // return 1;
  }
}

pub trait QOpenGLWindow_doneCurrent<RetType> {
  fn doneCurrent(self , rsthis: & QOpenGLWindow) -> RetType;
}

  // proto:  void QOpenGLWindow::doneCurrent();
impl<'a> /*trait*/ QOpenGLWindow_doneCurrent<()> for () {
  fn doneCurrent(self , rsthis: & QOpenGLWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLWindow11doneCurrentEv()};
     unsafe {_ZN13QOpenGLWindow11doneCurrentEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  GLuint QOpenGLWindow::defaultFramebufferObject();
impl /*struct*/ QOpenGLWindow {
  pub fn defaultFramebufferObject<RetType, T: QOpenGLWindow_defaultFramebufferObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.defaultFramebufferObject(self);
    // return 1;
  }
}

pub trait QOpenGLWindow_defaultFramebufferObject<RetType> {
  fn defaultFramebufferObject(self , rsthis: & QOpenGLWindow) -> RetType;
}

  // proto:  GLuint QOpenGLWindow::defaultFramebufferObject();
impl<'a> /*trait*/ QOpenGLWindow_defaultFramebufferObject<()> for () {
  fn defaultFramebufferObject(self , rsthis: & QOpenGLWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QOpenGLWindow24defaultFramebufferObjectEv()};
     unsafe {_ZNK13QOpenGLWindow24defaultFramebufferObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QOpenGLWindow::~QOpenGLWindow();
impl /*struct*/ QOpenGLWindow {
  pub fn Free<RetType, T: QOpenGLWindow_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QOpenGLWindow_Free<RetType> {
  fn Free(self , rsthis: & QOpenGLWindow) -> RetType;
}

  // proto:  void QOpenGLWindow::~QOpenGLWindow();
impl<'a> /*trait*/ QOpenGLWindow_Free<()> for () {
  fn Free(self , rsthis: & QOpenGLWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QOpenGLWindowD0Ev()};
     unsafe {_ZN13QOpenGLWindowD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QOpenGLWindow::metaObject();
impl /*struct*/ QOpenGLWindow {
  pub fn metaObject<RetType, T: QOpenGLWindow_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QOpenGLWindow_metaObject<RetType> {
  fn metaObject(self , rsthis: & QOpenGLWindow) -> RetType;
}

  // proto:  const QMetaObject * QOpenGLWindow::metaObject();
impl<'a> /*trait*/ QOpenGLWindow_metaObject<()> for () {
  fn metaObject(self , rsthis: & QOpenGLWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QOpenGLWindow10metaObjectEv()};
     unsafe {_ZNK13QOpenGLWindow10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

