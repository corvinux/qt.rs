// auto generated, do not modify.
// created: Sun Dec 27 22:52:02 2015
// src-file: /QtWidgets/qundostack.h
// dst-file: /src/widgets/qundostack.rs
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
use super::super::core::qobject::QObject; // 771
use std::ops::Deref;
use super::super::core::qstring::QString; // 771
// use super::qundostack::QUndoCommand; // 773
use super::qaction::QAction; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QUndoStack_Class_Size() -> c_int;
  // proto:  int QUndoStack::undoLimit();
  fn _ZNK10QUndoStack9undoLimitEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QUndoStack::undoTextChanged(const QString & undoText);
  fn _ZN10QUndoStack15undoTextChangedERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QUndoStack::redoTextChanged(const QString & redoText);
  fn _ZN10QUndoStack15redoTextChangedERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  const QUndoCommand * QUndoStack::command(int index);
  fn _ZNK10QUndoStack7commandEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  bool QUndoStack::canRedo();
  fn _ZNK10QUndoStack7canRedoEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  const QMetaObject * QUndoStack::metaObject();
  fn _ZNK10QUndoStack10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  QString QUndoStack::redoText();
  fn _ZNK10QUndoStack8redoTextEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QAction * QUndoStack::createUndoAction(QObject * parent, const QString & prefix);
  fn _ZNK10QUndoStack16createUndoActionEP7QObjectRK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  int QUndoStack::count();
  fn _ZNK10QUndoStack5countEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  QAction * QUndoStack::createRedoAction(QObject * parent, const QString & prefix);
  fn _ZNK10QUndoStack16createRedoActionEP7QObjectRK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  int QUndoStack::index();
  fn _ZNK10QUndoStack5indexEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QUndoStack::clear();
  fn _ZN10QUndoStack5clearEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QUndoStack::undo();
  fn _ZN10QUndoStack4undoEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QUndoStack::canRedoChanged(bool canRedo);
  fn _ZN10QUndoStack14canRedoChangedEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  bool QUndoStack::canUndo();
  fn _ZNK10QUndoStack7canUndoEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QUndoStack::isActive();
  fn _ZNK10QUndoStack8isActiveEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QUndoStack::cleanChanged(bool clean);
  fn _ZN10QUndoStack12cleanChangedEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QUndoStack::~QUndoStack();
  fn _ZN10QUndoStackD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QUndoStack::QUndoStack(QObject * parent);
  fn dector_ZN10QUndoStackC1EP7QObject(arg0: *mut c_void) -> *mut c_void;
  fn _ZN10QUndoStackC1EP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QUndoStack::indexChanged(int idx);
  fn _ZN10QUndoStack12indexChangedEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QUndoStack::canUndoChanged(bool canUndo);
  fn _ZN10QUndoStack14canUndoChangedEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  bool QUndoStack::isClean();
  fn _ZNK10QUndoStack7isCleanEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QUndoStack::redo();
  fn _ZN10QUndoStack4redoEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QUndoStack::beginMacro(const QString & text);
  fn _ZN10QUndoStack10beginMacroERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QUndoStack::setActive(bool active);
  fn _ZN10QUndoStack9setActiveEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  QString QUndoStack::undoText();
  fn _ZNK10QUndoStack8undoTextEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QUndoStack::cleanIndex();
  fn _ZNK10QUndoStack10cleanIndexEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QUndoStack::setIndex(int idx);
  fn _ZN10QUndoStack8setIndexEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QUndoStack::endMacro();
  fn _ZN10QUndoStack8endMacroEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QUndoStack::setUndoLimit(int limit);
  fn _ZN10QUndoStack12setUndoLimitEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QUndoStack::setClean();
  fn _ZN10QUndoStack8setCleanEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QUndoStack::QUndoStack(const QUndoStack & );
  fn dector_ZN10QUndoStackC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN10QUndoStackC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QString QUndoStack::text(int idx);
  fn _ZNK10QUndoStack4textEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  void QUndoStack::push(QUndoCommand * cmd);
  fn _ZN10QUndoStack4pushEP12QUndoCommand(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  fn QUndoCommand_Class_Size() -> c_int;
  // proto:  int QUndoCommand::id();
  fn _ZNK12QUndoCommand2idEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QUndoCommand::redo();
  fn _ZN12QUndoCommand4redoEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QUndoCommand::QUndoCommand(const QUndoCommand & );
  fn dector_ZN12QUndoCommandC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN12QUndoCommandC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QUndoCommand::QUndoCommand(QUndoCommand * parent);
  fn dector_ZN12QUndoCommandC1EPS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN12QUndoCommandC1EPS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QUndoCommand::undo();
  fn _ZN12QUndoCommand4undoEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QUndoCommand::QUndoCommand(const QString & text, QUndoCommand * parent);
  fn dector_ZN12QUndoCommandC1ERK7QStringPS_(arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  fn _ZN12QUndoCommandC1ERK7QStringPS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  bool QUndoCommand::mergeWith(const QUndoCommand * other);
  fn _ZN12QUndoCommand9mergeWithEPKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  QString QUndoCommand::text();
  fn _ZNK12QUndoCommand4textEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QUndoCommand::childCount();
  fn _ZNK12QUndoCommand10childCountEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  QString QUndoCommand::actionText();
  fn _ZNK12QUndoCommand10actionTextEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QUndoCommand::~QUndoCommand();
  fn _ZN12QUndoCommandD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  const QUndoCommand * QUndoCommand::child(int index);
  fn _ZNK12QUndoCommand5childEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  void QUndoCommand::setText(const QString & text);
  fn _ZN12QUndoCommand7setTextERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  fn QUndoStack_SlotProxy_connect__ZN10QUndoStack15undoTextChangedERK7QString(qthis: *mut c_void, fptr: *mut c_void);
  fn QUndoStack_SlotProxy_connect__ZN10QUndoStack15redoTextChangedERK7QString(qthis: *mut c_void, fptr: *mut c_void);
  fn QUndoStack_SlotProxy_connect__ZN10QUndoStack12cleanChangedEb(qthis: *mut c_void, fptr: *mut c_void);
  fn QUndoStack_SlotProxy_connect__ZN10QUndoStack14canRedoChangedEb(qthis: *mut c_void, fptr: *mut c_void);
  fn QUndoStack_SlotProxy_connect__ZN10QUndoStack12indexChangedEi(qthis: *mut c_void, fptr: *mut c_void);
  fn QUndoStack_SlotProxy_connect__ZN10QUndoStack14canUndoChangedEb(qthis: *mut c_void, fptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QUndoStack)=1
#[derive(Default)]
pub struct QUndoStack {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _redoTextChanged_1: QUndoStack_redoTextChanged_signal,
  pub _cleanChanged_1: QUndoStack_cleanChanged_signal,
  pub _canUndoChanged_1: QUndoStack_canUndoChanged_signal,
  pub _indexChanged_1: QUndoStack_indexChanged_signal,
  pub _canRedoChanged_1: QUndoStack_canRedoChanged_signal,
  pub _undoTextChanged_1: QUndoStack_undoTextChanged_signal,
}

// class sizeof(QUndoCommand)=16
#[derive(Default)]
pub struct QUndoCommand {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QUndoStack {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QUndoStack {
    return QUndoStack{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QUndoStack {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QUndoStack {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  int QUndoStack::undoLimit();
impl /*struct*/ QUndoStack {
  pub fn undoLimit<RetType, T: QUndoStack_undoLimit<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.undoLimit(self);
    // return 1;
  }
}

pub trait QUndoStack_undoLimit<RetType> {
  fn undoLimit(self , rsthis: & QUndoStack) -> RetType;
}

  // proto:  int QUndoStack::undoLimit();
impl<'a> /*trait*/ QUndoStack_undoLimit<i32> for () {
  fn undoLimit(self , rsthis: & QUndoStack) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QUndoStack9undoLimitEv()};
    let mut ret = unsafe {_ZNK10QUndoStack9undoLimitEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QUndoStack::undoTextChanged(const QString & undoText);
impl /*struct*/ QUndoStack {
  pub fn undoTextChanged<RetType, T: QUndoStack_undoTextChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.undoTextChanged(self);
    // return 1;
  }
}

pub trait QUndoStack_undoTextChanged<RetType> {
  fn undoTextChanged(self , rsthis: & QUndoStack) -> RetType;
}

  // proto:  void QUndoStack::undoTextChanged(const QString & undoText);
impl<'a> /*trait*/ QUndoStack_undoTextChanged<()> for (&'a QString) {
  fn undoTextChanged(self , rsthis: & QUndoStack) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoStack15undoTextChangedERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QUndoStack15undoTextChangedERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QUndoStack::redoTextChanged(const QString & redoText);
impl /*struct*/ QUndoStack {
  pub fn redoTextChanged<RetType, T: QUndoStack_redoTextChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.redoTextChanged(self);
    // return 1;
  }
}

pub trait QUndoStack_redoTextChanged<RetType> {
  fn redoTextChanged(self , rsthis: & QUndoStack) -> RetType;
}

  // proto:  void QUndoStack::redoTextChanged(const QString & redoText);
impl<'a> /*trait*/ QUndoStack_redoTextChanged<()> for (&'a QString) {
  fn redoTextChanged(self , rsthis: & QUndoStack) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoStack15redoTextChangedERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QUndoStack15redoTextChangedERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  const QUndoCommand * QUndoStack::command(int index);
impl /*struct*/ QUndoStack {
  pub fn command<RetType, T: QUndoStack_command<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.command(self);
    // return 1;
  }
}

pub trait QUndoStack_command<RetType> {
  fn command(self , rsthis: & QUndoStack) -> RetType;
}

  // proto:  const QUndoCommand * QUndoStack::command(int index);
impl<'a> /*trait*/ QUndoStack_command<QUndoCommand> for (i32) {
  fn command(self , rsthis: & QUndoStack) -> QUndoCommand {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QUndoStack7commandEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK10QUndoStack7commandEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QUndoCommand::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QUndoStack::canRedo();
impl /*struct*/ QUndoStack {
  pub fn canRedo<RetType, T: QUndoStack_canRedo<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.canRedo(self);
    // return 1;
  }
}

pub trait QUndoStack_canRedo<RetType> {
  fn canRedo(self , rsthis: & QUndoStack) -> RetType;
}

  // proto:  bool QUndoStack::canRedo();
impl<'a> /*trait*/ QUndoStack_canRedo<i8> for () {
  fn canRedo(self , rsthis: & QUndoStack) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QUndoStack7canRedoEv()};
    let mut ret = unsafe {_ZNK10QUndoStack7canRedoEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  const QMetaObject * QUndoStack::metaObject();
impl /*struct*/ QUndoStack {
  pub fn metaObject<RetType, T: QUndoStack_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QUndoStack_metaObject<RetType> {
  fn metaObject(self , rsthis: & QUndoStack) -> RetType;
}

  // proto:  const QMetaObject * QUndoStack::metaObject();
impl<'a> /*trait*/ QUndoStack_metaObject<()> for () {
  fn metaObject(self , rsthis: & QUndoStack) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QUndoStack10metaObjectEv()};
     unsafe {_ZNK10QUndoStack10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QString QUndoStack::redoText();
impl /*struct*/ QUndoStack {
  pub fn redoText<RetType, T: QUndoStack_redoText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.redoText(self);
    // return 1;
  }
}

pub trait QUndoStack_redoText<RetType> {
  fn redoText(self , rsthis: & QUndoStack) -> RetType;
}

  // proto:  QString QUndoStack::redoText();
impl<'a> /*trait*/ QUndoStack_redoText<QString> for () {
  fn redoText(self , rsthis: & QUndoStack) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QUndoStack8redoTextEv()};
    let mut ret = unsafe {_ZNK10QUndoStack8redoTextEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QAction * QUndoStack::createUndoAction(QObject * parent, const QString & prefix);
impl /*struct*/ QUndoStack {
  pub fn createUndoAction<RetType, T: QUndoStack_createUndoAction<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.createUndoAction(self);
    // return 1;
  }
}

pub trait QUndoStack_createUndoAction<RetType> {
  fn createUndoAction(self , rsthis: & QUndoStack) -> RetType;
}

  // proto:  QAction * QUndoStack::createUndoAction(QObject * parent, const QString & prefix);
impl<'a> /*trait*/ QUndoStack_createUndoAction<QAction> for (&'a QObject, &'a QString) {
  fn createUndoAction(self , rsthis: & QUndoStack) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QUndoStack16createUndoActionEP7QObjectRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK10QUndoStack16createUndoActionEP7QObjectRK7QString(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QAction::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QUndoStack::count();
impl /*struct*/ QUndoStack {
  pub fn count<RetType, T: QUndoStack_count<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.count(self);
    // return 1;
  }
}

pub trait QUndoStack_count<RetType> {
  fn count(self , rsthis: & QUndoStack) -> RetType;
}

  // proto:  int QUndoStack::count();
impl<'a> /*trait*/ QUndoStack_count<i32> for () {
  fn count(self , rsthis: & QUndoStack) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QUndoStack5countEv()};
    let mut ret = unsafe {_ZNK10QUndoStack5countEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QAction * QUndoStack::createRedoAction(QObject * parent, const QString & prefix);
impl /*struct*/ QUndoStack {
  pub fn createRedoAction<RetType, T: QUndoStack_createRedoAction<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.createRedoAction(self);
    // return 1;
  }
}

pub trait QUndoStack_createRedoAction<RetType> {
  fn createRedoAction(self , rsthis: & QUndoStack) -> RetType;
}

  // proto:  QAction * QUndoStack::createRedoAction(QObject * parent, const QString & prefix);
impl<'a> /*trait*/ QUndoStack_createRedoAction<QAction> for (&'a QObject, &'a QString) {
  fn createRedoAction(self , rsthis: & QUndoStack) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QUndoStack16createRedoActionEP7QObjectRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK10QUndoStack16createRedoActionEP7QObjectRK7QString(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QAction::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QUndoStack::index();
impl /*struct*/ QUndoStack {
  pub fn index<RetType, T: QUndoStack_index<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.index(self);
    // return 1;
  }
}

pub trait QUndoStack_index<RetType> {
  fn index(self , rsthis: & QUndoStack) -> RetType;
}

  // proto:  int QUndoStack::index();
impl<'a> /*trait*/ QUndoStack_index<i32> for () {
  fn index(self , rsthis: & QUndoStack) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QUndoStack5indexEv()};
    let mut ret = unsafe {_ZNK10QUndoStack5indexEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QUndoStack::clear();
impl /*struct*/ QUndoStack {
  pub fn clear<RetType, T: QUndoStack_clear<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clear(self);
    // return 1;
  }
}

pub trait QUndoStack_clear<RetType> {
  fn clear(self , rsthis: & QUndoStack) -> RetType;
}

  // proto:  void QUndoStack::clear();
impl<'a> /*trait*/ QUndoStack_clear<()> for () {
  fn clear(self , rsthis: & QUndoStack) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoStack5clearEv()};
     unsafe {_ZN10QUndoStack5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QUndoStack::undo();
impl /*struct*/ QUndoStack {
  pub fn undo<RetType, T: QUndoStack_undo<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.undo(self);
    // return 1;
  }
}

