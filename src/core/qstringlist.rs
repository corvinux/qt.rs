// auto generated, do not modify.
// created: Sun Jan 17 17:37:11 2016
// src-file: /QtCore/qstringlist.h
// dst-file: /src/core/qstringlist.rs
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
use super::qregularexpression::QRegularExpression; // 773
use super::qstring::QString; // 773
use super::qregexp::QRegExp; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QStringList_Class_Size() -> c_int;
  // proto:  int QStringList::lastIndexOf(const QRegularExpression & re, int from);
  fn _ZNK11QStringList11lastIndexOfERK18QRegularExpressioni(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int) -> c_int;
  // proto:  void QStringList::QStringList();
  fn _ZN11QStringListC2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  int QStringList::indexOf(const QRegExp & rx, int from);
  fn _ZNK11QStringList7indexOfERK7QRegExpi(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int) -> c_int;
  // proto:  int QStringList::indexOf(QRegExp & rx, int from);
  fn _ZNK11QStringList7indexOfER7QRegExpi(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int) -> c_int;
  // proto:  int QStringList::indexOf(const QRegularExpression & re, int from);
  fn _ZNK11QStringList7indexOfERK18QRegularExpressioni(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int) -> c_int;
  // proto:  int QStringList::lastIndexOf(const QRegExp & rx, int from);
  fn _ZNK11QStringList11lastIndexOfERK7QRegExpi(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int) -> c_int;
  // proto:  int QStringList::lastIndexOf(QRegExp & rx, int from);
  fn _ZNK11QStringList11lastIndexOfER7QRegExpi(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int) -> c_int;
  // proto:  void QStringList::QStringList(const QString & i);
  fn _ZN11QStringListC2ERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QStringList)=1
#[derive(Default)]
pub struct QStringList {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QStringList {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QStringList {
    return QStringList{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  int QStringList::lastIndexOf(const QRegularExpression & re, int from);
impl /*struct*/ QStringList {
  pub fn lastIndexOf<RetType, T: QStringList_lastIndexOf<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.lastIndexOf(self);
    // return 1;
  }
}

pub trait QStringList_lastIndexOf<RetType> {
  fn lastIndexOf(self , rsthis: & QStringList) -> RetType;
}

  // proto:  int QStringList::lastIndexOf(const QRegularExpression & re, int from);
impl<'a> /*trait*/ QStringList_lastIndexOf<i32> for (&'a QRegularExpression, i32) {
  fn lastIndexOf(self , rsthis: & QStringList) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QStringList11lastIndexOfERK18QRegularExpressioni()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK11QStringList11lastIndexOfERK18QRegularExpressioni(rsthis.qclsinst, arg0, arg1)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QStringList::QStringList();
impl /*struct*/ QStringList {
  pub fn new<T: QStringList_new>(value: T) -> QStringList {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QStringList_new {
  fn new(self) -> QStringList;
}

  // proto:  void QStringList::QStringList();
impl<'a> /*trait*/ QStringList_new for () {
  fn new(self) -> QStringList {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QStringListC2Ev()};
    let ctysz: c_int = unsafe{QStringList_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    unsafe {_ZN11QStringListC2Ev(qthis_ph)};
    let qthis: u64 = qthis_ph;
    let rsthis = QStringList{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QStringList::indexOf(const QRegExp & rx, int from);
impl /*struct*/ QStringList {
  pub fn indexOf<RetType, T: QStringList_indexOf<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.indexOf(self);
    // return 1;
  }
}

pub trait QStringList_indexOf<RetType> {
  fn indexOf(self , rsthis: & QStringList) -> RetType;
}

  // proto:  int QStringList::indexOf(const QRegExp & rx, int from);
impl<'a> /*trait*/ QStringList_indexOf<i32> for (&'a QRegExp, i32) {
  fn indexOf(self , rsthis: & QStringList) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QStringList7indexOfERK7QRegExpi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK11QStringList7indexOfERK7QRegExpi(rsthis.qclsinst, arg0, arg1)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QStringList::indexOf(const QRegularExpression & re, int from);
impl<'a> /*trait*/ QStringList_indexOf<i32> for (&'a QRegularExpression, i32) {
  fn indexOf(self , rsthis: & QStringList) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QStringList7indexOfERK18QRegularExpressioni()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK11QStringList7indexOfERK18QRegularExpressioni(rsthis.qclsinst, arg0, arg1)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QStringList::lastIndexOf(const QRegExp & rx, int from);
impl<'a> /*trait*/ QStringList_lastIndexOf<i32> for (&'a QRegExp, i32) {
  fn lastIndexOf(self , rsthis: & QStringList) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QStringList11lastIndexOfERK7QRegExpi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK11QStringList11lastIndexOfERK7QRegExpi(rsthis.qclsinst, arg0, arg1)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QStringList::QStringList(const QString & i);
impl<'a> /*trait*/ QStringList_new for (&'a QString) {
  fn new(self) -> QStringList {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QStringListC2ERK7QString()};
    let ctysz: c_int = unsafe{QStringList_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QStringListC2ERK7QString(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QStringList{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// <= body block end

