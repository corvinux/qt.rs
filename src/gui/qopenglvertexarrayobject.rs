// auto generated, do not modify.
// created: Fri Jan  1 15:54:32 2016
// src-file: /QtGui/qopenglvertexarrayobject.h
// dst-file: /src/gui/qopenglvertexarrayobject.rs
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
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QOpenGLVertexArrayObject_Class_Size() -> c_int;
  // proto:  void QOpenGLVertexArrayObject::QOpenGLVertexArrayObject(const QOpenGLVertexArrayObject & );
  fn dector_ZN24QOpenGLVertexArrayObjectC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN24QOpenGLVertexArrayObjectC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  GLuint QOpenGLVertexArrayObject::objectId();
  fn _ZNK24QOpenGLVertexArrayObject8objectIdEv(qthis: u64 /* *mut c_void*/) -> c_uint;
  // proto:  void QOpenGLVertexArrayObject::release();
  fn _ZN24QOpenGLVertexArrayObject7releaseEv(qthis: u64 /* *mut c_void*/);
  // proto:  const QMetaObject * QOpenGLVertexArrayObject::metaObject();
  fn _ZNK24QOpenGLVertexArrayObject10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QOpenGLVertexArrayObject::QOpenGLVertexArrayObject(QObject * parent);
  fn dector_ZN24QOpenGLVertexArrayObjectC1EP7QObject(arg0: *mut c_void) -> *mut c_void;
  fn _ZN24QOpenGLVertexArrayObjectC1EP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QOpenGLVertexArrayObject::bind();
  fn _ZN24QOpenGLVertexArrayObject4bindEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QOpenGLVertexArrayObject::isCreated();
  fn _ZNK24QOpenGLVertexArrayObject9isCreatedEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QOpenGLVertexArrayObject::destroy();
  fn _ZN24QOpenGLVertexArrayObject7destroyEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QOpenGLVertexArrayObject::create();
  fn _ZN24QOpenGLVertexArrayObject6createEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QOpenGLVertexArrayObject::~QOpenGLVertexArrayObject();
  fn _ZN24QOpenGLVertexArrayObjectD0Ev(qthis: u64 /* *mut c_void*/);
} // <= ext block end

// body block begin =>
// class sizeof(QOpenGLVertexArrayObject)=1
#[derive(Default)]
pub struct QOpenGLVertexArrayObject {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QOpenGLVertexArrayObject {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QOpenGLVertexArrayObject {
    return QOpenGLVertexArrayObject{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QOpenGLVertexArrayObject {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QOpenGLVertexArrayObject {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  void QOpenGLVertexArrayObject::QOpenGLVertexArrayObject(const QOpenGLVertexArrayObject & );
impl /*struct*/ QOpenGLVertexArrayObject {
  pub fn new<T: QOpenGLVertexArrayObject_new>(value: T) -> QOpenGLVertexArrayObject {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLVertexArrayObject_new {
  fn new(self) -> QOpenGLVertexArrayObject;
}

  // proto:  void QOpenGLVertexArrayObject::QOpenGLVertexArrayObject(const QOpenGLVertexArrayObject & );
impl<'a> /*trait*/ QOpenGLVertexArrayObject_new for (&'a QOpenGLVertexArrayObject) {
  fn new(self) -> QOpenGLVertexArrayObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QOpenGLVertexArrayObjectC1ERKS_()};
    let ctysz: c_int = unsafe{QOpenGLVertexArrayObject_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN24QOpenGLVertexArrayObjectC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN24QOpenGLVertexArrayObjectC1ERKS_(arg0)} as u64;
    let rsthis = QOpenGLVertexArrayObject{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  GLuint QOpenGLVertexArrayObject::objectId();
impl /*struct*/ QOpenGLVertexArrayObject {
  pub fn objectId<RetType, T: QOpenGLVertexArrayObject_objectId<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.objectId(self);
    // return 1;
  }
}

pub trait QOpenGLVertexArrayObject_objectId<RetType> {
  fn objectId(self , rsthis: & QOpenGLVertexArrayObject) -> RetType;
}

  // proto:  GLuint QOpenGLVertexArrayObject::objectId();
impl<'a> /*trait*/ QOpenGLVertexArrayObject_objectId<u32> for () {
  fn objectId(self , rsthis: & QOpenGLVertexArrayObject) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QOpenGLVertexArrayObject8objectIdEv()};
    let mut ret = unsafe {_ZNK24QOpenGLVertexArrayObject8objectIdEv(rsthis.qclsinst)};
    return ret as u32;
    // return 1;
  }
}

  // proto:  void QOpenGLVertexArrayObject::release();
impl /*struct*/ QOpenGLVertexArrayObject {
  pub fn release<RetType, T: QOpenGLVertexArrayObject_release<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.release(self);
    // return 1;
  }
}

pub trait QOpenGLVertexArrayObject_release<RetType> {
  fn release(self , rsthis: & QOpenGLVertexArrayObject) -> RetType;
}

