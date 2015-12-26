// auto generated, do not modify.
// created: Sat Dec 26 12:15:38 2015
// src-file: /QtGui/qrasterwindow.h
// dst-file: /src/gui/qrasterwindow.rs
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
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QRasterWindow_Class_Size() -> c_int;
  // proto:  void QRasterWindow::QRasterWindow(QWindow * parent);
  fn dector_ZN13QRasterWindowC1EP7QWindow(arg0: *mut c_void) -> *mut c_void;
  fn _ZN13QRasterWindowC1EP7QWindow(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  const QMetaObject * QRasterWindow::metaObject();
  fn _ZNK13QRasterWindow10metaObjectEv(qthis: *mut c_void);
  // proto:  void QRasterWindow::QRasterWindow(const QRasterWindow & );
  fn dector_ZN13QRasterWindowC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN13QRasterWindowC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QRasterWindow)=1
pub struct QRasterWindow {
  qbase: QPaintDeviceWindow,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QRasterWindow {
  pub fn inheritFrom(qthis: *mut c_void) -> QRasterWindow {
    return QRasterWindow{qbase: QPaintDeviceWindow::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QRasterWindow {
  type Target = QPaintDeviceWindow;

  fn deref(&self) -> &QPaintDeviceWindow {
    return & self.qbase;
  }
}
impl AsRef<QPaintDeviceWindow> for QRasterWindow {
  fn as_ref(& self) -> & QPaintDeviceWindow {
    return & self.qbase;
  }
}
  // proto:  void QRasterWindow::QRasterWindow(QWindow * parent);
impl /*struct*/ QRasterWindow {
  pub fn New<T: QRasterWindow_New>(value: T) -> QRasterWindow {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QRasterWindow_New {
  fn New(self) -> QRasterWindow;
}

  // proto:  void QRasterWindow::QRasterWindow(QWindow * parent);
impl<'a> /*trait*/ QRasterWindow_New for (&'a QWindow) {
  fn New(self) -> QRasterWindow {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QRasterWindowC1EP7QWindow()};
    let ctysz: c_int = unsafe{QRasterWindow_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN13QRasterWindowC1EP7QWindow(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN13QRasterWindowC1EP7QWindow(arg0)};
    let rsthis = QRasterWindow{/**/qbase: QPaintDeviceWindow::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QMetaObject * QRasterWindow::metaObject();
impl /*struct*/ QRasterWindow {
  pub fn metaObject<RetType, T: QRasterWindow_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QRasterWindow_metaObject<RetType> {
  fn metaObject(self , rsthis: & QRasterWindow) -> RetType;
}

  // proto:  const QMetaObject * QRasterWindow::metaObject();
impl<'a> /*trait*/ QRasterWindow_metaObject<()> for () {
  fn metaObject(self , rsthis: & QRasterWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QRasterWindow10metaObjectEv()};
     unsafe {_ZNK13QRasterWindow10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QRasterWindow::QRasterWindow(const QRasterWindow & );
impl<'a> /*trait*/ QRasterWindow_New for (&'a QRasterWindow) {
  fn New(self) -> QRasterWindow {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QRasterWindowC1ERKS_()};
    let ctysz: c_int = unsafe{QRasterWindow_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN13QRasterWindowC1ERKS_(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN13QRasterWindowC1ERKS_(arg0)};
    let rsthis = QRasterWindow{/**/qbase: QPaintDeviceWindow::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// <= body block end

