// auto generated, do not modify.
// created: Sat Dec 26 12:15:38 2015
// src-file: /QtGui/qtextlayout.h
// dst-file: /src/gui/qtextlayout.rs
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
use super::super::core::qrect::QRectF; // 771
use super::super::core::qpoint::QPointF; // 771
use super::qpainter::QPainter; // 773
use super::qfont::QFont; // 773
use super::super::core::qstring::QString; // 771
use super::qrawfont::QRawFont; // 773
use super::qtextoption::QTextOption; // 773
use super::qpaintdevice::QPaintDevice; // 773
// use super::qtextlayout::QTextLine; // 773
use super::qtextobject::QTextBlock; // 773
use super::qtextformat::QTextFormat; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QTextLine_Class_Size() -> c_int;
  // proto:  qreal QTextLine::ascent();
  fn _ZNK9QTextLine6ascentEv(qthis: *mut c_void) -> c_double;
  // proto:  qreal QTextLine::leading();
  fn _ZNK9QTextLine7leadingEv(qthis: *mut c_void) -> c_double;
  // proto:  int QTextLine::textStart();
  fn _ZNK9QTextLine9textStartEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QTextLine::leadingIncluded();
  fn _ZNK9QTextLine15leadingIncludedEv(qthis: *mut c_void) -> c_char;
  // proto:  qreal QTextLine::x();
  fn _ZNK9QTextLine1xEv(qthis: *mut c_void);
  // proto:  qreal QTextLine::height();
  fn _ZNK9QTextLine6heightEv(qthis: *mut c_void) -> c_double;
  // proto:  qreal QTextLine::y();
  fn _ZNK9QTextLine1yEv(qthis: *mut c_void);
  // proto:  qreal QTextLine::horizontalAdvance();
  fn _ZNK9QTextLine17horizontalAdvanceEv(qthis: *mut c_void) -> c_double;
  // proto:  QRectF QTextLine::naturalTextRect();
  fn _ZNK9QTextLine15naturalTextRectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTextLine::setNumColumns(int columns, qreal alignmentWidth);
  fn _ZN9QTextLine13setNumColumnsEid(qthis: *mut c_void, arg0: c_int, arg1: c_double);
  // proto:  qreal QTextLine::width();
  fn _ZNK9QTextLine5widthEv(qthis: *mut c_void) -> c_double;
  // proto:  void QTextLine::setLeadingIncluded(bool included);
  fn _ZN9QTextLine18setLeadingIncludedEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QTextLine::setPosition(const QPointF & pos);
  fn _ZN9QTextLine11setPositionERK7QPointF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  int QTextLine::lineNumber();
  fn _ZNK9QTextLine10lineNumberEv(qthis: *mut c_void) -> c_int;
  // proto:  QRectF QTextLine::rect();
  fn _ZNK9QTextLine4rectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTextLine::QTextLine();
  fn dector_ZN9QTextLineC1Ev() -> *mut c_void;
  fn demth_ZN9QTextLineC1Ev(qthis: *mut c_void);
  // proto:  void QTextLine::setNumColumns(int columns);
  fn _ZN9QTextLine13setNumColumnsEi(qthis: *mut c_void, arg0: c_int);
  // proto:  int QTextLine::textLength();
  fn _ZNK9QTextLine10textLengthEv(qthis: *mut c_void) -> c_int;
  // proto:  QPointF QTextLine::position();
  fn _ZNK9QTextLine8positionEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QList<QGlyphRun> QTextLine::glyphRuns(int from, int length);
  fn _ZNK9QTextLine9glyphRunsEii(qthis: *mut c_void, arg0: c_int, arg1: c_int);
  // proto:  qreal QTextLine::descent();
  fn _ZNK9QTextLine7descentEv(qthis: *mut c_void) -> c_double;
  // proto:  qreal QTextLine::naturalTextWidth();
  fn _ZNK9QTextLine16naturalTextWidthEv(qthis: *mut c_void) -> c_double;
  // proto:  void QTextLine::setLineWidth(qreal width);
  fn _ZN9QTextLine12setLineWidthEd(qthis: *mut c_void, arg0: c_double);
  // proto:  bool QTextLine::isValid();
  fn demth_ZNK9QTextLine7isValidEv(qthis: *mut c_void) -> c_char;
  fn QTextLayout_Class_Size() -> c_int;
  // proto:  void QTextLayout::setFont(const QFont & f);
  fn _ZN11QTextLayout7setFontERK5QFont(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QTextLayout::setText(const QString & string);
  fn _ZN11QTextLayout7setTextERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QTextLayout::isValidCursorPosition(int pos);
  fn _ZNK11QTextLayout21isValidCursorPositionEi(qthis: *mut c_void, arg0: c_int) -> c_char;
  // proto:  QRectF QTextLayout::boundingRect();
  fn _ZNK11QTextLayout12boundingRectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTextLayout::setRawFont(const QRawFont & rawFont);
  fn _ZN11QTextLayout10setRawFontERK8QRawFont(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QTextLayout::setTextOption(const QTextOption & option);
  fn _ZN11QTextLayout13setTextOptionERK11QTextOption(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QTextLayout::QTextLayout(const QString & text, const QFont & font, QPaintDevice * paintdevice);
  fn dector_ZN11QTextLayoutC1ERK7QStringRK5QFontP12QPaintDevice(arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) -> *mut c_void;
  fn _ZN11QTextLayoutC1ERK7QStringRK5QFontP12QPaintDevice(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  void QTextLayout::setPosition(const QPointF & p);
  fn _ZN11QTextLayout11setPositionERK7QPointF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QTextLine QTextLayout::lineForTextPosition(int pos);
  fn _ZNK11QTextLayout19lineForTextPositionEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  const QTextOption & QTextLayout::textOption();
  fn _ZNK11QTextLayout10textOptionEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QTextEngine * QTextLayout::engine();
  fn _ZNK11QTextLayout6engineEv(qthis: *mut c_void);
  // proto:  int QTextLayout::preeditAreaPosition();
  fn _ZNK11QTextLayout19preeditAreaPositionEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTextLayout::clearAdditionalFormats();
  fn _ZN11QTextLayout22clearAdditionalFormatsEv(qthis: *mut c_void);
  // proto:  int QTextLayout::leftCursorPosition(int oldPos);
  fn _ZNK11QTextLayout18leftCursorPositionEi(qthis: *mut c_void, arg0: c_int) -> c_int;
  // proto:  int QTextLayout::lineCount();
  fn _ZNK11QTextLayout9lineCountEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTextLayout::~QTextLayout();
  fn _ZN11QTextLayoutD0Ev(qthis: *mut c_void);
  // proto:  void QTextLayout::setCacheEnabled(bool enable);
  fn _ZN11QTextLayout15setCacheEnabledEb(qthis: *mut c_void, arg0: c_char);
  // proto:  QTextLine QTextLayout::lineAt(int i);
  fn _ZNK11QTextLayout6lineAtEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  int QTextLayout::rightCursorPosition(int oldPos);
  fn _ZNK11QTextLayout19rightCursorPositionEi(qthis: *mut c_void, arg0: c_int) -> c_int;
  // proto:  void QTextLayout::QTextLayout(const QTextBlock & b);
  fn dector_ZN11QTextLayoutC1ERK10QTextBlock(arg0: *mut c_void) -> *mut c_void;
  fn _ZN11QTextLayoutC1ERK10QTextBlock(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  qreal QTextLayout::minimumWidth();
  fn _ZNK11QTextLayout12minimumWidthEv(qthis: *mut c_void) -> c_double;
  // proto:  void QTextLayout::drawCursor(QPainter * p, const QPointF & pos, int cursorPosition);
  fn _ZNK11QTextLayout10drawCursorEP8QPainterRK7QPointFi(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: c_int);
  // proto:  QFont QTextLayout::font();
  fn _ZNK11QTextLayout4fontEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTextLayout::setPreeditArea(int position, const QString & text);
  fn _ZN11QTextLayout14setPreeditAreaEiRK7QString(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void);
  // proto:  void QTextLayout::beginLayout();
  fn _ZN11QTextLayout11beginLayoutEv(qthis: *mut c_void);
  // proto:  void QTextLayout::QTextLayout(const QString & text);
  fn dector_ZN11QTextLayoutC1ERK7QString(arg0: *mut c_void) -> *mut c_void;
  fn _ZN11QTextLayoutC1ERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QTextLayout::setFlags(int flags);
  fn _ZN11QTextLayout8setFlagsEi(qthis: *mut c_void, arg0: c_int);
  // proto:  QPointF QTextLayout::position();
  fn _ZNK11QTextLayout8positionEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTextLayout::clearLayout();
  fn _ZN11QTextLayout11clearLayoutEv(qthis: *mut c_void);
  // proto:  bool QTextLayout::cacheEnabled();
  fn _ZNK11QTextLayout12cacheEnabledEv(qthis: *mut c_void) -> c_char;
  // proto:  qreal QTextLayout::maximumWidth();
  fn _ZNK11QTextLayout12maximumWidthEv(qthis: *mut c_void) -> c_double;
  // proto:  QString QTextLayout::text();
  fn _ZNK11QTextLayout4textEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTextLayout::QTextLayout(const QTextLayout & );
  fn dector_ZN11QTextLayoutC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN11QTextLayoutC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QTextLine QTextLayout::createLine();
  fn _ZN11QTextLayout10createLineEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString QTextLayout::preeditAreaText();
  fn _ZNK11QTextLayout15preeditAreaTextEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTextLayout::drawCursor(QPainter * p, const QPointF & pos, int cursorPosition, int width);
  fn _ZNK11QTextLayout10drawCursorEP8QPainterRK7QPointFii(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: c_int, arg3: c_int);
  // proto:  void QTextLayout::endLayout();
  fn _ZN11QTextLayout9endLayoutEv(qthis: *mut c_void);
  // proto:  void QTextLayout::QTextLayout();
  fn dector_ZN11QTextLayoutC1Ev() -> *mut c_void;
  fn _ZN11QTextLayoutC1Ev(qthis: *mut c_void);
  // proto:  QList<QGlyphRun> QTextLayout::glyphRuns(int from, int length);
  fn _ZNK11QTextLayout9glyphRunsEii(qthis: *mut c_void, arg0: c_int, arg1: c_int);
  fn QTextInlineObject_Class_Size() -> c_int;
  // proto:  void QTextInlineObject::setAscent(qreal a);
  fn _ZN17QTextInlineObject9setAscentEd(qthis: *mut c_void, arg0: c_double);
  // proto:  qreal QTextInlineObject::width();
  fn _ZNK17QTextInlineObject5widthEv(qthis: *mut c_void) -> c_double;
  // proto:  int QTextInlineObject::formatIndex();
  fn _ZNK17QTextInlineObject11formatIndexEv(qthis: *mut c_void) -> c_int;
  // proto:  QRectF QTextInlineObject::rect();
  fn _ZNK17QTextInlineObject4rectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QTextInlineObject::textPosition();
  fn _ZNK17QTextInlineObject12textPositionEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTextInlineObject::setDescent(qreal d);
  fn _ZN17QTextInlineObject10setDescentEd(qthis: *mut c_void, arg0: c_double);
  // proto:  qreal QTextInlineObject::height();
  fn _ZNK17QTextInlineObject6heightEv(qthis: *mut c_void) -> c_double;
  // proto:  bool QTextInlineObject::isValid();
  fn demth_ZNK17QTextInlineObject7isValidEv(qthis: *mut c_void) -> c_char;
  // proto:  void QTextInlineObject::QTextInlineObject();
  fn dector_ZN17QTextInlineObjectC1Ev() -> *mut c_void;
  fn demth_ZN17QTextInlineObjectC1Ev(qthis: *mut c_void);
  // proto:  QTextFormat QTextInlineObject::format();
  fn _ZNK17QTextInlineObject6formatEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  qreal QTextInlineObject::descent();
  fn _ZNK17QTextInlineObject7descentEv(qthis: *mut c_void) -> c_double;
  // proto:  qreal QTextInlineObject::ascent();
  fn _ZNK17QTextInlineObject6ascentEv(qthis: *mut c_void) -> c_double;
  // proto:  void QTextInlineObject::setWidth(qreal w);
  fn _ZN17QTextInlineObject8setWidthEd(qthis: *mut c_void, arg0: c_double);
} // <= ext block end

// body block begin =>
// class sizeof(QTextLine)=16
pub struct QTextLine {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

// class sizeof(QTextLayout)=8
pub struct QTextLayout {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

// class sizeof(QTextInlineObject)=16
pub struct QTextInlineObject {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTextLine {
  pub fn inheritFrom(qthis: *mut c_void) -> QTextLine {
    return QTextLine{qclsinst: qthis};
  }
}
  // proto:  qreal QTextLine::ascent();
impl /*struct*/ QTextLine {
  pub fn ascent<RetType, T: QTextLine_ascent<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.ascent(self);
    // return 1;
  }
}

pub trait QTextLine_ascent<RetType> {
  fn ascent(self , rsthis: & QTextLine) -> RetType;
}

  // proto:  qreal QTextLine::ascent();
impl<'a> /*trait*/ QTextLine_ascent<f64> for () {
  fn ascent(self , rsthis: & QTextLine) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine6ascentEv()};
    let mut ret = unsafe {_ZNK9QTextLine6ascentEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  qreal QTextLine::leading();
impl /*struct*/ QTextLine {
  pub fn leading<RetType, T: QTextLine_leading<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.leading(self);
    // return 1;
  }
}

pub trait QTextLine_leading<RetType> {
  fn leading(self , rsthis: & QTextLine) -> RetType;
}

  // proto:  qreal QTextLine::leading();
impl<'a> /*trait*/ QTextLine_leading<f64> for () {
  fn leading(self , rsthis: & QTextLine) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine7leadingEv()};
    let mut ret = unsafe {_ZNK9QTextLine7leadingEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  int QTextLine::textStart();
impl /*struct*/ QTextLine {
  pub fn textStart<RetType, T: QTextLine_textStart<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.textStart(self);
    // return 1;
  }
}

pub trait QTextLine_textStart<RetType> {
  fn textStart(self , rsthis: & QTextLine) -> RetType;
}

  // proto:  int QTextLine::textStart();
impl<'a> /*trait*/ QTextLine_textStart<i32> for () {
  fn textStart(self , rsthis: & QTextLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine9textStartEv()};
    let mut ret = unsafe {_ZNK9QTextLine9textStartEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QTextLine::leadingIncluded();
impl /*struct*/ QTextLine {
  pub fn leadingIncluded<RetType, T: QTextLine_leadingIncluded<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.leadingIncluded(self);
    // return 1;
  }
}

pub trait QTextLine_leadingIncluded<RetType> {
  fn leadingIncluded(self , rsthis: & QTextLine) -> RetType;
}

  // proto:  bool QTextLine::leadingIncluded();
impl<'a> /*trait*/ QTextLine_leadingIncluded<i8> for () {
  fn leadingIncluded(self , rsthis: & QTextLine) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine15leadingIncludedEv()};
    let mut ret = unsafe {_ZNK9QTextLine15leadingIncludedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  qreal QTextLine::x();
impl /*struct*/ QTextLine {
  pub fn x<RetType, T: QTextLine_x<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.x(self);
    // return 1;
  }
}

pub trait QTextLine_x<RetType> {
  fn x(self , rsthis: & QTextLine) -> RetType;
}

  // proto:  qreal QTextLine::x();
impl<'a> /*trait*/ QTextLine_x<()> for () {
  fn x(self , rsthis: & QTextLine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine1xEv()};
     unsafe {_ZNK9QTextLine1xEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  qreal QTextLine::height();
impl /*struct*/ QTextLine {
  pub fn height<RetType, T: QTextLine_height<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.height(self);
    // return 1;
  }
}

pub trait QTextLine_height<RetType> {
  fn height(self , rsthis: & QTextLine) -> RetType;
}

  // proto:  qreal QTextLine::height();
impl<'a> /*trait*/ QTextLine_height<f64> for () {
  fn height(self , rsthis: & QTextLine) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine6heightEv()};
    let mut ret = unsafe {_ZNK9QTextLine6heightEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  qreal QTextLine::y();
impl /*struct*/ QTextLine {
  pub fn y<RetType, T: QTextLine_y<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.y(self);
    // return 1;
  }
}

pub trait QTextLine_y<RetType> {
  fn y(self , rsthis: & QTextLine) -> RetType;
}

  // proto:  qreal QTextLine::y();
impl<'a> /*trait*/ QTextLine_y<()> for () {
  fn y(self , rsthis: & QTextLine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine1yEv()};
     unsafe {_ZNK9QTextLine1yEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  qreal QTextLine::horizontalAdvance();
impl /*struct*/ QTextLine {
  pub fn horizontalAdvance<RetType, T: QTextLine_horizontalAdvance<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.horizontalAdvance(self);
    // return 1;
  }
}

pub trait QTextLine_horizontalAdvance<RetType> {
  fn horizontalAdvance(self , rsthis: & QTextLine) -> RetType;
}

  // proto:  qreal QTextLine::horizontalAdvance();
impl<'a> /*trait*/ QTextLine_horizontalAdvance<f64> for () {
  fn horizontalAdvance(self , rsthis: & QTextLine) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine17horizontalAdvanceEv()};
    let mut ret = unsafe {_ZNK9QTextLine17horizontalAdvanceEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  QRectF QTextLine::naturalTextRect();
impl /*struct*/ QTextLine {
  pub fn naturalTextRect<RetType, T: QTextLine_naturalTextRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.naturalTextRect(self);
    // return 1;
  }
}

pub trait QTextLine_naturalTextRect<RetType> {
  fn naturalTextRect(self , rsthis: & QTextLine) -> RetType;
}

  // proto:  QRectF QTextLine::naturalTextRect();
impl<'a> /*trait*/ QTextLine_naturalTextRect<QRectF> for () {
  fn naturalTextRect(self , rsthis: & QTextLine) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine15naturalTextRectEv()};
    let mut ret = unsafe {_ZNK9QTextLine15naturalTextRectEv(rsthis.qclsinst)};
    let mut ret1 = QRectF::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextLine::setNumColumns(int columns, qreal alignmentWidth);
