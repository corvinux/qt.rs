// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qpointf::QPointF;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  QPointF QScrollEvent::contentPos();
  fn _ZNK12QScrollEvent10contentPosEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QPointF QScrollEvent::overshootDistance();
  fn _ZNK12QScrollEvent17overshootDistanceEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QScrollEvent::FreeQScrollEvent();
  fn _ZN12QScrollEventD0Ev(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QScrollEvent)=64
pub struct QScrollEvent {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QScrollEvent {
  pub fn contentPos<RetType, T: QScrollEvent_contentPos<RetType>>(&mut self, value: T) -> RetType {
    return value.contentPos(self);
    // return 1;
  }
}

pub trait QScrollEvent_contentPos<RetType> {
  fn contentPos(self, rsthis: &mut QScrollEvent) -> RetType;
}

// proto:  QPointF QScrollEvent::contentPos();
impl<'a> /*trait*/ QScrollEvent_contentPos<QPointF> for () {
  fn contentPos(self, rsthis: &mut QScrollEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 64)};
    // unsafe{_ZNK12QScrollEvent10contentPosEv()};
    let mut ret = unsafe {_ZNK12QScrollEvent10contentPosEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QScrollEvent {
  pub fn overshootDistance<RetType, T: QScrollEvent_overshootDistance<RetType>>(&mut self, value: T) -> RetType {
    return value.overshootDistance(self);
    // return 1;
  }
}

pub trait QScrollEvent_overshootDistance<RetType> {
  fn overshootDistance(self, rsthis: &mut QScrollEvent) -> RetType;
}

// proto:  QPointF QScrollEvent::overshootDistance();
impl<'a> /*trait*/ QScrollEvent_overshootDistance<QPointF> for () {
  fn overshootDistance(self, rsthis: &mut QScrollEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 64)};
    // unsafe{_ZNK12QScrollEvent17overshootDistanceEv()};
    let mut ret = unsafe {_ZNK12QScrollEvent17overshootDistanceEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QScrollEvent {
  pub fn FreeQScrollEvent<RetType, T: QScrollEvent_FreeQScrollEvent<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQScrollEvent(self);
    // return 1;
  }
}

pub trait QScrollEvent_FreeQScrollEvent<RetType> {
  fn FreeQScrollEvent(self, rsthis: &mut QScrollEvent) -> RetType;
}

// proto:  void QScrollEvent::FreeQScrollEvent();
impl<'a> /*trait*/ QScrollEvent_FreeQScrollEvent<()> for () {
  fn FreeQScrollEvent(self, rsthis: &mut QScrollEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 64)};
    // unsafe{_ZN12QScrollEventD0Ev()};
     unsafe {_ZN12QScrollEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