pub trait QUndoStack_undo<RetType> {
  fn undo(self , rsthis: & QUndoStack) -> RetType;
}

  // proto:  void QUndoStack::undo();
impl<'a> /*trait*/ QUndoStack_undo<()> for () {
  fn undo(self , rsthis: & QUndoStack) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoStack4undoEv()};
     unsafe {_ZN10QUndoStack4undoEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QUndoStack::canRedoChanged(bool canRedo);
impl /*struct*/ QUndoStack {
  pub fn canRedoChanged<RetType, T: QUndoStack_canRedoChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.canRedoChanged(self);
    // return 1;
  }
}

pub trait QUndoStack_canRedoChanged<RetType> {
  fn canRedoChanged(self , rsthis: & QUndoStack) -> RetType;
}

  // proto:  void QUndoStack::canRedoChanged(bool canRedo);
impl<'a> /*trait*/ QUndoStack_canRedoChanged<()> for (i8) {
  fn canRedoChanged(self , rsthis: & QUndoStack) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoStack14canRedoChangedEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN10QUndoStack14canRedoChangedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QUndoStack::canUndo();
impl /*struct*/ QUndoStack {
  pub fn canUndo<RetType, T: QUndoStack_canUndo<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.canUndo(self);
    // return 1;
  }
}

