// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;
use super::qaccessibleinterface::QAccessibleInterface;
use super::qobject::QObject;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  QString QAccessibleTextInsertEvent::textInserted();
  fn _ZNK26QAccessibleTextInsertEvent12textInsertedEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QAccessibleTextInsertEvent::changePosition();
  fn _ZNK26QAccessibleTextInsertEvent14changePositionEv(qthis: *mut c_void) -> c_int;
  // proto:  void QAccessibleTextInsertEvent::QAccessibleTextInsertEvent(QAccessibleInterface * iface, int position, const QString & text);
  fn _ZN26QAccessibleTextInsertEventC1EP20QAccessibleInterfaceiRK7QString(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: *mut c_void);
  // proto:  void QAccessibleTextInsertEvent::QAccessibleTextInsertEvent(QObject * obj, int position, const QString & text);
  fn _ZN26QAccessibleTextInsertEventC1EP7QObjectiRK7QString(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: *mut c_void);
}

// body block begin
// class sizeof(QAccessibleTextInsertEvent)=48
pub struct QAccessibleTextInsertEvent {
  pub qclsinst: *mut c_void,
}

  // proto:  QString QAccessibleTextInsertEvent::textInserted();
impl /*struct*/ QAccessibleTextInsertEvent {
  pub fn textInserted<RetType, T: QAccessibleTextInsertEvent_textInserted<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.textInserted(self);
    // return 1;
  }
}

pub trait QAccessibleTextInsertEvent_textInserted<RetType> {
  fn textInserted(self , rsthis: &mut QAccessibleTextInsertEvent) -> RetType;
}

  // proto:  QString QAccessibleTextInsertEvent::textInserted();
impl<'a> /*trait*/ QAccessibleTextInsertEvent_textInserted<QString> for () {
  fn textInserted(self , rsthis: &mut QAccessibleTextInsertEvent) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK26QAccessibleTextInsertEvent12textInsertedEv()};
    let mut ret = unsafe {_ZNK26QAccessibleTextInsertEvent12textInsertedEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  int QAccessibleTextInsertEvent::changePosition();
impl /*struct*/ QAccessibleTextInsertEvent {
  pub fn changePosition<RetType, T: QAccessibleTextInsertEvent_changePosition<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.changePosition(self);
    // return 1;
  }
}

pub trait QAccessibleTextInsertEvent_changePosition<RetType> {
  fn changePosition(self , rsthis: &mut QAccessibleTextInsertEvent) -> RetType;
}

  // proto:  int QAccessibleTextInsertEvent::changePosition();
impl<'a> /*trait*/ QAccessibleTextInsertEvent_changePosition<i32> for () {
  fn changePosition(self , rsthis: &mut QAccessibleTextInsertEvent) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK26QAccessibleTextInsertEvent14changePositionEv()};
    let mut ret = unsafe {_ZNK26QAccessibleTextInsertEvent14changePositionEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QAccessibleTextInsertEvent::QAccessibleTextInsertEvent(QAccessibleInterface * iface, int position, const QString & text);
impl /*struct*/ QAccessibleTextInsertEvent {
  pub fn NewQAccessibleTextInsertEvent<T: QAccessibleTextInsertEvent_NewQAccessibleTextInsertEvent>(value: T) -> QAccessibleTextInsertEvent {
    let rsthis = value.NewQAccessibleTextInsertEvent();
    return rsthis;
    // return 1;
  }
}

pub trait QAccessibleTextInsertEvent_NewQAccessibleTextInsertEvent {
  fn NewQAccessibleTextInsertEvent(self) -> QAccessibleTextInsertEvent;
}

  // proto:  void QAccessibleTextInsertEvent::QAccessibleTextInsertEvent(QAccessibleInterface * iface, int position, const QString & text);
impl<'a> /*trait*/ QAccessibleTextInsertEvent_NewQAccessibleTextInsertEvent for (QAccessibleInterface, i32, QString) {
  fn NewQAccessibleTextInsertEvent(self) -> QAccessibleTextInsertEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN26QAccessibleTextInsertEventC1EP20QAccessibleInterfaceiRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZN26QAccessibleTextInsertEventC1EP20QAccessibleInterfaceiRK7QString(qthis, arg0, arg1, arg2)};
    let rsthis = QAccessibleTextInsertEvent{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QAccessibleTextInsertEvent::QAccessibleTextInsertEvent(QObject * obj, int position, const QString & text);
impl<'a> /*trait*/ QAccessibleTextInsertEvent_NewQAccessibleTextInsertEvent for (QObject, i32, QString) {
  fn NewQAccessibleTextInsertEvent(self) -> QAccessibleTextInsertEvent {
    let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN26QAccessibleTextInsertEventC1EP7QObjectiRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZN26QAccessibleTextInsertEventC1EP7QObjectiRK7QString(qthis, arg0, arg1, arg2)};
    let rsthis = QAccessibleTextInsertEvent{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