  // proto:  void QOpenGLVertexArrayObject::release();
impl<'a> /*trait*/ QOpenGLVertexArrayObject_release<()> for () {
  fn release(self , rsthis: & QOpenGLVertexArrayObject) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QOpenGLVertexArrayObject7releaseEv()};
     unsafe {_ZN24QOpenGLVertexArrayObject7releaseEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QOpenGLVertexArrayObject::metaObject();
impl /*struct*/ QOpenGLVertexArrayObject {
  pub fn metaObject<RetType, T: QOpenGLVertexArrayObject_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QOpenGLVertexArrayObject_metaObject<RetType> {
  fn metaObject(self , rsthis: & QOpenGLVertexArrayObject) -> RetType;
}

  // proto:  const QMetaObject * QOpenGLVertexArrayObject::metaObject();
impl<'a> /*trait*/ QOpenGLVertexArrayObject_metaObject<()> for () {
  fn metaObject(self , rsthis: & QOpenGLVertexArrayObject) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QOpenGLVertexArrayObject10metaObjectEv()};
     unsafe {_ZNK24QOpenGLVertexArrayObject10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QOpenGLVertexArrayObject::QOpenGLVertexArrayObject(QObject * parent);
impl<'a> /*trait*/ QOpenGLVertexArrayObject_new for (&'a QObject) {
  fn new(self) -> QOpenGLVertexArrayObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QOpenGLVertexArrayObjectC1EP7QObject()};
    let ctysz: c_int = unsafe{QOpenGLVertexArrayObject_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN24QOpenGLVertexArrayObjectC1EP7QObject(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN24QOpenGLVertexArrayObjectC1EP7QObject(arg0)} as u64;
    let rsthis = QOpenGLVertexArrayObject{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QOpenGLVertexArrayObject::bind();
impl /*struct*/ QOpenGLVertexArrayObject {
  pub fn bind<RetType, T: QOpenGLVertexArrayObject_bind<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.bind(self);
    // return 1;
  }
}

pub trait QOpenGLVertexArrayObject_bind<RetType> {
  fn bind(self , rsthis: & QOpenGLVertexArrayObject) -> RetType;
}

  // proto:  void QOpenGLVertexArrayObject::bind();
impl<'a> /*trait*/ QOpenGLVertexArrayObject_bind<()> for () {
  fn bind(self , rsthis: & QOpenGLVertexArrayObject) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QOpenGLVertexArrayObject4bindEv()};
     unsafe {_ZN24QOpenGLVertexArrayObject4bindEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QOpenGLVertexArrayObject::isCreated();
impl /*struct*/ QOpenGLVertexArrayObject {
  pub fn isCreated<RetType, T: QOpenGLVertexArrayObject_isCreated<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isCreated(self);
    // return 1;
  }
}

pub trait QOpenGLVertexArrayObject_isCreated<RetType> {
  fn isCreated(self , rsthis: & QOpenGLVertexArrayObject) -> RetType;
}

  // proto:  bool QOpenGLVertexArrayObject::isCreated();
impl<'a> /*trait*/ QOpenGLVertexArrayObject_isCreated<i8> for () {
  fn isCreated(self , rsthis: & QOpenGLVertexArrayObject) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QOpenGLVertexArrayObject9isCreatedEv()};
    let mut ret = unsafe {_ZNK24QOpenGLVertexArrayObject9isCreatedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QOpenGLVertexArrayObject::destroy();
impl /*struct*/ QOpenGLVertexArrayObject {
  pub fn destroy<RetType, T: QOpenGLVertexArrayObject_destroy<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.destroy(self);
    // return 1;
  }
}

pub trait QOpenGLVertexArrayObject_destroy<RetType> {
  fn destroy(self , rsthis: & QOpenGLVertexArrayObject) -> RetType;
}

  // proto:  void QOpenGLVertexArrayObject::destroy();
impl<'a> /*trait*/ QOpenGLVertexArrayObject_destroy<()> for () {
  fn destroy(self , rsthis: & QOpenGLVertexArrayObject) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QOpenGLVertexArrayObject7destroyEv()};
     unsafe {_ZN24QOpenGLVertexArrayObject7destroyEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QOpenGLVertexArrayObject::create();
impl /*struct*/ QOpenGLVertexArrayObject {
  pub fn create<RetType, T: QOpenGLVertexArrayObject_create<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.create(self);
    // return 1;
  }
}

pub trait QOpenGLVertexArrayObject_create<RetType> {
  fn create(self , rsthis: & QOpenGLVertexArrayObject) -> RetType;
}

  // proto:  bool QOpenGLVertexArrayObject::create();
impl<'a> /*trait*/ QOpenGLVertexArrayObject_create<i8> for () {
  fn create(self , rsthis: & QOpenGLVertexArrayObject) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QOpenGLVertexArrayObject6createEv()};
    let mut ret = unsafe {_ZN24QOpenGLVertexArrayObject6createEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QOpenGLVertexArrayObject::~QOpenGLVertexArrayObject();
impl /*struct*/ QOpenGLVertexArrayObject {
  pub fn free<RetType, T: QOpenGLVertexArrayObject_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QOpenGLVertexArrayObject_free<RetType> {
  fn free(self , rsthis: & QOpenGLVertexArrayObject) -> RetType;
}

  // proto:  void QOpenGLVertexArrayObject::~QOpenGLVertexArrayObject();
impl<'a> /*trait*/ QOpenGLVertexArrayObject_free<()> for () {
  fn free(self , rsthis: & QOpenGLVertexArrayObject) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QOpenGLVertexArrayObjectD0Ev()};
     unsafe {_ZN24QOpenGLVertexArrayObjectD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

