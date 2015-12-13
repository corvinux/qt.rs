// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qmatrix4x4::QMatrix4x4;
use super::qobject::QObject;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QGraphicsTransform::applyTo(QMatrix4x4 * matrix);
  fn _ZNK18QGraphicsTransform7applyToEP10QMatrix4x4(arg0: *mut c_void) -> i32;
  // proto: void QGraphicsTransform::FreeQGraphicsTransform();
  fn _ZN18QGraphicsTransformD0Ev() -> i32;
  // proto: void QGraphicsTransform::NewQGraphicsTransform(QObject * parent);
  fn _ZN18QGraphicsTransformC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: const QMetaObject * QGraphicsTransform::metaObject();
  fn _ZNK18QGraphicsTransform10metaObjectEv() -> i32;
}

// body block begin
// class sizeof(QGraphicsTransform)=1
pub struct QGraphicsTransform {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QGraphicsTransform {
  pub fn applyTo<T: QGraphicsTransform_applyTo>(&mut self, value: T) -> i32 {
    value.applyTo(self);
    return 1;
  }
}

pub trait QGraphicsTransform_applyTo {
  fn applyTo(self, this: &mut QGraphicsTransform) -> i32;
}

// proto: void QGraphicsTransform::applyTo(QMatrix4x4 * matrix);
impl<'a> /*trait*/ QGraphicsTransform_applyTo for (&'a mut QMatrix4x4) {
  fn applyTo(self, this: &mut QGraphicsTransform) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QGraphicsTransform7applyToEP10QMatrix4x4()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZNK18QGraphicsTransform7applyToEP10QMatrix4x4(arg0)};
    return 1;
  }
}

impl /*struct*/ QGraphicsTransform {
  pub fn FreeQGraphicsTransform<T: QGraphicsTransform_FreeQGraphicsTransform>(&mut self, value: T) -> i32 {
    value.FreeQGraphicsTransform(self);
    return 1;
  }
}

pub trait QGraphicsTransform_FreeQGraphicsTransform {
  fn FreeQGraphicsTransform(self, this: &mut QGraphicsTransform) -> i32;
}

// proto: void QGraphicsTransform::FreeQGraphicsTransform();
impl<'a> /*trait*/ QGraphicsTransform_FreeQGraphicsTransform for () {
  fn FreeQGraphicsTransform(self, this: &mut QGraphicsTransform) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QGraphicsTransformD0Ev()};
    unsafe {_ZN18QGraphicsTransformD0Ev()};
    return 1;
  }
}

impl /*struct*/ QGraphicsTransform {
  pub fn NewQGraphicsTransform<T: QGraphicsTransform_NewQGraphicsTransform>(value: T) -> QGraphicsTransform {
    let rsthis = value.NewQGraphicsTransform();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsTransform_NewQGraphicsTransform {
  fn NewQGraphicsTransform(self) -> QGraphicsTransform;
}

// proto: void QGraphicsTransform::NewQGraphicsTransform(QObject * parent);
impl<'a> /*trait*/ QGraphicsTransform_NewQGraphicsTransform for (&'a mut QObject) {
  fn NewQGraphicsTransform(self) -> QGraphicsTransform {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QGraphicsTransformC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN18QGraphicsTransformC1EP7QObject(qthis, arg0)};
    let rsthis = QGraphicsTransform{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGraphicsTransform {
  pub fn metaObject<T: QGraphicsTransform_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QGraphicsTransform_metaObject {
  fn metaObject(self, this: &mut QGraphicsTransform) -> i32;
}

// proto: const QMetaObject * QGraphicsTransform::metaObject();
impl<'a> /*trait*/ QGraphicsTransform_metaObject for () {
  fn metaObject(self, this: &mut QGraphicsTransform) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QGraphicsTransform10metaObjectEv()};
    unsafe {_ZNK18QGraphicsTransform10metaObjectEv()};
    return 1;
  }
}