pub trait QUndoStack_canUndo<RetType> {
  fn canUndo(self , rsthis: & QUndoStack) -> RetType;
}

  // proto:  bool QUndoStack::canUndo();
impl<'a> /*trait*/ QUndoStack_canUndo<i8> for () {
  fn canUndo(self , rsthis: & QUndoStack) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QUndoStack7canUndoEv()};
    let mut ret = unsafe {_ZNK10QUndoStack7canUndoEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QUndoStack::isActive();
impl /*struct*/ QUndoStack {
  pub fn isActive<RetType, T: QUndoStack_isActive<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isActive(self);
    // return 1;
  }
}

pub trait QUndoStack_isActive<RetType> {
  fn isActive(self , rsthis: & QUndoStack) -> RetType;
}

  // proto:  bool QUndoStack::isActive();
impl<'a> /*trait*/ QUndoStack_isActive<i8> for () {
  fn isActive(self , rsthis: & QUndoStack) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QUndoStack8isActiveEv()};
    let mut ret = unsafe {_ZNK10QUndoStack8isActiveEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QUndoStack::cleanChanged(bool clean);
impl /*struct*/ QUndoStack {
  pub fn cleanChanged<RetType, T: QUndoStack_cleanChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.cleanChanged(self);
    // return 1;
  }
}

