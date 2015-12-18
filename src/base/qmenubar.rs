// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;
use super::qaction::QAction;
use super::qmenu::QMenu;
use super::qsize::QSize;
use super::qpoint::QPoint;
use super::qwidget::QWidget;
use super::qicon::QIcon;
use super::qrect::QRect;
use super::qobject::QObject;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  QAction * QMenuBar::addAction(const QString & text);
  fn _ZN8QMenuBar9addActionERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QPlatformMenuBar * QMenuBar::platformMenuBar();
  fn _ZN8QMenuBar15platformMenuBarEv(qthis: *mut c_void) ;
  // proto:  void QMenuBar::setNativeMenuBar(bool nativeMenuBar);
  fn _ZN8QMenuBar16setNativeMenuBarEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QMenuBar::FreeQMenuBar();
  fn _ZN8QMenuBarD0Ev(qthis: *mut c_void) ;
  // proto:  QAction * QMenuBar::addMenu(QMenu * menu);
  fn _ZN8QMenuBar7addMenuEP5QMenu(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QMenuBar::triggered(QAction * action);
  fn _ZN8QMenuBar9triggeredEP7QAction(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QSize QMenuBar::sizeHint();
  fn _ZNK8QMenuBar8sizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QAction * QMenuBar::actionAt(const QPoint & );
  fn _ZNK8QMenuBar8actionAtERK6QPoint(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QMenuBar::hovered(QAction * action);
  fn _ZN8QMenuBar7hoveredEP7QAction(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  const QMetaObject * QMenuBar::metaObject();
  fn _ZNK8QMenuBar10metaObjectEv(qthis: *mut c_void) ;
  // proto:  bool QMenuBar::isNativeMenuBar();
  fn _ZNK8QMenuBar15isNativeMenuBarEv(qthis: *mut c_void) -> int8_t;
  // proto:  QAction * QMenuBar::insertSeparator(QAction * before);
  fn _ZN8QMenuBar15insertSeparatorEP7QAction(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QAction * QMenuBar::addSeparator();
  fn _ZN8QMenuBar12addSeparatorEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QSize QMenuBar::minimumSizeHint();
  fn _ZNK8QMenuBar15minimumSizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QMenuBar::isDefaultUp();
  fn _ZNK8QMenuBar11isDefaultUpEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QMenuBar::NewQMenuBar(const QMenuBar & );
  fn _ZN8QMenuBarC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QMenuBar::NewQMenuBar(QWidget * parent);
  fn _ZN8QMenuBarC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QMenuBar::setActiveAction(QAction * action);
  fn _ZN8QMenuBar15setActiveActionEP7QAction(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QMenuBar::clear();
  fn _ZN8QMenuBar5clearEv(qthis: *mut c_void) ;
  // proto:  QAction * QMenuBar::activeAction();
  fn _ZNK8QMenuBar12activeActionEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QMenu * QMenuBar::addMenu(const QIcon & icon, const QString & title);
  fn _ZN8QMenuBar7addMenuERK5QIconRK7QString(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  QMenu * QMenuBar::addMenu(const QString & title);
  fn _ZN8QMenuBar7addMenuERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QRect QMenuBar::actionGeometry(QAction * );
  fn _ZNK8QMenuBar14actionGeometryEP7QAction(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QAction * QMenuBar::insertMenu(QAction * before, QMenu * menu);
  fn _ZN8QMenuBar10insertMenuEP7QActionP5QMenu(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  void QMenuBar::setDefaultUp(bool );
  fn _ZN8QMenuBar12setDefaultUpEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QMenuBar::setVisible(bool visible);
  fn _ZN8QMenuBar10setVisibleEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  QAction * QMenuBar::addAction(const QString & text, const QObject * receiver, const char * member);
  fn _ZN8QMenuBar9addActionERK7QStringPK7QObjectPKc(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *const c_char) -> *mut c_void;
  // proto:  int QMenuBar::heightForWidth(int );
  fn _ZNK8QMenuBar14heightForWidthEi(qthis: *mut c_void, arg0: c_int) -> c_int;
}

// body block begin
// class sizeof(QMenuBar)=1
pub struct QMenuBar {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QMenuBar {
  pub fn addAction<RetType, T: QMenuBar_addAction<RetType>>(&mut self, value: T) -> RetType {
    return value.addAction(self);
    // return 1;
  }
}

pub trait QMenuBar_addAction<RetType> {
  fn addAction(self, rsthis: &mut QMenuBar) -> RetType;
}

// proto:  QAction * QMenuBar::addAction(const QString & text);
impl<'a> /*trait*/ QMenuBar_addAction<QAction> for (&'a  QString) {
  fn addAction(self, rsthis: &mut QMenuBar) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMenuBar9addActionERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN8QMenuBar9addActionERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QAction{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMenuBar {
  pub fn platformMenuBar<RetType, T: QMenuBar_platformMenuBar<RetType>>(&mut self, value: T) -> RetType {
    return value.platformMenuBar(self);
    // return 1;
  }
}

pub trait QMenuBar_platformMenuBar<RetType> {
  fn platformMenuBar(self, rsthis: &mut QMenuBar) -> RetType;
}

// proto:  QPlatformMenuBar * QMenuBar::platformMenuBar();
impl<'a> /*trait*/ QMenuBar_platformMenuBar<()> for () {
  fn platformMenuBar(self, rsthis: &mut QMenuBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMenuBar15platformMenuBarEv()};
     unsafe {_ZN8QMenuBar15platformMenuBarEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QMenuBar {
  pub fn setNativeMenuBar<RetType, T: QMenuBar_setNativeMenuBar<RetType>>(&mut self, value: T) -> RetType {
    return value.setNativeMenuBar(self);
    // return 1;
  }
}

pub trait QMenuBar_setNativeMenuBar<RetType> {
  fn setNativeMenuBar(self, rsthis: &mut QMenuBar) -> RetType;
}

// proto:  void QMenuBar::setNativeMenuBar(bool nativeMenuBar);
impl<'a> /*trait*/ QMenuBar_setNativeMenuBar<()> for (i8) {
  fn setNativeMenuBar(self, rsthis: &mut QMenuBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMenuBar16setNativeMenuBarEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN8QMenuBar16setNativeMenuBarEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMenuBar {
  pub fn FreeQMenuBar<RetType, T: QMenuBar_FreeQMenuBar<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQMenuBar(self);
    // return 1;
  }
}

pub trait QMenuBar_FreeQMenuBar<RetType> {
  fn FreeQMenuBar(self, rsthis: &mut QMenuBar) -> RetType;
}

// proto:  void QMenuBar::FreeQMenuBar();
impl<'a> /*trait*/ QMenuBar_FreeQMenuBar<()> for () {
  fn FreeQMenuBar(self, rsthis: &mut QMenuBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMenuBarD0Ev()};
     unsafe {_ZN8QMenuBarD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QMenuBar {
  pub fn addMenu<RetType, T: QMenuBar_addMenu<RetType>>(&mut self, value: T) -> RetType {
    return value.addMenu(self);
    // return 1;
  }
}

pub trait QMenuBar_addMenu<RetType> {
  fn addMenu(self, rsthis: &mut QMenuBar) -> RetType;
}

// proto:  QAction * QMenuBar::addMenu(QMenu * menu);
impl<'a> /*trait*/ QMenuBar_addMenu<QAction> for (&'a mut QMenu) {
  fn addMenu(self, rsthis: &mut QMenuBar) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMenuBar7addMenuEP5QMenu()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN8QMenuBar7addMenuEP5QMenu(rsthis.qclsinst, arg0)};
    let mut ret1 = QAction{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMenuBar {
  pub fn triggered<RetType, T: QMenuBar_triggered<RetType>>(&mut self, value: T) -> RetType {
    return value.triggered(self);
    // return 1;
  }
}

pub trait QMenuBar_triggered<RetType> {
  fn triggered(self, rsthis: &mut QMenuBar) -> RetType;
}

// proto:  void QMenuBar::triggered(QAction * action);
impl<'a> /*trait*/ QMenuBar_triggered<()> for (&'a mut QAction) {
  fn triggered(self, rsthis: &mut QMenuBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMenuBar9triggeredEP7QAction()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QMenuBar9triggeredEP7QAction(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMenuBar {
  pub fn sizeHint<RetType, T: QMenuBar_sizeHint<RetType>>(&mut self, value: T) -> RetType {
    return value.sizeHint(self);
    // return 1;
  }
}

pub trait QMenuBar_sizeHint<RetType> {
  fn sizeHint(self, rsthis: &mut QMenuBar) -> RetType;
}

// proto:  QSize QMenuBar::sizeHint();
impl<'a> /*trait*/ QMenuBar_sizeHint<QSize> for () {
  fn sizeHint(self, rsthis: &mut QMenuBar) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QMenuBar8sizeHintEv()};
    let mut ret = unsafe {_ZNK8QMenuBar8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMenuBar {
  pub fn actionAt<RetType, T: QMenuBar_actionAt<RetType>>(&mut self, value: T) -> RetType {
    return value.actionAt(self);
    // return 1;
  }
}

pub trait QMenuBar_actionAt<RetType> {
  fn actionAt(self, rsthis: &mut QMenuBar) -> RetType;
}

// proto:  QAction * QMenuBar::actionAt(const QPoint & );
impl<'a> /*trait*/ QMenuBar_actionAt<QAction> for (&'a  QPoint) {
  fn actionAt(self, rsthis: &mut QMenuBar) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QMenuBar8actionAtERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK8QMenuBar8actionAtERK6QPoint(rsthis.qclsinst, arg0)};
    let mut ret1 = QAction{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMenuBar {
  pub fn hovered<RetType, T: QMenuBar_hovered<RetType>>(&mut self, value: T) -> RetType {
    return value.hovered(self);
    // return 1;
  }
}

pub trait QMenuBar_hovered<RetType> {
  fn hovered(self, rsthis: &mut QMenuBar) -> RetType;
}

// proto:  void QMenuBar::hovered(QAction * action);
impl<'a> /*trait*/ QMenuBar_hovered<()> for (&'a mut QAction) {
  fn hovered(self, rsthis: &mut QMenuBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMenuBar7hoveredEP7QAction()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QMenuBar7hoveredEP7QAction(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMenuBar {
  pub fn metaObject<RetType, T: QMenuBar_metaObject<RetType>>(&mut self, value: T) -> RetType {
    return value.metaObject(self);
    // return 1;
  }
}

pub trait QMenuBar_metaObject<RetType> {
  fn metaObject(self, rsthis: &mut QMenuBar) -> RetType;
}

// proto:  const QMetaObject * QMenuBar::metaObject();
impl<'a> /*trait*/ QMenuBar_metaObject<()> for () {
  fn metaObject(self, rsthis: &mut QMenuBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QMenuBar10metaObjectEv()};
     unsafe {_ZNK8QMenuBar10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QMenuBar {
  pub fn isNativeMenuBar<RetType, T: QMenuBar_isNativeMenuBar<RetType>>(&mut self, value: T) -> RetType {
    return value.isNativeMenuBar(self);
    // return 1;
  }
}

pub trait QMenuBar_isNativeMenuBar<RetType> {
  fn isNativeMenuBar(self, rsthis: &mut QMenuBar) -> RetType;
}

// proto:  bool QMenuBar::isNativeMenuBar();
impl<'a> /*trait*/ QMenuBar_isNativeMenuBar<i8> for () {
  fn isNativeMenuBar(self, rsthis: &mut QMenuBar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QMenuBar15isNativeMenuBarEv()};
    let mut ret = unsafe {_ZNK8QMenuBar15isNativeMenuBarEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QMenuBar {
  pub fn insertSeparator<RetType, T: QMenuBar_insertSeparator<RetType>>(&mut self, value: T) -> RetType {
    return value.insertSeparator(self);
    // return 1;
  }
}

pub trait QMenuBar_insertSeparator<RetType> {
  fn insertSeparator(self, rsthis: &mut QMenuBar) -> RetType;
}

// proto:  QAction * QMenuBar::insertSeparator(QAction * before);
impl<'a> /*trait*/ QMenuBar_insertSeparator<QAction> for (&'a mut QAction) {
  fn insertSeparator(self, rsthis: &mut QMenuBar) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMenuBar15insertSeparatorEP7QAction()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN8QMenuBar15insertSeparatorEP7QAction(rsthis.qclsinst, arg0)};
    let mut ret1 = QAction{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMenuBar {
  pub fn addSeparator<RetType, T: QMenuBar_addSeparator<RetType>>(&mut self, value: T) -> RetType {
    return value.addSeparator(self);
    // return 1;
  }
}

pub trait QMenuBar_addSeparator<RetType> {
  fn addSeparator(self, rsthis: &mut QMenuBar) -> RetType;
}

// proto:  QAction * QMenuBar::addSeparator();
impl<'a> /*trait*/ QMenuBar_addSeparator<QAction> for () {
  fn addSeparator(self, rsthis: &mut QMenuBar) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMenuBar12addSeparatorEv()};
    let mut ret = unsafe {_ZN8QMenuBar12addSeparatorEv(rsthis.qclsinst)};
    let mut ret1 = QAction{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMenuBar {
  pub fn minimumSizeHint<RetType, T: QMenuBar_minimumSizeHint<RetType>>(&mut self, value: T) -> RetType {
    return value.minimumSizeHint(self);
    // return 1;
  }
}

pub trait QMenuBar_minimumSizeHint<RetType> {
  fn minimumSizeHint(self, rsthis: &mut QMenuBar) -> RetType;
}

// proto:  QSize QMenuBar::minimumSizeHint();
impl<'a> /*trait*/ QMenuBar_minimumSizeHint<QSize> for () {
  fn minimumSizeHint(self, rsthis: &mut QMenuBar) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QMenuBar15minimumSizeHintEv()};
    let mut ret = unsafe {_ZNK8QMenuBar15minimumSizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMenuBar {
  pub fn isDefaultUp<RetType, T: QMenuBar_isDefaultUp<RetType>>(&mut self, value: T) -> RetType {
    return value.isDefaultUp(self);
    // return 1;
  }
}

pub trait QMenuBar_isDefaultUp<RetType> {
  fn isDefaultUp(self, rsthis: &mut QMenuBar) -> RetType;
}

// proto:  bool QMenuBar::isDefaultUp();
impl<'a> /*trait*/ QMenuBar_isDefaultUp<i8> for () {
  fn isDefaultUp(self, rsthis: &mut QMenuBar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QMenuBar11isDefaultUpEv()};
    let mut ret = unsafe {_ZNK8QMenuBar11isDefaultUpEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QMenuBar {
  pub fn NewQMenuBar<T: QMenuBar_NewQMenuBar>(value: T) -> QMenuBar {
    let rsthis = value.NewQMenuBar();
    return rsthis;
    // return 1;
  }
}

pub trait QMenuBar_NewQMenuBar {
  fn NewQMenuBar(self) -> QMenuBar;
}

// proto: void QMenuBar::NewQMenuBar(const QMenuBar & );
impl<'a> /*trait*/ QMenuBar_NewQMenuBar for (&'a  QMenuBar) {
  fn NewQMenuBar(self) -> QMenuBar {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMenuBarC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN8QMenuBarC1ERKS_(qthis, arg0)};
    let rsthis = QMenuBar{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QMenuBar::NewQMenuBar(QWidget * parent);
impl<'a> /*trait*/ QMenuBar_NewQMenuBar for (&'a mut QWidget) {
  fn NewQMenuBar(self) -> QMenuBar {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMenuBarC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN8QMenuBarC1EP7QWidget(qthis, arg0)};
    let rsthis = QMenuBar{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QMenuBar {
  pub fn setActiveAction<RetType, T: QMenuBar_setActiveAction<RetType>>(&mut self, value: T) -> RetType {
    return value.setActiveAction(self);
    // return 1;
  }
}

pub trait QMenuBar_setActiveAction<RetType> {
  fn setActiveAction(self, rsthis: &mut QMenuBar) -> RetType;
}

// proto:  void QMenuBar::setActiveAction(QAction * action);
impl<'a> /*trait*/ QMenuBar_setActiveAction<()> for (&'a mut QAction) {
  fn setActiveAction(self, rsthis: &mut QMenuBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMenuBar15setActiveActionEP7QAction()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN8QMenuBar15setActiveActionEP7QAction(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMenuBar {
  pub fn clear<RetType, T: QMenuBar_clear<RetType>>(&mut self, value: T) -> RetType {
    return value.clear(self);
    // return 1;
  }
}

pub trait QMenuBar_clear<RetType> {
  fn clear(self, rsthis: &mut QMenuBar) -> RetType;
}

// proto:  void QMenuBar::clear();
impl<'a> /*trait*/ QMenuBar_clear<()> for () {
  fn clear(self, rsthis: &mut QMenuBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMenuBar5clearEv()};
     unsafe {_ZN8QMenuBar5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QMenuBar {
  pub fn activeAction<RetType, T: QMenuBar_activeAction<RetType>>(&mut self, value: T) -> RetType {
    return value.activeAction(self);
    // return 1;
  }
}

pub trait QMenuBar_activeAction<RetType> {
  fn activeAction(self, rsthis: &mut QMenuBar) -> RetType;
}

// proto:  QAction * QMenuBar::activeAction();
impl<'a> /*trait*/ QMenuBar_activeAction<QAction> for () {
  fn activeAction(self, rsthis: &mut QMenuBar) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QMenuBar12activeActionEv()};
    let mut ret = unsafe {_ZNK8QMenuBar12activeActionEv(rsthis.qclsinst)};
    let mut ret1 = QAction{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QMenu * QMenuBar::addMenu(const QIcon & icon, const QString & title);
impl<'a> /*trait*/ QMenuBar_addMenu<QMenu> for (&'a  QIcon, &'a  QString) {
  fn addMenu(self, rsthis: &mut QMenuBar) -> QMenu {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMenuBar7addMenuERK5QIconRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN8QMenuBar7addMenuERK5QIconRK7QString(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QMenu{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QMenu * QMenuBar::addMenu(const QString & title);
impl<'a> /*trait*/ QMenuBar_addMenu<QMenu> for (&'a  QString) {
  fn addMenu(self, rsthis: &mut QMenuBar) -> QMenu {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMenuBar7addMenuERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN8QMenuBar7addMenuERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QMenu{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMenuBar {
  pub fn actionGeometry<RetType, T: QMenuBar_actionGeometry<RetType>>(&mut self, value: T) -> RetType {
    return value.actionGeometry(self);
    // return 1;
  }
}

pub trait QMenuBar_actionGeometry<RetType> {
  fn actionGeometry(self, rsthis: &mut QMenuBar) -> RetType;
}

// proto:  QRect QMenuBar::actionGeometry(QAction * );
impl<'a> /*trait*/ QMenuBar_actionGeometry<QRect> for (&'a mut QAction) {
  fn actionGeometry(self, rsthis: &mut QMenuBar) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QMenuBar14actionGeometryEP7QAction()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK8QMenuBar14actionGeometryEP7QAction(rsthis.qclsinst, arg0)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMenuBar {
  pub fn insertMenu<RetType, T: QMenuBar_insertMenu<RetType>>(&mut self, value: T) -> RetType {
    return value.insertMenu(self);
    // return 1;
  }
}

pub trait QMenuBar_insertMenu<RetType> {
  fn insertMenu(self, rsthis: &mut QMenuBar) -> RetType;
}

// proto:  QAction * QMenuBar::insertMenu(QAction * before, QMenu * menu);
impl<'a> /*trait*/ QMenuBar_insertMenu<QAction> for (&'a mut QAction, &'a mut QMenu) {
  fn insertMenu(self, rsthis: &mut QMenuBar) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMenuBar10insertMenuEP7QActionP5QMenu()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN8QMenuBar10insertMenuEP7QActionP5QMenu(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QAction{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMenuBar {
  pub fn setDefaultUp<RetType, T: QMenuBar_setDefaultUp<RetType>>(&mut self, value: T) -> RetType {
    return value.setDefaultUp(self);
    // return 1;
  }
}

pub trait QMenuBar_setDefaultUp<RetType> {
  fn setDefaultUp(self, rsthis: &mut QMenuBar) -> RetType;
}

// proto:  void QMenuBar::setDefaultUp(bool );
impl<'a> /*trait*/ QMenuBar_setDefaultUp<()> for (i8) {
  fn setDefaultUp(self, rsthis: &mut QMenuBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMenuBar12setDefaultUpEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN8QMenuBar12setDefaultUpEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMenuBar {
  pub fn setVisible<RetType, T: QMenuBar_setVisible<RetType>>(&mut self, value: T) -> RetType {
    return value.setVisible(self);
    // return 1;
  }
}

pub trait QMenuBar_setVisible<RetType> {
  fn setVisible(self, rsthis: &mut QMenuBar) -> RetType;
}

// proto:  void QMenuBar::setVisible(bool visible);
impl<'a> /*trait*/ QMenuBar_setVisible<()> for (i8) {
  fn setVisible(self, rsthis: &mut QMenuBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMenuBar10setVisibleEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN8QMenuBar10setVisibleEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  QAction * QMenuBar::addAction(const QString & text, const QObject * receiver, const char * member);
impl<'a> /*trait*/ QMenuBar_addAction<QAction> for (&'a  QString, &'a  QObject, &'a  String) {
  fn addAction(self, rsthis: &mut QMenuBar) -> QAction {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QMenuBar9addActionERK7QStringPK7QObjectPKc()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.as_ptr()  as *const c_char;
    let mut ret = unsafe {_ZN8QMenuBar9addActionERK7QStringPK7QObjectPKc(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QAction{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMenuBar {
  pub fn heightForWidth<RetType, T: QMenuBar_heightForWidth<RetType>>(&mut self, value: T) -> RetType {
    return value.heightForWidth(self);
    // return 1;
  }
}

pub trait QMenuBar_heightForWidth<RetType> {
  fn heightForWidth(self, rsthis: &mut QMenuBar) -> RetType;
}

// proto:  int QMenuBar::heightForWidth(int );
impl<'a> /*trait*/ QMenuBar_heightForWidth<i32> for (i32) {
  fn heightForWidth(self, rsthis: &mut QMenuBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QMenuBar14heightForWidthEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK8QMenuBar14heightForWidthEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

