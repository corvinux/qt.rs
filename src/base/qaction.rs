// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;
use super::qobject::QObject;
use super::qfont::QFont;
use super::qvariant::QVariant;
use super::qicon::QIcon;
use super::qwidget::QWidget;
use super::qmenu::QMenu;
use super::qkeysequence::QKeySequence;
use super::qactiongroup::QActionGroup;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  QList<QWidget *> QAction::associatedWidgets();
  fn _ZNK7QAction17associatedWidgetsEv(qthis: *mut c_void) ;
  // proto:  void QAction::setAutoRepeat(bool );
  fn _ZN7QAction13setAutoRepeatEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  QString QAction::whatsThis();
  fn _ZNK7QAction9whatsThisEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QAction::NewQAction(const QString & text, QObject * parent);
  fn _ZN7QActionC1ERK7QStringP7QObject(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QAction::FreeQAction();
  fn _ZN7QActionD0Ev(qthis: *mut c_void) ;
  // proto:  bool QAction::isVisible();
  fn _ZNK7QAction9isVisibleEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QAction::setFont(const QFont & font);
  fn _ZN7QAction7setFontERK5QFont(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QAction::setData(const QVariant & var);
  fn _ZN7QAction7setDataERK8QVariant(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QAction::setIcon(const QIcon & icon);
  fn _ZN7QAction7setIconERK5QIcon(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QAction::NewQAction(QObject * parent);
  fn _ZN7QActionC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  const QMetaObject * QAction::metaObject();
  fn _ZNK7QAction10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QAction::triggered(bool checked);
  fn _ZN7QAction9triggeredEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QAction::toggled(bool );
  fn _ZN7QAction7toggledEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QAction::setText(const QString & text);
  fn _ZN7QAction7setTextERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QAction::showStatusText(QWidget * widget);
  fn _ZN7QAction14showStatusTextEP7QWidget(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  QString QAction::iconText();
  fn _ZNK7QAction8iconTextEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QAction::setIconVisibleInMenu(bool visible);
  fn _ZN7QAction20setIconVisibleInMenuEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  QString QAction::statusTip();
  fn _ZNK7QAction9statusTipEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QAction::isCheckable();
  fn _ZNK7QAction11isCheckableEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QAction::setWhatsThis(const QString & what);
  fn _ZN7QAction12setWhatsThisERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QMenu * QAction::menu();
  fn _ZNK7QAction4menuEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QAction::trigger();
  fn _ZN7QAction7triggerEv(qthis: *mut c_void) ;
  // proto:  void QAction::changed();
  fn _ZN7QAction7changedEv(qthis: *mut c_void) ;
  // proto:  QFont QAction::font();
  fn _ZNK7QAction4fontEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QAction::NewQAction(const QIcon & icon, const QString & text, QObject * parent);
  fn _ZN7QActionC1ERK5QIconRK7QStringP7QObject(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) ;
  // proto:  void QAction::setChecked(bool );
  fn _ZN7QAction10setCheckedEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QAction::setDisabled(bool b);
  fn _ZN7QAction11setDisabledEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QAction::setShortcut(const QKeySequence & shortcut);
  fn _ZN7QAction11setShortcutERK12QKeySequence(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QAction::toggle();
  fn _ZN7QAction6toggleEv(qthis: *mut c_void) ;
  // proto:  QKeySequence QAction::shortcut();
  fn _ZNK7QAction8shortcutEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QIcon QAction::icon();
  fn _ZNK7QAction4iconEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QAction::setActionGroup(QActionGroup * group);
  fn _ZN7QAction14setActionGroupEP12QActionGroup(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QAction::setMenu(QMenu * menu);
  fn _ZN7QAction7setMenuEP5QMenu(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QList<QKeySequence> QAction::shortcuts();
  fn _ZNK7QAction9shortcutsEv(qthis: *mut c_void) ;
  // proto:  void QAction::setCheckable(bool );
  fn _ZN7QAction12setCheckableEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  QString QAction::toolTip();
  fn _ZNK7QAction7toolTipEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QWidget * QAction::parentWidget();
  fn _ZNK7QAction12parentWidgetEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QAction::setSeparator(bool b);
  fn _ZN7QAction12setSeparatorEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  QList<QGraphicsWidget *> QAction::associatedGraphicsWidgets();
  fn _ZNK7QAction25associatedGraphicsWidgetsEv(qthis: *mut c_void) ;
  // proto:  void QAction::setVisible(bool );
  fn _ZN7QAction10setVisibleEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  bool QAction::isSeparator();
  fn _ZNK7QAction11isSeparatorEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QAction::setIconText(const QString & text);
  fn _ZN7QAction11setIconTextERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QActionGroup * QAction::actionGroup();
  fn _ZNK7QAction11actionGroupEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QAction::setStatusTip(const QString & statusTip);
  fn _ZN7QAction12setStatusTipERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QAction::hovered();
  fn _ZN7QAction7hoveredEv(qthis: *mut c_void) ;
  // proto:  QVariant QAction::data();
  fn _ZNK7QAction4dataEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QAction::NewQAction(const QAction & );
  fn _ZN7QActionC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QAction::isChecked();
  fn _ZNK7QAction9isCheckedEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QAction::autoRepeat();
  fn _ZNK7QAction10autoRepeatEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QAction::isEnabled();
  fn _ZNK7QAction9isEnabledEv(qthis: *mut c_void) -> int8_t;
  // proto:  QString QAction::text();
  fn _ZNK7QAction4textEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QAction::hover();
  fn _ZN7QAction5hoverEv(qthis: *mut c_void) ;
  // proto:  bool QAction::isIconVisibleInMenu();
  fn _ZNK7QAction19isIconVisibleInMenuEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QAction::setToolTip(const QString & tip);
  fn _ZN7QAction10setToolTipERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QAction::setEnabled(bool );
  fn _ZN7QAction10setEnabledEb(qthis: *mut c_void, arg0: int8_t) ;
}

// body block begin
// class sizeof(QAction)=1
pub struct QAction {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QAction {
  pub fn associatedWidgets<RetType, T: QAction_associatedWidgets<RetType>>(&mut self, value: T) -> RetType {
    return value.associatedWidgets(self);
    // return 1;
  }
}

pub trait QAction_associatedWidgets<RetType> {
  fn associatedWidgets(self, rsthis: &mut QAction) -> RetType;
}

// proto:  QList<QWidget *> QAction::associatedWidgets();
impl<'a> /*trait*/ QAction_associatedWidgets<()> for () {
  fn associatedWidgets(self, rsthis: &mut QAction) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QAction17associatedWidgetsEv()};
     unsafe {_ZNK7QAction17associatedWidgetsEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QAction {
  pub fn setAutoRepeat<RetType, T: QAction_setAutoRepeat<RetType>>(&mut self, value: T) -> RetType {
    return value.setAutoRepeat(self);
    // return 1;
  }
}

pub trait QAction_setAutoRepeat<RetType> {
  fn setAutoRepeat(self, rsthis: &mut QAction) -> RetType;
}

// proto:  void QAction::setAutoRepeat(bool );
impl<'a> /*trait*/ QAction_setAutoRepeat<()> for (i8) {
  fn setAutoRepeat(self, rsthis: &mut QAction) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QAction13setAutoRepeatEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN7QAction13setAutoRepeatEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QAction {
  pub fn whatsThis<RetType, T: QAction_whatsThis<RetType>>(&mut self, value: T) -> RetType {
    return value.whatsThis(self);
    // return 1;
  }
}

pub trait QAction_whatsThis<RetType> {
  fn whatsThis(self, rsthis: &mut QAction) -> RetType;
}

// proto:  QString QAction::whatsThis();
impl<'a> /*trait*/ QAction_whatsThis<QString> for () {
  fn whatsThis(self, rsthis: &mut QAction) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QAction9whatsThisEv()};
    let mut ret = unsafe {_ZNK7QAction9whatsThisEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QAction {
  pub fn NewQAction<T: QAction_NewQAction>(value: T) -> QAction {
    let rsthis = value.NewQAction();
    return rsthis;
    // return 1;
  }
}

pub trait QAction_NewQAction {
  fn NewQAction(self) -> QAction;
}

// proto: void QAction::NewQAction(const QString & text, QObject * parent);
impl<'a> /*trait*/ QAction_NewQAction for (&'a  QString, &'a mut QObject) {
  fn NewQAction(self) -> QAction {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QActionC1ERK7QStringP7QObject()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN7QActionC1ERK7QStringP7QObject(qthis, arg0, arg1)};
    let rsthis = QAction{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QAction {
  pub fn FreeQAction<RetType, T: QAction_FreeQAction<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQAction(self);
    // return 1;
  }
}

pub trait QAction_FreeQAction<RetType> {
  fn FreeQAction(self, rsthis: &mut QAction) -> RetType;
}

// proto:  void QAction::FreeQAction();
impl<'a> /*trait*/ QAction_FreeQAction<()> for () {
  fn FreeQAction(self, rsthis: &mut QAction) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QActionD0Ev()};
     unsafe {_ZN7QActionD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QAction {
  pub fn isVisible<RetType, T: QAction_isVisible<RetType>>(&mut self, value: T) -> RetType {
    return value.isVisible(self);
    // return 1;
  }
}

pub trait QAction_isVisible<RetType> {
  fn isVisible(self, rsthis: &mut QAction) -> RetType;
}

// proto:  bool QAction::isVisible();
impl<'a> /*trait*/ QAction_isVisible<i8> for () {
  fn isVisible(self, rsthis: &mut QAction) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QAction9isVisibleEv()};
    let mut ret = unsafe {_ZNK7QAction9isVisibleEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QAction {
  pub fn setFont<RetType, T: QAction_setFont<RetType>>(&mut self, value: T) -> RetType {
    return value.setFont(self);
    // return 1;
  }
}

pub trait QAction_setFont<RetType> {
  fn setFont(self, rsthis: &mut QAction) -> RetType;
}

// proto:  void QAction::setFont(const QFont & font);
impl<'a> /*trait*/ QAction_setFont<()> for (&'a  QFont) {
  fn setFont(self, rsthis: &mut QAction) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QAction7setFontERK5QFont()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QAction7setFontERK5QFont(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QAction {
  pub fn setData<RetType, T: QAction_setData<RetType>>(&mut self, value: T) -> RetType {
    return value.setData(self);
    // return 1;
  }
}

pub trait QAction_setData<RetType> {
  fn setData(self, rsthis: &mut QAction) -> RetType;
}

// proto:  void QAction::setData(const QVariant & var);
impl<'a> /*trait*/ QAction_setData<()> for (&'a  QVariant) {
  fn setData(self, rsthis: &mut QAction) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QAction7setDataERK8QVariant()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QAction7setDataERK8QVariant(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QAction {
  pub fn setIcon<RetType, T: QAction_setIcon<RetType>>(&mut self, value: T) -> RetType {
    return value.setIcon(self);
    // return 1;
  }
}

pub trait QAction_setIcon<RetType> {
  fn setIcon(self, rsthis: &mut QAction) -> RetType;
}

// proto:  void QAction::setIcon(const QIcon & icon);
impl<'a> /*trait*/ QAction_setIcon<()> for (&'a  QIcon) {
  fn setIcon(self, rsthis: &mut QAction) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QAction7setIconERK5QIcon()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QAction7setIconERK5QIcon(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto: void QAction::NewQAction(QObject * parent);
impl<'a> /*trait*/ QAction_NewQAction for (&'a mut QObject) {
  fn NewQAction(self) -> QAction {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QActionC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QActionC1EP7QObject(qthis, arg0)};
    let rsthis = QAction{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QAction {
  pub fn metaObject<RetType, T: QAction_metaObject<RetType>>(&mut self, value: T) -> RetType {
    return value.metaObject(self);
    // return 1;
  }
}

pub trait QAction_metaObject<RetType> {
  fn metaObject(self, rsthis: &mut QAction) -> RetType;
}

// proto:  const QMetaObject * QAction::metaObject();
impl<'a> /*trait*/ QAction_metaObject<()> for () {
  fn metaObject(self, rsthis: &mut QAction) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QAction10metaObjectEv()};
     unsafe {_ZNK7QAction10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QAction {
  pub fn triggered<RetType, T: QAction_triggered<RetType>>(&mut self, value: T) -> RetType {
    return value.triggered(self);
    // return 1;
  }
}

pub trait QAction_triggered<RetType> {
  fn triggered(self, rsthis: &mut QAction) -> RetType;
}

// proto:  void QAction::triggered(bool checked);
impl<'a> /*trait*/ QAction_triggered<()> for (i8) {
  fn triggered(self, rsthis: &mut QAction) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QAction9triggeredEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN7QAction9triggeredEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QAction {
  pub fn toggled<RetType, T: QAction_toggled<RetType>>(&mut self, value: T) -> RetType {
    return value.toggled(self);
    // return 1;
  }
}

pub trait QAction_toggled<RetType> {
  fn toggled(self, rsthis: &mut QAction) -> RetType;
}

// proto:  void QAction::toggled(bool );
impl<'a> /*trait*/ QAction_toggled<()> for (i8) {
  fn toggled(self, rsthis: &mut QAction) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QAction7toggledEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN7QAction7toggledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QAction {
  pub fn setText<RetType, T: QAction_setText<RetType>>(&mut self, value: T) -> RetType {
    return value.setText(self);
    // return 1;
  }
}

pub trait QAction_setText<RetType> {
  fn setText(self, rsthis: &mut QAction) -> RetType;
}

// proto:  void QAction::setText(const QString & text);
impl<'a> /*trait*/ QAction_setText<()> for (&'a  QString) {
  fn setText(self, rsthis: &mut QAction) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QAction7setTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QAction7setTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QAction {
  pub fn showStatusText<RetType, T: QAction_showStatusText<RetType>>(&mut self, value: T) -> RetType {
    return value.showStatusText(self);
    // return 1;
  }
}

pub trait QAction_showStatusText<RetType> {
  fn showStatusText(self, rsthis: &mut QAction) -> RetType;
}

// proto:  bool QAction::showStatusText(QWidget * widget);
impl<'a> /*trait*/ QAction_showStatusText<i8> for (&'a mut QWidget) {
  fn showStatusText(self, rsthis: &mut QAction) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QAction14showStatusTextEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN7QAction14showStatusTextEP7QWidget(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QAction {
  pub fn iconText<RetType, T: QAction_iconText<RetType>>(&mut self, value: T) -> RetType {
    return value.iconText(self);
    // return 1;
  }
}

pub trait QAction_iconText<RetType> {
  fn iconText(self, rsthis: &mut QAction) -> RetType;
}

// proto:  QString QAction::iconText();
impl<'a> /*trait*/ QAction_iconText<QString> for () {
  fn iconText(self, rsthis: &mut QAction) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QAction8iconTextEv()};
    let mut ret = unsafe {_ZNK7QAction8iconTextEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QAction {
  pub fn setIconVisibleInMenu<RetType, T: QAction_setIconVisibleInMenu<RetType>>(&mut self, value: T) -> RetType {
    return value.setIconVisibleInMenu(self);
    // return 1;
  }
}

pub trait QAction_setIconVisibleInMenu<RetType> {
  fn setIconVisibleInMenu(self, rsthis: &mut QAction) -> RetType;
}

// proto:  void QAction::setIconVisibleInMenu(bool visible);
impl<'a> /*trait*/ QAction_setIconVisibleInMenu<()> for (i8) {
  fn setIconVisibleInMenu(self, rsthis: &mut QAction) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QAction20setIconVisibleInMenuEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN7QAction20setIconVisibleInMenuEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QAction {
  pub fn statusTip<RetType, T: QAction_statusTip<RetType>>(&mut self, value: T) -> RetType {
    return value.statusTip(self);
    // return 1;
  }
}

pub trait QAction_statusTip<RetType> {
  fn statusTip(self, rsthis: &mut QAction) -> RetType;
}

// proto:  QString QAction::statusTip();
impl<'a> /*trait*/ QAction_statusTip<QString> for () {
  fn statusTip(self, rsthis: &mut QAction) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QAction9statusTipEv()};
    let mut ret = unsafe {_ZNK7QAction9statusTipEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QAction {
  pub fn isCheckable<RetType, T: QAction_isCheckable<RetType>>(&mut self, value: T) -> RetType {
    return value.isCheckable(self);
    // return 1;
  }
}

pub trait QAction_isCheckable<RetType> {
  fn isCheckable(self, rsthis: &mut QAction) -> RetType;
}

// proto:  bool QAction::isCheckable();
impl<'a> /*trait*/ QAction_isCheckable<i8> for () {
  fn isCheckable(self, rsthis: &mut QAction) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QAction11isCheckableEv()};
    let mut ret = unsafe {_ZNK7QAction11isCheckableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QAction {
  pub fn setWhatsThis<RetType, T: QAction_setWhatsThis<RetType>>(&mut self, value: T) -> RetType {
    return value.setWhatsThis(self);
    // return 1;
  }
}

pub trait QAction_setWhatsThis<RetType> {
  fn setWhatsThis(self, rsthis: &mut QAction) -> RetType;
}

// proto:  void QAction::setWhatsThis(const QString & what);
impl<'a> /*trait*/ QAction_setWhatsThis<()> for (&'a  QString) {
  fn setWhatsThis(self, rsthis: &mut QAction) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QAction12setWhatsThisERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QAction12setWhatsThisERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QAction {
  pub fn menu<RetType, T: QAction_menu<RetType>>(&mut self, value: T) -> RetType {
    return value.menu(self);
    // return 1;
  }
}

pub trait QAction_menu<RetType> {
  fn menu(self, rsthis: &mut QAction) -> RetType;
}

// proto:  QMenu * QAction::menu();
impl<'a> /*trait*/ QAction_menu<QMenu> for () {
  fn menu(self, rsthis: &mut QAction) -> QMenu {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QAction4menuEv()};
    let mut ret = unsafe {_ZNK7QAction4menuEv(rsthis.qclsinst)};
    let mut ret1 = QMenu{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QAction {
  pub fn trigger<RetType, T: QAction_trigger<RetType>>(&mut self, value: T) -> RetType {
    return value.trigger(self);
    // return 1;
  }
}

pub trait QAction_trigger<RetType> {
  fn trigger(self, rsthis: &mut QAction) -> RetType;
}

// proto:  void QAction::trigger();
impl<'a> /*trait*/ QAction_trigger<()> for () {
  fn trigger(self, rsthis: &mut QAction) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QAction7triggerEv()};
     unsafe {_ZN7QAction7triggerEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QAction {
  pub fn changed<RetType, T: QAction_changed<RetType>>(&mut self, value: T) -> RetType {
    return value.changed(self);
    // return 1;
  }
}

pub trait QAction_changed<RetType> {
  fn changed(self, rsthis: &mut QAction) -> RetType;
}

// proto:  void QAction::changed();
impl<'a> /*trait*/ QAction_changed<()> for () {
  fn changed(self, rsthis: &mut QAction) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QAction7changedEv()};
     unsafe {_ZN7QAction7changedEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QAction {
  pub fn font<RetType, T: QAction_font<RetType>>(&mut self, value: T) -> RetType {
    return value.font(self);
    // return 1;
  }
}

pub trait QAction_font<RetType> {
  fn font(self, rsthis: &mut QAction) -> RetType;
}

// proto:  QFont QAction::font();
impl<'a> /*trait*/ QAction_font<QFont> for () {
  fn font(self, rsthis: &mut QAction) -> QFont {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QAction4fontEv()};
    let mut ret = unsafe {_ZNK7QAction4fontEv(rsthis.qclsinst)};
    let mut ret1 = QFont{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QAction::NewQAction(const QIcon & icon, const QString & text, QObject * parent);
impl<'a> /*trait*/ QAction_NewQAction for (&'a  QIcon, &'a  QString, &'a mut QObject) {
  fn NewQAction(self) -> QAction {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QActionC1ERK5QIconRK7QStringP7QObject()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZN7QActionC1ERK5QIconRK7QStringP7QObject(qthis, arg0, arg1, arg2)};
    let rsthis = QAction{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QAction {
  pub fn setChecked<RetType, T: QAction_setChecked<RetType>>(&mut self, value: T) -> RetType {
    return value.setChecked(self);
    // return 1;
  }
}

pub trait QAction_setChecked<RetType> {
  fn setChecked(self, rsthis: &mut QAction) -> RetType;
}

// proto:  void QAction::setChecked(bool );
impl<'a> /*trait*/ QAction_setChecked<()> for (i8) {
  fn setChecked(self, rsthis: &mut QAction) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QAction10setCheckedEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN7QAction10setCheckedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QAction {
  pub fn setDisabled<RetType, T: QAction_setDisabled<RetType>>(&mut self, value: T) -> RetType {
    return value.setDisabled(self);
    // return 1;
  }
}

pub trait QAction_setDisabled<RetType> {
  fn setDisabled(self, rsthis: &mut QAction) -> RetType;
}

// proto:  void QAction::setDisabled(bool b);
impl<'a> /*trait*/ QAction_setDisabled<()> for (i8) {
  fn setDisabled(self, rsthis: &mut QAction) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QAction11setDisabledEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN7QAction11setDisabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QAction {
  pub fn setShortcut<RetType, T: QAction_setShortcut<RetType>>(&mut self, value: T) -> RetType {
    return value.setShortcut(self);
    // return 1;
  }
}

pub trait QAction_setShortcut<RetType> {
  fn setShortcut(self, rsthis: &mut QAction) -> RetType;
}

// proto:  void QAction::setShortcut(const QKeySequence & shortcut);
impl<'a> /*trait*/ QAction_setShortcut<()> for (&'a  QKeySequence) {
  fn setShortcut(self, rsthis: &mut QAction) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QAction11setShortcutERK12QKeySequence()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QAction11setShortcutERK12QKeySequence(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QAction {
  pub fn toggle<RetType, T: QAction_toggle<RetType>>(&mut self, value: T) -> RetType {
    return value.toggle(self);
    // return 1;
  }
}

pub trait QAction_toggle<RetType> {
  fn toggle(self, rsthis: &mut QAction) -> RetType;
}

// proto:  void QAction::toggle();
impl<'a> /*trait*/ QAction_toggle<()> for () {
  fn toggle(self, rsthis: &mut QAction) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QAction6toggleEv()};
     unsafe {_ZN7QAction6toggleEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QAction {
  pub fn shortcut<RetType, T: QAction_shortcut<RetType>>(&mut self, value: T) -> RetType {
    return value.shortcut(self);
    // return 1;
  }
}

pub trait QAction_shortcut<RetType> {
  fn shortcut(self, rsthis: &mut QAction) -> RetType;
}

// proto:  QKeySequence QAction::shortcut();
impl<'a> /*trait*/ QAction_shortcut<QKeySequence> for () {
  fn shortcut(self, rsthis: &mut QAction) -> QKeySequence {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QAction8shortcutEv()};
    let mut ret = unsafe {_ZNK7QAction8shortcutEv(rsthis.qclsinst)};
    let mut ret1 = QKeySequence{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QAction {
  pub fn icon<RetType, T: QAction_icon<RetType>>(&mut self, value: T) -> RetType {
    return value.icon(self);
    // return 1;
  }
}

pub trait QAction_icon<RetType> {
  fn icon(self, rsthis: &mut QAction) -> RetType;
}

// proto:  QIcon QAction::icon();
impl<'a> /*trait*/ QAction_icon<QIcon> for () {
  fn icon(self, rsthis: &mut QAction) -> QIcon {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QAction4iconEv()};
    let mut ret = unsafe {_ZNK7QAction4iconEv(rsthis.qclsinst)};
    let mut ret1 = QIcon{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QAction {
  pub fn setActionGroup<RetType, T: QAction_setActionGroup<RetType>>(&mut self, value: T) -> RetType {
    return value.setActionGroup(self);
    // return 1;
  }
}

pub trait QAction_setActionGroup<RetType> {
  fn setActionGroup(self, rsthis: &mut QAction) -> RetType;
}

// proto:  void QAction::setActionGroup(QActionGroup * group);
impl<'a> /*trait*/ QAction_setActionGroup<()> for (&'a mut QActionGroup) {
  fn setActionGroup(self, rsthis: &mut QAction) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QAction14setActionGroupEP12QActionGroup()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QAction14setActionGroupEP12QActionGroup(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QAction {
  pub fn setMenu<RetType, T: QAction_setMenu<RetType>>(&mut self, value: T) -> RetType {
    return value.setMenu(self);
    // return 1;
  }
}

pub trait QAction_setMenu<RetType> {
  fn setMenu(self, rsthis: &mut QAction) -> RetType;
}

// proto:  void QAction::setMenu(QMenu * menu);
impl<'a> /*trait*/ QAction_setMenu<()> for (&'a mut QMenu) {
  fn setMenu(self, rsthis: &mut QAction) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QAction7setMenuEP5QMenu()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QAction7setMenuEP5QMenu(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QAction {
  pub fn shortcuts<RetType, T: QAction_shortcuts<RetType>>(&mut self, value: T) -> RetType {
    return value.shortcuts(self);
    // return 1;
  }
}

pub trait QAction_shortcuts<RetType> {
  fn shortcuts(self, rsthis: &mut QAction) -> RetType;
}

// proto:  QList<QKeySequence> QAction::shortcuts();
impl<'a> /*trait*/ QAction_shortcuts<()> for () {
  fn shortcuts(self, rsthis: &mut QAction) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QAction9shortcutsEv()};
     unsafe {_ZNK7QAction9shortcutsEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QAction {
  pub fn setCheckable<RetType, T: QAction_setCheckable<RetType>>(&mut self, value: T) -> RetType {
    return value.setCheckable(self);
    // return 1;
  }
}

pub trait QAction_setCheckable<RetType> {
  fn setCheckable(self, rsthis: &mut QAction) -> RetType;
}

// proto:  void QAction::setCheckable(bool );
impl<'a> /*trait*/ QAction_setCheckable<()> for (i8) {
  fn setCheckable(self, rsthis: &mut QAction) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QAction12setCheckableEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN7QAction12setCheckableEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QAction {
  pub fn toolTip<RetType, T: QAction_toolTip<RetType>>(&mut self, value: T) -> RetType {
    return value.toolTip(self);
    // return 1;
  }
}

pub trait QAction_toolTip<RetType> {
  fn toolTip(self, rsthis: &mut QAction) -> RetType;
}

// proto:  QString QAction::toolTip();
impl<'a> /*trait*/ QAction_toolTip<QString> for () {
  fn toolTip(self, rsthis: &mut QAction) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QAction7toolTipEv()};
    let mut ret = unsafe {_ZNK7QAction7toolTipEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QAction {
  pub fn parentWidget<RetType, T: QAction_parentWidget<RetType>>(&mut self, value: T) -> RetType {
    return value.parentWidget(self);
    // return 1;
  }
}

pub trait QAction_parentWidget<RetType> {
  fn parentWidget(self, rsthis: &mut QAction) -> RetType;
}

// proto:  QWidget * QAction::parentWidget();
impl<'a> /*trait*/ QAction_parentWidget<QWidget> for () {
  fn parentWidget(self, rsthis: &mut QAction) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QAction12parentWidgetEv()};
    let mut ret = unsafe {_ZNK7QAction12parentWidgetEv(rsthis.qclsinst)};
    let mut ret1 = QWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QAction {
  pub fn setSeparator<RetType, T: QAction_setSeparator<RetType>>(&mut self, value: T) -> RetType {
    return value.setSeparator(self);
    // return 1;
  }
}

pub trait QAction_setSeparator<RetType> {
  fn setSeparator(self, rsthis: &mut QAction) -> RetType;
}

// proto:  void QAction::setSeparator(bool b);
impl<'a> /*trait*/ QAction_setSeparator<()> for (i8) {
  fn setSeparator(self, rsthis: &mut QAction) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QAction12setSeparatorEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN7QAction12setSeparatorEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QAction {
  pub fn associatedGraphicsWidgets<RetType, T: QAction_associatedGraphicsWidgets<RetType>>(&mut self, value: T) -> RetType {
    return value.associatedGraphicsWidgets(self);
    // return 1;
  }
}

pub trait QAction_associatedGraphicsWidgets<RetType> {
  fn associatedGraphicsWidgets(self, rsthis: &mut QAction) -> RetType;
}

// proto:  QList<QGraphicsWidget *> QAction::associatedGraphicsWidgets();
impl<'a> /*trait*/ QAction_associatedGraphicsWidgets<()> for () {
  fn associatedGraphicsWidgets(self, rsthis: &mut QAction) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QAction25associatedGraphicsWidgetsEv()};
     unsafe {_ZNK7QAction25associatedGraphicsWidgetsEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QAction {
  pub fn setVisible<RetType, T: QAction_setVisible<RetType>>(&mut self, value: T) -> RetType {
    return value.setVisible(self);
    // return 1;
  }
}

pub trait QAction_setVisible<RetType> {
  fn setVisible(self, rsthis: &mut QAction) -> RetType;
}

// proto:  void QAction::setVisible(bool );
impl<'a> /*trait*/ QAction_setVisible<()> for (i8) {
  fn setVisible(self, rsthis: &mut QAction) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QAction10setVisibleEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN7QAction10setVisibleEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QAction {
  pub fn isSeparator<RetType, T: QAction_isSeparator<RetType>>(&mut self, value: T) -> RetType {
    return value.isSeparator(self);
    // return 1;
  }
}

pub trait QAction_isSeparator<RetType> {
  fn isSeparator(self, rsthis: &mut QAction) -> RetType;
}

// proto:  bool QAction::isSeparator();
impl<'a> /*trait*/ QAction_isSeparator<i8> for () {
  fn isSeparator(self, rsthis: &mut QAction) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QAction11isSeparatorEv()};
    let mut ret = unsafe {_ZNK7QAction11isSeparatorEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QAction {
  pub fn setIconText<RetType, T: QAction_setIconText<RetType>>(&mut self, value: T) -> RetType {
    return value.setIconText(self);
    // return 1;
  }
}

pub trait QAction_setIconText<RetType> {
  fn setIconText(self, rsthis: &mut QAction) -> RetType;
}

// proto:  void QAction::setIconText(const QString & text);
impl<'a> /*trait*/ QAction_setIconText<()> for (&'a  QString) {
  fn setIconText(self, rsthis: &mut QAction) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QAction11setIconTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QAction11setIconTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QAction {
  pub fn actionGroup<RetType, T: QAction_actionGroup<RetType>>(&mut self, value: T) -> RetType {
    return value.actionGroup(self);
    // return 1;
  }
}

pub trait QAction_actionGroup<RetType> {
  fn actionGroup(self, rsthis: &mut QAction) -> RetType;
}

// proto:  QActionGroup * QAction::actionGroup();
impl<'a> /*trait*/ QAction_actionGroup<QActionGroup> for () {
  fn actionGroup(self, rsthis: &mut QAction) -> QActionGroup {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QAction11actionGroupEv()};
    let mut ret = unsafe {_ZNK7QAction11actionGroupEv(rsthis.qclsinst)};
    let mut ret1 = QActionGroup{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QAction {
  pub fn setStatusTip<RetType, T: QAction_setStatusTip<RetType>>(&mut self, value: T) -> RetType {
    return value.setStatusTip(self);
    // return 1;
  }
}

pub trait QAction_setStatusTip<RetType> {
  fn setStatusTip(self, rsthis: &mut QAction) -> RetType;
}

// proto:  void QAction::setStatusTip(const QString & statusTip);
impl<'a> /*trait*/ QAction_setStatusTip<()> for (&'a  QString) {
  fn setStatusTip(self, rsthis: &mut QAction) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QAction12setStatusTipERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QAction12setStatusTipERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QAction {
  pub fn hovered<RetType, T: QAction_hovered<RetType>>(&mut self, value: T) -> RetType {
    return value.hovered(self);
    // return 1;
  }
}

pub trait QAction_hovered<RetType> {
  fn hovered(self, rsthis: &mut QAction) -> RetType;
}

// proto:  void QAction::hovered();
impl<'a> /*trait*/ QAction_hovered<()> for () {
  fn hovered(self, rsthis: &mut QAction) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QAction7hoveredEv()};
     unsafe {_ZN7QAction7hoveredEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QAction {
  pub fn data<RetType, T: QAction_data<RetType>>(&mut self, value: T) -> RetType {
    return value.data(self);
    // return 1;
  }
}

pub trait QAction_data<RetType> {
  fn data(self, rsthis: &mut QAction) -> RetType;
}

// proto:  QVariant QAction::data();
impl<'a> /*trait*/ QAction_data<QVariant> for () {
  fn data(self, rsthis: &mut QAction) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QAction4dataEv()};
    let mut ret = unsafe {_ZNK7QAction4dataEv(rsthis.qclsinst)};
    let mut ret1 = QVariant{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QAction::NewQAction(const QAction & );
impl<'a> /*trait*/ QAction_NewQAction for (&'a  QAction) {
  fn NewQAction(self) -> QAction {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QActionC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QActionC1ERKS_(qthis, arg0)};
    let rsthis = QAction{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QAction {
  pub fn isChecked<RetType, T: QAction_isChecked<RetType>>(&mut self, value: T) -> RetType {
    return value.isChecked(self);
    // return 1;
  }
}

pub trait QAction_isChecked<RetType> {
  fn isChecked(self, rsthis: &mut QAction) -> RetType;
}

// proto:  bool QAction::isChecked();
impl<'a> /*trait*/ QAction_isChecked<i8> for () {
  fn isChecked(self, rsthis: &mut QAction) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QAction9isCheckedEv()};
    let mut ret = unsafe {_ZNK7QAction9isCheckedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QAction {
  pub fn autoRepeat<RetType, T: QAction_autoRepeat<RetType>>(&mut self, value: T) -> RetType {
    return value.autoRepeat(self);
    // return 1;
  }
}

pub trait QAction_autoRepeat<RetType> {
  fn autoRepeat(self, rsthis: &mut QAction) -> RetType;
}

// proto:  bool QAction::autoRepeat();
impl<'a> /*trait*/ QAction_autoRepeat<i8> for () {
  fn autoRepeat(self, rsthis: &mut QAction) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QAction10autoRepeatEv()};
    let mut ret = unsafe {_ZNK7QAction10autoRepeatEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QAction {
  pub fn isEnabled<RetType, T: QAction_isEnabled<RetType>>(&mut self, value: T) -> RetType {
    return value.isEnabled(self);
    // return 1;
  }
}

pub trait QAction_isEnabled<RetType> {
  fn isEnabled(self, rsthis: &mut QAction) -> RetType;
}

// proto:  bool QAction::isEnabled();
impl<'a> /*trait*/ QAction_isEnabled<i8> for () {
  fn isEnabled(self, rsthis: &mut QAction) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QAction9isEnabledEv()};
    let mut ret = unsafe {_ZNK7QAction9isEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QAction {
  pub fn text<RetType, T: QAction_text<RetType>>(&mut self, value: T) -> RetType {
    return value.text(self);
    // return 1;
  }
}

pub trait QAction_text<RetType> {
  fn text(self, rsthis: &mut QAction) -> RetType;
}

// proto:  QString QAction::text();
impl<'a> /*trait*/ QAction_text<QString> for () {
  fn text(self, rsthis: &mut QAction) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QAction4textEv()};
    let mut ret = unsafe {_ZNK7QAction4textEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QAction {
  pub fn hover<RetType, T: QAction_hover<RetType>>(&mut self, value: T) -> RetType {
    return value.hover(self);
    // return 1;
  }
}

pub trait QAction_hover<RetType> {
  fn hover(self, rsthis: &mut QAction) -> RetType;
}

// proto:  void QAction::hover();
impl<'a> /*trait*/ QAction_hover<()> for () {
  fn hover(self, rsthis: &mut QAction) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QAction5hoverEv()};
     unsafe {_ZN7QAction5hoverEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QAction {
  pub fn isIconVisibleInMenu<RetType, T: QAction_isIconVisibleInMenu<RetType>>(&mut self, value: T) -> RetType {
    return value.isIconVisibleInMenu(self);
    // return 1;
  }
}

pub trait QAction_isIconVisibleInMenu<RetType> {
  fn isIconVisibleInMenu(self, rsthis: &mut QAction) -> RetType;
}

// proto:  bool QAction::isIconVisibleInMenu();
impl<'a> /*trait*/ QAction_isIconVisibleInMenu<i8> for () {
  fn isIconVisibleInMenu(self, rsthis: &mut QAction) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QAction19isIconVisibleInMenuEv()};
    let mut ret = unsafe {_ZNK7QAction19isIconVisibleInMenuEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QAction {
  pub fn setToolTip<RetType, T: QAction_setToolTip<RetType>>(&mut self, value: T) -> RetType {
    return value.setToolTip(self);
    // return 1;
  }
}

pub trait QAction_setToolTip<RetType> {
  fn setToolTip(self, rsthis: &mut QAction) -> RetType;
}

// proto:  void QAction::setToolTip(const QString & tip);
impl<'a> /*trait*/ QAction_setToolTip<()> for (&'a  QString) {
  fn setToolTip(self, rsthis: &mut QAction) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QAction10setToolTipERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QAction10setToolTipERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QAction {
  pub fn setEnabled<RetType, T: QAction_setEnabled<RetType>>(&mut self, value: T) -> RetType {
    return value.setEnabled(self);
    // return 1;
  }
}

pub trait QAction_setEnabled<RetType> {
  fn setEnabled(self, rsthis: &mut QAction) -> RetType;
}

// proto:  void QAction::setEnabled(bool );
impl<'a> /*trait*/ QAction_setEnabled<()> for (i8) {
  fn setEnabled(self, rsthis: &mut QAction) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QAction10setEnabledEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN7QAction10setEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

