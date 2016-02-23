// auto generated, do not modify.
// created: Mon Feb 22 23:57:02 2016
// src-file: /QtWidgets/qmdiarea.h
// dst-file: /src/widgets/qmdiarea.rs
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
use super::qabstractscrollarea::*; // 773
use std::ops::Deref;
use super::super::gui::qbrush::*; // 771
use super::qwidget::*; // 773
use super::qmdisubwindow::*; // 773
use super::super::core::qobjectdefs::*; // 771
use super::super::core::qsize::*; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QMdiArea_Class_Size() -> c_int;
  // proto:  void QMdiArea::activateNextSubWindow();
  fn C_ZN8QMdiArea21activateNextSubWindowEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QMdiArea::setBackground(const QBrush & background);
  fn C_ZN8QMdiArea13setBackgroundERK6QBrush(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QMdiArea::~QMdiArea();
  fn C_ZN8QMdiAreaD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QMdiArea::removeSubWindow(QWidget * widget);
  fn C_ZN8QMdiArea15removeSubWindowEP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QMdiArea::setTabsClosable(bool closable);
  fn C_ZN8QMdiArea15setTabsClosableEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  QMdiSubWindow * QMdiArea::currentSubWindow();
  fn C_ZNK8QMdiArea16currentSubWindowEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QMdiArea::tabsMovable();
  fn C_ZNK8QMdiArea11tabsMovableEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QMdiArea::activatePreviousSubWindow();
  fn C_ZN8QMdiArea25activatePreviousSubWindowEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QMdiArea::setDocumentMode(bool enabled);
  fn C_ZN8QMdiArea15setDocumentModeEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  bool QMdiArea::documentMode();
  fn C_ZNK8QMdiArea12documentModeEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QMdiArea::setActiveSubWindow(QMdiSubWindow * window);
  fn C_ZN8QMdiArea18setActiveSubWindowEP13QMdiSubWindow(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QMdiSubWindow * QMdiArea::activeSubWindow();
  fn C_ZNK8QMdiArea15activeSubWindowEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QMdiArea::setTabsMovable(bool movable);
  fn C_ZN8QMdiArea14setTabsMovableEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  const QMetaObject * QMdiArea::metaObject();
  fn C_ZNK8QMdiArea10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QMdiArea::QMdiArea(QWidget * parent);
  fn C_ZN8QMdiAreaC2EP7QWidget(arg0: *mut c_void) -> u64;
  // proto:  QSize QMdiArea::sizeHint();
  fn C_ZNK8QMdiArea8sizeHintEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QMdiArea::closeAllSubWindows();
  fn C_ZN8QMdiArea18closeAllSubWindowsEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QMdiArea::cascadeSubWindows();
  fn C_ZN8QMdiArea17cascadeSubWindowsEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QMdiArea::closeActiveSubWindow();
  fn C_ZN8QMdiArea20closeActiveSubWindowEv(qthis: u64 /* *mut c_void*/);
  // proto:  QBrush QMdiArea::background();
  fn C_ZNK8QMdiArea10backgroundEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QMdiArea::tileSubWindows();
  fn C_ZN8QMdiArea14tileSubWindowsEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QMdiArea::tabsClosable();
  fn C_ZNK8QMdiArea12tabsClosableEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QSize QMdiArea::minimumSizeHint();
  fn C_ZNK8QMdiArea15minimumSizeHintEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  fn QMdiArea_SlotProxy_connect__ZN8QMdiArea18subWindowActivatedEP13QMdiSubWindow(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QMdiArea)=1
#[derive(Default)]
pub struct QMdiArea {
  qbase: QAbstractScrollArea,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _subWindowActivated: QMdiArea_subWindowActivated_signal,
}

impl /*struct*/ QMdiArea {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QMdiArea {
    return QMdiArea{qbase: QAbstractScrollArea::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QMdiArea {
  type Target = QAbstractScrollArea;

  fn deref(&self) -> &QAbstractScrollArea {
    return & self.qbase;
  }
}
impl AsRef<QAbstractScrollArea> for QMdiArea {
  fn as_ref(& self) -> & QAbstractScrollArea {
    return & self.qbase;
  }
}
  // proto:  void QMdiArea::activateNextSubWindow();
impl /*struct*/ QMdiArea {
  pub fn activateNextSubWindow<RetType, T: QMdiArea_activateNextSubWindow<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.activateNextSubWindow(self);
    // return 1;
  }
}

pub trait QMdiArea_activateNextSubWindow<RetType> {
  fn activateNextSubWindow(self , rsthis: & QMdiArea) -> RetType;
}

  // proto:  void QMdiArea::activateNextSubWindow();
impl<'a> /*trait*/ QMdiArea_activateNextSubWindow<()> for () {
  fn activateNextSubWindow(self , rsthis: & QMdiArea) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMdiArea21activateNextSubWindowEv()};
     unsafe {C_ZN8QMdiArea21activateNextSubWindowEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QMdiArea::setBackground(const QBrush & background);
impl /*struct*/ QMdiArea {
  pub fn setBackground<RetType, T: QMdiArea_setBackground<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setBackground(self);
    // return 1;
  }
}

pub trait QMdiArea_setBackground<RetType> {
  fn setBackground(self , rsthis: & QMdiArea) -> RetType;
}

  // proto:  void QMdiArea::setBackground(const QBrush & background);
impl<'a> /*trait*/ QMdiArea_setBackground<()> for (&'a QBrush) {
  fn setBackground(self , rsthis: & QMdiArea) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMdiArea13setBackgroundERK6QBrush()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN8QMdiArea13setBackgroundERK6QBrush(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QMdiArea::~QMdiArea();
impl /*struct*/ QMdiArea {
  pub fn free<RetType, T: QMdiArea_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QMdiArea_free<RetType> {
  fn free(self , rsthis: & QMdiArea) -> RetType;
}

  // proto:  void QMdiArea::~QMdiArea();
impl<'a> /*trait*/ QMdiArea_free<()> for () {
  fn free(self , rsthis: & QMdiArea) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMdiAreaD2Ev()};
     unsafe {C_ZN8QMdiAreaD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QMdiArea::removeSubWindow(QWidget * widget);
impl /*struct*/ QMdiArea {
  pub fn removeSubWindow<RetType, T: QMdiArea_removeSubWindow<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.removeSubWindow(self);
    // return 1;
  }
}

pub trait QMdiArea_removeSubWindow<RetType> {
  fn removeSubWindow(self , rsthis: & QMdiArea) -> RetType;
}

  // proto:  void QMdiArea::removeSubWindow(QWidget * widget);
impl<'a> /*trait*/ QMdiArea_removeSubWindow<()> for (&'a QWidget) {
  fn removeSubWindow(self , rsthis: & QMdiArea) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMdiArea15removeSubWindowEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN8QMdiArea15removeSubWindowEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QMdiArea::setTabsClosable(bool closable);
impl /*struct*/ QMdiArea {
  pub fn setTabsClosable<RetType, T: QMdiArea_setTabsClosable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTabsClosable(self);
    // return 1;
  }
}

pub trait QMdiArea_setTabsClosable<RetType> {
  fn setTabsClosable(self , rsthis: & QMdiArea) -> RetType;
}

  // proto:  void QMdiArea::setTabsClosable(bool closable);
impl<'a> /*trait*/ QMdiArea_setTabsClosable<()> for (i8) {
  fn setTabsClosable(self , rsthis: & QMdiArea) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMdiArea15setTabsClosableEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN8QMdiArea15setTabsClosableEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QMdiSubWindow * QMdiArea::currentSubWindow();
impl /*struct*/ QMdiArea {
  pub fn currentSubWindow<RetType, T: QMdiArea_currentSubWindow<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.currentSubWindow(self);
    // return 1;
  }
}

pub trait QMdiArea_currentSubWindow<RetType> {
  fn currentSubWindow(self , rsthis: & QMdiArea) -> RetType;
}

  // proto:  QMdiSubWindow * QMdiArea::currentSubWindow();
impl<'a> /*trait*/ QMdiArea_currentSubWindow<QMdiSubWindow> for () {
  fn currentSubWindow(self , rsthis: & QMdiArea) -> QMdiSubWindow {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QMdiArea16currentSubWindowEv()};
    let mut ret = unsafe {C_ZNK8QMdiArea16currentSubWindowEv(rsthis.qclsinst)};
    let mut ret1 = QMdiSubWindow::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QMdiArea::tabsMovable();
impl /*struct*/ QMdiArea {
  pub fn tabsMovable<RetType, T: QMdiArea_tabsMovable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.tabsMovable(self);
    // return 1;
  }
}

pub trait QMdiArea_tabsMovable<RetType> {
  fn tabsMovable(self , rsthis: & QMdiArea) -> RetType;
}

  // proto:  bool QMdiArea::tabsMovable();
impl<'a> /*trait*/ QMdiArea_tabsMovable<i8> for () {
  fn tabsMovable(self , rsthis: & QMdiArea) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QMdiArea11tabsMovableEv()};
    let mut ret = unsafe {C_ZNK8QMdiArea11tabsMovableEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QMdiArea::activatePreviousSubWindow();
impl /*struct*/ QMdiArea {
  pub fn activatePreviousSubWindow<RetType, T: QMdiArea_activatePreviousSubWindow<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.activatePreviousSubWindow(self);
    // return 1;
  }
}

