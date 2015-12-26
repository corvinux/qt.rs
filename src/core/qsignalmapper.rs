// auto generated, do not modify.
// created: Sat Dec 26 10:16:52 2015
// src-file: /QtCore/qsignalmapper.h
// dst-file: /src/core/qsignalmapper.rs
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
use super::qobject::QObject; // 773
use std::ops::Deref;
use super::qstring::QString; // 773
use super::super::widgets::qwidget::QWidget; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QSignalMapper_Class_Size() -> c_int;
  // proto:  void QSignalMapper::removeMappings(QObject * sender);
  fn _ZN13QSignalMapper14removeMappingsEP7QObject(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QSignalMapper::map(QObject * sender);
  fn _ZN13QSignalMapper3mapEP7QObject(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QSignalMapper::mapped(int );
  fn _ZN13QSignalMapper6mappedEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QSignalMapper::QSignalMapper(const QSignalMapper & );
  fn dector_ZN13QSignalMapperC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN13QSignalMapperC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  const QMetaObject * QSignalMapper::metaObject();
  fn _ZNK13QSignalMapper10metaObjectEv(qthis: *mut c_void);
  // proto:  void QSignalMapper::mapped(QObject * );
  fn _ZN13QSignalMapper6mappedEP7QObject(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QSignalMapper::setMapping(QObject * sender, QObject * object);
  fn _ZN13QSignalMapper10setMappingEP7QObjectS1_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  QObject * QSignalMapper::mapping(int id);
  fn _ZNK13QSignalMapper7mappingEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  void QSignalMapper::QSignalMapper(QObject * parent);
  fn dector_ZN13QSignalMapperC1EP7QObject(arg0: *mut c_void) -> *mut c_void;
  fn _ZN13QSignalMapperC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QSignalMapper::~QSignalMapper();
  fn _ZN13QSignalMapperD0Ev(qthis: *mut c_void);
  // proto:  void QSignalMapper::mapped(const QString & );
  fn _ZN13QSignalMapper6mappedERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QSignalMapper::setMapping(QObject * sender, int id);
  fn _ZN13QSignalMapper10setMappingEP7QObjecti(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int);
  // proto:  QObject * QSignalMapper::mapping(const QString & text);
  fn _ZNK13QSignalMapper7mappingERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QSignalMapper::map();
  fn _ZN13QSignalMapper3mapEv(qthis: *mut c_void);
  // proto:  QObject * QSignalMapper::mapping(QObject * object);
  fn _ZNK13QSignalMapper7mappingEP7QObject(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QSignalMapper::mapped(QWidget * );
  fn _ZN13QSignalMapper6mappedEP7QWidget(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QSignalMapper::setMapping(QObject * sender, const QString & text);
  fn _ZN13QSignalMapper10setMappingEP7QObjectRK7QString(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  QObject * QSignalMapper::mapping(QWidget * widget);
  fn _ZNK13QSignalMapper7mappingEP7QWidget(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QSignalMapper::setMapping(QObject * sender, QWidget * widget);
  fn _ZN13QSignalMapper10setMappingEP7QObjectP7QWidget(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QSignalMapper)=1
pub struct QSignalMapper {
  qbase: QObject,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QSignalMapper {
  pub fn inheritFrom(qthis: *mut c_void) -> QSignalMapper {
    return QSignalMapper{qbase: QObject::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QSignalMapper {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QSignalMapper {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  void QSignalMapper::removeMappings(QObject * sender);
impl /*struct*/ QSignalMapper {
  pub fn removeMappings<RetType, T: QSignalMapper_removeMappings<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.removeMappings(self);
    // return 1;
  }
}

pub trait QSignalMapper_removeMappings<RetType> {
  fn removeMappings(self , rsthis: & QSignalMapper) -> RetType;
}

  // proto:  void QSignalMapper::removeMappings(QObject * sender);
impl<'a> /*trait*/ QSignalMapper_removeMappings<()> for (&'a QObject) {
  fn removeMappings(self , rsthis: & QSignalMapper) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSignalMapper14removeMappingsEP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QSignalMapper14removeMappingsEP7QObject(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QSignalMapper::map(QObject * sender);
impl /*struct*/ QSignalMapper {
  pub fn map<RetType, T: QSignalMapper_map<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.map(self);
    // return 1;
  }
}

pub trait QSignalMapper_map<RetType> {
  fn map(self , rsthis: & QSignalMapper) -> RetType;
}

  // proto:  void QSignalMapper::map(QObject * sender);
impl<'a> /*trait*/ QSignalMapper_map<()> for (&'a QObject) {
  fn map(self , rsthis: & QSignalMapper) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSignalMapper3mapEP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QSignalMapper3mapEP7QObject(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QSignalMapper::mapped(int );
impl /*struct*/ QSignalMapper {
  pub fn mapped<RetType, T: QSignalMapper_mapped<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mapped(self);
    // return 1;
  }
}

pub trait QSignalMapper_mapped<RetType> {
  fn mapped(self , rsthis: & QSignalMapper) -> RetType;
}

  // proto:  void QSignalMapper::mapped(int );
impl<'a> /*trait*/ QSignalMapper_mapped<()> for (i32) {
  fn mapped(self , rsthis: & QSignalMapper) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSignalMapper6mappedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN13QSignalMapper6mappedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QSignalMapper::QSignalMapper(const QSignalMapper & );
impl /*struct*/ QSignalMapper {
  pub fn New<T: QSignalMapper_New>(value: T) -> QSignalMapper {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QSignalMapper_New {
  fn New(self) -> QSignalMapper;
}

  // proto:  void QSignalMapper::QSignalMapper(const QSignalMapper & );
impl<'a> /*trait*/ QSignalMapper_New for (&'a QSignalMapper) {
  fn New(self) -> QSignalMapper {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSignalMapperC1ERKS_()};
    let ctysz: c_int = unsafe{QSignalMapper_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN13QSignalMapperC1ERKS_(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN13QSignalMapperC1ERKS_(arg0)};
    let rsthis = QSignalMapper{/**/qbase: QObject::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QMetaObject * QSignalMapper::metaObject();
impl /*struct*/ QSignalMapper {
  pub fn metaObject<RetType, T: QSignalMapper_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QSignalMapper_metaObject<RetType> {
  fn metaObject(self , rsthis: & QSignalMapper) -> RetType;
}

  // proto:  const QMetaObject * QSignalMapper::metaObject();
impl<'a> /*trait*/ QSignalMapper_metaObject<()> for () {
  fn metaObject(self , rsthis: & QSignalMapper) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QSignalMapper10metaObjectEv()};
     unsafe {_ZNK13QSignalMapper10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QSignalMapper::mapped(QObject * );
impl<'a> /*trait*/ QSignalMapper_mapped<()> for (&'a QObject) {
  fn mapped(self , rsthis: & QSignalMapper) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSignalMapper6mappedEP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QSignalMapper6mappedEP7QObject(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QSignalMapper::setMapping(QObject * sender, QObject * object);
impl /*struct*/ QSignalMapper {
  pub fn setMapping<RetType, T: QSignalMapper_setMapping<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMapping(self);
    // return 1;
  }
}

pub trait QSignalMapper_setMapping<RetType> {
  fn setMapping(self , rsthis: & QSignalMapper) -> RetType;
}

  // proto:  void QSignalMapper::setMapping(QObject * sender, QObject * object);
impl<'a> /*trait*/ QSignalMapper_setMapping<()> for (&'a QObject, &'a QObject) {
  fn setMapping(self , rsthis: & QSignalMapper) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSignalMapper10setMappingEP7QObjectS1_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN13QSignalMapper10setMappingEP7QObjectS1_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QObject * QSignalMapper::mapping(int id);
impl /*struct*/ QSignalMapper {
  pub fn mapping<RetType, T: QSignalMapper_mapping<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mapping(self);
    // return 1;
  }
}

pub trait QSignalMapper_mapping<RetType> {
  fn mapping(self , rsthis: & QSignalMapper) -> RetType;
}

  // proto:  QObject * QSignalMapper::mapping(int id);
impl<'a> /*trait*/ QSignalMapper_mapping<QObject> for (i32) {
  fn mapping(self , rsthis: & QSignalMapper) -> QObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QSignalMapper7mappingEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK13QSignalMapper7mappingEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QObject::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QSignalMapper::QSignalMapper(QObject * parent);
impl<'a> /*trait*/ QSignalMapper_New for (&'a QObject) {
  fn New(self) -> QSignalMapper {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSignalMapperC1EP7QObject()};
    let ctysz: c_int = unsafe{QSignalMapper_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN13QSignalMapperC1EP7QObject(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN13QSignalMapperC1EP7QObject(arg0)};
    let rsthis = QSignalMapper{/**/qbase: QObject::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QSignalMapper::~QSignalMapper();
impl /*struct*/ QSignalMapper {
  pub fn Free<RetType, T: QSignalMapper_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QSignalMapper_Free<RetType> {
  fn Free(self , rsthis: & QSignalMapper) -> RetType;
}

  // proto:  void QSignalMapper::~QSignalMapper();
impl<'a> /*trait*/ QSignalMapper_Free<()> for () {
  fn Free(self , rsthis: & QSignalMapper) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSignalMapperD0Ev()};
     unsafe {_ZN13QSignalMapperD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QSignalMapper::mapped(const QString & );
impl<'a> /*trait*/ QSignalMapper_mapped<()> for (&'a QString) {
  fn mapped(self , rsthis: & QSignalMapper) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSignalMapper6mappedERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QSignalMapper6mappedERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QSignalMapper::setMapping(QObject * sender, int id);
impl<'a> /*trait*/ QSignalMapper_setMapping<()> for (&'a QObject, i32) {
  fn setMapping(self , rsthis: & QSignalMapper) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSignalMapper10setMappingEP7QObjecti()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN13QSignalMapper10setMappingEP7QObjecti(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QObject * QSignalMapper::mapping(const QString & text);
impl<'a> /*trait*/ QSignalMapper_mapping<QObject> for (&'a QString) {
  fn mapping(self , rsthis: & QSignalMapper) -> QObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QSignalMapper7mappingERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QSignalMapper7mappingERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QObject::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QSignalMapper::map();
impl<'a> /*trait*/ QSignalMapper_map<()> for () {
  fn map(self , rsthis: & QSignalMapper) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSignalMapper3mapEv()};
     unsafe {_ZN13QSignalMapper3mapEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QObject * QSignalMapper::mapping(QObject * object);
impl<'a> /*trait*/ QSignalMapper_mapping<QObject> for (&'a QObject) {
  fn mapping(self , rsthis: & QSignalMapper) -> QObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QSignalMapper7mappingEP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QSignalMapper7mappingEP7QObject(rsthis.qclsinst, arg0)};
    let mut ret1 = QObject::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QSignalMapper::mapped(QWidget * );
impl<'a> /*trait*/ QSignalMapper_mapped<()> for (&'a QWidget) {
  fn mapped(self , rsthis: & QSignalMapper) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSignalMapper6mappedEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QSignalMapper6mappedEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QSignalMapper::setMapping(QObject * sender, const QString & text);
impl<'a> /*trait*/ QSignalMapper_setMapping<()> for (&'a QObject, &'a QString) {
  fn setMapping(self , rsthis: & QSignalMapper) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSignalMapper10setMappingEP7QObjectRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN13QSignalMapper10setMappingEP7QObjectRK7QString(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QObject * QSignalMapper::mapping(QWidget * widget);
impl<'a> /*trait*/ QSignalMapper_mapping<QObject> for (&'a QWidget) {
  fn mapping(self , rsthis: & QSignalMapper) -> QObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QSignalMapper7mappingEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QSignalMapper7mappingEP7QWidget(rsthis.qclsinst, arg0)};
    let mut ret1 = QObject::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QSignalMapper::setMapping(QObject * sender, QWidget * widget);
impl<'a> /*trait*/ QSignalMapper_setMapping<()> for (&'a QObject, &'a QWidget) {
  fn setMapping(self , rsthis: & QSignalMapper) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSignalMapper10setMappingEP7QObjectP7QWidget()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN13QSignalMapper10setMappingEP7QObjectP7QWidget(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// <= body block end

