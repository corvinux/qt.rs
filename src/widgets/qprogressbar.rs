// auto generated, do not modify.
// created: Sat Dec 26 10:52:38 2015
// src-file: /QtWidgets/qprogressbar.h
// dst-file: /src/widgets/qprogressbar.rs
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
use super::super::core::qsize::QSize; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QProgressBar_Class_Size() -> c_int;
  // proto:  QString QProgressBar::format();
  fn _ZNK12QProgressBar6formatEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QProgressBar::reset();
  fn _ZN12QProgressBar5resetEv(qthis: *mut c_void);
  // proto:  const QMetaObject * QProgressBar::metaObject();
  fn _ZNK12QProgressBar10metaObjectEv(qthis: *mut c_void);
  // proto:  int QProgressBar::maximum();
  fn _ZNK12QProgressBar7maximumEv(qthis: *mut c_void) -> c_int;
  // proto:  void QProgressBar::setFormat(const QString & format);
  fn _ZN12QProgressBar9setFormatERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QProgressBar::invertedAppearance();
  fn _ZNK12QProgressBar18invertedAppearanceEv(qthis: *mut c_void) -> c_char;
  // proto:  QString QProgressBar::text();
  fn _ZNK12QProgressBar4textEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QProgressBar::setMinimum(int minimum);
  fn _ZN12QProgressBar10setMinimumEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QProgressBar::QProgressBar(const QProgressBar & );
  fn dector_ZN12QProgressBarC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN12QProgressBarC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QProgressBar::valueChanged(int value);
  fn _ZN12QProgressBar12valueChangedEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QProgressBar::QProgressBar(QWidget * parent);
  fn dector_ZN12QProgressBarC1EP7QWidget(arg0: *mut c_void) -> *mut c_void;
  fn _ZN12QProgressBarC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QProgressBar::setTextVisible(bool visible);
  fn _ZN12QProgressBar14setTextVisibleEb(qthis: *mut c_void, arg0: c_char);
  // proto:  int QProgressBar::value();
  fn _ZNK12QProgressBar5valueEv(qthis: *mut c_void) -> c_int;
  // proto:  void QProgressBar::setValue(int value);
  fn _ZN12QProgressBar8setValueEi(qthis: *mut c_void, arg0: c_int);
  // proto:  QSize QProgressBar::minimumSizeHint();
  fn _ZNK12QProgressBar15minimumSizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QProgressBar::minimum();
  fn _ZNK12QProgressBar7minimumEv(qthis: *mut c_void) -> c_int;
  // proto:  void QProgressBar::setRange(int minimum, int maximum);
  fn _ZN12QProgressBar8setRangeEii(qthis: *mut c_void, arg0: c_int, arg1: c_int);
  // proto:  QSize QProgressBar::sizeHint();
  fn _ZNK12QProgressBar8sizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QProgressBar::resetFormat();
  fn _ZN12QProgressBar11resetFormatEv(qthis: *mut c_void);
  // proto:  bool QProgressBar::isTextVisible();
  fn _ZNK12QProgressBar13isTextVisibleEv(qthis: *mut c_void) -> c_char;
  // proto:  void QProgressBar::setInvertedAppearance(bool invert);
  fn _ZN12QProgressBar21setInvertedAppearanceEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QProgressBar::~QProgressBar();
  fn _ZN12QProgressBarD0Ev(qthis: *mut c_void);
  // proto:  void QProgressBar::setMaximum(int maximum);
  fn _ZN12QProgressBar10setMaximumEi(qthis: *mut c_void, arg0: c_int);
} // <= ext block end

