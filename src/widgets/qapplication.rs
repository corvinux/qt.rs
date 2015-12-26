// auto generated, do not modify.
// created: Sat Dec 26 10:16:52 2015
// src-file: /QtWidgets/qapplication.h
// dst-file: /src/widgets/qapplication.rs
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
use super::super::gui::qguiapplication::QGuiApplication; // 771
use std::ops::Deref;
use super::super::core::qstring::QString; // 771
use super::super::gui::qpalette::QPalette; // 771
use super::qwidget::QWidget; // 773
use super::super::gui::qfontmetrics::QFontMetrics; // 771
use super::super::gui::qfont::QFont; // 771
use super::qstyle::QStyle; // 773
use super::super::core::qpoint::QPoint; // 771
use super::qdesktopwidget::QDesktopWidget; // 773
use super::super::core::qsize::QSize; // 771
use super::super::gui::qicon::QIcon; // 771
use super::super::core::qobject::QObject; // 771
use super::super::core::qcoreevent::QEvent; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QApplication_Class_Size() -> c_int;
  // proto:  QString QApplication::styleSheet();
  fn _ZNK12QApplication10styleSheetEv(qthis: *mut c_void) -> *mut c_void;
  // proto: static QPalette QApplication::palette(const char * className);
  fn _ZN12QApplication7paletteEPKc(arg0: *mut c_char) -> *mut c_void;
  // proto: static QWidget * QApplication::activeWindow();
  fn _ZN12QApplication12activeWindowEv() -> *mut c_void;
  // proto: static void QApplication::setKeyboardInputInterval(int );
  fn _ZN12QApplication24setKeyboardInputIntervalEi(arg0: c_int);
  // proto: static QWidget * QApplication::focusWidget();
  fn _ZN12QApplication11focusWidgetEv() -> *mut c_void;
  // proto: static QFontMetrics QApplication::fontMetrics();
  fn _ZN12QApplication11fontMetricsEv() -> *mut c_void;
  // proto: static QFont QApplication::font(const char * className);
  fn _ZN12QApplication4fontEPKc(arg0: *mut c_char) -> *mut c_void;
  // proto: static QStyle * QApplication::style();
  fn _ZN12QApplication5styleEv() -> *mut c_void;
  // proto: static QWidget * QApplication::widgetAt(const QPoint & p);
  fn _ZN12QApplication8widgetAtERK6QPoint(arg0: *mut c_void) -> *mut c_void;
  // proto: static void QApplication::setActiveWindow(QWidget * act);
  fn _ZN12QApplication15setActiveWindowEP7QWidget(arg0: *mut c_void);
  // proto: static QFont QApplication::font();
  fn _ZN12QApplication4fontEv() -> *mut c_void;
  // proto: static void QApplication::setWheelScrollLines(int );
  fn _ZN12QApplication19setWheelScrollLinesEi(arg0: c_int);
  // proto:  void QApplication::setStyleSheet(const QString & sheet);
  fn _ZN12QApplication13setStyleSheetERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QApplication::setAutoSipEnabled(const bool enabled);
  fn _ZN12QApplication17setAutoSipEnabledEb(qthis: *mut c_void, arg0: c_char);
  // proto:  const QMetaObject * QApplication::metaObject();
  fn _ZNK12QApplication10metaObjectEv(qthis: *mut c_void);
  // proto: static int QApplication::keyboardInputInterval();
  fn _ZN12QApplication21keyboardInputIntervalEv() -> c_int;
  // proto: static int QApplication::cursorFlashTime();
  fn _ZN12QApplication15cursorFlashTimeEv() -> c_int;
  // proto: static QDesktopWidget * QApplication::desktop();
  fn _ZN12QApplication7desktopEv() -> *mut c_void;
  // proto: static void QApplication::setStartDragDistance(int l);
  fn _ZN12QApplication20setStartDragDistanceEi(arg0: c_int);
  // proto: static int QApplication::colorSpec();
  fn _ZN12QApplication9colorSpecEv() -> c_int;
  // proto: static void QApplication::setFont(const QFont & , const char * className);
  fn _ZN12QApplication7setFontERK5QFontPKc(arg0: *mut c_void, arg1: *mut c_char);
  // proto: static void QApplication::closeAllWindows();
  fn _ZN12QApplication15closeAllWindowsEv();
  // proto: static void QApplication::setCursorFlashTime(int );
  fn _ZN12QApplication18setCursorFlashTimeEi(arg0: c_int);
  // proto: static void QApplication::setStartDragTime(int ms);
  fn _ZN12QApplication16setStartDragTimeEi(arg0: c_int);
  // proto: static void QApplication::alert(QWidget * widget, int duration);
  fn _ZN12QApplication5alertEP7QWidgeti(arg0: *mut c_void, arg1: c_int);
  // proto: static QPalette QApplication::palette(const QWidget * );
  fn _ZN12QApplication7paletteEPK7QWidget(arg0: *mut c_void) -> *mut c_void;
  // proto: static int QApplication::wheelScrollLines();
  fn _ZN12QApplication16wheelScrollLinesEv() -> c_int;
  // proto:  void QApplication::QApplication(const QApplication & );
  fn dector_ZN12QApplicationC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN12QApplicationC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto: static void QApplication::aboutQt();
  fn _ZN12QApplication7aboutQtEv();
  // proto: static QWidget * QApplication::activeModalWidget();
  fn _ZN12QApplication17activeModalWidgetEv() -> *mut c_void;
  // proto: static QWidget * QApplication::activePopupWidget();
  fn _ZN12QApplication17activePopupWidgetEv() -> *mut c_void;
  // proto:  void QApplication::QApplication(int & argc, char ** argv, int );
  fn dector_ZN12QApplicationC1ERiPPci(arg0: *mut c_int, arg1: *mut c_char, arg2: c_int) -> *mut c_void;
  fn _ZN12QApplicationC1ERiPPci(qthis: *mut c_void, arg0: *mut c_int, arg1: *mut c_char, arg2: c_int);
  // proto: static QFont QApplication::font(const QWidget * );
  fn _ZN12QApplication4fontEPK7QWidget(arg0: *mut c_void) -> *mut c_void;
  // proto:  void QApplication::focusChanged(QWidget * old, QWidget * now);
  fn _ZN12QApplication12focusChangedEP7QWidgetS1_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto: static int QApplication::startDragDistance();
  fn _ZN12QApplication17startDragDistanceEv() -> c_int;
  // proto: static void QApplication::setStyle(QStyle * );
  fn _ZN12QApplication8setStyleEP6QStyle(arg0: *mut c_void);
  // proto:  void QApplication::~QApplication();
  fn _ZN12QApplicationD0Ev(qthis: *mut c_void);
  // proto: static void QApplication::setDoubleClickInterval(int );
  fn _ZN12QApplication22setDoubleClickIntervalEi(arg0: c_int);
  // proto: static void QApplication::setGlobalStrut(const QSize & );
  fn _ZN12QApplication14setGlobalStrutERK5QSize(arg0: *mut c_void);
  // proto: static void QApplication::setColorSpec(int );
  fn _ZN12QApplication12setColorSpecEi(arg0: c_int);
  // proto: static QWidgetList QApplication::allWidgets();
  fn _ZN12QApplication10allWidgetsEv();
  // proto: static QSize QApplication::globalStrut();
  fn _ZN12QApplication11globalStrutEv() -> *mut c_void;
  // proto: static void QApplication::setPalette(const QPalette & , const char * className);
  fn _ZN12QApplication10setPaletteERK8QPalettePKc(arg0: *mut c_void, arg1: *mut c_char);
  // proto: static QStyle * QApplication::setStyle(const QString & );
  fn _ZN12QApplication8setStyleERK7QString(arg0: *mut c_void) -> *mut c_void;
  // proto: static QWidgetList QApplication::topLevelWidgets();
  fn _ZN12QApplication15topLevelWidgetsEv();
  // proto: static int QApplication::exec();
  fn _ZN12QApplication4execEv() -> c_int;
  // proto: static void QApplication::setWindowIcon(const QIcon & icon);
  fn _ZN12QApplication13setWindowIconERK5QIcon(arg0: *mut c_void);
  // proto: static void QApplication::beep();
  fn _ZN12QApplication4beepEv();
  // proto:  bool QApplication::notify(QObject * , QEvent * );
  fn _ZN12QApplication6notifyEP7QObjectP6QEvent(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> c_char;
  // proto:  bool QApplication::autoSipEnabled();
  fn _ZNK12QApplication14autoSipEnabledEv(qthis: *mut c_void) -> c_char;
  // proto: static QWidget * QApplication::topLevelAt(const QPoint & p);
  fn _ZN12QApplication10topLevelAtERK6QPoint(arg0: *mut c_void) -> *mut c_void;
  // proto: static int QApplication::startDragTime();
  fn _ZN12QApplication13startDragTimeEv() -> c_int;
  // proto: static int QApplication::doubleClickInterval();
  fn _ZN12QApplication19doubleClickIntervalEv() -> c_int;
  // proto: static QIcon QApplication::windowIcon();
  fn _ZN12QApplication10windowIconEv() -> *mut c_void;
} // <= ext block end

