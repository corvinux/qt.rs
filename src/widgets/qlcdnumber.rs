// auto generated, do not modify.
// created: Tue Dec 22 23:21:28 2015
// src-file: /QtWidgets/qlcdnumber.h
// dst-file: /src/widgets/qlcdnumber.rs
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
use super::qframe::QFrame; // 773
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
  // proto:  void QLCDNumber::display(int num);
  fn _ZN10QLCDNumber7displayEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QLCDNumber::setHexMode();
  fn _ZN10QLCDNumber10setHexModeEv(qthis: *mut c_void);
  // proto:  void QLCDNumber::display(double num);
  fn _ZN10QLCDNumber7displayEd(qthis: *mut c_void, arg0: c_double);
  // proto:  const QMetaObject * QLCDNumber::metaObject();
  fn _ZNK10QLCDNumber10metaObjectEv(qthis: *mut c_void);
  // proto:  void QLCDNumber::QLCDNumber(const QLCDNumber & );
  fn _ZN10QLCDNumberC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  int QLCDNumber::digitCount();
  fn _ZNK10QLCDNumber10digitCountEv(qthis: *mut c_void) -> c_int;
  // proto:  void QLCDNumber::~QLCDNumber();
  fn _ZN10QLCDNumberD0Ev(qthis: *mut c_void);
  // proto:  bool QLCDNumber::checkOverflow(int num);
  fn _ZNK10QLCDNumber13checkOverflowEi(qthis: *mut c_void, arg0: c_int) -> c_char;
  // proto:  void QLCDNumber::setDecMode();
  fn _ZN10QLCDNumber10setDecModeEv(qthis: *mut c_void);
  // proto:  void QLCDNumber::QLCDNumber(uint numDigits, QWidget * parent);
  fn _ZN10QLCDNumberC1EjP7QWidget(qthis: *mut c_void, arg0: c_uint, arg1: *mut c_void);
  // proto:  bool QLCDNumber::checkOverflow(double num);
  fn _ZNK10QLCDNumber13checkOverflowEd(qthis: *mut c_void, arg0: c_double) -> c_char;
  // proto:  QSize QLCDNumber::sizeHint();
  fn _ZNK10QLCDNumber8sizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QLCDNumber::display(const QString & str);
  fn _ZN10QLCDNumber7displayERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QLCDNumber::QLCDNumber(QWidget * parent);
  fn _ZN10QLCDNumberC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  double QLCDNumber::value();
  fn _ZNK10QLCDNumber5valueEv(qthis: *mut c_void) -> c_double;
  // proto:  void QLCDNumber::setBinMode();
  fn _ZN10QLCDNumber10setBinModeEv(qthis: *mut c_void);
  // proto:  int QLCDNumber::intValue();
  fn _ZNK10QLCDNumber8intValueEv(qthis: *mut c_void) -> c_int;
  // proto:  void QLCDNumber::setDigitCount(int nDigits);
  fn _ZN10QLCDNumber13setDigitCountEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QLCDNumber::setSmallDecimalPoint(bool );
  fn _ZN10QLCDNumber20setSmallDecimalPointEb(qthis: *mut c_void, arg0: c_char);
  // proto:  bool QLCDNumber::smallDecimalPoint();
  fn _ZNK10QLCDNumber17smallDecimalPointEv(qthis: *mut c_void) -> c_char;
  // proto:  void QLCDNumber::setOctMode();
  fn _ZN10QLCDNumber10setOctModeEv(qthis: *mut c_void);
  // proto:  void QLCDNumber::overflow();
  fn _ZN10QLCDNumber8overflowEv(qthis: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QLCDNumber)=1
