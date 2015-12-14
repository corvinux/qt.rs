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
  // proto:  void QCloseEvent::FreeQCloseEvent();
  fn _ZN11QCloseEventD0Ev(qthis: *mut c_void) ;
  // proto:  void QCloseEvent::NewQCloseEvent();
  fn _ZN11QCloseEventC1Ev(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QCloseEvent)=24
pub struct QCloseEvent {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QCloseEvent {
  pub fn FreeQCloseEvent<T: QCloseEvent_FreeQCloseEvent>(&mut self, value: T)  {
     value.FreeQCloseEvent(self);
    // return 1;
  }
}

pub trait QCloseEvent_FreeQCloseEvent {
  fn FreeQCloseEvent(self, rsthis: &mut QCloseEvent) ;
}

// proto:  void QCloseEvent::FreeQCloseEvent();
impl<'a> /*trait*/ QCloseEvent_FreeQCloseEvent for () {
  fn FreeQCloseEvent(self, rsthis: &mut QCloseEvent)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QCloseEventD0Ev()};
     unsafe {_ZN11QCloseEventD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QCloseEvent {
  pub fn NewQCloseEvent<T: QCloseEvent_NewQCloseEvent>(value: T) -> QCloseEvent {
    let rsthis = value.NewQCloseEvent();
    return rsthis;
    // return 1;
  }
}

pub trait QCloseEvent_NewQCloseEvent {
  fn NewQCloseEvent(self) -> QCloseEvent;
}

// proto: void QCloseEvent::NewQCloseEvent();
impl<'a> /*trait*/ QCloseEvent_NewQCloseEvent for () {
  fn NewQCloseEvent(self) -> QCloseEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QCloseEventC1Ev()};
    unsafe {_ZN11QCloseEventC1Ev(qthis)};
    let rsthis = QCloseEvent{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

