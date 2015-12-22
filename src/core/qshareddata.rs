// auto generated, do not modify.
// created: Tue Dec 22 23:21:28 2015
// src-file: /QtCore/qshareddata.h
// dst-file: /src/core/qshareddata.rs
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
use std::ops::Deref;
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  void QSharedData::QSharedData();
  fn _ZN11QSharedDataC1Ev(qthis: *mut c_void);
  // proto:  void QSharedData::QSharedData(const QSharedData & );
  fn _ZN11QSharedDataC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QSharedData)=1
pub struct QSharedData {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QSharedData {
  pub fn inheritFrom(qthis: *mut c_void) -> QSharedData {
    return QSharedData{qclsinst: qthis};
  }
}
  // proto:  void QSharedData::QSharedData();
impl /*struct*/ QSharedData {
  pub fn NewQSharedData<T: QSharedData_NewQSharedData>(value: T) -> QSharedData {
    let rsthis = value.NewQSharedData();
    return rsthis;
    // return 1;
  }
}

pub trait QSharedData_NewQSharedData {
  fn NewQSharedData(self) -> QSharedData;
}

  // proto:  void QSharedData::QSharedData();
impl<'a> /*trait*/ QSharedData_NewQSharedData for () {
  fn NewQSharedData(self) -> QSharedData {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QSharedDataC1Ev()};
    unsafe {_ZN11QSharedDataC1Ev(qthis)};
    let rsthis = QSharedData{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QSharedData::QSharedData(const QSharedData & );
impl<'a> /*trait*/ QSharedData_NewQSharedData for (QSharedData) {
  fn NewQSharedData(self) -> QSharedData {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QSharedDataC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QSharedDataC1ERKS_(qthis, arg0)};
    let rsthis = QSharedData{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// <= body block end

