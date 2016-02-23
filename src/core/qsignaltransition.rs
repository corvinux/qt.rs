// auto generated, do not modify.
// created: Mon Feb 22 23:57:02 2016
// src-file: /QtCore/qsignaltransition.h
// dst-file: /src/core/qsignaltransition.rs
//

// header block begin =>
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;


// <= header block end

// main block begin =>
// <= main block end

// use block begin =>
use super::qabstracttransition::*; // 773
use std::ops::Deref;
use super::qobject::*; // 773
use super::qbytearray::*; // 773
use super::qstate::*; // 773
use super::qobjectdefs::*; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QSignalTransition_Class_Size() -> c_int;
  // proto:  void QSignalTransition::setSenderObject(const QObject * sender);
  fn C_ZN17QSignalTransition15setSenderObjectEPK7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QByteArray QSignalTransition::signal();
  fn C_ZNK17QSignalTransition6signalEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QSignalTransition::~QSignalTransition();
  fn C_ZN17QSignalTransitionD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QSignalTransition::QSignalTransition(const QObject * sender, const char * signal, QState * sourceState);
  fn C_ZN17QSignalTransitionC2EPK7QObjectPKcP6QState(arg0: *mut c_void, arg1: *mut c_char, arg2: *mut c_void) -> u64;
  // proto:  QObject * QSignalTransition::senderObject();
  fn C_ZNK17QSignalTransition12senderObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QSignalTransition::QSignalTransition(QState * sourceState);
  fn C_ZN17QSignalTransitionC2EP6QState(arg0: *mut c_void) -> u64;
  // proto:  void QSignalTransition::setSignal(const QByteArray & signal);
  fn C_ZN17QSignalTransition9setSignalERK10QByteArray(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  const QMetaObject * QSignalTransition::metaObject();
  fn C_ZNK17QSignalTransition10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
} // <= ext block end

// body block begin =>
// class sizeof(QSignalTransition)=1
#[derive(Default)]
pub struct QSignalTransition {
  qbase: QAbstractTransition,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _senderObjectChanged: QSignalTransition_senderObjectChanged_signal,
  pub _signalChanged: QSignalTransition_signalChanged_signal,
}

impl /*struct*/ QSignalTransition {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QSignalTransition {
    return QSignalTransition{qbase: QAbstractTransition::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QSignalTransition {
  type Target = QAbstractTransition;

  fn deref(&self) -> &QAbstractTransition {
    return & self.qbase;
  }
}
impl AsRef<QAbstractTransition> for QSignalTransition {
  fn as_ref(& self) -> & QAbstractTransition {
    return & self.qbase;
  }
}
  // proto:  void QSignalTransition::setSenderObject(const QObject * sender);
impl /*struct*/ QSignalTransition {
  pub fn setSenderObject<RetType, T: QSignalTransition_setSenderObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSenderObject(self);
    // return 1;
  }
}

pub trait QSignalTransition_setSenderObject<RetType> {
  fn setSenderObject(self , rsthis: & QSignalTransition) -> RetType;
}

  // proto:  void QSignalTransition::setSenderObject(const QObject * sender);
impl<'a> /*trait*/ QSignalTransition_setSenderObject<()> for (&'a QObject) {
  fn setSenderObject(self , rsthis: & QSignalTransition) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QSignalTransition15setSenderObjectEPK7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN17QSignalTransition15setSenderObjectEPK7QObject(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QByteArray QSignalTransition::signal();
impl /*struct*/ QSignalTransition {
  pub fn signal<RetType, T: QSignalTransition_signal<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.signal(self);
    // return 1;
  }
}

pub trait QSignalTransition_signal<RetType> {
  fn signal(self , rsthis: & QSignalTransition) -> RetType;
}

