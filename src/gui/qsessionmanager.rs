// auto generated, do not modify.
// created: Mon Feb 22 23:57:02 2016
// src-file: /QtGui/qsessionmanager.h
// dst-file: /src/gui/qsessionmanager.rs
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
use super::super::core::qobject::*; // 771
use std::ops::Deref;
use super::super::core::qstring::*; // 771
use super::super::core::qstringlist::*; // 771
use super::super::core::qobjectdefs::*; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QSessionManager_Class_Size() -> c_int;
  // proto:  QString QSessionManager::sessionId();
  fn C_ZNK15QSessionManager9sessionIdEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QString QSessionManager::sessionKey();
  fn C_ZNK15QSessionManager10sessionKeyEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QSessionManager::setRestartCommand(const QStringList & );
  fn C_ZN15QSessionManager17setRestartCommandERK11QStringList(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  const QMetaObject * QSessionManager::metaObject();
  fn C_ZNK15QSessionManager10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QSessionManager::allowsErrorInteraction();
  fn C_ZN15QSessionManager22allowsErrorInteractionEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QStringList QSessionManager::restartCommand();
  fn C_ZNK15QSessionManager14restartCommandEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QSessionManager::requestPhase2();
  fn C_ZN15QSessionManager13requestPhase2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  bool QSessionManager::isPhase2();
  fn C_ZNK15QSessionManager8isPhase2Ev(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QSessionManager::release();
  fn C_ZN15QSessionManager7releaseEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QSessionManager::setManagerProperty(const QString & name, const QString & value);
  fn C_ZN15QSessionManager18setManagerPropertyERK7QStringS2_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QSessionManager::setManagerProperty(const QString & name, const QStringList & value);
  fn C_ZN15QSessionManager18setManagerPropertyERK7QStringRK11QStringList(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  QStringList QSessionManager::discardCommand();
  fn C_ZNK15QSessionManager14discardCommandEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QSessionManager::cancel();
  fn C_ZN15QSessionManager6cancelEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QSessionManager::setDiscardCommand(const QStringList & );
  fn C_ZN15QSessionManager17setDiscardCommandERK11QStringList(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QSessionManager::allowsInteraction();
  fn C_ZN15QSessionManager17allowsInteractionEv(qthis: u64 /* *mut c_void*/) -> c_char;
} // <= ext block end

// body block begin =>
// class sizeof(QSessionManager)=1
#[derive(Default)]
pub struct QSessionManager {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QSessionManager {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QSessionManager {
    return QSessionManager{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QSessionManager {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QSessionManager {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  QString QSessionManager::sessionId();
impl /*struct*/ QSessionManager {
  pub fn sessionId<RetType, T: QSessionManager_sessionId<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sessionId(self);
    // return 1;
  }
}

pub trait QSessionManager_sessionId<RetType> {
  fn sessionId(self , rsthis: & QSessionManager) -> RetType;
}

  // proto:  QString QSessionManager::sessionId();
impl<'a> /*trait*/ QSessionManager_sessionId<QString> for () {
  fn sessionId(self , rsthis: & QSessionManager) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSessionManager9sessionIdEv()};
    let mut ret = unsafe {C_ZNK15QSessionManager9sessionIdEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QSessionManager::sessionKey();
impl /*struct*/ QSessionManager {
  pub fn sessionKey<RetType, T: QSessionManager_sessionKey<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sessionKey(self);
    // return 1;
  }
}

pub trait QSessionManager_sessionKey<RetType> {
  fn sessionKey(self , rsthis: & QSessionManager) -> RetType;
}

  // proto:  QString QSessionManager::sessionKey();
impl<'a> /*trait*/ QSessionManager_sessionKey<QString> for () {
  fn sessionKey(self , rsthis: & QSessionManager) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSessionManager10sessionKeyEv()};
    let mut ret = unsafe {C_ZNK15QSessionManager10sessionKeyEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QSessionManager::setRestartCommand(const QStringList & );
impl /*struct*/ QSessionManager {
  pub fn setRestartCommand<RetType, T: QSessionManager_setRestartCommand<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setRestartCommand(self);
    // return 1;
  }
}

pub trait QSessionManager_setRestartCommand<RetType> {
  fn setRestartCommand(self , rsthis: & QSessionManager) -> RetType;
}

  // proto:  void QSessionManager::setRestartCommand(const QStringList & );
impl<'a> /*trait*/ QSessionManager_setRestartCommand<()> for (&'a QStringList) {
  fn setRestartCommand(self , rsthis: & QSessionManager) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSessionManager17setRestartCommandERK11QStringList()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN15QSessionManager17setRestartCommandERK11QStringList(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QSessionManager::metaObject();
impl /*struct*/ QSessionManager {
  pub fn metaObject<RetType, T: QSessionManager_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QSessionManager_metaObject<RetType> {
  fn metaObject(self , rsthis: & QSessionManager) -> RetType;
}

  // proto:  const QMetaObject * QSessionManager::metaObject();
impl<'a> /*trait*/ QSessionManager_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QSessionManager) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSessionManager10metaObjectEv()};
    let mut ret = unsafe {C_ZNK15QSessionManager10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QSessionManager::allowsErrorInteraction();
impl /*struct*/ QSessionManager {
  pub fn allowsErrorInteraction<RetType, T: QSessionManager_allowsErrorInteraction<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.allowsErrorInteraction(self);
    // return 1;
  }
}

pub trait QSessionManager_allowsErrorInteraction<RetType> {
  fn allowsErrorInteraction(self , rsthis: & QSessionManager) -> RetType;
}

  // proto:  bool QSessionManager::allowsErrorInteraction();
impl<'a> /*trait*/ QSessionManager_allowsErrorInteraction<i8> for () {
  fn allowsErrorInteraction(self , rsthis: & QSessionManager) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSessionManager22allowsErrorInteractionEv()};
    let mut ret = unsafe {C_ZN15QSessionManager22allowsErrorInteractionEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  QStringList QSessionManager::restartCommand();
impl /*struct*/ QSessionManager {
  pub fn restartCommand<RetType, T: QSessionManager_restartCommand<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.restartCommand(self);
    // return 1;
  }
}

pub trait QSessionManager_restartCommand<RetType> {
  fn restartCommand(self , rsthis: & QSessionManager) -> RetType;
}

  // proto:  QStringList QSessionManager::restartCommand();
impl<'a> /*trait*/ QSessionManager_restartCommand<QStringList> for () {
  fn restartCommand(self , rsthis: & QSessionManager) -> QStringList {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSessionManager14restartCommandEv()};
    let mut ret = unsafe {C_ZNK15QSessionManager14restartCommandEv(rsthis.qclsinst)};
    let mut ret1 = QStringList::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QSessionManager::requestPhase2();
impl /*struct*/ QSessionManager {
  pub fn requestPhase2<RetType, T: QSessionManager_requestPhase2<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.requestPhase2(self);
    // return 1;
  }
}

pub trait QSessionManager_requestPhase2<RetType> {
  fn requestPhase2(self , rsthis: & QSessionManager) -> RetType;
}

  // proto:  void QSessionManager::requestPhase2();
impl<'a> /*trait*/ QSessionManager_requestPhase2<()> for () {
  fn requestPhase2(self , rsthis: & QSessionManager) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSessionManager13requestPhase2Ev()};
     unsafe {C_ZN15QSessionManager13requestPhase2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QSessionManager::isPhase2();
impl /*struct*/ QSessionManager {
  pub fn isPhase2<RetType, T: QSessionManager_isPhase2<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isPhase2(self);
    // return 1;
  }
}

pub trait QSessionManager_isPhase2<RetType> {
  fn isPhase2(self , rsthis: & QSessionManager) -> RetType;
}

  // proto:  bool QSessionManager::isPhase2();
impl<'a> /*trait*/ QSessionManager_isPhase2<i8> for () {
  fn isPhase2(self , rsthis: & QSessionManager) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSessionManager8isPhase2Ev()};
    let mut ret = unsafe {C_ZNK15QSessionManager8isPhase2Ev(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QSessionManager::release();
impl /*struct*/ QSessionManager {
  pub fn release<RetType, T: QSessionManager_release<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.release(self);
    // return 1;
  }
}

pub trait QSessionManager_release<RetType> {
  fn release(self , rsthis: & QSessionManager) -> RetType;
}

  // proto:  void QSessionManager::release();
impl<'a> /*trait*/ QSessionManager_release<()> for () {
  fn release(self , rsthis: & QSessionManager) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSessionManager7releaseEv()};
     unsafe {C_ZN15QSessionManager7releaseEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QSessionManager::setManagerProperty(const QString & name, const QString & value);
impl /*struct*/ QSessionManager {
  pub fn setManagerProperty<RetType, T: QSessionManager_setManagerProperty<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setManagerProperty(self);
    // return 1;
  }
}

pub trait QSessionManager_setManagerProperty<RetType> {
  fn setManagerProperty(self , rsthis: & QSessionManager) -> RetType;
}

  // proto:  void QSessionManager::setManagerProperty(const QString & name, const QString & value);
impl<'a> /*trait*/ QSessionManager_setManagerProperty<()> for (&'a QString, &'a QString) {
  fn setManagerProperty(self , rsthis: & QSessionManager) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSessionManager18setManagerPropertyERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {C_ZN15QSessionManager18setManagerPropertyERK7QStringS2_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QSessionManager::setManagerProperty(const QString & name, const QStringList & value);
impl<'a> /*trait*/ QSessionManager_setManagerProperty<()> for (&'a QString, &'a QStringList) {
  fn setManagerProperty(self , rsthis: & QSessionManager) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSessionManager18setManagerPropertyERK7QStringRK11QStringList()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {C_ZN15QSessionManager18setManagerPropertyERK7QStringRK11QStringList(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QStringList QSessionManager::discardCommand();
impl /*struct*/ QSessionManager {
  pub fn discardCommand<RetType, T: QSessionManager_discardCommand<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.discardCommand(self);
    // return 1;
  }
}

pub trait QSessionManager_discardCommand<RetType> {
  fn discardCommand(self , rsthis: & QSessionManager) -> RetType;
}

  // proto:  QStringList QSessionManager::discardCommand();
impl<'a> /*trait*/ QSessionManager_discardCommand<QStringList> for () {
  fn discardCommand(self , rsthis: & QSessionManager) -> QStringList {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSessionManager14discardCommandEv()};
    let mut ret = unsafe {C_ZNK15QSessionManager14discardCommandEv(rsthis.qclsinst)};
    let mut ret1 = QStringList::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QSessionManager::cancel();
impl /*struct*/ QSessionManager {
  pub fn cancel<RetType, T: QSessionManager_cancel<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.cancel(self);
    // return 1;
  }
}

pub trait QSessionManager_cancel<RetType> {
  fn cancel(self , rsthis: & QSessionManager) -> RetType;
}

  // proto:  void QSessionManager::cancel();
impl<'a> /*trait*/ QSessionManager_cancel<()> for () {
  fn cancel(self , rsthis: & QSessionManager) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSessionManager6cancelEv()};
     unsafe {C_ZN15QSessionManager6cancelEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QSessionManager::setDiscardCommand(const QStringList & );
impl /*struct*/ QSessionManager {
  pub fn setDiscardCommand<RetType, T: QSessionManager_setDiscardCommand<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDiscardCommand(self);
    // return 1;
  }
}

pub trait QSessionManager_setDiscardCommand<RetType> {
  fn setDiscardCommand(self , rsthis: & QSessionManager) -> RetType;
}

  // proto:  void QSessionManager::setDiscardCommand(const QStringList & );
impl<'a> /*trait*/ QSessionManager_setDiscardCommand<()> for (&'a QStringList) {
  fn setDiscardCommand(self , rsthis: & QSessionManager) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSessionManager17setDiscardCommandERK11QStringList()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN15QSessionManager17setDiscardCommandERK11QStringList(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QSessionManager::allowsInteraction();
impl /*struct*/ QSessionManager {
  pub fn allowsInteraction<RetType, T: QSessionManager_allowsInteraction<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.allowsInteraction(self);
    // return 1;
  }
}

pub trait QSessionManager_allowsInteraction<RetType> {
  fn allowsInteraction(self , rsthis: & QSessionManager) -> RetType;
}

  // proto:  bool QSessionManager::allowsInteraction();
impl<'a> /*trait*/ QSessionManager_allowsInteraction<i8> for () {
  fn allowsInteraction(self , rsthis: & QSessionManager) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSessionManager17allowsInteractionEv()};
    let mut ret = unsafe {C_ZN15QSessionManager17allowsInteractionEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

// <= body block end

