// auto generated, do not modify.
// created: Sun Jan 24 17:41:38 2016
// src-file: /QtCore/qelapsedtimer.h
// dst-file: /src/core/qelapsedtimer.rs
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
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QElapsedTimer_Class_Size() -> c_int;
  // proto:  void QElapsedTimer::start();
  fn C_ZN13QElapsedTimer5startEv(qthis: u64 /* *mut c_void*/);
  // proto:  qint64 QElapsedTimer::nsecsElapsed();
  fn C_ZNK13QElapsedTimer12nsecsElapsedEv(qthis: u64 /* *mut c_void*/) -> c_longlong;
  // proto:  void QElapsedTimer::invalidate();
  fn C_ZN13QElapsedTimer10invalidateEv(qthis: u64 /* *mut c_void*/);
  // proto: static bool QElapsedTimer::isMonotonic();
  fn C_ZN13QElapsedTimer11isMonotonicEv() -> c_char;
  // proto:  void QElapsedTimer::QElapsedTimer();
  fn C_ZN13QElapsedTimerC2Ev() -> u64;
  // proto:  qint64 QElapsedTimer::msecsTo(const QElapsedTimer & other);
  fn C_ZNK13QElapsedTimer7msecsToERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_longlong;
  // proto:  qint64 QElapsedTimer::msecsSinceReference();
  fn C_ZNK13QElapsedTimer19msecsSinceReferenceEv(qthis: u64 /* *mut c_void*/) -> c_longlong;
  // proto:  bool QElapsedTimer::hasExpired(qint64 timeout);
  fn C_ZNK13QElapsedTimer10hasExpiredEx(qthis: u64 /* *mut c_void*/, arg0: c_longlong) -> c_char;
  // proto:  qint64 QElapsedTimer::restart();
  fn C_ZN13QElapsedTimer7restartEv(qthis: u64 /* *mut c_void*/) -> c_longlong;
  // proto:  bool QElapsedTimer::isValid();
  fn C_ZNK13QElapsedTimer7isValidEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  qint64 QElapsedTimer::secsTo(const QElapsedTimer & other);
  fn C_ZNK13QElapsedTimer6secsToERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_longlong;
  // proto:  qint64 QElapsedTimer::elapsed();
  fn C_ZNK13QElapsedTimer7elapsedEv(qthis: u64 /* *mut c_void*/) -> c_longlong;
} // <= ext block end

// body block begin =>
// class sizeof(QElapsedTimer)=16
#[derive(Default)]
pub struct QElapsedTimer {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QElapsedTimer {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QElapsedTimer {
    return QElapsedTimer{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QElapsedTimer::start();
impl /*struct*/ QElapsedTimer {
  pub fn start<RetType, T: QElapsedTimer_start<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.start(self);
    // return 1;
  }
}

pub trait QElapsedTimer_start<RetType> {
  fn start(self , rsthis: & QElapsedTimer) -> RetType;
}

  // proto:  void QElapsedTimer::start();
impl<'a> /*trait*/ QElapsedTimer_start<()> for () {
  fn start(self , rsthis: & QElapsedTimer) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QElapsedTimer5startEv()};
     unsafe {C_ZN13QElapsedTimer5startEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  qint64 QElapsedTimer::nsecsElapsed();
impl /*struct*/ QElapsedTimer {
  pub fn nsecsElapsed<RetType, T: QElapsedTimer_nsecsElapsed<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.nsecsElapsed(self);
    // return 1;
  }
}

pub trait QElapsedTimer_nsecsElapsed<RetType> {
  fn nsecsElapsed(self , rsthis: & QElapsedTimer) -> RetType;
}

  // proto:  qint64 QElapsedTimer::nsecsElapsed();
impl<'a> /*trait*/ QElapsedTimer_nsecsElapsed<i64> for () {
  fn nsecsElapsed(self , rsthis: & QElapsedTimer) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QElapsedTimer12nsecsElapsedEv()};
    let mut ret = unsafe {C_ZNK13QElapsedTimer12nsecsElapsedEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

  // proto:  void QElapsedTimer::invalidate();
impl /*struct*/ QElapsedTimer {
  pub fn invalidate<RetType, T: QElapsedTimer_invalidate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.invalidate(self);
    // return 1;
  }
}

pub trait QElapsedTimer_invalidate<RetType> {
  fn invalidate(self , rsthis: & QElapsedTimer) -> RetType;
}

  // proto:  void QElapsedTimer::invalidate();
impl<'a> /*trait*/ QElapsedTimer_invalidate<()> for () {
  fn invalidate(self , rsthis: & QElapsedTimer) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QElapsedTimer10invalidateEv()};
     unsafe {C_ZN13QElapsedTimer10invalidateEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto: static bool QElapsedTimer::isMonotonic();
impl /*struct*/ QElapsedTimer {
  pub fn isMonotonic_s<RetType, T: QElapsedTimer_isMonotonic_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.isMonotonic_s();
    // return 1;
  }
}

pub trait QElapsedTimer_isMonotonic_s<RetType> {
  fn isMonotonic_s(self ) -> RetType;
}

  // proto: static bool QElapsedTimer::isMonotonic();
impl<'a> /*trait*/ QElapsedTimer_isMonotonic_s<i8> for () {
  fn isMonotonic_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QElapsedTimer11isMonotonicEv()};
    let mut ret = unsafe {C_ZN13QElapsedTimer11isMonotonicEv()};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QElapsedTimer::QElapsedTimer();
