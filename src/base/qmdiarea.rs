// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qbrush::QBrush;
use super::qwidget::QWidget;
use super::qmdisubwindow::QMdiSubWindow;
use super::qsize::QSize;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QMdiArea::activateNextSubWindow();
  fn _ZN8QMdiArea21activateNextSubWindowEv(qthis: *mut c_void) ;
  // proto:  void QMdiArea::setBackground(const QBrush & background);
  fn _ZN8QMdiArea13setBackgroundERK6QBrush(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QMdiArea::FreeQMdiArea();
  fn _ZN8QMdiAreaD0Ev(qthis: *mut c_void) ;
  // proto:  void QMdiArea::removeSubWindow(QWidget * widget);
  fn _ZN8QMdiArea15removeSubWindowEP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QMdiArea::setTabsClosable(bool closable);
  fn _ZN8QMdiArea15setTabsClosableEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  QMdiSubWindow * QMdiArea::currentSubWindow();
  fn _ZNK8QMdiArea16currentSubWindowEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QMdiArea::tabsMovable();
  fn _ZNK8QMdiArea11tabsMovableEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QMdiArea::activatePreviousSubWindow();
  fn _ZN8QMdiArea25activatePreviousSubWindowEv(qthis: *mut c_void) ;
  // proto:  void QMdiArea::setDocumentMode(bool enabled);
  fn _ZN8QMdiArea15setDocumentModeEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  bool QMdiArea::documentMode();
  fn _ZNK8QMdiArea12documentModeEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QMdiArea::setActiveSubWindow(QMdiSubWindow * window);
  fn _ZN8QMdiArea18setActiveSubWindowEP13QMdiSubWindow(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QMdiSubWindow * QMdiArea::activeSubWindow();
  fn _ZNK8QMdiArea15activeSubWindowEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QMdiArea::setTabsMovable(bool movable);
  fn _ZN8QMdiArea14setTabsMovableEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  const QMetaObject * QMdiArea::metaObject();
  fn _ZNK8QMdiArea10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QMdiArea::NewQMdiArea(QWidget * parent);
  fn _ZN8QMdiAreaC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QSize QMdiArea::sizeHint();
  fn _ZNK8QMdiArea8sizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QMdiArea::closeAllSubWindows();
  fn _ZN8QMdiArea18closeAllSubWindowsEv(qthis: *mut c_void) ;
  // proto:  void QMdiArea::NewQMdiArea(const QMdiArea & );
  fn _ZN8QMdiAreaC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QMdiArea::subWindowActivated(QMdiSubWindow * );
  fn _ZN8QMdiArea18subWindowActivatedEP13QMdiSubWindow(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QMdiArea::cascadeSubWindows();
  fn _ZN8QMdiArea17cascadeSubWindowsEv(qthis: *mut c_void) ;
  // proto:  void QMdiArea::closeActiveSubWindow();
  fn _ZN8QMdiArea20closeActiveSubWindowEv(qthis: *mut c_void) ;
  // proto:  QBrush QMdiArea::background();
  fn _ZNK8QMdiArea10backgroundEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QMdiArea::tileSubWindows();
  fn _ZN8QMdiArea14tileSubWindowsEv(qthis: *mut c_void) ;
  // proto:  bool QMdiArea::tabsClosable();
  fn _ZNK8QMdiArea12tabsClosableEv(qthis: *mut c_void) -> int8_t;
  // proto:  QSize QMdiArea::minimumSizeHint();
  fn _ZNK8QMdiArea15minimumSizeHintEv(qthis: *mut c_void) -> *mut c_void;
}

// body block begin
// class sizeof(QMdiArea)=1
pub struct QMdiArea {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QMdiArea {
  pub fn activateNextSubWindow<T: QMdiArea_activateNextSubWindow>(&mut self, value: T)  {
     value.activateNextSubWindow(self);
    // return 1;
  }
}

pub trait QMdiArea_activateNextSubWindow {
  fn activateNextSubWindow(self, rsthis: &mut QMdiArea) ;
}

// proto:  void QMdiArea::activateNextSubWindow();
impl<'a> /*trait*/ QMdiArea_activateNextSubWindow for () {
  fn activateNextSubWindow(self, rsthis: &mut QMdiArea)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMdiArea21activateNextSubWindowEv()};
     unsafe {_ZN8QMdiArea21activateNextSubWindowEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QMdiArea {
  pub fn setBackground<T: QMdiArea_setBackground>(&mut self, value: T)  {
     value.setBackground(self);
    // return 1;
  }
}

pub trait QMdiArea_setBackground {
  fn setBackground(self, rsthis: &mut QMdiArea) ;
}

// proto:  void QMdiArea::setBackground(const QBrush & background);
impl<'a> /*trait*/ QMdiArea_setBackground for (&'a  QBrush) {
  fn setBackground(self, rsthis: &mut QMdiArea)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMdiArea13setBackgroundERK6QBrush()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QMdiArea13setBackgroundERK6QBrush(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMdiArea {
  pub fn FreeQMdiArea<T: QMdiArea_FreeQMdiArea>(&mut self, value: T)  {
     value.FreeQMdiArea(self);
    // return 1;
  }
}

pub trait QMdiArea_FreeQMdiArea {
  fn FreeQMdiArea(self, rsthis: &mut QMdiArea) ;
}

// proto:  void QMdiArea::FreeQMdiArea();
impl<'a> /*trait*/ QMdiArea_FreeQMdiArea for () {
  fn FreeQMdiArea(self, rsthis: &mut QMdiArea)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMdiAreaD0Ev()};
     unsafe {_ZN8QMdiAreaD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QMdiArea {
  pub fn removeSubWindow<T: QMdiArea_removeSubWindow>(&mut self, value: T)  {
     value.removeSubWindow(self);
    // return 1;
  }
}

pub trait QMdiArea_removeSubWindow {
  fn removeSubWindow(self, rsthis: &mut QMdiArea) ;
}

// proto:  void QMdiArea::removeSubWindow(QWidget * widget);
impl<'a> /*trait*/ QMdiArea_removeSubWindow for (&'a mut QWidget) {
  fn removeSubWindow(self, rsthis: &mut QMdiArea)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMdiArea15removeSubWindowEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QMdiArea15removeSubWindowEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMdiArea {
  pub fn setTabsClosable<T: QMdiArea_setTabsClosable>(&mut self, value: T)  {
     value.setTabsClosable(self);
    // return 1;
  }
}

pub trait QMdiArea_setTabsClosable {
  fn setTabsClosable(self, rsthis: &mut QMdiArea) ;
}

// proto:  void QMdiArea::setTabsClosable(bool closable);
impl<'a> /*trait*/ QMdiArea_setTabsClosable for (i8) {
  fn setTabsClosable(self, rsthis: &mut QMdiArea)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMdiArea15setTabsClosableEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN8QMdiArea15setTabsClosableEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMdiArea {
  pub fn currentSubWindow<T: QMdiArea_currentSubWindow>(&mut self, value: T) -> QMdiSubWindow {
    return value.currentSubWindow(self);
    // return 1;
  }
}

pub trait QMdiArea_currentSubWindow {
  fn currentSubWindow(self, rsthis: &mut QMdiArea) -> QMdiSubWindow;
}

// proto:  QMdiSubWindow * QMdiArea::currentSubWindow();
impl<'a> /*trait*/ QMdiArea_currentSubWindow for () {
  fn currentSubWindow(self, rsthis: &mut QMdiArea) -> QMdiSubWindow {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QMdiArea16currentSubWindowEv()};
    let mut ret = unsafe {_ZNK8QMdiArea16currentSubWindowEv(rsthis.qclsinst)};
    let mut ret1 = QMdiSubWindow{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMdiArea {
  pub fn tabsMovable<T: QMdiArea_tabsMovable>(&mut self, value: T) -> i8 {
    return value.tabsMovable(self);
    // return 1;
  }
}

pub trait QMdiArea_tabsMovable {
  fn tabsMovable(self, rsthis: &mut QMdiArea) -> i8;
}

// proto:  bool QMdiArea::tabsMovable();
impl<'a> /*trait*/ QMdiArea_tabsMovable for () {
  fn tabsMovable(self, rsthis: &mut QMdiArea) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QMdiArea11tabsMovableEv()};
    let mut ret = unsafe {_ZNK8QMdiArea11tabsMovableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QMdiArea {
  pub fn activatePreviousSubWindow<T: QMdiArea_activatePreviousSubWindow>(&mut self, value: T)  {
     value.activatePreviousSubWindow(self);
    // return 1;
  }
}

pub trait QMdiArea_activatePreviousSubWindow {
  fn activatePreviousSubWindow(self, rsthis: &mut QMdiArea) ;
}

// proto:  void QMdiArea::activatePreviousSubWindow();
impl<'a> /*trait*/ QMdiArea_activatePreviousSubWindow for () {
  fn activatePreviousSubWindow(self, rsthis: &mut QMdiArea)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMdiArea25activatePreviousSubWindowEv()};
     unsafe {_ZN8QMdiArea25activatePreviousSubWindowEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QMdiArea {
  pub fn setDocumentMode<T: QMdiArea_setDocumentMode>(&mut self, value: T)  {
     value.setDocumentMode(self);
    // return 1;
  }
}

pub trait QMdiArea_setDocumentMode {
  fn setDocumentMode(self, rsthis: &mut QMdiArea) ;
}

// proto:  void QMdiArea::setDocumentMode(bool enabled);
impl<'a> /*trait*/ QMdiArea_setDocumentMode for (i8) {
  fn setDocumentMode(self, rsthis: &mut QMdiArea)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMdiArea15setDocumentModeEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN8QMdiArea15setDocumentModeEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMdiArea {
  pub fn documentMode<T: QMdiArea_documentMode>(&mut self, value: T) -> i8 {
    return value.documentMode(self);
    // return 1;
  }
}

pub trait QMdiArea_documentMode {
  fn documentMode(self, rsthis: &mut QMdiArea) -> i8;
}

// proto:  bool QMdiArea::documentMode();
impl<'a> /*trait*/ QMdiArea_documentMode for () {
  fn documentMode(self, rsthis: &mut QMdiArea) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QMdiArea12documentModeEv()};
    let mut ret = unsafe {_ZNK8QMdiArea12documentModeEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QMdiArea {
  pub fn setActiveSubWindow<T: QMdiArea_setActiveSubWindow>(&mut self, value: T)  {
     value.setActiveSubWindow(self);
    // return 1;
  }
}

pub trait QMdiArea_setActiveSubWindow {
  fn setActiveSubWindow(self, rsthis: &mut QMdiArea) ;
}

// proto:  void QMdiArea::setActiveSubWindow(QMdiSubWindow * window);
impl<'a> /*trait*/ QMdiArea_setActiveSubWindow for (&'a mut QMdiSubWindow) {
  fn setActiveSubWindow(self, rsthis: &mut QMdiArea)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMdiArea18setActiveSubWindowEP13QMdiSubWindow()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QMdiArea18setActiveSubWindowEP13QMdiSubWindow(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMdiArea {
  pub fn activeSubWindow<T: QMdiArea_activeSubWindow>(&mut self, value: T) -> QMdiSubWindow {
    return value.activeSubWindow(self);
    // return 1;
  }
}

pub trait QMdiArea_activeSubWindow {
  fn activeSubWindow(self, rsthis: &mut QMdiArea) -> QMdiSubWindow;
}

// proto:  QMdiSubWindow * QMdiArea::activeSubWindow();
impl<'a> /*trait*/ QMdiArea_activeSubWindow for () {
  fn activeSubWindow(self, rsthis: &mut QMdiArea) -> QMdiSubWindow {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QMdiArea15activeSubWindowEv()};
    let mut ret = unsafe {_ZNK8QMdiArea15activeSubWindowEv(rsthis.qclsinst)};
    let mut ret1 = QMdiSubWindow{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMdiArea {
  pub fn setTabsMovable<T: QMdiArea_setTabsMovable>(&mut self, value: T)  {
     value.setTabsMovable(self);
    // return 1;
  }
}

pub trait QMdiArea_setTabsMovable {
  fn setTabsMovable(self, rsthis: &mut QMdiArea) ;
}

// proto:  void QMdiArea::setTabsMovable(bool movable);
impl<'a> /*trait*/ QMdiArea_setTabsMovable for (i8) {
  fn setTabsMovable(self, rsthis: &mut QMdiArea)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMdiArea14setTabsMovableEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN8QMdiArea14setTabsMovableEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMdiArea {
  pub fn metaObject<T: QMdiArea_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QMdiArea_metaObject {
  fn metaObject(self, rsthis: &mut QMdiArea) ;
}

// proto:  const QMetaObject * QMdiArea::metaObject();
impl<'a> /*trait*/ QMdiArea_metaObject for () {
  fn metaObject(self, rsthis: &mut QMdiArea)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QMdiArea10metaObjectEv()};
     unsafe {_ZNK8QMdiArea10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QMdiArea {
  pub fn NewQMdiArea<T: QMdiArea_NewQMdiArea>(value: T) -> QMdiArea {
    let rsthis = value.NewQMdiArea();
    return rsthis;
    // return 1;
  }
}

pub trait QMdiArea_NewQMdiArea {
  fn NewQMdiArea(self) -> QMdiArea;
}

// proto: void QMdiArea::NewQMdiArea(QWidget * parent);
impl<'a> /*trait*/ QMdiArea_NewQMdiArea for (&'a mut QWidget) {
  fn NewQMdiArea(self) -> QMdiArea {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMdiAreaC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN8QMdiAreaC1EP7QWidget(qthis, arg0)};
    let rsthis = QMdiArea{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QMdiArea {
  pub fn sizeHint<T: QMdiArea_sizeHint>(&mut self, value: T) -> QSize {
    return value.sizeHint(self);
    // return 1;
  }
}

pub trait QMdiArea_sizeHint {
  fn sizeHint(self, rsthis: &mut QMdiArea) -> QSize;
}

// proto:  QSize QMdiArea::sizeHint();
impl<'a> /*trait*/ QMdiArea_sizeHint for () {
  fn sizeHint(self, rsthis: &mut QMdiArea) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QMdiArea8sizeHintEv()};
    let mut ret = unsafe {_ZNK8QMdiArea8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMdiArea {
  pub fn closeAllSubWindows<T: QMdiArea_closeAllSubWindows>(&mut self, value: T)  {
     value.closeAllSubWindows(self);
    // return 1;
  }
}

pub trait QMdiArea_closeAllSubWindows {
  fn closeAllSubWindows(self, rsthis: &mut QMdiArea) ;
}

// proto:  void QMdiArea::closeAllSubWindows();
impl<'a> /*trait*/ QMdiArea_closeAllSubWindows for () {
  fn closeAllSubWindows(self, rsthis: &mut QMdiArea)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMdiArea18closeAllSubWindowsEv()};
     unsafe {_ZN8QMdiArea18closeAllSubWindowsEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto: void QMdiArea::NewQMdiArea(const QMdiArea & );
impl<'a> /*trait*/ QMdiArea_NewQMdiArea for (&'a  QMdiArea) {
  fn NewQMdiArea(self) -> QMdiArea {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMdiAreaC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN8QMdiAreaC1ERKS_(qthis, arg0)};
    let rsthis = QMdiArea{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QMdiArea {
  pub fn subWindowActivated<T: QMdiArea_subWindowActivated>(&mut self, value: T)  {
     value.subWindowActivated(self);
    // return 1;
  }
}

pub trait QMdiArea_subWindowActivated {
  fn subWindowActivated(self, rsthis: &mut QMdiArea) ;
}

// proto:  void QMdiArea::subWindowActivated(QMdiSubWindow * );
impl<'a> /*trait*/ QMdiArea_subWindowActivated for (&'a mut QMdiSubWindow) {
  fn subWindowActivated(self, rsthis: &mut QMdiArea)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMdiArea18subWindowActivatedEP13QMdiSubWindow()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QMdiArea18subWindowActivatedEP13QMdiSubWindow(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMdiArea {
  pub fn cascadeSubWindows<T: QMdiArea_cascadeSubWindows>(&mut self, value: T)  {
     value.cascadeSubWindows(self);
    // return 1;
  }
}

pub trait QMdiArea_cascadeSubWindows {
  fn cascadeSubWindows(self, rsthis: &mut QMdiArea) ;
}

// proto:  void QMdiArea::cascadeSubWindows();
impl<'a> /*trait*/ QMdiArea_cascadeSubWindows for () {
  fn cascadeSubWindows(self, rsthis: &mut QMdiArea)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMdiArea17cascadeSubWindowsEv()};
     unsafe {_ZN8QMdiArea17cascadeSubWindowsEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QMdiArea {
  pub fn closeActiveSubWindow<T: QMdiArea_closeActiveSubWindow>(&mut self, value: T)  {
     value.closeActiveSubWindow(self);
    // return 1;
  }
}

pub trait QMdiArea_closeActiveSubWindow {
  fn closeActiveSubWindow(self, rsthis: &mut QMdiArea) ;
}

// proto:  void QMdiArea::closeActiveSubWindow();
impl<'a> /*trait*/ QMdiArea_closeActiveSubWindow for () {
  fn closeActiveSubWindow(self, rsthis: &mut QMdiArea)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMdiArea20closeActiveSubWindowEv()};
     unsafe {_ZN8QMdiArea20closeActiveSubWindowEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QMdiArea {
  pub fn background<T: QMdiArea_background>(&mut self, value: T) -> QBrush {
    return value.background(self);
    // return 1;
  }
}

pub trait QMdiArea_background {
  fn background(self, rsthis: &mut QMdiArea) -> QBrush;
}

// proto:  QBrush QMdiArea::background();
impl<'a> /*trait*/ QMdiArea_background for () {
  fn background(self, rsthis: &mut QMdiArea) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QMdiArea10backgroundEv()};
    let mut ret = unsafe {_ZNK8QMdiArea10backgroundEv(rsthis.qclsinst)};
    let mut ret1 = QBrush{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMdiArea {
  pub fn tileSubWindows<T: QMdiArea_tileSubWindows>(&mut self, value: T)  {
     value.tileSubWindows(self);
    // return 1;
  }
}

pub trait QMdiArea_tileSubWindows {
  fn tileSubWindows(self, rsthis: &mut QMdiArea) ;
}

// proto:  void QMdiArea::tileSubWindows();
impl<'a> /*trait*/ QMdiArea_tileSubWindows for () {
  fn tileSubWindows(self, rsthis: &mut QMdiArea)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMdiArea14tileSubWindowsEv()};
     unsafe {_ZN8QMdiArea14tileSubWindowsEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QMdiArea {
  pub fn tabsClosable<T: QMdiArea_tabsClosable>(&mut self, value: T) -> i8 {
    return value.tabsClosable(self);
    // return 1;
  }
}

pub trait QMdiArea_tabsClosable {
  fn tabsClosable(self, rsthis: &mut QMdiArea) -> i8;
}

// proto:  bool QMdiArea::tabsClosable();
impl<'a> /*trait*/ QMdiArea_tabsClosable for () {
  fn tabsClosable(self, rsthis: &mut QMdiArea) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QMdiArea12tabsClosableEv()};
    let mut ret = unsafe {_ZNK8QMdiArea12tabsClosableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QMdiArea {
  pub fn minimumSizeHint<T: QMdiArea_minimumSizeHint>(&mut self, value: T) -> QSize {
    return value.minimumSizeHint(self);
    // return 1;
  }
}

pub trait QMdiArea_minimumSizeHint {
  fn minimumSizeHint(self, rsthis: &mut QMdiArea) -> QSize;
}

// proto:  QSize QMdiArea::minimumSizeHint();
impl<'a> /*trait*/ QMdiArea_minimumSizeHint for () {
  fn minimumSizeHint(self, rsthis: &mut QMdiArea) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QMdiArea15minimumSizeHintEv()};
    let mut ret = unsafe {_ZNK8QMdiArea15minimumSizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

