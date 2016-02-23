// auto generated, do not modify.
// created: Mon Feb 22 23:57:02 2016
// src-file: /QtCore/qcryptographichash.h
// dst-file: /src/core/qcryptographichash.rs
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
use super::qiodevice::*; // 773
use super::qbytearray::*; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QCryptographicHash_Class_Size() -> c_int;
  // proto:  bool QCryptographicHash::addData(QIODevice * device);
  fn C_ZN18QCryptographicHash7addDataEP9QIODevice(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  void QCryptographicHash::~QCryptographicHash();
  fn C_ZN18QCryptographicHashD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QCryptographicHash::reset();
  fn C_ZN18QCryptographicHash5resetEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QCryptographicHash::addData(const char * data, int length);
  fn C_ZN18QCryptographicHash7addDataEPKci(qthis: u64 /* *mut c_void*/, arg0: *mut c_char, arg1: c_int);
  // proto:  QByteArray QCryptographicHash::result();
  fn C_ZNK18QCryptographicHash6resultEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QCryptographicHash::addData(const QByteArray & data);
  fn C_ZN18QCryptographicHash7addDataERK10QByteArray(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QCryptographicHash)=8
#[derive(Default)]
pub struct QCryptographicHash {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QCryptographicHash {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QCryptographicHash {
    return QCryptographicHash{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  bool QCryptographicHash::addData(QIODevice * device);
impl /*struct*/ QCryptographicHash {
  pub fn addData<RetType, T: QCryptographicHash_addData<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addData(self);
    // return 1;
  }
}

pub trait QCryptographicHash_addData<RetType> {
  fn addData(self , rsthis: & QCryptographicHash) -> RetType;
}

  // proto:  bool QCryptographicHash::addData(QIODevice * device);
impl<'a> /*trait*/ QCryptographicHash_addData<i8> for (&'a QIODevice) {
  fn addData(self , rsthis: & QCryptographicHash) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCryptographicHash7addDataEP9QIODevice()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN18QCryptographicHash7addDataEP9QIODevice(rsthis.qclsinst, arg0)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QCryptographicHash::~QCryptographicHash();
impl /*struct*/ QCryptographicHash {
  pub fn free<RetType, T: QCryptographicHash_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QCryptographicHash_free<RetType> {
  fn free(self , rsthis: & QCryptographicHash) -> RetType;
}

  // proto:  void QCryptographicHash::~QCryptographicHash();
impl<'a> /*trait*/ QCryptographicHash_free<()> for () {
  fn free(self , rsthis: & QCryptographicHash) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCryptographicHashD2Ev()};
     unsafe {C_ZN18QCryptographicHashD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QCryptographicHash::reset();
impl /*struct*/ QCryptographicHash {
  pub fn reset<RetType, T: QCryptographicHash_reset<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.reset(self);
    // return 1;
  }
}

pub trait QCryptographicHash_reset<RetType> {
  fn reset(self , rsthis: & QCryptographicHash) -> RetType;
}

  // proto:  void QCryptographicHash::reset();
impl<'a> /*trait*/ QCryptographicHash_reset<()> for () {
  fn reset(self , rsthis: & QCryptographicHash) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCryptographicHash5resetEv()};
     unsafe {C_ZN18QCryptographicHash5resetEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QCryptographicHash::addData(const char * data, int length);
impl<'a> /*trait*/ QCryptographicHash_addData<()> for (&'a  String, i32) {
  fn addData(self , rsthis: & QCryptographicHash) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCryptographicHash7addDataEPKci()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_int;
     unsafe {C_ZN18QCryptographicHash7addDataEPKci(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QByteArray QCryptographicHash::result();
impl /*struct*/ QCryptographicHash {
  pub fn result<RetType, T: QCryptographicHash_result<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.result(self);
    // return 1;
  }
}

pub trait QCryptographicHash_result<RetType> {
  fn result(self , rsthis: & QCryptographicHash) -> RetType;
}

  // proto:  QByteArray QCryptographicHash::result();
impl<'a> /*trait*/ QCryptographicHash_result<QByteArray> for () {
  fn result(self , rsthis: & QCryptographicHash) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QCryptographicHash6resultEv()};
    let mut ret = unsafe {C_ZNK18QCryptographicHash6resultEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QCryptographicHash::addData(const QByteArray & data);
impl<'a> /*trait*/ QCryptographicHash_addData<()> for (&'a QByteArray) {
  fn addData(self , rsthis: & QCryptographicHash) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCryptographicHash7addDataERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN18QCryptographicHash7addDataERK10QByteArray(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// <= body block end