pub struct QLCDNumber {
  qbase: QFrame,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QLCDNumber {
  pub fn inheritFrom(qthis: *mut c_void) -> QLCDNumber {
    return QLCDNumber{qbase: QFrame::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QLCDNumber {
  type Target = QFrame;

  fn deref(&self) -> &QFrame {
    return &self.qbase;
  }
}
impl AsRef<QFrame> for QLCDNumber {
  fn as_ref(&self) -> &QFrame {
    return &self.qbase;
  }
}
  // proto:  void QLCDNumber::display(int num);
impl /*struct*/ QLCDNumber {
  pub fn display<RetType, T: QLCDNumber_display<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.display(self);
    // return 1;
  }
}

pub trait QLCDNumber_display<RetType> {
  fn display(self , rsthis: &mut QLCDNumber) -> RetType;
}

  // proto:  void QLCDNumber::display(int num);
impl<'a> /*trait*/ QLCDNumber_display<()> for (i32) {
  fn display(self , rsthis: &mut QLCDNumber) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QLCDNumber7displayEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QLCDNumber7displayEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QLCDNumber::setHexMode();
impl /*struct*/ QLCDNumber {
  pub fn setHexMode<RetType, T: QLCDNumber_setHexMode<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setHexMode(self);
    // return 1;
  }
}

pub trait QLCDNumber_setHexMode<RetType> {
  fn setHexMode(self , rsthis: &mut QLCDNumber) -> RetType;
}

  // proto:  void QLCDNumber::setHexMode();
impl<'a> /*trait*/ QLCDNumber_setHexMode<()> for () {
  fn setHexMode(self , rsthis: &mut QLCDNumber) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QLCDNumber10setHexModeEv()};
     unsafe {_ZN10QLCDNumber10setHexModeEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QLCDNumber::display(double num);
impl<'a> /*trait*/ QLCDNumber_display<()> for (f64) {
  fn display(self , rsthis: &mut QLCDNumber) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QLCDNumber7displayEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN10QLCDNumber7displayEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QLCDNumber::metaObject();
impl /*struct*/ QLCDNumber {
  pub fn metaObject<RetType, T: QLCDNumber_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QLCDNumber_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QLCDNumber) -> RetType;
}

  // proto:  const QMetaObject * QLCDNumber::metaObject();
impl<'a> /*trait*/ QLCDNumber_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QLCDNumber) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QLCDNumber10metaObjectEv()};
     unsafe {_ZNK10QLCDNumber10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QLCDNumber::QLCDNumber(const QLCDNumber & );
impl /*struct*/ QLCDNumber {
  pub fn NewQLCDNumber<T: QLCDNumber_NewQLCDNumber>(value: T) -> QLCDNumber {
    let rsthis = value.NewQLCDNumber();
    return rsthis;
    // return 1;
  }
}

pub trait QLCDNumber_NewQLCDNumber {
  fn NewQLCDNumber(self) -> QLCDNumber;
}

  // proto:  void QLCDNumber::QLCDNumber(const QLCDNumber & );
impl<'a> /*trait*/ QLCDNumber_NewQLCDNumber for (QLCDNumber) {
  fn NewQLCDNumber(self) -> QLCDNumber {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QLCDNumberC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN10QLCDNumberC1ERKS_(qthis, arg0)};
    let rsthis = QLCDNumber{/**/qbase: QFrame::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QLCDNumber::digitCount();
impl /*struct*/ QLCDNumber {
  pub fn digitCount<RetType, T: QLCDNumber_digitCount<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.digitCount(self);
    // return 1;
  }
}

pub trait QLCDNumber_digitCount<RetType> {
  fn digitCount(self , rsthis: &mut QLCDNumber) -> RetType;
}

  // proto:  int QLCDNumber::digitCount();
impl<'a> /*trait*/ QLCDNumber_digitCount<i32> for () {
  fn digitCount(self , rsthis: &mut QLCDNumber) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QLCDNumber10digitCountEv()};
    let mut ret = unsafe {_ZNK10QLCDNumber10digitCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QLCDNumber::~QLCDNumber();
impl /*struct*/ QLCDNumber {
  pub fn FreeQLCDNumber<RetType, T: QLCDNumber_FreeQLCDNumber<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQLCDNumber(self);
    // return 1;
  }
}

pub trait QLCDNumber_FreeQLCDNumber<RetType> {
  fn FreeQLCDNumber(self , rsthis: &mut QLCDNumber) -> RetType;
}

  // proto:  void QLCDNumber::~QLCDNumber();
impl<'a> /*trait*/ QLCDNumber_FreeQLCDNumber<()> for () {
  fn FreeQLCDNumber(self , rsthis: &mut QLCDNumber) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QLCDNumberD0Ev()};
     unsafe {_ZN10QLCDNumberD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QLCDNumber::checkOverflow(int num);
impl /*struct*/ QLCDNumber {
  pub fn checkOverflow<RetType, T: QLCDNumber_checkOverflow<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.checkOverflow(self);
    // return 1;
  }
}

pub trait QLCDNumber_checkOverflow<RetType> {
  fn checkOverflow(self , rsthis: &mut QLCDNumber) -> RetType;
}

  // proto:  bool QLCDNumber::checkOverflow(int num);
