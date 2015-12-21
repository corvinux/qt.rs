// auto generated, do not modify.
// created: Mon Dec 21 22:54:38 2015
// src-file: /QtGui/qstatictext.h
// dst-file: /src/gui/qstatictext.rs
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
use super::super::core::qstring::QString; // 771
use super::super::core::qsize::QSizeF; // 771
use super::qtransform::QTransform; // 773
use super::qfont::QFont; // 773
use super::qtextoption::QTextOption; // 773
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  void QStaticText::QStaticText(const QString & text);
  fn _ZN11QStaticTextC1ERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QSizeF QStaticText::size();
  fn _ZNK11QStaticText4sizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString QStaticText::text();
  fn _ZNK11QStaticText4textEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QStaticText::~QStaticText();
  fn _ZN11QStaticTextD0Ev(qthis: *mut c_void);
  // proto:  void QStaticText::setText(const QString & text);
  fn _ZN11QStaticText7setTextERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QStaticText::QStaticText();
  fn _ZN11QStaticTextC1Ev(qthis: *mut c_void);
  // proto:  void QStaticText::prepare(const QTransform & matrix, const QFont & font);
  fn _ZN11QStaticText7prepareERK10QTransformRK5QFont(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QStaticText::setTextOption(const QTextOption & textOption);
  fn _ZN11QStaticText13setTextOptionERK11QTextOption(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QStaticText::setTextWidth(qreal textWidth);
  fn _ZN11QStaticText12setTextWidthEd(qthis: *mut c_void, arg0: c_double);
  // proto:  qreal QStaticText::textWidth();
  fn _ZNK11QStaticText9textWidthEv(qthis: *mut c_void) -> c_double;
  // proto:  void QStaticText::swap(QStaticText & other);
  fn _ZN11QStaticText4swapERS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QTextOption QStaticText::textOption();
  fn _ZNK11QStaticText10textOptionEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QStaticText::QStaticText(const QStaticText & other);
  fn _ZN11QStaticTextC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QStaticText)=1
pub struct QStaticText {
  pub qclsinst: *mut c_void,
}

  // proto:  void QStaticText::QStaticText(const QString & text);
impl /*struct*/ QStaticText {
  pub fn NewQStaticText<T: QStaticText_NewQStaticText>(value: T) -> QStaticText {
    let rsthis = value.NewQStaticText();
    return rsthis;
    // return 1;
  }
}

pub trait QStaticText_NewQStaticText {
  fn NewQStaticText(self) -> QStaticText;
}

  // proto:  void QStaticText::QStaticText(const QString & text);
impl<'a> /*trait*/ QStaticText_NewQStaticText for (QString) {
  fn NewQStaticText(self) -> QStaticText {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QStaticTextC1ERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QStaticTextC1ERK7QString(qthis, arg0)};
    let rsthis = QStaticText{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QSizeF QStaticText::size();
impl /*struct*/ QStaticText {
  pub fn size<RetType, T: QStaticText_size<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.size(self);
    // return 1;
  }
}

pub trait QStaticText_size<RetType> {
  fn size(self , rsthis: &mut QStaticText) -> RetType;
}

  // proto:  QSizeF QStaticText::size();
impl<'a> /*trait*/ QStaticText_size<QSizeF> for () {
  fn size(self , rsthis: &mut QStaticText) -> QSizeF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QStaticText4sizeEv()};
    let mut ret = unsafe {_ZNK11QStaticText4sizeEv(rsthis.qclsinst)};
    let mut ret1 = QSizeF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QString QStaticText::text();
impl /*struct*/ QStaticText {
  pub fn text<RetType, T: QStaticText_text<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.text(self);
    // return 1;
  }
}

pub trait QStaticText_text<RetType> {
  fn text(self , rsthis: &mut QStaticText) -> RetType;
}

  // proto:  QString QStaticText::text();
impl<'a> /*trait*/ QStaticText_text<QString> for () {
  fn text(self , rsthis: &mut QStaticText) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QStaticText4textEv()};
    let mut ret = unsafe {_ZNK11QStaticText4textEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QStaticText::~QStaticText();
impl /*struct*/ QStaticText {
  pub fn FreeQStaticText<RetType, T: QStaticText_FreeQStaticText<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQStaticText(self);
    // return 1;
  }
}

pub trait QStaticText_FreeQStaticText<RetType> {
  fn FreeQStaticText(self , rsthis: &mut QStaticText) -> RetType;
}

  // proto:  void QStaticText::~QStaticText();
impl<'a> /*trait*/ QStaticText_FreeQStaticText<()> for () {
  fn FreeQStaticText(self , rsthis: &mut QStaticText) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QStaticTextD0Ev()};
     unsafe {_ZN11QStaticTextD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QStaticText::setText(const QString & text);
impl /*struct*/ QStaticText {
  pub fn setText<RetType, T: QStaticText_setText<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setText(self);
    // return 1;
  }
}

