// auto generated, do not modify.
// created: Tue Dec 22 23:21:28 2015
// src-file: /QtCore/qcollator.h
// dst-file: /src/core/qcollator.rs
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
use super::qlocale::QLocale; // 773
use super::qchar::QChar; // 773
use super::qstring::QString; // 773
// use super::qcollator::QCollatorSortKey; // 773
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  bool QCollator::numericMode();
  fn _ZNK9QCollator11numericModeEv(qthis: *mut c_void) -> c_char;
  // proto:  void QCollator::QCollator(const QLocale & locale);
  fn _ZN9QCollatorC1ERK7QLocale(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QCollator::setLocale(const QLocale & locale);
  fn _ZN9QCollator9setLocaleERK7QLocale(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QCollator::setNumericMode(bool on);
  fn _ZN9QCollator14setNumericModeEb(qthis: *mut c_void, arg0: c_char);
  // proto:  int QCollator::compare(const QChar * s1, int len1, const QChar * s2, int len2);
  fn _ZNK9QCollator7compareEPK5QChariS2_i(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: *mut c_void, arg3: c_int) -> c_int;
  // proto:  QCollatorSortKey QCollator::sortKey(const QString & string);
  fn _ZNK9QCollator7sortKeyERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  int QCollator::compare(const QString & s1, const QString & s2);
  fn _ZNK9QCollator7compareERK7QStringS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> c_int;
  // proto:  void QCollator::~QCollator();
  fn _ZN9QCollatorD0Ev(qthis: *mut c_void);
  // proto:  bool QCollator::ignorePunctuation();
  fn _ZNK9QCollator17ignorePunctuationEv(qthis: *mut c_void) -> c_char;
  // proto:  void QCollator::QCollator(const QCollator & );
  fn _ZN9QCollatorC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QLocale QCollator::locale();
  fn _ZNK9QCollator6localeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QCollator::swap(QCollator & other);
  fn _ZN9QCollator4swapERS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QCollator::setIgnorePunctuation(bool on);
  fn _ZN9QCollator20setIgnorePunctuationEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QCollatorSortKey::~QCollatorSortKey();
  fn _ZN16QCollatorSortKeyD0Ev(qthis: *mut c_void);
  // proto:  void QCollatorSortKey::swap(QCollatorSortKey & other);
  fn _ZN16QCollatorSortKey4swapERS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  int QCollatorSortKey::compare(const QCollatorSortKey & key);
  fn _ZNK16QCollatorSortKey7compareERKS_(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto:  void QCollatorSortKey::QCollatorSortKey(const QCollatorSortKey & other);
  fn _ZN16QCollatorSortKeyC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QCollatorSortKey::QCollatorSortKey();
  fn _ZN16QCollatorSortKeyC1Ev(qthis: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QCollator)=8
pub struct QCollator {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

// class sizeof(QCollatorSortKey)=1
pub struct QCollatorSortKey {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QCollator {
  pub fn inheritFrom(qthis: *mut c_void) -> QCollator {
    return QCollator{qclsinst: qthis};
  }
}
  // proto:  bool QCollator::numericMode();
impl /*struct*/ QCollator {
  pub fn numericMode<RetType, T: QCollator_numericMode<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.numericMode(self);
    // return 1;
  }
}

pub trait QCollator_numericMode<RetType> {
  fn numericMode(self , rsthis: &mut QCollator) -> RetType;
}

  // proto:  bool QCollator::numericMode();
impl<'a> /*trait*/ QCollator_numericMode<i8> for () {
  fn numericMode(self , rsthis: &mut QCollator) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QCollator11numericModeEv()};
    let mut ret = unsafe {_ZNK9QCollator11numericModeEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QCollator::QCollator(const QLocale & locale);
impl /*struct*/ QCollator {
  pub fn NewQCollator<T: QCollator_NewQCollator>(value: T) -> QCollator {
    let rsthis = value.NewQCollator();
    return rsthis;
    // return 1;
  }
}

pub trait QCollator_NewQCollator {
  fn NewQCollator(self) -> QCollator;
}

  // proto:  void QCollator::QCollator(const QLocale & locale);
impl<'a> /*trait*/ QCollator_NewQCollator for (QLocale) {
  fn NewQCollator(self) -> QCollator {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QCollatorC1ERK7QLocale()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QCollatorC1ERK7QLocale(qthis, arg0)};
    let rsthis = QCollator{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QCollator::setLocale(const QLocale & locale);
impl /*struct*/ QCollator {
  pub fn setLocale<RetType, T: QCollator_setLocale<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setLocale(self);
    // return 1;
  }
}

pub trait QCollator_setLocale<RetType> {
  fn setLocale(self , rsthis: &mut QCollator) -> RetType;
}

  // proto:  void QCollator::setLocale(const QLocale & locale);
impl<'a> /*trait*/ QCollator_setLocale<()> for (QLocale) {
  fn setLocale(self , rsthis: &mut QCollator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QCollator9setLocaleERK7QLocale()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QCollator9setLocaleERK7QLocale(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QCollator::setNumericMode(bool on);
impl /*struct*/ QCollator {
  pub fn setNumericMode<RetType, T: QCollator_setNumericMode<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setNumericMode(self);
    // return 1;
  }
}

pub trait QCollator_setNumericMode<RetType> {
  fn setNumericMode(self , rsthis: &mut QCollator) -> RetType;
}

  // proto:  void QCollator::setNumericMode(bool on);
impl<'a> /*trait*/ QCollator_setNumericMode<()> for (i8) {
  fn setNumericMode(self , rsthis: &mut QCollator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QCollator14setNumericModeEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN9QCollator14setNumericModeEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QCollator::compare(const QChar * s1, int len1, const QChar * s2, int len2);
impl /*struct*/ QCollator {
  pub fn compare<RetType, T: QCollator_compare<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.compare(self);
    // return 1;
  }
}

pub trait QCollator_compare<RetType> {
  fn compare(self , rsthis: &mut QCollator) -> RetType;
}

  // proto:  int QCollator::compare(const QChar * s1, int len1, const QChar * s2, int len2);
impl<'a> /*trait*/ QCollator_compare<i32> for (QChar, i32, QChar, i32) {
  fn compare(self , rsthis: &mut QCollator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QCollator7compareEPK5QChariS2_i()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3  as c_int;
    let mut ret = unsafe {_ZNK9QCollator7compareEPK5QChariS2_i(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QCollatorSortKey QCollator::sortKey(const QString & string);
impl /*struct*/ QCollator {
  pub fn sortKey<RetType, T: QCollator_sortKey<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.sortKey(self);
    // return 1;
  }
}

pub trait QCollator_sortKey<RetType> {
  fn sortKey(self , rsthis: &mut QCollator) -> RetType;
}

  // proto:  QCollatorSortKey QCollator::sortKey(const QString & string);
impl<'a> /*trait*/ QCollator_sortKey<QCollatorSortKey> for (QString) {
  fn sortKey(self , rsthis: &mut QCollator) -> QCollatorSortKey {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QCollator7sortKeyERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QCollator7sortKeyERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QCollatorSortKey::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  int QCollator::compare(const QString & s1, const QString & s2);
impl<'a> /*trait*/ QCollator_compare<i32> for (QString, QString) {
  fn compare(self , rsthis: &mut QCollator) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QCollator7compareERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QCollator7compareERK7QStringS2_(rsthis.qclsinst, arg0, arg1)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QCollator::~QCollator();
impl /*struct*/ QCollator {
  pub fn FreeQCollator<RetType, T: QCollator_FreeQCollator<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQCollator(self);
    // return 1;
  }
}

pub trait QCollator_FreeQCollator<RetType> {
  fn FreeQCollator(self , rsthis: &mut QCollator) -> RetType;
}

  // proto:  void QCollator::~QCollator();
impl<'a> /*trait*/ QCollator_FreeQCollator<()> for () {
  fn FreeQCollator(self , rsthis: &mut QCollator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QCollatorD0Ev()};
     unsafe {_ZN9QCollatorD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QCollator::ignorePunctuation();
impl /*struct*/ QCollator {
  pub fn ignorePunctuation<RetType, T: QCollator_ignorePunctuation<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.ignorePunctuation(self);
    // return 1;
  }
}

pub trait QCollator_ignorePunctuation<RetType> {
  fn ignorePunctuation(self , rsthis: &mut QCollator) -> RetType;
}

  // proto:  bool QCollator::ignorePunctuation();
impl<'a> /*trait*/ QCollator_ignorePunctuation<i8> for () {
  fn ignorePunctuation(self , rsthis: &mut QCollator) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QCollator17ignorePunctuationEv()};
    let mut ret = unsafe {_ZNK9QCollator17ignorePunctuationEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QCollator::QCollator(const QCollator & );
impl<'a> /*trait*/ QCollator_NewQCollator for (QCollator) {
  fn NewQCollator(self) -> QCollator {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QCollatorC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QCollatorC1ERKS_(qthis, arg0)};
    let rsthis = QCollator{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QLocale QCollator::locale();
impl /*struct*/ QCollator {
  pub fn locale<RetType, T: QCollator_locale<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.locale(self);
    // return 1;
  }
}

pub trait QCollator_locale<RetType> {
  fn locale(self , rsthis: &mut QCollator) -> RetType;
}

  // proto:  QLocale QCollator::locale();
impl<'a> /*trait*/ QCollator_locale<QLocale> for () {
  fn locale(self , rsthis: &mut QCollator) -> QLocale {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QCollator6localeEv()};
    let mut ret = unsafe {_ZNK9QCollator6localeEv(rsthis.qclsinst)};
    let mut ret1 = QLocale::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QCollator::swap(QCollator & other);
impl /*struct*/ QCollator {
  pub fn swap<RetType, T: QCollator_swap<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QCollator_swap<RetType> {
  fn swap(self , rsthis: &mut QCollator) -> RetType;
}

  // proto:  void QCollator::swap(QCollator & other);
impl<'a> /*trait*/ QCollator_swap<()> for (QCollator) {
  fn swap(self , rsthis: &mut QCollator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QCollator4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QCollator4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QCollator::setIgnorePunctuation(bool on);
impl /*struct*/ QCollator {
  pub fn setIgnorePunctuation<RetType, T: QCollator_setIgnorePunctuation<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setIgnorePunctuation(self);
    // return 1;
  }
}

pub trait QCollator_setIgnorePunctuation<RetType> {
  fn setIgnorePunctuation(self , rsthis: &mut QCollator) -> RetType;
}

  // proto:  void QCollator::setIgnorePunctuation(bool on);
impl<'a> /*trait*/ QCollator_setIgnorePunctuation<()> for (i8) {
  fn setIgnorePunctuation(self , rsthis: &mut QCollator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QCollator20setIgnorePunctuationEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN9QCollator20setIgnorePunctuationEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QCollatorSortKey {
  pub fn inheritFrom(qthis: *mut c_void) -> QCollatorSortKey {
    return QCollatorSortKey{qclsinst: qthis};
  }
}
  // proto:  void QCollatorSortKey::~QCollatorSortKey();
impl /*struct*/ QCollatorSortKey {
  pub fn FreeQCollatorSortKey<RetType, T: QCollatorSortKey_FreeQCollatorSortKey<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQCollatorSortKey(self);
    // return 1;
  }
}

pub trait QCollatorSortKey_FreeQCollatorSortKey<RetType> {
  fn FreeQCollatorSortKey(self , rsthis: &mut QCollatorSortKey) -> RetType;
}

  // proto:  void QCollatorSortKey::~QCollatorSortKey();
impl<'a> /*trait*/ QCollatorSortKey_FreeQCollatorSortKey<()> for () {
  fn FreeQCollatorSortKey(self , rsthis: &mut QCollatorSortKey) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCollatorSortKeyD0Ev()};
     unsafe {_ZN16QCollatorSortKeyD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QCollatorSortKey::swap(QCollatorSortKey & other);
impl /*struct*/ QCollatorSortKey {
  pub fn swap<RetType, T: QCollatorSortKey_swap<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QCollatorSortKey_swap<RetType> {
  fn swap(self , rsthis: &mut QCollatorSortKey) -> RetType;
}

  // proto:  void QCollatorSortKey::swap(QCollatorSortKey & other);
impl<'a> /*trait*/ QCollatorSortKey_swap<()> for (QCollatorSortKey) {
  fn swap(self , rsthis: &mut QCollatorSortKey) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCollatorSortKey4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN16QCollatorSortKey4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QCollatorSortKey::compare(const QCollatorSortKey & key);
impl /*struct*/ QCollatorSortKey {
  pub fn compare<RetType, T: QCollatorSortKey_compare<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.compare(self);
    // return 1;
  }
}

pub trait QCollatorSortKey_compare<RetType> {
  fn compare(self , rsthis: &mut QCollatorSortKey) -> RetType;
}

  // proto:  int QCollatorSortKey::compare(const QCollatorSortKey & key);
impl<'a> /*trait*/ QCollatorSortKey_compare<i32> for (QCollatorSortKey) {
  fn compare(self , rsthis: &mut QCollatorSortKey) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QCollatorSortKey7compareERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK16QCollatorSortKey7compareERKS_(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QCollatorSortKey::QCollatorSortKey(const QCollatorSortKey & other);
impl /*struct*/ QCollatorSortKey {
  pub fn NewQCollatorSortKey<T: QCollatorSortKey_NewQCollatorSortKey>(value: T) -> QCollatorSortKey {
    let rsthis = value.NewQCollatorSortKey();
    return rsthis;
    // return 1;
  }
}

pub trait QCollatorSortKey_NewQCollatorSortKey {
  fn NewQCollatorSortKey(self) -> QCollatorSortKey;
}

  // proto:  void QCollatorSortKey::QCollatorSortKey(const QCollatorSortKey & other);
impl<'a> /*trait*/ QCollatorSortKey_NewQCollatorSortKey for (QCollatorSortKey) {
  fn NewQCollatorSortKey(self) -> QCollatorSortKey {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCollatorSortKeyC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN16QCollatorSortKeyC1ERKS_(qthis, arg0)};
    let rsthis = QCollatorSortKey{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QCollatorSortKey::QCollatorSortKey();
impl<'a> /*trait*/ QCollatorSortKey_NewQCollatorSortKey for () {
  fn NewQCollatorSortKey(self) -> QCollatorSortKey {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QCollatorSortKeyC1Ev()};
    unsafe {_ZN16QCollatorSortKeyC1Ev(qthis)};
    let rsthis = QCollatorSortKey{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// <= body block end

