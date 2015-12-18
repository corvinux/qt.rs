// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qobject::QObject;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QOpenGLVertexArrayObject::NewQOpenGLVertexArrayObject(const QOpenGLVertexArrayObject & );
  fn _ZN24QOpenGLVertexArrayObjectC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QOpenGLVertexArrayObject::GLuint QOpenGLVertexArrayObject::objectId();
  fn _ZNK24QOpenGLVertexArrayObject8objectIdEv(qthis: *mut c_void) ;
  // proto:  void QOpenGLVertexArrayObject::release();
  fn _ZN24QOpenGLVertexArrayObject7releaseEv(qthis: *mut c_void) ;
  // proto:  const QMetaObject * QOpenGLVertexArrayObject::metaObject();
  fn _ZNK24QOpenGLVertexArrayObject10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QOpenGLVertexArrayObject::NewQOpenGLVertexArrayObject(QObject * parent);
  fn _ZN24QOpenGLVertexArrayObjectC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QOpenGLVertexArrayObject::bind();
  fn _ZN24QOpenGLVertexArrayObject4bindEv(qthis: *mut c_void) ;
  // proto:  bool QOpenGLVertexArrayObject::isCreated();
  fn _ZNK24QOpenGLVertexArrayObject9isCreatedEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QOpenGLVertexArrayObject::destroy();
  fn _ZN24QOpenGLVertexArrayObject7destroyEv(qthis: *mut c_void) ;
  // proto:  bool QOpenGLVertexArrayObject::create();
  fn _ZN24QOpenGLVertexArrayObject6createEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QOpenGLVertexArrayObject::FreeQOpenGLVertexArrayObject();
  fn _ZN24QOpenGLVertexArrayObjectD0Ev(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QOpenGLVertexArrayObject)=1
