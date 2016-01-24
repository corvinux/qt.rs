// auto generated, do not modify.
// created: Sun Jan 24 17:41:38 2016
// src-file: /QtWidgets/qabstractspinbox.h
// dst-file: /src/widgets/qabstractspinbox.rs
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
use super::qwidget::QWidget; // 773
use std::ops::Deref;
use super::super::core::qstring::QString; // 771
use super::super::core::qcoreevent::QEvent; // 771
use super::super::core::qobjectdefs::QMetaObject; // 771
use super::super::core::qsize::QSize; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QAbstractSpinBox_Class_Size() -> c_int;
  // proto:  void QAbstractSpinBox::stepBy(int steps);
  fn C_ZN16QAbstractSpinBox6stepByEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QAbstractSpinBox::setReadOnly(bool r);
  fn C_ZN16QAbstractSpinBox11setReadOnlyEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QAbstractSpinBox::setFrame(bool );
  fn C_ZN16QAbstractSpinBox8setFrameEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QAbstractSpinBox::setSpecialValueText(const QString & txt);
  fn C_ZN16QAbstractSpinBox19setSpecialValueTextERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QAbstractSpinBox::setAccelerated(bool on);
  fn C_ZN16QAbstractSpinBox14setAcceleratedEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QAbstractSpinBox::interpretText();
  fn C_ZN16QAbstractSpinBox13interpretTextEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QAbstractSpinBox::event(QEvent * event);
  fn C_ZN16QAbstractSpinBox5eventEP6QEvent(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  bool QAbstractSpinBox::keyboardTracking();
  fn C_ZNK16QAbstractSpinBox16keyboardTrackingEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  const QMetaObject * QAbstractSpinBox::metaObject();
  fn C_ZNK16QAbstractSpinBox10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QSize QAbstractSpinBox::sizeHint();
  fn C_ZNK16QAbstractSpinBox8sizeHintEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QAbstractSpinBox::~QAbstractSpinBox();
  fn C_ZN16QAbstractSpinBoxD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QAbstractSpinBox::fixup(QString & input);
  fn C_ZNK16QAbstractSpinBox5fixupER7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QAbstractSpinBox::selectAll();
  fn C_ZN16QAbstractSpinBox9selectAllEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QAbstractSpinBox::stepDown();
  fn C_ZN16QAbstractSpinBox8stepDownEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QAbstractSpinBox::clear();
  fn C_ZN16QAbstractSpinBox5clearEv(qthis: u64 /* *mut c_void*/);
  // proto:  QString QAbstractSpinBox::text();
  fn C_ZNK16QAbstractSpinBox4textEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QString QAbstractSpinBox::specialValueText();
  fn C_ZNK16QAbstractSpinBox16specialValueTextEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QSize QAbstractSpinBox::minimumSizeHint();
  fn C_ZNK16QAbstractSpinBox15minimumSizeHintEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QAbstractSpinBox::wrapping();
  fn C_ZNK16QAbstractSpinBox8wrappingEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QAbstractSpinBox::QAbstractSpinBox(QWidget * parent);
  fn C_ZN16QAbstractSpinBoxC2EP7QWidget(arg0: *mut c_void) -> u64;
  // proto:  void QAbstractSpinBox::stepUp();
  fn C_ZN16QAbstractSpinBox6stepUpEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QAbstractSpinBox::setWrapping(bool w);
  fn C_ZN16QAbstractSpinBox11setWrappingEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QAbstractSpinBox::setKeyboardTracking(bool kt);
  fn C_ZN16QAbstractSpinBox19setKeyboardTrackingEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  bool QAbstractSpinBox::isAccelerated();
  fn C_ZNK16QAbstractSpinBox13isAcceleratedEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QAbstractSpinBox::setGroupSeparatorShown(bool shown);
  fn C_ZN16QAbstractSpinBox22setGroupSeparatorShownEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  bool QAbstractSpinBox::isReadOnly();
  fn C_ZNK16QAbstractSpinBox10isReadOnlyEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QAbstractSpinBox::hasAcceptableInput();
  fn C_ZNK16QAbstractSpinBox18hasAcceptableInputEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QAbstractSpinBox::isGroupSeparatorShown();
  fn C_ZNK16QAbstractSpinBox21isGroupSeparatorShownEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QAbstractSpinBox::hasFrame();
  fn C_ZNK16QAbstractSpinBox8hasFrameEv(qthis: u64 /* *mut c_void*/) -> c_char;
  fn QAbstractSpinBox_SlotProxy_connect__ZN16QAbstractSpinBox15editingFinishedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QAbstractSpinBox)=1
