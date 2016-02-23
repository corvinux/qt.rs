// auto generated, do not modify.
// created: Mon Feb 22 23:57:02 2016
// src-file: /QtCore/qeventloop.h
// dst-file: /src/core/qeventloop.rs
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
use super::qobject::*; // 773
use std::ops::Deref;
use super::qobjectdefs::*; // 773
use super::qcoreevent::*; // 773
use super::qthread::*; // 773
// use super::qeventloop::QEventLoop; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QEventLoop_Class_Size() -> c_int;
  // proto:  void QEventLoop::exit(int returnCode);
  fn C_ZN10QEventLoop4exitEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QEventLoop::quit();
  fn C_ZN10QEventLoop4quitEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QEventLoop::QEventLoop(QObject * parent);
  fn C_ZN10QEventLoopC2EP7QObject(arg0: *mut c_void) -> u64;
  // proto:  bool QEventLoop::isRunning();
  fn C_ZNK10QEventLoop9isRunningEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  const QMetaObject * QEventLoop::metaObject();
  fn C_ZNK10QEventLoop10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QEventLoop::wakeUp();
  fn C_ZN10QEventLoop6wakeUpEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QEventLoop::~QEventLoop();
  fn C_ZN10QEventLoopD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  bool QEventLoop::event(QEvent * event);
  fn C_ZN10QEventLoop5eventEP6QEvent(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  fn QEventLoopLocker_Class_Size() -> c_int;
  // proto:  void QEventLoopLocker::QEventLoopLocker(QThread * thread);
  fn C_ZN16QEventLoopLockerC2EP7QThread(arg0: *mut c_void) -> u64;
  // proto:  void QEventLoopLocker::QEventLoopLocker(QEventLoop * loop);
  fn C_ZN16QEventLoopLockerC2EP10QEventLoop(arg0: *mut c_void) -> u64;
  // proto:  void QEventLoopLocker::QEventLoopLocker();
  fn C_ZN16QEventLoopLockerC2Ev() -> u64;
  // proto:  void QEventLoopLocker::~QEventLoopLocker();
  fn C_ZN16QEventLoopLockerD2Ev(qthis: u64 /* *mut c_void*/);
} // <= ext block end

