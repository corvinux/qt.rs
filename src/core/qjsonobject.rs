// auto generated, do not modify.
// created: Mon Feb 22 23:57:02 2016
// src-file: /QtCore/qjsonobject.h
// dst-file: /src/core/qjsonobject.rs
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
use std::ops::Deref;
use super::qstring::*; // 773
use super::qjsonvalue::*; // 773
// use super::qhash::*; // 775
use super::qstringlist::*; // 773
// use super::qmap::*; // 775
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QJsonObject_Class_Size() -> c_int;
  // proto:  bool QJsonObject::isEmpty();
  fn C_ZNK11QJsonObject7isEmptyEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  int QJsonObject::length();
  fn C_ZNK11QJsonObject6lengthEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QJsonObject::remove(const QString & key);
  fn C_ZN11QJsonObject6removeERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QJsonObject::~QJsonObject();
  fn C_ZN11QJsonObjectD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QJsonValue QJsonObject::value(const QString & key);
  fn C_ZNK11QJsonObject5valueERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  int QJsonObject::size();
  fn C_ZNK11QJsonObject4sizeEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  int QJsonObject::count();
  fn C_ZNK11QJsonObject5countEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  bool QJsonObject::empty();
  fn C_ZNK11QJsonObject5emptyEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QJsonValue QJsonObject::take(const QString & key);
  fn C_ZN11QJsonObject4takeERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  QVariantHash QJsonObject::toVariantHash();
  fn C_ZNK11QJsonObject13toVariantHashEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QStringList QJsonObject::keys();
  fn C_ZNK11QJsonObject4keysEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QJsonObject::contains(const QString & key);
  fn C_ZNK11QJsonObject8containsERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  void QJsonObject::QJsonObject();
  fn C_ZN11QJsonObjectC2Ev() -> u64;
  // proto:  QVariantMap QJsonObject::toVariantMap();
  fn C_ZNK11QJsonObject12toVariantMapEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
} // <= ext block end

// body block begin =>
// class sizeof(QJsonObject)=16
#[derive(Default)]
pub struct QJsonObject {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QJsonObject {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QJsonObject {
    return QJsonObject{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  bool QJsonObject::isEmpty();
impl /*struct*/ QJsonObject {
  pub fn isEmpty<RetType, T: QJsonObject_isEmpty<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isEmpty(self);
    // return 1;
  }
}

pub trait QJsonObject_isEmpty<RetType> {
  fn isEmpty(self , rsthis: & QJsonObject) -> RetType;
}

  // proto:  bool QJsonObject::isEmpty();
impl<'a> /*trait*/ QJsonObject_isEmpty<i8> for () {
  fn isEmpty(self , rsthis: & QJsonObject) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QJsonObject7isEmptyEv()};
    let mut ret = unsafe {C_ZNK11QJsonObject7isEmptyEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  int QJsonObject::length();
impl /*struct*/ QJsonObject {
  pub fn length<RetType, T: QJsonObject_length<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.length(self);
    // return 1;
  }
}

pub trait QJsonObject_length<RetType> {
  fn length(self , rsthis: & QJsonObject) -> RetType;
}

  // proto:  int QJsonObject::length();
impl<'a> /*trait*/ QJsonObject_length<i32> for () {
  fn length(self , rsthis: & QJsonObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QJsonObject6lengthEv()};
    let mut ret = unsafe {C_ZNK11QJsonObject6lengthEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  void QJsonObject::remove(const QString & key);
impl /*struct*/ QJsonObject {
  pub fn remove<RetType, T: QJsonObject_remove<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.remove(self);
    // return 1;
  }
}

pub trait QJsonObject_remove<RetType> {
  fn remove(self , rsthis: & QJsonObject) -> RetType;
}

  // proto:  void QJsonObject::remove(const QString & key);
impl<'a> /*trait*/ QJsonObject_remove<()> for (&'a QString) {
  fn remove(self , rsthis: & QJsonObject) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QJsonObject6removeERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN11QJsonObject6removeERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QJsonObject::~QJsonObject();
impl /*struct*/ QJsonObject {
  pub fn free<RetType, T: QJsonObject_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QJsonObject_free<RetType> {
  fn free(self , rsthis: & QJsonObject) -> RetType;
}

  // proto:  void QJsonObject::~QJsonObject();
impl<'a> /*trait*/ QJsonObject_free<()> for () {
  fn free(self , rsthis: & QJsonObject) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QJsonObjectD2Ev()};
     unsafe {C_ZN11QJsonObjectD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QJsonValue QJsonObject::value(const QString & key);
impl /*struct*/ QJsonObject {
  pub fn value<RetType, T: QJsonObject_value<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.value(self);
    // return 1;
  }
}

pub trait QJsonObject_value<RetType> {
  fn value(self , rsthis: & QJsonObject) -> RetType;
}

  // proto:  QJsonValue QJsonObject::value(const QString & key);
impl<'a> /*trait*/ QJsonObject_value<QJsonValue> for (&'a QString) {
  fn value(self , rsthis: & QJsonObject) -> QJsonValue {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QJsonObject5valueERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK11QJsonObject5valueERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QJsonValue::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QJsonObject::size();
impl /*struct*/ QJsonObject {
  pub fn size<RetType, T: QJsonObject_size<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.size(self);
    // return 1;
  }
}

pub trait QJsonObject_size<RetType> {
  fn size(self , rsthis: & QJsonObject) -> RetType;
}

