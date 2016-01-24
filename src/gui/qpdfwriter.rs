// auto generated, do not modify.
// created: Sun Jan 24 17:41:38 2016
// src-file: /QtGui/qpdfwriter.h
// dst-file: /src/gui/qpdfwriter.rs
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
use super::super::core::qsize::QSizeF; // 771
use super::super::core::qobjectdefs::QMetaObject; // 771
use super::super::core::qiodevice::QIODevice; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QPdfWriter_Class_Size() -> c_int;
  // proto:  void QPdfWriter::~QPdfWriter();
  fn C_ZN10QPdfWriterD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QPdfWriter::setCreator(const QString & creator);
  fn C_ZN10QPdfWriter10setCreatorERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QPdfWriter::setPageSizeMM(const QSizeF & size);
  fn C_ZN10QPdfWriter13setPageSizeMMERK6QSizeF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QPdfWriter::setResolution(int resolution);
  fn C_ZN10QPdfWriter13setResolutionEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  bool QPdfWriter::newPage();
  fn C_ZN10QPdfWriter7newPageEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QString QPdfWriter::title();
  fn C_ZNK10QPdfWriter5titleEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QString QPdfWriter::creator();
  fn C_ZNK10QPdfWriter7creatorEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QPdfWriter::resolution();
  fn C_ZNK10QPdfWriter10resolutionEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  const QMetaObject * QPdfWriter::metaObject();
  fn C_ZNK10QPdfWriter10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QPdfWriter::QPdfWriter(const QString & filename);
  fn C_ZN10QPdfWriterC2ERK7QString(arg0: *mut c_void) -> u64;
  // proto:  void QPdfWriter::QPdfWriter(QIODevice * device);
  fn C_ZN10QPdfWriterC2EP9QIODevice(arg0: *mut c_void) -> u64;
  // proto:  void QPdfWriter::setTitle(const QString & title);
  fn C_ZN10QPdfWriter8setTitleERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QPdfWriter)=1
