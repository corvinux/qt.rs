// auto generated, do not modify.
// created: Mon Feb 22 23:57:02 2016
// src-file: /QtWidgets/qproxystyle.h
// dst-file: /src/widgets/qproxystyle.rs
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
use super::qcommonstyle::*; // 773
use std::ops::Deref;
use super::qwidget::*; // 773
use super::qstyleoption::*; // 773
use super::super::gui::qpainter::*; // 771
use super::super::core::qstring::*; // 771
use super::qapplication::*; // 773
use super::super::gui::qpalette::*; // 771
use super::qstyle::*; // 773
use super::super::gui::qpixmap::*; // 771
use super::super::core::qpoint::*; // 771
use super::super::core::qsize::*; // 771
use super::super::core::qrect::*; // 771
use super::super::gui::qfontmetrics::*; // 771
use super::super::core::qobjectdefs::*; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QProxyStyle_Class_Size() -> c_int;
  // proto:  void QProxyStyle::unpolish(QWidget * widget);
  fn C_ZN11QProxyStyle8unpolishEP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QProxyStyle::QProxyStyle(const QString & key);
  fn C_ZN11QProxyStyleC2ERK7QString(arg0: *mut c_void) -> u64;
  // proto:  void QProxyStyle::unpolish(QApplication * app);
  fn C_ZN11QProxyStyle8unpolishEP12QApplication(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QPalette QProxyStyle::standardPalette();
  fn C_ZNK11QProxyStyle15standardPaletteEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QProxyStyle::setBaseStyle(QStyle * style);
  fn C_ZN11QProxyStyle12setBaseStyleEP6QStyle(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QProxyStyle::polish(QPalette & pal);
  fn C_ZN11QProxyStyle6polishER8QPalette(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QProxyStyle::drawItemPixmap(QPainter * painter, const QRect & rect, int alignment, const QPixmap & pixmap);
  fn C_ZNK11QProxyStyle14drawItemPixmapEP8QPainterRK5QRectiRK7QPixmap(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: c_int, arg3: *mut c_void);
  // proto:  void QProxyStyle::~QProxyStyle();
  fn C_ZN11QProxyStyleD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QStyle * QProxyStyle::baseStyle();
  fn C_ZNK11QProxyStyle9baseStyleEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QProxyStyle::polish(QApplication * app);
  fn C_ZN11QProxyStyle6polishEP12QApplication(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QProxyStyle::polish(QWidget * widget);
  fn C_ZN11QProxyStyle6polishEP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QRect QProxyStyle::itemTextRect(const QFontMetrics & fm, const QRect & r, int flags, bool enabled, const QString & text);
  fn C_ZNK11QProxyStyle12itemTextRectERK12QFontMetricsRK5QRectibRK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: c_int, arg3: c_char, arg4: *mut c_void) -> *mut c_void;
  // proto:  QRect QProxyStyle::itemPixmapRect(const QRect & r, int flags, const QPixmap & pixmap);
  fn C_ZNK11QProxyStyle14itemPixmapRectERK5QRectiRK7QPixmap(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int, arg2: *mut c_void) -> *mut c_void;
  // proto:  const QMetaObject * QProxyStyle::metaObject();
  fn C_ZNK11QProxyStyle10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QProxyStyle::QProxyStyle(QStyle * style);
  fn C_ZN11QProxyStyleC2EP6QStyle(arg0: *mut c_void) -> u64;
} // <= ext block end

// body block begin =>
// class sizeof(QProxyStyle)=1
#[derive(Default)]
pub struct QProxyStyle {
  qbase: QCommonStyle,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QProxyStyle {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QProxyStyle {
    return QProxyStyle{qbase: QCommonStyle::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QProxyStyle {
  type Target = QCommonStyle;

  fn deref(&self) -> &QCommonStyle {
    return & self.qbase;
  }
}
impl AsRef<QCommonStyle> for QProxyStyle {
  fn as_ref(& self) -> & QCommonStyle {
    return & self.qbase;
  }
}
  // proto:  void QProxyStyle::unpolish(QWidget * widget);
impl /*struct*/ QProxyStyle {
  pub fn unpolish<RetType, T: QProxyStyle_unpolish<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.unpolish(self);
    // return 1;
  }
}

pub trait QProxyStyle_unpolish<RetType> {
  fn unpolish(self , rsthis: & QProxyStyle) -> RetType;
}

  // proto:  void QProxyStyle::unpolish(QWidget * widget);
impl<'a> /*trait*/ QProxyStyle_unpolish<()> for (&'a QWidget) {
  fn unpolish(self , rsthis: & QProxyStyle) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QProxyStyle8unpolishEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN11QProxyStyle8unpolishEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QProxyStyle::QProxyStyle(const QString & key);
impl /*struct*/ QProxyStyle {
  pub fn new<T: QProxyStyle_new>(value: T) -> QProxyStyle {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QProxyStyle_new {
  fn new(self) -> QProxyStyle;
}

  // proto:  void QProxyStyle::QProxyStyle(const QString & key);
impl<'a> /*trait*/ QProxyStyle_new for (&'a QString) {
  fn new(self) -> QProxyStyle {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QProxyStyleC2ERK7QString()};
    let ctysz: c_int = unsafe{QProxyStyle_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN11QProxyStyleC2ERK7QString(arg0)};
    let rsthis = QProxyStyle{qbase: QCommonStyle::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QProxyStyle::unpolish(QApplication * app);
impl<'a> /*trait*/ QProxyStyle_unpolish<()> for (&'a QApplication) {
  fn unpolish(self , rsthis: & QProxyStyle) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QProxyStyle8unpolishEP12QApplication()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN11QProxyStyle8unpolishEP12QApplication(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QPalette QProxyStyle::standardPalette();
impl /*struct*/ QProxyStyle {
  pub fn standardPalette<RetType, T: QProxyStyle_standardPalette<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.standardPalette(self);
    // return 1;
  }
}

pub trait QProxyStyle_standardPalette<RetType> {
  fn standardPalette(self , rsthis: & QProxyStyle) -> RetType;
}

  // proto:  QPalette QProxyStyle::standardPalette();
impl<'a> /*trait*/ QProxyStyle_standardPalette<QPalette> for () {
  fn standardPalette(self , rsthis: & QProxyStyle) -> QPalette {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QProxyStyle15standardPaletteEv()};
    let mut ret = unsafe {C_ZNK11QProxyStyle15standardPaletteEv(rsthis.qclsinst)};
    let mut ret1 = QPalette::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QProxyStyle::setBaseStyle(QStyle * style);
impl /*struct*/ QProxyStyle {
  pub fn setBaseStyle<RetType, T: QProxyStyle_setBaseStyle<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setBaseStyle(self);
    // return 1;
  }
}

pub trait QProxyStyle_setBaseStyle<RetType> {
  fn setBaseStyle(self , rsthis: & QProxyStyle) -> RetType;
}

  // proto:  void QProxyStyle::setBaseStyle(QStyle * style);
impl<'a> /*trait*/ QProxyStyle_setBaseStyle<()> for (&'a QStyle) {
  fn setBaseStyle(self , rsthis: & QProxyStyle) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QProxyStyle12setBaseStyleEP6QStyle()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN11QProxyStyle12setBaseStyleEP6QStyle(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QProxyStyle::polish(QPalette & pal);
impl /*struct*/ QProxyStyle {
  pub fn polish<RetType, T: QProxyStyle_polish<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.polish(self);
    // return 1;
  }
}

pub trait QProxyStyle_polish<RetType> {
  fn polish(self , rsthis: & QProxyStyle) -> RetType;
}

  // proto:  void QProxyStyle::polish(QPalette & pal);
impl<'a> /*trait*/ QProxyStyle_polish<()> for (&'a QPalette) {
  fn polish(self , rsthis: & QProxyStyle) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QProxyStyle6polishER8QPalette()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN11QProxyStyle6polishER8QPalette(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QProxyStyle::drawItemPixmap(QPainter * painter, const QRect & rect, int alignment, const QPixmap & pixmap);
impl /*struct*/ QProxyStyle {
  pub fn drawItemPixmap<RetType, T: QProxyStyle_drawItemPixmap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.drawItemPixmap(self);
    // return 1;
  }
}

pub trait QProxyStyle_drawItemPixmap<RetType> {
  fn drawItemPixmap(self , rsthis: & QProxyStyle) -> RetType;
}

  // proto:  void QProxyStyle::drawItemPixmap(QPainter * painter, const QRect & rect, int alignment, const QPixmap & pixmap);
impl<'a> /*trait*/ QProxyStyle_drawItemPixmap<()> for (&'a QPainter, &'a QRect, i32, &'a QPixmap) {
  fn drawItemPixmap(self , rsthis: & QProxyStyle) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QProxyStyle14drawItemPixmapEP8QPainterRK5QRectiRK7QPixmap()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
    let arg3 = self.3.qclsinst  as *mut c_void;
     unsafe {C_ZNK11QProxyStyle14drawItemPixmapEP8QPainterRK5QRectiRK7QPixmap(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  void QProxyStyle::~QProxyStyle();
impl /*struct*/ QProxyStyle {
  pub fn free<RetType, T: QProxyStyle_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QProxyStyle_free<RetType> {
  fn free(self , rsthis: & QProxyStyle) -> RetType;
}

  // proto:  void QProxyStyle::~QProxyStyle();
impl<'a> /*trait*/ QProxyStyle_free<()> for () {
  fn free(self , rsthis: & QProxyStyle) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QProxyStyleD2Ev()};
     unsafe {C_ZN11QProxyStyleD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QStyle * QProxyStyle::baseStyle();
impl /*struct*/ QProxyStyle {
  pub fn baseStyle<RetType, T: QProxyStyle_baseStyle<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.baseStyle(self);
    // return 1;
  }
}

pub trait QProxyStyle_baseStyle<RetType> {
  fn baseStyle(self , rsthis: & QProxyStyle) -> RetType;
}

  // proto:  QStyle * QProxyStyle::baseStyle();
impl<'a> /*trait*/ QProxyStyle_baseStyle<QStyle> for () {
  fn baseStyle(self , rsthis: & QProxyStyle) -> QStyle {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QProxyStyle9baseStyleEv()};
    let mut ret = unsafe {C_ZNK11QProxyStyle9baseStyleEv(rsthis.qclsinst)};
    let mut ret1 = QStyle::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QProxyStyle::polish(QApplication * app);
impl<'a> /*trait*/ QProxyStyle_polish<()> for (&'a QApplication) {
  fn polish(self , rsthis: & QProxyStyle) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QProxyStyle6polishEP12QApplication()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN11QProxyStyle6polishEP12QApplication(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QProxyStyle::polish(QWidget * widget);
impl<'a> /*trait*/ QProxyStyle_polish<()> for (&'a QWidget) {
  fn polish(self , rsthis: & QProxyStyle) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QProxyStyle6polishEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN11QProxyStyle6polishEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QRect QProxyStyle::itemTextRect(const QFontMetrics & fm, const QRect & r, int flags, bool enabled, const QString & text);
impl /*struct*/ QProxyStyle {
  pub fn itemTextRect<RetType, T: QProxyStyle_itemTextRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.itemTextRect(self);
    // return 1;
  }
}

pub trait QProxyStyle_itemTextRect<RetType> {
  fn itemTextRect(self , rsthis: & QProxyStyle) -> RetType;
}

  // proto:  QRect QProxyStyle::itemTextRect(const QFontMetrics & fm, const QRect & r, int flags, bool enabled, const QString & text);
impl<'a> /*trait*/ QProxyStyle_itemTextRect<QRect> for (&'a QFontMetrics, &'a QRect, i32, i8, &'a QString) {
  fn itemTextRect(self , rsthis: & QProxyStyle) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QProxyStyle12itemTextRectERK12QFontMetricsRK5QRectibRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_char;
    let arg4 = self.4.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK11QProxyStyle12itemTextRectERK12QFontMetricsRK5QRectibRK7QString(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4)};
    let mut ret1 = QRect::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QRect QProxyStyle::itemPixmapRect(const QRect & r, int flags, const QPixmap & pixmap);
impl /*struct*/ QProxyStyle {
  pub fn itemPixmapRect<RetType, T: QProxyStyle_itemPixmapRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.itemPixmapRect(self);
    // return 1;
  }
}

pub trait QProxyStyle_itemPixmapRect<RetType> {
  fn itemPixmapRect(self , rsthis: & QProxyStyle) -> RetType;
}

  // proto:  QRect QProxyStyle::itemPixmapRect(const QRect & r, int flags, const QPixmap & pixmap);
impl<'a> /*trait*/ QProxyStyle_itemPixmapRect<QRect> for (&'a QRect, i32, &'a QPixmap) {
  fn itemPixmapRect(self , rsthis: & QProxyStyle) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QProxyStyle14itemPixmapRectERK5QRectiRK7QPixmap()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK11QProxyStyle14itemPixmapRectERK5QRectiRK7QPixmap(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QRect::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QMetaObject * QProxyStyle::metaObject();
impl /*struct*/ QProxyStyle {
  pub fn metaObject<RetType, T: QProxyStyle_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QProxyStyle_metaObject<RetType> {
  fn metaObject(self , rsthis: & QProxyStyle) -> RetType;
}

  // proto:  const QMetaObject * QProxyStyle::metaObject();
impl<'a> /*trait*/ QProxyStyle_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QProxyStyle) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QProxyStyle10metaObjectEv()};
    let mut ret = unsafe {C_ZNK11QProxyStyle10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QProxyStyle::QProxyStyle(QStyle * style);
impl<'a> /*trait*/ QProxyStyle_new for (Option<&'a QStyle>) {
  fn new(self) -> QProxyStyle {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QProxyStyleC2EP6QStyle()};
    let ctysz: c_int = unsafe{QProxyStyle_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = (if self.is_none() {0} else {self.unwrap().qclsinst})  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN11QProxyStyleC2EP6QStyle(arg0)};
    let rsthis = QProxyStyle{qbase: QCommonStyle::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// <= body block end