#[derive(Default)]
pub struct QAbstractSpinBox {
  qbase: QWidget,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _editingFinished: QAbstractSpinBox_editingFinished_signal,
}

impl /*struct*/ QAbstractSpinBox {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QAbstractSpinBox {
    return QAbstractSpinBox{qbase: QWidget::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QAbstractSpinBox {
  type Target = QWidget;

  fn deref(&self) -> &QWidget {
    return & self.qbase;
  }
}
impl AsRef<QWidget> for QAbstractSpinBox {
  fn as_ref(& self) -> & QWidget {
    return & self.qbase;
  }
}
  // proto:  void QAbstractSpinBox::stepBy(int steps);
impl /*struct*/ QAbstractSpinBox {
  pub fn stepBy<RetType, T: QAbstractSpinBox_stepBy<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.stepBy(self);
    // return 1;
  }
}

pub trait QAbstractSpinBox_stepBy<RetType> {
  fn stepBy(self , rsthis: & QAbstractSpinBox) -> RetType;
}

  // proto:  void QAbstractSpinBox::stepBy(int steps);
impl<'a> /*trait*/ QAbstractSpinBox_stepBy<()> for (i32) {
  fn stepBy(self , rsthis: & QAbstractSpinBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QAbstractSpinBox6stepByEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN16QAbstractSpinBox6stepByEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QAbstractSpinBox::setReadOnly(bool r);
impl /*struct*/ QAbstractSpinBox {
  pub fn setReadOnly<RetType, T: QAbstractSpinBox_setReadOnly<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setReadOnly(self);
    // return 1;
  }
}

pub trait QAbstractSpinBox_setReadOnly<RetType> {
  fn setReadOnly(self , rsthis: & QAbstractSpinBox) -> RetType;
}

  // proto:  void QAbstractSpinBox::setReadOnly(bool r);
impl<'a> /*trait*/ QAbstractSpinBox_setReadOnly<()> for (i8) {
  fn setReadOnly(self , rsthis: & QAbstractSpinBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QAbstractSpinBox11setReadOnlyEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN16QAbstractSpinBox11setReadOnlyEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QAbstractSpinBox::setFrame(bool );
impl /*struct*/ QAbstractSpinBox {
  pub fn setFrame<RetType, T: QAbstractSpinBox_setFrame<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFrame(self);
    // return 1;
  }
}

pub trait QAbstractSpinBox_setFrame<RetType> {
  fn setFrame(self , rsthis: & QAbstractSpinBox) -> RetType;
}

  // proto:  void QAbstractSpinBox::setFrame(bool );
impl<'a> /*trait*/ QAbstractSpinBox_setFrame<()> for (i8) {
  fn setFrame(self , rsthis: & QAbstractSpinBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QAbstractSpinBox8setFrameEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN16QAbstractSpinBox8setFrameEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QAbstractSpinBox::setSpecialValueText(const QString & txt);
impl /*struct*/ QAbstractSpinBox {
  pub fn setSpecialValueText<RetType, T: QAbstractSpinBox_setSpecialValueText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSpecialValueText(self);
    // return 1;
  }
}

pub trait QAbstractSpinBox_setSpecialValueText<RetType> {
  fn setSpecialValueText(self , rsthis: & QAbstractSpinBox) -> RetType;
}

