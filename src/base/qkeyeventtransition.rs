// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstate::QState;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QKeyEventTransition::setKey(int key);
  fn _ZN19QKeyEventTransition6setKeyEi(arg0: c_int) -> i32;
  // proto: const QMetaObject * QKeyEventTransition::metaObject();
  fn _ZNK19QKeyEventTransition10metaObjectEv() -> i32;
  // proto: void QKeyEventTransition::FreeQKeyEventTransition();
  fn _ZN19QKeyEventTransitionD0Ev() -> i32;
  // proto: int QKeyEventTransition::key();
  fn _ZNK19QKeyEventTransition3keyEv() -> i32;
  // proto: void QKeyEventTransition::NewQKeyEventTransition(QState * sourceState);
  fn _ZN19QKeyEventTransitionC1EP6QState(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: void QKeyEventTransition::NewQKeyEventTransition(const QKeyEventTransition & );
  fn _ZN19QKeyEventTransitionC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
}

// body block begin
// class sizeof(QKeyEventTransition)=1
pub struct QKeyEventTransition {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QKeyEventTransition {
  pub fn setKey<T: QKeyEventTransition_setKey>(&mut self, value: T) -> i32 {
    value.setKey(self);
    return 1;
  }
}

pub trait QKeyEventTransition_setKey {
  fn setKey(self, this: &mut QKeyEventTransition) -> i32;
}

// proto: void QKeyEventTransition::setKey(int key);
impl<'a> /*trait*/ QKeyEventTransition_setKey for (i32) {
  fn setKey(self, this: &mut QKeyEventTransition) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QKeyEventTransition6setKeyEi()};
    let arg0 = self  as c_int;
    unsafe {_ZN19QKeyEventTransition6setKeyEi(arg0)};
    return 1;
  }
}

impl /*struct*/ QKeyEventTransition {
  pub fn metaObject<T: QKeyEventTransition_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QKeyEventTransition_metaObject {
  fn metaObject(self, this: &mut QKeyEventTransition) -> i32;
}

// proto: const QMetaObject * QKeyEventTransition::metaObject();
impl<'a> /*trait*/ QKeyEventTransition_metaObject for () {
  fn metaObject(self, this: &mut QKeyEventTransition) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QKeyEventTransition10metaObjectEv()};
    unsafe {_ZNK19QKeyEventTransition10metaObjectEv()};
    return 1;
  }
}

impl /*struct*/ QKeyEventTransition {
  pub fn FreeQKeyEventTransition<T: QKeyEventTransition_FreeQKeyEventTransition>(&mut self, value: T) -> i32 {
    value.FreeQKeyEventTransition(self);
    return 1;
  }
}

pub trait QKeyEventTransition_FreeQKeyEventTransition {
  fn FreeQKeyEventTransition(self, this: &mut QKeyEventTransition) -> i32;
}

// proto: void QKeyEventTransition::FreeQKeyEventTransition();
impl<'a> /*trait*/ QKeyEventTransition_FreeQKeyEventTransition for () {
  fn FreeQKeyEventTransition(self, this: &mut QKeyEventTransition) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QKeyEventTransitionD0Ev()};
    unsafe {_ZN19QKeyEventTransitionD0Ev()};
    return 1;
  }
}

impl /*struct*/ QKeyEventTransition {
  pub fn key<T: QKeyEventTransition_key>(&mut self, value: T) -> i32 {
    value.key(self);
    return 1;
  }
}

pub trait QKeyEventTransition_key {
  fn key(self, this: &mut QKeyEventTransition) -> i32;
}

// proto: int QKeyEventTransition::key();
impl<'a> /*trait*/ QKeyEventTransition_key for () {
  fn key(self, this: &mut QKeyEventTransition) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QKeyEventTransition3keyEv()};
    unsafe {_ZNK19QKeyEventTransition3keyEv()};
    return 1;
  }
}

impl /*struct*/ QKeyEventTransition {
  pub fn NewQKeyEventTransition<T: QKeyEventTransition_NewQKeyEventTransition>(value: T) -> QKeyEventTransition {
    let rsthis = value.NewQKeyEventTransition();
    return rsthis;
    // return 1;
  }
}

pub trait QKeyEventTransition_NewQKeyEventTransition {
  fn NewQKeyEventTransition(self) -> QKeyEventTransition;
}

// proto: void QKeyEventTransition::NewQKeyEventTransition(QState * sourceState);
impl<'a> /*trait*/ QKeyEventTransition_NewQKeyEventTransition for (&'a mut QState) {
  fn NewQKeyEventTransition(self) -> QKeyEventTransition {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QKeyEventTransitionC1EP6QState()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN19QKeyEventTransitionC1EP6QState(qthis, arg0)};
    let rsthis = QKeyEventTransition{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QKeyEventTransition::NewQKeyEventTransition(const QKeyEventTransition & );
impl<'a> /*trait*/ QKeyEventTransition_NewQKeyEventTransition for (&'a  QKeyEventTransition) {
  fn NewQKeyEventTransition(self) -> QKeyEventTransition {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QKeyEventTransitionC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN19QKeyEventTransitionC1ERKS_(qthis, arg0)};
    let rsthis = QKeyEventTransition{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