pub trait QUndoStack_cleanChanged<RetType> {
  fn cleanChanged(self , rsthis: & QUndoStack) -> RetType;
}

  // proto:  void QUndoStack::cleanChanged(bool clean);
impl<'a> /*trait*/ QUndoStack_cleanChanged<()> for (i8) {
  fn cleanChanged(self , rsthis: & QUndoStack) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoStack12cleanChangedEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN10QUndoStack12cleanChangedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QUndoStack::~QUndoStack();
impl /*struct*/ QUndoStack {
  pub fn Free<RetType, T: QUndoStack_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QUndoStack_Free<RetType> {
  fn Free(self , rsthis: & QUndoStack) -> RetType;
}

  // proto:  void QUndoStack::~QUndoStack();
impl<'a> /*trait*/ QUndoStack_Free<()> for () {
  fn Free(self , rsthis: & QUndoStack) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoStackD0Ev()};
     unsafe {_ZN10QUndoStackD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QUndoStack::QUndoStack(QObject * parent);
impl /*struct*/ QUndoStack {
  pub fn New<T: QUndoStack_New>(value: T) -> QUndoStack {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QUndoStack_New {
  fn New(self) -> QUndoStack;
}

  // proto:  void QUndoStack::QUndoStack(QObject * parent);
impl<'a> /*trait*/ QUndoStack_New for (&'a QObject) {
  fn New(self) -> QUndoStack {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoStackC1EP7QObject()};
    let ctysz: c_int = unsafe{QUndoStack_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN10QUndoStackC1EP7QObject(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN10QUndoStackC1EP7QObject(arg0)} as u64;
    let rsthis = QUndoStack{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QUndoStack::indexChanged(int idx);
impl /*struct*/ QUndoStack {
  pub fn indexChanged<RetType, T: QUndoStack_indexChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.indexChanged(self);
    // return 1;
  }
}

pub trait QUndoStack_indexChanged<RetType> {
  fn indexChanged(self , rsthis: & QUndoStack) -> RetType;
}

  // proto:  void QUndoStack::indexChanged(int idx);
impl<'a> /*trait*/ QUndoStack_indexChanged<()> for (i32) {
  fn indexChanged(self , rsthis: & QUndoStack) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoStack12indexChangedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QUndoStack12indexChangedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QUndoStack::canUndoChanged(bool canUndo);
impl /*struct*/ QUndoStack {
  pub fn canUndoChanged<RetType, T: QUndoStack_canUndoChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.canUndoChanged(self);
    // return 1;
  }
}

pub trait QUndoStack_canUndoChanged<RetType> {
  fn canUndoChanged(self , rsthis: & QUndoStack) -> RetType;
}

  // proto:  void QUndoStack::canUndoChanged(bool canUndo);
impl<'a> /*trait*/ QUndoStack_canUndoChanged<()> for (i8) {
  fn canUndoChanged(self , rsthis: & QUndoStack) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoStack14canUndoChangedEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN10QUndoStack14canUndoChangedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QUndoStack::isClean();
impl /*struct*/ QUndoStack {
  pub fn isClean<RetType, T: QUndoStack_isClean<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isClean(self);
    // return 1;
  }
}

pub trait QUndoStack_isClean<RetType> {
  fn isClean(self , rsthis: & QUndoStack) -> RetType;
}

  // proto:  bool QUndoStack::isClean();
impl<'a> /*trait*/ QUndoStack_isClean<i8> for () {
  fn isClean(self , rsthis: & QUndoStack) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QUndoStack7isCleanEv()};
    let mut ret = unsafe {_ZNK10QUndoStack7isCleanEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QUndoStack::redo();
impl /*struct*/ QUndoStack {
  pub fn redo<RetType, T: QUndoStack_redo<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.redo(self);
    // return 1;
  }
}

pub trait QUndoStack_redo<RetType> {
  fn redo(self , rsthis: & QUndoStack) -> RetType;
}

  // proto:  void QUndoStack::redo();
impl<'a> /*trait*/ QUndoStack_redo<()> for () {
  fn redo(self , rsthis: & QUndoStack) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoStack4redoEv()};
     unsafe {_ZN10QUndoStack4redoEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QUndoStack::beginMacro(const QString & text);
impl /*struct*/ QUndoStack {
  pub fn beginMacro<RetType, T: QUndoStack_beginMacro<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.beginMacro(self);
    // return 1;
  }
}