  // proto:  int QJsonObject::size();
impl<'a> /*trait*/ QJsonObject_size<i32> for () {
  fn size(self , rsthis: & QJsonObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QJsonObject4sizeEv()};
    let mut ret = unsafe {C_ZNK11QJsonObject4sizeEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  int QJsonObject::count();
impl /*struct*/ QJsonObject {
  pub fn count<RetType, T: QJsonObject_count<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.count(self);
    // return 1;
  }
}

pub trait QJsonObject_count<RetType> {
  fn count(self , rsthis: & QJsonObject) -> RetType;
}

  // proto:  int QJsonObject::count();
impl<'a> /*trait*/ QJsonObject_count<i32> for () {
  fn count(self , rsthis: & QJsonObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QJsonObject5countEv()};
    let mut ret = unsafe {C_ZNK11QJsonObject5countEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  bool QJsonObject::empty();
impl /*struct*/ QJsonObject {
  pub fn empty<RetType, T: QJsonObject_empty<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.empty(self);
    // return 1;
  }
}

pub trait QJsonObject_empty<RetType> {
  fn empty(self , rsthis: & QJsonObject) -> RetType;
}

  // proto:  bool QJsonObject::empty();
impl<'a> /*trait*/ QJsonObject_empty<i8> for () {
  fn empty(self , rsthis: & QJsonObject) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QJsonObject5emptyEv()};
    let mut ret = unsafe {C_ZNK11QJsonObject5emptyEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  QJsonValue QJsonObject::take(const QString & key);
impl /*struct*/ QJsonObject {
  pub fn take<RetType, T: QJsonObject_take<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.take(self);
    // return 1;
  }
}

pub trait QJsonObject_take<RetType> {
  fn take(self , rsthis: & QJsonObject) -> RetType;
}

  // proto:  QJsonValue QJsonObject::take(const QString & key);
impl<'a> /*trait*/ QJsonObject_take<QJsonValue> for (&'a QString) {
  fn take(self , rsthis: & QJsonObject) -> QJsonValue {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QJsonObject4takeERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN11QJsonObject4takeERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QJsonValue::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QVariantHash QJsonObject::toVariantHash();
impl /*struct*/ QJsonObject {
  pub fn toVariantHash<RetType, T: QJsonObject_toVariantHash<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toVariantHash(self);
    // return 1;
  }
}

pub trait QJsonObject_toVariantHash<RetType> {
  fn toVariantHash(self , rsthis: & QJsonObject) -> RetType;
}

  // proto:  QVariantHash QJsonObject::toVariantHash();
impl<'a> /*trait*/ QJsonObject_toVariantHash<u64> for () {
  fn toVariantHash(self , rsthis: & QJsonObject) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QJsonObject13toVariantHashEv()};
    let mut ret = unsafe {C_ZNK11QJsonObject13toVariantHashEv(rsthis.qclsinst)};
    return ret as u64; // 5
    // return 1;
  }
}

  // proto:  QStringList QJsonObject::keys();
impl /*struct*/ QJsonObject {
  pub fn keys<RetType, T: QJsonObject_keys<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.keys(self);
    // return 1;
  }
}

pub trait QJsonObject_keys<RetType> {
  fn keys(self , rsthis: & QJsonObject) -> RetType;
}

  // proto:  QStringList QJsonObject::keys();
impl<'a> /*trait*/ QJsonObject_keys<QStringList> for () {
  fn keys(self , rsthis: & QJsonObject) -> QStringList {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QJsonObject4keysEv()};
    let mut ret = unsafe {C_ZNK11QJsonObject4keysEv(rsthis.qclsinst)};
    let mut ret1 = QStringList::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QJsonObject::contains(const QString & key);
impl /*struct*/ QJsonObject {
  pub fn contains<RetType, T: QJsonObject_contains<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.contains(self);
    // return 1;
  }
}

pub trait QJsonObject_contains<RetType> {
  fn contains(self , rsthis: & QJsonObject) -> RetType;
}

  // proto:  bool QJsonObject::contains(const QString & key);
impl<'a> /*trait*/ QJsonObject_contains<i8> for (&'a QString) {
  fn contains(self , rsthis: & QJsonObject) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QJsonObject8containsERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK11QJsonObject8containsERK7QString(rsthis.qclsinst, arg0)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QJsonObject::QJsonObject();
impl /*struct*/ QJsonObject {
  pub fn new<T: QJsonObject_new>(value: T) -> QJsonObject {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QJsonObject_new {
  fn new(self) -> QJsonObject;
}

  // proto:  void QJsonObject::QJsonObject();
impl<'a> /*trait*/ QJsonObject_new for () {
  fn new(self) -> QJsonObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QJsonObjectC2Ev()};
    let ctysz: c_int = unsafe{QJsonObject_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN11QJsonObjectC2Ev()};
    let rsthis = QJsonObject{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QVariantMap QJsonObject::toVariantMap();
impl /*struct*/ QJsonObject {
  pub fn toVariantMap<RetType, T: QJsonObject_toVariantMap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toVariantMap(self);
    // return 1;
  }
}

pub trait QJsonObject_toVariantMap<RetType> {
  fn toVariantMap(self , rsthis: & QJsonObject) -> RetType;
}

  // proto:  QVariantMap QJsonObject::toVariantMap();
impl<'a> /*trait*/ QJsonObject_toVariantMap<u64> for () {
  fn toVariantMap(self , rsthis: & QJsonObject) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QJsonObject12toVariantMapEv()};
    let mut ret = unsafe {C_ZNK11QJsonObject12toVariantMapEv(rsthis.qclsinst)};
    return ret as u64; // 5
    // return 1;
  }
}

// <= body block end

