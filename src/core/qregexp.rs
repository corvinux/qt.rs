// auto generated, do not modify.
// created: Wed Dec 23 22:29:56 2015
// src-file: /QtCore/qregexp.h
// dst-file: /src/core/qregexp.rs
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
use super::qstring::QString; // 773
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  void QRegExp::QRegExp(const QRegExp & rx);
  fn _ZN7QRegExpC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QStringList QRegExp::capturedTexts();
  fn _ZN7QRegExp13capturedTextsEv(qthis: *mut c_void);
  // proto:  int QRegExp::captureCount();
  fn _ZNK7QRegExp12captureCountEv(qthis: *mut c_void) -> c_int;
  // proto: static QString QRegExp::escape(const QString & str);
  fn _ZN7QRegExp6escapeERK7QString(arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QRegExp::isEmpty();
  fn _ZNK7QRegExp7isEmptyEv(qthis: *mut c_void) -> c_char;
  // proto:  bool QRegExp::isMinimal();
  fn _ZNK7QRegExp9isMinimalEv(qthis: *mut c_void) -> c_char;
  // proto:  int QRegExp::matchedLength();
  fn _ZNK7QRegExp13matchedLengthEv(qthis: *mut c_void) -> c_int;
  // proto:  QString QRegExp::pattern();
  fn _ZNK7QRegExp7patternEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QRegExp::setPattern(const QString & pattern);
  fn _ZN7QRegExp10setPatternERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QRegExp::isValid();
  fn _ZNK7QRegExp7isValidEv(qthis: *mut c_void) -> c_char;
  // proto:  void QRegExp::~QRegExp();
  fn _ZN7QRegExpD0Ev(qthis: *mut c_void);
  // proto:  bool QRegExp::exactMatch(const QString & str);
  fn _ZNK7QRegExp10exactMatchERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  void QRegExp::swap(QRegExp & other);
  fn _ZN7QRegExp4swapERS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  int QRegExp::pos(int nth);
  fn _ZN7QRegExp3posEi(qthis: *mut c_void, arg0: c_int) -> c_int;
  // proto:  void QRegExp::QRegExp();
  fn _ZN7QRegExpC1Ev(qthis: *mut c_void);
  // proto:  QString QRegExp::cap(int nth);
  fn _ZN7QRegExp3capEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  QString QRegExp::errorString();
  fn _ZN7QRegExp11errorStringEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QRegExp::setMinimal(bool minimal);
  fn _ZN7QRegExp10setMinimalEb(qthis: *mut c_void, arg0: c_char);
} // <= ext block end

// body block begin =>
// class sizeof(QRegExp)=8
pub struct QRegExp {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QRegExp {
  pub fn inheritFrom(qthis: *mut c_void) -> QRegExp {
    return QRegExp{qclsinst: qthis};
  }
}
  // proto:  void QRegExp::QRegExp(const QRegExp & rx);
impl /*struct*/ QRegExp {
  pub fn New<T: QRegExp_New>(value: T) -> QRegExp {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QRegExp_New {
  fn New(self) -> QRegExp;
}

  // proto:  void QRegExp::QRegExp(const QRegExp & rx);
impl<'a> /*trait*/ QRegExp_New for (&'a QRegExp) {
  fn New(self) -> QRegExp {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QRegExpC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QRegExpC1ERKS_(qthis, arg0)};
    let rsthis = QRegExp{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QStringList QRegExp::capturedTexts();
impl /*struct*/ QRegExp {
  pub fn capturedTexts<RetType, T: QRegExp_capturedTexts<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.capturedTexts(self);
    // return 1;
  }
}

pub trait QRegExp_capturedTexts<RetType> {
  fn capturedTexts(self , rsthis: & QRegExp) -> RetType;
}

  // proto:  QStringList QRegExp::capturedTexts();
impl<'a> /*trait*/ QRegExp_capturedTexts<()> for () {
  fn capturedTexts(self , rsthis: & QRegExp) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QRegExp13capturedTextsEv()};
     unsafe {_ZN7QRegExp13capturedTextsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QRegExp::captureCount();
impl /*struct*/ QRegExp {
  pub fn captureCount<RetType, T: QRegExp_captureCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.captureCount(self);
    // return 1;
  }
}

pub trait QRegExp_captureCount<RetType> {
  fn captureCount(self , rsthis: & QRegExp) -> RetType;
}

