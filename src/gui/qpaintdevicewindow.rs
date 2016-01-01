// auto generated, do not modify.
// created: Fri Jan  1 15:54:32 2016
// src-file: /QtGui/qpaintdevicewindow.h
// dst-file: /src/gui/qpaintdevicewindow.rs
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
use super::qwindow::QWindow; // 773
use std::ops::Deref;
use super::qregion::QRegion; // 773
use super::super::core::qrect::QRect; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QPaintDeviceWindow_Class_Size() -> c_int;
  // proto:  void QPaintDeviceWindow::update(const QRegion & region);
  fn _ZN18QPaintDeviceWindow6updateERK7QRegion(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QPaintDeviceWindow::update();
  fn _ZN18QPaintDeviceWindow6updateEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QPaintDeviceWindow::QPaintDeviceWindow(const QPaintDeviceWindow & );
  fn dector_ZN18QPaintDeviceWindowC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN18QPaintDeviceWindowC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  const QMetaObject * QPaintDeviceWindow::metaObject();
  fn _ZNK18QPaintDeviceWindow10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QPaintDeviceWindow::update(const QRect & rect);
  fn _ZN18QPaintDeviceWindow6updateERK5QRect(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QPaintDeviceWindow)=1
#[derive(Default)]
pub struct QPaintDeviceWindow {
  qbase: QWindow,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QPaintDeviceWindow {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QPaintDeviceWindow {
    return QPaintDeviceWindow{qbase: QWindow::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QPaintDeviceWindow {
  type Target = QWindow;

  fn deref(&self) -> &QWindow {
    return & self.qbase;
  }
}
impl AsRef<QWindow> for QPaintDeviceWindow {
  fn as_ref(& self) -> & QWindow {
    return & self.qbase;
  }
}
  // proto:  void QPaintDeviceWindow::update(const QRegion & region);
impl /*struct*/ QPaintDeviceWindow {
  pub fn update<RetType, T: QPaintDeviceWindow_update<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.update(self);
    // return 1;
  }
}

pub trait QPaintDeviceWindow_update<RetType> {
  fn update(self , rsthis: & QPaintDeviceWindow) -> RetType;
}

  // proto:  void QPaintDeviceWindow::update(const QRegion & region);
impl<'a> /*trait*/ QPaintDeviceWindow_update<()> for (&'a QRegion) {
  fn update(self , rsthis: & QPaintDeviceWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QPaintDeviceWindow6updateERK7QRegion()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN18QPaintDeviceWindow6updateERK7QRegion(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPaintDeviceWindow::update();
impl<'a> /*trait*/ QPaintDeviceWindow_update<()> for () {
  fn update(self , rsthis: & QPaintDeviceWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QPaintDeviceWindow6updateEv()};
     unsafe {_ZN18QPaintDeviceWindow6updateEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QPaintDeviceWindow::QPaintDeviceWindow(const QPaintDeviceWindow & );
impl /*struct*/ QPaintDeviceWindow {
  pub fn new<T: QPaintDeviceWindow_new>(value: T) -> QPaintDeviceWindow {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QPaintDeviceWindow_new {
  fn new(self) -> QPaintDeviceWindow;
}

  // proto:  void QPaintDeviceWindow::QPaintDeviceWindow(const QPaintDeviceWindow & );
impl<'a> /*trait*/ QPaintDeviceWindow_new for (&'a QPaintDeviceWindow) {
  fn new(self) -> QPaintDeviceWindow {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QPaintDeviceWindowC1ERKS_()};
    let ctysz: c_int = unsafe{QPaintDeviceWindow_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN18QPaintDeviceWindowC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN18QPaintDeviceWindowC1ERKS_(arg0)} as u64;
    let rsthis = QPaintDeviceWindow{qbase: QWindow::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QMetaObject * QPaintDeviceWindow::metaObject();
impl /*struct*/ QPaintDeviceWindow {
  pub fn metaObject<RetType, T: QPaintDeviceWindow_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QPaintDeviceWindow_metaObject<RetType> {
  fn metaObject(self , rsthis: & QPaintDeviceWindow) -> RetType;
}

  // proto:  const QMetaObject * QPaintDeviceWindow::metaObject();
impl<'a> /*trait*/ QPaintDeviceWindow_metaObject<()> for () {
  fn metaObject(self , rsthis: & QPaintDeviceWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QPaintDeviceWindow10metaObjectEv()};
     unsafe {_ZNK18QPaintDeviceWindow10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QPaintDeviceWindow::update(const QRect & rect);
impl<'a> /*trait*/ QPaintDeviceWindow_update<()> for (&'a QRect) {
  fn update(self , rsthis: & QPaintDeviceWindow) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QPaintDeviceWindow6updateERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN18QPaintDeviceWindow6updateERK5QRect(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// <= body block end

