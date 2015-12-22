// auto generated, do not modify.
// created: Tue Dec 22 23:21:28 2015
// src-file: /QtWidgets/qtabwidget.h
// dst-file: /src/widgets/qtabwidget.rs
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
use super::qtabbar::QTabBar; // 773
use super::super::gui::qicon::QIcon; // 771
use super::super::core::qsize::QSize; // 771
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  void QTabWidget::setCurrentWidget(QWidget * widget);
  fn _ZN10QTabWidget16setCurrentWidgetEP7QWidget(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  int QTabWidget::count();
  fn _ZNK10QTabWidget5countEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTabWidget::tabCloseRequested(int index);
  fn _ZN10QTabWidget17tabCloseRequestedEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QTabWidget::setDocumentMode(bool set);
  fn _ZN10QTabWidget15setDocumentModeEb(qthis: *mut c_void, arg0: c_char);
  // proto:  int QTabWidget::heightForWidth(int width);
  fn _ZNK10QTabWidget14heightForWidthEi(qthis: *mut c_void, arg0: c_int) -> c_int;
  // proto:  int QTabWidget::addTab(QWidget * widget, const QString & );
  fn _ZN10QTabWidget6addTabEP7QWidgetRK7QString(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> c_int;
  // proto:  QString QTabWidget::tabText(int index);
  fn _ZNK10QTabWidget7tabTextEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  void QTabWidget::clear();
  fn _ZN10QTabWidget5clearEv(qthis: *mut c_void);
  // proto:  bool QTabWidget::hasHeightForWidth();
  fn _ZNK10QTabWidget17hasHeightForWidthEv(qthis: *mut c_void) -> c_char;
  // proto:  QTabBar * QTabWidget::tabBar();
  fn _ZNK10QTabWidget6tabBarEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QTabWidget::tabsClosable();
  fn _ZNK10QTabWidget12tabsClosableEv(qthis: *mut c_void) -> c_char;
  // proto:  int QTabWidget::insertTab(int index, QWidget * widget, const QIcon & icon, const QString & label);
  fn _ZN10QTabWidget9insertTabEiP7QWidgetRK5QIconRK7QString(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void, arg2: *mut c_void, arg3: *mut c_void) -> c_int;
  // proto:  int QTabWidget::addTab(QWidget * widget, const QIcon & icon, const QString & label);
  fn _ZN10QTabWidget6addTabEP7QWidgetRK5QIconRK7QString(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) -> c_int;
  // proto:  void QTabWidget::setUsesScrollButtons(bool useButtons);
  fn _ZN10QTabWidget20setUsesScrollButtonsEb(qthis: *mut c_void, arg0: c_char);
  // proto:  const QMetaObject * QTabWidget::metaObject();
  fn _ZNK10QTabWidget10metaObjectEv(qthis: *mut c_void);
  // proto:  QString QTabWidget::tabToolTip(int index);
  fn _ZNK10QTabWidget10tabToolTipEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  QWidget * QTabWidget::currentWidget();
  fn _ZNK10QTabWidget13currentWidgetEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTabWidget::setIconSize(const QSize & size);
  fn _ZN10QTabWidget11setIconSizeERK5QSize(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QWidget * QTabWidget::widget(int index);
  fn _ZNK10QTabWidget6widgetEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  void QTabWidget::setMovable(bool movable);
  fn _ZN10QTabWidget10setMovableEb(qthis: *mut c_void, arg0: c_char);
  // proto:  bool QTabWidget::documentMode();
  fn _ZNK10QTabWidget12documentModeEv(qthis: *mut c_void) -> c_char;
  // proto:  QString QTabWidget::tabWhatsThis(int index);
  fn _ZNK10QTabWidget12tabWhatsThisEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  void QTabWidget::tabBarClicked(int index);
  fn _ZN10QTabWidget13tabBarClickedEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QTabWidget::setTabText(int index, const QString & );
  fn _ZN10QTabWidget10setTabTextEiRK7QString(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void);
  // proto:  void QTabWidget::QTabWidget(const QTabWidget & );
  fn _ZN10QTabWidgetC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QTabWidget::QTabWidget(QWidget * parent);
  fn _ZN10QTabWidgetC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QTabWidget::tabBarAutoHide();
  fn _ZNK10QTabWidget14tabBarAutoHideEv(qthis: *mut c_void) -> c_char;
  // proto:  void QTabWidget::currentChanged(int index);
  fn _ZN10QTabWidget14currentChangedEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QTabWidget::setTabIcon(int index, const QIcon & icon);
  fn _ZN10QTabWidget10setTabIconEiRK5QIcon(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void);
  // proto:  QIcon QTabWidget::tabIcon(int index);
  fn _ZNK10QTabWidget7tabIconEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  bool QTabWidget::isTabEnabled(int index);
  fn _ZNK10QTabWidget12isTabEnabledEi(qthis: *mut c_void, arg0: c_int) -> c_char;
  // proto:  void QTabWidget::setTabBarAutoHide(bool enabled);
  fn _ZN10QTabWidget17setTabBarAutoHideEb(qthis: *mut c_void, arg0: c_char);
  // proto:  QSize QTabWidget::iconSize();
  fn _ZNK10QTabWidget8iconSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTabWidget::setTabsClosable(bool closeable);
  fn _ZN10QTabWidget15setTabsClosableEb(qthis: *mut c_void, arg0: c_char);
  // proto:  QSize QTabWidget::minimumSizeHint();
  fn _ZNK10QTabWidget15minimumSizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTabWidget::setCurrentIndex(int index);
  fn _ZN10QTabWidget15setCurrentIndexEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QTabWidget::~QTabWidget();
  fn _ZN10QTabWidgetD0Ev(qthis: *mut c_void);
  // proto:  void QTabWidget::setTabWhatsThis(int index, const QString & text);
  fn _ZN10QTabWidget15setTabWhatsThisEiRK7QString(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void);
  // proto:  QSize QTabWidget::sizeHint();
  fn _ZNK10QTabWidget8sizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QTabWidget::indexOf(QWidget * widget);
  fn _ZNK10QTabWidget7indexOfEP7QWidget(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto:  void QTabWidget::removeTab(int index);
  fn _ZN10QTabWidget9removeTabEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QTabWidget::setTabToolTip(int index, const QString & tip);
  fn _ZN10QTabWidget13setTabToolTipEiRK7QString(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void);
  // proto:  bool QTabWidget::isMovable();
  fn _ZNK10QTabWidget9isMovableEv(qthis: *mut c_void) -> c_char;
  // proto:  bool QTabWidget::usesScrollButtons();
  fn _ZNK10QTabWidget17usesScrollButtonsEv(qthis: *mut c_void) -> c_char;
  // proto:  void QTabWidget::tabBarDoubleClicked(int index);
  fn _ZN10QTabWidget19tabBarDoubleClickedEi(qthis: *mut c_void, arg0: c_int);
  // proto:  int QTabWidget::currentIndex();
  fn _ZNK10QTabWidget12currentIndexEv(qthis: *mut c_void) -> c_int;
  // proto:  int QTabWidget::insertTab(int index, QWidget * widget, const QString & );
  fn _ZN10QTabWidget9insertTabEiP7QWidgetRK7QString(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void, arg2: *mut c_void) -> c_int;
  // proto:  void QTabWidget::setTabEnabled(int index, bool );
  fn _ZN10QTabWidget13setTabEnabledEib(qthis: *mut c_void, arg0: c_int, arg1: c_char);
} // <= ext block end

// body block begin =>
// class sizeof(QTabWidget)=1
pub struct QTabWidget {
  qbase: QWidget,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTabWidget {
  pub fn inheritFrom(qthis: *mut c_void) -> QTabWidget {
    return QTabWidget{qbase: QWidget::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QTabWidget {
  type Target = QWidget;

  fn deref(&self) -> &QWidget {
    return &self.qbase;
  }
}
impl AsRef<QWidget> for QTabWidget {
  fn as_ref(&self) -> &QWidget {
    return &self.qbase;
  }
}
  // proto:  void QTabWidget::setCurrentWidget(QWidget * widget);
impl /*struct*/ QTabWidget {
  pub fn setCurrentWidget<RetType, T: QTabWidget_setCurrentWidget<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setCurrentWidget(self);
    // return 1;
  }
}

pub trait QTabWidget_setCurrentWidget<RetType> {
  fn setCurrentWidget(self , rsthis: &mut QTabWidget) -> RetType;
}

  // proto:  void QTabWidget::setCurrentWidget(QWidget * widget);
impl<'a> /*trait*/ QTabWidget_setCurrentWidget<()> for (QWidget) {
  fn setCurrentWidget(self , rsthis: &mut QTabWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget16setCurrentWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QTabWidget16setCurrentWidgetEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QTabWidget::count();
impl /*struct*/ QTabWidget {
  pub fn count<RetType, T: QTabWidget_count<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.count(self);
    // return 1;
  }
}

pub trait QTabWidget_count<RetType> {
  fn count(self , rsthis: &mut QTabWidget) -> RetType;
}

  // proto:  int QTabWidget::count();
impl<'a> /*trait*/ QTabWidget_count<i32> for () {
  fn count(self , rsthis: &mut QTabWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget5countEv()};
    let mut ret = unsafe {_ZNK10QTabWidget5countEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QTabWidget::tabCloseRequested(int index);
impl /*struct*/ QTabWidget {
  pub fn tabCloseRequested<RetType, T: QTabWidget_tabCloseRequested<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.tabCloseRequested(self);
    // return 1;
  }
}

pub trait QTabWidget_tabCloseRequested<RetType> {
  fn tabCloseRequested(self , rsthis: &mut QTabWidget) -> RetType;
}

  // proto:  void QTabWidget::tabCloseRequested(int index);
impl<'a> /*trait*/ QTabWidget_tabCloseRequested<()> for (i32) {
  fn tabCloseRequested(self , rsthis: &mut QTabWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget17tabCloseRequestedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QTabWidget17tabCloseRequestedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTabWidget::setDocumentMode(bool set);
impl /*struct*/ QTabWidget {
  pub fn setDocumentMode<RetType, T: QTabWidget_setDocumentMode<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setDocumentMode(self);
    // return 1;
  }
}

pub trait QTabWidget_setDocumentMode<RetType> {
  fn setDocumentMode(self , rsthis: &mut QTabWidget) -> RetType;
}

  // proto:  void QTabWidget::setDocumentMode(bool set);
impl<'a> /*trait*/ QTabWidget_setDocumentMode<()> for (i8) {
  fn setDocumentMode(self , rsthis: &mut QTabWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget15setDocumentModeEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN10QTabWidget15setDocumentModeEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QTabWidget::heightForWidth(int width);
impl /*struct*/ QTabWidget {
  pub fn heightForWidth<RetType, T: QTabWidget_heightForWidth<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.heightForWidth(self);
    // return 1;
  }
}

pub trait QTabWidget_heightForWidth<RetType> {
  fn heightForWidth(self , rsthis: &mut QTabWidget) -> RetType;
}

  // proto:  int QTabWidget::heightForWidth(int width);
impl<'a> /*trait*/ QTabWidget_heightForWidth<i32> for (i32) {
  fn heightForWidth(self , rsthis: &mut QTabWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget14heightForWidthEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK10QTabWidget14heightForWidthEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QTabWidget::addTab(QWidget * widget, const QString & );
impl /*struct*/ QTabWidget {
  pub fn addTab<RetType, T: QTabWidget_addTab<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.addTab(self);
    // return 1;
  }
}

pub trait QTabWidget_addTab<RetType> {
  fn addTab(self , rsthis: &mut QTabWidget) -> RetType;
}

  // proto:  int QTabWidget::addTab(QWidget * widget, const QString & );
impl<'a> /*trait*/ QTabWidget_addTab<i32> for (QWidget, QString) {
  fn addTab(self , rsthis: &mut QTabWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget6addTabEP7QWidgetRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN10QTabWidget6addTabEP7QWidgetRK7QString(rsthis.qclsinst, arg0, arg1)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QString QTabWidget::tabText(int index);
impl /*struct*/ QTabWidget {
  pub fn tabText<RetType, T: QTabWidget_tabText<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.tabText(self);
    // return 1;
  }
}

pub trait QTabWidget_tabText<RetType> {
  fn tabText(self , rsthis: &mut QTabWidget) -> RetType;
}

  // proto:  QString QTabWidget::tabText(int index);
impl<'a> /*trait*/ QTabWidget_tabText<QString> for (i32) {
  fn tabText(self , rsthis: &mut QTabWidget) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget7tabTextEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK10QTabWidget7tabTextEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTabWidget::clear();
impl /*struct*/ QTabWidget {
  pub fn clear<RetType, T: QTabWidget_clear<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.clear(self);
    // return 1;
  }
}

pub trait QTabWidget_clear<RetType> {
  fn clear(self , rsthis: &mut QTabWidget) -> RetType;
}

  // proto:  void QTabWidget::clear();
impl<'a> /*trait*/ QTabWidget_clear<()> for () {
  fn clear(self , rsthis: &mut QTabWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget5clearEv()};
     unsafe {_ZN10QTabWidget5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QTabWidget::hasHeightForWidth();
impl /*struct*/ QTabWidget {
  pub fn hasHeightForWidth<RetType, T: QTabWidget_hasHeightForWidth<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.hasHeightForWidth(self);
    // return 1;
  }
}

pub trait QTabWidget_hasHeightForWidth<RetType> {
  fn hasHeightForWidth(self , rsthis: &mut QTabWidget) -> RetType;
}

  // proto:  bool QTabWidget::hasHeightForWidth();
impl<'a> /*trait*/ QTabWidget_hasHeightForWidth<i8> for () {
  fn hasHeightForWidth(self , rsthis: &mut QTabWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget17hasHeightForWidthEv()};
    let mut ret = unsafe {_ZNK10QTabWidget17hasHeightForWidthEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QTabBar * QTabWidget::tabBar();
impl /*struct*/ QTabWidget {
  pub fn tabBar<RetType, T: QTabWidget_tabBar<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.tabBar(self);
    // return 1;
  }
}

pub trait QTabWidget_tabBar<RetType> {
  fn tabBar(self , rsthis: &mut QTabWidget) -> RetType;
}

  // proto:  QTabBar * QTabWidget::tabBar();
impl<'a> /*trait*/ QTabWidget_tabBar<QTabBar> for () {
  fn tabBar(self , rsthis: &mut QTabWidget) -> QTabBar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget6tabBarEv()};
    let mut ret = unsafe {_ZNK10QTabWidget6tabBarEv(rsthis.qclsinst)};
    let mut ret1 = QTabBar::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QTabWidget::tabsClosable();
impl /*struct*/ QTabWidget {
  pub fn tabsClosable<RetType, T: QTabWidget_tabsClosable<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.tabsClosable(self);
    // return 1;
  }
}

pub trait QTabWidget_tabsClosable<RetType> {
  fn tabsClosable(self , rsthis: &mut QTabWidget) -> RetType;
}

  // proto:  bool QTabWidget::tabsClosable();
impl<'a> /*trait*/ QTabWidget_tabsClosable<i8> for () {
  fn tabsClosable(self , rsthis: &mut QTabWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget12tabsClosableEv()};
    let mut ret = unsafe {_ZNK10QTabWidget12tabsClosableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QTabWidget::insertTab(int index, QWidget * widget, const QIcon & icon, const QString & label);
impl /*struct*/ QTabWidget {
  pub fn insertTab<RetType, T: QTabWidget_insertTab<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.insertTab(self);
    // return 1;
  }
}

pub trait QTabWidget_insertTab<RetType> {
  fn insertTab(self , rsthis: &mut QTabWidget) -> RetType;
}

  // proto:  int QTabWidget::insertTab(int index, QWidget * widget, const QIcon & icon, const QString & label);
impl<'a> /*trait*/ QTabWidget_insertTab<i32> for (i32, QWidget, QIcon, QString) {
  fn insertTab(self , rsthis: &mut QTabWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget9insertTabEiP7QWidgetRK5QIconRK7QString()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN10QTabWidget9insertTabEiP7QWidgetRK5QIconRK7QString(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QTabWidget::addTab(QWidget * widget, const QIcon & icon, const QString & label);
impl<'a> /*trait*/ QTabWidget_addTab<i32> for (QWidget, QIcon, QString) {
  fn addTab(self , rsthis: &mut QTabWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget6addTabEP7QWidgetRK5QIconRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN10QTabWidget6addTabEP7QWidgetRK5QIconRK7QString(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QTabWidget::setUsesScrollButtons(bool useButtons);
impl /*struct*/ QTabWidget {
  pub fn setUsesScrollButtons<RetType, T: QTabWidget_setUsesScrollButtons<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setUsesScrollButtons(self);
    // return 1;
  }
}

pub trait QTabWidget_setUsesScrollButtons<RetType> {
  fn setUsesScrollButtons(self , rsthis: &mut QTabWidget) -> RetType;
}

  // proto:  void QTabWidget::setUsesScrollButtons(bool useButtons);
impl<'a> /*trait*/ QTabWidget_setUsesScrollButtons<()> for (i8) {
  fn setUsesScrollButtons(self , rsthis: &mut QTabWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget20setUsesScrollButtonsEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN10QTabWidget20setUsesScrollButtonsEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QTabWidget::metaObject();
impl /*struct*/ QTabWidget {
  pub fn metaObject<RetType, T: QTabWidget_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QTabWidget_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QTabWidget) -> RetType;
}

  // proto:  const QMetaObject * QTabWidget::metaObject();
impl<'a> /*trait*/ QTabWidget_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QTabWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget10metaObjectEv()};
     unsafe {_ZNK10QTabWidget10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QString QTabWidget::tabToolTip(int index);
impl /*struct*/ QTabWidget {
  pub fn tabToolTip<RetType, T: QTabWidget_tabToolTip<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.tabToolTip(self);
    // return 1;
  }
}

pub trait QTabWidget_tabToolTip<RetType> {
  fn tabToolTip(self , rsthis: &mut QTabWidget) -> RetType;
}

  // proto:  QString QTabWidget::tabToolTip(int index);
impl<'a> /*trait*/ QTabWidget_tabToolTip<QString> for (i32) {
  fn tabToolTip(self , rsthis: &mut QTabWidget) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget10tabToolTipEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK10QTabWidget10tabToolTipEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QWidget * QTabWidget::currentWidget();
impl /*struct*/ QTabWidget {
  pub fn currentWidget<RetType, T: QTabWidget_currentWidget<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.currentWidget(self);
    // return 1;
  }
}

pub trait QTabWidget_currentWidget<RetType> {
  fn currentWidget(self , rsthis: &mut QTabWidget) -> RetType;
}

  // proto:  QWidget * QTabWidget::currentWidget();
impl<'a> /*trait*/ QTabWidget_currentWidget<QWidget> for () {
  fn currentWidget(self , rsthis: &mut QTabWidget) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget13currentWidgetEv()};
    let mut ret = unsafe {_ZNK10QTabWidget13currentWidgetEv(rsthis.qclsinst)};
    let mut ret1 = QWidget::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTabWidget::setIconSize(const QSize & size);
impl /*struct*/ QTabWidget {
  pub fn setIconSize<RetType, T: QTabWidget_setIconSize<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setIconSize(self);
    // return 1;
  }
}

pub trait QTabWidget_setIconSize<RetType> {
  fn setIconSize(self , rsthis: &mut QTabWidget) -> RetType;
}

  // proto:  void QTabWidget::setIconSize(const QSize & size);
impl<'a> /*trait*/ QTabWidget_setIconSize<()> for (QSize) {
  fn setIconSize(self , rsthis: &mut QTabWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget11setIconSizeERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QTabWidget11setIconSizeERK5QSize(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QWidget * QTabWidget::widget(int index);
impl /*struct*/ QTabWidget {
  pub fn widget<RetType, T: QTabWidget_widget<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.widget(self);
    // return 1;
  }
}

pub trait QTabWidget_widget<RetType> {
  fn widget(self , rsthis: &mut QTabWidget) -> RetType;
}

  // proto:  QWidget * QTabWidget::widget(int index);
impl<'a> /*trait*/ QTabWidget_widget<QWidget> for (i32) {
  fn widget(self , rsthis: &mut QTabWidget) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget6widgetEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK10QTabWidget6widgetEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QWidget::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTabWidget::setMovable(bool movable);
impl /*struct*/ QTabWidget {
  pub fn setMovable<RetType, T: QTabWidget_setMovable<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setMovable(self);
    // return 1;
  }
}

pub trait QTabWidget_setMovable<RetType> {
  fn setMovable(self , rsthis: &mut QTabWidget) -> RetType;
}

  // proto:  void QTabWidget::setMovable(bool movable);
impl<'a> /*trait*/ QTabWidget_setMovable<()> for (i8) {
  fn setMovable(self , rsthis: &mut QTabWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget10setMovableEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN10QTabWidget10setMovableEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QTabWidget::documentMode();
impl /*struct*/ QTabWidget {
  pub fn documentMode<RetType, T: QTabWidget_documentMode<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.documentMode(self);
    // return 1;
  }
}

pub trait QTabWidget_documentMode<RetType> {
  fn documentMode(self , rsthis: &mut QTabWidget) -> RetType;
}

  // proto:  bool QTabWidget::documentMode();
impl<'a> /*trait*/ QTabWidget_documentMode<i8> for () {
  fn documentMode(self , rsthis: &mut QTabWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget12documentModeEv()};
    let mut ret = unsafe {_ZNK10QTabWidget12documentModeEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QString QTabWidget::tabWhatsThis(int index);
impl /*struct*/ QTabWidget {
  pub fn tabWhatsThis<RetType, T: QTabWidget_tabWhatsThis<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.tabWhatsThis(self);
    // return 1;
  }
}

pub trait QTabWidget_tabWhatsThis<RetType> {
  fn tabWhatsThis(self , rsthis: &mut QTabWidget) -> RetType;
}

  // proto:  QString QTabWidget::tabWhatsThis(int index);
impl<'a> /*trait*/ QTabWidget_tabWhatsThis<QString> for (i32) {
  fn tabWhatsThis(self , rsthis: &mut QTabWidget) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget12tabWhatsThisEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK10QTabWidget12tabWhatsThisEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTabWidget::tabBarClicked(int index);
impl /*struct*/ QTabWidget {
  pub fn tabBarClicked<RetType, T: QTabWidget_tabBarClicked<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.tabBarClicked(self);
    // return 1;
  }
}

pub trait QTabWidget_tabBarClicked<RetType> {
  fn tabBarClicked(self , rsthis: &mut QTabWidget) -> RetType;
}

  // proto:  void QTabWidget::tabBarClicked(int index);
impl<'a> /*trait*/ QTabWidget_tabBarClicked<()> for (i32) {
  fn tabBarClicked(self , rsthis: &mut QTabWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget13tabBarClickedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QTabWidget13tabBarClickedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTabWidget::setTabText(int index, const QString & );
impl /*struct*/ QTabWidget {
  pub fn setTabText<RetType, T: QTabWidget_setTabText<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setTabText(self);
    // return 1;
  }
}

pub trait QTabWidget_setTabText<RetType> {
  fn setTabText(self , rsthis: &mut QTabWidget) -> RetType;
}

  // proto:  void QTabWidget::setTabText(int index, const QString & );
impl<'a> /*trait*/ QTabWidget_setTabText<()> for (i32, QString) {
  fn setTabText(self , rsthis: &mut QTabWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget10setTabTextEiRK7QString()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN10QTabWidget10setTabTextEiRK7QString(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QTabWidget::QTabWidget(const QTabWidget & );
impl /*struct*/ QTabWidget {
  pub fn NewQTabWidget<T: QTabWidget_NewQTabWidget>(value: T) -> QTabWidget {
    let rsthis = value.NewQTabWidget();
    return rsthis;
    // return 1;
  }
}

pub trait QTabWidget_NewQTabWidget {
  fn NewQTabWidget(self) -> QTabWidget;
}

  // proto:  void QTabWidget::QTabWidget(const QTabWidget & );
impl<'a> /*trait*/ QTabWidget_NewQTabWidget for (QTabWidget) {
  fn NewQTabWidget(self) -> QTabWidget {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidgetC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN10QTabWidgetC1ERKS_(qthis, arg0)};
    let rsthis = QTabWidget{/**/qbase: QWidget::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QTabWidget::QTabWidget(QWidget * parent);
impl<'a> /*trait*/ QTabWidget_NewQTabWidget for (QWidget) {
  fn NewQTabWidget(self) -> QTabWidget {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidgetC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN10QTabWidgetC1EP7QWidget(qthis, arg0)};
    let rsthis = QTabWidget{/**/qbase: QWidget::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QTabWidget::tabBarAutoHide();
impl /*struct*/ QTabWidget {
  pub fn tabBarAutoHide<RetType, T: QTabWidget_tabBarAutoHide<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.tabBarAutoHide(self);
    // return 1;
  }
}

pub trait QTabWidget_tabBarAutoHide<RetType> {
  fn tabBarAutoHide(self , rsthis: &mut QTabWidget) -> RetType;
}

  // proto:  bool QTabWidget::tabBarAutoHide();
impl<'a> /*trait*/ QTabWidget_tabBarAutoHide<i8> for () {
  fn tabBarAutoHide(self , rsthis: &mut QTabWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget14tabBarAutoHideEv()};
    let mut ret = unsafe {_ZNK10QTabWidget14tabBarAutoHideEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QTabWidget::currentChanged(int index);
impl /*struct*/ QTabWidget {
  pub fn currentChanged<RetType, T: QTabWidget_currentChanged<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.currentChanged(self);
    // return 1;
  }
}

pub trait QTabWidget_currentChanged<RetType> {
  fn currentChanged(self , rsthis: &mut QTabWidget) -> RetType;
}

  // proto:  void QTabWidget::currentChanged(int index);
impl<'a> /*trait*/ QTabWidget_currentChanged<()> for (i32) {
  fn currentChanged(self , rsthis: &mut QTabWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget14currentChangedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QTabWidget14currentChangedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTabWidget::setTabIcon(int index, const QIcon & icon);
impl /*struct*/ QTabWidget {
  pub fn setTabIcon<RetType, T: QTabWidget_setTabIcon<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setTabIcon(self);
    // return 1;
  }
}

pub trait QTabWidget_setTabIcon<RetType> {
  fn setTabIcon(self , rsthis: &mut QTabWidget) -> RetType;
}

  // proto:  void QTabWidget::setTabIcon(int index, const QIcon & icon);
impl<'a> /*trait*/ QTabWidget_setTabIcon<()> for (i32, QIcon) {
  fn setTabIcon(self , rsthis: &mut QTabWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget10setTabIconEiRK5QIcon()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN10QTabWidget10setTabIconEiRK5QIcon(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QIcon QTabWidget::tabIcon(int index);
impl /*struct*/ QTabWidget {
  pub fn tabIcon<RetType, T: QTabWidget_tabIcon<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.tabIcon(self);
    // return 1;
  }
}

pub trait QTabWidget_tabIcon<RetType> {
  fn tabIcon(self , rsthis: &mut QTabWidget) -> RetType;
}

  // proto:  QIcon QTabWidget::tabIcon(int index);
impl<'a> /*trait*/ QTabWidget_tabIcon<QIcon> for (i32) {
  fn tabIcon(self , rsthis: &mut QTabWidget) -> QIcon {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget7tabIconEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK10QTabWidget7tabIconEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QIcon::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QTabWidget::isTabEnabled(int index);
impl /*struct*/ QTabWidget {
  pub fn isTabEnabled<RetType, T: QTabWidget_isTabEnabled<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isTabEnabled(self);
    // return 1;
  }
}

pub trait QTabWidget_isTabEnabled<RetType> {
  fn isTabEnabled(self , rsthis: &mut QTabWidget) -> RetType;
}

  // proto:  bool QTabWidget::isTabEnabled(int index);
impl<'a> /*trait*/ QTabWidget_isTabEnabled<i8> for (i32) {
  fn isTabEnabled(self , rsthis: &mut QTabWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget12isTabEnabledEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK10QTabWidget12isTabEnabledEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QTabWidget::setTabBarAutoHide(bool enabled);
impl /*struct*/ QTabWidget {
  pub fn setTabBarAutoHide<RetType, T: QTabWidget_setTabBarAutoHide<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setTabBarAutoHide(self);
    // return 1;
  }
}

pub trait QTabWidget_setTabBarAutoHide<RetType> {
  fn setTabBarAutoHide(self , rsthis: &mut QTabWidget) -> RetType;
}

  // proto:  void QTabWidget::setTabBarAutoHide(bool enabled);
impl<'a> /*trait*/ QTabWidget_setTabBarAutoHide<()> for (i8) {
  fn setTabBarAutoHide(self , rsthis: &mut QTabWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget17setTabBarAutoHideEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN10QTabWidget17setTabBarAutoHideEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QSize QTabWidget::iconSize();
impl /*struct*/ QTabWidget {
  pub fn iconSize<RetType, T: QTabWidget_iconSize<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.iconSize(self);
    // return 1;
  }
}

pub trait QTabWidget_iconSize<RetType> {
  fn iconSize(self , rsthis: &mut QTabWidget) -> RetType;
}

  // proto:  QSize QTabWidget::iconSize();
impl<'a> /*trait*/ QTabWidget_iconSize<QSize> for () {
  fn iconSize(self , rsthis: &mut QTabWidget) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget8iconSizeEv()};
    let mut ret = unsafe {_ZNK10QTabWidget8iconSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTabWidget::setTabsClosable(bool closeable);
impl /*struct*/ QTabWidget {
  pub fn setTabsClosable<RetType, T: QTabWidget_setTabsClosable<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setTabsClosable(self);
    // return 1;
  }
}

pub trait QTabWidget_setTabsClosable<RetType> {
  fn setTabsClosable(self , rsthis: &mut QTabWidget) -> RetType;
}

  // proto:  void QTabWidget::setTabsClosable(bool closeable);
impl<'a> /*trait*/ QTabWidget_setTabsClosable<()> for (i8) {
  fn setTabsClosable(self , rsthis: &mut QTabWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget15setTabsClosableEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN10QTabWidget15setTabsClosableEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QSize QTabWidget::minimumSizeHint();
impl /*struct*/ QTabWidget {
  pub fn minimumSizeHint<RetType, T: QTabWidget_minimumSizeHint<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.minimumSizeHint(self);
    // return 1;
  }
}

pub trait QTabWidget_minimumSizeHint<RetType> {
  fn minimumSizeHint(self , rsthis: &mut QTabWidget) -> RetType;
}

  // proto:  QSize QTabWidget::minimumSizeHint();
impl<'a> /*trait*/ QTabWidget_minimumSizeHint<QSize> for () {
  fn minimumSizeHint(self , rsthis: &mut QTabWidget) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget15minimumSizeHintEv()};
    let mut ret = unsafe {_ZNK10QTabWidget15minimumSizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTabWidget::setCurrentIndex(int index);
impl /*struct*/ QTabWidget {
  pub fn setCurrentIndex<RetType, T: QTabWidget_setCurrentIndex<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setCurrentIndex(self);
    // return 1;
  }
}

pub trait QTabWidget_setCurrentIndex<RetType> {
  fn setCurrentIndex(self , rsthis: &mut QTabWidget) -> RetType;
}

  // proto:  void QTabWidget::setCurrentIndex(int index);
impl<'a> /*trait*/ QTabWidget_setCurrentIndex<()> for (i32) {
  fn setCurrentIndex(self , rsthis: &mut QTabWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget15setCurrentIndexEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QTabWidget15setCurrentIndexEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTabWidget::~QTabWidget();
impl /*struct*/ QTabWidget {
  pub fn FreeQTabWidget<RetType, T: QTabWidget_FreeQTabWidget<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQTabWidget(self);
    // return 1;
  }
}

pub trait QTabWidget_FreeQTabWidget<RetType> {
  fn FreeQTabWidget(self , rsthis: &mut QTabWidget) -> RetType;
}

  // proto:  void QTabWidget::~QTabWidget();
impl<'a> /*trait*/ QTabWidget_FreeQTabWidget<()> for () {
  fn FreeQTabWidget(self , rsthis: &mut QTabWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidgetD0Ev()};
     unsafe {_ZN10QTabWidgetD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTabWidget::setTabWhatsThis(int index, const QString & text);
impl /*struct*/ QTabWidget {
  pub fn setTabWhatsThis<RetType, T: QTabWidget_setTabWhatsThis<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setTabWhatsThis(self);
    // return 1;
  }
}

pub trait QTabWidget_setTabWhatsThis<RetType> {
  fn setTabWhatsThis(self , rsthis: &mut QTabWidget) -> RetType;
}

  // proto:  void QTabWidget::setTabWhatsThis(int index, const QString & text);
impl<'a> /*trait*/ QTabWidget_setTabWhatsThis<()> for (i32, QString) {
  fn setTabWhatsThis(self , rsthis: &mut QTabWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget15setTabWhatsThisEiRK7QString()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN10QTabWidget15setTabWhatsThisEiRK7QString(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QSize QTabWidget::sizeHint();
impl /*struct*/ QTabWidget {
  pub fn sizeHint<RetType, T: QTabWidget_sizeHint<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.sizeHint(self);
    // return 1;
  }
}

pub trait QTabWidget_sizeHint<RetType> {
  fn sizeHint(self , rsthis: &mut QTabWidget) -> RetType;
}

  // proto:  QSize QTabWidget::sizeHint();
impl<'a> /*trait*/ QTabWidget_sizeHint<QSize> for () {
  fn sizeHint(self , rsthis: &mut QTabWidget) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget8sizeHintEv()};
    let mut ret = unsafe {_ZNK10QTabWidget8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  int QTabWidget::indexOf(QWidget * widget);
impl /*struct*/ QTabWidget {
  pub fn indexOf<RetType, T: QTabWidget_indexOf<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.indexOf(self);
    // return 1;
  }
}

pub trait QTabWidget_indexOf<RetType> {
  fn indexOf(self , rsthis: &mut QTabWidget) -> RetType;
}

  // proto:  int QTabWidget::indexOf(QWidget * widget);
impl<'a> /*trait*/ QTabWidget_indexOf<i32> for (QWidget) {
  fn indexOf(self , rsthis: &mut QTabWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget7indexOfEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK10QTabWidget7indexOfEP7QWidget(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QTabWidget::removeTab(int index);
impl /*struct*/ QTabWidget {
  pub fn removeTab<RetType, T: QTabWidget_removeTab<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.removeTab(self);
    // return 1;
  }
}

pub trait QTabWidget_removeTab<RetType> {
  fn removeTab(self , rsthis: &mut QTabWidget) -> RetType;
}

  // proto:  void QTabWidget::removeTab(int index);
impl<'a> /*trait*/ QTabWidget_removeTab<()> for (i32) {
  fn removeTab(self , rsthis: &mut QTabWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget9removeTabEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QTabWidget9removeTabEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTabWidget::setTabToolTip(int index, const QString & tip);
impl /*struct*/ QTabWidget {
  pub fn setTabToolTip<RetType, T: QTabWidget_setTabToolTip<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setTabToolTip(self);
    // return 1;
  }
}

pub trait QTabWidget_setTabToolTip<RetType> {
  fn setTabToolTip(self , rsthis: &mut QTabWidget) -> RetType;
}

  // proto:  void QTabWidget::setTabToolTip(int index, const QString & tip);
impl<'a> /*trait*/ QTabWidget_setTabToolTip<()> for (i32, QString) {
  fn setTabToolTip(self , rsthis: &mut QTabWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget13setTabToolTipEiRK7QString()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN10QTabWidget13setTabToolTipEiRK7QString(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  bool QTabWidget::isMovable();
impl /*struct*/ QTabWidget {
  pub fn isMovable<RetType, T: QTabWidget_isMovable<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isMovable(self);
    // return 1;
  }
}

pub trait QTabWidget_isMovable<RetType> {
  fn isMovable(self , rsthis: &mut QTabWidget) -> RetType;
}

  // proto:  bool QTabWidget::isMovable();
impl<'a> /*trait*/ QTabWidget_isMovable<i8> for () {
  fn isMovable(self , rsthis: &mut QTabWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget9isMovableEv()};
    let mut ret = unsafe {_ZNK10QTabWidget9isMovableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QTabWidget::usesScrollButtons();
impl /*struct*/ QTabWidget {
  pub fn usesScrollButtons<RetType, T: QTabWidget_usesScrollButtons<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.usesScrollButtons(self);
    // return 1;
  }
}

pub trait QTabWidget_usesScrollButtons<RetType> {
  fn usesScrollButtons(self , rsthis: &mut QTabWidget) -> RetType;
}

  // proto:  bool QTabWidget::usesScrollButtons();
impl<'a> /*trait*/ QTabWidget_usesScrollButtons<i8> for () {
  fn usesScrollButtons(self , rsthis: &mut QTabWidget) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget17usesScrollButtonsEv()};
    let mut ret = unsafe {_ZNK10QTabWidget17usesScrollButtonsEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QTabWidget::tabBarDoubleClicked(int index);
impl /*struct*/ QTabWidget {
  pub fn tabBarDoubleClicked<RetType, T: QTabWidget_tabBarDoubleClicked<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.tabBarDoubleClicked(self);
    // return 1;
  }
}

pub trait QTabWidget_tabBarDoubleClicked<RetType> {
  fn tabBarDoubleClicked(self , rsthis: &mut QTabWidget) -> RetType;
}

  // proto:  void QTabWidget::tabBarDoubleClicked(int index);
impl<'a> /*trait*/ QTabWidget_tabBarDoubleClicked<()> for (i32) {
  fn tabBarDoubleClicked(self , rsthis: &mut QTabWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget19tabBarDoubleClickedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN10QTabWidget19tabBarDoubleClickedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QTabWidget::currentIndex();
impl /*struct*/ QTabWidget {
  pub fn currentIndex<RetType, T: QTabWidget_currentIndex<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.currentIndex(self);
    // return 1;
  }
}

pub trait QTabWidget_currentIndex<RetType> {
  fn currentIndex(self , rsthis: &mut QTabWidget) -> RetType;
}

  // proto:  int QTabWidget::currentIndex();
impl<'a> /*trait*/ QTabWidget_currentIndex<i32> for () {
  fn currentIndex(self , rsthis: &mut QTabWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTabWidget12currentIndexEv()};
    let mut ret = unsafe {_ZNK10QTabWidget12currentIndexEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QTabWidget::insertTab(int index, QWidget * widget, const QString & );
impl<'a> /*trait*/ QTabWidget_insertTab<i32> for (i32, QWidget, QString) {
  fn insertTab(self , rsthis: &mut QTabWidget) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget9insertTabEiP7QWidgetRK7QString()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN10QTabWidget9insertTabEiP7QWidgetRK7QString(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QTabWidget::setTabEnabled(int index, bool );
impl /*struct*/ QTabWidget {
  pub fn setTabEnabled<RetType, T: QTabWidget_setTabEnabled<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setTabEnabled(self);
    // return 1;
  }
}

pub trait QTabWidget_setTabEnabled<RetType> {
  fn setTabEnabled(self , rsthis: &mut QTabWidget) -> RetType;
}

  // proto:  void QTabWidget::setTabEnabled(int index, bool );
impl<'a> /*trait*/ QTabWidget_setTabEnabled<()> for (i32, i8) {
  fn setTabEnabled(self , rsthis: &mut QTabWidget) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTabWidget13setTabEnabledEib()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_char;
     unsafe {_ZN10QTabWidget13setTabEnabledEib(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// <= body block end

