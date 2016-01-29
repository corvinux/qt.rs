// auto generated, do not modify.
// created: Thu Jan 28 22:38:45 2016
// src-file: /QtCore/qthreadpool.h
// dst-file: /src/core/qthreadpool.rs
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
use super::qrunnable::*; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QThreadPool_Class_Size() -> c_int;
  // proto:  void QThreadPool::~QThreadPool();
  fn C_ZN11QThreadPoolD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  int QThreadPool::expiryTimeout();
  fn C_ZNK11QThreadPool13expiryTimeoutEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  bool QThreadPool::waitForDone(int msecs);
  fn C_ZN11QThreadPool11waitForDoneEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_char;
  // proto:  const QMetaObject * QThreadPool::metaObject();
  fn C_ZNK11QThreadPool10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QThreadPool::cancel(QRunnable * runnable);
  fn C_ZN11QThreadPool6cancelEP9QRunnable(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QThreadPool::tryStart(QRunnable * runnable);
  fn C_ZN11QThreadPool8tryStartEP9QRunnable(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto: static QThreadPool * QThreadPool::globalInstance();
  fn C_ZN11QThreadPool14globalInstanceEv() -> *mut c_void;
  // proto:  void QThreadPool::setMaxThreadCount(int maxThreadCount);
  fn C_ZN11QThreadPool17setMaxThreadCountEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QThreadPool::setExpiryTimeout(int expiryTimeout);
  fn C_ZN11QThreadPool16setExpiryTimeoutEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QThreadPool::reserveThread();
  fn C_ZN11QThreadPool13reserveThreadEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QThreadPool::clear();
  fn C_ZN11QThreadPool5clearEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QThreadPool::QThreadPool(QObject * parent);
  fn C_ZN11QThreadPoolC2EP7QObject(arg0: *mut c_void) -> u64;
  // proto:  void QThreadPool::start(QRunnable * runnable, int priority);
  fn C_ZN11QThreadPool5startEP9QRunnablei(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int);
  // proto:  int QThreadPool::maxThreadCount();
  fn C_ZNK11QThreadPool14maxThreadCountEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QThreadPool::releaseThread();
  fn C_ZN11QThreadPool13releaseThreadEv(qthis: u64 /* *mut c_void*/);
  // proto:  int QThreadPool::activeThreadCount();
  fn C_ZNK11QThreadPool17activeThreadCountEv(qthis: u64 /* *mut c_void*/) -> c_int;
} // <= ext block end

// body block begin =>
// class sizeof(QThreadPool)=1
#[derive(Default)]
pub struct QThreadPool {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QThreadPool {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QThreadPool {
    return QThreadPool{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QThreadPool {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QThreadPool {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  void QThreadPool::~QThreadPool();
impl /*struct*/ QThreadPool {
  pub fn free<RetType, T: QThreadPool_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QThreadPool_free<RetType> {
  fn free(self , rsthis: & QThreadPool) -> RetType;
}

  // proto:  void QThreadPool::~QThreadPool();
impl<'a> /*trait*/ QThreadPool_free<()> for () {
  fn free(self , rsthis: & QThreadPool) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QThreadPoolD2Ev()};
     unsafe {C_ZN11QThreadPoolD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QThreadPool::expiryTimeout();
impl /*struct*/ QThreadPool {
  pub fn expiryTimeout<RetType, T: QThreadPool_expiryTimeout<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.expiryTimeout(self);
    // return 1;
  }
}

pub trait QThreadPool_expiryTimeout<RetType> {
  fn expiryTimeout(self , rsthis: & QThreadPool) -> RetType;
}

  // proto:  int QThreadPool::expiryTimeout();
impl<'a> /*trait*/ QThreadPool_expiryTimeout<i32> for () {
  fn expiryTimeout(self , rsthis: & QThreadPool) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QThreadPool13expiryTimeoutEv()};
    let mut ret = unsafe {C_ZNK11QThreadPool13expiryTimeoutEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  bool QThreadPool::waitForDone(int msecs);
impl /*struct*/ QThreadPool {
  pub fn waitForDone<RetType, T: QThreadPool_waitForDone<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.waitForDone(self);
    // return 1;
  }
}

pub trait QThreadPool_waitForDone<RetType> {
  fn waitForDone(self , rsthis: & QThreadPool) -> RetType;
}

  // proto:  bool QThreadPool::waitForDone(int msecs);
impl<'a> /*trait*/ QThreadPool_waitForDone<i8> for (i32) {
  fn waitForDone(self , rsthis: & QThreadPool) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QThreadPool11waitForDoneEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZN11QThreadPool11waitForDoneEi(rsthis.qclsinst, arg0)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  const QMetaObject * QThreadPool::metaObject();
impl /*struct*/ QThreadPool {
  pub fn metaObject<RetType, T: QThreadPool_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QThreadPool_metaObject<RetType> {
  fn metaObject(self , rsthis: & QThreadPool) -> RetType;
}

  // proto:  const QMetaObject * QThreadPool::metaObject();
impl<'a> /*trait*/ QThreadPool_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QThreadPool) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QThreadPool10metaObjectEv()};
    let mut ret = unsafe {C_ZNK11QThreadPool10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QThreadPool::cancel(QRunnable * runnable);
impl /*struct*/ QThreadPool {
  pub fn cancel<RetType, T: QThreadPool_cancel<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.cancel(self);
    // return 1;
  }
}

pub trait QThreadPool_cancel<RetType> {
  fn cancel(self , rsthis: & QThreadPool) -> RetType;
}

  // proto:  void QThreadPool::cancel(QRunnable * runnable);
impl<'a> /*trait*/ QThreadPool_cancel<()> for (&'a QRunnable) {
  fn cancel(self , rsthis: & QThreadPool) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QThreadPool6cancelEP9QRunnable()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN11QThreadPool6cancelEP9QRunnable(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QThreadPool::tryStart(QRunnable * runnable);
impl /*struct*/ QThreadPool {
  pub fn tryStart<RetType, T: QThreadPool_tryStart<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.tryStart(self);
    // return 1;
  }
}

pub trait QThreadPool_tryStart<RetType> {
  fn tryStart(self , rsthis: & QThreadPool) -> RetType;
}

  // proto:  bool QThreadPool::tryStart(QRunnable * runnable);
impl<'a> /*trait*/ QThreadPool_tryStart<i8> for (&'a QRunnable) {
  fn tryStart(self , rsthis: & QThreadPool) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QThreadPool8tryStartEP9QRunnable()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN11QThreadPool8tryStartEP9QRunnable(rsthis.qclsinst, arg0)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto: static QThreadPool * QThreadPool::globalInstance();
impl /*struct*/ QThreadPool {
  pub fn globalInstance_s<RetType, T: QThreadPool_globalInstance_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.globalInstance_s();
    // return 1;
  }
}

pub trait QThreadPool_globalInstance_s<RetType> {
  fn globalInstance_s(self ) -> RetType;
}

  // proto: static QThreadPool * QThreadPool::globalInstance();
impl<'a> /*trait*/ QThreadPool_globalInstance_s<QThreadPool> for () {
  fn globalInstance_s(self ) -> QThreadPool {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QThreadPool14globalInstanceEv()};
    let mut ret = unsafe {C_ZN11QThreadPool14globalInstanceEv()};
    let mut ret1 = QThreadPool::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QThreadPool::setMaxThreadCount(int maxThreadCount);
impl /*struct*/ QThreadPool {
  pub fn setMaxThreadCount<RetType, T: QThreadPool_setMaxThreadCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMaxThreadCount(self);
    // return 1;
  }
}

pub trait QThreadPool_setMaxThreadCount<RetType> {
  fn setMaxThreadCount(self , rsthis: & QThreadPool) -> RetType;
}

  // proto:  void QThreadPool::setMaxThreadCount(int maxThreadCount);
impl<'a> /*trait*/ QThreadPool_setMaxThreadCount<()> for (i32) {
  fn setMaxThreadCount(self , rsthis: & QThreadPool) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QThreadPool17setMaxThreadCountEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN11QThreadPool17setMaxThreadCountEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QThreadPool::setExpiryTimeout(int expiryTimeout);
impl /*struct*/ QThreadPool {
  pub fn setExpiryTimeout<RetType, T: QThreadPool_setExpiryTimeout<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setExpiryTimeout(self);
    // return 1;
  }
}

pub trait QThreadPool_setExpiryTimeout<RetType> {
  fn setExpiryTimeout(self , rsthis: & QThreadPool) -> RetType;
}

  // proto:  void QThreadPool::setExpiryTimeout(int expiryTimeout);
impl<'a> /*trait*/ QThreadPool_setExpiryTimeout<()> for (i32) {
  fn setExpiryTimeout(self , rsthis: & QThreadPool) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QThreadPool16setExpiryTimeoutEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN11QThreadPool16setExpiryTimeoutEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QThreadPool::reserveThread();
impl /*struct*/ QThreadPool {
  pub fn reserveThread<RetType, T: QThreadPool_reserveThread<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.reserveThread(self);
    // return 1;
  }
}

pub trait QThreadPool_reserveThread<RetType> {
  fn reserveThread(self , rsthis: & QThreadPool) -> RetType;
}

  // proto:  void QThreadPool::reserveThread();
impl<'a> /*trait*/ QThreadPool_reserveThread<()> for () {
  fn reserveThread(self , rsthis: & QThreadPool) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QThreadPool13reserveThreadEv()};
     unsafe {C_ZN11QThreadPool13reserveThreadEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QThreadPool::clear();
impl /*struct*/ QThreadPool {
  pub fn clear<RetType, T: QThreadPool_clear<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clear(self);
    // return 1;
  }
}

pub trait QThreadPool_clear<RetType> {
  fn clear(self , rsthis: & QThreadPool) -> RetType;
}

  // proto:  void QThreadPool::clear();
impl<'a> /*trait*/ QThreadPool_clear<()> for () {
  fn clear(self , rsthis: & QThreadPool) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QThreadPool5clearEv()};
     unsafe {C_ZN11QThreadPool5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QThreadPool::QThreadPool(QObject * parent);
impl /*struct*/ QThreadPool {
  pub fn new<T: QThreadPool_new>(value: T) -> QThreadPool {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QThreadPool_new {
  fn new(self) -> QThreadPool;
}

  // proto:  void QThreadPool::QThreadPool(QObject * parent);
impl<'a> /*trait*/ QThreadPool_new for (&'a QObject) {
  fn new(self) -> QThreadPool {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QThreadPoolC2EP7QObject()};
    let ctysz: c_int = unsafe{QThreadPool_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN11QThreadPoolC2EP7QObject(arg0)};
    let rsthis = QThreadPool{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QThreadPool::start(QRunnable * runnable, int priority);
impl /*struct*/ QThreadPool {
  pub fn start<RetType, T: QThreadPool_start<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.start(self);
    // return 1;
  }
}

pub trait QThreadPool_start<RetType> {
  fn start(self , rsthis: & QThreadPool) -> RetType;
}

  // proto:  void QThreadPool::start(QRunnable * runnable, int priority);
impl<'a> /*trait*/ QThreadPool_start<()> for (&'a QRunnable, i32) {
  fn start(self , rsthis: & QThreadPool) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QThreadPool5startEP9QRunnablei()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {C_ZN11QThreadPool5startEP9QRunnablei(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  int QThreadPool::maxThreadCount();
impl /*struct*/ QThreadPool {
  pub fn maxThreadCount<RetType, T: QThreadPool_maxThreadCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.maxThreadCount(self);
    // return 1;
  }
}

pub trait QThreadPool_maxThreadCount<RetType> {
  fn maxThreadCount(self , rsthis: & QThreadPool) -> RetType;
}

  // proto:  int QThreadPool::maxThreadCount();
impl<'a> /*trait*/ QThreadPool_maxThreadCount<i32> for () {
  fn maxThreadCount(self , rsthis: & QThreadPool) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QThreadPool14maxThreadCountEv()};
    let mut ret = unsafe {C_ZNK11QThreadPool14maxThreadCountEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  void QThreadPool::releaseThread();
impl /*struct*/ QThreadPool {
  pub fn releaseThread<RetType, T: QThreadPool_releaseThread<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.releaseThread(self);
    // return 1;
  }
}

pub trait QThreadPool_releaseThread<RetType> {
  fn releaseThread(self , rsthis: & QThreadPool) -> RetType;
}

  // proto:  void QThreadPool::releaseThread();
impl<'a> /*trait*/ QThreadPool_releaseThread<()> for () {
  fn releaseThread(self , rsthis: & QThreadPool) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QThreadPool13releaseThreadEv()};
     unsafe {C_ZN11QThreadPool13releaseThreadEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QThreadPool::activeThreadCount();
impl /*struct*/ QThreadPool {
  pub fn activeThreadCount<RetType, T: QThreadPool_activeThreadCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.activeThreadCount(self);
    // return 1;
  }
}

pub trait QThreadPool_activeThreadCount<RetType> {
  fn activeThreadCount(self , rsthis: & QThreadPool) -> RetType;
}

  // proto:  int QThreadPool::activeThreadCount();
impl<'a> /*trait*/ QThreadPool_activeThreadCount<i32> for () {
  fn activeThreadCount(self , rsthis: & QThreadPool) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QThreadPool17activeThreadCountEv()};
    let mut ret = unsafe {C_ZNK11QThreadPool17activeThreadCountEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

// <= body block end