// body block begin =>
// class sizeof(QApplication)=1
pub struct QApplication {
  qbase: QGuiApplication,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QApplication {
  pub fn inheritFrom(qthis: *mut c_void) -> QApplication {
    return QApplication{qbase: QGuiApplication::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QApplication {
  type Target = QGuiApplication;

  fn deref(&self) -> &QGuiApplication {
    return & self.qbase;
  }
}
impl AsRef<QGuiApplication> for QApplication {
  fn as_ref(& self) -> & QGuiApplication {
    return & self.qbase;
  }
}
  // proto:  QString QApplication::styleSheet();
impl /*struct*/ QApplication {
  pub fn styleSheet<RetType, T: QApplication_styleSheet<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.styleSheet(self);
    // return 1;
  }
}

pub trait QApplication_styleSheet<RetType> {
  fn styleSheet(self , rsthis: & QApplication) -> RetType;
}

  // proto:  QString QApplication::styleSheet();
impl<'a> /*trait*/ QApplication_styleSheet<QString> for () {
  fn styleSheet(self , rsthis: & QApplication) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QApplication10styleSheetEv()};
    let mut ret = unsafe {_ZNK12QApplication10styleSheetEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto: static QPalette QApplication::palette(const char * className);
impl /*struct*/ QApplication {
  pub fn palette_s<RetType, T: QApplication_palette_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.palette_s();
    // return 1;
  }
}

pub trait QApplication_palette_s<RetType> {
  fn palette_s(self ) -> RetType;
}

  // proto: static QPalette QApplication::palette(const char * className);
impl<'a> /*trait*/ QApplication_palette_s<QPalette> for (&'a  String) {
  fn palette_s(self ) -> QPalette {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication7paletteEPKc()};
    let arg0 = self.as_ptr()  as *mut c_char;
    let mut ret = unsafe {_ZN12QApplication7paletteEPKc(arg0)};
    let mut ret1 = QPalette::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto: static QWidget * QApplication::activeWindow();
impl /*struct*/ QApplication {
  pub fn activeWindow_s<RetType, T: QApplication_activeWindow_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.activeWindow_s();
    // return 1;
  }
}

pub trait QApplication_activeWindow_s<RetType> {
  fn activeWindow_s(self ) -> RetType;
}