pub trait QMdiArea_activatePreviousSubWindow<RetType> {
  fn activatePreviousSubWindow(self , rsthis: & QMdiArea) -> RetType;
}

  // proto:  void QMdiArea::activatePreviousSubWindow();
impl<'a> /*trait*/ QMdiArea_activatePreviousSubWindow<()> for () {
  fn activatePreviousSubWindow(self , rsthis: & QMdiArea) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMdiArea25activatePreviousSubWindowEv()};
     unsafe {C_ZN8QMdiArea25activatePreviousSubWindowEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QMdiArea::setDocumentMode(bool enabled);
impl /*struct*/ QMdiArea {
  pub fn setDocumentMode<RetType, T: QMdiArea_setDocumentMode<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDocumentMode(self);
    // return 1;
  }
}

pub trait QMdiArea_setDocumentMode<RetType> {
  fn setDocumentMode(self , rsthis: & QMdiArea) -> RetType;
}

  // proto:  void QMdiArea::setDocumentMode(bool enabled);
impl<'a> /*trait*/ QMdiArea_setDocumentMode<()> for (i8) {
  fn setDocumentMode(self , rsthis: & QMdiArea) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMdiArea15setDocumentModeEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN8QMdiArea15setDocumentModeEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QMdiArea::documentMode();