#[derive(Default)]
pub struct QPdfWriter {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QPdfWriter {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QPdfWriter {
    return QPdfWriter{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QPdfWriter {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QPdfWriter {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  void QPdfWriter::~QPdfWriter();
impl /*struct*/ QPdfWriter {
  pub fn free<RetType, T: QPdfWriter_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QPdfWriter_free<RetType> {
  fn free(self , rsthis: & QPdfWriter) -> RetType;
}

  // proto:  void QPdfWriter::~QPdfWriter();
impl<'a> /*trait*/ QPdfWriter_free<()> for () {
  fn free(self , rsthis: & QPdfWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPdfWriterD2Ev()};
     unsafe {C_ZN10QPdfWriterD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QPdfWriter::setCreator(const QString & creator);
impl /*struct*/ QPdfWriter {
  pub fn setCreator<RetType, T: QPdfWriter_setCreator<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCreator(self);
    // return 1;
  }
}

pub trait QPdfWriter_setCreator<RetType> {
  fn setCreator(self , rsthis: & QPdfWriter) -> RetType;
}

  // proto:  void QPdfWriter::setCreator(const QString & creator);
impl<'a> /*trait*/ QPdfWriter_setCreator<()> for (&'a QString) {
  fn setCreator(self , rsthis: & QPdfWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPdfWriter10setCreatorERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN10QPdfWriter10setCreatorERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPdfWriter::setPageSizeMM(const QSizeF & size);
impl /*struct*/ QPdfWriter {
  pub fn setPageSizeMM<RetType, T: QPdfWriter_setPageSizeMM<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPageSizeMM(self);
    // return 1;
  }
}

pub trait QPdfWriter_setPageSizeMM<RetType> {
  fn setPageSizeMM(self , rsthis: & QPdfWriter) -> RetType;
}

  // proto:  void QPdfWriter::setPageSizeMM(const QSizeF & size);
impl<'a> /*trait*/ QPdfWriter_setPageSizeMM<()> for (&'a QSizeF) {
  fn setPageSizeMM(self , rsthis: & QPdfWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPdfWriter13setPageSizeMMERK6QSizeF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN10QPdfWriter13setPageSizeMMERK6QSizeF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPdfWriter::setResolution(int resolution);
impl /*struct*/ QPdfWriter {
  pub fn setResolution<RetType, T: QPdfWriter_setResolution<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setResolution(self);
    // return 1;
  }
}

pub trait QPdfWriter_setResolution<RetType> {
  fn setResolution(self , rsthis: & QPdfWriter) -> RetType;
}

  // proto:  void QPdfWriter::setResolution(int resolution);
impl<'a> /*trait*/ QPdfWriter_setResolution<()> for (i32) {
  fn setResolution(self , rsthis: & QPdfWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPdfWriter13setResolutionEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN10QPdfWriter13setResolutionEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QPdfWriter::newPage();
impl /*struct*/ QPdfWriter {
  pub fn newPage<RetType, T: QPdfWriter_newPage<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.newPage(self);
    // return 1;
  }
}

pub trait QPdfWriter_newPage<RetType> {
  fn newPage(self , rsthis: & QPdfWriter) -> RetType;
}

  // proto:  bool QPdfWriter::newPage();
impl<'a> /*trait*/ QPdfWriter_newPage<i8> for () {
  fn newPage(self , rsthis: & QPdfWriter) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPdfWriter7newPageEv()};
    let mut ret = unsafe {C_ZN10QPdfWriter7newPageEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QString QPdfWriter::title();
impl /*struct*/ QPdfWriter {
  pub fn title<RetType, T: QPdfWriter_title<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.title(self);
    // return 1;
  }
}

pub trait QPdfWriter_title<RetType> {
  fn title(self , rsthis: & QPdfWriter) -> RetType;
}

  // proto:  QString QPdfWriter::title();
impl<'a> /*trait*/ QPdfWriter_title<QString> for () {
  fn title(self , rsthis: & QPdfWriter) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QPdfWriter5titleEv()};
    let mut ret = unsafe {C_ZNK10QPdfWriter5titleEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QPdfWriter::creator();
impl /*struct*/ QPdfWriter {
  pub fn creator<RetType, T: QPdfWriter_creator<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.creator(self);
    // return 1;
  }
}

pub trait QPdfWriter_creator<RetType> {
  fn creator(self , rsthis: & QPdfWriter) -> RetType;
}

  // proto:  QString QPdfWriter::creator();
impl<'a> /*trait*/ QPdfWriter_creator<QString> for () {
  fn creator(self , rsthis: & QPdfWriter) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QPdfWriter7creatorEv()};
    let mut ret = unsafe {C_ZNK10QPdfWriter7creatorEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QPdfWriter::resolution();
impl /*struct*/ QPdfWriter {
  pub fn resolution<RetType, T: QPdfWriter_resolution<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.resolution(self);
    // return 1;
  }
}

pub trait QPdfWriter_resolution<RetType> {
  fn resolution(self , rsthis: & QPdfWriter) -> RetType;
}

  // proto:  int QPdfWriter::resolution();
impl<'a> /*trait*/ QPdfWriter_resolution<i32> for () {
  fn resolution(self , rsthis: & QPdfWriter) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QPdfWriter10resolutionEv()};
    let mut ret = unsafe {C_ZNK10QPdfWriter10resolutionEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  const QMetaObject * QPdfWriter::metaObject();
impl /*struct*/ QPdfWriter {
  pub fn metaObject<RetType, T: QPdfWriter_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QPdfWriter_metaObject<RetType> {
  fn metaObject(self , rsthis: & QPdfWriter) -> RetType;
}

  // proto:  const QMetaObject * QPdfWriter::metaObject();
impl<'a> /*trait*/ QPdfWriter_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QPdfWriter) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QPdfWriter10metaObjectEv()};
    let mut ret = unsafe {C_ZNK10QPdfWriter10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPdfWriter::QPdfWriter(const QString & filename);
impl /*struct*/ QPdfWriter {
  pub fn new<T: QPdfWriter_new>(value: T) -> QPdfWriter {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QPdfWriter_new {
  fn new(self) -> QPdfWriter;
}

  // proto:  void QPdfWriter::QPdfWriter(const QString & filename);
impl<'a> /*trait*/ QPdfWriter_new for (&'a QString) {
  fn new(self) -> QPdfWriter {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPdfWriterC2ERK7QString()};
    let ctysz: c_int = unsafe{QPdfWriter_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN10QPdfWriterC2ERK7QString(arg0)};
    let rsthis = QPdfWriter{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QPdfWriter::QPdfWriter(QIODevice * device);
impl<'a> /*trait*/ QPdfWriter_new for (&'a QIODevice) {
  fn new(self) -> QPdfWriter {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPdfWriterC2EP9QIODevice()};
    let ctysz: c_int = unsafe{QPdfWriter_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN10QPdfWriterC2EP9QIODevice(arg0)};
    let rsthis = QPdfWriter{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QPdfWriter::setTitle(const QString & title);
impl /*struct*/ QPdfWriter {
  pub fn setTitle<RetType, T: QPdfWriter_setTitle<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTitle(self);
    // return 1;
  }
}

pub trait QPdfWriter_setTitle<RetType> {
  fn setTitle(self , rsthis: & QPdfWriter) -> RetType;
}

  // proto:  void QPdfWriter::setTitle(const QString & title);
impl<'a> /*trait*/ QPdfWriter_setTitle<()> for (&'a QString) {
  fn setTitle(self , rsthis: & QPdfWriter) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QPdfWriter8setTitleERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN10QPdfWriter8setTitleERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// <= body block end