  // proto: static QWidget * QApplication::activeWindow();
impl<'a> /*trait*/ QApplication_activeWindow_s<QWidget> for () {
  fn activeWindow_s(self ) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication12activeWindowEv()};
    let mut ret = unsafe {_ZN12QApplication12activeWindowEv()};
    let mut ret1 = QWidget::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto: static void QApplication::setKeyboardInputInterval(int );
impl /*struct*/ QApplication {
  pub fn setKeyboardInputInterval_s<RetType, T: QApplication_setKeyboardInputInterval_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setKeyboardInputInterval_s();
    // return 1;
  }
}

pub trait QApplication_setKeyboardInputInterval_s<RetType> {
  fn setKeyboardInputInterval_s(self ) -> RetType;
}

  // proto: static void QApplication::setKeyboardInputInterval(int );
impl<'a> /*trait*/ QApplication_setKeyboardInputInterval_s<()> for (i32) {
  fn setKeyboardInputInterval_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication24setKeyboardInputIntervalEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN12QApplication24setKeyboardInputIntervalEi(arg0)};
    // return 1;
  }
}

  // proto: static QWidget * QApplication::focusWidget();
impl /*struct*/ QApplication {
  pub fn focusWidget_s<RetType, T: QApplication_focusWidget_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.focusWidget_s();
    // return 1;
  }
}

pub trait QApplication_focusWidget_s<RetType> {
  fn focusWidget_s(self ) -> RetType;
}

  // proto: static QWidget * QApplication::focusWidget();
impl<'a> /*trait*/ QApplication_focusWidget_s<QWidget> for () {
  fn focusWidget_s(self ) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication11focusWidgetEv()};
    let mut ret = unsafe {_ZN12QApplication11focusWidgetEv()};
    let mut ret1 = QWidget::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto: static QFontMetrics QApplication::fontMetrics();
impl /*struct*/ QApplication {
  pub fn fontMetrics_s<RetType, T: QApplication_fontMetrics_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.fontMetrics_s();
    // return 1;
  }
}

pub trait QApplication_fontMetrics_s<RetType> {
  fn fontMetrics_s(self ) -> RetType;
}

  // proto: static QFontMetrics QApplication::fontMetrics();
impl<'a> /*trait*/ QApplication_fontMetrics_s<QFontMetrics> for () {
  fn fontMetrics_s(self ) -> QFontMetrics {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication11fontMetricsEv()};
    let mut ret = unsafe {_ZN12QApplication11fontMetricsEv()};
    let mut ret1 = QFontMetrics::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto: static QFont QApplication::font(const char * className);
impl /*struct*/ QApplication {
  pub fn font_s<RetType, T: QApplication_font_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.font_s();
    // return 1;
  }
}

pub trait QApplication_font_s<RetType> {
  fn font_s(self ) -> RetType;
}

  // proto: static QFont QApplication::font(const char * className);
impl<'a> /*trait*/ QApplication_font_s<QFont> for (&'a  String) {
  fn font_s(self ) -> QFont {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication4fontEPKc()};
    let arg0 = self.as_ptr()  as *mut c_char;
    let mut ret = unsafe {_ZN12QApplication4fontEPKc(arg0)};
    let mut ret1 = QFont::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto: static QStyle * QApplication::style();
impl /*struct*/ QApplication {
  pub fn style_s<RetType, T: QApplication_style_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.style_s();
    // return 1;
  }
}

pub trait QApplication_style_s<RetType> {
  fn style_s(self ) -> RetType;
}

  // proto: static QStyle * QApplication::style();
impl<'a> /*trait*/ QApplication_style_s<QStyle> for () {
  fn style_s(self ) -> QStyle {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication5styleEv()};
    let mut ret = unsafe {_ZN12QApplication5styleEv()};
    let mut ret1 = QStyle::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto: static QWidget * QApplication::widgetAt(const QPoint & p);
impl /*struct*/ QApplication {
  pub fn widgetAt_s<RetType, T: QApplication_widgetAt_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.widgetAt_s();
    // return 1;
  }
}

pub trait QApplication_widgetAt_s<RetType> {
  fn widgetAt_s(self ) -> RetType;
}

  // proto: static QWidget * QApplication::widgetAt(const QPoint & p);
impl<'a> /*trait*/ QApplication_widgetAt_s<QWidget> for (&'a QPoint) {
  fn widgetAt_s(self ) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication8widgetAtERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN12QApplication8widgetAtERK6QPoint(arg0)};
    let mut ret1 = QWidget::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto: static void QApplication::setActiveWindow(QWidget * act);
impl /*struct*/ QApplication {
  pub fn setActiveWindow_s<RetType, T: QApplication_setActiveWindow_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setActiveWindow_s();
    // return 1;
  }
}

pub trait QApplication_setActiveWindow_s<RetType> {
  fn setActiveWindow_s(self ) -> RetType;
}

  // proto: static void QApplication::setActiveWindow(QWidget * act);
impl<'a> /*trait*/ QApplication_setActiveWindow_s<()> for (&'a QWidget) {
  fn setActiveWindow_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication15setActiveWindowEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QApplication15setActiveWindowEP7QWidget(arg0)};
    // return 1;
  }
}

  // proto: static QFont QApplication::font();
impl<'a> /*trait*/ QApplication_font_s<QFont> for () {
  fn font_s(self ) -> QFont {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication4fontEv()};
    let mut ret = unsafe {_ZN12QApplication4fontEv()};
    let mut ret1 = QFont::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto: static void QApplication::setWheelScrollLines(int );