impl<'a> /*trait*/ QLCDNumber_checkOverflow<i8> for (i32) {
  fn checkOverflow(self , rsthis: &mut QLCDNumber) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QLCDNumber13checkOverflowEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK10QLCDNumber13checkOverflowEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QLCDNumber::setDecMode();
impl /*struct*/ QLCDNumber {
  pub fn setDecMode<RetType, T: QLCDNumber_setDecMode<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setDecMode(self);
    // return 1;
  }
}

pub trait QLCDNumber_setDecMode<RetType> {
  fn setDecMode(self , rsthis: &mut QLCDNumber) -> RetType;
}

  // proto:  void QLCDNumber::setDecMode();
impl<'a> /*trait*/ QLCDNumber_setDecMode<()> for () {
  fn setDecMode(self , rsthis: &mut QLCDNumber) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QLCDNumber10setDecModeEv()};
     unsafe {_ZN10QLCDNumber10setDecModeEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QLCDNumber::QLCDNumber(uint numDigits, QWidget * parent);
impl<'a> /*trait*/ QLCDNumber_NewQLCDNumber for (u32, QWidget) {
  fn NewQLCDNumber(self) -> QLCDNumber {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QLCDNumberC1EjP7QWidget()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN10QLCDNumberC1EjP7QWidget(qthis, arg0, arg1)};
    let rsthis = QLCDNumber{/**/qbase: QFrame::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QLCDNumber::checkOverflow(double num);
impl<'a> /*trait*/ QLCDNumber_checkOverflow<i8> for (f64) {
  fn checkOverflow(self , rsthis: &mut QLCDNumber) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QLCDNumber13checkOverflowEd()};
    let arg0 = self  as c_double;
    let mut ret = unsafe {_ZNK10QLCDNumber13checkOverflowEd(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QSize QLCDNumber::sizeHint();
impl /*struct*/ QLCDNumber {
  pub fn sizeHint<RetType, T: QLCDNumber_sizeHint<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.sizeHint(self);
    // return 1;
  }
}

pub trait QLCDNumber_sizeHint<RetType> {
  fn sizeHint(self , rsthis: &mut QLCDNumber) -> RetType;
}

  // proto:  QSize QLCDNumber::sizeHint();
impl<'a> /*trait*/ QLCDNumber_sizeHint<QSize> for () {
  fn sizeHint(self , rsthis: &mut QLCDNumber) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QLCDNumber8sizeHintEv()};
    let mut ret = unsafe {_ZNK10QLCDNumber8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QLCDNumber::display(const QString & str);
impl<'a> /*trait*/ QLCDNumber_display<()> for (QString) {
  fn display(self , rsthis: &mut QLCDNumber) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QLCDNumber7displayERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QLCDNumber7displayERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QLCDNumber::QLCDNumber(QWidget * parent);
impl<'a> /*trait*/ QLCDNumber_NewQLCDNumber for (QWidget) {
  fn NewQLCDNumber(self) -> QLCDNumber {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QLCDNumberC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN10QLCDNumberC1EP7QWidget(qthis, arg0)};
    let rsthis = QLCDNumber{/**/qbase: QFrame::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  double QLCDNumber::value();
impl /*struct*/ QLCDNumber {
  pub fn value<RetType, T: QLCDNumber_value<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.value(self);
    // return 1;
  }
}

pub trait QLCDNumber_value<RetType> {
  fn value(self , rsthis: &mut QLCDNumber) -> RetType;
}

  // proto:  double QLCDNumber::value();
impl<'a> /*trait*/ QLCDNumber_value<f64> for () {
  fn value(self , rsthis: &mut QLCDNumber) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QLCDNumber5valueEv()};
    let mut ret = unsafe {_ZNK10QLCDNumber5valueEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QLCDNumber::setBinMode();
impl /*struct*/ QLCDNumber {
  pub fn setBinMode<RetType, T: QLCDNumber_setBinMode<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setBinMode(self);
    // return 1;
  }
}

pub trait QLCDNumber_setBinMode<RetType> {
  fn setBinMode(self , rsthis: &mut QLCDNumber) -> RetType;
}

  // proto:  void QLCDNumber::setBinMode();
impl<'a> /*trait*/ QLCDNumber_setBinMode<()> for () {
  fn setBinMode(self , rsthis: &mut QLCDNumber) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QLCDNumber10setBinModeEv()};
     unsafe {_ZN10QLCDNumber10setBinModeEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QLCDNumber::intValue();
impl /*struct*/ QLCDNumber {
  pub fn intValue<RetType, T: QLCDNumber_intValue<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.intValue(self);
    // return 1;
  }
}

