// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qkeysequence::QKeySequence;
use super::qwidget::QWidget;
use super::qstring::QString;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QShortcut::setKey(const QKeySequence & key);
  fn _ZN9QShortcut6setKeyERK12QKeySequence(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QShortcut::activated();
  fn _ZN9QShortcut9activatedEv(qthis: *mut c_void) ;
  // proto:  const QMetaObject * QShortcut::metaObject();
  fn _ZNK9QShortcut10metaObjectEv(qthis: *mut c_void) ;
  // proto:  QWidget * QShortcut::parentWidget();
  fn _ZNK9QShortcut12parentWidgetEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QShortcut::setAutoRepeat(bool on);
  fn _ZN9QShortcut13setAutoRepeatEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  bool QShortcut::isEnabled();
  fn _ZNK9QShortcut9isEnabledEv(qthis: *mut c_void) -> int8_t;
  // proto:  QKeySequence QShortcut::key();
  fn _ZNK9QShortcut3keyEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QShortcut::FreeQShortcut();
  fn _ZN9QShortcutD0Ev(qthis: *mut c_void) ;
  // proto:  void QShortcut::setWhatsThis(const QString & text);
  fn _ZN9QShortcut12setWhatsThisERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QShortcut::setEnabled(bool enable);
  fn _ZN9QShortcut10setEnabledEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  int QShortcut::id();
  fn _ZNK9QShortcut2idEv(qthis: *mut c_void) -> c_int;
  // proto:  QString QShortcut::whatsThis();
  fn _ZNK9QShortcut9whatsThisEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QShortcut::NewQShortcut(QWidget * parent);
  fn _ZN9QShortcutC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QShortcut::activatedAmbiguously();
  fn _ZN9QShortcut20activatedAmbiguouslyEv(qthis: *mut c_void) ;
  // proto:  bool QShortcut::autoRepeat();
  fn _ZNK9QShortcut10autoRepeatEv(qthis: *mut c_void) -> int8_t;
}

