// auto generated, do not modify.
// created: Mon Dec 21 22:54:38 2015
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
use super::qstring::QString; // 773
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  bool QJsonObject::isEmpty();
  fn _ZNK11QJsonObject7isEmptyEv(qthis: *mut c_void) -> c_char;
  // proto:  int QJsonObject::length();
  fn _ZNK11QJsonObject6lengthEv(qthis: *mut c_void) -> c_int;
  // proto:  void QJsonObject::remove(const QString & key);
  fn _ZN11QJsonObject6removeERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QJsonObject::~QJsonObject();
  fn _ZN11QJsonObjectD0Ev(qthis: *mut c_void);
  // proto:  QJsonValue QJsonObject::value(const QString & key);
  fn _ZNK11QJsonObject5valueERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  int QJsonObject::size();
  fn _ZNK11QJsonObject4sizeEv(qthis: *mut c_void) -> c_int;
  // proto:  int QJsonObject::count();
  fn _ZNK11QJsonObject5countEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QJsonObject::empty();
  fn _ZNK11QJsonObject5emptyEv(qthis: *mut c_void) -> c_char;
  // proto:  QJsonValue QJsonObject::take(const QString & key);
  fn _ZN11QJsonObject4takeERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QVariantHash QJsonObject::toVariantHash();
  fn _ZNK11QJsonObject13toVariantHashEv(qthis: *mut c_void);
  // proto:  QStringList QJsonObject::keys();
  fn _ZNK11QJsonObject4keysEv(qthis: *mut c_void);
  // proto:  bool QJsonObject::contains(const QString & key);
  fn _ZNK11QJsonObject8containsERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  void QJsonObject::QJsonObject();
  fn _ZN11QJsonObjectC1Ev(qthis: *mut c_void);
  // proto:  QVariantMap QJsonObject::toVariantMap();
  fn _ZNK11QJsonObject12toVariantMapEv(qthis: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QJsonObject)=16
pub struct QJsonObject {
  pub qclsinst: *mut c_void,
}

  // proto:  bool QJsonObject::isEmpty();
impl /*struct*/ QJsonObject {
  pub fn isEmpty<RetType, T: QJsonObject_isEmpty<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isEmpty(self);
    // return 1;
  }
}

pub trait QJsonObject_isEmpty<RetType> {
  fn isEmpty(self , rsthis: &mut QJsonObject) -> RetType;
}

  // proto:  bool QJsonObject::isEmpty();
impl<'a> /*trait*/ QJsonObject_isEmpty<i8> for () {
  fn isEmpty(self , rsthis: &mut QJsonObject) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QJsonObject7isEmptyEv()};
    let mut ret = unsafe {_ZNK11QJsonObject7isEmptyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QJsonObject::length();
impl /*struct*/ QJsonObject {
  pub fn length<RetType, T: QJsonObject_length<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.length(self);
    // return 1;
  }
}

pub trait QJsonObject_length<RetType> {
  fn length(self , rsthis: &mut QJsonObject) -> RetType;
}

  // proto:  int QJsonObject::length();
impl<'a> /*trait*/ QJsonObject_length<i32> for () {
  fn length(self , rsthis: &mut QJsonObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QJsonObject6lengthEv()};
    let mut ret = unsafe {_ZNK11QJsonObject6lengthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QJsonObject::remove(const QString & key);
impl /*struct*/ QJsonObject {
  pub fn remove<RetType, T: QJsonObject_remove<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.remove(self);
    // return 1;
  }
}

pub trait QJsonObject_remove<RetType> {
  fn remove(self , rsthis: &mut QJsonObject) -> RetType;
}

  // proto:  void QJsonObject::remove(const QString & key);
impl<'a> /*trait*/ QJsonObject_remove<()> for (QString) {
  fn remove(self , rsthis: &mut QJsonObject) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QJsonObject6removeERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QJsonObject6removeERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QJsonObject::~QJsonObject();
impl /*struct*/ QJsonObject {
  pub fn FreeQJsonObject<RetType, T: QJsonObject_FreeQJsonObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQJsonObject(self);
    // return 1;
  }
}

pub trait QJsonObject_FreeQJsonObject<RetType> {
  fn FreeQJsonObject(self , rsthis: &mut QJsonObject) -> RetType;
}

  // proto:  void QJsonObject::~QJsonObject();
impl<'a> /*trait*/ QJsonObject_FreeQJsonObject<()> for () {
  fn FreeQJsonObject(self , rsthis: &mut QJsonObject) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QJsonObjectD0Ev()};
     unsafe {_ZN11QJsonObjectD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QJsonValue QJsonObject::value(const QString & key);
impl /*struct*/ QJsonObject {
  pub fn value<RetType, T: QJsonObject_value<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.value(self);
    // return 1;
  }
}

pub trait QJsonObject_value<RetType> {
  fn value(self , rsthis: &mut QJsonObject) -> RetType;
}

  // proto:  QJsonValue QJsonObject::value(const QString & key);
impl<'a> /*trait*/ QJsonObject_value<()> for (QString) {
  fn value(self , rsthis: &mut QJsonObject) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QJsonObject5valueERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZNK11QJsonObject5valueERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QJsonObject::size();
impl /*struct*/ QJsonObject {
  pub fn size<RetType, T: QJsonObject_size<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.size(self);
    // return 1;
  }
}

pub trait QJsonObject_size<RetType> {
  fn size(self , rsthis: &mut QJsonObject) -> RetType;
}

  // proto:  int QJsonObject::size();
