// auto generated, do not modify.
// created: Tue Dec 22 23:21:28 2015
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
use super::qcommonstyle::QCommonStyle; // 773
use std::ops::Deref;
use super::qwidget::QWidget; // 773
use super::qstyleoption::QStyleOptionComplex; // 773
use super::super::gui::qpainter::QPainter; // 771
use super::super::core::qstring::QString; // 771
use super::qapplication::QApplication; // 773
use super::qstyleoption::QStyleOption; // 773
use super::qstyleoption::QStyleHintReturn; // 773
use super::super::gui::qpalette::QPalette; // 771
use super::qstyle::QStyle; // 773
use super::super::gui::qpixmap::QPixmap; // 771
use super::super::core::qpoint::QPoint; // 771
use super::super::core::qsize::QSize; // 771
use super::super::core::qrect::QRect; // 771
use super::super::gui::qfontmetrics::QFontMetrics; // 771
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  void QProxyStyle::unpolish(QWidget * widget);
  fn _ZN11QProxyStyle8unpolishEP7QWidget(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QProxyStyle::QProxyStyle(const QString & key);
  fn _ZN11QProxyStyleC1ERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QProxyStyle::unpolish(QApplication * app);
  fn _ZN11QProxyStyle8unpolishEP12QApplication(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QPalette QProxyStyle::standardPalette();
  fn _ZNK11QProxyStyle15standardPaletteEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QProxyStyle::setBaseStyle(QStyle * style);
  fn _ZN11QProxyStyle12setBaseStyleEP6QStyle(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QProxyStyle::polish(QPalette & pal);
  fn _ZN11QProxyStyle6polishER8QPalette(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QProxyStyle::drawItemPixmap(QPainter * painter, const QRect & rect, int alignment, const QPixmap & pixmap);
  fn _ZNK11QProxyStyle14drawItemPixmapEP8QPainterRK5QRectiRK7QPixmap(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: c_int, arg3: *mut c_void);
  // proto:  void QProxyStyle::~QProxyStyle();
  fn _ZN11QProxyStyleD0Ev(qthis: *mut c_void);
  // proto:  QStyle * QProxyStyle::baseStyle();
  fn _ZNK11QProxyStyle9baseStyleEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QProxyStyle::QProxyStyle(const QProxyStyle & );
  fn _ZN11QProxyStyleC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QProxyStyle::polish(QApplication * app);
  fn _ZN11QProxyStyle6polishEP12QApplication(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QProxyStyle::polish(QWidget * widget);
  fn _ZN11QProxyStyle6polishEP7QWidget(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QRect QProxyStyle::itemTextRect(const QFontMetrics & fm, const QRect & r, int flags, bool enabled, const QString & text);
  fn _ZNK11QProxyStyle12itemTextRectERK12QFontMetricsRK5QRectibRK7QString(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: c_int, arg3: c_char, arg4: *mut c_void) -> *mut c_void;
  // proto:  QRect QProxyStyle::itemPixmapRect(const QRect & r, int flags, const QPixmap & pixmap);
  fn _ZNK11QProxyStyle14itemPixmapRectERK5QRectiRK7QPixmap(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: *mut c_void) -> *mut c_void;
  // proto:  const QMetaObject * QProxyStyle::metaObject();
  fn _ZNK11QProxyStyle10metaObjectEv(qthis: *mut c_void);
  // proto:  void QProxyStyle::QProxyStyle(QStyle * style);
  fn _ZN11QProxyStyleC1EP6QStyle(qthis: *mut c_void, arg0: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QProxyStyle)=1
pub struct QProxyStyle {
  qbase: QCommonStyle,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QProxyStyle {
  pub fn inheritFrom(qthis: *mut c_void) -> QProxyStyle {
    return QProxyStyle{qbase: QCommonStyle::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QProxyStyle {
  type Target = QCommonStyle;

  fn deref(&self) -> &QCommonStyle {
    return &self.qbase;
  }
}
impl AsRef<QCommonStyle> for QProxyStyle {
  fn as_ref(&self) -> &QCommonStyle {
    return &self.qbase;
  }
}
  // proto:  void QProxyStyle::unpolish(QWidget * widget);
impl /*struct*/ QProxyStyle {
  pub fn unpolish<RetType, T: QProxyStyle_unpolish<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.unpolish(self);
    // return 1;
  }
}

pub trait QProxyStyle_unpolish<RetType> {
  fn unpolish(self , rsthis: &mut QProxyStyle) -> RetType;
}

  // proto:  void QProxyStyle::unpolish(QWidget * widget);
impl<'a> /*trait*/ QProxyStyle_unpolish<()> for (QWidget) {
  fn unpolish(self , rsthis: &mut QProxyStyle) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QProxyStyle8unpolishEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QProxyStyle8unpolishEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QProxyStyle::QProxyStyle(const QString & key);
impl /*struct*/ QProxyStyle {
  pub fn NewQProxyStyle<T: QProxyStyle_NewQProxyStyle>(value: T) -> QProxyStyle {
    let rsthis = value.NewQProxyStyle();
    return rsthis;
    // return 1;
  }
}

pub trait QProxyStyle_NewQProxyStyle {
  fn NewQProxyStyle(self) -> QProxyStyle;
}

  // proto:  void QProxyStyle::QProxyStyle(const QString & key);
impl<'a> /*trait*/ QProxyStyle_NewQProxyStyle for (QString) {
  fn NewQProxyStyle(self) -> QProxyStyle {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QProxyStyleC1ERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QProxyStyleC1ERK7QString(qthis, arg0)};
    let rsthis = QProxyStyle{/**/qbase: QCommonStyle::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QProxyStyle::unpolish(QApplication * app);
impl<'a> /*trait*/ QProxyStyle_unpolish<()> for (QApplication) {
  fn unpolish(self , rsthis: &mut QProxyStyle) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QProxyStyle8unpolishEP12QApplication()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QProxyStyle8unpolishEP12QApplication(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QPalette QProxyStyle::standardPalette();
impl /*struct*/ QProxyStyle {
  pub fn standardPalette<RetType, T: QProxyStyle_standardPalette<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.standardPalette(self);
    // return 1;
  }
}

pub trait QProxyStyle_standardPalette<RetType> {
  fn standardPalette(self , rsthis: &mut QProxyStyle) -> RetType;
}

  // proto:  QPalette QProxyStyle::standardPalette();
impl<'a> /*trait*/ QProxyStyle_standardPalette<QPalette> for () {
  fn standardPalette(self , rsthis: &mut QProxyStyle) -> QPalette {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QProxyStyle15standardPaletteEv()};
    let mut ret = unsafe {_ZNK11QProxyStyle15standardPaletteEv(rsthis.qclsinst)};
    let mut ret1 = QPalette::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QProxyStyle::setBaseStyle(QStyle * style);
impl /*struct*/ QProxyStyle {
  pub fn setBaseStyle<RetType, T: QProxyStyle_setBaseStyle<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setBaseStyle(self);
    // return 1;
  }
}

pub trait QProxyStyle_setBaseStyle<RetType> {
  fn setBaseStyle(self , rsthis: &mut QProxyStyle) -> RetType;
}

  // proto:  void QProxyStyle::setBaseStyle(QStyle * style);
impl<'a> /*trait*/ QProxyStyle_setBaseStyle<()> for (QStyle) {
  fn setBaseStyle(self , rsthis: &mut QProxyStyle) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QProxyStyle12setBaseStyleEP6QStyle()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QProxyStyle12setBaseStyleEP6QStyle(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QProxyStyle::polish(QPalette & pal);
impl /*struct*/ QProxyStyle {
  pub fn polish<RetType, T: QProxyStyle_polish<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.polish(self);
    // return 1;
  }
}

pub trait QProxyStyle_polish<RetType> {
  fn polish(self , rsthis: &mut QProxyStyle) -> RetType;
}

  // proto:  void QProxyStyle::polish(QPalette & pal);
impl<'a> /*trait*/ QProxyStyle_polish<()> for (QPalette) {
  fn polish(self , rsthis: &mut QProxyStyle) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QProxyStyle6polishER8QPalette()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QProxyStyle6polishER8QPalette(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QProxyStyle::drawItemPixmap(QPainter * painter, const QRect & rect, int alignment, const QPixmap & pixmap);
impl /*struct*/ QProxyStyle {
  pub fn drawItemPixmap<RetType, T: QProxyStyle_drawItemPixmap<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.drawItemPixmap(self);
    // return 1;
  }
}

pub trait QProxyStyle_drawItemPixmap<RetType> {
  fn drawItemPixmap(self , rsthis: &mut QProxyStyle) -> RetType;
}

  // proto:  void QProxyStyle::drawItemPixmap(QPainter * painter, const QRect & rect, int alignment, const QPixmap & pixmap);
impl<'a> /*trait*/ QProxyStyle_drawItemPixmap<()> for (QPainter, QRect, i32, QPixmap) {
  fn drawItemPixmap(self , rsthis: &mut QProxyStyle) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QProxyStyle14drawItemPixmapEP8QPainterRK5QRectiRK7QPixmap()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
    let arg3 = self.3.qclsinst  as *mut c_void;
     unsafe {_ZNK11QProxyStyle14drawItemPixmapEP8QPainterRK5QRectiRK7QPixmap(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  void QProxyStyle::~QProxyStyle();
impl /*struct*/ QProxyStyle {
  pub fn FreeQProxyStyle<RetType, T: QProxyStyle_FreeQProxyStyle<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQProxyStyle(self);
    // return 1;
  }
}

pub trait QProxyStyle_FreeQProxyStyle<RetType> {
  fn FreeQProxyStyle(self , rsthis: &mut QProxyStyle) -> RetType;
}

  // proto:  void QProxyStyle::~QProxyStyle();
impl<'a> /*trait*/ QProxyStyle_FreeQProxyStyle<()> for () {
  fn FreeQProxyStyle(self , rsthis: &mut QProxyStyle) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QProxyStyleD0Ev()};
     unsafe {_ZN11QProxyStyleD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QStyle * QProxyStyle::baseStyle();
impl /*struct*/ QProxyStyle {
  pub fn baseStyle<RetType, T: QProxyStyle_baseStyle<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.baseStyle(self);
    // return 1;
  }
}

pub trait QProxyStyle_baseStyle<RetType> {
  fn baseStyle(self , rsthis: &mut QProxyStyle) -> RetType;
}

  // proto:  QStyle * QProxyStyle::baseStyle();
impl<'a> /*trait*/ QProxyStyle_baseStyle<QStyle> for () {
  fn baseStyle(self , rsthis: &mut QProxyStyle) -> QStyle {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QProxyStyle9baseStyleEv()};
    let mut ret = unsafe {_ZNK11QProxyStyle9baseStyleEv(rsthis.qclsinst)};
    let mut ret1 = QStyle::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QProxyStyle::QProxyStyle(const QProxyStyle & );
impl<'a> /*trait*/ QProxyStyle_NewQProxyStyle for (QProxyStyle) {
  fn NewQProxyStyle(self) -> QProxyStyle {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QProxyStyleC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QProxyStyleC1ERKS_(qthis, arg0)};
    let rsthis = QProxyStyle{/**/qbase: QCommonStyle::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QProxyStyle::polish(QApplication * app);
impl<'a> /*trait*/ QProxyStyle_polish<()> for (QApplication) {
  fn polish(self , rsthis: &mut QProxyStyle) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QProxyStyle6polishEP12QApplication()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QProxyStyle6polishEP12QApplication(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QProxyStyle::polish(QWidget * widget);
impl<'a> /*trait*/ QProxyStyle_polish<()> for (QWidget) {
  fn polish(self , rsthis: &mut QProxyStyle) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QProxyStyle6polishEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QProxyStyle6polishEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QRect QProxyStyle::itemTextRect(const QFontMetrics & fm, const QRect & r, int flags, bool enabled, const QString & text);
impl /*struct*/ QProxyStyle {
  pub fn itemTextRect<RetType, T: QProxyStyle_itemTextRect<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.itemTextRect(self);
    // return 1;
  }
}

pub trait QProxyStyle_itemTextRect<RetType> {
  fn itemTextRect(self , rsthis: &mut QProxyStyle) -> RetType;
}

  // proto:  QRect QProxyStyle::itemTextRect(const QFontMetrics & fm, const QRect & r, int flags, bool enabled, const QString & text);
impl<'a> /*trait*/ QProxyStyle_itemTextRect<QRect> for (QFontMetrics, QRect, i32, i8, QString) {
  fn itemTextRect(self , rsthis: &mut QProxyStyle) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QProxyStyle12itemTextRectERK12QFontMetricsRK5QRectibRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_char;
    let arg4 = self.4.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK11QProxyStyle12itemTextRectERK12QFontMetricsRK5QRectibRK7QString(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4)};
    let mut ret1 = QRect::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QRect QProxyStyle::itemPixmapRect(const QRect & r, int flags, const QPixmap & pixmap);
impl /*struct*/ QProxyStyle {
  pub fn itemPixmapRect<RetType, T: QProxyStyle_itemPixmapRect<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.itemPixmapRect(self);
    // return 1;
  }
}

pub trait QProxyStyle_itemPixmapRect<RetType> {
  fn itemPixmapRect(self , rsthis: &mut QProxyStyle) -> RetType;
}

  // proto:  QRect QProxyStyle::itemPixmapRect(const QRect & r, int flags, const QPixmap & pixmap);
impl<'a> /*trait*/ QProxyStyle_itemPixmapRect<QRect> for (QRect, i32, QPixmap) {
  fn itemPixmapRect(self , rsthis: &mut QProxyStyle) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QProxyStyle14itemPixmapRectERK5QRectiRK7QPixmap()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK11QProxyStyle14itemPixmapRectERK5QRectiRK7QPixmap(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QRect::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  const QMetaObject * QProxyStyle::metaObject();
impl /*struct*/ QProxyStyle {
  pub fn metaObject<RetType, T: QProxyStyle_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QProxyStyle_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QProxyStyle) -> RetType;
}

  // proto:  const QMetaObject * QProxyStyle::metaObject();
impl<'a> /*trait*/ QProxyStyle_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QProxyStyle) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QProxyStyle10metaObjectEv()};
     unsafe {_ZNK11QProxyStyle10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QProxyStyle::QProxyStyle(QStyle * style);
impl<'a> /*trait*/ QProxyStyle_NewQProxyStyle for (QStyle) {
  fn NewQProxyStyle(self) -> QProxyStyle {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QProxyStyleC1EP6QStyle()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QProxyStyleC1EP6QStyle(qthis, arg0)};
    let rsthis = QProxyStyle{/**/qbase: QCommonStyle::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// <= body block end