impl /*struct*/ QTextLine {
  pub fn setNumColumns<RetType, T: QTextLine_setNumColumns<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setNumColumns(self);
    // return 1;
  }
}

pub trait QTextLine_setNumColumns<RetType> {
  fn setNumColumns(self , rsthis: & QTextLine) -> RetType;
}

  // proto:  void QTextLine::setNumColumns(int columns, qreal alignmentWidth);
impl<'a> /*trait*/ QTextLine_setNumColumns<()> for (i32, f64) {
  fn setNumColumns(self , rsthis: & QTextLine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextLine13setNumColumnsEid()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_double;
     unsafe {_ZN9QTextLine13setNumColumnsEid(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  qreal QTextLine::width();
impl /*struct*/ QTextLine {
  pub fn width<RetType, T: QTextLine_width<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.width(self);
    // return 1;
  }
}

pub trait QTextLine_width<RetType> {
  fn width(self , rsthis: & QTextLine) -> RetType;
}

  // proto:  qreal QTextLine::width();
impl<'a> /*trait*/ QTextLine_width<f64> for () {
  fn width(self , rsthis: & QTextLine) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine5widthEv()};
    let mut ret = unsafe {_ZNK9QTextLine5widthEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QTextLine::setLeadingIncluded(bool included);
impl /*struct*/ QTextLine {
  pub fn setLeadingIncluded<RetType, T: QTextLine_setLeadingIncluded<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setLeadingIncluded(self);
    // return 1;
  }
}

pub trait QTextLine_setLeadingIncluded<RetType> {
  fn setLeadingIncluded(self , rsthis: & QTextLine) -> RetType;
}

  // proto:  void QTextLine::setLeadingIncluded(bool included);
impl<'a> /*trait*/ QTextLine_setLeadingIncluded<()> for (i8) {
  fn setLeadingIncluded(self , rsthis: & QTextLine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextLine18setLeadingIncludedEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN9QTextLine18setLeadingIncludedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextLine::setPosition(const QPointF & pos);
impl /*struct*/ QTextLine {
  pub fn setPosition<RetType, T: QTextLine_setPosition<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPosition(self);
    // return 1;
  }
}

pub trait QTextLine_setPosition<RetType> {
  fn setPosition(self , rsthis: & QTextLine) -> RetType;
}

  // proto:  void QTextLine::setPosition(const QPointF & pos);
impl<'a> /*trait*/ QTextLine_setPosition<()> for (&'a QPointF) {
  fn setPosition(self , rsthis: & QTextLine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextLine11setPositionERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QTextLine11setPositionERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QTextLine::lineNumber();
impl /*struct*/ QTextLine {
  pub fn lineNumber<RetType, T: QTextLine_lineNumber<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.lineNumber(self);
    // return 1;
  }
}

pub trait QTextLine_lineNumber<RetType> {
  fn lineNumber(self , rsthis: & QTextLine) -> RetType;
}