impl /*struct*/ QMdiArea {
  pub fn documentMode<RetType, T: QMdiArea_documentMode<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.documentMode(self);
    // return 1;
  }
}

pub trait QMdiArea_documentMode<RetType> {
  fn documentMode(self , rsthis: & QMdiArea) -> RetType;
}

  // proto:  bool QMdiArea::documentMode();
impl<'a> /*trait*/ QMdiArea_documentMode<i8> for () {
  fn documentMode(self , rsthis: & QMdiArea) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QMdiArea12documentModeEv()};
    let mut ret = unsafe {C_ZNK8QMdiArea12documentModeEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QMdiArea::setActiveSubWindow(QMdiSubWindow * window);
impl /*struct*/ QMdiArea {
  pub fn setActiveSubWindow<RetType, T: QMdiArea_setActiveSubWindow<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setActiveSubWindow(self);
    // return 1;
  }
}

pub trait QMdiArea_setActiveSubWindow<RetType> {
  fn setActiveSubWindow(self , rsthis: & QMdiArea) -> RetType;
}

  // proto:  void QMdiArea::setActiveSubWindow(QMdiSubWindow * window);
impl<'a> /*trait*/ QMdiArea_setActiveSubWindow<()> for (&'a QMdiSubWindow) {
  fn setActiveSubWindow(self , rsthis: & QMdiArea) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMdiArea18setActiveSubWindowEP13QMdiSubWindow()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN8QMdiArea18setActiveSubWindowEP13QMdiSubWindow(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QMdiSubWindow * QMdiArea::activeSubWindow();
impl /*struct*/ QMdiArea {
  pub fn activeSubWindow<RetType, T: QMdiArea_activeSubWindow<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.activeSubWindow(self);
    // return 1;
  }
}

pub trait QMdiArea_activeSubWindow<RetType> {
  fn activeSubWindow(self , rsthis: & QMdiArea) -> RetType;
}

  // proto:  QMdiSubWindow * QMdiArea::activeSubWindow();
impl<'a> /*trait*/ QMdiArea_activeSubWindow<QMdiSubWindow> for () {
  fn activeSubWindow(self , rsthis: & QMdiArea) -> QMdiSubWindow {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QMdiArea15activeSubWindowEv()};
    let mut ret = unsafe {C_ZNK8QMdiArea15activeSubWindowEv(rsthis.qclsinst)};
    let mut ret1 = QMdiSubWindow::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QMdiArea::setTabsMovable(bool movable);
impl /*struct*/ QMdiArea {
  pub fn setTabsMovable<RetType, T: QMdiArea_setTabsMovable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTabsMovable(self);
    // return 1;
  }
}

pub trait QMdiArea_setTabsMovable<RetType> {
  fn setTabsMovable(self , rsthis: & QMdiArea) -> RetType;
}

  // proto:  void QMdiArea::setTabsMovable(bool movable);