impl /*struct*/ QApplication {
  pub fn setWheelScrollLines_s<RetType, T: QApplication_setWheelScrollLines_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setWheelScrollLines_s();
    // return 1;
  }
}

pub trait QApplication_setWheelScrollLines_s<RetType> {
  fn setWheelScrollLines_s(self ) -> RetType;
}

  // proto: static void QApplication::setWheelScrollLines(int );
impl<'a> /*trait*/ QApplication_setWheelScrollLines_s<()> for (i32) {
  fn setWheelScrollLines_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication19setWheelScrollLinesEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN12QApplication19setWheelScrollLinesEi(arg0)};
    // return 1;
  }
}

  // proto:  void QApplication::setStyleSheet(const QString & sheet);
impl /*struct*/ QApplication {
  pub fn setStyleSheet<RetType, T: QApplication_setStyleSheet<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setStyleSheet(self);
    // return 1;
  }
}

pub trait QApplication_setStyleSheet<RetType> {
  fn setStyleSheet(self , rsthis: & QApplication) -> RetType;
}

  // proto:  void QApplication::setStyleSheet(const QString & sheet);
impl<'a> /*trait*/ QApplication_setStyleSheet<()> for (&'a QString) {
  fn setStyleSheet(self , rsthis: & QApplication) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication13setStyleSheetERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QApplication13setStyleSheetERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QApplication::setAutoSipEnabled(const bool enabled);
impl /*struct*/ QApplication {
  pub fn setAutoSipEnabled<RetType, T: QApplication_setAutoSipEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setAutoSipEnabled(self);
    // return 1;
  }
}

pub trait QApplication_setAutoSipEnabled<RetType> {
  fn setAutoSipEnabled(self , rsthis: & QApplication) -> RetType;
}

  // proto:  void QApplication::setAutoSipEnabled(const bool enabled);
impl<'a> /*trait*/ QApplication_setAutoSipEnabled<()> for (i8) {
  fn setAutoSipEnabled(self , rsthis: & QApplication) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication17setAutoSipEnabledEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN12QApplication17setAutoSipEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QApplication::metaObject();
impl /*struct*/ QApplication {
  pub fn metaObject<RetType, T: QApplication_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QApplication_metaObject<RetType> {
  fn metaObject(self , rsthis: & QApplication) -> RetType;
}

  // proto:  const QMetaObject * QApplication::metaObject();
impl<'a> /*trait*/ QApplication_metaObject<()> for () {
  fn metaObject(self , rsthis: & QApplication) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QApplication10metaObjectEv()};
     unsafe {_ZNK12QApplication10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto: static int QApplication::keyboardInputInterval();
impl /*struct*/ QApplication {
  pub fn keyboardInputInterval_s<RetType, T: QApplication_keyboardInputInterval_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.keyboardInputInterval_s();
    // return 1;
  }
}

pub trait QApplication_keyboardInputInterval_s<RetType> {
  fn keyboardInputInterval_s(self ) -> RetType;
}

  // proto: static int QApplication::keyboardInputInterval();
impl<'a> /*trait*/ QApplication_keyboardInputInterval_s<i32> for () {
  fn keyboardInputInterval_s(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication21keyboardInputIntervalEv()};
    let mut ret = unsafe {_ZN12QApplication21keyboardInputIntervalEv()};
    return ret as i32;
    // return 1;
  }
}

  // proto: static int QApplication::cursorFlashTime();
impl /*struct*/ QApplication {
  pub fn cursorFlashTime_s<RetType, T: QApplication_cursorFlashTime_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.cursorFlashTime_s();
    // return 1;
  }
}

pub trait QApplication_cursorFlashTime_s<RetType> {
  fn cursorFlashTime_s(self ) -> RetType;
}

  // proto: static int QApplication::cursorFlashTime();
impl<'a> /*trait*/ QApplication_cursorFlashTime_s<i32> for () {
  fn cursorFlashTime_s(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication15cursorFlashTimeEv()};
    let mut ret = unsafe {_ZN12QApplication15cursorFlashTimeEv()};
    return ret as i32;
    // return 1;
  }
}

  // proto: static QDesktopWidget * QApplication::desktop();
impl /*struct*/ QApplication {
  pub fn desktop_s<RetType, T: QApplication_desktop_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.desktop_s();
    // return 1;
  }
}

pub trait QApplication_desktop_s<RetType> {
  fn desktop_s(self ) -> RetType;
}

  // proto: static QDesktopWidget * QApplication::desktop();
impl<'a> /*trait*/ QApplication_desktop_s<QDesktopWidget> for () {
  fn desktop_s(self ) -> QDesktopWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication7desktopEv()};
    let mut ret = unsafe {_ZN12QApplication7desktopEv()};
    let mut ret1 = QDesktopWidget::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto: static void QApplication::setStartDragDistance(int l);
impl /*struct*/ QApplication {
  pub fn setStartDragDistance_s<RetType, T: QApplication_setStartDragDistance_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setStartDragDistance_s();
    // return 1;
  }
}

pub trait QApplication_setStartDragDistance_s<RetType> {
  fn setStartDragDistance_s(self ) -> RetType;
}

  // proto: static void QApplication::setStartDragDistance(int l);
impl<'a> /*trait*/ QApplication_setStartDragDistance_s<()> for (i32) {
  fn setStartDragDistance_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication20setStartDragDistanceEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN12QApplication20setStartDragDistanceEi(arg0)};
    // return 1;
  }
}

  // proto: static int QApplication::colorSpec();