  // proto:  int QTextLine::lineNumber();
impl<'a> /*trait*/ QTextLine_lineNumber<i32> for () {
  fn lineNumber(self , rsthis: & QTextLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine10lineNumberEv()};
    let mut ret = unsafe {_ZNK9QTextLine10lineNumberEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QRectF QTextLine::rect();
impl /*struct*/ QTextLine {
  pub fn rect<RetType, T: QTextLine_rect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rect(self);
    // return 1;
  }
}

pub trait QTextLine_rect<RetType> {
  fn rect(self , rsthis: & QTextLine) -> RetType;
}

  // proto:  QRectF QTextLine::rect();
impl<'a> /*trait*/ QTextLine_rect<QRectF> for () {
  fn rect(self , rsthis: & QTextLine) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine4rectEv()};
    let mut ret = unsafe {_ZNK9QTextLine4rectEv(rsthis.qclsinst)};
    let mut ret1 = QRectF::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextLine::QTextLine();
impl /*struct*/ QTextLine {
  pub fn New<T: QTextLine_New>(value: T) -> QTextLine {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QTextLine_New {
  fn New(self) -> QTextLine;
}

  // proto:  void QTextLine::QTextLine();
impl<'a> /*trait*/ QTextLine_New for () {
  fn New(self) -> QTextLine {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextLineC1Ev()};
    let ctysz: c_int = unsafe{QTextLine_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    // unsafe {_ZN9QTextLineC1Ev(qthis)};
    let qthis: *mut c_void = unsafe {dector_ZN9QTextLineC1Ev()};
    let rsthis = QTextLine{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QTextLine::setNumColumns(int columns);
impl<'a> /*trait*/ QTextLine_setNumColumns<()> for (i32) {
  fn setNumColumns(self , rsthis: & QTextLine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextLine13setNumColumnsEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QTextLine13setNumColumnsEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QTextLine::textLength();
impl /*struct*/ QTextLine {
  pub fn textLength<RetType, T: QTextLine_textLength<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.textLength(self);
    // return 1;
  }
}

pub trait QTextLine_textLength<RetType> {
  fn textLength(self , rsthis: & QTextLine) -> RetType;
}

  // proto:  int QTextLine::textLength();
impl<'a> /*trait*/ QTextLine_textLength<i32> for () {
  fn textLength(self , rsthis: & QTextLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine10textLengthEv()};
    let mut ret = unsafe {_ZNK9QTextLine10textLengthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QPointF QTextLine::position();
impl /*struct*/ QTextLine {
  pub fn position<RetType, T: QTextLine_position<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.position(self);
    // return 1;
  }
}

pub trait QTextLine_position<RetType> {
  fn position(self , rsthis: & QTextLine) -> RetType;
}

  // proto:  QPointF QTextLine::position();
impl<'a> /*trait*/ QTextLine_position<QPointF> for () {
  fn position(self , rsthis: & QTextLine) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine8positionEv()};
    let mut ret = unsafe {_ZNK9QTextLine8positionEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QList<QGlyphRun> QTextLine::glyphRuns(int from, int length);
impl /*struct*/ QTextLine {
  pub fn glyphRuns<RetType, T: QTextLine_glyphRuns<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glyphRuns(self);
    // return 1;
  }
}

pub trait QTextLine_glyphRuns<RetType> {
  fn glyphRuns(self , rsthis: & QTextLine) -> RetType;
}

  // proto:  QList<QGlyphRun> QTextLine::glyphRuns(int from, int length);
impl<'a> /*trait*/ QTextLine_glyphRuns<()> for (i32, i32) {
  fn glyphRuns(self , rsthis: & QTextLine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine9glyphRunsEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZNK9QTextLine9glyphRunsEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  qreal QTextLine::descent();
impl /*struct*/ QTextLine {
  pub fn descent<RetType, T: QTextLine_descent<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.descent(self);
    // return 1;
  }
}

pub trait QTextLine_descent<RetType> {
  fn descent(self , rsthis: & QTextLine) -> RetType;
}

  // proto:  qreal QTextLine::descent();
impl<'a> /*trait*/ QTextLine_descent<f64> for () {
  fn descent(self , rsthis: & QTextLine) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine7descentEv()};
    let mut ret = unsafe {_ZNK9QTextLine7descentEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  qreal QTextLine::naturalTextWidth();
impl /*struct*/ QTextLine {
  pub fn naturalTextWidth<RetType, T: QTextLine_naturalTextWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.naturalTextWidth(self);
    // return 1;
  }
}

pub trait QTextLine_naturalTextWidth<RetType> {
  fn naturalTextWidth(self , rsthis: & QTextLine) -> RetType;
}

  // proto:  qreal QTextLine::naturalTextWidth();
impl<'a> /*trait*/ QTextLine_naturalTextWidth<f64> for () {
  fn naturalTextWidth(self , rsthis: & QTextLine) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine16naturalTextWidthEv()};
    let mut ret = unsafe {_ZNK9QTextLine16naturalTextWidthEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QTextLine::setLineWidth(qreal width);
impl /*struct*/ QTextLine {
  pub fn setLineWidth<RetType, T: QTextLine_setLineWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setLineWidth(self);
    // return 1;
  }
}

pub trait QTextLine_setLineWidth<RetType> {
  fn setLineWidth(self , rsthis: & QTextLine) -> RetType;
}

