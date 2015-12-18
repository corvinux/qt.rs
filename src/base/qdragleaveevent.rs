// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QDragLeaveEvent::FreeQDragLeaveEvent();
  fn _ZN15QDragLeaveEventD0Ev(qthis: *mut c_void) ;
  // proto:  void QDragLeaveEvent::NewQDragLeaveEvent();
  fn _ZN15QDragLeaveEventC1Ev(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QDragLeaveEvent)=24
pub struct QDragLeaveEvent {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QDragLeaveEvent {
  pub fn FreeQDragLeaveEvent<RetType, T: QDragLeaveEvent_FreeQDragLeaveEvent<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQDragLeaveEvent(self);
    // return 1;
  }
}

pub trait QDragLeaveEvent_FreeQDragLeaveEvent<RetType> {
  fn FreeQDragLeaveEvent(self, rsthis: &mut QDragLeaveEvent) -> RetType;
}

// proto:  void QDragLeaveEvent::FreeQDragLeaveEvent();
impl<'a> /*trait*/ QDragLeaveEvent_FreeQDragLeaveEvent<()> for () {
  fn FreeQDragLeaveEvent(self, rsthis: &mut QDragLeaveEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QDragLeaveEventD0Ev()};
     unsafe {_ZN15QDragLeaveEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QDragLeaveEvent {
  pub fn NewQDragLeaveEvent<T: QDragLeaveEvent_NewQDragLeaveEvent>(value: T) -> QDragLeaveEvent {
    let rsthis = value.NewQDragLeaveEvent();
    return rsthis;
    // return 1;
  }
}

pub trait QDragLeaveEvent_NewQDragLeaveEvent {
  fn NewQDragLeaveEvent(self) -> QDragLeaveEvent;
}

// proto: void QDragLeaveEvent::NewQDragLeaveEvent();
impl<'a> /*trait*/ QDragLeaveEvent_NewQDragLeaveEvent for () {
  fn NewQDragLeaveEvent(self) -> QDragLeaveEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QDragLeaveEventC1Ev()};
    unsafe {_ZN15QDragLeaveEventC1Ev(qthis)};
    let rsthis = QDragLeaveEvent{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

