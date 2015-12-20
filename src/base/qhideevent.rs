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
  // proto:  void QHideEvent::QHideEvent();
  fn _ZN10QHideEventC1Ev(qthis: *mut c_void);
  // proto:  void QHideEvent::~QHideEvent();
  fn _ZN10QHideEventD0Ev(qthis: *mut c_void);
}

// body block begin
// class sizeof(QHideEvent)=24
pub struct QHideEvent {
  pub qclsinst: *mut c_void,
}

  // proto:  void QHideEvent::QHideEvent();
impl /*struct*/ QHideEvent {
  pub fn NewQHideEvent<T: QHideEvent_NewQHideEvent>(value: T) -> QHideEvent {
    let rsthis = value.NewQHideEvent();
    return rsthis;
    // return 1;
  }
}

pub trait QHideEvent_NewQHideEvent {
  fn NewQHideEvent(self) -> QHideEvent;
}

  // proto:  void QHideEvent::QHideEvent();
impl<'a> /*trait*/ QHideEvent_NewQHideEvent for () {
  fn NewQHideEvent(self) -> QHideEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QHideEventC1Ev()};
    unsafe {_ZN10QHideEventC1Ev(qthis)};
    let rsthis = QHideEvent{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QHideEvent::~QHideEvent();
impl /*struct*/ QHideEvent {
  pub fn FreeQHideEvent<RetType, T: QHideEvent_FreeQHideEvent<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQHideEvent(self);
    // return 1;
  }
}

pub trait QHideEvent_FreeQHideEvent<RetType> {
  fn FreeQHideEvent(self , rsthis: &mut QHideEvent) -> RetType;
}

  // proto:  void QHideEvent::~QHideEvent();
impl<'a> /*trait*/ QHideEvent_FreeQHideEvent<()> for () {
  fn FreeQHideEvent(self , rsthis: &mut QHideEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QHideEventD0Ev()};
     unsafe {_ZN10QHideEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

