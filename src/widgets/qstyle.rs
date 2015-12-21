// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qpixmap::QPixmap;
use super::qstyleoption::QStyleOption;
use super::qwidget::QWidget;
use super::qstyleoptioncomplex::QStyleOptionComplex;
use super::qpainter::QPainter;
use super::qpoint::QPoint;
use super::qrect::QRect;
use super::qpalette::QPalette;
use super::qfontmetrics::QFontMetrics;
use super::qstring::QString;
use super::qstylehintreturn::QStyleHintReturn;
use super::qsize::QSize;
use super::qapplication::QApplication;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QStyle::QStyle(const QStyle & );
  fn _ZN6QStyleC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QStyle::unpolish(QWidget * );
  fn _ZN6QStyle8unpolishEP7QWidget(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QStyle::~QStyle();
  fn _ZN6QStyleD0Ev(qthis: *mut c_void);
  // proto:  void QStyle::polish(QPalette & );
  fn _ZN6QStyle6polishER8QPalette(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QStyle::QStyle();
  fn _ZN6QStyleC1Ev(qthis: *mut c_void);
  // proto:  QRect QStyle::itemPixmapRect(const QRect & r, int flags, const QPixmap & pixmap);
  fn _ZNK6QStyle14itemPixmapRectERK5QRectiRK7QPixmap(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: *mut c_void) -> *mut c_void;
  // proto:  QRect QStyle::itemTextRect(const QFontMetrics & fm, const QRect & r, int flags, bool enabled, const QString & text);
  fn _ZNK6QStyle12itemTextRectERK12QFontMetricsRK5QRectibRK7QString(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: c_int, arg3: c_char, arg4: *mut c_void) -> *mut c_void;
  // proto:  const QStyle * QStyle::proxy();
  fn _ZNK6QStyle5proxyEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QPalette QStyle::standardPalette();
  fn _ZNK6QStyle15standardPaletteEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  const QMetaObject * QStyle::metaObject();
  fn _ZNK6QStyle10metaObjectEv(qthis: *mut c_void);
  // proto:  void QStyle::polish(QApplication * );
  fn _ZN6QStyle6polishEP12QApplication(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QStyle::drawItemPixmap(QPainter * painter, const QRect & rect, int alignment, const QPixmap & pixmap);
  fn _ZNK6QStyle14drawItemPixmapEP8QPainterRK5QRectiRK7QPixmap(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: c_int, arg3: *mut c_void);
  // proto:  void QStyle::polish(QWidget * );
  fn _ZN6QStyle6polishEP7QWidget(qthis: *mut c_void, arg0: *mut c_void);
  // proto: static int QStyle::sliderPositionFromValue(int min, int max, int val, int space, bool upsideDown);
  fn _ZN6QStyle23sliderPositionFromValueEiiiib(arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_char) -> c_int;
  // proto: static int QStyle::sliderValueFromPosition(int min, int max, int pos, int space, bool upsideDown);
  fn _ZN6QStyle23sliderValueFromPositionEiiiib(arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_char) -> c_int;
  // proto:  void QStyle::unpolish(QApplication * );
  fn _ZN6QStyle8unpolishEP12QApplication(qthis: *mut c_void, arg0: *mut c_void);
}

// body block begin
// class sizeof(QStyle)=1
pub struct QStyle {
  pub qclsinst: *mut c_void,
}

  // proto:  void QStyle::QStyle(const QStyle & );
impl /*struct*/ QStyle {
  pub fn NewQStyle<T: QStyle_NewQStyle>(value: T) -> QStyle {
    let rsthis = value.NewQStyle();
    return rsthis;
    // return 1;
  }
}

pub trait QStyle_NewQStyle {
  fn NewQStyle(self) -> QStyle;
}

  // proto:  void QStyle::QStyle(const QStyle & );
impl<'a> /*trait*/ QStyle_NewQStyle for (QStyle) {
  fn NewQStyle(self) -> QStyle {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QStyleC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN6QStyleC1ERKS_(qthis, arg0)};
    let rsthis = QStyle{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyle::unpolish(QWidget * );
impl /*struct*/ QStyle {
  pub fn unpolish<RetType, T: QStyle_unpolish<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.unpolish(self);
    // return 1;
  }
}

pub trait QStyle_unpolish<RetType> {
  fn unpolish(self , rsthis: &mut QStyle) -> RetType;
}

  // proto:  void QStyle::unpolish(QWidget * );
impl<'a> /*trait*/ QStyle_unpolish<()> for (QWidget) {
  fn unpolish(self , rsthis: &mut QStyle) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QStyle8unpolishEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QStyle8unpolishEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QStyle::~QStyle();
impl /*struct*/ QStyle {
  pub fn FreeQStyle<RetType, T: QStyle_FreeQStyle<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQStyle(self);
    // return 1;
  }
}

pub trait QStyle_FreeQStyle<RetType> {
  fn FreeQStyle(self , rsthis: &mut QStyle) -> RetType;
}

  // proto:  void QStyle::~QStyle();
impl<'a> /*trait*/ QStyle_FreeQStyle<()> for () {
  fn FreeQStyle(self , rsthis: &mut QStyle) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QStyleD0Ev()};
     unsafe {_ZN6QStyleD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QStyle::polish(QPalette & );
impl /*struct*/ QStyle {
  pub fn polish<RetType, T: QStyle_polish<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.polish(self);
    // return 1;
  }
}

