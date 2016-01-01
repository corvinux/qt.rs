// auto generated, do not modify.
// created: Fri Jan  1 12:13:41 2016
// src-file: /QtCore/qdiriterator.h
// dst-file: /src/core/qdiriterator.rs
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
use super::qdir::QDir; // 773
use super::qstringlist::QStringList; // 773
use super::qfileinfo::QFileInfo; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QDirIterator_Class_Size() -> c_int;
  // proto:  QString QDirIterator::fileName();
  fn _ZNK12QDirIterator8fileNameEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QString QDirIterator::path();
  fn _ZNK12QDirIterator4pathEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QDirIterator::QDirIterator(const QDirIterator & );
  fn dector_ZN12QDirIteratorC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN12QDirIteratorC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QString QDirIterator::next();
  fn _ZN12QDirIterator4nextEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QString QDirIterator::filePath();
  fn _ZNK12QDirIterator8filePathEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QDirIterator::~QDirIterator();
  fn _ZN12QDirIteratorD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QFileInfo QDirIterator::fileInfo();
  fn _ZNK12QDirIterator8fileInfoEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QDirIterator::hasNext();
  fn _ZNK12QDirIterator7hasNextEv(qthis: u64 /* *mut c_void*/) -> c_char;
} // <= ext block end

// body block begin =>
// class sizeof(QDirIterator)=1
#[derive(Default)]
pub struct QDirIterator {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QDirIterator {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QDirIterator {
    return QDirIterator{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  QString QDirIterator::fileName();
impl /*struct*/ QDirIterator {
  pub fn fileName<RetType, T: QDirIterator_fileName<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.fileName(self);
    // return 1;
  }
}

pub trait QDirIterator_fileName<RetType> {
  fn fileName(self , rsthis: & QDirIterator) -> RetType;
}

  // proto:  QString QDirIterator::fileName();
impl<'a> /*trait*/ QDirIterator_fileName<QString> for () {
  fn fileName(self , rsthis: & QDirIterator) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QDirIterator8fileNameEv()};
    let mut ret = unsafe {_ZNK12QDirIterator8fileNameEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QDirIterator::path();
impl /*struct*/ QDirIterator {
  pub fn path<RetType, T: QDirIterator_path<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.path(self);
    // return 1;
  }
}

pub trait QDirIterator_path<RetType> {
  fn path(self , rsthis: & QDirIterator) -> RetType;
}

  // proto:  QString QDirIterator::path();
impl<'a> /*trait*/ QDirIterator_path<QString> for () {
  fn path(self , rsthis: & QDirIterator) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QDirIterator4pathEv()};
    let mut ret = unsafe {_ZNK12QDirIterator4pathEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QDirIterator::QDirIterator(const QDirIterator & );
impl /*struct*/ QDirIterator {
  pub fn new<T: QDirIterator_new>(value: T) -> QDirIterator {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QDirIterator_new {
  fn new(self) -> QDirIterator;
}

  // proto:  void QDirIterator::QDirIterator(const QDirIterator & );
impl<'a> /*trait*/ QDirIterator_new for (&'a QDirIterator) {
  fn new(self) -> QDirIterator {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QDirIteratorC1ERKS_()};
    let ctysz: c_int = unsafe{QDirIterator_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN12QDirIteratorC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN12QDirIteratorC1ERKS_(arg0)} as u64;
    let rsthis = QDirIterator{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QString QDirIterator::next();
impl /*struct*/ QDirIterator {
  pub fn next<RetType, T: QDirIterator_next<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.next(self);
    // return 1;
  }
}

pub trait QDirIterator_next<RetType> {
  fn next(self , rsthis: & QDirIterator) -> RetType;
}

  // proto:  QString QDirIterator::next();
impl<'a> /*trait*/ QDirIterator_next<QString> for () {
  fn next(self , rsthis: & QDirIterator) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QDirIterator4nextEv()};
    let mut ret = unsafe {_ZN12QDirIterator4nextEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QDirIterator::filePath();
impl /*struct*/ QDirIterator {
  pub fn filePath<RetType, T: QDirIterator_filePath<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.filePath(self);
    // return 1;
  }
}

pub trait QDirIterator_filePath<RetType> {
  fn filePath(self , rsthis: & QDirIterator) -> RetType;
}

  // proto:  QString QDirIterator::filePath();
impl<'a> /*trait*/ QDirIterator_filePath<QString> for () {
  fn filePath(self , rsthis: & QDirIterator) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QDirIterator8filePathEv()};
    let mut ret = unsafe {_ZNK12QDirIterator8filePathEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QDirIterator::~QDirIterator();
impl /*struct*/ QDirIterator {
  pub fn free<RetType, T: QDirIterator_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QDirIterator_free<RetType> {
  fn free(self , rsthis: & QDirIterator) -> RetType;
}

  // proto:  void QDirIterator::~QDirIterator();
impl<'a> /*trait*/ QDirIterator_free<()> for () {
  fn free(self , rsthis: & QDirIterator) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QDirIteratorD0Ev()};
     unsafe {_ZN12QDirIteratorD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QFileInfo QDirIterator::fileInfo();
impl /*struct*/ QDirIterator {
  pub fn fileInfo<RetType, T: QDirIterator_fileInfo<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.fileInfo(self);
    // return 1;
  }
}

pub trait QDirIterator_fileInfo<RetType> {
  fn fileInfo(self , rsthis: & QDirIterator) -> RetType;
}

  // proto:  QFileInfo QDirIterator::fileInfo();
impl<'a> /*trait*/ QDirIterator_fileInfo<QFileInfo> for () {
  fn fileInfo(self , rsthis: & QDirIterator) -> QFileInfo {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QDirIterator8fileInfoEv()};
    let mut ret = unsafe {_ZNK12QDirIterator8fileInfoEv(rsthis.qclsinst)};
    let mut ret1 = QFileInfo::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QDirIterator::hasNext();
impl /*struct*/ QDirIterator {
  pub fn hasNext<RetType, T: QDirIterator_hasNext<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasNext(self);
    // return 1;
  }
}

pub trait QDirIterator_hasNext<RetType> {
  fn hasNext(self , rsthis: & QDirIterator) -> RetType;
}

  // proto:  bool QDirIterator::hasNext();
impl<'a> /*trait*/ QDirIterator_hasNext<i8> for () {
  fn hasNext(self , rsthis: & QDirIterator) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QDirIterator7hasNextEv()};
    let mut ret = unsafe {_ZNK12QDirIterator7hasNextEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// <= body block end

