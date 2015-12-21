// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qobject::QObject;
use super::qpointf::QPointF;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  QPointF QGesture::hotSpot();
  fn _ZNK8QGesture7hotSpotEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QGesture::hasHotSpot();
  fn _ZNK8QGesture10hasHotSpotEv(qthis: *mut c_void) -> c_char;
  // proto:  void QGesture::unsetHotSpot();
  fn _ZN8QGesture12unsetHotSpotEv(qthis: *mut c_void);
  // proto:  const QMetaObject * QGesture::metaObject();
  fn _ZNK8QGesture10metaObjectEv(qthis: *mut c_void);
  // proto:  void QGesture::QGesture(QObject * parent);
  fn _ZN8QGestureC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QGesture::setHotSpot(const QPointF & value);
  fn _ZN8QGesture10setHotSpotERK7QPointF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QGesture::~QGesture();
  fn _ZN8QGestureD0Ev(qthis: *mut c_void);
}

// body block begin
// class sizeof(QGesture)=1
pub struct QGesture {
  pub qclsinst: *mut c_void,
}

  // proto:  QPointF QGesture::hotSpot();
impl /*struct*/ QGesture {
  pub fn hotSpot<RetType, T: QGesture_hotSpot<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.hotSpot(self);
    // return 1;
  }
}

pub trait QGesture_hotSpot<RetType> {
  fn hotSpot(self , rsthis: &mut QGesture) -> RetType;
}

  // proto:  QPointF QGesture::hotSpot();
impl<'a> /*trait*/ QGesture_hotSpot<QPointF> for () {
  fn hotSpot(self , rsthis: &mut QGesture) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QGesture7hotSpotEv()};
    let mut ret = unsafe {_ZNK8QGesture7hotSpotEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  bool QGesture::hasHotSpot();
impl /*struct*/ QGesture {
  pub fn hasHotSpot<RetType, T: QGesture_hasHotSpot<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.hasHotSpot(self);
    // return 1;
  }
}

pub trait QGesture_hasHotSpot<RetType> {
  fn hasHotSpot(self , rsthis: &mut QGesture) -> RetType;
}

  // proto:  bool QGesture::hasHotSpot();
impl<'a> /*trait*/ QGesture_hasHotSpot<i8> for () {
  fn hasHotSpot(self , rsthis: &mut QGesture) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QGesture10hasHotSpotEv()};
    let mut ret = unsafe {_ZNK8QGesture10hasHotSpotEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QGesture::unsetHotSpot();
impl /*struct*/ QGesture {
  pub fn unsetHotSpot<RetType, T: QGesture_unsetHotSpot<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.unsetHotSpot(self);
    // return 1;
  }
}

pub trait QGesture_unsetHotSpot<RetType> {
  fn unsetHotSpot(self , rsthis: &mut QGesture) -> RetType;
}

  // proto:  void QGesture::unsetHotSpot();
impl<'a> /*trait*/ QGesture_unsetHotSpot<()> for () {
  fn unsetHotSpot(self , rsthis: &mut QGesture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QGesture12unsetHotSpotEv()};
     unsafe {_ZN8QGesture12unsetHotSpotEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QGesture::metaObject();
impl /*struct*/ QGesture {
  pub fn metaObject<RetType, T: QGesture_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QGesture_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QGesture) -> RetType;
}

  // proto:  const QMetaObject * QGesture::metaObject();
impl<'a> /*trait*/ QGesture_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QGesture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QGesture10metaObjectEv()};
     unsafe {_ZNK8QGesture10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGesture::QGesture(QObject * parent);
impl /*struct*/ QGesture {
  pub fn NewQGesture<T: QGesture_NewQGesture>(value: T) -> QGesture {
    let rsthis = value.NewQGesture();
    return rsthis;
    // return 1;
  }
}

pub trait QGesture_NewQGesture {
  fn NewQGesture(self) -> QGesture;
}

  // proto:  void QGesture::QGesture(QObject * parent);
impl<'a> /*trait*/ QGesture_NewQGesture for (QObject) {
  fn NewQGesture(self) -> QGesture {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QGestureC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN8QGestureC1EP7QObject(qthis, arg0)};
    let rsthis = QGesture{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QGesture::setHotSpot(const QPointF & value);
impl /*struct*/ QGesture {
  pub fn setHotSpot<RetType, T: QGesture_setHotSpot<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setHotSpot(self);
    // return 1;
  }
}

pub trait QGesture_setHotSpot<RetType> {
  fn setHotSpot(self , rsthis: &mut QGesture) -> RetType;
}

  // proto:  void QGesture::setHotSpot(const QPointF & value);
impl<'a> /*trait*/ QGesture_setHotSpot<()> for (QPointF) {
  fn setHotSpot(self , rsthis: &mut QGesture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QGesture10setHotSpotERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QGesture10setHotSpotERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGesture::~QGesture();
impl /*struct*/ QGesture {
  pub fn FreeQGesture<RetType, T: QGesture_FreeQGesture<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQGesture(self);
    // return 1;
  }
}

pub trait QGesture_FreeQGesture<RetType> {
  fn FreeQGesture(self , rsthis: &mut QGesture) -> RetType;
}

  // proto:  void QGesture::~QGesture();
impl<'a> /*trait*/ QGesture_FreeQGesture<()> for () {
  fn FreeQGesture(self , rsthis: &mut QGesture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QGestureD0Ev()};
     unsafe {_ZN8QGestureD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