  // proto:  void QAbstractSpinBox::setSpecialValueText(const QString & txt);
impl<'a> /*trait*/ QAbstractSpinBox_setSpecialValueText<()> for (&'a QString) {
  fn setSpecialValueText(self , rsthis: & QAbstractSpinBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QAbstractSpinBox19setSpecialValueTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN16QAbstractSpinBox19setSpecialValueTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QAbstractSpinBox::setAccelerated(bool on);
impl /*struct*/ QAbstractSpinBox {
  pub fn setAccelerated<RetType, T: QAbstractSpinBox_setAccelerated<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setAccelerated(self);
    // return 1;
  }
}

pub trait QAbstractSpinBox_setAccelerated<RetType> {
  fn setAccelerated(self , rsthis: & QAbstractSpinBox) -> RetType;
}

  // proto:  void QAbstractSpinBox::setAccelerated(bool on);
impl<'a> /*trait*/ QAbstractSpinBox_setAccelerated<()> for (i8) {
  fn setAccelerated(self , rsthis: & QAbstractSpinBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QAbstractSpinBox14setAcceleratedEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN16QAbstractSpinBox14setAcceleratedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QAbstractSpinBox::interpretText();
impl /*struct*/ QAbstractSpinBox {
  pub fn interpretText<RetType, T: QAbstractSpinBox_interpretText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.interpretText(self);
    // return 1;
  }
}

pub trait QAbstractSpinBox_interpretText<RetType> {
  fn interpretText(self , rsthis: & QAbstractSpinBox) -> RetType;
}

  // proto:  void QAbstractSpinBox::interpretText();
impl<'a> /*trait*/ QAbstractSpinBox_interpretText<()> for () {
  fn interpretText(self , rsthis: & QAbstractSpinBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QAbstractSpinBox13interpretTextEv()};
     unsafe {C_ZN16QAbstractSpinBox13interpretTextEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QAbstractSpinBox::event(QEvent * event);
impl /*struct*/ QAbstractSpinBox {
  pub fn event<RetType, T: QAbstractSpinBox_event<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.event(self);
    // return 1;
  }
}

pub trait QAbstractSpinBox_event<RetType> {
  fn event(self , rsthis: & QAbstractSpinBox) -> RetType;
}

  // proto:  bool QAbstractSpinBox::event(QEvent * event);
impl<'a> /*trait*/ QAbstractSpinBox_event<i8> for (&'a QEvent) {
  fn event(self , rsthis: & QAbstractSpinBox) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QAbstractSpinBox5eventEP6QEvent()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN16QAbstractSpinBox5eventEP6QEvent(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QAbstractSpinBox::keyboardTracking();
impl /*struct*/ QAbstractSpinBox {
  pub fn keyboardTracking<RetType, T: QAbstractSpinBox_keyboardTracking<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.keyboardTracking(self);
    // return 1;
  }
}

pub trait QAbstractSpinBox_keyboardTracking<RetType> {
  fn keyboardTracking(self , rsthis: & QAbstractSpinBox) -> RetType;
}

