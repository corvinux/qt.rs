// auto generated, do not modify.
// created: Tue Dec 22 23:21:28 2015
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
use super::qabstractbutton::QAbstractButton; // 773
use std::ops::Deref;
use super::qwidget::QWidget; // 773
use super::super::core::qsize::QSize; // 771
use super::super::core::qstring::QString; // 771
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  const QMetaObject * QRadioButton::metaObject();
  fn _ZNK12QRadioButton10metaObjectEv(qthis: *mut c_void);
  // proto:  void QRadioButton::QRadioButton(QWidget * parent);
  fn _ZN12QRadioButtonC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QSize QRadioButton::sizeHint();
  fn _ZNK12QRadioButton8sizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QSize QRadioButton::minimumSizeHint();
  fn _ZNK12QRadioButton15minimumSizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QRadioButton::~QRadioButton();
  fn _ZN12QRadioButtonD0Ev(qthis: *mut c_void);
  // proto:  void QRadioButton::QRadioButton(const QRadioButton & );
  fn _ZN12QRadioButtonC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QRadioButton::QRadioButton(const QString & text, QWidget * parent);
  fn _ZN12QRadioButtonC1ERK7QStringP7QWidget(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QRadioButton)=1
pub struct QRadioButton {
  qbase: QAbstractButton,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QRadioButton {
  pub fn inheritFrom(qthis: *mut c_void) -> QRadioButton {
    return QRadioButton{qbase: QAbstractButton::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QRadioButton {
  type Target = QAbstractButton;

  fn deref(&self) -> &QAbstractButton {
    return &self.qbase;
  }
}
impl AsRef<QAbstractButton> for QRadioButton {
  fn as_ref(&self) -> &QAbstractButton {
    return &self.qbase;
  }
}
  // proto:  const QMetaObject * QRadioButton::metaObject();
impl /*struct*/ QRadioButton {
  pub fn metaObject<RetType, T: QRadioButton_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QRadioButton_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QRadioButton) -> RetType;
}

  // proto:  const QMetaObject * QRadioButton::metaObject();
impl<'a> /*trait*/ QRadioButton_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QRadioButton) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QRadioButton10metaObjectEv()};
     unsafe {_ZNK12QRadioButton10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QRadioButton::QRadioButton(QWidget * parent);
impl /*struct*/ QRadioButton {
  pub fn NewQRadioButton<T: QRadioButton_NewQRadioButton>(value: T) -> QRadioButton {
    let rsthis = value.NewQRadioButton();
    return rsthis;
    // return 1;
  }
}

pub trait QRadioButton_NewQRadioButton {
  fn NewQRadioButton(self) -> QRadioButton;
}

  // proto:  void QRadioButton::QRadioButton(QWidget * parent);
impl<'a> /*trait*/ QRadioButton_NewQRadioButton for (QWidget) {
  fn NewQRadioButton(self) -> QRadioButton {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QRadioButtonC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QRadioButtonC1EP7QWidget(qthis, arg0)};
    let rsthis = QRadioButton{/**/qbase: QAbstractButton::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QSize QRadioButton::sizeHint();
impl /*struct*/ QRadioButton {
  pub fn sizeHint<RetType, T: QRadioButton_sizeHint<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.sizeHint(self);
    // return 1;
  }
}

pub trait QRadioButton_sizeHint<RetType> {
  fn sizeHint(self , rsthis: &mut QRadioButton) -> RetType;
}

  // proto:  QSize QRadioButton::sizeHint();
impl<'a> /*trait*/ QRadioButton_sizeHint<QSize> for () {
  fn sizeHint(self , rsthis: &mut QRadioButton) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QRadioButton8sizeHintEv()};
    let mut ret = unsafe {_ZNK12QRadioButton8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QSize QRadioButton::minimumSizeHint();
impl /*struct*/ QRadioButton {
  pub fn minimumSizeHint<RetType, T: QRadioButton_minimumSizeHint<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.minimumSizeHint(self);
    // return 1;
  }
}

pub trait QRadioButton_minimumSizeHint<RetType> {
  fn minimumSizeHint(self , rsthis: &mut QRadioButton) -> RetType;
}

  // proto:  QSize QRadioButton::minimumSizeHint();
impl<'a> /*trait*/ QRadioButton_minimumSizeHint<QSize> for () {
  fn minimumSizeHint(self , rsthis: &mut QRadioButton) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QRadioButton15minimumSizeHintEv()};
    let mut ret = unsafe {_ZNK12QRadioButton15minimumSizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QRadioButton::~QRadioButton();
impl /*struct*/ QRadioButton {
  pub fn FreeQRadioButton<RetType, T: QRadioButton_FreeQRadioButton<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQRadioButton(self);
    // return 1;
  }
}

pub trait QRadioButton_FreeQRadioButton<RetType> {
  fn FreeQRadioButton(self , rsthis: &mut QRadioButton) -> RetType;
}

  // proto:  void QRadioButton::~QRadioButton();
impl<'a> /*trait*/ QRadioButton_FreeQRadioButton<()> for () {
  fn FreeQRadioButton(self , rsthis: &mut QRadioButton) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QRadioButtonD0Ev()};
     unsafe {_ZN12QRadioButtonD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QRadioButton::QRadioButton(const QRadioButton & );
impl<'a> /*trait*/ QRadioButton_NewQRadioButton for (QRadioButton) {
  fn NewQRadioButton(self) -> QRadioButton {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QRadioButtonC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QRadioButtonC1ERKS_(qthis, arg0)};
    let rsthis = QRadioButton{/**/qbase: QAbstractButton::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QRadioButton::QRadioButton(const QString & text, QWidget * parent);
impl<'a> /*trait*/ QRadioButton_NewQRadioButton for (QString, QWidget) {
  fn NewQRadioButton(self) -> QRadioButton {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QRadioButtonC1ERK7QStringP7QWidget()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN12QRadioButtonC1ERK7QStringP7QWidget(qthis, arg0, arg1)};
    let rsthis = QRadioButton{/**/qbase: QAbstractButton::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// <= body block end

