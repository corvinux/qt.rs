// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qobject::QObject;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QBasicTimer::FreeQBasicTimer();
  fn _ZN11QBasicTimerD0Ev(qthis: *mut c_void) ;
  // proto:  void QBasicTimer::stop();
  fn _ZN11QBasicTimer4stopEv(qthis: *mut c_void) ;
  // proto:  int QBasicTimer::timerId();
  fn _ZNK11QBasicTimer7timerIdEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QBasicTimer::isActive();
  fn _ZNK11QBasicTimer8isActiveEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QBasicTimer::NewQBasicTimer();
  fn _ZN11QBasicTimerC1Ev(qthis: *mut c_void) ;
  // proto:  void QBasicTimer::start(int msec, QObject * obj);
  fn _ZN11QBasicTimer5startEiP7QObject(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) ;
}

// body block begin
// class sizeof(QBasicTimer)=4
pub struct QBasicTimer {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QBasicTimer {
  pub fn FreeQBasicTimer<RetType, T: QBasicTimer_FreeQBasicTimer<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQBasicTimer(self);
    // return 1;
  }
}

pub trait QBasicTimer_FreeQBasicTimer<RetType> {
  fn FreeQBasicTimer(self, rsthis: &mut QBasicTimer) -> RetType;
}

// proto:  void QBasicTimer::FreeQBasicTimer();
impl<'a> /*trait*/ QBasicTimer_FreeQBasicTimer<()> for () {
  fn FreeQBasicTimer(self, rsthis: &mut QBasicTimer) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QBasicTimerD0Ev()};
     unsafe {_ZN11QBasicTimerD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QBasicTimer {
  pub fn stop<RetType, T: QBasicTimer_stop<RetType>>(&mut self, value: T) -> RetType {
    return value.stop(self);
    // return 1;
  }
}

pub trait QBasicTimer_stop<RetType> {
  fn stop(self, rsthis: &mut QBasicTimer) -> RetType;
}

// proto:  void QBasicTimer::stop();
impl<'a> /*trait*/ QBasicTimer_stop<()> for () {
  fn stop(self, rsthis: &mut QBasicTimer) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QBasicTimer4stopEv()};
     unsafe {_ZN11QBasicTimer4stopEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QBasicTimer {
  pub fn timerId<RetType, T: QBasicTimer_timerId<RetType>>(&mut self, value: T) -> RetType {
    return value.timerId(self);
    // return 1;
  }
}

pub trait QBasicTimer_timerId<RetType> {
  fn timerId(self, rsthis: &mut QBasicTimer) -> RetType;
}

// proto:  int QBasicTimer::timerId();
impl<'a> /*trait*/ QBasicTimer_timerId<i32> for () {
  fn timerId(self, rsthis: &mut QBasicTimer) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QBasicTimer7timerIdEv()};
    let mut ret = unsafe {_ZNK11QBasicTimer7timerIdEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QBasicTimer {
  pub fn isActive<RetType, T: QBasicTimer_isActive<RetType>>(&mut self, value: T) -> RetType {
    return value.isActive(self);
    // return 1;
  }
}

pub trait QBasicTimer_isActive<RetType> {
  fn isActive(self, rsthis: &mut QBasicTimer) -> RetType;
}

// proto:  bool QBasicTimer::isActive();
impl<'a> /*trait*/ QBasicTimer_isActive<i8> for () {
  fn isActive(self, rsthis: &mut QBasicTimer) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QBasicTimer8isActiveEv()};
    let mut ret = unsafe {_ZNK11QBasicTimer8isActiveEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QBasicTimer {
  pub fn NewQBasicTimer<T: QBasicTimer_NewQBasicTimer>(value: T) -> QBasicTimer {
    let rsthis = value.NewQBasicTimer();
    return rsthis;
    // return 1;
  }
}

pub trait QBasicTimer_NewQBasicTimer {
  fn NewQBasicTimer(self) -> QBasicTimer;
}

// proto: void QBasicTimer::NewQBasicTimer();
impl<'a> /*trait*/ QBasicTimer_NewQBasicTimer for () {
  fn NewQBasicTimer(self) -> QBasicTimer {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QBasicTimerC1Ev()};
    unsafe {_ZN11QBasicTimerC1Ev(qthis)};
    let rsthis = QBasicTimer{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QBasicTimer {
  pub fn start<RetType, T: QBasicTimer_start<RetType>>(&mut self, value: T) -> RetType {
    return value.start(self);
    // return 1;
  }
}

pub trait QBasicTimer_start<RetType> {
  fn start(self, rsthis: &mut QBasicTimer) -> RetType;
}

// proto:  void QBasicTimer::start(int msec, QObject * obj);
impl<'a> /*trait*/ QBasicTimer_start<()> for (i32, &'a mut QObject) {
  fn start(self, rsthis: &mut QBasicTimer) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QBasicTimer5startEiP7QObject()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN11QBasicTimer5startEiP7QObject(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

