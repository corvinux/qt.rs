// auto generated, do not modify.
// created: Mon Dec 21 22:54:38 2015
// src-file: /QtCore/qmessageauthenticationcode.h
// dst-file: /src/core/qmessageauthenticationcode.rs
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
use super::qbytearray::QByteArray; // 773
use super::qiodevice::QIODevice; // 773
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  QByteArray QMessageAuthenticationCode::result();
  fn _ZNK26QMessageAuthenticationCode6resultEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QMessageAuthenticationCode::addData(const QByteArray & data);
  fn _ZN26QMessageAuthenticationCode7addDataERK10QByteArray(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QMessageAuthenticationCode::QMessageAuthenticationCode(const QMessageAuthenticationCode & );
  fn _ZN26QMessageAuthenticationCodeC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QMessageAuthenticationCode::addData(const char * data, int length);
  fn _ZN26QMessageAuthenticationCode7addDataEPKci(qthis: *mut c_void, arg0: *mut c_char, arg1: c_int);
  // proto:  void QMessageAuthenticationCode::~QMessageAuthenticationCode();
  fn _ZN26QMessageAuthenticationCodeD0Ev(qthis: *mut c_void);
  // proto:  void QMessageAuthenticationCode::reset();
  fn _ZN26QMessageAuthenticationCode5resetEv(qthis: *mut c_void);
  // proto:  bool QMessageAuthenticationCode::addData(QIODevice * device);
  fn _ZN26QMessageAuthenticationCode7addDataEP9QIODevice(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  void QMessageAuthenticationCode::setKey(const QByteArray & key);
  fn _ZN26QMessageAuthenticationCode6setKeyERK10QByteArray(qthis: *mut c_void, arg0: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QMessageAuthenticationCode)=8
pub struct QMessageAuthenticationCode {
  pub qclsinst: *mut c_void,
}

  // proto:  QByteArray QMessageAuthenticationCode::result();
impl /*struct*/ QMessageAuthenticationCode {
  pub fn result<RetType, T: QMessageAuthenticationCode_result<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.result(self);
    // return 1;
  }
}

pub trait QMessageAuthenticationCode_result<RetType> {
  fn result(self , rsthis: &mut QMessageAuthenticationCode) -> RetType;
}

  // proto:  QByteArray QMessageAuthenticationCode::result();
impl<'a> /*trait*/ QMessageAuthenticationCode_result<QByteArray> for () {
  fn result(self , rsthis: &mut QMessageAuthenticationCode) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK26QMessageAuthenticationCode6resultEv()};
    let mut ret = unsafe {_ZNK26QMessageAuthenticationCode6resultEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QMessageAuthenticationCode::addData(const QByteArray & data);
impl /*struct*/ QMessageAuthenticationCode {
  pub fn addData<RetType, T: QMessageAuthenticationCode_addData<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.addData(self);
    // return 1;
  }
}

pub trait QMessageAuthenticationCode_addData<RetType> {
  fn addData(self , rsthis: &mut QMessageAuthenticationCode) -> RetType;
}

  // proto:  void QMessageAuthenticationCode::addData(const QByteArray & data);
