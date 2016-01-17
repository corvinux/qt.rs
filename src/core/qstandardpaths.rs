// auto generated, do not modify.
// created: Sun Jan 17 17:37:11 2016
// src-file: /QtCore/qstandardpaths.h
// dst-file: /src/core/qstandardpaths.rs
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
use super::qstringlist::QStringList; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QStandardPaths_Class_Size() -> c_int;
  // proto:  void QStandardPaths::QStandardPaths();
  fn _ZN14QStandardPathsC2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QStandardPaths::~QStandardPaths();
  fn _ZN14QStandardPathsD2Ev(qthis: u64 /* *mut c_void*/);
  // proto: static void QStandardPaths::setTestModeEnabled(bool testMode);
  fn _ZN14QStandardPaths18setTestModeEnabledEb(arg0: c_char);
  // proto: static QString QStandardPaths::findExecutable(const QString & executableName, const QStringList & paths);
  fn _ZN14QStandardPaths14findExecutableERK7QStringRK11QStringList(arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto: static void QStandardPaths::enableTestMode(bool testMode);
  fn _ZN14QStandardPaths14enableTestModeEb(arg0: c_char);
  // proto: static bool QStandardPaths::isTestModeEnabled();
  fn _ZN14QStandardPaths17isTestModeEnabledEv() -> c_char;
} // <= ext block end

// body block begin =>
// class sizeof(QStandardPaths)=1
#[derive(Default)]
pub struct QStandardPaths {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QStandardPaths {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QStandardPaths {
    return QStandardPaths{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QStandardPaths::QStandardPaths();
impl /*struct*/ QStandardPaths {
  pub fn new<T: QStandardPaths_new>(value: T) -> QStandardPaths {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QStandardPaths_new {
  fn new(self) -> QStandardPaths;
}

  // proto:  void QStandardPaths::QStandardPaths();
impl<'a> /*trait*/ QStandardPaths_new for () {
  fn new(self) -> QStandardPaths {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QStandardPathsC2Ev()};
    let ctysz: c_int = unsafe{QStandardPaths_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    unsafe {_ZN14QStandardPathsC2Ev(qthis_ph)};
    let qthis: u64 = qthis_ph;
    let rsthis = QStandardPaths{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStandardPaths::~QStandardPaths();
impl /*struct*/ QStandardPaths {
  pub fn free<RetType, T: QStandardPaths_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QStandardPaths_free<RetType> {
  fn free(self , rsthis: & QStandardPaths) -> RetType;
}

  // proto:  void QStandardPaths::~QStandardPaths();
impl<'a> /*trait*/ QStandardPaths_free<()> for () {
  fn free(self , rsthis: & QStandardPaths) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QStandardPathsD2Ev()};
     unsafe {_ZN14QStandardPathsD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto: static void QStandardPaths::setTestModeEnabled(bool testMode);
impl /*struct*/ QStandardPaths {
  pub fn setTestModeEnabled_s<RetType, T: QStandardPaths_setTestModeEnabled_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setTestModeEnabled_s();
    // return 1;
  }
}

pub trait QStandardPaths_setTestModeEnabled_s<RetType> {
  fn setTestModeEnabled_s(self ) -> RetType;
}

  // proto: static void QStandardPaths::setTestModeEnabled(bool testMode);
impl<'a> /*trait*/ QStandardPaths_setTestModeEnabled_s<()> for (i8) {
  fn setTestModeEnabled_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QStandardPaths18setTestModeEnabledEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN14QStandardPaths18setTestModeEnabledEb(arg0)};
    // return 1;
  }
}

  // proto: static QString QStandardPaths::findExecutable(const QString & executableName, const QStringList & paths);
impl /*struct*/ QStandardPaths {
  pub fn findExecutable_s<RetType, T: QStandardPaths_findExecutable_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.findExecutable_s();
    // return 1;
  }
}

pub trait QStandardPaths_findExecutable_s<RetType> {
  fn findExecutable_s(self ) -> RetType;
}

  // proto: static QString QStandardPaths::findExecutable(const QString & executableName, const QStringList & paths);
impl<'a> /*trait*/ QStandardPaths_findExecutable_s<QString> for (&'a QString, &'a QStringList) {
  fn findExecutable_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QStandardPaths14findExecutableERK7QStringRK11QStringList()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN14QStandardPaths14findExecutableERK7QStringRK11QStringList(arg0, arg1)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static void QStandardPaths::enableTestMode(bool testMode);
impl /*struct*/ QStandardPaths {
  pub fn enableTestMode_s<RetType, T: QStandardPaths_enableTestMode_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.enableTestMode_s();
    // return 1;
  }
}

pub trait QStandardPaths_enableTestMode_s<RetType> {
  fn enableTestMode_s(self ) -> RetType;
}

  // proto: static void QStandardPaths::enableTestMode(bool testMode);
impl<'a> /*trait*/ QStandardPaths_enableTestMode_s<()> for (i8) {
  fn enableTestMode_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QStandardPaths14enableTestModeEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN14QStandardPaths14enableTestModeEb(arg0)};
    // return 1;
  }
}

  // proto: static bool QStandardPaths::isTestModeEnabled();
impl /*struct*/ QStandardPaths {
  pub fn isTestModeEnabled_s<RetType, T: QStandardPaths_isTestModeEnabled_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.isTestModeEnabled_s();
    // return 1;
  }
}

pub trait QStandardPaths_isTestModeEnabled_s<RetType> {
  fn isTestModeEnabled_s(self ) -> RetType;
}

  // proto: static bool QStandardPaths::isTestModeEnabled();
impl<'a> /*trait*/ QStandardPaths_isTestModeEnabled_s<i8> for () {
  fn isTestModeEnabled_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QStandardPaths17isTestModeEnabledEv()};
    let mut ret = unsafe {_ZN14QStandardPaths17isTestModeEnabledEv()};
    return ret as i8;
    // return 1;
  }
}

// <= body block end

