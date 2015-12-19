// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qobject::QObject;
use super::qbytearray::QByteArray;
use super::qstate::QState;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QSignalTransition::setSenderObject(const QObject * sender);
  fn _ZN17QSignalTransition15setSenderObjectEPK7QObject(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QByteArray QSignalTransition::signal();
  fn _ZNK17QSignalTransition6signalEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QSignalTransition::~QSignalTransition();
  fn _ZN17QSignalTransitionD0Ev(qthis: *mut c_void);
  // proto:  void QSignalTransition::QSignalTransition(const QSignalTransition & );
  fn _ZN17QSignalTransitionC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QSignalTransition::QSignalTransition(const QObject * sender, const char * signal, QState * sourceState);
  fn _ZN17QSignalTransitionC1EPK7QObjectPKcP6QState(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_char, arg2: *mut c_void);
  // proto:  QObject * QSignalTransition::senderObject();
  fn _ZNK17QSignalTransition12senderObjectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QSignalTransition::QSignalTransition(QState * sourceState);
  fn _ZN17QSignalTransitionC1EP6QState(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QSignalTransition::setSignal(const QByteArray & signal);
  fn _ZN17QSignalTransition9setSignalERK10QByteArray(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  const QMetaObject * QSignalTransition::metaObject();
  fn _ZNK17QSignalTransition10metaObjectEv(qthis: *mut c_void);
}

// body block begin
// class sizeof(QSignalTransition)=1
pub struct QSignalTransition {
  pub qclsinst: *mut c_void,
}

  // proto:  void QSignalTransition::setSenderObject(const QObject * sender);
impl /*struct*/ QSignalTransition {
  pub fn setSenderObject<RetType, T: QSignalTransition_setSenderObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setSenderObject(self);
    // return 1;
  }
}

pub trait QSignalTransition_setSenderObject<RetType> {
  fn setSenderObject(self , rsthis: &mut QSignalTransition) -> RetType;
}

  // proto:  void QSignalTransition::setSenderObject(const QObject * sender);
impl<'a> /*trait*/ QSignalTransition_setSenderObject<()> for (QObject) {
  fn setSenderObject(self , rsthis: &mut QSignalTransition) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QSignalTransition15setSenderObjectEPK7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN17QSignalTransition15setSenderObjectEPK7QObject(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QByteArray QSignalTransition::signal();
impl /*struct*/ QSignalTransition {
  pub fn signal<RetType, T: QSignalTransition_signal<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.signal(self);
    // return 1;
  }
}

pub trait QSignalTransition_signal<RetType> {
  fn signal(self , rsthis: &mut QSignalTransition) -> RetType;
}

  // proto:  QByteArray QSignalTransition::signal();
impl<'a> /*trait*/ QSignalTransition_signal<QByteArray> for () {
  fn signal(self , rsthis: &mut QSignalTransition) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QSignalTransition6signalEv()};
    let mut ret = unsafe {_ZNK17QSignalTransition6signalEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QSignalTransition::~QSignalTransition();
impl /*struct*/ QSignalTransition {
  pub fn FreeQSignalTransition<RetType, T: QSignalTransition_FreeQSignalTransition<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQSignalTransition(self);
    // return 1;
  }
}

pub trait QSignalTransition_FreeQSignalTransition<RetType> {
  fn FreeQSignalTransition(self , rsthis: &mut QSignalTransition) -> RetType;
}

  // proto:  void QSignalTransition::~QSignalTransition();
impl<'a> /*trait*/ QSignalTransition_FreeQSignalTransition<()> for () {
  fn FreeQSignalTransition(self , rsthis: &mut QSignalTransition) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QSignalTransitionD0Ev()};
     unsafe {_ZN17QSignalTransitionD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QSignalTransition::QSignalTransition(const QSignalTransition & );
impl /*struct*/ QSignalTransition {
  pub fn NewQSignalTransition<T: QSignalTransition_NewQSignalTransition>(value: T) -> QSignalTransition {
    let rsthis = value.NewQSignalTransition();
    return rsthis;
    // return 1;
  }
}

pub trait QSignalTransition_NewQSignalTransition {
  fn NewQSignalTransition(self) -> QSignalTransition;
}

  // proto:  void QSignalTransition::QSignalTransition(const QSignalTransition & );
impl<'a> /*trait*/ QSignalTransition_NewQSignalTransition for (QSignalTransition) {
  fn NewQSignalTransition(self) -> QSignalTransition {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QSignalTransitionC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN17QSignalTransitionC1ERKS_(qthis, arg0)};
    let rsthis = QSignalTransition{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QSignalTransition::QSignalTransition(const QObject * sender, const char * signal, QState * sourceState);
impl<'a> /*trait*/ QSignalTransition_NewQSignalTransition for (QObject, &'a  String, QState) {
  fn NewQSignalTransition(self) -> QSignalTransition {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QSignalTransitionC1EPK7QObjectPKcP6QState()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *mut c_char;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZN17QSignalTransitionC1EPK7QObjectPKcP6QState(qthis, arg0, arg1, arg2)};
    let rsthis = QSignalTransition{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QObject * QSignalTransition::senderObject();
impl /*struct*/ QSignalTransition {
  pub fn senderObject<RetType, T: QSignalTransition_senderObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.senderObject(self);
    // return 1;
  }
}

pub trait QSignalTransition_senderObject<RetType> {
  fn senderObject(self , rsthis: &mut QSignalTransition) -> RetType;
}

  // proto:  QObject * QSignalTransition::senderObject();
impl<'a> /*trait*/ QSignalTransition_senderObject<QObject> for () {
  fn senderObject(self , rsthis: &mut QSignalTransition) -> QObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QSignalTransition12senderObjectEv()};
    let mut ret = unsafe {_ZNK17QSignalTransition12senderObjectEv(rsthis.qclsinst)};
    let mut ret1 = QObject{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QSignalTransition::QSignalTransition(QState * sourceState);
impl<'a> /*trait*/ QSignalTransition_NewQSignalTransition for (QState) {
  fn NewQSignalTransition(self) -> QSignalTransition {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QSignalTransitionC1EP6QState()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN17QSignalTransitionC1EP6QState(qthis, arg0)};
    let rsthis = QSignalTransition{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QSignalTransition::setSignal(const QByteArray & signal);
impl /*struct*/ QSignalTransition {
  pub fn setSignal<RetType, T: QSignalTransition_setSignal<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setSignal(self);
    // return 1;
  }
}

pub trait QSignalTransition_setSignal<RetType> {
  fn setSignal(self , rsthis: &mut QSignalTransition) -> RetType;
}

  // proto:  void QSignalTransition::setSignal(const QByteArray & signal);
impl<'a> /*trait*/ QSignalTransition_setSignal<()> for (QByteArray) {
  fn setSignal(self , rsthis: &mut QSignalTransition) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QSignalTransition9setSignalERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN17QSignalTransition9setSignalERK10QByteArray(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QSignalTransition::metaObject();
impl /*struct*/ QSignalTransition {
  pub fn metaObject<RetType, T: QSignalTransition_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QSignalTransition_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QSignalTransition) -> RetType;
}

  // proto:  const QMetaObject * QSignalTransition::metaObject();
impl<'a> /*trait*/ QSignalTransition_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QSignalTransition) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QSignalTransition10metaObjectEv()};
     unsafe {_ZNK17QSignalTransition10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