// body block begin
// class sizeof(QShortcut)=1
pub struct QShortcut {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QShortcut {
  pub fn setKey<RetType, T: QShortcut_setKey<RetType>>(&mut self, value: T) -> RetType {
    return value.setKey(self);
    // return 1;
  }
}

pub trait QShortcut_setKey<RetType> {
  fn setKey(self, rsthis: &mut QShortcut) -> RetType;
}

// proto:  void QShortcut::setKey(const QKeySequence & key);
impl<'a> /*trait*/ QShortcut_setKey<()> for (&'a  QKeySequence) {
  fn setKey(self, rsthis: &mut QShortcut) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QShortcut6setKeyERK12QKeySequence()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QShortcut6setKeyERK12QKeySequence(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QShortcut {
  pub fn activated<RetType, T: QShortcut_activated<RetType>>(&mut self, value: T) -> RetType {
    return value.activated(self);
    // return 1;
  }
}

pub trait QShortcut_activated<RetType> {
  fn activated(self, rsthis: &mut QShortcut) -> RetType;
}

// proto:  void QShortcut::activated();
impl<'a> /*trait*/ QShortcut_activated<()> for () {
  fn activated(self, rsthis: &mut QShortcut) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QShortcut9activatedEv()};
     unsafe {_ZN9QShortcut9activatedEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QShortcut {
  pub fn metaObject<RetType, T: QShortcut_metaObject<RetType>>(&mut self, value: T) -> RetType {
    return value.metaObject(self);
    // return 1;
  }
}

pub trait QShortcut_metaObject<RetType> {
  fn metaObject(self, rsthis: &mut QShortcut) -> RetType;
}

// proto:  const QMetaObject * QShortcut::metaObject();
impl<'a> /*trait*/ QShortcut_metaObject<()> for () {
  fn metaObject(self, rsthis: &mut QShortcut) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QShortcut10metaObjectEv()};
     unsafe {_ZNK9QShortcut10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QShortcut {
  pub fn parentWidget<RetType, T: QShortcut_parentWidget<RetType>>(&mut self, value: T) -> RetType {
    return value.parentWidget(self);
    // return 1;
  }
}

pub trait QShortcut_parentWidget<RetType> {
  fn parentWidget(self, rsthis: &mut QShortcut) -> RetType;
}

// proto:  QWidget * QShortcut::parentWidget();
impl<'a> /*trait*/ QShortcut_parentWidget<QWidget> for () {
  fn parentWidget(self, rsthis: &mut QShortcut) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QShortcut12parentWidgetEv()};
    let mut ret = unsafe {_ZNK9QShortcut12parentWidgetEv(rsthis.qclsinst)};
    let mut ret1 = QWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QShortcut {
  pub fn setAutoRepeat<RetType, T: QShortcut_setAutoRepeat<RetType>>(&mut self, value: T) -> RetType {
    return value.setAutoRepeat(self);
    // return 1;
  }
}

pub trait QShortcut_setAutoRepeat<RetType> {
  fn setAutoRepeat(self, rsthis: &mut QShortcut) -> RetType;
}

// proto:  void QShortcut::setAutoRepeat(bool on);
impl<'a> /*trait*/ QShortcut_setAutoRepeat<()> for (i8) {
  fn setAutoRepeat(self, rsthis: &mut QShortcut) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QShortcut13setAutoRepeatEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN9QShortcut13setAutoRepeatEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QShortcut {
  pub fn isEnabled<RetType, T: QShortcut_isEnabled<RetType>>(&mut self, value: T) -> RetType {
    return value.isEnabled(self);
    // return 1;
  }
}

pub trait QShortcut_isEnabled<RetType> {
  fn isEnabled(self, rsthis: &mut QShortcut) -> RetType;
}

// proto:  bool QShortcut::isEnabled();
impl<'a> /*trait*/ QShortcut_isEnabled<i8> for () {
  fn isEnabled(self, rsthis: &mut QShortcut) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QShortcut9isEnabledEv()};
    let mut ret = unsafe {_ZNK9QShortcut9isEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QShortcut {
  pub fn key<RetType, T: QShortcut_key<RetType>>(&mut self, value: T) -> RetType {
    return value.key(self);
    // return 1;
  }
}

pub trait QShortcut_key<RetType> {
  fn key(self, rsthis: &mut QShortcut) -> RetType;
}

// proto:  QKeySequence QShortcut::key();
impl<'a> /*trait*/ QShortcut_key<QKeySequence> for () {
  fn key(self, rsthis: &mut QShortcut) -> QKeySequence {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QShortcut3keyEv()};
    let mut ret = unsafe {_ZNK9QShortcut3keyEv(rsthis.qclsinst)};
    let mut ret1 = QKeySequence{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QShortcut {
  pub fn FreeQShortcut<RetType, T: QShortcut_FreeQShortcut<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQShortcut(self);
    // return 1;
  }
}

pub trait QShortcut_FreeQShortcut<RetType> {
  fn FreeQShortcut(self, rsthis: &mut QShortcut) -> RetType;
}

// proto:  void QShortcut::FreeQShortcut();
impl<'a> /*trait*/ QShortcut_FreeQShortcut<()> for () {
  fn FreeQShortcut(self, rsthis: &mut QShortcut) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QShortcutD0Ev()};
     unsafe {_ZN9QShortcutD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QShortcut {
  pub fn setWhatsThis<RetType, T: QShortcut_setWhatsThis<RetType>>(&mut self, value: T) -> RetType {
    return value.setWhatsThis(self);
    // return 1;
  }
}

pub trait QShortcut_setWhatsThis<RetType> {
  fn setWhatsThis(self, rsthis: &mut QShortcut) -> RetType;
}

// proto:  void QShortcut::setWhatsThis(const QString & text);
impl<'a> /*trait*/ QShortcut_setWhatsThis<()> for (&'a  QString) {
  fn setWhatsThis(self, rsthis: &mut QShortcut) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QShortcut12setWhatsThisERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QShortcut12setWhatsThisERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QShortcut {
  pub fn setEnabled<RetType, T: QShortcut_setEnabled<RetType>>(&mut self, value: T) -> RetType {
    return value.setEnabled(self);
    // return 1;
  }
}

pub trait QShortcut_setEnabled<RetType> {
  fn setEnabled(self, rsthis: &mut QShortcut) -> RetType;
}

// proto:  void QShortcut::setEnabled(bool enable);
impl<'a> /*trait*/ QShortcut_setEnabled<()> for (i8) {
  fn setEnabled(self, rsthis: &mut QShortcut) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QShortcut10setEnabledEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN9QShortcut10setEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QShortcut {
  pub fn id<RetType, T: QShortcut_id<RetType>>(&mut self, value: T) -> RetType {
    return value.id(self);
    // return 1;
  }
}

pub trait QShortcut_id<RetType> {
  fn id(self, rsthis: &mut QShortcut) -> RetType;
}

// proto:  int QShortcut::id();
impl<'a> /*trait*/ QShortcut_id<i32> for () {
  fn id(self, rsthis: &mut QShortcut) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QShortcut2idEv()};
    let mut ret = unsafe {_ZNK9QShortcut2idEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QShortcut {
  pub fn whatsThis<RetType, T: QShortcut_whatsThis<RetType>>(&mut self, value: T) -> RetType {
    return value.whatsThis(self);
    // return 1;
  }
}

pub trait QShortcut_whatsThis<RetType> {
  fn whatsThis(self, rsthis: &mut QShortcut) -> RetType;
}

// proto:  QString QShortcut::whatsThis();
impl<'a> /*trait*/ QShortcut_whatsThis<QString> for () {
  fn whatsThis(self, rsthis: &mut QShortcut) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QShortcut9whatsThisEv()};
    let mut ret = unsafe {_ZNK9QShortcut9whatsThisEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QShortcut {
  pub fn NewQShortcut<T: QShortcut_NewQShortcut>(value: T) -> QShortcut {
    let rsthis = value.NewQShortcut();
    return rsthis;
    // return 1;
  }
}

pub trait QShortcut_NewQShortcut {
  fn NewQShortcut(self) -> QShortcut;
}

// proto: void QShortcut::NewQShortcut(QWidget * parent);
impl<'a> /*trait*/ QShortcut_NewQShortcut for (&'a mut QWidget) {
  fn NewQShortcut(self) -> QShortcut {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QShortcutC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QShortcutC1EP7QWidget(qthis, arg0)};
    let rsthis = QShortcut{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QShortcut {
  pub fn activatedAmbiguously<RetType, T: QShortcut_activatedAmbiguously<RetType>>(&mut self, value: T) -> RetType {
    return value.activatedAmbiguously(self);
    // return 1;
  }
}

pub trait QShortcut_activatedAmbiguously<RetType> {
  fn activatedAmbiguously(self, rsthis: &mut QShortcut) -> RetType;
}

// proto:  void QShortcut::activatedAmbiguously();
impl<'a> /*trait*/ QShortcut_activatedAmbiguously<()> for () {
  fn activatedAmbiguously(self, rsthis: &mut QShortcut) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QShortcut20activatedAmbiguouslyEv()};
     unsafe {_ZN9QShortcut20activatedAmbiguouslyEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QShortcut {
  pub fn autoRepeat<RetType, T: QShortcut_autoRepeat<RetType>>(&mut self, value: T) -> RetType {
    return value.autoRepeat(self);
    // return 1;
  }
}

pub trait QShortcut_autoRepeat<RetType> {
  fn autoRepeat(self, rsthis: &mut QShortcut) -> RetType;
}

// proto:  bool QShortcut::autoRepeat();
impl<'a> /*trait*/ QShortcut_autoRepeat<i8> for () {
  fn autoRepeat(self, rsthis: &mut QShortcut) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QShortcut10autoRepeatEv()};
    let mut ret = unsafe {_ZNK9QShortcut10autoRepeatEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

