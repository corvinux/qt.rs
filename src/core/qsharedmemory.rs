// auto generated, do not modify.
// created: Mon Feb 22 23:57:02 2016
// src-file: /QtCore/qsharedmemory.h
// dst-file: /src/core/qsharedmemory.rs
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
use super::qobject::*; // 773
use std::ops::Deref;
use super::qstring::*; // 773
use super::qobjectdefs::*; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QSharedMemory_Class_Size() -> c_int;
  // proto:  int QSharedMemory::size();
  fn C_ZNK13QSharedMemory4sizeEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QSharedMemory::setNativeKey(const QString & key);
  fn C_ZN13QSharedMemory12setNativeKeyERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QSharedMemory::QSharedMemory(const QString & key, QObject * parent);
  fn C_ZN13QSharedMemoryC2ERK7QStringP7QObject(arg0: *mut c_void, arg1: *mut c_void) -> u64;
  // proto:  QString QSharedMemory::errorString();
  fn C_ZNK13QSharedMemory11errorStringEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QSharedMemory::setKey(const QString & key);
  fn C_ZN13QSharedMemory6setKeyERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QString QSharedMemory::key();
  fn C_ZNK13QSharedMemory3keyEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  const void * QSharedMemory::constData();
  fn C_ZNK13QSharedMemory9constDataEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void * QSharedMemory::data();
  fn C_ZN13QSharedMemory4dataEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QSharedMemory::isAttached();
  fn C_ZNK13QSharedMemory10isAttachedEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QSharedMemory::lock();
  fn C_ZN13QSharedMemory4lockEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QSharedMemory::~QSharedMemory();
  fn C_ZN13QSharedMemoryD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  bool QSharedMemory::unlock();
  fn C_ZN13QSharedMemory6unlockEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QSharedMemory::detach();
  fn C_ZN13QSharedMemory6detachEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QString QSharedMemory::nativeKey();
  fn C_ZNK13QSharedMemory9nativeKeyEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  const QMetaObject * QSharedMemory::metaObject();
  fn C_ZNK13QSharedMemory10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QSharedMemory::QSharedMemory(QObject * parent);
  fn C_ZN13QSharedMemoryC2EP7QObject(arg0: *mut c_void) -> u64;
} // <= ext block end

// body block begin =>
// class sizeof(QSharedMemory)=1
#[derive(Default)]
pub struct QSharedMemory {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QSharedMemory {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QSharedMemory {
    return QSharedMemory{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QSharedMemory {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QSharedMemory {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  int QSharedMemory::size();
impl /*struct*/ QSharedMemory {
  pub fn size<RetType, T: QSharedMemory_size<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.size(self);
    // return 1;
  }
}

pub trait QSharedMemory_size<RetType> {
  fn size(self , rsthis: & QSharedMemory) -> RetType;
}

  // proto:  int QSharedMemory::size();
impl<'a> /*trait*/ QSharedMemory_size<i32> for () {
  fn size(self , rsthis: & QSharedMemory) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QSharedMemory4sizeEv()};
    let mut ret = unsafe {C_ZNK13QSharedMemory4sizeEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  void QSharedMemory::setNativeKey(const QString & key);
impl /*struct*/ QSharedMemory {
  pub fn setNativeKey<RetType, T: QSharedMemory_setNativeKey<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setNativeKey(self);
    // return 1;
  }
}

pub trait QSharedMemory_setNativeKey<RetType> {
  fn setNativeKey(self , rsthis: & QSharedMemory) -> RetType;
}

  // proto:  void QSharedMemory::setNativeKey(const QString & key);
impl<'a> /*trait*/ QSharedMemory_setNativeKey<()> for (&'a QString) {
  fn setNativeKey(self , rsthis: & QSharedMemory) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSharedMemory12setNativeKeyERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN13QSharedMemory12setNativeKeyERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QSharedMemory::QSharedMemory(const QString & key, QObject * parent);
impl /*struct*/ QSharedMemory {
  pub fn new<T: QSharedMemory_new>(value: T) -> QSharedMemory {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QSharedMemory_new {
  fn new(self) -> QSharedMemory;
}

  // proto:  void QSharedMemory::QSharedMemory(const QString & key, QObject * parent);
impl<'a> /*trait*/ QSharedMemory_new for (&'a QString, Option<&'a QObject>) {
  fn new(self) -> QSharedMemory {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSharedMemoryC2ERK7QStringP7QObject()};
    let ctysz: c_int = unsafe{QSharedMemory_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = (if self.1.is_none() {0} else {self.1.unwrap().qclsinst})  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN13QSharedMemoryC2ERK7QStringP7QObject(arg0, arg1)};
    let rsthis = QSharedMemory{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QString QSharedMemory::errorString();
impl /*struct*/ QSharedMemory {
  pub fn errorString<RetType, T: QSharedMemory_errorString<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.errorString(self);
    // return 1;
  }
}

pub trait QSharedMemory_errorString<RetType> {
  fn errorString(self , rsthis: & QSharedMemory) -> RetType;
}

  // proto:  QString QSharedMemory::errorString();
impl<'a> /*trait*/ QSharedMemory_errorString<QString> for () {
  fn errorString(self , rsthis: & QSharedMemory) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QSharedMemory11errorStringEv()};
    let mut ret = unsafe {C_ZNK13QSharedMemory11errorStringEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QSharedMemory::setKey(const QString & key);
impl /*struct*/ QSharedMemory {
  pub fn setKey<RetType, T: QSharedMemory_setKey<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setKey(self);
    // return 1;
  }
}

pub trait QSharedMemory_setKey<RetType> {
  fn setKey(self , rsthis: & QSharedMemory) -> RetType;
}