impl /*struct*/ QElapsedTimer {
  pub fn new<T: QElapsedTimer_new>(value: T) -> QElapsedTimer {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QElapsedTimer_new {
  fn new(self) -> QElapsedTimer;
}

  // proto:  void QElapsedTimer::QElapsedTimer();
impl<'a> /*trait*/ QElapsedTimer_new for () {
  fn new(self) -> QElapsedTimer {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QElapsedTimerC2Ev()};
    let ctysz: c_int = unsafe{QElapsedTimer_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN13QElapsedTimerC2Ev()};
    let rsthis = QElapsedTimer{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  qint64 QElapsedTimer::msecsTo(const QElapsedTimer & other);
impl /*struct*/ QElapsedTimer {
  pub fn msecsTo<RetType, T: QElapsedTimer_msecsTo<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.msecsTo(self);
    // return 1;
  }
}

pub trait QElapsedTimer_msecsTo<RetType> {
  fn msecsTo(self , rsthis: & QElapsedTimer) -> RetType;
}

  // proto:  qint64 QElapsedTimer::msecsTo(const QElapsedTimer & other);
impl<'a> /*trait*/ QElapsedTimer_msecsTo<i64> for (&'a QElapsedTimer) {
  fn msecsTo(self , rsthis: & QElapsedTimer) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QElapsedTimer7msecsToERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK13QElapsedTimer7msecsToERKS_(rsthis.qclsinst, arg0)};
    return ret as i64;
    // return 1;
  }
}

  // proto:  qint64 QElapsedTimer::msecsSinceReference();
impl /*struct*/ QElapsedTimer {
  pub fn msecsSinceReference<RetType, T: QElapsedTimer_msecsSinceReference<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.msecsSinceReference(self);
    // return 1;
  }
}

pub trait QElapsedTimer_msecsSinceReference<RetType> {
  fn msecsSinceReference(self , rsthis: & QElapsedTimer) -> RetType;
}

  // proto:  qint64 QElapsedTimer::msecsSinceReference();
impl<'a> /*trait*/ QElapsedTimer_msecsSinceReference<i64> for () {
  fn msecsSinceReference(self , rsthis: & QElapsedTimer) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QElapsedTimer19msecsSinceReferenceEv()};
    let mut ret = unsafe {C_ZNK13QElapsedTimer19msecsSinceReferenceEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

  // proto:  bool QElapsedTimer::hasExpired(qint64 timeout);
impl /*struct*/ QElapsedTimer {
  pub fn hasExpired<RetType, T: QElapsedTimer_hasExpired<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasExpired(self);
    // return 1;
  }
}

pub trait QElapsedTimer_hasExpired<RetType> {
  fn hasExpired(self , rsthis: & QElapsedTimer) -> RetType;
}

  // proto:  bool QElapsedTimer::hasExpired(qint64 timeout);
impl<'a> /*trait*/ QElapsedTimer_hasExpired<i8> for (i64) {
  fn hasExpired(self , rsthis: & QElapsedTimer) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QElapsedTimer10hasExpiredEx()};
    let arg0 = self  as c_longlong;
    let mut ret = unsafe {C_ZNK13QElapsedTimer10hasExpiredEx(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  qint64 QElapsedTimer::restart();
impl /*struct*/ QElapsedTimer {
  pub fn restart<RetType, T: QElapsedTimer_restart<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.restart(self);
    // return 1;
  }
}

pub trait QElapsedTimer_restart<RetType> {
  fn restart(self , rsthis: & QElapsedTimer) -> RetType;
}

  // proto:  qint64 QElapsedTimer::restart();
impl<'a> /*trait*/ QElapsedTimer_restart<i64> for () {
  fn restart(self , rsthis: & QElapsedTimer) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QElapsedTimer7restartEv()};
    let mut ret = unsafe {C_ZN13QElapsedTimer7restartEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

  // proto:  bool QElapsedTimer::isValid();
impl /*struct*/ QElapsedTimer {
  pub fn isValid<RetType, T: QElapsedTimer_isValid<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QElapsedTimer_isValid<RetType> {
  fn isValid(self , rsthis: & QElapsedTimer) -> RetType;
}

  // proto:  bool QElapsedTimer::isValid();
impl<'a> /*trait*/ QElapsedTimer_isValid<i8> for () {
  fn isValid(self , rsthis: & QElapsedTimer) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QElapsedTimer7isValidEv()};
    let mut ret = unsafe {C_ZNK13QElapsedTimer7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  qint64 QElapsedTimer::secsTo(const QElapsedTimer & other);
impl /*struct*/ QElapsedTimer {
  pub fn secsTo<RetType, T: QElapsedTimer_secsTo<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.secsTo(self);
    // return 1;
  }
}

pub trait QElapsedTimer_secsTo<RetType> {
  fn secsTo(self , rsthis: & QElapsedTimer) -> RetType;
}

  // proto:  qint64 QElapsedTimer::secsTo(const QElapsedTimer & other);
impl<'a> /*trait*/ QElapsedTimer_secsTo<i64> for (&'a QElapsedTimer) {
  fn secsTo(self , rsthis: & QElapsedTimer) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QElapsedTimer6secsToERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK13QElapsedTimer6secsToERKS_(rsthis.qclsinst, arg0)};
    return ret as i64;
    // return 1;
  }
}

  // proto:  qint64 QElapsedTimer::elapsed();
impl /*struct*/ QElapsedTimer {
  pub fn elapsed<RetType, T: QElapsedTimer_elapsed<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.elapsed(self);
    // return 1;
  }
}

pub trait QElapsedTimer_elapsed<RetType> {
  fn elapsed(self , rsthis: & QElapsedTimer) -> RetType;
}

  // proto:  qint64 QElapsedTimer::elapsed();
impl<'a> /*trait*/ QElapsedTimer_elapsed<i64> for () {
  fn elapsed(self , rsthis: & QElapsedTimer) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QElapsedTimer7elapsedEv()};
    let mut ret = unsafe {C_ZNK13QElapsedTimer7elapsedEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

// <= body block end

