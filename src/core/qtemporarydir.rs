// auto generated, do not modify.
// created: Sat Dec 26 10:52:38 2015
// src-file: /QtCore/qtemporarydir.h
// dst-file: /src/core/qtemporarydir.rs
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
use super::qstring::QString; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QTemporaryDir_Class_Size() -> c_int;
  // proto:  bool QTemporaryDir::remove();
  fn _ZN13QTemporaryDir6removeEv(qthis: *mut c_void) -> c_char;
  // proto:  bool QTemporaryDir::autoRemove();
  fn _ZNK13QTemporaryDir10autoRemoveEv(qthis: *mut c_void) -> c_char;
  // proto:  bool QTemporaryDir::isValid();
  fn _ZNK13QTemporaryDir7isValidEv(qthis: *mut c_void) -> c_char;
  // proto:  void QTemporaryDir::setAutoRemove(bool b);
  fn _ZN13QTemporaryDir13setAutoRemoveEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QTemporaryDir::~QTemporaryDir();
  fn _ZN13QTemporaryDirD0Ev(qthis: *mut c_void);
  // proto:  void QTemporaryDir::QTemporaryDir();
  fn dector_ZN13QTemporaryDirC1Ev() -> *mut c_void;
  fn _ZN13QTemporaryDirC1Ev(qthis: *mut c_void);
  // proto:  void QTemporaryDir::QTemporaryDir(const QString & templateName);
  fn dector_ZN13QTemporaryDirC1ERK7QString(arg0: *mut c_void) -> *mut c_void;
  fn _ZN13QTemporaryDirC1ERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QString QTemporaryDir::path();
  fn _ZNK13QTemporaryDir4pathEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTemporaryDir::QTemporaryDir(const QTemporaryDir & );
  fn dector_ZN13QTemporaryDirC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN13QTemporaryDirC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QTemporaryDir)=1
pub struct QTemporaryDir {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTemporaryDir {
  pub fn inheritFrom(qthis: *mut c_void) -> QTemporaryDir {
    return QTemporaryDir{qclsinst: qthis};
  }
}
  // proto:  bool QTemporaryDir::remove();
impl /*struct*/ QTemporaryDir {
  pub fn remove<RetType, T: QTemporaryDir_remove<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.remove(self);
    // return 1;
  }
}

pub trait QTemporaryDir_remove<RetType> {
  fn remove(self , rsthis: & QTemporaryDir) -> RetType;
}

  // proto:  bool QTemporaryDir::remove();
impl<'a> /*trait*/ QTemporaryDir_remove<i8> for () {
  fn remove(self , rsthis: & QTemporaryDir) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTemporaryDir6removeEv()};
    let mut ret = unsafe {_ZN13QTemporaryDir6removeEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QTemporaryDir::autoRemove();
impl /*struct*/ QTemporaryDir {
  pub fn autoRemove<RetType, T: QTemporaryDir_autoRemove<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.autoRemove(self);
    // return 1;
  }
}

pub trait QTemporaryDir_autoRemove<RetType> {
  fn autoRemove(self , rsthis: & QTemporaryDir) -> RetType;
}

  // proto:  bool QTemporaryDir::autoRemove();