  // proto:  bool QAbstractSpinBox::keyboardTracking();
impl<'a> /*trait*/ QAbstractSpinBox_keyboardTracking<i8> for () {
  fn keyboardTracking(self , rsthis: & QAbstractSpinBox) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QAbstractSpinBox16keyboardTrackingEv()};
    let mut ret = unsafe {C_ZNK16QAbstractSpinBox16keyboardTrackingEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  const QMetaObject * QAbstractSpinBox::metaObject();
impl /*struct*/ QAbstractSpinBox {
  pub fn metaObject<RetType, T: QAbstractSpinBox_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QAbstractSpinBox_metaObject<RetType> {
  fn metaObject(self , rsthis: & QAbstractSpinBox) -> RetType;
}

  // proto:  const QMetaObject * QAbstractSpinBox::metaObject();
impl<'a> /*trait*/ QAbstractSpinBox_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QAbstractSpinBox) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QAbstractSpinBox10metaObjectEv()};
    let mut ret = unsafe {C_ZNK16QAbstractSpinBox10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QSize QAbstractSpinBox::sizeHint();
impl /*struct*/ QAbstractSpinBox {
  pub fn sizeHint<RetType, T: QAbstractSpinBox_sizeHint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sizeHint(self);
    // return 1;
  }
}

pub trait QAbstractSpinBox_sizeHint<RetType> {
  fn sizeHint(self , rsthis: & QAbstractSpinBox) -> RetType;
}

  // proto:  QSize QAbstractSpinBox::sizeHint();
impl<'a> /*trait*/ QAbstractSpinBox_sizeHint<QSize> for () {
  fn sizeHint(self , rsthis: & QAbstractSpinBox) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QAbstractSpinBox8sizeHintEv()};
    let mut ret = unsafe {C_ZNK16QAbstractSpinBox8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QAbstractSpinBox::~QAbstractSpinBox();
impl /*struct*/ QAbstractSpinBox {
  pub fn free<RetType, T: QAbstractSpinBox_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QAbstractSpinBox_free<RetType> {
  fn free(self , rsthis: & QAbstractSpinBox) -> RetType;
}

  // proto:  void QAbstractSpinBox::~QAbstractSpinBox();
impl<'a> /*trait*/ QAbstractSpinBox_free<()> for () {
  fn free(self , rsthis: & QAbstractSpinBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QAbstractSpinBoxD2Ev()};
     unsafe {C_ZN16QAbstractSpinBoxD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QAbstractSpinBox::fixup(QString & input);
impl /*struct*/ QAbstractSpinBox {
  pub fn fixup<RetType, T: QAbstractSpinBox_fixup<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.fixup(self);
    // return 1;
  }
}

pub trait QAbstractSpinBox_fixup<RetType> {
  fn fixup(self , rsthis: & QAbstractSpinBox) -> RetType;
}

  // proto:  void QAbstractSpinBox::fixup(QString & input);
impl<'a> /*trait*/ QAbstractSpinBox_fixup<()> for (&'a QString) {
  fn fixup(self , rsthis: & QAbstractSpinBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QAbstractSpinBox5fixupER7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZNK16QAbstractSpinBox5fixupER7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QAbstractSpinBox::selectAll();
impl /*struct*/ QAbstractSpinBox {
  pub fn selectAll<RetType, T: QAbstractSpinBox_selectAll<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.selectAll(self);
    // return 1;
  }
}

pub trait QAbstractSpinBox_selectAll<RetType> {
  fn selectAll(self , rsthis: & QAbstractSpinBox) -> RetType;
}

  // proto:  void QAbstractSpinBox::selectAll();
impl<'a> /*trait*/ QAbstractSpinBox_selectAll<()> for () {
  fn selectAll(self , rsthis: & QAbstractSpinBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QAbstractSpinBox9selectAllEv()};
     unsafe {C_ZN16QAbstractSpinBox9selectAllEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QAbstractSpinBox::stepDown();
impl /*struct*/ QAbstractSpinBox {
  pub fn stepDown<RetType, T: QAbstractSpinBox_stepDown<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.stepDown(self);
    // return 1;
  }
}

pub trait QAbstractSpinBox_stepDown<RetType> {
  fn stepDown(self , rsthis: & QAbstractSpinBox) -> RetType;
}

  // proto:  void QAbstractSpinBox::stepDown();
impl<'a> /*trait*/ QAbstractSpinBox_stepDown<()> for () {
  fn stepDown(self , rsthis: & QAbstractSpinBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QAbstractSpinBox8stepDownEv()};
     unsafe {C_ZN16QAbstractSpinBox8stepDownEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QAbstractSpinBox::clear();
impl /*struct*/ QAbstractSpinBox {
  pub fn clear<RetType, T: QAbstractSpinBox_clear<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clear(self);
    // return 1;
  }
}

pub trait QAbstractSpinBox_clear<RetType> {
  fn clear(self , rsthis: & QAbstractSpinBox) -> RetType;
}

  // proto:  void QAbstractSpinBox::clear();
impl<'a> /*trait*/ QAbstractSpinBox_clear<()> for () {
  fn clear(self , rsthis: & QAbstractSpinBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QAbstractSpinBox5clearEv()};
     unsafe {C_ZN16QAbstractSpinBox5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QString QAbstractSpinBox::text();
impl /*struct*/ QAbstractSpinBox {
  pub fn text<RetType, T: QAbstractSpinBox_text<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.text(self);
    // return 1;
  }
}

pub trait QAbstractSpinBox_text<RetType> {
  fn text(self , rsthis: & QAbstractSpinBox) -> RetType;
}

  // proto:  QString QAbstractSpinBox::text();
impl<'a> /*trait*/ QAbstractSpinBox_text<QString> for () {
  fn text(self , rsthis: & QAbstractSpinBox) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QAbstractSpinBox4textEv()};
    let mut ret = unsafe {C_ZNK16QAbstractSpinBox4textEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QAbstractSpinBox::specialValueText();
impl /*struct*/ QAbstractSpinBox {
  pub fn specialValueText<RetType, T: QAbstractSpinBox_specialValueText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.specialValueText(self);
    // return 1;
  }
}

pub trait QAbstractSpinBox_specialValueText<RetType> {
  fn specialValueText(self , rsthis: & QAbstractSpinBox) -> RetType;
}

  // proto:  QString QAbstractSpinBox::specialValueText();
impl<'a> /*trait*/ QAbstractSpinBox_specialValueText<QString> for () {
  fn specialValueText(self , rsthis: & QAbstractSpinBox) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QAbstractSpinBox16specialValueTextEv()};
    let mut ret = unsafe {C_ZNK16QAbstractSpinBox16specialValueTextEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QSize QAbstractSpinBox::minimumSizeHint();
impl /*struct*/ QAbstractSpinBox {
  pub fn minimumSizeHint<RetType, T: QAbstractSpinBox_minimumSizeHint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.minimumSizeHint(self);
    // return 1;
  }
}

pub trait QAbstractSpinBox_minimumSizeHint<RetType> {
  fn minimumSizeHint(self , rsthis: & QAbstractSpinBox) -> RetType;
}

  // proto:  QSize QAbstractSpinBox::minimumSizeHint();
impl<'a> /*trait*/ QAbstractSpinBox_minimumSizeHint<QSize> for () {
  fn minimumSizeHint(self , rsthis: & QAbstractSpinBox) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QAbstractSpinBox15minimumSizeHintEv()};
    let mut ret = unsafe {C_ZNK16QAbstractSpinBox15minimumSizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QAbstractSpinBox::wrapping();
impl /*struct*/ QAbstractSpinBox {
  pub fn wrapping<RetType, T: QAbstractSpinBox_wrapping<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.wrapping(self);
    // return 1;
  }
}

pub trait QAbstractSpinBox_wrapping<RetType> {
  fn wrapping(self , rsthis: & QAbstractSpinBox) -> RetType;
}