  // proto:  void QSharedMemory::setKey(const QString & key);
impl<'a> /*trait*/ QSharedMemory_setKey<()> for (&'a QString) {
  fn setKey(self , rsthis: & QSharedMemory) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSharedMemory6setKeyERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN13QSharedMemory6setKeyERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QSharedMemory::key();
impl /*struct*/ QSharedMemory {
  pub fn key<RetType, T: QSharedMemory_key<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.key(self);
    // return 1;
  }
}

pub trait QSharedMemory_key<RetType> {
  fn key(self , rsthis: & QSharedMemory) -> RetType;
}

  // proto:  QString QSharedMemory::key();
impl<'a> /*trait*/ QSharedMemory_key<QString> for () {
  fn key(self , rsthis: & QSharedMemory) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QSharedMemory3keyEv()};
    let mut ret = unsafe {C_ZNK13QSharedMemory3keyEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const void * QSharedMemory::constData();
impl /*struct*/ QSharedMemory {
  pub fn constData<RetType, T: QSharedMemory_constData<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.constData(self);
    // return 1;
  }
}

pub trait QSharedMemory_constData<RetType> {
  fn constData(self , rsthis: & QSharedMemory) -> RetType;
}

  // proto:  const void * QSharedMemory::constData();
impl<'a> /*trait*/ QSharedMemory_constData<*mut c_void> for () {
  fn constData(self , rsthis: & QSharedMemory) -> *mut c_void {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QSharedMemory9constDataEv()};
    let mut ret = unsafe {C_ZNK13QSharedMemory9constDataEv(rsthis.qclsinst)};
    return ret as *mut c_void; // 1
    // return 1;
  }
}

  // proto:  void * QSharedMemory::data();
impl /*struct*/ QSharedMemory {
  pub fn data<RetType, T: QSharedMemory_data<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.data(self);
    // return 1;
  }
}

pub trait QSharedMemory_data<RetType> {
  fn data(self , rsthis: & QSharedMemory) -> RetType;
}

  // proto:  void * QSharedMemory::data();
impl<'a> /*trait*/ QSharedMemory_data<*mut c_void> for () {
  fn data(self , rsthis: & QSharedMemory) -> *mut c_void {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSharedMemory4dataEv()};
    let mut ret = unsafe {C_ZN13QSharedMemory4dataEv(rsthis.qclsinst)};
    return ret as *mut c_void; // 1
    // return 1;
  }
}

  // proto:  bool QSharedMemory::isAttached();
impl /*struct*/ QSharedMemory {
  pub fn isAttached<RetType, T: QSharedMemory_isAttached<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isAttached(self);
    // return 1;
  }
}

pub trait QSharedMemory_isAttached<RetType> {
  fn isAttached(self , rsthis: & QSharedMemory) -> RetType;
}

  // proto:  bool QSharedMemory::isAttached();
