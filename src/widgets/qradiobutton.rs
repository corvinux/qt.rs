// auto generated, do not modify.
// created: Thu Jan 28 22:38:45 2016
// src-file: /QtWidgets/qradiobutton.h
// dst-file: /src/widgets/qradiobutton.rs
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
use super::qabstractbutton::*; // 773
use std::ops::Deref;
use super::super::core::qobjectdefs::*; // 771
use super::qwidget::*; // 773
use super::super::core::qsize::*; // 771
use super::super::core::qstring::*; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QRadioButton_Class_Size() -> c_int;
  // proto:  const QMetaObject * QRadioButton::metaObject();
  fn C_ZNK12QRadioButton10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QRadioButton::QRadioButton(QWidget * parent);
  fn C_ZN12QRadioButtonC2EP7QWidget(arg0: *mut c_void) -> u64;
  // proto:  QSize QRadioButton::sizeHint();
  fn C_ZNK12QRadioButton8sizeHintEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QSize QRadioButton::minimumSizeHint();
  fn C_ZNK12QRadioButton15minimumSizeHintEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QRadioButton::~QRadioButton();
  fn C_ZN12QRadioButtonD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QRadioButton::QRadioButton(const QString & text, QWidget * parent);
  fn C_ZN12QRadioButtonC2ERK7QStringP7QWidget(arg0: *mut c_void, arg1: *mut c_void) -> u64;
} // <= ext block end

// body block begin =>
// class sizeof(QRadioButton)=1
#[derive(Default)]
pub struct QRadioButton {
  qbase: QAbstractButton,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QRadioButton {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QRadioButton {
    return QRadioButton{qbase: QAbstractButton::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QRadioButton {
  type Target = QAbstractButton;

  fn deref(&self) -> &QAbstractButton {
    return & self.qbase;
  }
}
impl AsRef<QAbstractButton> for QRadioButton {
  fn as_ref(& self) -> & QAbstractButton {
    return & self.qbase;
  }
}
  // proto:  const QMetaObject * QRadioButton::metaObject();
impl /*struct*/ QRadioButton {
  pub fn metaObject<RetType, T: QRadioButton_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QRadioButton_metaObject<RetType> {
  fn metaObject(self , rsthis: & QRadioButton) -> RetType;
}

  // proto:  const QMetaObject * QRadioButton::metaObject();
impl<'a> /*trait*/ QRadioButton_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QRadioButton) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QRadioButton10metaObjectEv()};
    let mut ret = unsafe {C_ZNK12QRadioButton10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QRadioButton::QRadioButton(QWidget * parent);
impl /*struct*/ QRadioButton {
  pub fn new<T: QRadioButton_new>(value: T) -> QRadioButton {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QRadioButton_new {
  fn new(self) -> QRadioButton;
}

  // proto:  void QRadioButton::QRadioButton(QWidget * parent);
impl<'a> /*trait*/ QRadioButton_new for (&'a QWidget) {
  fn new(self) -> QRadioButton {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QRadioButtonC2EP7QWidget()};
    let ctysz: c_int = unsafe{QRadioButton_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN12QRadioButtonC2EP7QWidget(arg0)};
    let rsthis = QRadioButton{qbase: QAbstractButton::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QSize QRadioButton::sizeHint();
impl /*struct*/ QRadioButton {
  pub fn sizeHint<RetType, T: QRadioButton_sizeHint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sizeHint(self);
    // return 1;
  }
}

pub trait QRadioButton_sizeHint<RetType> {
  fn sizeHint(self , rsthis: & QRadioButton) -> RetType;
}

  // proto:  QSize QRadioButton::sizeHint();
impl<'a> /*trait*/ QRadioButton_sizeHint<QSize> for () {
  fn sizeHint(self , rsthis: & QRadioButton) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QRadioButton8sizeHintEv()};
    let mut ret = unsafe {C_ZNK12QRadioButton8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QSize QRadioButton::minimumSizeHint();
impl /*struct*/ QRadioButton {
  pub fn minimumSizeHint<RetType, T: QRadioButton_minimumSizeHint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.minimumSizeHint(self);
    // return 1;
  }
}

pub trait QRadioButton_minimumSizeHint<RetType> {
  fn minimumSizeHint(self , rsthis: & QRadioButton) -> RetType;
}

  // proto:  QSize QRadioButton::minimumSizeHint();
impl<'a> /*trait*/ QRadioButton_minimumSizeHint<QSize> for () {
  fn minimumSizeHint(self , rsthis: & QRadioButton) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QRadioButton15minimumSizeHintEv()};
    let mut ret = unsafe {C_ZNK12QRadioButton15minimumSizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QRadioButton::~QRadioButton();
impl /*struct*/ QRadioButton {
  pub fn free<RetType, T: QRadioButton_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QRadioButton_free<RetType> {
  fn free(self , rsthis: & QRadioButton) -> RetType;
}

  // proto:  void QRadioButton::~QRadioButton();
impl<'a> /*trait*/ QRadioButton_free<()> for () {
  fn free(self , rsthis: & QRadioButton) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QRadioButtonD2Ev()};
     unsafe {C_ZN12QRadioButtonD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QRadioButton::QRadioButton(const QString & text, QWidget * parent);
impl<'a> /*trait*/ QRadioButton_new for (&'a QString, &'a QWidget) {
  fn new(self) -> QRadioButton {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QRadioButtonC2ERK7QStringP7QWidget()};
    let ctysz: c_int = unsafe{QRadioButton_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN12QRadioButtonC2ERK7QStringP7QWidget(arg0, arg1)};
    let rsthis = QRadioButton{qbase: QAbstractButton::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// <= body block end