impl<'a> /*trait*/ QTemporaryDir_autoRemove<i8> for () {
  fn autoRemove(self , rsthis: & QTemporaryDir) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTemporaryDir10autoRemoveEv()};
    let mut ret = unsafe {_ZNK13QTemporaryDir10autoRemoveEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QTemporaryDir::isValid();
impl /*struct*/ QTemporaryDir {
  pub fn isValid<RetType, T: QTemporaryDir_isValid<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QTemporaryDir_isValid<RetType> {
  fn isValid(self , rsthis: & QTemporaryDir) -> RetType;
}

  // proto:  bool QTemporaryDir::isValid();
impl<'a> /*trait*/ QTemporaryDir_isValid<i8> for () {
  fn isValid(self , rsthis: & QTemporaryDir) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTemporaryDir7isValidEv()};
    let mut ret = unsafe {_ZNK13QTemporaryDir7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QTemporaryDir::setAutoRemove(bool b);
impl /*struct*/ QTemporaryDir {
  pub fn setAutoRemove<RetType, T: QTemporaryDir_setAutoRemove<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setAutoRemove(self);
    // return 1;
  }
}

pub trait QTemporaryDir_setAutoRemove<RetType> {
  fn setAutoRemove(self , rsthis: & QTemporaryDir) -> RetType;
}

  // proto:  void QTemporaryDir::setAutoRemove(bool b);
impl<'a> /*trait*/ QTemporaryDir_setAutoRemove<()> for (i8) {
  fn setAutoRemove(self , rsthis: & QTemporaryDir) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTemporaryDir13setAutoRemoveEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN13QTemporaryDir13setAutoRemoveEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTemporaryDir::~QTemporaryDir();
impl /*struct*/ QTemporaryDir {
  pub fn Free<RetType, T: QTemporaryDir_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QTemporaryDir_Free<RetType> {
  fn Free(self , rsthis: & QTemporaryDir) -> RetType;
}

  // proto:  void QTemporaryDir::~QTemporaryDir();
impl<'a> /*trait*/ QTemporaryDir_Free<()> for () {
  fn Free(self , rsthis: & QTemporaryDir) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTemporaryDirD0Ev()};
     unsafe {_ZN13QTemporaryDirD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTemporaryDir::QTemporaryDir();
impl /*struct*/ QTemporaryDir {
  pub fn New<T: QTemporaryDir_New>(value: T) -> QTemporaryDir {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QTemporaryDir_New {
  fn New(self) -> QTemporaryDir;
}

  // proto:  void QTemporaryDir::QTemporaryDir();
impl<'a> /*trait*/ QTemporaryDir_New for () {
  fn New(self) -> QTemporaryDir {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTemporaryDirC1Ev()};
    let ctysz: c_int = unsafe{QTemporaryDir_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    // unsafe {_ZN13QTemporaryDirC1Ev(qthis)};
    let qthis: *mut c_void = unsafe {dector_ZN13QTemporaryDirC1Ev()};
    let rsthis = QTemporaryDir{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QTemporaryDir::QTemporaryDir(const QString & templateName);
impl<'a> /*trait*/ QTemporaryDir_New for (&'a QString) {
  fn New(self) -> QTemporaryDir {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTemporaryDirC1ERK7QString()};
    let ctysz: c_int = unsafe{QTemporaryDir_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN13QTemporaryDirC1ERK7QString(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN13QTemporaryDirC1ERK7QString(arg0)};
    let rsthis = QTemporaryDir{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QString QTemporaryDir::path();
impl /*struct*/ QTemporaryDir {
  pub fn path<RetType, T: QTemporaryDir_path<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.path(self);
    // return 1;
  }
}

pub trait QTemporaryDir_path<RetType> {
  fn path(self , rsthis: & QTemporaryDir) -> RetType;
}

  // proto:  QString QTemporaryDir::path();
impl<'a> /*trait*/ QTemporaryDir_path<QString> for () {
  fn path(self , rsthis: & QTemporaryDir) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTemporaryDir4pathEv()};
    let mut ret = unsafe {_ZNK13QTemporaryDir4pathEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTemporaryDir::QTemporaryDir(const QTemporaryDir & );
impl<'a> /*trait*/ QTemporaryDir_New for (&'a QTemporaryDir) {
  fn New(self) -> QTemporaryDir {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTemporaryDirC1ERKS_()};
    let ctysz: c_int = unsafe{QTemporaryDir_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN13QTemporaryDirC1ERKS_(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN13QTemporaryDirC1ERKS_(arg0)};
    let rsthis = QTemporaryDir{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// <= body block end

