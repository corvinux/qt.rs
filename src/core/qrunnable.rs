// auto generated, do not modify.
// created: Tue Dec 22 23:21:28 2015
// src-file: /QtCore/qrunnable.h
// dst-file: /src/core/qrunnable.rs
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
  // proto:  void QRunnable::~QRunnable();
  fn _ZN9QRunnableD0Ev(qthis: *mut c_void);
  // proto:  void QRunnable::setAutoDelete(bool _autoDelete);
  fn _ZN9QRunnable13setAutoDeleteEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QRunnable::QRunnable();
  fn _ZN9QRunnableC1Ev(qthis: *mut c_void);
  // proto:  void QRunnable::run();
  fn _ZN9QRunnable3runEv(qthis: *mut c_void);
  // proto:  bool QRunnable::autoDelete();
  fn _ZNK9QRunnable10autoDeleteEv(qthis: *mut c_void) -> c_char;
} // <= ext block end

// body block begin =>
// class sizeof(QRunnable)=16
pub struct QRunnable {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QRunnable {
  pub fn inheritFrom(qthis: *mut c_void) -> QRunnable {
    return QRunnable{qclsinst: qthis};
  }
}
  // proto:  void QRunnable::~QRunnable();
impl /*struct*/ QRunnable {
  pub fn FreeQRunnable<RetType, T: QRunnable_FreeQRunnable<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQRunnable(self);
    // return 1;
  }
}

pub trait QRunnable_FreeQRunnable<RetType> {
  fn FreeQRunnable(self , rsthis: &mut QRunnable) -> RetType;
}

  // proto:  void QRunnable::~QRunnable();
impl<'a> /*trait*/ QRunnable_FreeQRunnable<()> for () {
  fn FreeQRunnable(self , rsthis: &mut QRunnable) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QRunnableD0Ev()};
     unsafe {_ZN9QRunnableD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QRunnable::setAutoDelete(bool _autoDelete);
impl /*struct*/ QRunnable {
  pub fn setAutoDelete<RetType, T: QRunnable_setAutoDelete<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setAutoDelete(self);
    // return 1;
  }
}

pub trait QRunnable_setAutoDelete<RetType> {
  fn setAutoDelete(self , rsthis: &mut QRunnable) -> RetType;
}

  // proto:  void QRunnable::setAutoDelete(bool _autoDelete);
impl<'a> /*trait*/ QRunnable_setAutoDelete<()> for (i8) {
  fn setAutoDelete(self , rsthis: &mut QRunnable) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QRunnable13setAutoDeleteEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN9QRunnable13setAutoDeleteEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QRunnable::QRunnable();
impl /*struct*/ QRunnable {
  pub fn NewQRunnable<T: QRunnable_NewQRunnable>(value: T) -> QRunnable {
    let rsthis = value.NewQRunnable();
    return rsthis;
    // return 1;
  }
}

pub trait QRunnable_NewQRunnable {
  fn NewQRunnable(self) -> QRunnable;
}

  // proto:  void QRunnable::QRunnable();
impl<'a> /*trait*/ QRunnable_NewQRunnable for () {
  fn NewQRunnable(self) -> QRunnable {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QRunnableC1Ev()};
    unsafe {_ZN9QRunnableC1Ev(qthis)};
    let rsthis = QRunnable{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QRunnable::run();
impl /*struct*/ QRunnable {
  pub fn run<RetType, T: QRunnable_run<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.run(self);
    // return 1;
  }
}

pub trait QRunnable_run<RetType> {
  fn run(self , rsthis: &mut QRunnable) -> RetType;
}

  // proto:  void QRunnable::run();
impl<'a> /*trait*/ QRunnable_run<()> for () {
  fn run(self , rsthis: &mut QRunnable) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QRunnable3runEv()};
     unsafe {_ZN9QRunnable3runEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QRunnable::autoDelete();
impl /*struct*/ QRunnable {
  pub fn autoDelete<RetType, T: QRunnable_autoDelete<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.autoDelete(self);
    // return 1;
  }
}

pub trait QRunnable_autoDelete<RetType> {
  fn autoDelete(self , rsthis: &mut QRunnable) -> RetType;
}

  // proto:  bool QRunnable::autoDelete();
impl<'a> /*trait*/ QRunnable_autoDelete<i8> for () {
  fn autoDelete(self , rsthis: &mut QRunnable) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QRunnable10autoDeleteEv()};
    let mut ret = unsafe {_ZNK9QRunnable10autoDeleteEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// <= body block end