// body block begin =>
// class sizeof(QEventLoop)=1
#[derive(Default)]
pub struct QEventLoop {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QEventLoopLocker)=8
#[derive(Default)]
pub struct QEventLoopLocker {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QEventLoop {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QEventLoop {
    return QEventLoop{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QEventLoop {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QEventLoop {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  void QEventLoop::exit(int returnCode);
impl /*struct*/ QEventLoop {
  pub fn exit<RetType, T: QEventLoop_exit<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.exit(self);
    // return 1;
  }
}

pub trait QEventLoop_exit<RetType> {
  fn exit(self , rsthis: & QEventLoop) -> RetType;
}

  // proto:  void QEventLoop::exit(int returnCode);
impl<'a> /*trait*/ QEventLoop_exit<()> for (Option<i32>) {
  fn exit(self , rsthis: & QEventLoop) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QEventLoop4exitEi()};
    let arg0 = (if self.is_none() {0} else {self.unwrap()})  as c_int;
     unsafe {C_ZN10QEventLoop4exitEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QEventLoop::quit();
impl /*struct*/ QEventLoop {
  pub fn quit<RetType, T: QEventLoop_quit<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.quit(self);
    // return 1;
  }
}

pub trait QEventLoop_quit<RetType> {
  fn quit(self , rsthis: & QEventLoop) -> RetType;
}

  // proto:  void QEventLoop::quit();
impl<'a> /*trait*/ QEventLoop_quit<()> for () {
  fn quit(self , rsthis: & QEventLoop) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QEventLoop4quitEv()};
     unsafe {C_ZN10QEventLoop4quitEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QEventLoop::QEventLoop(QObject * parent);
impl /*struct*/ QEventLoop {
  pub fn new<T: QEventLoop_new>(value: T) -> QEventLoop {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QEventLoop_new {
  fn new(self) -> QEventLoop;
}

  // proto:  void QEventLoop::QEventLoop(QObject * parent);
impl<'a> /*trait*/ QEventLoop_new for (Option<&'a QObject>) {
  fn new(self) -> QEventLoop {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QEventLoopC2EP7QObject()};
    let ctysz: c_int = unsafe{QEventLoop_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = (if self.is_none() {0} else {self.unwrap().qclsinst})  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN10QEventLoopC2EP7QObject(arg0)};
    let rsthis = QEventLoop{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QEventLoop::isRunning();
impl /*struct*/ QEventLoop {
  pub fn isRunning<RetType, T: QEventLoop_isRunning<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isRunning(self);
    // return 1;
  }
}

pub trait QEventLoop_isRunning<RetType> {
  fn isRunning(self , rsthis: & QEventLoop) -> RetType;
}

  // proto:  bool QEventLoop::isRunning();
impl<'a> /*trait*/ QEventLoop_isRunning<i8> for () {
  fn isRunning(self , rsthis: & QEventLoop) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QEventLoop9isRunningEv()};
    let mut ret = unsafe {C_ZNK10QEventLoop9isRunningEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  const QMetaObject * QEventLoop::metaObject();
impl /*struct*/ QEventLoop {
  pub fn metaObject<RetType, T: QEventLoop_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QEventLoop_metaObject<RetType> {
  fn metaObject(self , rsthis: & QEventLoop) -> RetType;
}

  // proto:  const QMetaObject * QEventLoop::metaObject();
impl<'a> /*trait*/ QEventLoop_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QEventLoop) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QEventLoop10metaObjectEv()};
    let mut ret = unsafe {C_ZNK10QEventLoop10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QEventLoop::wakeUp();
impl /*struct*/ QEventLoop {
  pub fn wakeUp<RetType, T: QEventLoop_wakeUp<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.wakeUp(self);
    // return 1;
  }
}

pub trait QEventLoop_wakeUp<RetType> {
  fn wakeUp(self , rsthis: & QEventLoop) -> RetType;
}

  // proto:  void QEventLoop::wakeUp();
impl<'a> /*trait*/ QEventLoop_wakeUp<()> for () {
  fn wakeUp(self , rsthis: & QEventLoop) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QEventLoop6wakeUpEv()};
     unsafe {C_ZN10QEventLoop6wakeUpEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QEventLoop::~QEventLoop();
impl /*struct*/ QEventLoop {
  pub fn free<RetType, T: QEventLoop_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QEventLoop_free<RetType> {
  fn free(self , rsthis: & QEventLoop) -> RetType;
}

  // proto:  void QEventLoop::~QEventLoop();
impl<'a> /*trait*/ QEventLoop_free<()> for () {
  fn free(self , rsthis: & QEventLoop) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QEventLoopD2Ev()};
     unsafe {C_ZN10QEventLoopD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QEventLoop::event(QEvent * event);
impl /*struct*/ QEventLoop {
  pub fn event<RetType, T: QEventLoop_event<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.event(self);
    // return 1;
  }
}

pub trait QEventLoop_event<RetType> {
  fn event(self , rsthis: & QEventLoop) -> RetType;
}

  // proto:  bool QEventLoop::event(QEvent * event);
impl<'a> /*trait*/ QEventLoop_event<i8> for (&'a QEvent) {
  fn event(self , rsthis: & QEventLoop) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QEventLoop5eventEP6QEvent()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN10QEventLoop5eventEP6QEvent(rsthis.qclsinst, arg0)};
    return ret as i8; // 1
    // return 1;
  }
}

impl /*struct*/ QEventLoopLocker {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QEventLoopLocker {
    return QEventLoopLocker{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QEventLoopLocker::QEventLoopLocker(QThread * thread);
impl /*struct*/ QEventLoopLocker {
  pub fn new<T: QEventLoopLocker_new>(value: T) -> QEventLoopLocker {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QEventLoopLocker_new {
  fn new(self) -> QEventLoopLocker;
}

  // proto:  void QEventLoopLocker::QEventLoopLocker(QThread * thread);
impl<'a> /*trait*/ QEventLoopLocker_new for (&'a QThread) {
  fn new(self) -> QEventLoopLocker {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QEventLoopLockerC2EP7QThread()};
    let ctysz: c_int = unsafe{QEventLoopLocker_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN16QEventLoopLockerC2EP7QThread(arg0)};
    let rsthis = QEventLoopLocker{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QEventLoopLocker::QEventLoopLocker(QEventLoop * loop);
impl<'a> /*trait*/ QEventLoopLocker_new for (&'a QEventLoop) {
  fn new(self) -> QEventLoopLocker {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QEventLoopLockerC2EP10QEventLoop()};
    let ctysz: c_int = unsafe{QEventLoopLocker_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN16QEventLoopLockerC2EP10QEventLoop(arg0)};
    let rsthis = QEventLoopLocker{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QEventLoopLocker::QEventLoopLocker();
impl<'a> /*trait*/ QEventLoopLocker_new for () {
  fn new(self) -> QEventLoopLocker {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QEventLoopLockerC2Ev()};
    let ctysz: c_int = unsafe{QEventLoopLocker_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN16QEventLoopLockerC2Ev()};
    let rsthis = QEventLoopLocker{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QEventLoopLocker::~QEventLoopLocker();
impl /*struct*/ QEventLoopLocker {
  pub fn free<RetType, T: QEventLoopLocker_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QEventLoopLocker_free<RetType> {
  fn free(self , rsthis: & QEventLoopLocker) -> RetType;
}

  // proto:  void QEventLoopLocker::~QEventLoopLocker();
impl<'a> /*trait*/ QEventLoopLocker_free<()> for () {
  fn free(self , rsthis: & QEventLoopLocker) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QEventLoopLockerD2Ev()};
     unsafe {C_ZN16QEventLoopLockerD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