impl<'a> /*trait*/ QMessageAuthenticationCode_addData<()> for (QByteArray) {
  fn addData(self , rsthis: &mut QMessageAuthenticationCode) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QMessageAuthenticationCode7addDataERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN26QMessageAuthenticationCode7addDataERK10QByteArray(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QMessageAuthenticationCode::QMessageAuthenticationCode(const QMessageAuthenticationCode & );
impl /*struct*/ QMessageAuthenticationCode {
  pub fn NewQMessageAuthenticationCode<T: QMessageAuthenticationCode_NewQMessageAuthenticationCode>(value: T) -> QMessageAuthenticationCode {
    let rsthis = value.NewQMessageAuthenticationCode();
    return rsthis;
    // return 1;
  }
}

pub trait QMessageAuthenticationCode_NewQMessageAuthenticationCode {
  fn NewQMessageAuthenticationCode(self) -> QMessageAuthenticationCode;
}

  // proto:  void QMessageAuthenticationCode::QMessageAuthenticationCode(const QMessageAuthenticationCode & );
impl<'a> /*trait*/ QMessageAuthenticationCode_NewQMessageAuthenticationCode for (QMessageAuthenticationCode) {
  fn NewQMessageAuthenticationCode(self) -> QMessageAuthenticationCode {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QMessageAuthenticationCodeC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN26QMessageAuthenticationCodeC1ERKS_(qthis, arg0)};
    let rsthis = QMessageAuthenticationCode{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QMessageAuthenticationCode::addData(const char * data, int length);
impl<'a> /*trait*/ QMessageAuthenticationCode_addData<()> for (&'a  String, i32) {
  fn addData(self , rsthis: &mut QMessageAuthenticationCode) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QMessageAuthenticationCode7addDataEPKci()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as c_int;
     unsafe {_ZN26QMessageAuthenticationCode7addDataEPKci(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QMessageAuthenticationCode::~QMessageAuthenticationCode();
impl /*struct*/ QMessageAuthenticationCode {
  pub fn FreeQMessageAuthenticationCode<RetType, T: QMessageAuthenticationCode_FreeQMessageAuthenticationCode<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQMessageAuthenticationCode(self);
    // return 1;
  }
}

pub trait QMessageAuthenticationCode_FreeQMessageAuthenticationCode<RetType> {
  fn FreeQMessageAuthenticationCode(self , rsthis: &mut QMessageAuthenticationCode) -> RetType;
}

  // proto:  void QMessageAuthenticationCode::~QMessageAuthenticationCode();
impl<'a> /*trait*/ QMessageAuthenticationCode_FreeQMessageAuthenticationCode<()> for () {
  fn FreeQMessageAuthenticationCode(self , rsthis: &mut QMessageAuthenticationCode) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QMessageAuthenticationCodeD0Ev()};
     unsafe {_ZN26QMessageAuthenticationCodeD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QMessageAuthenticationCode::reset();
impl /*struct*/ QMessageAuthenticationCode {
  pub fn reset<RetType, T: QMessageAuthenticationCode_reset<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.reset(self);
    // return 1;
  }
}

pub trait QMessageAuthenticationCode_reset<RetType> {
  fn reset(self , rsthis: &mut QMessageAuthenticationCode) -> RetType;
}

  // proto:  void QMessageAuthenticationCode::reset();
impl<'a> /*trait*/ QMessageAuthenticationCode_reset<()> for () {
  fn reset(self , rsthis: &mut QMessageAuthenticationCode) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QMessageAuthenticationCode5resetEv()};
     unsafe {_ZN26QMessageAuthenticationCode5resetEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QMessageAuthenticationCode::addData(QIODevice * device);
impl<'a> /*trait*/ QMessageAuthenticationCode_addData<i8> for (QIODevice) {
  fn addData(self , rsthis: &mut QMessageAuthenticationCode) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QMessageAuthenticationCode7addDataEP9QIODevice()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN26QMessageAuthenticationCode7addDataEP9QIODevice(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QMessageAuthenticationCode::setKey(const QByteArray & key);
impl /*struct*/ QMessageAuthenticationCode {
  pub fn setKey<RetType, T: QMessageAuthenticationCode_setKey<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setKey(self);
    // return 1;
  }
}

pub trait QMessageAuthenticationCode_setKey<RetType> {
  fn setKey(self , rsthis: &mut QMessageAuthenticationCode) -> RetType;
}

  // proto:  void QMessageAuthenticationCode::setKey(const QByteArray & key);
impl<'a> /*trait*/ QMessageAuthenticationCode_setKey<()> for (QByteArray) {
  fn setKey(self , rsthis: &mut QMessageAuthenticationCode) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QMessageAuthenticationCode6setKeyERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN26QMessageAuthenticationCode6setKeyERK10QByteArray(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// <= body block end