pub struct QOpenGLVertexArrayObject {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QOpenGLVertexArrayObject {
  pub fn NewQOpenGLVertexArrayObject<T: QOpenGLVertexArrayObject_NewQOpenGLVertexArrayObject>(value: T) -> QOpenGLVertexArrayObject {
    let rsthis = value.NewQOpenGLVertexArrayObject();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLVertexArrayObject_NewQOpenGLVertexArrayObject {
  fn NewQOpenGLVertexArrayObject(self) -> QOpenGLVertexArrayObject;
}

// proto: void QOpenGLVertexArrayObject::NewQOpenGLVertexArrayObject(const QOpenGLVertexArrayObject & );
impl<'a> /*trait*/ QOpenGLVertexArrayObject_NewQOpenGLVertexArrayObject for (&'a  QOpenGLVertexArrayObject) {
  fn NewQOpenGLVertexArrayObject(self) -> QOpenGLVertexArrayObject {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QOpenGLVertexArrayObjectC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN24QOpenGLVertexArrayObjectC1ERKS_(qthis, arg0)};
    let rsthis = QOpenGLVertexArrayObject{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QOpenGLVertexArrayObject {
  pub fn objectId<RetType, T: QOpenGLVertexArrayObject_objectId<RetType>>(&mut self, value: T) -> RetType {
    return value.objectId(self);
    // return 1;
  }
}

pub trait QOpenGLVertexArrayObject_objectId<RetType> {
  fn objectId(self, rsthis: &mut QOpenGLVertexArrayObject) -> RetType;
}

// proto:  QOpenGLVertexArrayObject::GLuint QOpenGLVertexArrayObject::objectId();
impl<'a> /*trait*/ QOpenGLVertexArrayObject_objectId<()> for () {
  fn objectId(self, rsthis: &mut QOpenGLVertexArrayObject) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QOpenGLVertexArrayObject8objectIdEv()};
     unsafe {_ZNK24QOpenGLVertexArrayObject8objectIdEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLVertexArrayObject {
  pub fn release<RetType, T: QOpenGLVertexArrayObject_release<RetType>>(&mut self, value: T) -> RetType {
    return value.release(self);
    // return 1;
  }
}

pub trait QOpenGLVertexArrayObject_release<RetType> {
  fn release(self, rsthis: &mut QOpenGLVertexArrayObject) -> RetType;
}

// proto:  void QOpenGLVertexArrayObject::release();
impl<'a> /*trait*/ QOpenGLVertexArrayObject_release<()> for () {
  fn release(self, rsthis: &mut QOpenGLVertexArrayObject) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QOpenGLVertexArrayObject7releaseEv()};
     unsafe {_ZN24QOpenGLVertexArrayObject7releaseEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLVertexArrayObject {
  pub fn metaObject<RetType, T: QOpenGLVertexArrayObject_metaObject<RetType>>(&mut self, value: T) -> RetType {
    return value.metaObject(self);
    // return 1;
  }
}

pub trait QOpenGLVertexArrayObject_metaObject<RetType> {
  fn metaObject(self, rsthis: &mut QOpenGLVertexArrayObject) -> RetType;
}

// proto:  const QMetaObject * QOpenGLVertexArrayObject::metaObject();
impl<'a> /*trait*/ QOpenGLVertexArrayObject_metaObject<()> for () {
  fn metaObject(self, rsthis: &mut QOpenGLVertexArrayObject) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QOpenGLVertexArrayObject10metaObjectEv()};
     unsafe {_ZNK24QOpenGLVertexArrayObject10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto: void QOpenGLVertexArrayObject::NewQOpenGLVertexArrayObject(QObject * parent);
impl<'a> /*trait*/ QOpenGLVertexArrayObject_NewQOpenGLVertexArrayObject for (&'a mut QObject) {
  fn NewQOpenGLVertexArrayObject(self) -> QOpenGLVertexArrayObject {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QOpenGLVertexArrayObjectC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN24QOpenGLVertexArrayObjectC1EP7QObject(qthis, arg0)};
    let rsthis = QOpenGLVertexArrayObject{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QOpenGLVertexArrayObject {
  pub fn bind<RetType, T: QOpenGLVertexArrayObject_bind<RetType>>(&mut self, value: T) -> RetType {
    return value.bind(self);
    // return 1;
  }
}

pub trait QOpenGLVertexArrayObject_bind<RetType> {
  fn bind(self, rsthis: &mut QOpenGLVertexArrayObject) -> RetType;
}

// proto:  void QOpenGLVertexArrayObject::bind();
impl<'a> /*trait*/ QOpenGLVertexArrayObject_bind<()> for () {
  fn bind(self, rsthis: &mut QOpenGLVertexArrayObject) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QOpenGLVertexArrayObject4bindEv()};
     unsafe {_ZN24QOpenGLVertexArrayObject4bindEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLVertexArrayObject {
  pub fn isCreated<RetType, T: QOpenGLVertexArrayObject_isCreated<RetType>>(&mut self, value: T) -> RetType {
    return value.isCreated(self);
    // return 1;
  }
}

pub trait QOpenGLVertexArrayObject_isCreated<RetType> {
  fn isCreated(self, rsthis: &mut QOpenGLVertexArrayObject) -> RetType;
}

// proto:  bool QOpenGLVertexArrayObject::isCreated();
impl<'a> /*trait*/ QOpenGLVertexArrayObject_isCreated<i8> for () {
  fn isCreated(self, rsthis: &mut QOpenGLVertexArrayObject) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK24QOpenGLVertexArrayObject9isCreatedEv()};
    let mut ret = unsafe {_ZNK24QOpenGLVertexArrayObject9isCreatedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QOpenGLVertexArrayObject {
  pub fn destroy<RetType, T: QOpenGLVertexArrayObject_destroy<RetType>>(&mut self, value: T) -> RetType {
    return value.destroy(self);
    // return 1;
  }
}

pub trait QOpenGLVertexArrayObject_destroy<RetType> {
  fn destroy(self, rsthis: &mut QOpenGLVertexArrayObject) -> RetType;
}

// proto:  void QOpenGLVertexArrayObject::destroy();
impl<'a> /*trait*/ QOpenGLVertexArrayObject_destroy<()> for () {
  fn destroy(self, rsthis: &mut QOpenGLVertexArrayObject) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QOpenGLVertexArrayObject7destroyEv()};
     unsafe {_ZN24QOpenGLVertexArrayObject7destroyEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOpenGLVertexArrayObject {
  pub fn create<RetType, T: QOpenGLVertexArrayObject_create<RetType>>(&mut self, value: T) -> RetType {
    return value.create(self);
    // return 1;
  }
}

pub trait QOpenGLVertexArrayObject_create<RetType> {
  fn create(self, rsthis: &mut QOpenGLVertexArrayObject) -> RetType;
}

// proto:  bool QOpenGLVertexArrayObject::create();
impl<'a> /*trait*/ QOpenGLVertexArrayObject_create<i8> for () {
  fn create(self, rsthis: &mut QOpenGLVertexArrayObject) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QOpenGLVertexArrayObject6createEv()};
    let mut ret = unsafe {_ZN24QOpenGLVertexArrayObject6createEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QOpenGLVertexArrayObject {
  pub fn FreeQOpenGLVertexArrayObject<RetType, T: QOpenGLVertexArrayObject_FreeQOpenGLVertexArrayObject<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQOpenGLVertexArrayObject(self);
    // return 1;
  }
}

pub trait QOpenGLVertexArrayObject_FreeQOpenGLVertexArrayObject<RetType> {
  fn FreeQOpenGLVertexArrayObject(self, rsthis: &mut QOpenGLVertexArrayObject) -> RetType;
}

// proto:  void QOpenGLVertexArrayObject::FreeQOpenGLVertexArrayObject();
impl<'a> /*trait*/ QOpenGLVertexArrayObject_FreeQOpenGLVertexArrayObject<()> for () {
  fn FreeQOpenGLVertexArrayObject(self, rsthis: &mut QOpenGLVertexArrayObject) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QOpenGLVertexArrayObjectD0Ev()};
     unsafe {_ZN24QOpenGLVertexArrayObjectD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