  // proto:  bool QAbstractSpinBox::wrapping();
impl<'a> /*trait*/ QAbstractSpinBox_wrapping<i8> for () {
  fn wrapping(self , rsthis: & QAbstractSpinBox) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QAbstractSpinBox8wrappingEv()};
    let mut ret = unsafe {C_ZNK16QAbstractSpinBox8wrappingEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QAbstractSpinBox::QAbstractSpinBox(QWidget * parent);
impl /*struct*/ QAbstractSpinBox {
  pub fn new<T: QAbstractSpinBox_new>(value: T) -> QAbstractSpinBox {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QAbstractSpinBox_new {
  fn new(self) -> QAbstractSpinBox;
}

  // proto:  void QAbstractSpinBox::QAbstractSpinBox(QWidget * parent);
impl<'a> /*trait*/ QAbstractSpinBox_new for (&'a QWidget) {
  fn new(self) -> QAbstractSpinBox {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QAbstractSpinBoxC2EP7QWidget()};
    let ctysz: c_int = unsafe{QAbstractSpinBox_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN16QAbstractSpinBoxC2EP7QWidget(arg0)};
    let rsthis = QAbstractSpinBox{qbase: QWidget::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QAbstractSpinBox::stepUp();
impl /*struct*/ QAbstractSpinBox {
  pub fn stepUp<RetType, T: QAbstractSpinBox_stepUp<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.stepUp(self);
    // return 1;
  }
}

pub trait QAbstractSpinBox_stepUp<RetType> {
  fn stepUp(self , rsthis: & QAbstractSpinBox) -> RetType;
}

  // proto:  void QAbstractSpinBox::stepUp();
impl<'a> /*trait*/ QAbstractSpinBox_stepUp<()> for () {
  fn stepUp(self , rsthis: & QAbstractSpinBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QAbstractSpinBox6stepUpEv()};
     unsafe {C_ZN16QAbstractSpinBox6stepUpEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QAbstractSpinBox::setWrapping(bool w);
impl /*struct*/ QAbstractSpinBox {
  pub fn setWrapping<RetType, T: QAbstractSpinBox_setWrapping<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setWrapping(self);
    // return 1;
  }
}

pub trait QAbstractSpinBox_setWrapping<RetType> {
  fn setWrapping(self , rsthis: & QAbstractSpinBox) -> RetType;
}

  // proto:  void QAbstractSpinBox::setWrapping(bool w);
impl<'a> /*trait*/ QAbstractSpinBox_setWrapping<()> for (i8) {
  fn setWrapping(self , rsthis: & QAbstractSpinBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QAbstractSpinBox11setWrappingEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN16QAbstractSpinBox11setWrappingEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QAbstractSpinBox::setKeyboardTracking(bool kt);
impl /*struct*/ QAbstractSpinBox {
  pub fn setKeyboardTracking<RetType, T: QAbstractSpinBox_setKeyboardTracking<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setKeyboardTracking(self);
    // return 1;
  }
}

pub trait QAbstractSpinBox_setKeyboardTracking<RetType> {
  fn setKeyboardTracking(self , rsthis: & QAbstractSpinBox) -> RetType;
}

  // proto:  void QAbstractSpinBox::setKeyboardTracking(bool kt);
impl<'a> /*trait*/ QAbstractSpinBox_setKeyboardTracking<()> for (i8) {
  fn setKeyboardTracking(self , rsthis: & QAbstractSpinBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QAbstractSpinBox19setKeyboardTrackingEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN16QAbstractSpinBox19setKeyboardTrackingEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QAbstractSpinBox::isAccelerated();
impl /*struct*/ QAbstractSpinBox {
  pub fn isAccelerated<RetType, T: QAbstractSpinBox_isAccelerated<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isAccelerated(self);
    // return 1;
  }
}

pub trait QAbstractSpinBox_isAccelerated<RetType> {
  fn isAccelerated(self , rsthis: & QAbstractSpinBox) -> RetType;
}

  // proto:  bool QAbstractSpinBox::isAccelerated();
impl<'a> /*trait*/ QAbstractSpinBox_isAccelerated<i8> for () {
  fn isAccelerated(self , rsthis: & QAbstractSpinBox) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QAbstractSpinBox13isAcceleratedEv()};
    let mut ret = unsafe {C_ZNK16QAbstractSpinBox13isAcceleratedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QAbstractSpinBox::setGroupSeparatorShown(bool shown);
impl /*struct*/ QAbstractSpinBox {
  pub fn setGroupSeparatorShown<RetType, T: QAbstractSpinBox_setGroupSeparatorShown<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setGroupSeparatorShown(self);
    // return 1;
  }
}

pub trait QAbstractSpinBox_setGroupSeparatorShown<RetType> {
  fn setGroupSeparatorShown(self , rsthis: & QAbstractSpinBox) -> RetType;
}

  // proto:  void QAbstractSpinBox::setGroupSeparatorShown(bool shown);
impl<'a> /*trait*/ QAbstractSpinBox_setGroupSeparatorShown<()> for (i8) {
  fn setGroupSeparatorShown(self , rsthis: & QAbstractSpinBox) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QAbstractSpinBox22setGroupSeparatorShownEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN16QAbstractSpinBox22setGroupSeparatorShownEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QAbstractSpinBox::isReadOnly();
impl /*struct*/ QAbstractSpinBox {
  pub fn isReadOnly<RetType, T: QAbstractSpinBox_isReadOnly<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isReadOnly(self);
    // return 1;
  }
}

pub trait QAbstractSpinBox_isReadOnly<RetType> {
  fn isReadOnly(self , rsthis: & QAbstractSpinBox) -> RetType;
}

  // proto:  bool QAbstractSpinBox::isReadOnly();
impl<'a> /*trait*/ QAbstractSpinBox_isReadOnly<i8> for () {
  fn isReadOnly(self , rsthis: & QAbstractSpinBox) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QAbstractSpinBox10isReadOnlyEv()};
    let mut ret = unsafe {C_ZNK16QAbstractSpinBox10isReadOnlyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QAbstractSpinBox::hasAcceptableInput();
impl /*struct*/ QAbstractSpinBox {
  pub fn hasAcceptableInput<RetType, T: QAbstractSpinBox_hasAcceptableInput<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasAcceptableInput(self);
    // return 1;
  }
}

