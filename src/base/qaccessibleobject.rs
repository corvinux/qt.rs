// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qobject::QObject;
use super::qrect::QRect;
use super::qaccessibleinterface::QAccessibleInterface;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QAccessibleObject::NewQAccessibleObject(QObject * object);
  fn _ZN17QAccessibleObjectC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QObject * QAccessibleObject::object();
  fn _ZNK17QAccessibleObject6objectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QRect QAccessibleObject::rect();
  fn _ZNK17QAccessibleObject4rectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QAccessibleInterface * QAccessibleObject::childAt(int x, int y);
  fn _ZNK17QAccessibleObject7childAtEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  void QAccessibleObject::NewQAccessibleObject(const QAccessibleObject & );
  fn _ZN17QAccessibleObjectC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QAccessibleObject::isValid();
  fn _ZNK17QAccessibleObject7isValidEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QAccessibleObject::FreeQAccessibleObject();
  fn _ZN17QAccessibleObjectD0Ev(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QAccessibleObject)=16
pub struct QAccessibleObject {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QAccessibleObject {
  pub fn NewQAccessibleObject<T: QAccessibleObject_NewQAccessibleObject>(value: T) -> QAccessibleObject {
    let rsthis = value.NewQAccessibleObject();
    return rsthis;
    // return 1;
  }
}

pub trait QAccessibleObject_NewQAccessibleObject {
  fn NewQAccessibleObject(self) -> QAccessibleObject;
}

// proto: void QAccessibleObject::NewQAccessibleObject(QObject * object);
impl<'a> /*trait*/ QAccessibleObject_NewQAccessibleObject for (&'a mut QObject) {
  fn NewQAccessibleObject(self) -> QAccessibleObject {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QAccessibleObjectC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN17QAccessibleObjectC1EP7QObject(qthis, arg0)};
    let rsthis = QAccessibleObject{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QAccessibleObject {
  pub fn object<T: QAccessibleObject_object>(&mut self, value: T) -> QObject {
    return value.object(self);
    // return 1;
  }
}

pub trait QAccessibleObject_object {
  fn object(self, rsthis: &mut QAccessibleObject) -> QObject;
}

// proto:  QObject * QAccessibleObject::object();
impl<'a> /*trait*/ QAccessibleObject_object for () {
  fn object(self, rsthis: &mut QAccessibleObject) -> QObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QAccessibleObject6objectEv()};
    let mut ret = unsafe {_ZNK17QAccessibleObject6objectEv(rsthis.qclsinst)};
    let mut ret1 = QObject{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QAccessibleObject {
  pub fn rect<T: QAccessibleObject_rect>(&mut self, value: T) -> QRect {
    return value.rect(self);
    // return 1;
  }
}

pub trait QAccessibleObject_rect {
  fn rect(self, rsthis: &mut QAccessibleObject) -> QRect;
}

// proto:  QRect QAccessibleObject::rect();
impl<'a> /*trait*/ QAccessibleObject_rect for () {
  fn rect(self, rsthis: &mut QAccessibleObject) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QAccessibleObject4rectEv()};
    let mut ret = unsafe {_ZNK17QAccessibleObject4rectEv(rsthis.qclsinst)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QAccessibleObject {
  pub fn childAt<T: QAccessibleObject_childAt>(&mut self, value: T) -> QAccessibleInterface {
    return value.childAt(self);
    // return 1;
  }
}

pub trait QAccessibleObject_childAt {
  fn childAt(self, rsthis: &mut QAccessibleObject) -> QAccessibleInterface;
}

// proto:  QAccessibleInterface * QAccessibleObject::childAt(int x, int y);
impl<'a> /*trait*/ QAccessibleObject_childAt for (i32, i32) {
  fn childAt(self, rsthis: &mut QAccessibleObject) -> QAccessibleInterface {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QAccessibleObject7childAtEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK17QAccessibleObject7childAtEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QAccessibleInterface{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QAccessibleObject::NewQAccessibleObject(const QAccessibleObject & );
impl<'a> /*trait*/ QAccessibleObject_NewQAccessibleObject for (&'a  QAccessibleObject) {
  fn NewQAccessibleObject(self) -> QAccessibleObject {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QAccessibleObjectC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN17QAccessibleObjectC1ERKS_(qthis, arg0)};
    let rsthis = QAccessibleObject{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QAccessibleObject {
  pub fn isValid<T: QAccessibleObject_isValid>(&mut self, value: T) -> i8 {
    return value.isValid(self);
    // return 1;
  }
}

pub trait QAccessibleObject_isValid {
  fn isValid(self, rsthis: &mut QAccessibleObject) -> i8;
}

// proto:  bool QAccessibleObject::isValid();
impl<'a> /*trait*/ QAccessibleObject_isValid for () {
  fn isValid(self, rsthis: &mut QAccessibleObject) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QAccessibleObject7isValidEv()};
    let mut ret = unsafe {_ZNK17QAccessibleObject7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QAccessibleObject {
  pub fn FreeQAccessibleObject<T: QAccessibleObject_FreeQAccessibleObject>(&mut self, value: T)  {
     value.FreeQAccessibleObject(self);
    // return 1;
  }
}

pub trait QAccessibleObject_FreeQAccessibleObject {
  fn FreeQAccessibleObject(self, rsthis: &mut QAccessibleObject) ;
}

// proto:  void QAccessibleObject::FreeQAccessibleObject();
impl<'a> /*trait*/ QAccessibleObject_FreeQAccessibleObject for () {
  fn FreeQAccessibleObject(self, rsthis: &mut QAccessibleObject)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QAccessibleObjectD0Ev()};
     unsafe {_ZN17QAccessibleObjectD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

