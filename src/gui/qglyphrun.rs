// auto generated, do not modify.
// created: Mon Feb 22 23:57:02 2016
// src-file: /QtGui/qglyphrun.h
// dst-file: /src/gui/qglyphrun.rs
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
use std::ops::Deref;
use super::super::core::qrect::*; // 771
use super::super::core::qpoint::*; // 771
// use super::qvector::*; // 775
use super::qrawfont::*; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QGlyphRun_Class_Size() -> c_int;
  // proto:  void QGlyphRun::~QGlyphRun();
  fn C_ZN9QGlyphRunD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QGlyphRun::setBoundingRect(const QRectF & boundingRect);
  fn C_ZN9QGlyphRun15setBoundingRectERK6QRectF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QGlyphRun::overline();
  fn C_ZNK9QGlyphRun8overlineEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QGlyphRun::setRawData(const quint32 * glyphIndexArray, const QPointF * glyphPositionArray, int size);
  fn C_ZN9QGlyphRun10setRawDataEPKjPK7QPointFi(qthis: u64 /* *mut c_void*/, arg0: *mut c_uint, arg1: *mut c_void, arg2: c_int);
  // proto:  void QGlyphRun::setOverline(bool overline);
  fn C_ZN9QGlyphRun11setOverlineEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QGlyphRun::swap(QGlyphRun & other);
  fn C_ZN9QGlyphRun4swapERS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QGlyphRun::setUnderline(bool underline);
  fn C_ZN9QGlyphRun12setUnderlineEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  QVector<QPointF> QGlyphRun::positions();
  fn C_ZNK9QGlyphRun9positionsEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QGlyphRun::clear();
  fn C_ZN9QGlyphRun5clearEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QGlyphRun::strikeOut();
  fn C_ZNK9QGlyphRun9strikeOutEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QGlyphRun::QGlyphRun();
  fn C_ZN9QGlyphRunC2Ev() -> u64;
  // proto:  QRawFont QGlyphRun::rawFont();
  fn C_ZNK9QGlyphRun7rawFontEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QGlyphRun::setRawFont(const QRawFont & rawFont);
  fn C_ZN9QGlyphRun10setRawFontERK8QRawFont(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QGlyphRun::QGlyphRun(const QGlyphRun & other);
  fn C_ZN9QGlyphRunC2ERKS_(arg0: *mut c_void) -> u64;
  // proto:  bool QGlyphRun::isRightToLeft();
  fn C_ZNK9QGlyphRun13isRightToLeftEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QVector<quint32> QGlyphRun::glyphIndexes();
  fn C_ZNK9QGlyphRun12glyphIndexesEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QRectF QGlyphRun::boundingRect();
  fn C_ZNK9QGlyphRun12boundingRectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QGlyphRun::setRightToLeft(bool on);
  fn C_ZN9QGlyphRun14setRightToLeftEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  bool QGlyphRun::underline();
  fn C_ZNK9QGlyphRun9underlineEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QGlyphRun::setStrikeOut(bool strikeOut);
  fn C_ZN9QGlyphRun12setStrikeOutEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  bool QGlyphRun::isEmpty();
  fn C_ZNK9QGlyphRun7isEmptyEv(qthis: u64 /* *mut c_void*/) -> c_char;
} // <= ext block end

// body block begin =>
// class sizeof(QGlyphRun)=1
#[derive(Default)]
pub struct QGlyphRun {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QGlyphRun {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QGlyphRun {
    return QGlyphRun{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QGlyphRun::~QGlyphRun();
impl /*struct*/ QGlyphRun {
  pub fn free<RetType, T: QGlyphRun_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QGlyphRun_free<RetType> {
  fn free(self , rsthis: & QGlyphRun) -> RetType;
}

  // proto:  void QGlyphRun::~QGlyphRun();
impl<'a> /*trait*/ QGlyphRun_free<()> for () {
  fn free(self , rsthis: & QGlyphRun) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QGlyphRunD2Ev()};
     unsafe {C_ZN9QGlyphRunD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGlyphRun::setBoundingRect(const QRectF & boundingRect);
impl /*struct*/ QGlyphRun {
  pub fn setBoundingRect<RetType, T: QGlyphRun_setBoundingRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setBoundingRect(self);
    // return 1;
  }
}

pub trait QGlyphRun_setBoundingRect<RetType> {
  fn setBoundingRect(self , rsthis: & QGlyphRun) -> RetType;
}

  // proto:  void QGlyphRun::setBoundingRect(const QRectF & boundingRect);
impl<'a> /*trait*/ QGlyphRun_setBoundingRect<()> for (&'a QRectF) {
  fn setBoundingRect(self , rsthis: & QGlyphRun) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QGlyphRun15setBoundingRectERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN9QGlyphRun15setBoundingRectERK6QRectF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QGlyphRun::overline();
impl /*struct*/ QGlyphRun {
  pub fn overline<RetType, T: QGlyphRun_overline<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.overline(self);
    // return 1;
  }
}

pub trait QGlyphRun_overline<RetType> {
  fn overline(self , rsthis: & QGlyphRun) -> RetType;
}