pub trait QAbstractSpinBox_hasAcceptableInput<RetType> {
  fn hasAcceptableInput(self , rsthis: & QAbstractSpinBox) -> RetType;
}

  // proto:  bool QAbstractSpinBox::hasAcceptableInput();
impl<'a> /*trait*/ QAbstractSpinBox_hasAcceptableInput<i8> for () {
  fn hasAcceptableInput(self , rsthis: & QAbstractSpinBox) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QAbstractSpinBox18hasAcceptableInputEv()};
    let mut ret = unsafe {C_ZNK16QAbstractSpinBox18hasAcceptableInputEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QAbstractSpinBox::isGroupSeparatorShown();
impl /*struct*/ QAbstractSpinBox {
  pub fn isGroupSeparatorShown<RetType, T: QAbstractSpinBox_isGroupSeparatorShown<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isGroupSeparatorShown(self);
    // return 1;
  }
}

pub trait QAbstractSpinBox_isGroupSeparatorShown<RetType> {
  fn isGroupSeparatorShown(self , rsthis: & QAbstractSpinBox) -> RetType;
}

  // proto:  bool QAbstractSpinBox::isGroupSeparatorShown();
impl<'a> /*trait*/ QAbstractSpinBox_isGroupSeparatorShown<i8> for () {
  fn isGroupSeparatorShown(self , rsthis: & QAbstractSpinBox) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QAbstractSpinBox21isGroupSeparatorShownEv()};
    let mut ret = unsafe {C_ZNK16QAbstractSpinBox21isGroupSeparatorShownEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QAbstractSpinBox::hasFrame();
impl /*struct*/ QAbstractSpinBox {
  pub fn hasFrame<RetType, T: QAbstractSpinBox_hasFrame<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasFrame(self);
    // return 1;
  }
}