  // proto:  int QRegExp::captureCount();
impl<'a> /*trait*/ QRegExp_captureCount<i32> for () {
  fn captureCount(self , rsthis: & QRegExp) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QRegExp12captureCountEv()};
    let mut ret = unsafe {_ZNK7QRegExp12captureCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto: static QString QRegExp::escape(const QString & str);
impl /*struct*/ QRegExp {
  pub fn escape_s<RetType, T: QRegExp_escape_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.escape_s();
    // return 1;
  }
}

pub trait QRegExp_escape_s<RetType> {
  fn escape_s(self ) -> RetType;
}

  // proto: static QString QRegExp::escape(const QString & str);
impl<'a> /*trait*/ QRegExp_escape_s<QString> for (&'a QString) {
  fn escape_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QRegExp6escapeERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN7QRegExp6escapeERK7QString(arg0)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QRegExp::isEmpty();
impl /*struct*/ QRegExp {
  pub fn isEmpty<RetType, T: QRegExp_isEmpty<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isEmpty(self);
    // return 1;
  }
}

pub trait QRegExp_isEmpty<RetType> {
  fn isEmpty(self , rsthis: & QRegExp) -> RetType;
}

  // proto:  bool QRegExp::isEmpty();
impl<'a> /*trait*/ QRegExp_isEmpty<i8> for () {
  fn isEmpty(self , rsthis: & QRegExp) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QRegExp7isEmptyEv()};
    let mut ret = unsafe {_ZNK7QRegExp7isEmptyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QRegExp::isMinimal();
impl /*struct*/ QRegExp {
  pub fn isMinimal<RetType, T: QRegExp_isMinimal<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isMinimal(self);
    // return 1;
  }
}

pub trait QRegExp_isMinimal<RetType> {
  fn isMinimal(self , rsthis: & QRegExp) -> RetType;
}

  // proto:  bool QRegExp::isMinimal();
impl<'a> /*trait*/ QRegExp_isMinimal<i8> for () {
  fn isMinimal(self , rsthis: & QRegExp) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QRegExp9isMinimalEv()};
    let mut ret = unsafe {_ZNK7QRegExp9isMinimalEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QRegExp::matchedLength();
impl /*struct*/ QRegExp {
  pub fn matchedLength<RetType, T: QRegExp_matchedLength<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.matchedLength(self);
    // return 1;
  }
}

pub trait QRegExp_matchedLength<RetType> {
  fn matchedLength(self , rsthis: & QRegExp) -> RetType;
}

  // proto:  int QRegExp::matchedLength();
impl<'a> /*trait*/ QRegExp_matchedLength<i32> for () {
  fn matchedLength(self , rsthis: & QRegExp) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QRegExp13matchedLengthEv()};
    let mut ret = unsafe {_ZNK7QRegExp13matchedLengthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QString QRegExp::pattern();
impl /*struct*/ QRegExp {
  pub fn pattern<RetType, T: QRegExp_pattern<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.pattern(self);
    // return 1;
  }
}

pub trait QRegExp_pattern<RetType> {
  fn pattern(self , rsthis: & QRegExp) -> RetType;
}

  // proto:  QString QRegExp::pattern();
impl<'a> /*trait*/ QRegExp_pattern<QString> for () {
  fn pattern(self , rsthis: & QRegExp) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QRegExp7patternEv()};
    let mut ret = unsafe {_ZNK7QRegExp7patternEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QRegExp::setPattern(const QString & pattern);
impl /*struct*/ QRegExp {
  pub fn setPattern<RetType, T: QRegExp_setPattern<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPattern(self);
    // return 1;
  }
}

pub trait QRegExp_setPattern<RetType> {
  fn setPattern(self , rsthis: & QRegExp) -> RetType;
}

  // proto:  void QRegExp::setPattern(const QString & pattern);
impl<'a> /*trait*/ QRegExp_setPattern<()> for (&'a QString) {
  fn setPattern(self , rsthis: & QRegExp) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QRegExp10setPatternERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QRegExp10setPatternERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QRegExp::isValid();
impl /*struct*/ QRegExp {
  pub fn isValid<RetType, T: QRegExp_isValid<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QRegExp_isValid<RetType> {
  fn isValid(self , rsthis: & QRegExp) -> RetType;
}