  // proto:  bool QGlyphRun::overline();
impl<'a> /*trait*/ QGlyphRun_overline<i8> for () {
  fn overline(self , rsthis: & QGlyphRun) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QGlyphRun8overlineEv()};
    let mut ret = unsafe {C_ZNK9QGlyphRun8overlineEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QGlyphRun::setRawData(const quint32 * glyphIndexArray, const QPointF * glyphPositionArray, int size);
impl /*struct*/ QGlyphRun {
  pub fn setRawData<RetType, T: QGlyphRun_setRawData<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setRawData(self);
    // return 1;
  }
}

pub trait QGlyphRun_setRawData<RetType> {
  fn setRawData(self , rsthis: & QGlyphRun) -> RetType;
}

  // proto:  void QGlyphRun::setRawData(const quint32 * glyphIndexArray, const QPointF * glyphPositionArray, int size);
impl<'a> /*trait*/ QGlyphRun_setRawData<()> for (&'a  Vec<u32>, &'a QPointF, i32) {
  fn setRawData(self , rsthis: & QGlyphRun) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QGlyphRun10setRawDataEPKjPK7QPointFi()};
    let arg0 = self.0.as_ptr()  as *mut c_uint;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
     unsafe {C_ZN9QGlyphRun10setRawDataEPKjPK7QPointFi(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QGlyphRun::setOverline(bool overline);
impl /*struct*/ QGlyphRun {
  pub fn setOverline<RetType, T: QGlyphRun_setOverline<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setOverline(self);
    // return 1;
  }
}

pub trait QGlyphRun_setOverline<RetType> {
  fn setOverline(self , rsthis: & QGlyphRun) -> RetType;
}

  // proto:  void QGlyphRun::setOverline(bool overline);
impl<'a> /*trait*/ QGlyphRun_setOverline<()> for (i8) {
  fn setOverline(self , rsthis: & QGlyphRun) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QGlyphRun11setOverlineEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN9QGlyphRun11setOverlineEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGlyphRun::swap(QGlyphRun & other);
impl /*struct*/ QGlyphRun {
  pub fn swap<RetType, T: QGlyphRun_swap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QGlyphRun_swap<RetType> {
  fn swap(self , rsthis: & QGlyphRun) -> RetType;
}

  // proto:  void QGlyphRun::swap(QGlyphRun & other);
impl<'a> /*trait*/ QGlyphRun_swap<()> for (&'a QGlyphRun) {
  fn swap(self , rsthis: & QGlyphRun) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QGlyphRun4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN9QGlyphRun4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGlyphRun::setUnderline(bool underline);
impl /*struct*/ QGlyphRun {
  pub fn setUnderline<RetType, T: QGlyphRun_setUnderline<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setUnderline(self);
    // return 1;
  }
}

pub trait QGlyphRun_setUnderline<RetType> {
  fn setUnderline(self , rsthis: & QGlyphRun) -> RetType;
}

  // proto:  void QGlyphRun::setUnderline(bool underline);
impl<'a> /*trait*/ QGlyphRun_setUnderline<()> for (i8) {
  fn setUnderline(self , rsthis: & QGlyphRun) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QGlyphRun12setUnderlineEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN9QGlyphRun12setUnderlineEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QVector<QPointF> QGlyphRun::positions();
impl /*struct*/ QGlyphRun {
  pub fn positions<RetType, T: QGlyphRun_positions<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.positions(self);
    // return 1;
  }
}

pub trait QGlyphRun_positions<RetType> {
  fn positions(self , rsthis: & QGlyphRun) -> RetType;
}

  // proto:  QVector<QPointF> QGlyphRun::positions();
impl<'a> /*trait*/ QGlyphRun_positions<u64> for () {
  fn positions(self , rsthis: & QGlyphRun) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QGlyphRun9positionsEv()};
    let mut ret = unsafe {C_ZNK9QGlyphRun9positionsEv(rsthis.qclsinst)};
    return ret as u64; // 5
    // return 1;
  }
}

  // proto:  void QGlyphRun::clear();
