// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qpoint::QPoint;
use super::qstring::QString;
use super::qwidget::QWidget;
use super::qobject::QObject;
use super::qaction::QAction;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: static void QWhatsThis::hideText();
  fn _ZN10QWhatsThis8hideTextEv();
  // proto: static void QWhatsThis::enterWhatsThisMode();
  fn _ZN10QWhatsThis18enterWhatsThisModeEv();
  // proto: static bool QWhatsThis::inWhatsThisMode();
  fn _ZN10QWhatsThis15inWhatsThisModeEv() -> c_char;
  // proto: static void QWhatsThis::leaveWhatsThisMode();
  fn _ZN10QWhatsThis18leaveWhatsThisModeEv();
  // proto:  void QWhatsThis::QWhatsThis();
  fn _ZN10QWhatsThisC1Ev(qthis: *mut c_void);
  // proto: static void QWhatsThis::showText(const QPoint & pos, const QString & text, QWidget * w);
  fn _ZN10QWhatsThis8showTextERK6QPointRK7QStringP7QWidget(arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto: static QAction * QWhatsThis::createAction(QObject * parent);
  fn _ZN10QWhatsThis12createActionEP7QObject(arg0: *mut c_void) -> *mut c_void;
}

// body block begin
// class sizeof(QWhatsThis)=1
pub struct QWhatsThis {
  pub qclsinst: *mut c_void,
}

  // proto: static void QWhatsThis::hideText();
impl /*struct*/ QWhatsThis {
  pub fn hideText_s<RetType, T: QWhatsThis_hideText_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.hideText_s();
    // return 1;
  }
}

pub trait QWhatsThis_hideText_s<RetType> {
  fn hideText_s(self ) -> RetType;
}

  // proto: static void QWhatsThis::hideText();
impl<'a> /*trait*/ QWhatsThis_hideText_s<()> for () {
  fn hideText_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QWhatsThis8hideTextEv()};
     unsafe {_ZN10QWhatsThis8hideTextEv()};
    // return 1;
  }
}

  // proto: static void QWhatsThis::enterWhatsThisMode();
impl /*struct*/ QWhatsThis {
  pub fn enterWhatsThisMode_s<RetType, T: QWhatsThis_enterWhatsThisMode_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.enterWhatsThisMode_s();
    // return 1;
  }
}

pub trait QWhatsThis_enterWhatsThisMode_s<RetType> {
  fn enterWhatsThisMode_s(self ) -> RetType;
}

  // proto: static void QWhatsThis::enterWhatsThisMode();
impl<'a> /*trait*/ QWhatsThis_enterWhatsThisMode_s<()> for () {
  fn enterWhatsThisMode_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QWhatsThis18enterWhatsThisModeEv()};
     unsafe {_ZN10QWhatsThis18enterWhatsThisModeEv()};
    // return 1;
  }
}

  // proto: static bool QWhatsThis::inWhatsThisMode();
impl /*struct*/ QWhatsThis {
  pub fn inWhatsThisMode_s<RetType, T: QWhatsThis_inWhatsThisMode_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.inWhatsThisMode_s();
    // return 1;
  }
}

pub trait QWhatsThis_inWhatsThisMode_s<RetType> {
  fn inWhatsThisMode_s(self ) -> RetType;
}

  // proto: static bool QWhatsThis::inWhatsThisMode();
impl<'a> /*trait*/ QWhatsThis_inWhatsThisMode_s<i8> for () {
  fn inWhatsThisMode_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QWhatsThis15inWhatsThisModeEv()};
    let mut ret = unsafe {_ZN10QWhatsThis15inWhatsThisModeEv()};
    return ret as i8;
    // return 1;
  }
}

  // proto: static void QWhatsThis::leaveWhatsThisMode();
impl /*struct*/ QWhatsThis {
  pub fn leaveWhatsThisMode_s<RetType, T: QWhatsThis_leaveWhatsThisMode_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.leaveWhatsThisMode_s();
    // return 1;
  }
}

pub trait QWhatsThis_leaveWhatsThisMode_s<RetType> {
  fn leaveWhatsThisMode_s(self ) -> RetType;
}

  // proto: static void QWhatsThis::leaveWhatsThisMode();
impl<'a> /*trait*/ QWhatsThis_leaveWhatsThisMode_s<()> for () {
  fn leaveWhatsThisMode_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QWhatsThis18leaveWhatsThisModeEv()};
     unsafe {_ZN10QWhatsThis18leaveWhatsThisModeEv()};
    // return 1;
  }
}

  // proto:  void QWhatsThis::QWhatsThis();
impl /*struct*/ QWhatsThis {
  pub fn NewQWhatsThis<T: QWhatsThis_NewQWhatsThis>(value: T) -> QWhatsThis {
    let rsthis = value.NewQWhatsThis();
    return rsthis;
    // return 1;
  }
}

pub trait QWhatsThis_NewQWhatsThis {
  fn NewQWhatsThis(self) -> QWhatsThis;
}

  // proto:  void QWhatsThis::QWhatsThis();
impl<'a> /*trait*/ QWhatsThis_NewQWhatsThis for () {
  fn NewQWhatsThis(self) -> QWhatsThis {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QWhatsThisC1Ev()};
    unsafe {_ZN10QWhatsThisC1Ev(qthis)};
    let rsthis = QWhatsThis{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto: static void QWhatsThis::showText(const QPoint & pos, const QString & text, QWidget * w);
impl /*struct*/ QWhatsThis {
  pub fn showText_s<RetType, T: QWhatsThis_showText_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.showText_s();
    // return 1;
  }
}

pub trait QWhatsThis_showText_s<RetType> {
  fn showText_s(self ) -> RetType;
}

  // proto: static void QWhatsThis::showText(const QPoint & pos, const QString & text, QWidget * w);
impl<'a> /*trait*/ QWhatsThis_showText_s<()> for (QPoint, QString, QWidget) {
  fn showText_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QWhatsThis8showTextERK6QPointRK7QStringP7QWidget()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN10QWhatsThis8showTextERK6QPointRK7QStringP7QWidget(arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto: static QAction * QWhatsThis::createAction(QObject * parent);
impl /*struct*/ QWhatsThis {
  pub fn createAction_s<RetType, T: QWhatsThis_createAction_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.createAction_s();
    // return 1;
  }
}

pub trait QWhatsThis_createAction_s<RetType> {
  fn createAction_s(self ) -> RetType;
}

  // proto: static QAction * QWhatsThis::createAction(QObject * parent);
impl<'a> /*trait*/ QWhatsThis_createAction_s<QAction> for (QObject) {
  fn createAction_s(self ) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QWhatsThis12createActionEP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN10QWhatsThis12createActionEP7QObject(arg0)};
    let mut ret1 = QAction{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