  // proto:  QByteArray QSignalTransition::signal();
impl<'a> /*trait*/ QSignalTransition_signal<QByteArray> for () {
  fn signal(self , rsthis: & QSignalTransition) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QSignalTransition6signalEv()};
    let mut ret = unsafe {C_ZNK17QSignalTransition6signalEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QSignalTransition::~QSignalTransition();
impl /*struct*/ QSignalTransition {
  pub fn free<RetType, T: QSignalTransition_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QSignalTransition_free<RetType> {
  fn free(self , rsthis: & QSignalTransition) -> RetType;
}

  // proto:  void QSignalTransition::~QSignalTransition();
impl<'a> /*trait*/ QSignalTransition_free<()> for () {
  fn free(self , rsthis: & QSignalTransition) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QSignalTransitionD2Ev()};
     unsafe {C_ZN17QSignalTransitionD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QSignalTransition::QSignalTransition(const QObject * sender, const char * signal, QState * sourceState);
impl /*struct*/ QSignalTransition {
  pub fn new<T: QSignalTransition_new>(value: T) -> QSignalTransition {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QSignalTransition_new {
  fn new(self) -> QSignalTransition;
}

  // proto:  void QSignalTransition::QSignalTransition(const QObject * sender, const char * signal, QState * sourceState);
impl<'a> /*trait*/ QSignalTransition_new for (&'a QObject, &'a  String, Option<&'a QState>) {
  fn new(self) -> QSignalTransition {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QSignalTransitionC2EPK7QObjectPKcP6QState()};
    let ctysz: c_int = unsafe{QSignalTransition_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *mut c_char;
    let arg2 = (if self.2.is_none() {0} else {self.2.unwrap().qclsinst})  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN17QSignalTransitionC2EPK7QObjectPKcP6QState(arg0, arg1, arg2)};
    let rsthis = QSignalTransition{qbase: QAbstractTransition::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QObject * QSignalTransition::senderObject();
impl /*struct*/ QSignalTransition {
  pub fn senderObject<RetType, T: QSignalTransition_senderObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.senderObject(self);
    // return 1;
  }
}

pub trait QSignalTransition_senderObject<RetType> {
  fn senderObject(self , rsthis: & QSignalTransition) -> RetType;
}

  // proto:  QObject * QSignalTransition::senderObject();
impl<'a> /*trait*/ QSignalTransition_senderObject<QObject> for () {
  fn senderObject(self , rsthis: & QSignalTransition) -> QObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QSignalTransition12senderObjectEv()};
    let mut ret = unsafe {C_ZNK17QSignalTransition12senderObjectEv(rsthis.qclsinst)};
    let mut ret1 = QObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QSignalTransition::QSignalTransition(QState * sourceState);
impl<'a> /*trait*/ QSignalTransition_new for (Option<&'a QState>) {
  fn new(self) -> QSignalTransition {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QSignalTransitionC2EP6QState()};
    let ctysz: c_int = unsafe{QSignalTransition_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = (if self.is_none() {0} else {self.unwrap().qclsinst})  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN17QSignalTransitionC2EP6QState(arg0)};
    let rsthis = QSignalTransition{qbase: QAbstractTransition::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QSignalTransition::setSignal(const QByteArray & signal);
impl /*struct*/ QSignalTransition {
  pub fn setSignal<RetType, T: QSignalTransition_setSignal<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSignal(self);
    // return 1;
  }
}

pub trait QSignalTransition_setSignal<RetType> {
  fn setSignal(self , rsthis: & QSignalTransition) -> RetType;
}

  // proto:  void QSignalTransition::setSignal(const QByteArray & signal);
impl<'a> /*trait*/ QSignalTransition_setSignal<()> for (&'a QByteArray) {
  fn setSignal(self , rsthis: & QSignalTransition) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QSignalTransition9setSignalERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN17QSignalTransition9setSignalERK10QByteArray(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QSignalTransition::metaObject();
impl /*struct*/ QSignalTransition {
  pub fn metaObject<RetType, T: QSignalTransition_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QSignalTransition_metaObject<RetType> {
  fn metaObject(self , rsthis: & QSignalTransition) -> RetType;
}

  // proto:  const QMetaObject * QSignalTransition::metaObject();
impl<'a> /*trait*/ QSignalTransition_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QSignalTransition) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QSignalTransition10metaObjectEv()};
    let mut ret = unsafe {C_ZNK17QSignalTransition10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

#[derive(Default)] // for QSignalTransition_senderObjectChanged
pub struct QSignalTransition_senderObjectChanged_signal{poi:u64}
impl /* struct */ QSignalTransition {
  pub fn senderObjectChanged(&self) -> QSignalTransition_senderObjectChanged_signal {
     return QSignalTransition_senderObjectChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QSignalTransition_senderObjectChanged_signal {
  pub fn connect<T: QSignalTransition_senderObjectChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QSignalTransition_senderObjectChanged_signal_connect {
  fn connect(self, sigthis: QSignalTransition_senderObjectChanged_signal);
}

#[derive(Default)] // for QSignalTransition_signalChanged
pub struct QSignalTransition_signalChanged_signal{poi:u64}
impl /* struct */ QSignalTransition {
  pub fn signalChanged(&self) -> QSignalTransition_signalChanged_signal {
     return QSignalTransition_signalChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QSignalTransition_signalChanged_signal {
  pub fn connect<T: QSignalTransition_signalChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QSignalTransition_signalChanged_signal_connect {
  fn connect(self, sigthis: QSignalTransition_signalChanged_signal);
}

// <= body block end