pub trait QUndoStack_beginMacro<RetType> {
  fn beginMacro(self , rsthis: & QUndoStack) -> RetType;
}

  // proto:  void QUndoStack::beginMacro(const QString & text);
impl<'a> /*trait*/ QUndoStack_beginMacro<()> for (&'a QString) {
  fn beginMacro(self , rsthis: & QUndoStack) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoStack10beginMacroERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QUndoStack10beginMacroERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QUndoStack::setActive(bool active);
impl /*struct*/ QUndoStack {
  pub fn setActive<RetType, T: QUndoStack_setActive<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setActive(self);
    // return 1;
  }
}

pub trait QUndoStack_setActive<RetType> {
  fn setActive(self , rsthis: & QUndoStack) -> RetType;
}

  // proto:  void QUndoStack::setActive(bool active);
impl<'a> /*trait*/ QUndoStack_setActive<()> for (i8) {
  fn setActive(self , rsthis: & QUndoStack) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoStack9setActiveEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN10QUndoStack9setActiveEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QUndoStack::undoText();
impl /*struct*/ QUndoStack {
  pub fn undoText<RetType, T: QUndoStack_undoText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.undoText(self);
    // return 1;
  }
}

pub trait QUndoStack_undoText<RetType> {
  fn undoText(self , rsthis: & QUndoStack) -> RetType;
}

  // proto:  QString QUndoStack::undoText();
impl<'a> /*trait*/ QUndoStack_undoText<QString> for () {
  fn undoText(self , rsthis: & QUndoStack) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QUndoStack8undoTextEv()};
    let mut ret = unsafe {_ZNK10QUndoStack8undoTextEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QUndoStack::cleanIndex();
impl /*struct*/ QUndoStack {
  pub fn cleanIndex<RetType, T: QUndoStack_cleanIndex<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.cleanIndex(self);
    // return 1;
  }
}

pub trait QUndoStack_cleanIndex<RetType> {
  fn cleanIndex(self , rsthis: & QUndoStack) -> RetType;
}

  // proto:  int QUndoStack::cleanIndex();
impl<'a> /*trait*/ QUndoStack_cleanIndex<i32> for () {
  fn cleanIndex(self , rsthis: & QUndoStack) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QUndoStack10cleanIndexEv()};
    let mut ret = unsafe {_ZNK10QUndoStack10cleanIndexEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QUndoStack::setIndex(int idx);
impl /*struct*/ QUndoStack {
  pub fn setIndex<RetType, T: QUndoStack_setIndex<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setIndex(self);
    // return 1;
  }
}

pub trait QUndoStack_setIndex<RetType> {
  fn setIndex(self , rsthis: & QUndoStack) -> RetType;
}

  // proto:  void QUndoStack::setIndex(int idx);
impl<'a> /*trait*/ QUndoStack_setIndex<()> for (i32) {
  fn setIndex(self , rsthis: & QUndoStack) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoStack8setIndexEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QUndoStack8setIndexEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QUndoStack::endMacro();
impl /*struct*/ QUndoStack {
  pub fn endMacro<RetType, T: QUndoStack_endMacro<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.endMacro(self);
    // return 1;
  }
}

pub trait QUndoStack_endMacro<RetType> {
  fn endMacro(self , rsthis: & QUndoStack) -> RetType;
}

  // proto:  void QUndoStack::endMacro();
impl<'a> /*trait*/ QUndoStack_endMacro<()> for () {
  fn endMacro(self , rsthis: & QUndoStack) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoStack8endMacroEv()};
     unsafe {_ZN10QUndoStack8endMacroEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QUndoStack::setUndoLimit(int limit);
impl /*struct*/ QUndoStack {
  pub fn setUndoLimit<RetType, T: QUndoStack_setUndoLimit<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setUndoLimit(self);
    // return 1;
  }
}

pub trait QUndoStack_setUndoLimit<RetType> {
  fn setUndoLimit(self , rsthis: & QUndoStack) -> RetType;
}

  // proto:  void QUndoStack::setUndoLimit(int limit);
impl<'a> /*trait*/ QUndoStack_setUndoLimit<()> for (i32) {
  fn setUndoLimit(self , rsthis: & QUndoStack) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoStack12setUndoLimitEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QUndoStack12setUndoLimitEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QUndoStack::setClean();
impl /*struct*/ QUndoStack {
  pub fn setClean<RetType, T: QUndoStack_setClean<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setClean(self);
    // return 1;
  }
}

pub trait QUndoStack_setClean<RetType> {
  fn setClean(self , rsthis: & QUndoStack) -> RetType;
}

  // proto:  void QUndoStack::setClean();