impl /*struct*/ QApplication {
  pub fn colorSpec_s<RetType, T: QApplication_colorSpec_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.colorSpec_s();
    // return 1;
  }
}

pub trait QApplication_colorSpec_s<RetType> {
  fn colorSpec_s(self ) -> RetType;
}

  // proto: static int QApplication::colorSpec();
impl<'a> /*trait*/ QApplication_colorSpec_s<i32> for () {
  fn colorSpec_s(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication9colorSpecEv()};
    let mut ret = unsafe {_ZN12QApplication9colorSpecEv()};
    return ret as i32;
    // return 1;
  }
}

  // proto: static void QApplication::setFont(const QFont & , const char * className);
impl /*struct*/ QApplication {
  pub fn setFont_s<RetType, T: QApplication_setFont_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setFont_s();
    // return 1;
  }
}

pub trait QApplication_setFont_s<RetType> {
  fn setFont_s(self ) -> RetType;
}

  // proto: static void QApplication::setFont(const QFont & , const char * className);
impl<'a> /*trait*/ QApplication_setFont_s<()> for (&'a QFont, &'a  String) {
  fn setFont_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication7setFontERK5QFontPKc()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *mut c_char;
     unsafe {_ZN12QApplication7setFontERK5QFontPKc(arg0, arg1)};
    // return 1;
  }
}

  // proto: static void QApplication::closeAllWindows();
impl /*struct*/ QApplication {
  pub fn closeAllWindows_s<RetType, T: QApplication_closeAllWindows_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.closeAllWindows_s();
    // return 1;
  }
}

pub trait QApplication_closeAllWindows_s<RetType> {
  fn closeAllWindows_s(self ) -> RetType;
}

  // proto: static void QApplication::closeAllWindows();
impl<'a> /*trait*/ QApplication_closeAllWindows_s<()> for () {
  fn closeAllWindows_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication15closeAllWindowsEv()};
     unsafe {_ZN12QApplication15closeAllWindowsEv()};
    // return 1;
  }
}

  // proto: static void QApplication::setCursorFlashTime(int );
impl /*struct*/ QApplication {
  pub fn setCursorFlashTime_s<RetType, T: QApplication_setCursorFlashTime_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setCursorFlashTime_s();
    // return 1;
  }
}

pub trait QApplication_setCursorFlashTime_s<RetType> {
  fn setCursorFlashTime_s(self ) -> RetType;
}

  // proto: static void QApplication::setCursorFlashTime(int );
impl<'a> /*trait*/ QApplication_setCursorFlashTime_s<()> for (i32) {
  fn setCursorFlashTime_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication18setCursorFlashTimeEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN12QApplication18setCursorFlashTimeEi(arg0)};
    // return 1;
  }
}

  // proto: static void QApplication::setStartDragTime(int ms);
impl /*struct*/ QApplication {
  pub fn setStartDragTime_s<RetType, T: QApplication_setStartDragTime_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setStartDragTime_s();
    // return 1;
  }
}

pub trait QApplication_setStartDragTime_s<RetType> {
  fn setStartDragTime_s(self ) -> RetType;
}

  // proto: static void QApplication::setStartDragTime(int ms);
impl<'a> /*trait*/ QApplication_setStartDragTime_s<()> for (i32) {
  fn setStartDragTime_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication16setStartDragTimeEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN12QApplication16setStartDragTimeEi(arg0)};
    // return 1;
  }
}

  // proto: static void QApplication::alert(QWidget * widget, int duration);
impl /*struct*/ QApplication {
  pub fn alert_s<RetType, T: QApplication_alert_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.alert_s();
    // return 1;
  }
}

pub trait QApplication_alert_s<RetType> {
  fn alert_s(self ) -> RetType;
}

  // proto: static void QApplication::alert(QWidget * widget, int duration);
impl<'a> /*trait*/ QApplication_alert_s<()> for (&'a QWidget, i32) {
  fn alert_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication5alertEP7QWidgeti()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN12QApplication5alertEP7QWidgeti(arg0, arg1)};
    // return 1;
  }
}

  // proto: static QPalette QApplication::palette(const QWidget * );
impl<'a> /*trait*/ QApplication_palette_s<QPalette> for (&'a QWidget) {
  fn palette_s(self ) -> QPalette {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication7paletteEPK7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN12QApplication7paletteEPK7QWidget(arg0)};
    let mut ret1 = QPalette::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto: static int QApplication::wheelScrollLines();
impl /*struct*/ QApplication {
  pub fn wheelScrollLines_s<RetType, T: QApplication_wheelScrollLines_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.wheelScrollLines_s();
    // return 1;
  }
}

pub trait QApplication_wheelScrollLines_s<RetType> {
  fn wheelScrollLines_s(self ) -> RetType;
}

  // proto: static int QApplication::wheelScrollLines();
impl<'a> /*trait*/ QApplication_wheelScrollLines_s<i32> for () {
  fn wheelScrollLines_s(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication16wheelScrollLinesEv()};
    let mut ret = unsafe {_ZN12QApplication16wheelScrollLinesEv()};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QApplication::QApplication(const QApplication & );