impl<'a> /*trait*/ QMdiArea_setTabsMovable<()> for (i8) {
  fn setTabsMovable(self , rsthis: & QMdiArea) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMdiArea14setTabsMovableEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN8QMdiArea14setTabsMovableEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QMdiArea::metaObject();
impl /*struct*/ QMdiArea {
  pub fn metaObject<RetType, T: QMdiArea_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QMdiArea_metaObject<RetType> {
  fn metaObject(self , rsthis: & QMdiArea) -> RetType;
}

  // proto:  const QMetaObject * QMdiArea::metaObject();
impl<'a> /*trait*/ QMdiArea_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QMdiArea) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QMdiArea10metaObjectEv()};
    let mut ret = unsafe {C_ZNK8QMdiArea10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QMdiArea::QMdiArea(QWidget * parent);
impl /*struct*/ QMdiArea {
  pub fn new<T: QMdiArea_new>(value: T) -> QMdiArea {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QMdiArea_new {
  fn new(self) -> QMdiArea;
}

  // proto:  void QMdiArea::QMdiArea(QWidget * parent);
impl<'a> /*trait*/ QMdiArea_new for (Option<&'a QWidget>) {
  fn new(self) -> QMdiArea {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMdiAreaC2EP7QWidget()};
    let ctysz: c_int = unsafe{QMdiArea_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = (if self.is_none() {0} else {self.unwrap().qclsinst})  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN8QMdiAreaC2EP7QWidget(arg0)};
    let rsthis = QMdiArea{qbase: QAbstractScrollArea::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QSize QMdiArea::sizeHint();
impl /*struct*/ QMdiArea {
  pub fn sizeHint<RetType, T: QMdiArea_sizeHint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sizeHint(self);
    // return 1;
  }
}

pub trait QMdiArea_sizeHint<RetType> {
  fn sizeHint(self , rsthis: & QMdiArea) -> RetType;
}

  // proto:  QSize QMdiArea::sizeHint();
impl<'a> /*trait*/ QMdiArea_sizeHint<QSize> for () {
  fn sizeHint(self , rsthis: & QMdiArea) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QMdiArea8sizeHintEv()};
    let mut ret = unsafe {C_ZNK8QMdiArea8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QMdiArea::closeAllSubWindows();
impl /*struct*/ QMdiArea {
  pub fn closeAllSubWindows<RetType, T: QMdiArea_closeAllSubWindows<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.closeAllSubWindows(self);
    // return 1;
  }
}

pub trait QMdiArea_closeAllSubWindows<RetType> {
  fn closeAllSubWindows(self , rsthis: & QMdiArea) -> RetType;
}

  // proto:  void QMdiArea::closeAllSubWindows();
impl<'a> /*trait*/ QMdiArea_closeAllSubWindows<()> for () {
  fn closeAllSubWindows(self , rsthis: & QMdiArea) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMdiArea18closeAllSubWindowsEv()};
     unsafe {C_ZN8QMdiArea18closeAllSubWindowsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QMdiArea::cascadeSubWindows();
impl /*struct*/ QMdiArea {
  pub fn cascadeSubWindows<RetType, T: QMdiArea_cascadeSubWindows<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.cascadeSubWindows(self);
    // return 1;
  }
}

pub trait QMdiArea_cascadeSubWindows<RetType> {
  fn cascadeSubWindows(self , rsthis: & QMdiArea) -> RetType;
}

  // proto:  void QMdiArea::cascadeSubWindows();
impl<'a> /*trait*/ QMdiArea_cascadeSubWindows<()> for () {
  fn cascadeSubWindows(self , rsthis: & QMdiArea) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMdiArea17cascadeSubWindowsEv()};
     unsafe {C_ZN8QMdiArea17cascadeSubWindowsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QMdiArea::closeActiveSubWindow();
impl /*struct*/ QMdiArea {
  pub fn closeActiveSubWindow<RetType, T: QMdiArea_closeActiveSubWindow<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.closeActiveSubWindow(self);
    // return 1;
  }
}

pub trait QMdiArea_closeActiveSubWindow<RetType> {
  fn closeActiveSubWindow(self , rsthis: & QMdiArea) -> RetType;
}

  // proto:  void QMdiArea::closeActiveSubWindow();
impl<'a> /*trait*/ QMdiArea_closeActiveSubWindow<()> for () {
  fn closeActiveSubWindow(self , rsthis: & QMdiArea) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMdiArea20closeActiveSubWindowEv()};
     unsafe {C_ZN8QMdiArea20closeActiveSubWindowEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QBrush QMdiArea::background();
impl /*struct*/ QMdiArea {
  pub fn background<RetType, T: QMdiArea_background<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.background(self);
    // return 1;
  }
}