pub trait QStaticText_setText<RetType> {
  fn setText(self , rsthis: &mut QStaticText) -> RetType;
}

  // proto:  void QStaticText::setText(const QString & text);
impl<'a> /*trait*/ QStaticText_setText<()> for (QString) {
  fn setText(self , rsthis: &mut QStaticText) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QStaticText7setTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QStaticText7setTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QStaticText::QStaticText();
impl<'a> /*trait*/ QStaticText_NewQStaticText for () {
  fn NewQStaticText(self) -> QStaticText {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QStaticTextC1Ev()};
    unsafe {_ZN11QStaticTextC1Ev(qthis)};
    let rsthis = QStaticText{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStaticText::prepare(const QTransform & matrix, const QFont & font);
impl /*struct*/ QStaticText {
  pub fn prepare<RetType, T: QStaticText_prepare<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.prepare(self);
    // return 1;
  }
}

pub trait QStaticText_prepare<RetType> {
  fn prepare(self , rsthis: &mut QStaticText) -> RetType;
}

  // proto:  void QStaticText::prepare(const QTransform & matrix, const QFont & font);
impl<'a> /*trait*/ QStaticText_prepare<()> for (QTransform, QFont) {
  fn prepare(self , rsthis: &mut QStaticText) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QStaticText7prepareERK10QTransformRK5QFont()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN11QStaticText7prepareERK10QTransformRK5QFont(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QStaticText::setTextOption(const QTextOption & textOption);
impl /*struct*/ QStaticText {
  pub fn setTextOption<RetType, T: QStaticText_setTextOption<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setTextOption(self);
    // return 1;
  }
}

pub trait QStaticText_setTextOption<RetType> {
  fn setTextOption(self , rsthis: &mut QStaticText) -> RetType;
}

  // proto:  void QStaticText::setTextOption(const QTextOption & textOption);
impl<'a> /*trait*/ QStaticText_setTextOption<()> for (QTextOption) {
  fn setTextOption(self , rsthis: &mut QStaticText) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QStaticText13setTextOptionERK11QTextOption()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QStaticText13setTextOptionERK11QTextOption(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QStaticText::setTextWidth(qreal textWidth);
impl /*struct*/ QStaticText {
  pub fn setTextWidth<RetType, T: QStaticText_setTextWidth<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setTextWidth(self);
    // return 1;
  }
}

pub trait QStaticText_setTextWidth<RetType> {
  fn setTextWidth(self , rsthis: &mut QStaticText) -> RetType;
}

  // proto:  void QStaticText::setTextWidth(qreal textWidth);
impl<'a> /*trait*/ QStaticText_setTextWidth<()> for (f64) {
  fn setTextWidth(self , rsthis: &mut QStaticText) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QStaticText12setTextWidthEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN11QStaticText12setTextWidthEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  qreal QStaticText::textWidth();
impl /*struct*/ QStaticText {
  pub fn textWidth<RetType, T: QStaticText_textWidth<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.textWidth(self);
    // return 1;
  }
}

pub trait QStaticText_textWidth<RetType> {
  fn textWidth(self , rsthis: &mut QStaticText) -> RetType;
}

  // proto:  qreal QStaticText::textWidth();
impl<'a> /*trait*/ QStaticText_textWidth<f64> for () {
  fn textWidth(self , rsthis: &mut QStaticText) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QStaticText9textWidthEv()};
    let mut ret = unsafe {_ZNK11QStaticText9textWidthEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QStaticText::swap(QStaticText & other);
impl /*struct*/ QStaticText {
  pub fn swap<RetType, T: QStaticText_swap<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QStaticText_swap<RetType> {
  fn swap(self , rsthis: &mut QStaticText) -> RetType;
}

  // proto:  void QStaticText::swap(QStaticText & other);
impl<'a> /*trait*/ QStaticText_swap<()> for (QStaticText) {
  fn swap(self , rsthis: &mut QStaticText) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QStaticText4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QStaticText4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QTextOption QStaticText::textOption();
impl /*struct*/ QStaticText {
  pub fn textOption<RetType, T: QStaticText_textOption<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.textOption(self);
    // return 1;
  }
}

pub trait QStaticText_textOption<RetType> {
  fn textOption(self , rsthis: &mut QStaticText) -> RetType;
}

  // proto:  QTextOption QStaticText::textOption();
impl<'a> /*trait*/ QStaticText_textOption<QTextOption> for () {
  fn textOption(self , rsthis: &mut QStaticText) -> QTextOption {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QStaticText10textOptionEv()};
    let mut ret = unsafe {_ZNK11QStaticText10textOptionEv(rsthis.qclsinst)};
    let mut ret1 = QTextOption{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QStaticText::QStaticText(const QStaticText & other);
impl<'a> /*trait*/ QStaticText_NewQStaticText for (QStaticText) {
  fn NewQStaticText(self) -> QStaticText {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QStaticTextC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QStaticTextC1ERKS_(qthis, arg0)};
    let rsthis = QStaticText{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// <= body block end