impl<'a> /*trait*/ QUndoStack_setClean<()> for () {
  fn setClean(self , rsthis: & QUndoStack) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoStack8setCleanEv()};
     unsafe {_ZN10QUndoStack8setCleanEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QUndoStack::QUndoStack(const QUndoStack & );
impl<'a> /*trait*/ QUndoStack_New for (&'a QUndoStack) {
  fn New(self) -> QUndoStack {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoStackC1ERKS_()};
    let ctysz: c_int = unsafe{QUndoStack_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN10QUndoStackC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN10QUndoStackC1ERKS_(arg0)} as u64;
    let rsthis = QUndoStack{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QString QUndoStack::text(int idx);
impl /*struct*/ QUndoStack {
  pub fn text<RetType, T: QUndoStack_text<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.text(self);
    // return 1;
  }
}

pub trait QUndoStack_text<RetType> {
  fn text(self , rsthis: & QUndoStack) -> RetType;
}

  // proto:  QString QUndoStack::text(int idx);
impl<'a> /*trait*/ QUndoStack_text<QString> for (i32) {
  fn text(self , rsthis: & QUndoStack) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QUndoStack4textEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK10QUndoStack4textEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QUndoStack::push(QUndoCommand * cmd);
impl /*struct*/ QUndoStack {
  pub fn push<RetType, T: QUndoStack_push<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.push(self);
    // return 1;
  }
}

pub trait QUndoStack_push<RetType> {
  fn push(self , rsthis: & QUndoStack) -> RetType;
}

  // proto:  void QUndoStack::push(QUndoCommand * cmd);
