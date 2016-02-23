// auto generated, do not modify.
// created: Mon Feb 22 23:57:02 2016
// src-file: /QtWidgets/qscrollbar.h
// dst-file: /src/widgets/qscrollbar.rs
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
use super::super::core::qcoreevent::*; // 771
use super::super::core::qobjectdefs::*; // 771
use super::super::core::qsize::*; // 771
use super::qwidget::*; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QScrollBar_Class_Size() -> c_int;
  // proto:  bool QScrollBar::event(QEvent * event);
  fn C_ZN10QScrollBar5eventEP6QEvent(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  const QMetaObject * QScrollBar::metaObject();
  fn C_ZNK10QScrollBar10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QSize QScrollBar::sizeHint();
  fn C_ZNK10QScrollBar8sizeHintEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QScrollBar::QScrollBar(QWidget * parent);
  fn C_ZN10QScrollBarC2EP7QWidget(arg0: *mut c_void) -> u64;
  // proto:  void QScrollBar::~QScrollBar();
  fn C_ZN10QScrollBarD2Ev(qthis: u64 /* *mut c_void*/);
} // <= ext block end

// body block begin =>
// class sizeof(QScrollBar)=1
#[derive(Default)]
pub struct QScrollBar {
  qbase: QAbstractSlider,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QScrollBar {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QScrollBar {
    return QScrollBar{qbase: QAbstractSlider::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QScrollBar {
  type Target = QAbstractSlider;

  fn deref(&self) -> &QAbstractSlider {
    return & self.qbase;
  }
}
impl AsRef<QAbstractSlider> for QScrollBar {
  fn as_ref(& self) -> & QAbstractSlider {
    return & self.qbase;
  }
}
  // proto:  bool QScrollBar::event(QEvent * event);
impl /*struct*/ QScrollBar {
  pub fn event<RetType, T: QScrollBar_event<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.event(self);
    // return 1;
  }
}

pub trait QScrollBar_event<RetType> {
  fn event(self , rsthis: & QScrollBar) -> RetType;
}

  // proto:  bool QScrollBar::event(QEvent * event);
impl<'a> /*trait*/ QScrollBar_event<i8> for (&'a QEvent) {
  fn event(self , rsthis: & QScrollBar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QScrollBar5eventEP6QEvent()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN10QScrollBar5eventEP6QEvent(rsthis.qclsinst, arg0)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  const QMetaObject * QScrollBar::metaObject();
impl /*struct*/ QScrollBar {
  pub fn metaObject<RetType, T: QScrollBar_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QScrollBar_metaObject<RetType> {
  fn metaObject(self , rsthis: & QScrollBar) -> RetType;
}

  // proto:  const QMetaObject * QScrollBar::metaObject();
impl<'a> /*trait*/ QScrollBar_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QScrollBar) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QScrollBar10metaObjectEv()};
    let mut ret = unsafe {C_ZNK10QScrollBar10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QSize QScrollBar::sizeHint();
impl /*struct*/ QScrollBar {
  pub fn sizeHint<RetType, T: QScrollBar_sizeHint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sizeHint(self);
    // return 1;
  }
}

pub trait QScrollBar_sizeHint<RetType> {
  fn sizeHint(self , rsthis: & QScrollBar) -> RetType;
}

  // proto:  QSize QScrollBar::sizeHint();
impl<'a> /*trait*/ QScrollBar_sizeHint<QSize> for () {
  fn sizeHint(self , rsthis: & QScrollBar) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QScrollBar8sizeHintEv()};
    let mut ret = unsafe {C_ZNK10QScrollBar8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QScrollBar::QScrollBar(QWidget * parent);
impl /*struct*/ QScrollBar {
  pub fn new<T: QScrollBar_new>(value: T) -> QScrollBar {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QScrollBar_new {
  fn new(self) -> QScrollBar;
}

  // proto:  void QScrollBar::QScrollBar(QWidget * parent);
impl<'a> /*trait*/ QScrollBar_new for (Option<&'a QWidget>) {
  fn new(self) -> QScrollBar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QScrollBarC2EP7QWidget()};
    let ctysz: c_int = unsafe{QScrollBar_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = (if self.is_none() {0} else {self.unwrap().qclsinst})  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN10QScrollBarC2EP7QWidget(arg0)};
    let rsthis = QScrollBar{qbase: QAbstractSlider::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QScrollBar::~QScrollBar();
impl /*struct*/ QScrollBar {
  pub fn free<RetType, T: QScrollBar_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QScrollBar_free<RetType> {
  fn free(self , rsthis: & QScrollBar) -> RetType;
}

  // proto:  void QScrollBar::~QScrollBar();
impl<'a> /*trait*/ QScrollBar_free<()> for () {
  fn free(self , rsthis: & QScrollBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QScrollBarD2Ev()};
     unsafe {C_ZN10QScrollBarD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

