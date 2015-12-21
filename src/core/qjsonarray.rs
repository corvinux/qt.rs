// auto generated, do not modify.
// created: Mon Dec 21 22:54:38 2015
// src-file: /QtCore/qjsonarray.h
// dst-file: /src/core/qjsonarray.rs
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
use super::qstringlist::QStringList; // 773
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  QJsonValue QJsonArray::first();
  fn _ZNK10QJsonArray5firstEv(qthis: *mut c_void);
  // proto:  bool QJsonArray::empty();
  fn _ZNK10QJsonArray5emptyEv(qthis: *mut c_void) -> c_char;
  // proto:  QJsonValue QJsonArray::takeAt(int i);
  fn _ZN10QJsonArray6takeAtEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QJsonArray::removeLast();
  fn _ZN10QJsonArray10removeLastEv(qthis: *mut c_void);
  // proto:  void QJsonArray::pop_front();
  fn _ZN10QJsonArray9pop_frontEv(qthis: *mut c_void);
  // proto:  QVariantList QJsonArray::toVariantList();
  fn _ZNK10QJsonArray13toVariantListEv(qthis: *mut c_void);
  // proto:  void QJsonArray::~QJsonArray();
  fn _ZN10QJsonArrayD0Ev(qthis: *mut c_void);
  // proto:  int QJsonArray::size();
  fn _ZNK10QJsonArray4sizeEv(qthis: *mut c_void) -> c_int;
  // proto:  int QJsonArray::count();
  fn _ZNK10QJsonArray5countEv(qthis: *mut c_void) -> c_int;
  // proto:  void QJsonArray::QJsonArray();
  fn _ZN10QJsonArrayC1Ev(qthis: *mut c_void);
  // proto:  QJsonValue QJsonArray::at(int i);
  fn _ZNK10QJsonArray2atEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QJsonArray::pop_back();
  fn _ZN10QJsonArray8pop_backEv(qthis: *mut c_void);
  // proto:  bool QJsonArray::isEmpty();
  fn _ZNK10QJsonArray7isEmptyEv(qthis: *mut c_void) -> c_char;
  // proto: static QJsonArray QJsonArray::fromStringList(const QStringList & list);
  fn _ZN10QJsonArray14fromStringListERK11QStringList(arg0: *mut c_void);
  // proto:  QJsonValue QJsonArray::last();
  fn _ZNK10QJsonArray4lastEv(qthis: *mut c_void);
  // proto:  void QJsonArray::removeFirst();
  fn _ZN10QJsonArray11removeFirstEv(qthis: *mut c_void);
  // proto:  void QJsonArray::removeAt(int i);
  fn _ZN10QJsonArray8removeAtEi(qthis: *mut c_void, arg0: c_int);
} // <= ext block end

// body block begin =>
// class sizeof(QJsonArray)=16
pub struct QJsonArray {
  pub qclsinst: *mut c_void,
}

  // proto:  QJsonValue QJsonArray::first();
impl /*struct*/ QJsonArray {
  pub fn first<RetType, T: QJsonArray_first<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.first(self);
    // return 1;
  }
}

pub trait QJsonArray_first<RetType> {
  fn first(self , rsthis: &mut QJsonArray) -> RetType;
}

  // proto:  QJsonValue QJsonArray::first();
impl<'a> /*trait*/ QJsonArray_first<()> for () {
  fn first(self , rsthis: &mut QJsonArray) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QJsonArray5firstEv()};
     unsafe {_ZNK10QJsonArray5firstEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QJsonArray::empty();
impl /*struct*/ QJsonArray {
  pub fn empty<RetType, T: QJsonArray_empty<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.empty(self);
    // return 1;
  }
}

pub trait QJsonArray_empty<RetType> {
  fn empty(self , rsthis: &mut QJsonArray) -> RetType;
}

  // proto:  bool QJsonArray::empty();
impl<'a> /*trait*/ QJsonArray_empty<i8> for () {
  fn empty(self , rsthis: &mut QJsonArray) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QJsonArray5emptyEv()};
    let mut ret = unsafe {_ZNK10QJsonArray5emptyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QJsonValue QJsonArray::takeAt(int i);
impl /*struct*/ QJsonArray {
  pub fn takeAt<RetType, T: QJsonArray_takeAt<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.takeAt(self);
    // return 1;
  }
}

pub trait QJsonArray_takeAt<RetType> {
  fn takeAt(self , rsthis: &mut QJsonArray) -> RetType;
}

  // proto:  QJsonValue QJsonArray::takeAt(int i);