impl<'a> /*trait*/ QJsonObject_size<i32> for () {
  fn size(self , rsthis: &mut QJsonObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QJsonObject4sizeEv()};
    let mut ret = unsafe {_ZNK11QJsonObject4sizeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QJsonObject::count();
impl /*struct*/ QJsonObject {
  pub fn count<RetType, T: QJsonObject_count<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.count(self);
    // return 1;
  }
}

pub trait QJsonObject_count<RetType> {
  fn count(self , rsthis: &mut QJsonObject) -> RetType;
}

  // proto:  int QJsonObject::count();
impl<'a> /*trait*/ QJsonObject_count<i32> for () {
  fn count(self , rsthis: &mut QJsonObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QJsonObject5countEv()};
    let mut ret = unsafe {_ZNK11QJsonObject5countEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QJsonObject::empty();
impl /*struct*/ QJsonObject {
  pub fn empty<RetType, T: QJsonObject_empty<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.empty(self);
    // return 1;
  }
}

pub trait QJsonObject_empty<RetType> {
  fn empty(self , rsthis: &mut QJsonObject) -> RetType;
}

  // proto:  bool QJsonObject::empty();
impl<'a> /*trait*/ QJsonObject_empty<i8> for () {
  fn empty(self , rsthis: &mut QJsonObject) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QJsonObject5emptyEv()};
    let mut ret = unsafe {_ZNK11QJsonObject5emptyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QJsonValue QJsonObject::take(const QString & key);
impl /*struct*/ QJsonObject {
  pub fn take<RetType, T: QJsonObject_take<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.take(self);
    // return 1;
  }
}

pub trait QJsonObject_take<RetType> {
  fn take(self , rsthis: &mut QJsonObject) -> RetType;
}

  // proto:  QJsonValue QJsonObject::take(const QString & key);
impl<'a> /*trait*/ QJsonObject_take<()> for (QString) {
  fn take(self , rsthis: &mut QJsonObject) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QJsonObject4takeERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QJsonObject4takeERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QVariantHash QJsonObject::toVariantHash();
impl /*struct*/ QJsonObject {
  pub fn toVariantHash<RetType, T: QJsonObject_toVariantHash<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toVariantHash(self);
    // return 1;
  }
}

pub trait QJsonObject_toVariantHash<RetType> {
  fn toVariantHash(self , rsthis: &mut QJsonObject) -> RetType;
}

  // proto:  QVariantHash QJsonObject::toVariantHash();
impl<'a> /*trait*/ QJsonObject_toVariantHash<()> for () {
  fn toVariantHash(self , rsthis: &mut QJsonObject) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QJsonObject13toVariantHashEv()};
     unsafe {_ZNK11QJsonObject13toVariantHashEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QStringList QJsonObject::keys();
impl /*struct*/ QJsonObject {
  pub fn keys<RetType, T: QJsonObject_keys<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.keys(self);
    // return 1;
  }
}

pub trait QJsonObject_keys<RetType> {
  fn keys(self , rsthis: &mut QJsonObject) -> RetType;
}

  // proto:  QStringList QJsonObject::keys();
impl<'a> /*trait*/ QJsonObject_keys<()> for () {
  fn keys(self , rsthis: &mut QJsonObject) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QJsonObject4keysEv()};
     unsafe {_ZNK11QJsonObject4keysEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QJsonObject::contains(const QString & key);
impl /*struct*/ QJsonObject {
  pub fn contains<RetType, T: QJsonObject_contains<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.contains(self);
    // return 1;
  }
}

pub trait QJsonObject_contains<RetType> {
  fn contains(self , rsthis: &mut QJsonObject) -> RetType;
}

  // proto:  bool QJsonObject::contains(const QString & key);
impl<'a> /*trait*/ QJsonObject_contains<i8> for (QString) {
  fn contains(self , rsthis: &mut QJsonObject) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QJsonObject8containsERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK11QJsonObject8containsERK7QString(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QJsonObject::QJsonObject();
impl /*struct*/ QJsonObject {
  pub fn NewQJsonObject<T: QJsonObject_NewQJsonObject>(value: T) -> QJsonObject {
    let rsthis = value.NewQJsonObject();
    return rsthis;
    // return 1;
  }
}

pub trait QJsonObject_NewQJsonObject {
  fn NewQJsonObject(self) -> QJsonObject;
}

  // proto:  void QJsonObject::QJsonObject();
impl<'a> /*trait*/ QJsonObject_NewQJsonObject for () {
  fn NewQJsonObject(self) -> QJsonObject {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QJsonObjectC1Ev()};
    unsafe {_ZN11QJsonObjectC1Ev(qthis)};
    let rsthis = QJsonObject{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QVariantMap QJsonObject::toVariantMap();
impl /*struct*/ QJsonObject {
  pub fn toVariantMap<RetType, T: QJsonObject_toVariantMap<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toVariantMap(self);
    // return 1;
  }
}

pub trait QJsonObject_toVariantMap<RetType> {
  fn toVariantMap(self , rsthis: &mut QJsonObject) -> RetType;
}

  // proto:  QVariantMap QJsonObject::toVariantMap();
impl<'a> /*trait*/ QJsonObject_toVariantMap<()> for () {
  fn toVariantMap(self , rsthis: &mut QJsonObject) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QJsonObject12toVariantMapEv()};
     unsafe {_ZNK11QJsonObject12toVariantMapEv(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