  // proto:  void QTextLine::setLineWidth(qreal width);
impl<'a> /*trait*/ QTextLine_setLineWidth<()> for (f64) {
  fn setLineWidth(self , rsthis: & QTextLine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTextLine12setLineWidthEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN9QTextLine12setLineWidthEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QTextLine::isValid();
impl /*struct*/ QTextLine {
  pub fn isValid<RetType, T: QTextLine_isValid<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QTextLine_isValid<RetType> {
  fn isValid(self , rsthis: & QTextLine) -> RetType;
}

  // proto:  bool QTextLine::isValid();
impl<'a> /*trait*/ QTextLine_isValid<i8> for () {
  fn isValid(self , rsthis: & QTextLine) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTextLine7isValidEv()};
    let mut ret = unsafe {demth_ZNK9QTextLine7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QTextLayout {
  pub fn inheritFrom(qthis: *mut c_void) -> QTextLayout {
    return QTextLayout{qclsinst: qthis};
  }
}
  // proto:  void QTextLayout::setFont(const QFont & f);
impl /*struct*/ QTextLayout {
  pub fn setFont<RetType, T: QTextLayout_setFont<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFont(self);
    // return 1;
  }
}

pub trait QTextLayout_setFont<RetType> {
  fn setFont(self , rsthis: & QTextLayout) -> RetType;
}

  // proto:  void QTextLayout::setFont(const QFont & f);
impl<'a> /*trait*/ QTextLayout_setFont<()> for (&'a QFont) {
  fn setFont(self , rsthis: & QTextLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextLayout7setFontERK5QFont()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTextLayout7setFontERK5QFont(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextLayout::setText(const QString & string);
impl /*struct*/ QTextLayout {
  pub fn setText<RetType, T: QTextLayout_setText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setText(self);
    // return 1;
  }
}

pub trait QTextLayout_setText<RetType> {
  fn setText(self , rsthis: & QTextLayout) -> RetType;
}

  // proto:  void QTextLayout::setText(const QString & string);
impl<'a> /*trait*/ QTextLayout_setText<()> for (&'a QString) {
  fn setText(self , rsthis: & QTextLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextLayout7setTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTextLayout7setTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QTextLayout::isValidCursorPosition(int pos);
impl /*struct*/ QTextLayout {
  pub fn isValidCursorPosition<RetType, T: QTextLayout_isValidCursorPosition<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isValidCursorPosition(self);
    // return 1;
  }
}

pub trait QTextLayout_isValidCursorPosition<RetType> {
  fn isValidCursorPosition(self , rsthis: & QTextLayout) -> RetType;
}

  // proto:  bool QTextLayout::isValidCursorPosition(int pos);
impl<'a> /*trait*/ QTextLayout_isValidCursorPosition<i8> for (i32) {
  fn isValidCursorPosition(self , rsthis: & QTextLayout) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextLayout21isValidCursorPositionEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QTextLayout21isValidCursorPositionEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QRectF QTextLayout::boundingRect();
impl /*struct*/ QTextLayout {
  pub fn boundingRect<RetType, T: QTextLayout_boundingRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.boundingRect(self);
    // return 1;
  }
}

pub trait QTextLayout_boundingRect<RetType> {
  fn boundingRect(self , rsthis: & QTextLayout) -> RetType;
}

  // proto:  QRectF QTextLayout::boundingRect();
impl<'a> /*trait*/ QTextLayout_boundingRect<QRectF> for () {
  fn boundingRect(self , rsthis: & QTextLayout) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextLayout12boundingRectEv()};
    let mut ret = unsafe {_ZNK11QTextLayout12boundingRectEv(rsthis.qclsinst)};
    let mut ret1 = QRectF::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextLayout::setRawFont(const QRawFont & rawFont);
impl /*struct*/ QTextLayout {
  pub fn setRawFont<RetType, T: QTextLayout_setRawFont<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setRawFont(self);
    // return 1;
  }
}

pub trait QTextLayout_setRawFont<RetType> {
  fn setRawFont(self , rsthis: & QTextLayout) -> RetType;
}

  // proto:  void QTextLayout::setRawFont(const QRawFont & rawFont);