// body block begin =>
// class sizeof(QProgressBar)=1
pub struct QProgressBar {
  qbase: QWidget,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QProgressBar {
  pub fn inheritFrom(qthis: *mut c_void) -> QProgressBar {
    return QProgressBar{qbase: QWidget::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QProgressBar {
  type Target = QWidget;

  fn deref(&self) -> &QWidget {
    return & self.qbase;
  }
}
impl AsRef<QWidget> for QProgressBar {
  fn as_ref(& self) -> & QWidget {
    return & self.qbase;
  }
}
  // proto:  QString QProgressBar::format();
impl /*struct*/ QProgressBar {
  pub fn format<RetType, T: QProgressBar_format<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.format(self);
    // return 1;
  }
}

pub trait QProgressBar_format<RetType> {
  fn format(self , rsthis: & QProgressBar) -> RetType;
}

  // proto:  QString QProgressBar::format();
impl<'a> /*trait*/ QProgressBar_format<QString> for () {
  fn format(self , rsthis: & QProgressBar) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QProgressBar6formatEv()};
    let mut ret = unsafe {_ZNK12QProgressBar6formatEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QProgressBar::reset();
impl /*struct*/ QProgressBar {
  pub fn reset<RetType, T: QProgressBar_reset<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.reset(self);
    // return 1;
  }
}

pub trait QProgressBar_reset<RetType> {
  fn reset(self , rsthis: & QProgressBar) -> RetType;
}

  // proto:  void QProgressBar::reset();
impl<'a> /*trait*/ QProgressBar_reset<()> for () {
  fn reset(self , rsthis: & QProgressBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QProgressBar5resetEv()};
     unsafe {_ZN12QProgressBar5resetEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QProgressBar::metaObject();
impl /*struct*/ QProgressBar {
  pub fn metaObject<RetType, T: QProgressBar_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QProgressBar_metaObject<RetType> {
  fn metaObject(self , rsthis: & QProgressBar) -> RetType;
}

  // proto:  const QMetaObject * QProgressBar::metaObject();
impl<'a> /*trait*/ QProgressBar_metaObject<()> for () {
  fn metaObject(self , rsthis: & QProgressBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QProgressBar10metaObjectEv()};
     unsafe {_ZNK12QProgressBar10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QProgressBar::maximum();
impl /*struct*/ QProgressBar {
  pub fn maximum<RetType, T: QProgressBar_maximum<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.maximum(self);
    // return 1;
  }
}

pub trait QProgressBar_maximum<RetType> {
  fn maximum(self , rsthis: & QProgressBar) -> RetType;
}

  // proto:  int QProgressBar::maximum();
impl<'a> /*trait*/ QProgressBar_maximum<i32> for () {
  fn maximum(self , rsthis: & QProgressBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QProgressBar7maximumEv()};
    let mut ret = unsafe {_ZNK12QProgressBar7maximumEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QProgressBar::setFormat(const QString & format);
impl /*struct*/ QProgressBar {
  pub fn setFormat<RetType, T: QProgressBar_setFormat<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFormat(self);
    // return 1;
  }
}

pub trait QProgressBar_setFormat<RetType> {
  fn setFormat(self , rsthis: & QProgressBar) -> RetType;
}

  // proto:  void QProgressBar::setFormat(const QString & format);
impl<'a> /*trait*/ QProgressBar_setFormat<()> for (&'a QString) {
  fn setFormat(self , rsthis: & QProgressBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QProgressBar9setFormatERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QProgressBar9setFormatERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QProgressBar::invertedAppearance();
impl /*struct*/ QProgressBar {
  pub fn invertedAppearance<RetType, T: QProgressBar_invertedAppearance<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.invertedAppearance(self);
    // return 1;
  }
}

pub trait QProgressBar_invertedAppearance<RetType> {
  fn invertedAppearance(self , rsthis: & QProgressBar) -> RetType;
}

  // proto:  bool QProgressBar::invertedAppearance();
impl<'a> /*trait*/ QProgressBar_invertedAppearance<i8> for () {
  fn invertedAppearance(self , rsthis: & QProgressBar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QProgressBar18invertedAppearanceEv()};
    let mut ret = unsafe {_ZNK12QProgressBar18invertedAppearanceEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QString QProgressBar::text();
impl /*struct*/ QProgressBar {
  pub fn text<RetType, T: QProgressBar_text<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.text(self);
    // return 1;
  }
}

pub trait QProgressBar_text<RetType> {
  fn text(self , rsthis: & QProgressBar) -> RetType;
}

  // proto:  QString QProgressBar::text();
impl<'a> /*trait*/ QProgressBar_text<QString> for () {
  fn text(self , rsthis: & QProgressBar) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QProgressBar4textEv()};
    let mut ret = unsafe {_ZNK12QProgressBar4textEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QProgressBar::setMinimum(int minimum);
impl /*struct*/ QProgressBar {
  pub fn setMinimum<RetType, T: QProgressBar_setMinimum<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMinimum(self);
    // return 1;
  }
}

pub trait QProgressBar_setMinimum<RetType> {
  fn setMinimum(self , rsthis: & QProgressBar) -> RetType;
}

  // proto:  void QProgressBar::setMinimum(int minimum);
impl<'a> /*trait*/ QProgressBar_setMinimum<()> for (i32) {
  fn setMinimum(self , rsthis: & QProgressBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QProgressBar10setMinimumEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN12QProgressBar10setMinimumEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QProgressBar::QProgressBar(const QProgressBar & );
impl /*struct*/ QProgressBar {
  pub fn New<T: QProgressBar_New>(value: T) -> QProgressBar {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QProgressBar_New {
  fn New(self) -> QProgressBar;
}

  // proto:  void QProgressBar::QProgressBar(const QProgressBar & );
impl<'a> /*trait*/ QProgressBar_New for (&'a QProgressBar) {
  fn New(self) -> QProgressBar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QProgressBarC1ERKS_()};
    let ctysz: c_int = unsafe{QProgressBar_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN12QProgressBarC1ERKS_(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN12QProgressBarC1ERKS_(arg0)};
    let rsthis = QProgressBar{/**/qbase: QWidget::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QProgressBar::valueChanged(int value);
impl /*struct*/ QProgressBar {
  pub fn valueChanged<RetType, T: QProgressBar_valueChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.valueChanged(self);
    // return 1;
  }
}

pub trait QProgressBar_valueChanged<RetType> {
  fn valueChanged(self , rsthis: & QProgressBar) -> RetType;
}

  // proto:  void QProgressBar::valueChanged(int value);
impl<'a> /*trait*/ QProgressBar_valueChanged<()> for (i32) {
  fn valueChanged(self , rsthis: & QProgressBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QProgressBar12valueChangedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN12QProgressBar12valueChangedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QProgressBar::QProgressBar(QWidget * parent);
impl<'a> /*trait*/ QProgressBar_New for (&'a QWidget) {
  fn New(self) -> QProgressBar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QProgressBarC1EP7QWidget()};
    let ctysz: c_int = unsafe{QProgressBar_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN12QProgressBarC1EP7QWidget(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN12QProgressBarC1EP7QWidget(arg0)};
    let rsthis = QProgressBar{/**/qbase: QWidget::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QProgressBar::setTextVisible(bool visible);
impl /*struct*/ QProgressBar {
  pub fn setTextVisible<RetType, T: QProgressBar_setTextVisible<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTextVisible(self);
    // return 1;
  }
}

pub trait QProgressBar_setTextVisible<RetType> {
  fn setTextVisible(self , rsthis: & QProgressBar) -> RetType;
}

  // proto:  void QProgressBar::setTextVisible(bool visible);
impl<'a> /*trait*/ QProgressBar_setTextVisible<()> for (i8) {
  fn setTextVisible(self , rsthis: & QProgressBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QProgressBar14setTextVisibleEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN12QProgressBar14setTextVisibleEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QProgressBar::value();
impl /*struct*/ QProgressBar {
  pub fn value<RetType, T: QProgressBar_value<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.value(self);
    // return 1;
  }
}

pub trait QProgressBar_value<RetType> {
  fn value(self , rsthis: & QProgressBar) -> RetType;
}

  // proto:  int QProgressBar::value();
impl<'a> /*trait*/ QProgressBar_value<i32> for () {
  fn value(self , rsthis: & QProgressBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QProgressBar5valueEv()};
    let mut ret = unsafe {_ZNK12QProgressBar5valueEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QProgressBar::setValue(int value);
impl /*struct*/ QProgressBar {
  pub fn setValue<RetType, T: QProgressBar_setValue<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setValue(self);
    // return 1;
  }
}

pub trait QProgressBar_setValue<RetType> {
  fn setValue(self , rsthis: & QProgressBar) -> RetType;
}

  // proto:  void QProgressBar::setValue(int value);
impl<'a> /*trait*/ QProgressBar_setValue<()> for (i32) {
  fn setValue(self , rsthis: & QProgressBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QProgressBar8setValueEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN12QProgressBar8setValueEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QSize QProgressBar::minimumSizeHint();
impl /*struct*/ QProgressBar {
  pub fn minimumSizeHint<RetType, T: QProgressBar_minimumSizeHint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.minimumSizeHint(self);
    // return 1;
  }
}

pub trait QProgressBar_minimumSizeHint<RetType> {
  fn minimumSizeHint(self , rsthis: & QProgressBar) -> RetType;
}

  // proto:  QSize QProgressBar::minimumSizeHint();
impl<'a> /*trait*/ QProgressBar_minimumSizeHint<QSize> for () {
  fn minimumSizeHint(self , rsthis: & QProgressBar) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QProgressBar15minimumSizeHintEv()};
    let mut ret = unsafe {_ZNK12QProgressBar15minimumSizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  int QProgressBar::minimum();
impl /*struct*/ QProgressBar {
  pub fn minimum<RetType, T: QProgressBar_minimum<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.minimum(self);
    // return 1;
  }
}

pub trait QProgressBar_minimum<RetType> {
  fn minimum(self , rsthis: & QProgressBar) -> RetType;
}

  // proto:  int QProgressBar::minimum();
impl<'a> /*trait*/ QProgressBar_minimum<i32> for () {
  fn minimum(self , rsthis: & QProgressBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QProgressBar7minimumEv()};
    let mut ret = unsafe {_ZNK12QProgressBar7minimumEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QProgressBar::setRange(int minimum, int maximum);
impl /*struct*/ QProgressBar {
  pub fn setRange<RetType, T: QProgressBar_setRange<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setRange(self);
    // return 1;
  }
}

pub trait QProgressBar_setRange<RetType> {
  fn setRange(self , rsthis: & QProgressBar) -> RetType;
}

  // proto:  void QProgressBar::setRange(int minimum, int maximum);
impl<'a> /*trait*/ QProgressBar_setRange<()> for (i32, i32) {
  fn setRange(self , rsthis: & QProgressBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QProgressBar8setRangeEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN12QProgressBar8setRangeEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QSize QProgressBar::sizeHint();
impl /*struct*/ QProgressBar {
  pub fn sizeHint<RetType, T: QProgressBar_sizeHint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sizeHint(self);
    // return 1;
  }
}

pub trait QProgressBar_sizeHint<RetType> {
  fn sizeHint(self , rsthis: & QProgressBar) -> RetType;
}

  // proto:  QSize QProgressBar::sizeHint();
impl<'a> /*trait*/ QProgressBar_sizeHint<QSize> for () {
  fn sizeHint(self , rsthis: & QProgressBar) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QProgressBar8sizeHintEv()};
    let mut ret = unsafe {_ZNK12QProgressBar8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QProgressBar::resetFormat();
impl /*struct*/ QProgressBar {
  pub fn resetFormat<RetType, T: QProgressBar_resetFormat<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.resetFormat(self);
    // return 1;
  }
}

pub trait QProgressBar_resetFormat<RetType> {
  fn resetFormat(self , rsthis: & QProgressBar) -> RetType;
}

  // proto:  void QProgressBar::resetFormat();
impl<'a> /*trait*/ QProgressBar_resetFormat<()> for () {
  fn resetFormat(self , rsthis: & QProgressBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QProgressBar11resetFormatEv()};
     unsafe {_ZN12QProgressBar11resetFormatEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QProgressBar::isTextVisible();
impl /*struct*/ QProgressBar {
  pub fn isTextVisible<RetType, T: QProgressBar_isTextVisible<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isTextVisible(self);
    // return 1;
  }
}

pub trait QProgressBar_isTextVisible<RetType> {
  fn isTextVisible(self , rsthis: & QProgressBar) -> RetType;
}

  // proto:  bool QProgressBar::isTextVisible();
impl<'a> /*trait*/ QProgressBar_isTextVisible<i8> for () {
  fn isTextVisible(self , rsthis: & QProgressBar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QProgressBar13isTextVisibleEv()};
    let mut ret = unsafe {_ZNK12QProgressBar13isTextVisibleEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QProgressBar::setInvertedAppearance(bool invert);
impl /*struct*/ QProgressBar {
  pub fn setInvertedAppearance<RetType, T: QProgressBar_setInvertedAppearance<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setInvertedAppearance(self);
    // return 1;
  }
}

pub trait QProgressBar_setInvertedAppearance<RetType> {
  fn setInvertedAppearance(self , rsthis: & QProgressBar) -> RetType;
}

  // proto:  void QProgressBar::setInvertedAppearance(bool invert);
impl<'a> /*trait*/ QProgressBar_setInvertedAppearance<()> for (i8) {
  fn setInvertedAppearance(self , rsthis: & QProgressBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QProgressBar21setInvertedAppearanceEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN12QProgressBar21setInvertedAppearanceEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QProgressBar::~QProgressBar();
impl /*struct*/ QProgressBar {
  pub fn Free<RetType, T: QProgressBar_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QProgressBar_Free<RetType> {
  fn Free(self , rsthis: & QProgressBar) -> RetType;
}

  // proto:  void QProgressBar::~QProgressBar();
impl<'a> /*trait*/ QProgressBar_Free<()> for () {
  fn Free(self , rsthis: & QProgressBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QProgressBarD0Ev()};
     unsafe {_ZN12QProgressBarD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QProgressBar::setMaximum(int maximum);
impl /*struct*/ QProgressBar {
  pub fn setMaximum<RetType, T: QProgressBar_setMaximum<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMaximum(self);
    // return 1;
  }
}

pub trait QProgressBar_setMaximum<RetType> {
  fn setMaximum(self , rsthis: & QProgressBar) -> RetType;
}

  // proto:  void QProgressBar::setMaximum(int maximum);
impl<'a> /*trait*/ QProgressBar_setMaximum<()> for (i32) {
  fn setMaximum(self , rsthis: & QProgressBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QProgressBar10setMaximumEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN12QProgressBar10setMaximumEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// <= body block end