impl<'a> /*trait*/ QJsonArray_takeAt<()> for (i32) {
  fn takeAt(self , rsthis: &mut QJsonArray) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QJsonArray6takeAtEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QJsonArray6takeAtEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QJsonArray::removeLast();
impl /*struct*/ QJsonArray {
  pub fn removeLast<RetType, T: QJsonArray_removeLast<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.removeLast(self);
    // return 1;
  }
}

pub trait QJsonArray_removeLast<RetType> {
  fn removeLast(self , rsthis: &mut QJsonArray) -> RetType;
}

  // proto:  void QJsonArray::removeLast();
impl<'a> /*trait*/ QJsonArray_removeLast<()> for () {
  fn removeLast(self , rsthis: &mut QJsonArray) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QJsonArray10removeLastEv()};
     unsafe {_ZN10QJsonArray10removeLastEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QJsonArray::pop_front();
impl /*struct*/ QJsonArray {
  pub fn pop_front<RetType, T: QJsonArray_pop_front<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.pop_front(self);
    // return 1;
  }
}

pub trait QJsonArray_pop_front<RetType> {
  fn pop_front(self , rsthis: &mut QJsonArray) -> RetType;
}

  // proto:  void QJsonArray::pop_front();
impl<'a> /*trait*/ QJsonArray_pop_front<()> for () {
  fn pop_front(self , rsthis: &mut QJsonArray) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QJsonArray9pop_frontEv()};
     unsafe {_ZN10QJsonArray9pop_frontEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QVariantList QJsonArray::toVariantList();
impl /*struct*/ QJsonArray {
  pub fn toVariantList<RetType, T: QJsonArray_toVariantList<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toVariantList(self);
    // return 1;
  }
}

pub trait QJsonArray_toVariantList<RetType> {
  fn toVariantList(self , rsthis: &mut QJsonArray) -> RetType;
}

  // proto:  QVariantList QJsonArray::toVariantList();
impl<'a> /*trait*/ QJsonArray_toVariantList<()> for () {
  fn toVariantList(self , rsthis: &mut QJsonArray) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QJsonArray13toVariantListEv()};
     unsafe {_ZNK10QJsonArray13toVariantListEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QJsonArray::~QJsonArray();
impl /*struct*/ QJsonArray {
  pub fn FreeQJsonArray<RetType, T: QJsonArray_FreeQJsonArray<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQJsonArray(self);
    // return 1;
  }
}

pub trait QJsonArray_FreeQJsonArray<RetType> {
  fn FreeQJsonArray(self , rsthis: &mut QJsonArray) -> RetType;
}

  // proto:  void QJsonArray::~QJsonArray();
impl<'a> /*trait*/ QJsonArray_FreeQJsonArray<()> for () {
  fn FreeQJsonArray(self , rsthis: &mut QJsonArray) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QJsonArrayD0Ev()};
     unsafe {_ZN10QJsonArrayD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QJsonArray::size();
impl /*struct*/ QJsonArray {
  pub fn size<RetType, T: QJsonArray_size<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.size(self);
    // return 1;
  }
}

pub trait QJsonArray_size<RetType> {
  fn size(self , rsthis: &mut QJsonArray) -> RetType;
}

  // proto:  int QJsonArray::size();
impl<'a> /*trait*/ QJsonArray_size<i32> for () {
  fn size(self , rsthis: &mut QJsonArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QJsonArray4sizeEv()};
    let mut ret = unsafe {_ZNK10QJsonArray4sizeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QJsonArray::count();
impl /*struct*/ QJsonArray {
  pub fn count<RetType, T: QJsonArray_count<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.count(self);
    // return 1;
  }
}

pub trait QJsonArray_count<RetType> {
  fn count(self , rsthis: &mut QJsonArray) -> RetType;
}

  // proto:  int QJsonArray::count();
impl<'a> /*trait*/ QJsonArray_count<i32> for () {
  fn count(self , rsthis: &mut QJsonArray) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QJsonArray5countEv()};
    let mut ret = unsafe {_ZNK10QJsonArray5countEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QJsonArray::QJsonArray();
impl /*struct*/ QJsonArray {
  pub fn NewQJsonArray<T: QJsonArray_NewQJsonArray>(value: T) -> QJsonArray {
    let rsthis = value.NewQJsonArray();
    return rsthis;
    // return 1;
  }
}

pub trait QJsonArray_NewQJsonArray {
  fn NewQJsonArray(self) -> QJsonArray;
}

  // proto:  void QJsonArray::QJsonArray();
impl<'a> /*trait*/ QJsonArray_NewQJsonArray for () {
  fn NewQJsonArray(self) -> QJsonArray {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QJsonArrayC1Ev()};
    unsafe {_ZN10QJsonArrayC1Ev(qthis)};
    let rsthis = QJsonArray{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QJsonValue QJsonArray::at(int i);
impl /*struct*/ QJsonArray {
  pub fn at<RetType, T: QJsonArray_at<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.at(self);
    // return 1;
  }
}

pub trait QJsonArray_at<RetType> {
  fn at(self , rsthis: &mut QJsonArray) -> RetType;
}

  // proto:  QJsonValue QJsonArray::at(int i);
impl<'a> /*trait*/ QJsonArray_at<()> for (i32) {
  fn at(self , rsthis: &mut QJsonArray) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QJsonArray2atEi()};
    let arg0 = self  as c_int;
     unsafe {_ZNK10QJsonArray2atEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QJsonArray::pop_back();
impl /*struct*/ QJsonArray {
  pub fn pop_back<RetType, T: QJsonArray_pop_back<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.pop_back(self);
    // return 1;
  }
}