impl /*struct*/ QApplication {
  pub fn New<T: QApplication_New>(value: T) -> QApplication {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QApplication_New {
  fn New(self) -> QApplication;
}

  // proto:  void QApplication::QApplication(const QApplication & );
impl<'a> /*trait*/ QApplication_New for (&'a QApplication) {
  fn New(self) -> QApplication {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplicationC1ERKS_()};
    let ctysz: c_int = unsafe{QApplication_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN12QApplicationC1ERKS_(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN12QApplicationC1ERKS_(arg0)};
    let rsthis = QApplication{/**/qbase: QGuiApplication::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto: static void QApplication::aboutQt();
impl /*struct*/ QApplication {
  pub fn aboutQt_s<RetType, T: QApplication_aboutQt_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.aboutQt_s();
    // return 1;
  }
}

pub trait QApplication_aboutQt_s<RetType> {
  fn aboutQt_s(self ) -> RetType;
}

  // proto: static void QApplication::aboutQt();
impl<'a> /*trait*/ QApplication_aboutQt_s<()> for () {
  fn aboutQt_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication7aboutQtEv()};
     unsafe {_ZN12QApplication7aboutQtEv()};
    // return 1;
  }
}

  // proto: static QWidget * QApplication::activeModalWidget();
impl /*struct*/ QApplication {
  pub fn activeModalWidget_s<RetType, T: QApplication_activeModalWidget_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.activeModalWidget_s();
    // return 1;
  }
}

pub trait QApplication_activeModalWidget_s<RetType> {
  fn activeModalWidget_s(self ) -> RetType;
}

  // proto: static QWidget * QApplication::activeModalWidget();
impl<'a> /*trait*/ QApplication_activeModalWidget_s<QWidget> for () {
  fn activeModalWidget_s(self ) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication17activeModalWidgetEv()};
    let mut ret = unsafe {_ZN12QApplication17activeModalWidgetEv()};
    let mut ret1 = QWidget::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto: static QWidget * QApplication::activePopupWidget();
impl /*struct*/ QApplication {
  pub fn activePopupWidget_s<RetType, T: QApplication_activePopupWidget_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.activePopupWidget_s();
    // return 1;
  }
}

pub trait QApplication_activePopupWidget_s<RetType> {
  fn activePopupWidget_s(self ) -> RetType;
}

  // proto: static QWidget * QApplication::activePopupWidget();
impl<'a> /*trait*/ QApplication_activePopupWidget_s<QWidget> for () {
  fn activePopupWidget_s(self ) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication17activePopupWidgetEv()};
    let mut ret = unsafe {_ZN12QApplication17activePopupWidgetEv()};
    let mut ret1 = QWidget::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QApplication::QApplication(int & argc, char ** argv, int );
impl<'a> /*trait*/ QApplication_New for (&'a mut i32, &'a mut String, i32) {
  fn New(self) -> QApplication {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplicationC1ERiPPci()};
    let ctysz: c_int = unsafe{QApplication_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.0  as *mut c_int;
    let arg1 = self.1.as_ptr()  as *mut c_char;
    let arg2 = self.2  as c_int;
    // unsafe {_ZN12QApplicationC1ERiPPci(qthis, arg0, arg1, arg2)};
    let qthis: *mut c_void = unsafe {dector_ZN12QApplicationC1ERiPPci(arg0, arg1, arg2)};
    let rsthis = QApplication{/**/qbase: QGuiApplication::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto: static QFont QApplication::font(const QWidget * );
impl<'a> /*trait*/ QApplication_font_s<QFont> for (&'a QWidget) {
  fn font_s(self ) -> QFont {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication4fontEPK7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN12QApplication4fontEPK7QWidget(arg0)};
    let mut ret1 = QFont::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QApplication::focusChanged(QWidget * old, QWidget * now);
impl /*struct*/ QApplication {
  pub fn focusChanged<RetType, T: QApplication_focusChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.focusChanged(self);
    // return 1;
  }
}

pub trait QApplication_focusChanged<RetType> {
  fn focusChanged(self , rsthis: & QApplication) -> RetType;
}

  // proto:  void QApplication::focusChanged(QWidget * old, QWidget * now);
impl<'a> /*trait*/ QApplication_focusChanged<()> for (&'a QWidget, &'a QWidget) {
  fn focusChanged(self , rsthis: & QApplication) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication12focusChangedEP7QWidgetS1_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN12QApplication12focusChangedEP7QWidgetS1_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto: static int QApplication::startDragDistance();
impl /*struct*/ QApplication {
  pub fn startDragDistance_s<RetType, T: QApplication_startDragDistance_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.startDragDistance_s();
    // return 1;
  }
}

pub trait QApplication_startDragDistance_s<RetType> {
  fn startDragDistance_s(self ) -> RetType;
}

  // proto: static int QApplication::startDragDistance();
impl<'a> /*trait*/ QApplication_startDragDistance_s<i32> for () {
  fn startDragDistance_s(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication17startDragDistanceEv()};
    let mut ret = unsafe {_ZN12QApplication17startDragDistanceEv()};
    return ret as i32;
    // return 1;
  }
}

  // proto: static void QApplication::setStyle(QStyle * );
impl /*struct*/ QApplication {
  pub fn setStyle_s<RetType, T: QApplication_setStyle_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setStyle_s();
    // return 1;
  }
}

pub trait QApplication_setStyle_s<RetType> {
  fn setStyle_s(self ) -> RetType;
}

  // proto: static void QApplication::setStyle(QStyle * );
impl<'a> /*trait*/ QApplication_setStyle_s<()> for (&'a QStyle) {
  fn setStyle_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication8setStyleEP6QStyle()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QApplication8setStyleEP6QStyle(arg0)};
    // return 1;
  }
}

  // proto:  void QApplication::~QApplication();
impl /*struct*/ QApplication {
  pub fn Free<RetType, T: QApplication_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QApplication_Free<RetType> {
  fn Free(self , rsthis: & QApplication) -> RetType;
}

