// auto generated, do not modify.
// created: Mon Dec 21 22:54:38 2015
// src-file: /QtCore/qabstracttransition.h
// dst-file: /src/core/qabstracttransition.rs
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
use super::qstate::QState; // 773
use super::qstatemachine::QStateMachine; // 773
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  QAbstractState * QAbstractTransition::targetState();
  fn _ZNK19QAbstractTransition11targetStateEv(qthis: *mut c_void);
  // proto:  QList<QAbstractState *> QAbstractTransition::targetStates();
  fn _ZNK19QAbstractTransition12targetStatesEv(qthis: *mut c_void);
  // proto:  QState * QAbstractTransition::sourceState();
  fn _ZNK19QAbstractTransition11sourceStateEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QAbstractTransition::~QAbstractTransition();
  fn _ZN19QAbstractTransitionD0Ev(qthis: *mut c_void);
  // proto:  void QAbstractTransition::QAbstractTransition(QState * sourceState);
  fn _ZN19QAbstractTransitionC1EP6QState(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QList<QAbstractAnimation *> QAbstractTransition::animations();
  fn _ZNK19QAbstractTransition10animationsEv(qthis: *mut c_void);
  // proto:  const QMetaObject * QAbstractTransition::metaObject();
  fn _ZNK19QAbstractTransition10metaObjectEv(qthis: *mut c_void);
  // proto:  QStateMachine * QAbstractTransition::machine();
  fn _ZNK19QAbstractTransition7machineEv(qthis: *mut c_void) -> *mut c_void;
} // <= ext block end

// body block begin =>
// class sizeof(QAbstractTransition)=1
pub struct QAbstractTransition {
  pub qclsinst: *mut c_void,
}

  // proto:  QAbstractState * QAbstractTransition::targetState();
impl /*struct*/ QAbstractTransition {
  pub fn targetState<RetType, T: QAbstractTransition_targetState<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.targetState(self);
    // return 1;
  }
}

pub trait QAbstractTransition_targetState<RetType> {
  fn targetState(self , rsthis: &mut QAbstractTransition) -> RetType;
}

  // proto:  QAbstractState * QAbstractTransition::targetState();
impl<'a> /*trait*/ QAbstractTransition_targetState<()> for () {
  fn targetState(self , rsthis: &mut QAbstractTransition) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QAbstractTransition11targetStateEv()};
     unsafe {_ZNK19QAbstractTransition11targetStateEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QList<QAbstractState *> QAbstractTransition::targetStates();
impl /*struct*/ QAbstractTransition {
  pub fn targetStates<RetType, T: QAbstractTransition_targetStates<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.targetStates(self);
    // return 1;
  }
}

pub trait QAbstractTransition_targetStates<RetType> {
  fn targetStates(self , rsthis: &mut QAbstractTransition) -> RetType;
}

  // proto:  QList<QAbstractState *> QAbstractTransition::targetStates();
impl<'a> /*trait*/ QAbstractTransition_targetStates<()> for () {
  fn targetStates(self , rsthis: &mut QAbstractTransition) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QAbstractTransition12targetStatesEv()};
     unsafe {_ZNK19QAbstractTransition12targetStatesEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QState * QAbstractTransition::sourceState();
impl /*struct*/ QAbstractTransition {
  pub fn sourceState<RetType, T: QAbstractTransition_sourceState<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.sourceState(self);
    // return 1;
  }
}

pub trait QAbstractTransition_sourceState<RetType> {
  fn sourceState(self , rsthis: &mut QAbstractTransition) -> RetType;
}

  // proto:  QState * QAbstractTransition::sourceState();
impl<'a> /*trait*/ QAbstractTransition_sourceState<QState> for () {
  fn sourceState(self , rsthis: &mut QAbstractTransition) -> QState {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QAbstractTransition11sourceStateEv()};
    let mut ret = unsafe {_ZNK19QAbstractTransition11sourceStateEv(rsthis.qclsinst)};
    let mut ret1 = QState{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QAbstractTransition::~QAbstractTransition();
impl /*struct*/ QAbstractTransition {
  pub fn FreeQAbstractTransition<RetType, T: QAbstractTransition_FreeQAbstractTransition<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQAbstractTransition(self);
    // return 1;
  }
}

pub trait QAbstractTransition_FreeQAbstractTransition<RetType> {
  fn FreeQAbstractTransition(self , rsthis: &mut QAbstractTransition) -> RetType;
}

  // proto:  void QAbstractTransition::~QAbstractTransition();
impl<'a> /*trait*/ QAbstractTransition_FreeQAbstractTransition<()> for () {
  fn FreeQAbstractTransition(self , rsthis: &mut QAbstractTransition) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QAbstractTransitionD0Ev()};
     unsafe {_ZN19QAbstractTransitionD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QAbstractTransition::QAbstractTransition(QState * sourceState);
impl /*struct*/ QAbstractTransition {
  pub fn NewQAbstractTransition<T: QAbstractTransition_NewQAbstractTransition>(value: T) -> QAbstractTransition {
    let rsthis = value.NewQAbstractTransition();
    return rsthis;
    // return 1;
  }
}

pub trait QAbstractTransition_NewQAbstractTransition {
  fn NewQAbstractTransition(self) -> QAbstractTransition;
}

  // proto:  void QAbstractTransition::QAbstractTransition(QState * sourceState);
impl<'a> /*trait*/ QAbstractTransition_NewQAbstractTransition for (QState) {
  fn NewQAbstractTransition(self) -> QAbstractTransition {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QAbstractTransitionC1EP6QState()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN19QAbstractTransitionC1EP6QState(qthis, arg0)};
    let rsthis = QAbstractTransition{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QList<QAbstractAnimation *> QAbstractTransition::animations();
impl /*struct*/ QAbstractTransition {
  pub fn animations<RetType, T: QAbstractTransition_animations<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.animations(self);
    // return 1;
  }
}

pub trait QAbstractTransition_animations<RetType> {
  fn animations(self , rsthis: &mut QAbstractTransition) -> RetType;
}

  // proto:  QList<QAbstractAnimation *> QAbstractTransition::animations();
impl<'a> /*trait*/ QAbstractTransition_animations<()> for () {
  fn animations(self , rsthis: &mut QAbstractTransition) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QAbstractTransition10animationsEv()};
     unsafe {_ZNK19QAbstractTransition10animationsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QAbstractTransition::metaObject();
impl /*struct*/ QAbstractTransition {
  pub fn metaObject<RetType, T: QAbstractTransition_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QAbstractTransition_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QAbstractTransition) -> RetType;
}

  // proto:  const QMetaObject * QAbstractTransition::metaObject();
impl<'a> /*trait*/ QAbstractTransition_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QAbstractTransition) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QAbstractTransition10metaObjectEv()};
     unsafe {_ZNK19QAbstractTransition10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QStateMachine * QAbstractTransition::machine();
impl /*struct*/ QAbstractTransition {
  pub fn machine<RetType, T: QAbstractTransition_machine<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.machine(self);
    // return 1;
  }
}

pub trait QAbstractTransition_machine<RetType> {
  fn machine(self , rsthis: &mut QAbstractTransition) -> RetType;
}

  // proto:  QStateMachine * QAbstractTransition::machine();
impl<'a> /*trait*/ QAbstractTransition_machine<QStateMachine> for () {
  fn machine(self , rsthis: &mut QAbstractTransition) -> QStateMachine {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QAbstractTransition7machineEv()};
    let mut ret = unsafe {_ZNK19QAbstractTransition7machineEv(rsthis.qclsinst)};
    let mut ret1 = QStateMachine{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// <= body block end