pub trait QJsonArray_pop_back<RetType> {
  fn pop_back(self , rsthis: &mut QJsonArray) -> RetType;
}

  // proto:  void QJsonArray::pop_back();
impl<'a> /*trait*/ QJsonArray_pop_back<()> for () {
  fn pop_back(self , rsthis: &mut QJsonArray) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QJsonArray8pop_backEv()};
     unsafe {_ZN10QJsonArray8pop_backEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QJsonArray::isEmpty();
impl /*struct*/ QJsonArray {
  pub fn isEmpty<RetType, T: QJsonArray_isEmpty<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isEmpty(self);
    // return 1;
  }
}

pub trait QJsonArray_isEmpty<RetType> {
  fn isEmpty(self , rsthis: &mut QJsonArray) -> RetType;
}

  // proto:  bool QJsonArray::isEmpty();
impl<'a> /*trait*/ QJsonArray_isEmpty<i8> for () {
  fn isEmpty(self , rsthis: &mut QJsonArray) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QJsonArray7isEmptyEv()};
    let mut ret = unsafe {_ZNK10QJsonArray7isEmptyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto: static QJsonArray QJsonArray::fromStringList(const QStringList & list);
impl /*struct*/ QJsonArray {
  pub fn fromStringList_s<RetType, T: QJsonArray_fromStringList_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromStringList_s();
    // return 1;
  }
}

pub trait QJsonArray_fromStringList_s<RetType> {
  fn fromStringList_s(self ) -> RetType;
}

  // proto: static QJsonArray QJsonArray::fromStringList(const QStringList & list);
impl<'a> /*trait*/ QJsonArray_fromStringList_s<()> for (QStringList) {
  fn fromStringList_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QJsonArray14fromStringListERK11QStringList()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QJsonArray14fromStringListERK11QStringList(arg0)};
    // return 1;
  }
}

  // proto:  QJsonValue QJsonArray::last();
impl /*struct*/ QJsonArray {
  pub fn last<RetType, T: QJsonArray_last<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.last(self);
    // return 1;
  }
}

pub trait QJsonArray_last<RetType> {
  fn last(self , rsthis: &mut QJsonArray) -> RetType;
}

  // proto:  QJsonValue QJsonArray::last();
impl<'a> /*trait*/ QJsonArray_last<()> for () {
  fn last(self , rsthis: &mut QJsonArray) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QJsonArray4lastEv()};
     unsafe {_ZNK10QJsonArray4lastEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QJsonArray::removeFirst();
impl /*struct*/ QJsonArray {
  pub fn removeFirst<RetType, T: QJsonArray_removeFirst<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.removeFirst(self);
    // return 1;
  }
}

pub trait QJsonArray_removeFirst<RetType> {
  fn removeFirst(self , rsthis: &mut QJsonArray) -> RetType;
}

  // proto:  void QJsonArray::removeFirst();
impl<'a> /*trait*/ QJsonArray_removeFirst<()> for () {
  fn removeFirst(self , rsthis: &mut QJsonArray) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QJsonArray11removeFirstEv()};
     unsafe {_ZN10QJsonArray11removeFirstEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QJsonArray::removeAt(int i);
impl /*struct*/ QJsonArray {
  pub fn removeAt<RetType, T: QJsonArray_removeAt<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.removeAt(self);
    // return 1;
  }
}

pub trait QJsonArray_removeAt<RetType> {
  fn removeAt(self , rsthis: &mut QJsonArray) -> RetType;
}

  // proto:  void QJsonArray::removeAt(int i);
impl<'a> /*trait*/ QJsonArray_removeAt<()> for (i32) {
  fn removeAt(self , rsthis: &mut QJsonArray) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QJsonArray8removeAtEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QJsonArray8removeAtEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// <= body block end