impl /*struct*/ QGlyphRun {
  pub fn clear<RetType, T: QGlyphRun_clear<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clear(self);
    // return 1;
  }
}

pub trait QGlyphRun_clear<RetType> {
  fn clear(self , rsthis: & QGlyphRun) -> RetType;
}

  // proto:  void QGlyphRun::clear();
impl<'a> /*trait*/ QGlyphRun_clear<()> for () {
  fn clear(self , rsthis: & QGlyphRun) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QGlyphRun5clearEv()};
     unsafe {C_ZN9QGlyphRun5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QGlyphRun::strikeOut();
impl /*struct*/ QGlyphRun {
  pub fn strikeOut<RetType, T: QGlyphRun_strikeOut<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.strikeOut(self);
    // return 1;
  }
}

pub trait QGlyphRun_strikeOut<RetType> {
  fn strikeOut(self , rsthis: & QGlyphRun) -> RetType;
}

  // proto:  bool QGlyphRun::strikeOut();
impl<'a> /*trait*/ QGlyphRun_strikeOut<i8> for () {
  fn strikeOut(self , rsthis: & QGlyphRun) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QGlyphRun9strikeOutEv()};
    let mut ret = unsafe {C_ZNK9QGlyphRun9strikeOutEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QGlyphRun::QGlyphRun();
impl /*struct*/ QGlyphRun {
  pub fn new<T: QGlyphRun_new>(value: T) -> QGlyphRun {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QGlyphRun_new {
  fn new(self) -> QGlyphRun;
}

  // proto:  void QGlyphRun::QGlyphRun();
impl<'a> /*trait*/ QGlyphRun_new for () {
  fn new(self) -> QGlyphRun {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QGlyphRunC2Ev()};
    let ctysz: c_int = unsafe{QGlyphRun_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN9QGlyphRunC2Ev()};
    let rsthis = QGlyphRun{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QRawFont QGlyphRun::rawFont();
impl /*struct*/ QGlyphRun {
  pub fn rawFont<RetType, T: QGlyphRun_rawFont<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rawFont(self);
    // return 1;
  }
}

pub trait QGlyphRun_rawFont<RetType> {
  fn rawFont(self , rsthis: & QGlyphRun) -> RetType;
}

  // proto:  QRawFont QGlyphRun::rawFont();
impl<'a> /*trait*/ QGlyphRun_rawFont<QRawFont> for () {
  fn rawFont(self , rsthis: & QGlyphRun) -> QRawFont {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QGlyphRun7rawFontEv()};
    let mut ret = unsafe {C_ZNK9QGlyphRun7rawFontEv(rsthis.qclsinst)};
    let mut ret1 = QRawFont::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGlyphRun::setRawFont(const QRawFont & rawFont);
impl /*struct*/ QGlyphRun {
  pub fn setRawFont<RetType, T: QGlyphRun_setRawFont<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setRawFont(self);
    // return 1;
  }
}

pub trait QGlyphRun_setRawFont<RetType> {
  fn setRawFont(self , rsthis: & QGlyphRun) -> RetType;
}

  // proto:  void QGlyphRun::setRawFont(const QRawFont & rawFont);
impl<'a> /*trait*/ QGlyphRun_setRawFont<()> for (&'a QRawFont) {
  fn setRawFont(self , rsthis: & QGlyphRun) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QGlyphRun10setRawFontERK8QRawFont()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN9QGlyphRun10setRawFontERK8QRawFont(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGlyphRun::QGlyphRun(const QGlyphRun & other);
impl<'a> /*trait*/ QGlyphRun_new for (&'a QGlyphRun) {
  fn new(self) -> QGlyphRun {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QGlyphRunC2ERKS_()};
    let ctysz: c_int = unsafe{QGlyphRun_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN9QGlyphRunC2ERKS_(arg0)};
    let rsthis = QGlyphRun{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QGlyphRun::isRightToLeft();
impl /*struct*/ QGlyphRun {
  pub fn isRightToLeft<RetType, T: QGlyphRun_isRightToLeft<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isRightToLeft(self);
    // return 1;
  }
}

pub trait QGlyphRun_isRightToLeft<RetType> {
  fn isRightToLeft(self , rsthis: & QGlyphRun) -> RetType;
}

  // proto:  bool QGlyphRun::isRightToLeft();
impl<'a> /*trait*/ QGlyphRun_isRightToLeft<i8> for () {
  fn isRightToLeft(self , rsthis: & QGlyphRun) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QGlyphRun13isRightToLeftEv()};
    let mut ret = unsafe {C_ZNK9QGlyphRun13isRightToLeftEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  QVector<quint32> QGlyphRun::glyphIndexes();
impl /*struct*/ QGlyphRun {
  pub fn glyphIndexes<RetType, T: QGlyphRun_glyphIndexes<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glyphIndexes(self);
    // return 1;
  }
}

pub trait QGlyphRun_glyphIndexes<RetType> {
  fn glyphIndexes(self , rsthis: & QGlyphRun) -> RetType;
}

  // proto:  QVector<quint32> QGlyphRun::glyphIndexes();
impl<'a> /*trait*/ QGlyphRun_glyphIndexes<u64> for () {
  fn glyphIndexes(self , rsthis: & QGlyphRun) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QGlyphRun12glyphIndexesEv()};
    let mut ret = unsafe {C_ZNK9QGlyphRun12glyphIndexesEv(rsthis.qclsinst)};
    return ret as u64; // 5
    // return 1;
  }
}

  // proto:  QRectF QGlyphRun::boundingRect();
