// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qrect::QRect;
use super::qregion::QRegion;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QPaintEvent::~QPaintEvent();
  fn _ZN11QPaintEventD0Ev(qthis: *mut c_void);
  // proto:  const QRect & QPaintEvent::rect();
  fn _ZNK11QPaintEvent4rectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPaintEvent::QPaintEvent(const QRect & paintRect);
  fn _ZN11QPaintEventC1ERK5QRect(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  const QRegion & QPaintEvent::region();
  fn _ZNK11QPaintEvent6regionEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPaintEvent::QPaintEvent(const QRegion & paintRegion);
  fn _ZN11QPaintEventC1ERK7QRegion(qthis: *mut c_void, arg0: *mut c_void);
}

// body block begin
// class sizeof(QPaintEvent)=56
pub struct QPaintEvent {
  pub qclsinst: *mut c_void,
}

  // proto:  void QPaintEvent::~QPaintEvent();
impl /*struct*/ QPaintEvent {
  pub fn FreeQPaintEvent<RetType, T: QPaintEvent_FreeQPaintEvent<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQPaintEvent(self);
    // return 1;
  }
}

pub trait QPaintEvent_FreeQPaintEvent<RetType> {
  fn FreeQPaintEvent(self , rsthis: &mut QPaintEvent) -> RetType;
}

  // proto:  void QPaintEvent::~QPaintEvent();
impl<'a> /*trait*/ QPaintEvent_FreeQPaintEvent<()> for () {
  fn FreeQPaintEvent(self , rsthis: &mut QPaintEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 56)};
    // unsafe{_ZN11QPaintEventD0Ev()};
     unsafe {_ZN11QPaintEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  const QRect & QPaintEvent::rect();
impl /*struct*/ QPaintEvent {
  pub fn rect<RetType, T: QPaintEvent_rect<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.rect(self);
    // return 1;
  }
}

pub trait QPaintEvent_rect<RetType> {
  fn rect(self , rsthis: &mut QPaintEvent) -> RetType;
}

  // proto:  const QRect & QPaintEvent::rect();
impl<'a> /*trait*/ QPaintEvent_rect<QRect> for () {
  fn rect(self , rsthis: &mut QPaintEvent) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 56)};
    // unsafe{_ZNK11QPaintEvent4rectEv()};
    let mut ret = unsafe {_ZNK11QPaintEvent4rectEv(rsthis.qclsinst)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QPaintEvent::QPaintEvent(const QRect & paintRect);
impl /*struct*/ QPaintEvent {
  pub fn NewQPaintEvent<T: QPaintEvent_NewQPaintEvent>(value: T) -> QPaintEvent {
    let rsthis = value.NewQPaintEvent();
    return rsthis;
    // return 1;
  }
}

pub trait QPaintEvent_NewQPaintEvent {
  fn NewQPaintEvent(self) -> QPaintEvent;
}

  // proto:  void QPaintEvent::QPaintEvent(const QRect & paintRect);
impl<'a> /*trait*/ QPaintEvent_NewQPaintEvent for (QRect) {
  fn NewQPaintEvent(self) -> QPaintEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 56)};
    // unsafe{_ZN11QPaintEventC1ERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QPaintEventC1ERK5QRect(qthis, arg0)};
    let rsthis = QPaintEvent{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QRegion & QPaintEvent::region();
impl /*struct*/ QPaintEvent {
  pub fn region<RetType, T: QPaintEvent_region<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.region(self);
    // return 1;
  }
}

pub trait QPaintEvent_region<RetType> {
  fn region(self , rsthis: &mut QPaintEvent) -> RetType;
}

  // proto:  const QRegion & QPaintEvent::region();
impl<'a> /*trait*/ QPaintEvent_region<QRegion> for () {
  fn region(self , rsthis: &mut QPaintEvent) -> QRegion {
    // let qthis: *mut c_void = unsafe{calloc(1, 56)};
    // unsafe{_ZNK11QPaintEvent6regionEv()};
    let mut ret = unsafe {_ZNK11QPaintEvent6regionEv(rsthis.qclsinst)};
    let mut ret1 = QRegion{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QPaintEvent::QPaintEvent(const QRegion & paintRegion);
impl<'a> /*trait*/ QPaintEvent_NewQPaintEvent for (QRegion) {
  fn NewQPaintEvent(self) -> QPaintEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 56)};
    // unsafe{_ZN11QPaintEventC1ERK7QRegion()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QPaintEventC1ERK7QRegion(qthis, arg0)};
    let rsthis = QPaintEvent{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

