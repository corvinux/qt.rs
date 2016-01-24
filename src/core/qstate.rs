// auto generated, do not modify.
// created: Sun Jan 24 17:41:38 2016
// src-file: /QtCore/qstate.h
// dst-file: /src/core/qstate.rs
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
use super::qabstractstate::QAbstractState; // 773
use std::ops::Deref;
use super::qobject::QObject; // 773
use super::qvariant::QVariant; // 773
use super::qobjectdefs::QMetaObject; // 773
use super::qabstracttransition::QAbstractTransition; // 773
use super::qsignaltransition::QSignalTransition; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QState_Class_Size() -> c_int;
  // proto:  QAbstractState * QState::errorState();
  fn C_ZNK6QState10errorStateEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QAbstractState * QState::initialState();
  fn C_ZNK6QState12initialStateEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QState::~QState();
  fn C_ZN6QStateD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QState::assignProperty(QObject * object, const char * name, const QVariant & value);
  fn C_ZN6QState14assignPropertyEP7QObjectPKcRK8QVariant(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_char, arg2: *mut c_void);
  // proto:  void QState::QState(QState * parent);
  fn C_ZN6QStateC2EPS_(arg0: *mut c_void) -> u64;
  // proto:  const QMetaObject * QState::metaObject();
  fn C_ZNK6QState10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QState::setErrorState(QAbstractState * state);
  fn C_ZN6QState13setErrorStateEP14QAbstractState(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QState::addTransition(QAbstractTransition * transition);
  fn C_ZN6QState13addTransitionEP19QAbstractTransition(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QState::removeTransition(QAbstractTransition * transition);
  fn C_ZN6QState16removeTransitionEP19QAbstractTransition(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QSignalTransition * QState::addTransition(const QObject * sender, const char * signal, QAbstractState * target);
  fn C_ZN6QState13addTransitionEPK7QObjectPKcP14QAbstractState(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_char, arg2: *mut c_void) -> *mut c_void;
  // proto:  QAbstractTransition * QState::addTransition(QAbstractState * target);
  fn C_ZN6QState13addTransitionEP14QAbstractState(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  QList<QAbstractTransition *> QState::transitions();
  fn C_ZNK6QState11transitionsEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QState::setInitialState(QAbstractState * state);
  fn C_ZN6QState15setInitialStateEP14QAbstractState(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QState)=1
#[derive(Default)]
pub struct QState {
  qbase: QAbstractState,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _childModeChanged: QState_childModeChanged_signal,
  pub _errorStateChanged: QState_errorStateChanged_signal,
  pub _finished: QState_finished_signal,
  pub _propertiesAssigned: QState_propertiesAssigned_signal,
  pub _initialStateChanged: QState_initialStateChanged_signal,
}

impl /*struct*/ QState {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QState {
    return QState{qbase: QAbstractState::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QState {
  type Target = QAbstractState;

  fn deref(&self) -> &QAbstractState {
    return & self.qbase;
  }
}
impl AsRef<QAbstractState> for QState {
  fn as_ref(& self) -> & QAbstractState {
    return & self.qbase;
  }
}
  // proto:  QAbstractState * QState::errorState();
impl /*struct*/ QState {
  pub fn errorState<RetType, T: QState_errorState<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.errorState(self);
    // return 1;
  }
}

pub trait QState_errorState<RetType> {
  fn errorState(self , rsthis: & QState) -> RetType;
}

  // proto:  QAbstractState * QState::errorState();
impl<'a> /*trait*/ QState_errorState<QAbstractState> for () {
  fn errorState(self , rsthis: & QState) -> QAbstractState {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QState10errorStateEv()};
    let mut ret = unsafe {C_ZNK6QState10errorStateEv(rsthis.qclsinst)};
    let mut ret1 = QAbstractState::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QAbstractState * QState::initialState();
impl /*struct*/ QState {
  pub fn initialState<RetType, T: QState_initialState<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.initialState(self);
    // return 1;
  }
}

pub trait QState_initialState<RetType> {
  fn initialState(self , rsthis: & QState) -> RetType;
}

  // proto:  QAbstractState * QState::initialState();
impl<'a> /*trait*/ QState_initialState<QAbstractState> for () {
  fn initialState(self , rsthis: & QState) -> QAbstractState {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QState12initialStateEv()};
    let mut ret = unsafe {C_ZNK6QState12initialStateEv(rsthis.qclsinst)};
    let mut ret1 = QAbstractState::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QState::~QState();
impl /*struct*/ QState {
  pub fn free<RetType, T: QState_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QState_free<RetType> {
  fn free(self , rsthis: & QState) -> RetType;
}

  // proto:  void QState::~QState();
impl<'a> /*trait*/ QState_free<()> for () {
  fn free(self , rsthis: & QState) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QStateD2Ev()};
     unsafe {C_ZN6QStateD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QState::assignProperty(QObject * object, const char * name, const QVariant & value);
impl /*struct*/ QState {
  pub fn assignProperty<RetType, T: QState_assignProperty<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.assignProperty(self);
    // return 1;
  }
}

pub trait QState_assignProperty<RetType> {
  fn assignProperty(self , rsthis: & QState) -> RetType;
}

  // proto:  void QState::assignProperty(QObject * object, const char * name, const QVariant & value);
impl<'a> /*trait*/ QState_assignProperty<()> for (&'a QObject, &'a  String, &'a QVariant) {
  fn assignProperty(self , rsthis: & QState) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QState14assignPropertyEP7QObjectPKcRK8QVariant()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *mut c_char;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {C_ZN6QState14assignPropertyEP7QObjectPKcRK8QVariant(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QState::QState(QState * parent);
impl /*struct*/ QState {
  pub fn new<T: QState_new>(value: T) -> QState {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QState_new {
  fn new(self) -> QState;
}

  // proto:  void QState::QState(QState * parent);
impl<'a> /*trait*/ QState_new for (&'a QState) {
  fn new(self) -> QState {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QStateC2EPS_()};
    let ctysz: c_int = unsafe{QState_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN6QStateC2EPS_(arg0)};
    let rsthis = QState{qbase: QAbstractState::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QMetaObject * QState::metaObject();
impl /*struct*/ QState {
  pub fn metaObject<RetType, T: QState_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QState_metaObject<RetType> {
  fn metaObject(self , rsthis: & QState) -> RetType;
}

  // proto:  const QMetaObject * QState::metaObject();
impl<'a> /*trait*/ QState_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QState) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QState10metaObjectEv()};
    let mut ret = unsafe {C_ZNK6QState10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QState::setErrorState(QAbstractState * state);
impl /*struct*/ QState {
  pub fn setErrorState<RetType, T: QState_setErrorState<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setErrorState(self);
    // return 1;
  }
}

pub trait QState_setErrorState<RetType> {
  fn setErrorState(self , rsthis: & QState) -> RetType;
}

  // proto:  void QState::setErrorState(QAbstractState * state);
impl<'a> /*trait*/ QState_setErrorState<()> for (&'a QAbstractState) {
  fn setErrorState(self , rsthis: & QState) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QState13setErrorStateEP14QAbstractState()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN6QState13setErrorStateEP14QAbstractState(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QState::addTransition(QAbstractTransition * transition);
impl /*struct*/ QState {
  pub fn addTransition<RetType, T: QState_addTransition<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addTransition(self);
    // return 1;
  }
}

pub trait QState_addTransition<RetType> {
  fn addTransition(self , rsthis: & QState) -> RetType;
}

  // proto:  void QState::addTransition(QAbstractTransition * transition);
impl<'a> /*trait*/ QState_addTransition<()> for (&'a QAbstractTransition) {
  fn addTransition(self , rsthis: & QState) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QState13addTransitionEP19QAbstractTransition()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN6QState13addTransitionEP19QAbstractTransition(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QState::removeTransition(QAbstractTransition * transition);
impl /*struct*/ QState {
  pub fn removeTransition<RetType, T: QState_removeTransition<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.removeTransition(self);
    // return 1;
  }
}

pub trait QState_removeTransition<RetType> {
  fn removeTransition(self , rsthis: & QState) -> RetType;
}

  // proto:  void QState::removeTransition(QAbstractTransition * transition);
impl<'a> /*trait*/ QState_removeTransition<()> for (&'a QAbstractTransition) {
  fn removeTransition(self , rsthis: & QState) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QState16removeTransitionEP19QAbstractTransition()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN6QState16removeTransitionEP19QAbstractTransition(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QSignalTransition * QState::addTransition(const QObject * sender, const char * signal, QAbstractState * target);
impl<'a> /*trait*/ QState_addTransition<QSignalTransition> for (&'a QObject, &'a  String, &'a QAbstractState) {
  fn addTransition(self , rsthis: & QState) -> QSignalTransition {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QState13addTransitionEPK7QObjectPKcP14QAbstractState()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *mut c_char;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN6QState13addTransitionEPK7QObjectPKcP14QAbstractState(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QSignalTransition::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QAbstractTransition * QState::addTransition(QAbstractState * target);
impl<'a> /*trait*/ QState_addTransition<QAbstractTransition> for (&'a QAbstractState) {
  fn addTransition(self , rsthis: & QState) -> QAbstractTransition {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QState13addTransitionEP14QAbstractState()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN6QState13addTransitionEP14QAbstractState(rsthis.qclsinst, arg0)};
    let mut ret1 = QAbstractTransition::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QList<QAbstractTransition *> QState::transitions();
impl /*struct*/ QState {
  pub fn transitions<RetType, T: QState_transitions<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.transitions(self);
    // return 1;
  }
}

pub trait QState_transitions<RetType> {
  fn transitions(self , rsthis: & QState) -> RetType;
}

  // proto:  QList<QAbstractTransition *> QState::transitions();
impl<'a> /*trait*/ QState_transitions<()> for () {
  fn transitions(self , rsthis: & QState) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QState11transitionsEv()};
     unsafe {C_ZNK6QState11transitionsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QState::setInitialState(QAbstractState * state);
impl /*struct*/ QState {
  pub fn setInitialState<RetType, T: QState_setInitialState<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setInitialState(self);
    // return 1;
  }
}

pub trait QState_setInitialState<RetType> {
  fn setInitialState(self , rsthis: & QState) -> RetType;
}

  // proto:  void QState::setInitialState(QAbstractState * state);
impl<'a> /*trait*/ QState_setInitialState<()> for (&'a QAbstractState) {
  fn setInitialState(self , rsthis: & QState) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QState15setInitialStateEP14QAbstractState()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN6QState15setInitialStateEP14QAbstractState(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

#[derive(Default)] // for QState_childModeChanged
pub struct QState_childModeChanged_signal{poi:u64}
impl /* struct */ QState {
  pub fn childModeChanged(&self) -> QState_childModeChanged_signal {
     return QState_childModeChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QState_childModeChanged_signal {
  pub fn connect<T: QState_childModeChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QState_childModeChanged_signal_connect {
  fn connect(self, sigthis: QState_childModeChanged_signal);
}

#[derive(Default)] // for QState_errorStateChanged
pub struct QState_errorStateChanged_signal{poi:u64}
impl /* struct */ QState {
  pub fn errorStateChanged(&self) -> QState_errorStateChanged_signal {
     return QState_errorStateChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QState_errorStateChanged_signal {
  pub fn connect<T: QState_errorStateChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QState_errorStateChanged_signal_connect {
  fn connect(self, sigthis: QState_errorStateChanged_signal);
}

#[derive(Default)] // for QState_finished
pub struct QState_finished_signal{poi:u64}
impl /* struct */ QState {
  pub fn finished(&self) -> QState_finished_signal {
     return QState_finished_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QState_finished_signal {
  pub fn connect<T: QState_finished_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QState_finished_signal_connect {
  fn connect(self, sigthis: QState_finished_signal);
}

#[derive(Default)] // for QState_propertiesAssigned
pub struct QState_propertiesAssigned_signal{poi:u64}
impl /* struct */ QState {
  pub fn propertiesAssigned(&self) -> QState_propertiesAssigned_signal {
     return QState_propertiesAssigned_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QState_propertiesAssigned_signal {
  pub fn connect<T: QState_propertiesAssigned_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QState_propertiesAssigned_signal_connect {
  fn connect(self, sigthis: QState_propertiesAssigned_signal);
}

#[derive(Default)] // for QState_initialStateChanged
pub struct QState_initialStateChanged_signal{poi:u64}
impl /* struct */ QState {
  pub fn initialStateChanged(&self) -> QState_initialStateChanged_signal {
     return QState_initialStateChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QState_initialStateChanged_signal {
  pub fn connect<T: QState_initialStateChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QState_initialStateChanged_signal_connect {
  fn connect(self, sigthis: QState_initialStateChanged_signal);
}

// <= body block end

