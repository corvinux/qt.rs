// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  const QString & QInputMethodEvent::preeditString();
  fn _ZNK17QInputMethodEvent13preeditStringEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QInputMethodEvent::NewQInputMethodEvent();
  fn _ZN17QInputMethodEventC1Ev(qthis: *mut c_void) ;
  // proto:  int QInputMethodEvent::replacementStart();
  fn _ZNK17QInputMethodEvent16replacementStartEv(qthis: *mut c_void) -> c_int;
  // proto:  void QInputMethodEvent::NewQInputMethodEvent(const QInputMethodEvent & other);
  fn _ZN17QInputMethodEventC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  const QString & QInputMethodEvent::commitString();
  fn _ZNK17QInputMethodEvent12commitStringEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QInputMethodEvent::setCommitString(const QString & commitString, int replaceFrom, int replaceLength);
  fn _ZN17QInputMethodEvent15setCommitStringERK7QStringii(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: c_int) ;
  // proto:  int QInputMethodEvent::replacementLength();
  fn _ZNK17QInputMethodEvent17replacementLengthEv(qthis: *mut c_void) -> c_int;
}

// body block begin
// class sizeof(QInputMethodEvent)=1
pub struct QInputMethodEvent {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QInputMethodEvent {
  pub fn preeditString<T: QInputMethodEvent_preeditString>(&mut self, value: T) -> QString {
    return value.preeditString(self);
    // return 1;
  }
}

pub trait QInputMethodEvent_preeditString {
  fn preeditString(self, rsthis: &mut QInputMethodEvent) -> QString;
}

// proto:  const QString & QInputMethodEvent::preeditString();
impl<'a> /*trait*/ QInputMethodEvent_preeditString for () {
  fn preeditString(self, rsthis: &mut QInputMethodEvent) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QInputMethodEvent13preeditStringEv()};
    let mut ret = unsafe {_ZNK17QInputMethodEvent13preeditStringEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QInputMethodEvent {
  pub fn NewQInputMethodEvent<T: QInputMethodEvent_NewQInputMethodEvent>(value: T) -> QInputMethodEvent {
    let rsthis = value.NewQInputMethodEvent();
    return rsthis;
    // return 1;
  }
}

pub trait QInputMethodEvent_NewQInputMethodEvent {
  fn NewQInputMethodEvent(self) -> QInputMethodEvent;
}

// proto: void QInputMethodEvent::NewQInputMethodEvent();
impl<'a> /*trait*/ QInputMethodEvent_NewQInputMethodEvent for () {
  fn NewQInputMethodEvent(self) -> QInputMethodEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QInputMethodEventC1Ev()};
    unsafe {_ZN17QInputMethodEventC1Ev(qthis)};
    let rsthis = QInputMethodEvent{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QInputMethodEvent {
  pub fn replacementStart<T: QInputMethodEvent_replacementStart>(&mut self, value: T) -> i32 {
    return value.replacementStart(self);
    // return 1;
  }
}

pub trait QInputMethodEvent_replacementStart {
  fn replacementStart(self, rsthis: &mut QInputMethodEvent) -> i32;
}

// proto:  int QInputMethodEvent::replacementStart();
impl<'a> /*trait*/ QInputMethodEvent_replacementStart for () {
  fn replacementStart(self, rsthis: &mut QInputMethodEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QInputMethodEvent16replacementStartEv()};
    let mut ret = unsafe {_ZNK17QInputMethodEvent16replacementStartEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto: void QInputMethodEvent::NewQInputMethodEvent(const QInputMethodEvent & other);
impl<'a> /*trait*/ QInputMethodEvent_NewQInputMethodEvent for (&'a  QInputMethodEvent) {
  fn NewQInputMethodEvent(self) -> QInputMethodEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QInputMethodEventC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN17QInputMethodEventC1ERKS_(qthis, arg0)};
    let rsthis = QInputMethodEvent{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QInputMethodEvent {
  pub fn commitString<T: QInputMethodEvent_commitString>(&mut self, value: T) -> QString {
    return value.commitString(self);
    // return 1;
  }
}

pub trait QInputMethodEvent_commitString {
  fn commitString(self, rsthis: &mut QInputMethodEvent) -> QString;
}

// proto:  const QString & QInputMethodEvent::commitString();
impl<'a> /*trait*/ QInputMethodEvent_commitString for () {
  fn commitString(self, rsthis: &mut QInputMethodEvent) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QInputMethodEvent12commitStringEv()};
    let mut ret = unsafe {_ZNK17QInputMethodEvent12commitStringEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QInputMethodEvent {
  pub fn setCommitString<T: QInputMethodEvent_setCommitString>(&mut self, value: T)  {
     value.setCommitString(self);
    // return 1;
  }
}

pub trait QInputMethodEvent_setCommitString {
  fn setCommitString(self, rsthis: &mut QInputMethodEvent) ;
}

// proto:  void QInputMethodEvent::setCommitString(const QString & commitString, int replaceFrom, int replaceLength);
impl<'a> /*trait*/ QInputMethodEvent_setCommitString for (&'a  QString, i32, i32) {
  fn setCommitString(self, rsthis: &mut QInputMethodEvent)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QInputMethodEvent15setCommitStringERK7QStringii()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
     unsafe {_ZN17QInputMethodEvent15setCommitStringERK7QStringii(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

impl /*struct*/ QInputMethodEvent {
  pub fn replacementLength<T: QInputMethodEvent_replacementLength>(&mut self, value: T) -> i32 {
    return value.replacementLength(self);
    // return 1;
  }
}

pub trait QInputMethodEvent_replacementLength {
  fn replacementLength(self, rsthis: &mut QInputMethodEvent) -> i32;
}

// proto:  int QInputMethodEvent::replacementLength();
impl<'a> /*trait*/ QInputMethodEvent_replacementLength for () {
  fn replacementLength(self, rsthis: &mut QInputMethodEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QInputMethodEvent17replacementLengthEv()};
    let mut ret = unsafe {_ZNK17QInputMethodEvent17replacementLengthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