pub trait QLCDNumber_intValue<RetType> {
  fn intValue(self , rsthis: &mut QLCDNumber) -> RetType;
}

  // proto:  int QLCDNumber::intValue();
impl<'a> /*trait*/ QLCDNumber_intValue<i32> for () {
  fn intValue(self , rsthis: &mut QLCDNumber) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QLCDNumber8intValueEv()};
    let mut ret = unsafe {_ZNK10QLCDNumber8intValueEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QLCDNumber::setDigitCount(int nDigits);
impl /*struct*/ QLCDNumber {
  pub fn setDigitCount<RetType, T: QLCDNumber_setDigitCount<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setDigitCount(self);
    // return 1;
  }
}

pub trait QLCDNumber_setDigitCount<RetType> {
  fn setDigitCount(self , rsthis: &mut QLCDNumber) -> RetType;
}

  // proto:  void QLCDNumber::setDigitCount(int nDigits);
impl<'a> /*trait*/ QLCDNumber_setDigitCount<()> for (i32) {
  fn setDigitCount(self , rsthis: &mut QLCDNumber) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QLCDNumber13setDigitCountEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QLCDNumber13setDigitCountEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QLCDNumber::setSmallDecimalPoint(bool );
impl /*struct*/ QLCDNumber {
  pub fn setSmallDecimalPoint<RetType, T: QLCDNumber_setSmallDecimalPoint<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setSmallDecimalPoint(self);
    // return 1;
  }
}

pub trait QLCDNumber_setSmallDecimalPoint<RetType> {
  fn setSmallDecimalPoint(self , rsthis: &mut QLCDNumber) -> RetType;
}

  // proto:  void QLCDNumber::setSmallDecimalPoint(bool );
impl<'a> /*trait*/ QLCDNumber_setSmallDecimalPoint<()> for (i8) {
  fn setSmallDecimalPoint(self , rsthis: &mut QLCDNumber) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QLCDNumber20setSmallDecimalPointEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN10QLCDNumber20setSmallDecimalPointEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QLCDNumber::smallDecimalPoint();
impl /*struct*/ QLCDNumber {
  pub fn smallDecimalPoint<RetType, T: QLCDNumber_smallDecimalPoint<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.smallDecimalPoint(self);
    // return 1;
  }
}

pub trait QLCDNumber_smallDecimalPoint<RetType> {
  fn smallDecimalPoint(self , rsthis: &mut QLCDNumber) -> RetType;
}

  // proto:  bool QLCDNumber::smallDecimalPoint();
impl<'a> /*trait*/ QLCDNumber_smallDecimalPoint<i8> for () {
  fn smallDecimalPoint(self , rsthis: &mut QLCDNumber) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QLCDNumber17smallDecimalPointEv()};
    let mut ret = unsafe {_ZNK10QLCDNumber17smallDecimalPointEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QLCDNumber::setOctMode();
impl /*struct*/ QLCDNumber {
  pub fn setOctMode<RetType, T: QLCDNumber_setOctMode<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setOctMode(self);
    // return 1;
  }
}

pub trait QLCDNumber_setOctMode<RetType> {
  fn setOctMode(self , rsthis: &mut QLCDNumber) -> RetType;
}

  // proto:  void QLCDNumber::setOctMode();
impl<'a> /*trait*/ QLCDNumber_setOctMode<()> for () {
  fn setOctMode(self , rsthis: &mut QLCDNumber) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QLCDNumber10setOctModeEv()};
     unsafe {_ZN10QLCDNumber10setOctModeEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QLCDNumber::overflow();
impl /*struct*/ QLCDNumber {
  pub fn overflow<RetType, T: QLCDNumber_overflow<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.overflow(self);
    // return 1;
  }
}

pub trait QLCDNumber_overflow<RetType> {
  fn overflow(self , rsthis: &mut QLCDNumber) -> RetType;
}

  // proto:  void QLCDNumber::overflow();
impl<'a> /*trait*/ QLCDNumber_overflow<()> for () {
  fn overflow(self , rsthis: &mut QLCDNumber) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QLCDNumber8overflowEv()};
     unsafe {_ZN10QLCDNumber8overflowEv(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