pub trait QMdiArea_background<RetType> {
  fn background(self , rsthis: & QMdiArea) -> RetType;
}

  // proto:  QBrush QMdiArea::background();
impl<'a> /*trait*/ QMdiArea_background<QBrush> for () {
  fn background(self , rsthis: & QMdiArea) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QMdiArea10backgroundEv()};
    let mut ret = unsafe {C_ZNK8QMdiArea10backgroundEv(rsthis.qclsinst)};
    let mut ret1 = QBrush::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QMdiArea::tileSubWindows();
impl /*struct*/ QMdiArea {
  pub fn tileSubWindows<RetType, T: QMdiArea_tileSubWindows<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.tileSubWindows(self);
    // return 1;
  }
}

pub trait QMdiArea_tileSubWindows<RetType> {
  fn tileSubWindows(self , rsthis: & QMdiArea) -> RetType;
}

  // proto:  void QMdiArea::tileSubWindows();
impl<'a> /*trait*/ QMdiArea_tileSubWindows<()> for () {
  fn tileSubWindows(self , rsthis: & QMdiArea) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMdiArea14tileSubWindowsEv()};
     unsafe {C_ZN8QMdiArea14tileSubWindowsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QMdiArea::tabsClosable();
impl /*struct*/ QMdiArea {
  pub fn tabsClosable<RetType, T: QMdiArea_tabsClosable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.tabsClosable(self);
    // return 1;
  }
}

pub trait QMdiArea_tabsClosable<RetType> {
  fn tabsClosable(self , rsthis: & QMdiArea) -> RetType;
}

  // proto:  bool QMdiArea::tabsClosable();
impl<'a> /*trait*/ QMdiArea_tabsClosable<i8> for () {
  fn tabsClosable(self , rsthis: & QMdiArea) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QMdiArea12tabsClosableEv()};
    let mut ret = unsafe {C_ZNK8QMdiArea12tabsClosableEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  QSize QMdiArea::minimumSizeHint();
impl /*struct*/ QMdiArea {
  pub fn minimumSizeHint<RetType, T: QMdiArea_minimumSizeHint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.minimumSizeHint(self);
    // return 1;
  }
}

pub trait QMdiArea_minimumSizeHint<RetType> {
  fn minimumSizeHint(self , rsthis: & QMdiArea) -> RetType;
}

  // proto:  QSize QMdiArea::minimumSizeHint();
impl<'a> /*trait*/ QMdiArea_minimumSizeHint<QSize> for () {
  fn minimumSizeHint(self , rsthis: & QMdiArea) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QMdiArea15minimumSizeHintEv()};
    let mut ret = unsafe {C_ZNK8QMdiArea15minimumSizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

#[derive(Default)] // for QMdiArea_subWindowActivated
pub struct QMdiArea_subWindowActivated_signal{poi:u64}
impl /* struct */ QMdiArea {
  pub fn subWindowActivated(&self) -> QMdiArea_subWindowActivated_signal {
     return QMdiArea_subWindowActivated_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QMdiArea_subWindowActivated_signal {
  pub fn connect<T: QMdiArea_subWindowActivated_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QMdiArea_subWindowActivated_signal_connect {
  fn connect(self, sigthis: QMdiArea_subWindowActivated_signal);
}

// subWindowActivated(class QMdiSubWindow *)
extern fn QMdiArea_subWindowActivated_signal_connect_cb_0(rsfptr:fn(QMdiSubWindow), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QMdiSubWindow::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QMdiArea_subWindowActivated_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn(QMdiSubWindow)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QMdiSubWindow::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QMdiArea_subWindowActivated_signal_connect for fn(QMdiSubWindow) {
  fn connect(self, sigthis: QMdiArea_subWindowActivated_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QMdiArea_subWindowActivated_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QMdiArea_SlotProxy_connect__ZN8QMdiArea18subWindowActivatedEP13QMdiSubWindow(arg0, arg1, arg2)};
  }
}
impl /* trait */ QMdiArea_subWindowActivated_signal_connect for Box<Fn(QMdiSubWindow)> {
  fn connect(self, sigthis: QMdiArea_subWindowActivated_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QMdiArea_subWindowActivated_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QMdiArea_SlotProxy_connect__ZN8QMdiArea18subWindowActivatedEP13QMdiSubWindow(arg0, arg1, arg2)};
  }
}
// <= body block end