impl<'a> /*trait*/ QUndoStack_push<()> for (&'a QUndoCommand) {
  fn push(self , rsthis: & QUndoStack) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QUndoStack4pushEP12QUndoCommand()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QUndoStack4pushEP12QUndoCommand(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QUndoCommand {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QUndoCommand {
    return QUndoCommand{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  int QUndoCommand::id();
impl /*struct*/ QUndoCommand {
  pub fn id<RetType, T: QUndoCommand_id<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.id(self);
    // return 1;
  }
}

pub trait QUndoCommand_id<RetType> {
  fn id(self , rsthis: & QUndoCommand) -> RetType;
}

  // proto:  int QUndoCommand::id();
impl<'a> /*trait*/ QUndoCommand_id<i32> for () {
  fn id(self , rsthis: & QUndoCommand) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QUndoCommand2idEv()};
    let mut ret = unsafe {_ZNK12QUndoCommand2idEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QUndoCommand::redo();
impl /*struct*/ QUndoCommand {
  pub fn redo<RetType, T: QUndoCommand_redo<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.redo(self);
    // return 1;
  }
}

pub trait QUndoCommand_redo<RetType> {
  fn redo(self , rsthis: & QUndoCommand) -> RetType;
}

  // proto:  void QUndoCommand::redo();
impl<'a> /*trait*/ QUndoCommand_redo<()> for () {
  fn redo(self , rsthis: & QUndoCommand) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QUndoCommand4redoEv()};
     unsafe {_ZN12QUndoCommand4redoEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QUndoCommand::QUndoCommand(const QUndoCommand & );
impl /*struct*/ QUndoCommand {
  pub fn New<T: QUndoCommand_New>(value: T) -> QUndoCommand {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QUndoCommand_New {
  fn New(self) -> QUndoCommand;
}

  // proto:  void QUndoCommand::QUndoCommand(const QUndoCommand & );
impl<'a> /*trait*/ QUndoCommand_New for (&'a QUndoCommand) {
  fn New(self) -> QUndoCommand {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QUndoCommandC1ERKS_()};
    let ctysz: c_int = unsafe{QUndoCommand_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN12QUndoCommandC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN12QUndoCommandC1ERKS_(arg0)} as u64;
    let rsthis = QUndoCommand{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QUndoCommand::undo();
impl /*struct*/ QUndoCommand {
  pub fn undo<RetType, T: QUndoCommand_undo<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.undo(self);
    // return 1;
  }
}

pub trait QUndoCommand_undo<RetType> {
  fn undo(self , rsthis: & QUndoCommand) -> RetType;
}

  // proto:  void QUndoCommand::undo();
impl<'a> /*trait*/ QUndoCommand_undo<()> for () {
  fn undo(self , rsthis: & QUndoCommand) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QUndoCommand4undoEv()};
     unsafe {_ZN12QUndoCommand4undoEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QUndoCommand::QUndoCommand(const QString & text, QUndoCommand * parent);
impl<'a> /*trait*/ QUndoCommand_New for (&'a QString, &'a QUndoCommand) {
  fn New(self) -> QUndoCommand {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QUndoCommandC1ERK7QStringPS_()};
    let ctysz: c_int = unsafe{QUndoCommand_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    // unsafe {_ZN12QUndoCommandC1ERK7QStringPS_(qthis, arg0, arg1)};
    let qthis: u64 = unsafe {dector_ZN12QUndoCommandC1ERK7QStringPS_(arg0, arg1)} as u64;
    let rsthis = QUndoCommand{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QUndoCommand::mergeWith(const QUndoCommand * other);
impl /*struct*/ QUndoCommand {
  pub fn mergeWith<RetType, T: QUndoCommand_mergeWith<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mergeWith(self);
    // return 1;
  }
}

pub trait QUndoCommand_mergeWith<RetType> {
  fn mergeWith(self , rsthis: & QUndoCommand) -> RetType;
}

  // proto:  bool QUndoCommand::mergeWith(const QUndoCommand * other);
impl<'a> /*trait*/ QUndoCommand_mergeWith<i8> for (&'a QUndoCommand) {
  fn mergeWith(self , rsthis: & QUndoCommand) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QUndoCommand9mergeWithEPKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN12QUndoCommand9mergeWithEPKS_(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QString QUndoCommand::text();
impl /*struct*/ QUndoCommand {
  pub fn text<RetType, T: QUndoCommand_text<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.text(self);
    // return 1;
  }
}

pub trait QUndoCommand_text<RetType> {
  fn text(self , rsthis: & QUndoCommand) -> RetType;
}

  // proto:  QString QUndoCommand::text();
impl<'a> /*trait*/ QUndoCommand_text<QString> for () {
  fn text(self , rsthis: & QUndoCommand) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QUndoCommand4textEv()};
    let mut ret = unsafe {_ZNK12QUndoCommand4textEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QUndoCommand::childCount();
impl /*struct*/ QUndoCommand {
  pub fn childCount<RetType, T: QUndoCommand_childCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.childCount(self);
    // return 1;
  }
}

pub trait QUndoCommand_childCount<RetType> {
  fn childCount(self , rsthis: & QUndoCommand) -> RetType;
}

  // proto:  int QUndoCommand::childCount();
impl<'a> /*trait*/ QUndoCommand_childCount<i32> for () {
  fn childCount(self , rsthis: & QUndoCommand) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QUndoCommand10childCountEv()};
    let mut ret = unsafe {_ZNK12QUndoCommand10childCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QString QUndoCommand::actionText();
impl /*struct*/ QUndoCommand {
  pub fn actionText<RetType, T: QUndoCommand_actionText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.actionText(self);
    // return 1;
  }
}

pub trait QUndoCommand_actionText<RetType> {
  fn actionText(self , rsthis: & QUndoCommand) -> RetType;
}

  // proto:  QString QUndoCommand::actionText();
impl<'a> /*trait*/ QUndoCommand_actionText<QString> for () {
  fn actionText(self , rsthis: & QUndoCommand) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QUndoCommand10actionTextEv()};
    let mut ret = unsafe {_ZNK12QUndoCommand10actionTextEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QUndoCommand::~QUndoCommand();
impl /*struct*/ QUndoCommand {
  pub fn Free<RetType, T: QUndoCommand_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QUndoCommand_Free<RetType> {
  fn Free(self , rsthis: & QUndoCommand) -> RetType;
}

  // proto:  void QUndoCommand::~QUndoCommand();
impl<'a> /*trait*/ QUndoCommand_Free<()> for () {
  fn Free(self , rsthis: & QUndoCommand) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QUndoCommandD0Ev()};
     unsafe {_ZN12QUndoCommandD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  const QUndoCommand * QUndoCommand::child(int index);
impl /*struct*/ QUndoCommand {
  pub fn child<RetType, T: QUndoCommand_child<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.child(self);
    // return 1;
  }
}

pub trait QUndoCommand_child<RetType> {
  fn child(self , rsthis: & QUndoCommand) -> RetType;
}

  // proto:  const QUndoCommand * QUndoCommand::child(int index);
impl<'a> /*trait*/ QUndoCommand_child<QUndoCommand> for (i32) {
  fn child(self , rsthis: & QUndoCommand) -> QUndoCommand {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QUndoCommand5childEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK12QUndoCommand5childEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QUndoCommand::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QUndoCommand::setText(const QString & text);
impl /*struct*/ QUndoCommand {
  pub fn setText<RetType, T: QUndoCommand_setText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setText(self);
    // return 1;
  }
}

pub trait QUndoCommand_setText<RetType> {
  fn setText(self , rsthis: & QUndoCommand) -> RetType;
}

  // proto:  void QUndoCommand::setText(const QString & text);
impl<'a> /*trait*/ QUndoCommand_setText<()> for (&'a QString) {
  fn setText(self , rsthis: & QUndoCommand) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QUndoCommand7setTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QUndoCommand7setTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

#[derive(Default)] // for QUndoStack_redoTextChanged
pub struct QUndoStack_redoTextChanged_signal{poi:u64}
impl /* struct */ QUndoStack {
  pub fn redoTextChanged_1(self) -> QUndoStack_redoTextChanged_signal {
     return QUndoStack_redoTextChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QUndoStack_redoTextChanged_signal {
  pub fn connect<T: QUndoStack_redoTextChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QUndoStack_redoTextChanged_signal_connect {
  fn connect(self, sigthis: QUndoStack_redoTextChanged_signal);
}

#[derive(Default)] // for QUndoStack_cleanChanged
pub struct QUndoStack_cleanChanged_signal{poi:u64}
impl /* struct */ QUndoStack {
  pub fn cleanChanged_1(self) -> QUndoStack_cleanChanged_signal {
     return QUndoStack_cleanChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QUndoStack_cleanChanged_signal {
  pub fn connect<T: QUndoStack_cleanChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QUndoStack_cleanChanged_signal_connect {
  fn connect(self, sigthis: QUndoStack_cleanChanged_signal);
}

#[derive(Default)] // for QUndoStack_canUndoChanged
pub struct QUndoStack_canUndoChanged_signal{poi:u64}
impl /* struct */ QUndoStack {
  pub fn canUndoChanged_1(self) -> QUndoStack_canUndoChanged_signal {
     return QUndoStack_canUndoChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QUndoStack_canUndoChanged_signal {
  pub fn connect<T: QUndoStack_canUndoChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QUndoStack_canUndoChanged_signal_connect {
  fn connect(self, sigthis: QUndoStack_canUndoChanged_signal);
}

#[derive(Default)] // for QUndoStack_indexChanged
pub struct QUndoStack_indexChanged_signal{poi:u64}
impl /* struct */ QUndoStack {
  pub fn indexChanged_1(self) -> QUndoStack_indexChanged_signal {
     return QUndoStack_indexChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QUndoStack_indexChanged_signal {
  pub fn connect<T: QUndoStack_indexChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QUndoStack_indexChanged_signal_connect {
  fn connect(self, sigthis: QUndoStack_indexChanged_signal);
}

#[derive(Default)] // for QUndoStack_canRedoChanged
pub struct QUndoStack_canRedoChanged_signal{poi:u64}
impl /* struct */ QUndoStack {
  pub fn canRedoChanged_1(self) -> QUndoStack_canRedoChanged_signal {
     return QUndoStack_canRedoChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QUndoStack_canRedoChanged_signal {
  pub fn connect<T: QUndoStack_canRedoChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QUndoStack_canRedoChanged_signal_connect {
  fn connect(self, sigthis: QUndoStack_canRedoChanged_signal);
}

#[derive(Default)] // for QUndoStack_undoTextChanged
pub struct QUndoStack_undoTextChanged_signal{poi:u64}
impl /* struct */ QUndoStack {
  pub fn undoTextChanged_1(self) -> QUndoStack_undoTextChanged_signal {
     return QUndoStack_undoTextChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QUndoStack_undoTextChanged_signal {
  pub fn connect<T: QUndoStack_undoTextChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QUndoStack_undoTextChanged_signal_connect {
  fn connect(self, sigthis: QUndoStack_undoTextChanged_signal);
}

// undoTextChanged(const class QString &)
extern fn QUndoStack_undoTextChanged_signal_connect_cb_0(arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
}
impl /* trait */ QUndoStack_undoTextChanged_signal_connect for (extern fn(QString)) {
  fn connect(self, sigthis: QUndoStack_undoTextChanged_signal) {
    // do smth...
    unsafe {QUndoStack_SlotProxy_connect__ZN10QUndoStack15undoTextChangedERK7QString(sigthis.poi as *mut c_void, QUndoStack_undoTextChanged_signal_connect_cb_0 as *mut c_void)};
  }
}
// redoTextChanged(const class QString &)
extern fn QUndoStack_redoTextChanged_signal_connect_cb_1(arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
}
impl /* trait */ QUndoStack_redoTextChanged_signal_connect for (extern fn(QString)) {
  fn connect(self, sigthis: QUndoStack_redoTextChanged_signal) {
    // do smth...
    unsafe {QUndoStack_SlotProxy_connect__ZN10QUndoStack15redoTextChangedERK7QString(sigthis.poi as *mut c_void, QUndoStack_redoTextChanged_signal_connect_cb_1 as *mut c_void)};
  }
}
// cleanChanged(_Bool)
extern fn QUndoStack_cleanChanged_signal_connect_cb_2(arg0: c_char) {
  println!("{}:{}", file!(), line!());
}
impl /* trait */ QUndoStack_cleanChanged_signal_connect for (extern fn(i8)) {
  fn connect(self, sigthis: QUndoStack_cleanChanged_signal) {
    // do smth...
    unsafe {QUndoStack_SlotProxy_connect__ZN10QUndoStack12cleanChangedEb(sigthis.poi as *mut c_void, QUndoStack_cleanChanged_signal_connect_cb_2 as *mut c_void)};
  }
}
// canRedoChanged(_Bool)
extern fn QUndoStack_canRedoChanged_signal_connect_cb_3(arg0: c_char) {
  println!("{}:{}", file!(), line!());
}
impl /* trait */ QUndoStack_canRedoChanged_signal_connect for (extern fn(i8)) {
  fn connect(self, sigthis: QUndoStack_canRedoChanged_signal) {
    // do smth...
    unsafe {QUndoStack_SlotProxy_connect__ZN10QUndoStack14canRedoChangedEb(sigthis.poi as *mut c_void, QUndoStack_canRedoChanged_signal_connect_cb_3 as *mut c_void)};
  }
}
// indexChanged(int)
extern fn QUndoStack_indexChanged_signal_connect_cb_4(arg0: c_int) {
  println!("{}:{}", file!(), line!());
}
impl /* trait */ QUndoStack_indexChanged_signal_connect for (extern fn(i32)) {
  fn connect(self, sigthis: QUndoStack_indexChanged_signal) {
    // do smth...
    unsafe {QUndoStack_SlotProxy_connect__ZN10QUndoStack12indexChangedEi(sigthis.poi as *mut c_void, QUndoStack_indexChanged_signal_connect_cb_4 as *mut c_void)};
  }
}
// canUndoChanged(_Bool)
extern fn QUndoStack_canUndoChanged_signal_connect_cb_5(arg0: c_char) {
  println!("{}:{}", file!(), line!());
}
impl /* trait */ QUndoStack_canUndoChanged_signal_connect for (extern fn(i8)) {
  fn connect(self, sigthis: QUndoStack_canUndoChanged_signal) {
    // do smth...
    unsafe {QUndoStack_SlotProxy_connect__ZN10QUndoStack14canUndoChangedEb(sigthis.poi as *mut c_void, QUndoStack_canUndoChanged_signal_connect_cb_5 as *mut c_void)};
  }
}
// <= body block end