impl<'a> /*trait*/ QTextLayout_setRawFont<()> for (&'a QRawFont) {
  fn setRawFont(self , rsthis: & QTextLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextLayout10setRawFontERK8QRawFont()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTextLayout10setRawFontERK8QRawFont(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextLayout::setTextOption(const QTextOption & option);
impl /*struct*/ QTextLayout {
  pub fn setTextOption<RetType, T: QTextLayout_setTextOption<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTextOption(self);
    // return 1;
  }
}

pub trait QTextLayout_setTextOption<RetType> {
  fn setTextOption(self , rsthis: & QTextLayout) -> RetType;
}

  // proto:  void QTextLayout::setTextOption(const QTextOption & option);
impl<'a> /*trait*/ QTextLayout_setTextOption<()> for (&'a QTextOption) {
  fn setTextOption(self , rsthis: & QTextLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextLayout13setTextOptionERK11QTextOption()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTextLayout13setTextOptionERK11QTextOption(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextLayout::QTextLayout(const QString & text, const QFont & font, QPaintDevice * paintdevice);
impl /*struct*/ QTextLayout {
  pub fn New<T: QTextLayout_New>(value: T) -> QTextLayout {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QTextLayout_New {
  fn New(self) -> QTextLayout;
}

  // proto:  void QTextLayout::QTextLayout(const QString & text, const QFont & font, QPaintDevice * paintdevice);
impl<'a> /*trait*/ QTextLayout_New for (&'a QString, &'a QFont, &'a QPaintDevice) {
  fn New(self) -> QTextLayout {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextLayoutC1ERK7QStringRK5QFontP12QPaintDevice()};
    let ctysz: c_int = unsafe{QTextLayout_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    // unsafe {_ZN11QTextLayoutC1ERK7QStringRK5QFontP12QPaintDevice(qthis, arg0, arg1, arg2)};
    let qthis: *mut c_void = unsafe {dector_ZN11QTextLayoutC1ERK7QStringRK5QFontP12QPaintDevice(arg0, arg1, arg2)};
    let rsthis = QTextLayout{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QTextLayout::setPosition(const QPointF & p);
impl /*struct*/ QTextLayout {
  pub fn setPosition<RetType, T: QTextLayout_setPosition<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPosition(self);
    // return 1;
  }
}

pub trait QTextLayout_setPosition<RetType> {
  fn setPosition(self , rsthis: & QTextLayout) -> RetType;
}

  // proto:  void QTextLayout::setPosition(const QPointF & p);
impl<'a> /*trait*/ QTextLayout_setPosition<()> for (&'a QPointF) {
  fn setPosition(self , rsthis: & QTextLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextLayout11setPositionERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QTextLayout11setPositionERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QTextLine QTextLayout::lineForTextPosition(int pos);
impl /*struct*/ QTextLayout {
  pub fn lineForTextPosition<RetType, T: QTextLayout_lineForTextPosition<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.lineForTextPosition(self);
    // return 1;
  }
}

pub trait QTextLayout_lineForTextPosition<RetType> {
  fn lineForTextPosition(self , rsthis: & QTextLayout) -> RetType;
}

  // proto:  QTextLine QTextLayout::lineForTextPosition(int pos);
impl<'a> /*trait*/ QTextLayout_lineForTextPosition<QTextLine> for (i32) {
  fn lineForTextPosition(self , rsthis: & QTextLayout) -> QTextLine {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextLayout19lineForTextPositionEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QTextLayout19lineForTextPositionEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QTextLine::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  const QTextOption & QTextLayout::textOption();
impl /*struct*/ QTextLayout {
  pub fn textOption<RetType, T: QTextLayout_textOption<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.textOption(self);
    // return 1;
  }
}

pub trait QTextLayout_textOption<RetType> {
  fn textOption(self , rsthis: & QTextLayout) -> RetType;
}

  // proto:  const QTextOption & QTextLayout::textOption();
impl<'a> /*trait*/ QTextLayout_textOption<QTextOption> for () {
  fn textOption(self , rsthis: & QTextLayout) -> QTextOption {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextLayout10textOptionEv()};
    let mut ret = unsafe {_ZNK11QTextLayout10textOptionEv(rsthis.qclsinst)};
    let mut ret1 = QTextOption::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QTextEngine * QTextLayout::engine();
impl /*struct*/ QTextLayout {
  pub fn engine<RetType, T: QTextLayout_engine<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.engine(self);
    // return 1;
  }
}

pub trait QTextLayout_engine<RetType> {
  fn engine(self , rsthis: & QTextLayout) -> RetType;
}

  // proto:  QTextEngine * QTextLayout::engine();
impl<'a> /*trait*/ QTextLayout_engine<()> for () {
  fn engine(self , rsthis: & QTextLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextLayout6engineEv()};
     unsafe {_ZNK11QTextLayout6engineEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QTextLayout::preeditAreaPosition();
impl /*struct*/ QTextLayout {
  pub fn preeditAreaPosition<RetType, T: QTextLayout_preeditAreaPosition<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.preeditAreaPosition(self);
    // return 1;
  }
}

pub trait QTextLayout_preeditAreaPosition<RetType> {
  fn preeditAreaPosition(self , rsthis: & QTextLayout) -> RetType;
}

  // proto:  int QTextLayout::preeditAreaPosition();
impl<'a> /*trait*/ QTextLayout_preeditAreaPosition<i32> for () {
  fn preeditAreaPosition(self , rsthis: & QTextLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextLayout19preeditAreaPositionEv()};
    let mut ret = unsafe {_ZNK11QTextLayout19preeditAreaPositionEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QTextLayout::clearAdditionalFormats();
impl /*struct*/ QTextLayout {
  pub fn clearAdditionalFormats<RetType, T: QTextLayout_clearAdditionalFormats<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clearAdditionalFormats(self);
    // return 1;
  }
}

pub trait QTextLayout_clearAdditionalFormats<RetType> {
  fn clearAdditionalFormats(self , rsthis: & QTextLayout) -> RetType;
}

  // proto:  void QTextLayout::clearAdditionalFormats();
impl<'a> /*trait*/ QTextLayout_clearAdditionalFormats<()> for () {
  fn clearAdditionalFormats(self , rsthis: & QTextLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextLayout22clearAdditionalFormatsEv()};
     unsafe {_ZN11QTextLayout22clearAdditionalFormatsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QTextLayout::leftCursorPosition(int oldPos);
impl /*struct*/ QTextLayout {
  pub fn leftCursorPosition<RetType, T: QTextLayout_leftCursorPosition<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.leftCursorPosition(self);
    // return 1;
  }
}

pub trait QTextLayout_leftCursorPosition<RetType> {
  fn leftCursorPosition(self , rsthis: & QTextLayout) -> RetType;
}

  // proto:  int QTextLayout::leftCursorPosition(int oldPos);
impl<'a> /*trait*/ QTextLayout_leftCursorPosition<i32> for (i32) {
  fn leftCursorPosition(self , rsthis: & QTextLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextLayout18leftCursorPositionEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QTextLayout18leftCursorPositionEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QTextLayout::lineCount();
impl /*struct*/ QTextLayout {
  pub fn lineCount<RetType, T: QTextLayout_lineCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.lineCount(self);
    // return 1;
  }
}

pub trait QTextLayout_lineCount<RetType> {
  fn lineCount(self , rsthis: & QTextLayout) -> RetType;
}

  // proto:  int QTextLayout::lineCount();
impl<'a> /*trait*/ QTextLayout_lineCount<i32> for () {
  fn lineCount(self , rsthis: & QTextLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextLayout9lineCountEv()};
    let mut ret = unsafe {_ZNK11QTextLayout9lineCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QTextLayout::~QTextLayout();
impl /*struct*/ QTextLayout {
  pub fn Free<RetType, T: QTextLayout_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QTextLayout_Free<RetType> {
  fn Free(self , rsthis: & QTextLayout) -> RetType;
}

  // proto:  void QTextLayout::~QTextLayout();
impl<'a> /*trait*/ QTextLayout_Free<()> for () {
  fn Free(self , rsthis: & QTextLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextLayoutD0Ev()};
     unsafe {_ZN11QTextLayoutD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTextLayout::setCacheEnabled(bool enable);
impl /*struct*/ QTextLayout {
  pub fn setCacheEnabled<RetType, T: QTextLayout_setCacheEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCacheEnabled(self);
    // return 1;
  }
}

pub trait QTextLayout_setCacheEnabled<RetType> {
  fn setCacheEnabled(self , rsthis: & QTextLayout) -> RetType;
}

  // proto:  void QTextLayout::setCacheEnabled(bool enable);
impl<'a> /*trait*/ QTextLayout_setCacheEnabled<()> for (i8) {
  fn setCacheEnabled(self , rsthis: & QTextLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextLayout15setCacheEnabledEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN11QTextLayout15setCacheEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QTextLine QTextLayout::lineAt(int i);
impl /*struct*/ QTextLayout {
  pub fn lineAt<RetType, T: QTextLayout_lineAt<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.lineAt(self);
    // return 1;
  }
}

pub trait QTextLayout_lineAt<RetType> {
  fn lineAt(self , rsthis: & QTextLayout) -> RetType;
}

  // proto:  QTextLine QTextLayout::lineAt(int i);
impl<'a> /*trait*/ QTextLayout_lineAt<QTextLine> for (i32) {
  fn lineAt(self , rsthis: & QTextLayout) -> QTextLine {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextLayout6lineAtEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QTextLayout6lineAtEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QTextLine::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  int QTextLayout::rightCursorPosition(int oldPos);
impl /*struct*/ QTextLayout {
  pub fn rightCursorPosition<RetType, T: QTextLayout_rightCursorPosition<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rightCursorPosition(self);
    // return 1;
  }
}

pub trait QTextLayout_rightCursorPosition<RetType> {
  fn rightCursorPosition(self , rsthis: & QTextLayout) -> RetType;
}

  // proto:  int QTextLayout::rightCursorPosition(int oldPos);
impl<'a> /*trait*/ QTextLayout_rightCursorPosition<i32> for (i32) {
  fn rightCursorPosition(self , rsthis: & QTextLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextLayout19rightCursorPositionEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK11QTextLayout19rightCursorPositionEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QTextLayout::QTextLayout(const QTextBlock & b);
impl<'a> /*trait*/ QTextLayout_New for (&'a QTextBlock) {
  fn New(self) -> QTextLayout {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextLayoutC1ERK10QTextBlock()};
    let ctysz: c_int = unsafe{QTextLayout_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN11QTextLayoutC1ERK10QTextBlock(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN11QTextLayoutC1ERK10QTextBlock(arg0)};
    let rsthis = QTextLayout{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  qreal QTextLayout::minimumWidth();
impl /*struct*/ QTextLayout {
  pub fn minimumWidth<RetType, T: QTextLayout_minimumWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.minimumWidth(self);
    // return 1;
  }
}

pub trait QTextLayout_minimumWidth<RetType> {
  fn minimumWidth(self , rsthis: & QTextLayout) -> RetType;
}

  // proto:  qreal QTextLayout::minimumWidth();
impl<'a> /*trait*/ QTextLayout_minimumWidth<f64> for () {
  fn minimumWidth(self , rsthis: & QTextLayout) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextLayout12minimumWidthEv()};
    let mut ret = unsafe {_ZNK11QTextLayout12minimumWidthEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QTextLayout::drawCursor(QPainter * p, const QPointF & pos, int cursorPosition);
impl /*struct*/ QTextLayout {
  pub fn drawCursor<RetType, T: QTextLayout_drawCursor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.drawCursor(self);
    // return 1;
  }
}

pub trait QTextLayout_drawCursor<RetType> {
  fn drawCursor(self , rsthis: & QTextLayout) -> RetType;
}

  // proto:  void QTextLayout::drawCursor(QPainter * p, const QPointF & pos, int cursorPosition);
impl<'a> /*trait*/ QTextLayout_drawCursor<()> for (&'a QPainter, &'a QPointF, i32) {
  fn drawCursor(self , rsthis: & QTextLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextLayout10drawCursorEP8QPainterRK7QPointFi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
     unsafe {_ZNK11QTextLayout10drawCursorEP8QPainterRK7QPointFi(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  QFont QTextLayout::font();
impl /*struct*/ QTextLayout {
  pub fn font<RetType, T: QTextLayout_font<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.font(self);
    // return 1;
  }
}

pub trait QTextLayout_font<RetType> {
  fn font(self , rsthis: & QTextLayout) -> RetType;
}

  // proto:  QFont QTextLayout::font();
impl<'a> /*trait*/ QTextLayout_font<QFont> for () {
  fn font(self , rsthis: & QTextLayout) -> QFont {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextLayout4fontEv()};
    let mut ret = unsafe {_ZNK11QTextLayout4fontEv(rsthis.qclsinst)};
    let mut ret1 = QFont::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextLayout::setPreeditArea(int position, const QString & text);
impl /*struct*/ QTextLayout {
  pub fn setPreeditArea<RetType, T: QTextLayout_setPreeditArea<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPreeditArea(self);
    // return 1;
  }
}

pub trait QTextLayout_setPreeditArea<RetType> {
  fn setPreeditArea(self , rsthis: & QTextLayout) -> RetType;
}

  // proto:  void QTextLayout::setPreeditArea(int position, const QString & text);
impl<'a> /*trait*/ QTextLayout_setPreeditArea<()> for (i32, &'a QString) {
  fn setPreeditArea(self , rsthis: & QTextLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextLayout14setPreeditAreaEiRK7QString()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN11QTextLayout14setPreeditAreaEiRK7QString(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QTextLayout::beginLayout();
impl /*struct*/ QTextLayout {
  pub fn beginLayout<RetType, T: QTextLayout_beginLayout<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.beginLayout(self);
    // return 1;
  }
}

pub trait QTextLayout_beginLayout<RetType> {
  fn beginLayout(self , rsthis: & QTextLayout) -> RetType;
}

  // proto:  void QTextLayout::beginLayout();
impl<'a> /*trait*/ QTextLayout_beginLayout<()> for () {
  fn beginLayout(self , rsthis: & QTextLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextLayout11beginLayoutEv()};
     unsafe {_ZN11QTextLayout11beginLayoutEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTextLayout::QTextLayout(const QString & text);
impl<'a> /*trait*/ QTextLayout_New for (&'a QString) {
  fn New(self) -> QTextLayout {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextLayoutC1ERK7QString()};
    let ctysz: c_int = unsafe{QTextLayout_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN11QTextLayoutC1ERK7QString(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN11QTextLayoutC1ERK7QString(arg0)};
    let rsthis = QTextLayout{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QTextLayout::setFlags(int flags);
impl /*struct*/ QTextLayout {
  pub fn setFlags<RetType, T: QTextLayout_setFlags<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFlags(self);
    // return 1;
  }
}

pub trait QTextLayout_setFlags<RetType> {
  fn setFlags(self , rsthis: & QTextLayout) -> RetType;
}

  // proto:  void QTextLayout::setFlags(int flags);
impl<'a> /*trait*/ QTextLayout_setFlags<()> for (i32) {
  fn setFlags(self , rsthis: & QTextLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextLayout8setFlagsEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN11QTextLayout8setFlagsEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QPointF QTextLayout::position();
impl /*struct*/ QTextLayout {
  pub fn position<RetType, T: QTextLayout_position<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.position(self);
    // return 1;
  }
}

pub trait QTextLayout_position<RetType> {
  fn position(self , rsthis: & QTextLayout) -> RetType;
}

  // proto:  QPointF QTextLayout::position();
impl<'a> /*trait*/ QTextLayout_position<QPointF> for () {
  fn position(self , rsthis: & QTextLayout) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextLayout8positionEv()};
    let mut ret = unsafe {_ZNK11QTextLayout8positionEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextLayout::clearLayout();
impl /*struct*/ QTextLayout {
  pub fn clearLayout<RetType, T: QTextLayout_clearLayout<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clearLayout(self);
    // return 1;
  }
}

pub trait QTextLayout_clearLayout<RetType> {
  fn clearLayout(self , rsthis: & QTextLayout) -> RetType;
}

  // proto:  void QTextLayout::clearLayout();
impl<'a> /*trait*/ QTextLayout_clearLayout<()> for () {
  fn clearLayout(self , rsthis: & QTextLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextLayout11clearLayoutEv()};
     unsafe {_ZN11QTextLayout11clearLayoutEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QTextLayout::cacheEnabled();
impl /*struct*/ QTextLayout {
  pub fn cacheEnabled<RetType, T: QTextLayout_cacheEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.cacheEnabled(self);
    // return 1;
  }
}

pub trait QTextLayout_cacheEnabled<RetType> {
  fn cacheEnabled(self , rsthis: & QTextLayout) -> RetType;
}

  // proto:  bool QTextLayout::cacheEnabled();
impl<'a> /*trait*/ QTextLayout_cacheEnabled<i8> for () {
  fn cacheEnabled(self , rsthis: & QTextLayout) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextLayout12cacheEnabledEv()};
    let mut ret = unsafe {_ZNK11QTextLayout12cacheEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  qreal QTextLayout::maximumWidth();
impl /*struct*/ QTextLayout {
  pub fn maximumWidth<RetType, T: QTextLayout_maximumWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.maximumWidth(self);
    // return 1;
  }
}

pub trait QTextLayout_maximumWidth<RetType> {
  fn maximumWidth(self , rsthis: & QTextLayout) -> RetType;
}

  // proto:  qreal QTextLayout::maximumWidth();
impl<'a> /*trait*/ QTextLayout_maximumWidth<f64> for () {
  fn maximumWidth(self , rsthis: & QTextLayout) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextLayout12maximumWidthEv()};
    let mut ret = unsafe {_ZNK11QTextLayout12maximumWidthEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  QString QTextLayout::text();
impl /*struct*/ QTextLayout {
  pub fn text<RetType, T: QTextLayout_text<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.text(self);
    // return 1;
  }
}

pub trait QTextLayout_text<RetType> {
  fn text(self , rsthis: & QTextLayout) -> RetType;
}

  // proto:  QString QTextLayout::text();
impl<'a> /*trait*/ QTextLayout_text<QString> for () {
  fn text(self , rsthis: & QTextLayout) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextLayout4textEv()};
    let mut ret = unsafe {_ZNK11QTextLayout4textEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextLayout::QTextLayout(const QTextLayout & );
impl<'a> /*trait*/ QTextLayout_New for (&'a QTextLayout) {
  fn New(self) -> QTextLayout {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextLayoutC1ERKS_()};
    let ctysz: c_int = unsafe{QTextLayout_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN11QTextLayoutC1ERKS_(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN11QTextLayoutC1ERKS_(arg0)};
    let rsthis = QTextLayout{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QTextLine QTextLayout::createLine();
impl /*struct*/ QTextLayout {
  pub fn createLine<RetType, T: QTextLayout_createLine<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.createLine(self);
    // return 1;
  }
}

pub trait QTextLayout_createLine<RetType> {
  fn createLine(self , rsthis: & QTextLayout) -> RetType;
}

  // proto:  QTextLine QTextLayout::createLine();
impl<'a> /*trait*/ QTextLayout_createLine<QTextLine> for () {
  fn createLine(self , rsthis: & QTextLayout) -> QTextLine {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextLayout10createLineEv()};
    let mut ret = unsafe {_ZN11QTextLayout10createLineEv(rsthis.qclsinst)};
    let mut ret1 = QTextLine::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QTextLayout::preeditAreaText();
impl /*struct*/ QTextLayout {
  pub fn preeditAreaText<RetType, T: QTextLayout_preeditAreaText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.preeditAreaText(self);
    // return 1;
  }
}

pub trait QTextLayout_preeditAreaText<RetType> {
  fn preeditAreaText(self , rsthis: & QTextLayout) -> RetType;
}

  // proto:  QString QTextLayout::preeditAreaText();
impl<'a> /*trait*/ QTextLayout_preeditAreaText<QString> for () {
  fn preeditAreaText(self , rsthis: & QTextLayout) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextLayout15preeditAreaTextEv()};
    let mut ret = unsafe {_ZNK11QTextLayout15preeditAreaTextEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextLayout::drawCursor(QPainter * p, const QPointF & pos, int cursorPosition, int width);
impl<'a> /*trait*/ QTextLayout_drawCursor<()> for (&'a QPainter, &'a QPointF, i32, i32) {
  fn drawCursor(self , rsthis: & QTextLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextLayout10drawCursorEP8QPainterRK7QPointFii()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
     unsafe {_ZNK11QTextLayout10drawCursorEP8QPainterRK7QPointFii(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  void QTextLayout::endLayout();
impl /*struct*/ QTextLayout {
  pub fn endLayout<RetType, T: QTextLayout_endLayout<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.endLayout(self);
    // return 1;
  }
}

pub trait QTextLayout_endLayout<RetType> {
  fn endLayout(self , rsthis: & QTextLayout) -> RetType;
}

  // proto:  void QTextLayout::endLayout();
impl<'a> /*trait*/ QTextLayout_endLayout<()> for () {
  fn endLayout(self , rsthis: & QTextLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextLayout9endLayoutEv()};
     unsafe {_ZN11QTextLayout9endLayoutEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTextLayout::QTextLayout();
impl<'a> /*trait*/ QTextLayout_New for () {
  fn New(self) -> QTextLayout {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTextLayoutC1Ev()};
    let ctysz: c_int = unsafe{QTextLayout_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    // unsafe {_ZN11QTextLayoutC1Ev(qthis)};
    let qthis: *mut c_void = unsafe {dector_ZN11QTextLayoutC1Ev()};
    let rsthis = QTextLayout{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QList<QGlyphRun> QTextLayout::glyphRuns(int from, int length);
impl /*struct*/ QTextLayout {
  pub fn glyphRuns<RetType, T: QTextLayout_glyphRuns<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.glyphRuns(self);
    // return 1;
  }
}

pub trait QTextLayout_glyphRuns<RetType> {
  fn glyphRuns(self , rsthis: & QTextLayout) -> RetType;
}

  // proto:  QList<QGlyphRun> QTextLayout::glyphRuns(int from, int length);
impl<'a> /*trait*/ QTextLayout_glyphRuns<()> for (i32, i32) {
  fn glyphRuns(self , rsthis: & QTextLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTextLayout9glyphRunsEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZNK11QTextLayout9glyphRunsEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QTextInlineObject {
  pub fn inheritFrom(qthis: *mut c_void) -> QTextInlineObject {
    return QTextInlineObject{qclsinst: qthis};
  }
}
  // proto:  void QTextInlineObject::setAscent(qreal a);
impl /*struct*/ QTextInlineObject {
  pub fn setAscent<RetType, T: QTextInlineObject_setAscent<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setAscent(self);
    // return 1;
  }
}

pub trait QTextInlineObject_setAscent<RetType> {
  fn setAscent(self , rsthis: & QTextInlineObject) -> RetType;
}

  // proto:  void QTextInlineObject::setAscent(qreal a);
impl<'a> /*trait*/ QTextInlineObject_setAscent<()> for (f64) {
  fn setAscent(self , rsthis: & QTextInlineObject) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QTextInlineObject9setAscentEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN17QTextInlineObject9setAscentEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  qreal QTextInlineObject::width();
impl /*struct*/ QTextInlineObject {
  pub fn width<RetType, T: QTextInlineObject_width<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.width(self);
    // return 1;
  }
}

pub trait QTextInlineObject_width<RetType> {
  fn width(self , rsthis: & QTextInlineObject) -> RetType;
}

  // proto:  qreal QTextInlineObject::width();
impl<'a> /*trait*/ QTextInlineObject_width<f64> for () {
  fn width(self , rsthis: & QTextInlineObject) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QTextInlineObject5widthEv()};
    let mut ret = unsafe {_ZNK17QTextInlineObject5widthEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  int QTextInlineObject::formatIndex();
impl /*struct*/ QTextInlineObject {
  pub fn formatIndex<RetType, T: QTextInlineObject_formatIndex<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.formatIndex(self);
    // return 1;
  }
}

pub trait QTextInlineObject_formatIndex<RetType> {
  fn formatIndex(self , rsthis: & QTextInlineObject) -> RetType;
}

  // proto:  int QTextInlineObject::formatIndex();
impl<'a> /*trait*/ QTextInlineObject_formatIndex<i32> for () {
  fn formatIndex(self , rsthis: & QTextInlineObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QTextInlineObject11formatIndexEv()};
    let mut ret = unsafe {_ZNK17QTextInlineObject11formatIndexEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QRectF QTextInlineObject::rect();
impl /*struct*/ QTextInlineObject {
  pub fn rect<RetType, T: QTextInlineObject_rect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rect(self);
    // return 1;
  }
}

pub trait QTextInlineObject_rect<RetType> {
  fn rect(self , rsthis: & QTextInlineObject) -> RetType;
}

  // proto:  QRectF QTextInlineObject::rect();
impl<'a> /*trait*/ QTextInlineObject_rect<QRectF> for () {
  fn rect(self , rsthis: & QTextInlineObject) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QTextInlineObject4rectEv()};
    let mut ret = unsafe {_ZNK17QTextInlineObject4rectEv(rsthis.qclsinst)};
    let mut ret1 = QRectF::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  int QTextInlineObject::textPosition();
impl /*struct*/ QTextInlineObject {
  pub fn textPosition<RetType, T: QTextInlineObject_textPosition<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.textPosition(self);
    // return 1;
  }
}

pub trait QTextInlineObject_textPosition<RetType> {
  fn textPosition(self , rsthis: & QTextInlineObject) -> RetType;
}

  // proto:  int QTextInlineObject::textPosition();
impl<'a> /*trait*/ QTextInlineObject_textPosition<i32> for () {
  fn textPosition(self , rsthis: & QTextInlineObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QTextInlineObject12textPositionEv()};
    let mut ret = unsafe {_ZNK17QTextInlineObject12textPositionEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QTextInlineObject::setDescent(qreal d);
impl /*struct*/ QTextInlineObject {
  pub fn setDescent<RetType, T: QTextInlineObject_setDescent<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDescent(self);
    // return 1;
  }
}

pub trait QTextInlineObject_setDescent<RetType> {
  fn setDescent(self , rsthis: & QTextInlineObject) -> RetType;
}

  // proto:  void QTextInlineObject::setDescent(qreal d);
impl<'a> /*trait*/ QTextInlineObject_setDescent<()> for (f64) {
  fn setDescent(self , rsthis: & QTextInlineObject) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QTextInlineObject10setDescentEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN17QTextInlineObject10setDescentEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  qreal QTextInlineObject::height();
impl /*struct*/ QTextInlineObject {
  pub fn height<RetType, T: QTextInlineObject_height<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.height(self);
    // return 1;
  }
}

pub trait QTextInlineObject_height<RetType> {
  fn height(self , rsthis: & QTextInlineObject) -> RetType;
}

  // proto:  qreal QTextInlineObject::height();
impl<'a> /*trait*/ QTextInlineObject_height<f64> for () {
  fn height(self , rsthis: & QTextInlineObject) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QTextInlineObject6heightEv()};
    let mut ret = unsafe {_ZNK17QTextInlineObject6heightEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  bool QTextInlineObject::isValid();
impl /*struct*/ QTextInlineObject {
  pub fn isValid<RetType, T: QTextInlineObject_isValid<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QTextInlineObject_isValid<RetType> {
  fn isValid(self , rsthis: & QTextInlineObject) -> RetType;
}

  // proto:  bool QTextInlineObject::isValid();
impl<'a> /*trait*/ QTextInlineObject_isValid<i8> for () {
  fn isValid(self , rsthis: & QTextInlineObject) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QTextInlineObject7isValidEv()};
    let mut ret = unsafe {demth_ZNK17QTextInlineObject7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QTextInlineObject::QTextInlineObject();
impl /*struct*/ QTextInlineObject {
  pub fn New<T: QTextInlineObject_New>(value: T) -> QTextInlineObject {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QTextInlineObject_New {
  fn New(self) -> QTextInlineObject;
}

  // proto:  void QTextInlineObject::QTextInlineObject();
impl<'a> /*trait*/ QTextInlineObject_New for () {
  fn New(self) -> QTextInlineObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QTextInlineObjectC1Ev()};
    let ctysz: c_int = unsafe{QTextInlineObject_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    // unsafe {_ZN17QTextInlineObjectC1Ev(qthis)};
    let qthis: *mut c_void = unsafe {dector_ZN17QTextInlineObjectC1Ev()};
    let rsthis = QTextInlineObject{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QTextFormat QTextInlineObject::format();
impl /*struct*/ QTextInlineObject {
  pub fn format<RetType, T: QTextInlineObject_format<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.format(self);
    // return 1;
  }
}

pub trait QTextInlineObject_format<RetType> {
  fn format(self , rsthis: & QTextInlineObject) -> RetType;
}

  // proto:  QTextFormat QTextInlineObject::format();
impl<'a> /*trait*/ QTextInlineObject_format<QTextFormat> for () {
  fn format(self , rsthis: & QTextInlineObject) -> QTextFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QTextInlineObject6formatEv()};
    let mut ret = unsafe {_ZNK17QTextInlineObject6formatEv(rsthis.qclsinst)};
    let mut ret1 = QTextFormat::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  qreal QTextInlineObject::descent();
impl /*struct*/ QTextInlineObject {
  pub fn descent<RetType, T: QTextInlineObject_descent<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.descent(self);
    // return 1;
  }
}

pub trait QTextInlineObject_descent<RetType> {
  fn descent(self , rsthis: & QTextInlineObject) -> RetType;
}

  // proto:  qreal QTextInlineObject::descent();
impl<'a> /*trait*/ QTextInlineObject_descent<f64> for () {
  fn descent(self , rsthis: & QTextInlineObject) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QTextInlineObject7descentEv()};
    let mut ret = unsafe {_ZNK17QTextInlineObject7descentEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  qreal QTextInlineObject::ascent();
impl /*struct*/ QTextInlineObject {
  pub fn ascent<RetType, T: QTextInlineObject_ascent<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.ascent(self);
    // return 1;
  }
}

pub trait QTextInlineObject_ascent<RetType> {
  fn ascent(self , rsthis: & QTextInlineObject) -> RetType;
}

  // proto:  qreal QTextInlineObject::ascent();
impl<'a> /*trait*/ QTextInlineObject_ascent<f64> for () {
  fn ascent(self , rsthis: & QTextInlineObject) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QTextInlineObject6ascentEv()};
    let mut ret = unsafe {_ZNK17QTextInlineObject6ascentEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QTextInlineObject::setWidth(qreal w);
impl /*struct*/ QTextInlineObject {
  pub fn setWidth<RetType, T: QTextInlineObject_setWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setWidth(self);
    // return 1;
  }
}

pub trait QTextInlineObject_setWidth<RetType> {
  fn setWidth(self , rsthis: & QTextInlineObject) -> RetType;
}

  // proto:  void QTextInlineObject::setWidth(qreal w);
impl<'a> /*trait*/ QTextInlineObject_setWidth<()> for (f64) {
  fn setWidth(self , rsthis: & QTextInlineObject) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QTextInlineObject8setWidthEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN17QTextInlineObject8setWidthEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// <= body block end