  // proto:  void QApplication::~QApplication();
impl<'a> /*trait*/ QApplication_Free<()> for () {
  fn Free(self , rsthis: & QApplication) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplicationD0Ev()};
     unsafe {_ZN12QApplicationD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto: static void QApplication::setDoubleClickInterval(int );
impl /*struct*/ QApplication {
  pub fn setDoubleClickInterval_s<RetType, T: QApplication_setDoubleClickInterval_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setDoubleClickInterval_s();
    // return 1;
  }
}

pub trait QApplication_setDoubleClickInterval_s<RetType> {
  fn setDoubleClickInterval_s(self ) -> RetType;
}

  // proto: static void QApplication::setDoubleClickInterval(int );
impl<'a> /*trait*/ QApplication_setDoubleClickInterval_s<()> for (i32) {
  fn setDoubleClickInterval_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication22setDoubleClickIntervalEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN12QApplication22setDoubleClickIntervalEi(arg0)};
    // return 1;
  }
}

  // proto: static void QApplication::setGlobalStrut(const QSize & );
impl /*struct*/ QApplication {
  pub fn setGlobalStrut_s<RetType, T: QApplication_setGlobalStrut_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setGlobalStrut_s();
    // return 1;
  }
}

pub trait QApplication_setGlobalStrut_s<RetType> {
  fn setGlobalStrut_s(self ) -> RetType;
}

  // proto: static void QApplication::setGlobalStrut(const QSize & );
impl<'a> /*trait*/ QApplication_setGlobalStrut_s<()> for (&'a QSize) {
  fn setGlobalStrut_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication14setGlobalStrutERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QApplication14setGlobalStrutERK5QSize(arg0)};
    // return 1;
  }
}

  // proto: static void QApplication::setColorSpec(int );
impl /*struct*/ QApplication {
  pub fn setColorSpec_s<RetType, T: QApplication_setColorSpec_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setColorSpec_s();
    // return 1;
  }
}

pub trait QApplication_setColorSpec_s<RetType> {
  fn setColorSpec_s(self ) -> RetType;
}

  // proto: static void QApplication::setColorSpec(int );
impl<'a> /*trait*/ QApplication_setColorSpec_s<()> for (i32) {
  fn setColorSpec_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication12setColorSpecEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN12QApplication12setColorSpecEi(arg0)};
    // return 1;
  }
}

  // proto: static QWidgetList QApplication::allWidgets();
impl /*struct*/ QApplication {
  pub fn allWidgets_s<RetType, T: QApplication_allWidgets_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.allWidgets_s();
    // return 1;
  }
}

pub trait QApplication_allWidgets_s<RetType> {
  fn allWidgets_s(self ) -> RetType;
}

  // proto: static QWidgetList QApplication::allWidgets();
impl<'a> /*trait*/ QApplication_allWidgets_s<()> for () {
  fn allWidgets_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication10allWidgetsEv()};
     unsafe {_ZN12QApplication10allWidgetsEv()};
    // return 1;
  }
}

  // proto: static QSize QApplication::globalStrut();
impl /*struct*/ QApplication {
  pub fn globalStrut_s<RetType, T: QApplication_globalStrut_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.globalStrut_s();
    // return 1;
  }
}

pub trait QApplication_globalStrut_s<RetType> {
  fn globalStrut_s(self ) -> RetType;
}

  // proto: static QSize QApplication::globalStrut();
impl<'a> /*trait*/ QApplication_globalStrut_s<QSize> for () {
  fn globalStrut_s(self ) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication11globalStrutEv()};
    let mut ret = unsafe {_ZN12QApplication11globalStrutEv()};
    let mut ret1 = QSize::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto: static void QApplication::setPalette(const QPalette & , const char * className);
impl /*struct*/ QApplication {
  pub fn setPalette_s<RetType, T: QApplication_setPalette_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setPalette_s();
    // return 1;
  }
}

pub trait QApplication_setPalette_s<RetType> {
  fn setPalette_s(self ) -> RetType;
}

  // proto: static void QApplication::setPalette(const QPalette & , const char * className);
impl<'a> /*trait*/ QApplication_setPalette_s<()> for (&'a QPalette, &'a  String) {
  fn setPalette_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication10setPaletteERK8QPalettePKc()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *mut c_char;
     unsafe {_ZN12QApplication10setPaletteERK8QPalettePKc(arg0, arg1)};
    // return 1;
  }
}

  // proto: static QStyle * QApplication::setStyle(const QString & );
impl<'a> /*trait*/ QApplication_setStyle_s<QStyle> for (&'a QString) {
  fn setStyle_s(self ) -> QStyle {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication8setStyleERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN12QApplication8setStyleERK7QString(arg0)};
    let mut ret1 = QStyle::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto: static QWidgetList QApplication::topLevelWidgets();
impl /*struct*/ QApplication {
  pub fn topLevelWidgets_s<RetType, T: QApplication_topLevelWidgets_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.topLevelWidgets_s();
    // return 1;
  }
}

pub trait QApplication_topLevelWidgets_s<RetType> {
  fn topLevelWidgets_s(self ) -> RetType;
}

  // proto: static QWidgetList QApplication::topLevelWidgets();
impl<'a> /*trait*/ QApplication_topLevelWidgets_s<()> for () {
  fn topLevelWidgets_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication15topLevelWidgetsEv()};
     unsafe {_ZN12QApplication15topLevelWidgetsEv()};
    // return 1;
  }
}

  // proto: static int QApplication::exec();
impl /*struct*/ QApplication {
  pub fn exec_s<RetType, T: QApplication_exec_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.exec_s();
    // return 1;
  }
}

pub trait QApplication_exec_s<RetType> {
  fn exec_s(self ) -> RetType;
}

  // proto: static int QApplication::exec();
