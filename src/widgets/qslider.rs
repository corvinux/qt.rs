// auto generated, do not modify.
// created: Mon Feb 22 23:57:02 2016
// src-file: /QtWidgets/qslider.h
// dst-file: /src/widgets/qslider.rs
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
use super::qabstractslider::*; // 773
use std::ops::Deref;
use super::super::core::qobjectdefs::*; // 771
use super::super::core::qcoreevent::*; // 771
use super::super::core::qsize::*; // 771
use super::qwidget::*; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QSlider_Class_Size() -> c_int;
  // proto:  const QMetaObject * QSlider::metaObject();
  fn C_ZNK7QSlider10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QSlider::event(QEvent * event);
  fn C_ZN7QSlider5eventEP6QEvent(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  int QSlider::tickInterval();
  fn C_ZNK7QSlider12tickIntervalEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  QSize QSlider::sizeHint();
  fn C_ZNK7QSlider8sizeHintEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QSlider::setTickInterval(int ti);
  fn C_ZN7QSlider15setTickIntervalEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QSlider::~QSlider();
  fn C_ZN7QSliderD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QSlider::QSlider(QWidget * parent);
  fn C_ZN7QSliderC2EP7QWidget(arg0: *mut c_void) -> u64;
  // proto:  QSize QSlider::minimumSizeHint();
  fn C_ZNK7QSlider15minimumSizeHintEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
} // <= ext block end

// body block begin =>
// class sizeof(QSlider)=1
#[derive(Default)]
pub struct QSlider {
  qbase: QAbstractSlider,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QSlider {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QSlider {
    return QSlider{qbase: QAbstractSlider::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QSlider {
  type Target = QAbstractSlider;

  fn deref(&self) -> &QAbstractSlider {
    return & self.qbase;
  }
}
impl AsRef<QAbstractSlider> for QSlider {
  fn as_ref(& self) -> & QAbstractSlider {
    return & self.qbase;
  }
}
  // proto:  const QMetaObject * QSlider::metaObject();
impl /*struct*/ QSlider {
  pub fn metaObject<RetType, T: QSlider_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QSlider_metaObject<RetType> {
  fn metaObject(self , rsthis: & QSlider) -> RetType;
}

  // proto:  const QMetaObject * QSlider::metaObject();
impl<'a> /*trait*/ QSlider_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QSlider) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QSlider10metaObjectEv()};
    let mut ret = unsafe {C_ZNK7QSlider10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QSlider::event(QEvent * event);
impl /*struct*/ QSlider {
  pub fn event<RetType, T: QSlider_event<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.event(self);
    // return 1;
  }
}

pub trait QSlider_event<RetType> {
  fn event(self , rsthis: & QSlider) -> RetType;
}

  // proto:  bool QSlider::event(QEvent * event);
impl<'a> /*trait*/ QSlider_event<i8> for (&'a QEvent) {
  fn event(self , rsthis: & QSlider) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QSlider5eventEP6QEvent()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN7QSlider5eventEP6QEvent(rsthis.qclsinst, arg0)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  int QSlider::tickInterval();
impl /*struct*/ QSlider {
  pub fn tickInterval<RetType, T: QSlider_tickInterval<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.tickInterval(self);
    // return 1;
  }
}

pub trait QSlider_tickInterval<RetType> {
  fn tickInterval(self , rsthis: & QSlider) -> RetType;
}

  // proto:  int QSlider::tickInterval();
impl<'a> /*trait*/ QSlider_tickInterval<i32> for () {
  fn tickInterval(self , rsthis: & QSlider) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QSlider12tickIntervalEv()};
    let mut ret = unsafe {C_ZNK7QSlider12tickIntervalEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  QSize QSlider::sizeHint();
impl /*struct*/ QSlider {
  pub fn sizeHint<RetType, T: QSlider_sizeHint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sizeHint(self);
    // return 1;
  }
}

pub trait QSlider_sizeHint<RetType> {
  fn sizeHint(self , rsthis: & QSlider) -> RetType;
}

  // proto:  QSize QSlider::sizeHint();
impl<'a> /*trait*/ QSlider_sizeHint<QSize> for () {
  fn sizeHint(self , rsthis: & QSlider) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QSlider8sizeHintEv()};
    let mut ret = unsafe {C_ZNK7QSlider8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QSlider::setTickInterval(int ti);
impl /*struct*/ QSlider {
  pub fn setTickInterval<RetType, T: QSlider_setTickInterval<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTickInterval(self);
    // return 1;
  }
}

pub trait QSlider_setTickInterval<RetType> {
  fn setTickInterval(self , rsthis: & QSlider) -> RetType;
}

  // proto:  void QSlider::setTickInterval(int ti);
impl<'a> /*trait*/ QSlider_setTickInterval<()> for (i32) {
  fn setTickInterval(self , rsthis: & QSlider) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QSlider15setTickIntervalEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN7QSlider15setTickIntervalEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QSlider::~QSlider();
impl /*struct*/ QSlider {
  pub fn free<RetType, T: QSlider_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QSlider_free<RetType> {
  fn free(self , rsthis: & QSlider) -> RetType;
}

  // proto:  void QSlider::~QSlider();
impl<'a> /*trait*/ QSlider_free<()> for () {
  fn free(self , rsthis: & QSlider) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QSliderD2Ev()};
     unsafe {C_ZN7QSliderD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QSlider::QSlider(QWidget * parent);
impl /*struct*/ QSlider {
  pub fn new<T: QSlider_new>(value: T) -> QSlider {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QSlider_new {
  fn new(self) -> QSlider;
}

  // proto:  void QSlider::QSlider(QWidget * parent);
impl<'a> /*trait*/ QSlider_new for (Option<&'a QWidget>) {
  fn new(self) -> QSlider {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QSliderC2EP7QWidget()};
    let ctysz: c_int = unsafe{QSlider_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = (if self.is_none() {0} else {self.unwrap().qclsinst})  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN7QSliderC2EP7QWidget(arg0)};
    let rsthis = QSlider{qbase: QAbstractSlider::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QSize QSlider::minimumSizeHint();
impl /*struct*/ QSlider {
  pub fn minimumSizeHint<RetType, T: QSlider_minimumSizeHint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.minimumSizeHint(self);
    // return 1;
  }
}

pub trait QSlider_minimumSizeHint<RetType> {
  fn minimumSizeHint(self , rsthis: & QSlider) -> RetType;
}

  // proto:  QSize QSlider::minimumSizeHint();
impl<'a> /*trait*/ QSlider_minimumSizeHint<QSize> for () {
  fn minimumSizeHint(self , rsthis: & QSlider) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QSlider15minimumSizeHintEv()};
    let mut ret = unsafe {C_ZNK7QSlider15minimumSizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

// <= body block end