pub trait QAbstractSpinBox_hasFrame<RetType> {
  fn hasFrame(self , rsthis: & QAbstractSpinBox) -> RetType;
}

  // proto:  bool QAbstractSpinBox::hasFrame();
impl<'a> /*trait*/ QAbstractSpinBox_hasFrame<i8> for () {
  fn hasFrame(self , rsthis: & QAbstractSpinBox) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QAbstractSpinBox8hasFrameEv()};
    let mut ret = unsafe {C_ZNK16QAbstractSpinBox8hasFrameEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

#[derive(Default)] // for QAbstractSpinBox_editingFinished
pub struct QAbstractSpinBox_editingFinished_signal{poi:u64}
impl /* struct */ QAbstractSpinBox {
  pub fn editingFinished(&self) -> QAbstractSpinBox_editingFinished_signal {
     return QAbstractSpinBox_editingFinished_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QAbstractSpinBox_editingFinished_signal {
  pub fn connect<T: QAbstractSpinBox_editingFinished_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QAbstractSpinBox_editingFinished_signal_connect {
  fn connect(self, sigthis: QAbstractSpinBox_editingFinished_signal);
}

// editingFinished()
extern fn QAbstractSpinBox_editingFinished_signal_connect_cb_0(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QAbstractSpinBox_editingFinished_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QAbstractSpinBox_editingFinished_signal_connect for fn() {
  fn connect(self, sigthis: QAbstractSpinBox_editingFinished_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QAbstractSpinBox_editingFinished_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QAbstractSpinBox_SlotProxy_connect__ZN16QAbstractSpinBox15editingFinishedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QAbstractSpinBox_editingFinished_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QAbstractSpinBox_editingFinished_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QAbstractSpinBox_editingFinished_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QAbstractSpinBox_SlotProxy_connect__ZN16QAbstractSpinBox15editingFinishedEv(arg0, arg1, arg2)};
  }
}
// <= body block end