  // proto:  bool QRegExp::isValid();
impl<'a> /*trait*/ QRegExp_isValid<i8> for () {
  fn isValid(self , rsthis: & QRegExp) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QRegExp7isValidEv()};
    let mut ret = unsafe {_ZNK7QRegExp7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QRegExp::~QRegExp();
impl /*struct*/ QRegExp {
  pub fn Free<RetType, T: QRegExp_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QRegExp_Free<RetType> {
  fn Free(self , rsthis: & QRegExp) -> RetType;
}

  // proto:  void QRegExp::~QRegExp();
impl<'a> /*trait*/ QRegExp_Free<()> for () {
  fn Free(self , rsthis: & QRegExp) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QRegExpD0Ev()};
     unsafe {_ZN7QRegExpD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QRegExp::exactMatch(const QString & str);
impl /*struct*/ QRegExp {
  pub fn exactMatch<RetType, T: QRegExp_exactMatch<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.exactMatch(self);
    // return 1;
  }
}

pub trait QRegExp_exactMatch<RetType> {
  fn exactMatch(self , rsthis: & QRegExp) -> RetType;
}

  // proto:  bool QRegExp::exactMatch(const QString & str);
impl<'a> /*trait*/ QRegExp_exactMatch<i8> for (&'a QString) {
  fn exactMatch(self , rsthis: & QRegExp) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QRegExp10exactMatchERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QRegExp10exactMatchERK7QString(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QRegExp::swap(QRegExp & other);
impl /*struct*/ QRegExp {
  pub fn swap<RetType, T: QRegExp_swap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QRegExp_swap<RetType> {
  fn swap(self , rsthis: & QRegExp) -> RetType;
}

  // proto:  void QRegExp::swap(QRegExp & other);
impl<'a> /*trait*/ QRegExp_swap<()> for (&'a QRegExp) {
  fn swap(self , rsthis: & QRegExp) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QRegExp4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QRegExp4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QRegExp::pos(int nth);
impl /*struct*/ QRegExp {
  pub fn pos<RetType, T: QRegExp_pos<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.pos(self);
    // return 1;
  }
}

pub trait QRegExp_pos<RetType> {
  fn pos(self , rsthis: & QRegExp) -> RetType;
}

  // proto:  int QRegExp::pos(int nth);
impl<'a> /*trait*/ QRegExp_pos<i32> for (i32) {
  fn pos(self , rsthis: & QRegExp) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QRegExp3posEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN7QRegExp3posEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QRegExp::QRegExp();
impl<'a> /*trait*/ QRegExp_New for () {
  fn New(self) -> QRegExp {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QRegExpC1Ev()};
    unsafe {_ZN7QRegExpC1Ev(qthis)};
    let rsthis = QRegExp{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QString QRegExp::cap(int nth);
impl /*struct*/ QRegExp {
  pub fn cap<RetType, T: QRegExp_cap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.cap(self);
    // return 1;
  }
}

pub trait QRegExp_cap<RetType> {
  fn cap(self , rsthis: & QRegExp) -> RetType;
}

  // proto:  QString QRegExp::cap(int nth);
impl<'a> /*trait*/ QRegExp_cap<QString> for (i32) {
  fn cap(self , rsthis: & QRegExp) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QRegExp3capEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN7QRegExp3capEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QRegExp::errorString();
impl /*struct*/ QRegExp {
  pub fn errorString<RetType, T: QRegExp_errorString<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.errorString(self);
    // return 1;
  }
}

pub trait QRegExp_errorString<RetType> {
  fn errorString(self , rsthis: & QRegExp) -> RetType;
}

  // proto:  QString QRegExp::errorString();
impl<'a> /*trait*/ QRegExp_errorString<QString> for () {
  fn errorString(self , rsthis: & QRegExp) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QRegExp11errorStringEv()};
    let mut ret = unsafe {_ZN7QRegExp11errorStringEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QRegExp::setMinimal(bool minimal);
impl /*struct*/ QRegExp {
  pub fn setMinimal<RetType, T: QRegExp_setMinimal<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMinimal(self);
    // return 1;
  }
}

pub trait QRegExp_setMinimal<RetType> {
  fn setMinimal(self , rsthis: & QRegExp) -> RetType;
}

  // proto:  void QRegExp::setMinimal(bool minimal);
impl<'a> /*trait*/ QRegExp_setMinimal<()> for (i8) {
  fn setMinimal(self , rsthis: & QRegExp) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QRegExp10setMinimalEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN7QRegExp10setMinimalEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// <= body block end