pub trait QStyle_polish<RetType> {
  fn polish(self , rsthis: &mut QStyle) -> RetType;
}

  // proto:  void QStyle::polish(QPalette & );
impl<'a> /*trait*/ QStyle_polish<()> for (QPalette) {
  fn polish(self , rsthis: &mut QStyle) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QStyle6polishER8QPalette()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QStyle6polishER8QPalette(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QStyle::QStyle();
impl<'a> /*trait*/ QStyle_NewQStyle for () {
  fn NewQStyle(self) -> QStyle {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QStyleC1Ev()};
    unsafe {_ZN6QStyleC1Ev(qthis)};
    let rsthis = QStyle{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QRect QStyle::itemPixmapRect(const QRect & r, int flags, const QPixmap & pixmap);
impl /*struct*/ QStyle {
  pub fn itemPixmapRect<RetType, T: QStyle_itemPixmapRect<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.itemPixmapRect(self);
    // return 1;
  }
}

pub trait QStyle_itemPixmapRect<RetType> {
  fn itemPixmapRect(self , rsthis: &mut QStyle) -> RetType;
}

  // proto:  QRect QStyle::itemPixmapRect(const QRect & r, int flags, const QPixmap & pixmap);
impl<'a> /*trait*/ QStyle_itemPixmapRect<QRect> for (QRect, i32, QPixmap) {
  fn itemPixmapRect(self , rsthis: &mut QStyle) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QStyle14itemPixmapRectERK5QRectiRK7QPixmap()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK6QStyle14itemPixmapRectERK5QRectiRK7QPixmap(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QRect QStyle::itemTextRect(const QFontMetrics & fm, const QRect & r, int flags, bool enabled, const QString & text);
impl /*struct*/ QStyle {
  pub fn itemTextRect<RetType, T: QStyle_itemTextRect<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.itemTextRect(self);
    // return 1;
  }
}

pub trait QStyle_itemTextRect<RetType> {
  fn itemTextRect(self , rsthis: &mut QStyle) -> RetType;
}

  // proto:  QRect QStyle::itemTextRect(const QFontMetrics & fm, const QRect & r, int flags, bool enabled, const QString & text);
impl<'a> /*trait*/ QStyle_itemTextRect<QRect> for (QFontMetrics, QRect, i32, i8, QString) {
  fn itemTextRect(self , rsthis: &mut QStyle) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QStyle12itemTextRectERK12QFontMetricsRK5QRectibRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_char;
    let arg4 = self.4.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK6QStyle12itemTextRectERK12QFontMetricsRK5QRectibRK7QString(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  const QStyle * QStyle::proxy();
impl /*struct*/ QStyle {
  pub fn proxy<RetType, T: QStyle_proxy<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.proxy(self);
    // return 1;
  }
}

pub trait QStyle_proxy<RetType> {
  fn proxy(self , rsthis: &mut QStyle) -> RetType;
}

  // proto:  const QStyle * QStyle::proxy();
impl<'a> /*trait*/ QStyle_proxy<QStyle> for () {
  fn proxy(self , rsthis: &mut QStyle) -> QStyle {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QStyle5proxyEv()};
    let mut ret = unsafe {_ZNK6QStyle5proxyEv(rsthis.qclsinst)};
    let mut ret1 = QStyle{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QPalette QStyle::standardPalette();
impl /*struct*/ QStyle {
  pub fn standardPalette<RetType, T: QStyle_standardPalette<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.standardPalette(self);
    // return 1;
  }
}

pub trait QStyle_standardPalette<RetType> {
  fn standardPalette(self , rsthis: &mut QStyle) -> RetType;
}

  // proto:  QPalette QStyle::standardPalette();
impl<'a> /*trait*/ QStyle_standardPalette<QPalette> for () {
  fn standardPalette(self , rsthis: &mut QStyle) -> QPalette {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QStyle15standardPaletteEv()};
    let mut ret = unsafe {_ZNK6QStyle15standardPaletteEv(rsthis.qclsinst)};
    let mut ret1 = QPalette{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  const QMetaObject * QStyle::metaObject();
impl /*struct*/ QStyle {
  pub fn metaObject<RetType, T: QStyle_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QStyle_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QStyle) -> RetType;
}

  // proto:  const QMetaObject * QStyle::metaObject();
impl<'a> /*trait*/ QStyle_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QStyle) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QStyle10metaObjectEv()};
     unsafe {_ZNK6QStyle10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QStyle::polish(QApplication * );
impl<'a> /*trait*/ QStyle_polish<()> for (QApplication) {
  fn polish(self , rsthis: &mut QStyle) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QStyle6polishEP12QApplication()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QStyle6polishEP12QApplication(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QStyle::drawItemPixmap(QPainter * painter, const QRect & rect, int alignment, const QPixmap & pixmap);
impl /*struct*/ QStyle {
  pub fn drawItemPixmap<RetType, T: QStyle_drawItemPixmap<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.drawItemPixmap(self);
    // return 1;
  }
}

pub trait QStyle_drawItemPixmap<RetType> {
  fn drawItemPixmap(self , rsthis: &mut QStyle) -> RetType;
}

  // proto:  void QStyle::drawItemPixmap(QPainter * painter, const QRect & rect, int alignment, const QPixmap & pixmap);
impl<'a> /*trait*/ QStyle_drawItemPixmap<()> for (QPainter, QRect, i32, QPixmap) {
  fn drawItemPixmap(self , rsthis: &mut QStyle) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QStyle14drawItemPixmapEP8QPainterRK5QRectiRK7QPixmap()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
    let arg3 = self.3.qclsinst  as *mut c_void;
     unsafe {_ZNK6QStyle14drawItemPixmapEP8QPainterRK5QRectiRK7QPixmap(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  void QStyle::polish(QWidget * );
impl<'a> /*trait*/ QStyle_polish<()> for (QWidget) {
  fn polish(self , rsthis: &mut QStyle) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QStyle6polishEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QStyle6polishEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto: static int QStyle::sliderPositionFromValue(int min, int max, int val, int space, bool upsideDown);
impl /*struct*/ QStyle {
  pub fn sliderPositionFromValue_s<RetType, T: QStyle_sliderPositionFromValue_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.sliderPositionFromValue_s();
    // return 1;
  }
}

pub trait QStyle_sliderPositionFromValue_s<RetType> {
  fn sliderPositionFromValue_s(self ) -> RetType;
}

  // proto: static int QStyle::sliderPositionFromValue(int min, int max, int val, int space, bool upsideDown);
impl<'a> /*trait*/ QStyle_sliderPositionFromValue_s<i32> for (i32, i32, i32, i32, i8) {
  fn sliderPositionFromValue_s(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QStyle23sliderPositionFromValueEiiiib()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    let arg4 = self.4  as c_char;
    let mut ret = unsafe {_ZN6QStyle23sliderPositionFromValueEiiiib(arg0, arg1, arg2, arg3, arg4)};
    return ret as i32;
    // return 1;
  }
}

  // proto: static int QStyle::sliderValueFromPosition(int min, int max, int pos, int space, bool upsideDown);
impl /*struct*/ QStyle {
  pub fn sliderValueFromPosition_s<RetType, T: QStyle_sliderValueFromPosition_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.sliderValueFromPosition_s();
    // return 1;
  }
}

pub trait QStyle_sliderValueFromPosition_s<RetType> {
  fn sliderValueFromPosition_s(self ) -> RetType;
}

  // proto: static int QStyle::sliderValueFromPosition(int min, int max, int pos, int space, bool upsideDown);
impl<'a> /*trait*/ QStyle_sliderValueFromPosition_s<i32> for (i32, i32, i32, i32, i8) {
  fn sliderValueFromPosition_s(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QStyle23sliderValueFromPositionEiiiib()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    let arg4 = self.4  as c_char;
    let mut ret = unsafe {_ZN6QStyle23sliderValueFromPositionEiiiib(arg0, arg1, arg2, arg3, arg4)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QStyle::unpolish(QApplication * );
impl<'a> /*trait*/ QStyle_unpolish<()> for (QApplication) {
  fn unpolish(self , rsthis: &mut QStyle) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QStyle8unpolishEP12QApplication()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QStyle8unpolishEP12QApplication(rsthis.qclsinst, arg0)};
    // return 1;
  }
}