impl<'a> /*trait*/ QSharedMemory_isAttached<i8> for () {
  fn isAttached(self , rsthis: & QSharedMemory) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QSharedMemory10isAttachedEv()};
    let mut ret = unsafe {C_ZNK13QSharedMemory10isAttachedEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  bool QSharedMemory::lock();
impl /*struct*/ QSharedMemory {
  pub fn lock<RetType, T: QSharedMemory_lock<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.lock(self);
    // return 1;
  }
}

pub trait QSharedMemory_lock<RetType> {
  fn lock(self , rsthis: & QSharedMemory) -> RetType;
}

  // proto:  bool QSharedMemory::lock();
impl<'a> /*trait*/ QSharedMemory_lock<i8> for () {
  fn lock(self , rsthis: & QSharedMemory) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSharedMemory4lockEv()};
    let mut ret = unsafe {C_ZN13QSharedMemory4lockEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QSharedMemory::~QSharedMemory();
impl /*struct*/ QSharedMemory {
  pub fn free<RetType, T: QSharedMemory_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QSharedMemory_free<RetType> {
  fn free(self , rsthis: & QSharedMemory) -> RetType;
}

  // proto:  void QSharedMemory::~QSharedMemory();
impl<'a> /*trait*/ QSharedMemory_free<()> for () {
  fn free(self , rsthis: & QSharedMemory) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSharedMemoryD2Ev()};
     unsafe {C_ZN13QSharedMemoryD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QSharedMemory::unlock();
impl /*struct*/ QSharedMemory {
  pub fn unlock<RetType, T: QSharedMemory_unlock<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.unlock(self);
    // return 1;
  }
}

pub trait QSharedMemory_unlock<RetType> {
  fn unlock(self , rsthis: & QSharedMemory) -> RetType;
}

  // proto:  bool QSharedMemory::unlock();
impl<'a> /*trait*/ QSharedMemory_unlock<i8> for () {
  fn unlock(self , rsthis: & QSharedMemory) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSharedMemory6unlockEv()};
    let mut ret = unsafe {C_ZN13QSharedMemory6unlockEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  bool QSharedMemory::detach();
impl /*struct*/ QSharedMemory {
  pub fn detach<RetType, T: QSharedMemory_detach<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.detach(self);
    // return 1;
  }
}

pub trait QSharedMemory_detach<RetType> {
  fn detach(self , rsthis: & QSharedMemory) -> RetType;
}

  // proto:  bool QSharedMemory::detach();
impl<'a> /*trait*/ QSharedMemory_detach<i8> for () {
  fn detach(self , rsthis: & QSharedMemory) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSharedMemory6detachEv()};
    let mut ret = unsafe {C_ZN13QSharedMemory6detachEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  QString QSharedMemory::nativeKey();
impl /*struct*/ QSharedMemory {
  pub fn nativeKey<RetType, T: QSharedMemory_nativeKey<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.nativeKey(self);
    // return 1;
  }
}

pub trait QSharedMemory_nativeKey<RetType> {
  fn nativeKey(self , rsthis: & QSharedMemory) -> RetType;
}

  // proto:  QString QSharedMemory::nativeKey();
impl<'a> /*trait*/ QSharedMemory_nativeKey<QString> for () {
  fn nativeKey(self , rsthis: & QSharedMemory) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QSharedMemory9nativeKeyEv()};
    let mut ret = unsafe {C_ZNK13QSharedMemory9nativeKeyEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QMetaObject * QSharedMemory::metaObject();
impl /*struct*/ QSharedMemory {
  pub fn metaObject<RetType, T: QSharedMemory_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QSharedMemory_metaObject<RetType> {
  fn metaObject(self , rsthis: & QSharedMemory) -> RetType;
}

  // proto:  const QMetaObject * QSharedMemory::metaObject();
impl<'a> /*trait*/ QSharedMemory_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QSharedMemory) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QSharedMemory10metaObjectEv()};
    let mut ret = unsafe {C_ZNK13QSharedMemory10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QSharedMemory::QSharedMemory(QObject * parent);
impl<'a> /*trait*/ QSharedMemory_new for (Option<&'a QObject>) {
  fn new(self) -> QSharedMemory {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSharedMemoryC2EP7QObject()};
    let ctysz: c_int = unsafe{QSharedMemory_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = (if self.is_none() {0} else {self.unwrap().qclsinst})  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN13QSharedMemoryC2EP7QObject(arg0)};
    let rsthis = QSharedMemory{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// <= body block end