impl<'a> /*trait*/ QApplication_exec_s<i32> for () {
  fn exec_s(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication4execEv()};
    let mut ret = unsafe {_ZN12QApplication4execEv()};
    return ret as i32;
    // return 1;
  }
}

  // proto: static void QApplication::setWindowIcon(const QIcon & icon);
impl /*struct*/ QApplication {
  pub fn setWindowIcon_s<RetType, T: QApplication_setWindowIcon_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setWindowIcon_s();
    // return 1;
  }
}

pub trait QApplication_setWindowIcon_s<RetType> {
  fn setWindowIcon_s(self ) -> RetType;
}

  // proto: static void QApplication::setWindowIcon(const QIcon & icon);
impl<'a> /*trait*/ QApplication_setWindowIcon_s<()> for (&'a QIcon) {
  fn setWindowIcon_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication13setWindowIconERK5QIcon()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QApplication13setWindowIconERK5QIcon(arg0)};
    // return 1;
  }
}

  // proto: static void QApplication::beep();
impl /*struct*/ QApplication {
  pub fn beep_s<RetType, T: QApplication_beep_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.beep_s();
    // return 1;
  }
}

pub trait QApplication_beep_s<RetType> {
  fn beep_s(self ) -> RetType;
}

  // proto: static void QApplication::beep();
impl<'a> /*trait*/ QApplication_beep_s<()> for () {
  fn beep_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication4beepEv()};
     unsafe {_ZN12QApplication4beepEv()};
    // return 1;
  }
}

  // proto:  bool QApplication::notify(QObject * , QEvent * );
impl /*struct*/ QApplication {
  pub fn notify<RetType, T: QApplication_notify<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.notify(self);
    // return 1;
  }
}

pub trait QApplication_notify<RetType> {
  fn notify(self , rsthis: & QApplication) -> RetType;
}

  // proto:  bool QApplication::notify(QObject * , QEvent * );
impl<'a> /*trait*/ QApplication_notify<i8> for (&'a QObject, &'a QEvent) {
  fn notify(self , rsthis: & QApplication) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication6notifyEP7QObjectP6QEvent()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN12QApplication6notifyEP7QObjectP6QEvent(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QApplication::autoSipEnabled();
impl /*struct*/ QApplication {
  pub fn autoSipEnabled<RetType, T: QApplication_autoSipEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.autoSipEnabled(self);
    // return 1;
  }
}

pub trait QApplication_autoSipEnabled<RetType> {
  fn autoSipEnabled(self , rsthis: & QApplication) -> RetType;
}

  // proto:  bool QApplication::autoSipEnabled();
impl<'a> /*trait*/ QApplication_autoSipEnabled<i8> for () {
  fn autoSipEnabled(self , rsthis: & QApplication) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QApplication14autoSipEnabledEv()};
    let mut ret = unsafe {_ZNK12QApplication14autoSipEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto: static QWidget * QApplication::topLevelAt(const QPoint & p);
impl /*struct*/ QApplication {
  pub fn topLevelAt_s<RetType, T: QApplication_topLevelAt_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.topLevelAt_s();
    // return 1;
  }
}

pub trait QApplication_topLevelAt_s<RetType> {
  fn topLevelAt_s(self ) -> RetType;
}

  // proto: static QWidget * QApplication::topLevelAt(const QPoint & p);
impl<'a> /*trait*/ QApplication_topLevelAt_s<QWidget> for (&'a QPoint) {
  fn topLevelAt_s(self ) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication10topLevelAtERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN12QApplication10topLevelAtERK6QPoint(arg0)};
    let mut ret1 = QWidget::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto: static int QApplication::startDragTime();
impl /*struct*/ QApplication {
  pub fn startDragTime_s<RetType, T: QApplication_startDragTime_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.startDragTime_s();
    // return 1;
  }
}

pub trait QApplication_startDragTime_s<RetType> {
  fn startDragTime_s(self ) -> RetType;
}

  // proto: static int QApplication::startDragTime();
impl<'a> /*trait*/ QApplication_startDragTime_s<i32> for () {
  fn startDragTime_s(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication13startDragTimeEv()};
    let mut ret = unsafe {_ZN12QApplication13startDragTimeEv()};
    return ret as i32;
    // return 1;
  }
}

  // proto: static int QApplication::doubleClickInterval();
impl /*struct*/ QApplication {
  pub fn doubleClickInterval_s<RetType, T: QApplication_doubleClickInterval_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.doubleClickInterval_s();
    // return 1;
  }
}

pub trait QApplication_doubleClickInterval_s<RetType> {
  fn doubleClickInterval_s(self ) -> RetType;
}

  // proto: static int QApplication::doubleClickInterval();
impl<'a> /*trait*/ QApplication_doubleClickInterval_s<i32> for () {
  fn doubleClickInterval_s(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication19doubleClickIntervalEv()};
    let mut ret = unsafe {_ZN12QApplication19doubleClickIntervalEv()};
    return ret as i32;
    // return 1;
  }
}

  // proto: static QIcon QApplication::windowIcon();
impl /*struct*/ QApplication {
  pub fn windowIcon_s<RetType, T: QApplication_windowIcon_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.windowIcon_s();
    // return 1;
  }
}

pub trait QApplication_windowIcon_s<RetType> {
  fn windowIcon_s(self ) -> RetType;
}

  // proto: static QIcon QApplication::windowIcon();
impl<'a> /*trait*/ QApplication_windowIcon_s<QIcon> for () {
  fn windowIcon_s(self ) -> QIcon {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QApplication10windowIconEv()};
    let mut ret = unsafe {_ZN12QApplication10windowIconEv()};
    let mut ret1 = QIcon::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

// <= body block end