impl /*struct*/ QGlyphRun {
  pub fn boundingRect<RetType, T: QGlyphRun_boundingRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.boundingRect(self);
    // return 1;
  }
}

pub trait QGlyphRun_boundingRect<RetType> {
  fn boundingRect(self , rsthis: & QGlyphRun) -> RetType;
}

  // proto:  QRectF QGlyphRun::boundingRect();
impl<'a> /*trait*/ QGlyphRun_boundingRect<QRectF> for () {
  fn boundingRect(self , rsthis: & QGlyphRun) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QGlyphRun12boundingRectEv()};
    let mut ret = unsafe {C_ZNK9QGlyphRun12boundingRectEv(rsthis.qclsinst)};
    let mut ret1 = QRectF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGlyphRun::setRightToLeft(bool on);
impl /*struct*/ QGlyphRun {
  pub fn setRightToLeft<RetType, T: QGlyphRun_setRightToLeft<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setRightToLeft(self);
    // return 1;
  }
}

pub trait QGlyphRun_setRightToLeft<RetType> {
  fn setRightToLeft(self , rsthis: & QGlyphRun) -> RetType;
}

  // proto:  void QGlyphRun::setRightToLeft(bool on);
impl<'a> /*trait*/ QGlyphRun_setRightToLeft<()> for (i8) {
  fn setRightToLeft(self , rsthis: & QGlyphRun) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QGlyphRun14setRightToLeftEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN9QGlyphRun14setRightToLeftEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QGlyphRun::underline();
impl /*struct*/ QGlyphRun {
  pub fn underline<RetType, T: QGlyphRun_underline<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.underline(self);
    // return 1;
  }
}

pub trait QGlyphRun_underline<RetType> {
  fn underline(self , rsthis: & QGlyphRun) -> RetType;
}

  // proto:  bool QGlyphRun::underline();
impl<'a> /*trait*/ QGlyphRun_underline<i8> for () {
  fn underline(self , rsthis: & QGlyphRun) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QGlyphRun9underlineEv()};
    let mut ret = unsafe {C_ZNK9QGlyphRun9underlineEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QGlyphRun::setStrikeOut(bool strikeOut);
impl /*struct*/ QGlyphRun {
  pub fn setStrikeOut<RetType, T: QGlyphRun_setStrikeOut<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setStrikeOut(self);
    // return 1;
  }
}

pub trait QGlyphRun_setStrikeOut<RetType> {
  fn setStrikeOut(self , rsthis: & QGlyphRun) -> RetType;
}

  // proto:  void QGlyphRun::setStrikeOut(bool strikeOut);
impl<'a> /*trait*/ QGlyphRun_setStrikeOut<()> for (i8) {
  fn setStrikeOut(self , rsthis: & QGlyphRun) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QGlyphRun12setStrikeOutEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN9QGlyphRun12setStrikeOutEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QGlyphRun::isEmpty();
impl /*struct*/ QGlyphRun {
  pub fn isEmpty<RetType, T: QGlyphRun_isEmpty<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isEmpty(self);
    // return 1;
  }
}

pub trait QGlyphRun_isEmpty<RetType> {
  fn isEmpty(self , rsthis: & QGlyphRun) -> RetType;
}

  // proto:  bool QGlyphRun::isEmpty();
impl<'a> /*trait*/ QGlyphRun_isEmpty<i8> for () {
  fn isEmpty(self , rsthis: & QGlyphRun) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QGlyphRun7isEmptyEv()};
    let mut ret = unsafe {C_ZNK9QGlyphRun7isEmptyEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

// <= body block end

