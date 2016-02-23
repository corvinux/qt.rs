// auto generated, do not modify.
// created: Mon Feb 22 23:57:02 2016
// src-file: /QtCore/qsystemsemaphore.h
// dst-file: /src/core/qsystemsemaphore.rs
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
use super::qstring::*; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QSystemSemaphore_Class_Size() -> c_int;
  // proto:  QString QSystemSemaphore::key();
  fn C_ZNK16QSystemSemaphore3keyEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QSystemSemaphore::release(int n);
  fn C_ZN16QSystemSemaphore7releaseEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_char;
  // proto:  QString QSystemSemaphore::errorString();
  fn C_ZNK16QSystemSemaphore11errorStringEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QSystemSemaphore::acquire();
  fn C_ZN16QSystemSemaphore7acquireEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QSystemSemaphore::~QSystemSemaphore();
  fn C_ZN16QSystemSemaphoreD2Ev(qthis: u64 /* *mut c_void*/);
} // <= ext block end

// body block begin =>
// class sizeof(QSystemSemaphore)=1
#[derive(Default)]
pub struct QSystemSemaphore {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QSystemSemaphore {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QSystemSemaphore {
    return QSystemSemaphore{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  QString QSystemSemaphore::key();
impl /*struct*/ QSystemSemaphore {
  pub fn key<RetType, T: QSystemSemaphore_key<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.key(self);
    // return 1;
  }
}

pub trait QSystemSemaphore_key<RetType> {
  fn key(self , rsthis: & QSystemSemaphore) -> RetType;
}

  // proto:  QString QSystemSemaphore::key();
impl<'a> /*trait*/ QSystemSemaphore_key<QString> for () {
  fn key(self , rsthis: & QSystemSemaphore) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QSystemSemaphore3keyEv()};
    let mut ret = unsafe {C_ZNK16QSystemSemaphore3keyEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QSystemSemaphore::release(int n);
impl /*struct*/ QSystemSemaphore {
  pub fn release<RetType, T: QSystemSemaphore_release<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.release(self);
    // return 1;
  }
}

pub trait QSystemSemaphore_release<RetType> {
  fn release(self , rsthis: & QSystemSemaphore) -> RetType;
}

  // proto:  bool QSystemSemaphore::release(int n);
impl<'a> /*trait*/ QSystemSemaphore_release<i8> for (Option<i32>) {
  fn release(self , rsthis: & QSystemSemaphore) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QSystemSemaphore7releaseEi()};
    let arg0 = (if self.is_none() {1} else {self.unwrap()})  as c_int;
    let mut ret = unsafe {C_ZN16QSystemSemaphore7releaseEi(rsthis.qclsinst, arg0)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  QString QSystemSemaphore::errorString();
impl /*struct*/ QSystemSemaphore {
  pub fn errorString<RetType, T: QSystemSemaphore_errorString<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.errorString(self);
    // return 1;
  }
}

pub trait QSystemSemaphore_errorString<RetType> {
  fn errorString(self , rsthis: & QSystemSemaphore) -> RetType;
}

  // proto:  QString QSystemSemaphore::errorString();
impl<'a> /*trait*/ QSystemSemaphore_errorString<QString> for () {
  fn errorString(self , rsthis: & QSystemSemaphore) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QSystemSemaphore11errorStringEv()};
    let mut ret = unsafe {C_ZNK16QSystemSemaphore11errorStringEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QSystemSemaphore::acquire();
impl /*struct*/ QSystemSemaphore {
  pub fn acquire<RetType, T: QSystemSemaphore_acquire<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.acquire(self);
    // return 1;
  }
}

pub trait QSystemSemaphore_acquire<RetType> {
  fn acquire(self , rsthis: & QSystemSemaphore) -> RetType;
}

  // proto:  bool QSystemSemaphore::acquire();
impl<'a> /*trait*/ QSystemSemaphore_acquire<i8> for () {
  fn acquire(self , rsthis: & QSystemSemaphore) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QSystemSemaphore7acquireEv()};
    let mut ret = unsafe {C_ZN16QSystemSemaphore7acquireEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QSystemSemaphore::~QSystemSemaphore();
impl /*struct*/ QSystemSemaphore {
  pub fn free<RetType, T: QSystemSemaphore_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QSystemSemaphore_free<RetType> {
  fn free(self , rsthis: & QSystemSemaphore) -> RetType;
}

  // proto:  void QSystemSemaphore::~QSystemSemaphore();
impl<'a> /*trait*/ QSystemSemaphore_free<()> for () {
  fn free(self , rsthis: & QSystemSemaphore) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QSystemSemaphoreD2Ev()};
     unsafe {C_ZN16QSystemSemaphoreD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

