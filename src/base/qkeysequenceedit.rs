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

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto: void QKeySequenceEdit::NewQKeySequenceEdit(const QKeySequenceEdit & );
  fn _ZN16QKeySequenceEditC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QKeySequenceEdit::NewQKeySequenceEdit(const QKeySequence & keySequence, QWidget * parent);
  fn _ZN16QKeySequenceEditC1ERK12QKeySequenceP7QWidget(qthis: *mut c_void, arg0: *const c_void, arg1: *mut c_void) -> i32;
  // proto: void QKeySequenceEdit::clear();
  fn _ZN16QKeySequenceEdit5clearEv() -> i32;
  // proto: void QKeySequenceEdit::setKeySequence(const QKeySequence & keySequence);
  fn _ZN16QKeySequenceEdit14setKeySequenceERK12QKeySequence(arg0: *const c_void) -> i32;
  // proto: QKeySequence QKeySequenceEdit::keySequence();
  fn _ZNK16QKeySequenceEdit11keySequenceEv() -> i32;
  // proto: void QKeySequenceEdit::FreeQKeySequenceEdit();
  fn _ZN16QKeySequenceEditD0Ev() -> i32;
  // proto: void QKeySequenceEdit::keySequenceChanged(const QKeySequence & keySequence);
  fn _ZN16QKeySequenceEdit18keySequenceChangedERK12QKeySequence(arg0: *const c_void) -> i32;
  // proto: void QKeySequenceEdit::editingFinished();
  fn _ZN16QKeySequenceEdit15editingFinishedEv() -> i32;
  // proto: void QKeySequenceEdit::NewQKeySequenceEdit(QWidget * parent);
  fn _ZN16QKeySequenceEditC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  // proto: const QMetaObject * QKeySequenceEdit::metaObject();
  fn _ZNK16QKeySequenceEdit10metaObjectEv() -> i32;
}

// body block begin
// class sizeof(QKeySequenceEdit)=1
pub struct QKeySequenceEdit {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QKeySequenceEdit {
  pub fn NewQKeySequenceEdit<T: QKeySequenceEdit_NewQKeySequenceEdit>(value: T) -> QKeySequenceEdit {
    let rsthis = value.NewQKeySequenceEdit();
    return rsthis;
    // return 1;
  }
}

pub trait QKeySequenceEdit_NewQKeySequenceEdit {
  fn NewQKeySequenceEdit(self) -> QKeySequenceEdit;
}

// proto: void QKeySequenceEdit::NewQKeySequenceEdit(const QKeySequenceEdit & );
impl<'a> /*trait*/ QKeySequenceEdit_NewQKeySequenceEdit for (&'a  QKeySequenceEdit) {
  fn NewQKeySequenceEdit(self) -> QKeySequenceEdit {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QKeySequenceEditC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN16QKeySequenceEditC1ERKS_(qthis, arg0)};
    let rsthis = QKeySequenceEdit{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QKeySequenceEdit::NewQKeySequenceEdit(const QKeySequence & keySequence, QWidget * parent);
impl<'a> /*trait*/ QKeySequenceEdit_NewQKeySequenceEdit for (&'a  QKeySequence, &'a mut QWidget) {
  fn NewQKeySequenceEdit(self) -> QKeySequenceEdit {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QKeySequenceEditC1ERK12QKeySequenceP7QWidget()};
    let arg0 = self.0.qclsinst  as *const c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN16QKeySequenceEditC1ERK12QKeySequenceP7QWidget(qthis, arg0, arg1)};
    let rsthis = QKeySequenceEdit{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QKeySequenceEdit {
  pub fn clear<T: QKeySequenceEdit_clear>(&mut self, value: T) -> i32 {
    value.clear(self);
    return 1;
  }
}

pub trait QKeySequenceEdit_clear {
  fn clear(self, this: &mut QKeySequenceEdit) -> i32;
}

// proto: void QKeySequenceEdit::clear();
impl<'a> /*trait*/ QKeySequenceEdit_clear for () {
  fn clear(self, this: &mut QKeySequenceEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QKeySequenceEdit5clearEv()};
    unsafe {_ZN16QKeySequenceEdit5clearEv()};
    return 1;
  }
}

impl /*struct*/ QKeySequenceEdit {
  pub fn setKeySequence<T: QKeySequenceEdit_setKeySequence>(&mut self, value: T) -> i32 {
    value.setKeySequence(self);
    return 1;
  }
}

pub trait QKeySequenceEdit_setKeySequence {
  fn setKeySequence(self, this: &mut QKeySequenceEdit) -> i32;
}

// proto: void QKeySequenceEdit::setKeySequence(const QKeySequence & keySequence);
impl<'a> /*trait*/ QKeySequenceEdit_setKeySequence for (&'a  QKeySequence) {
  fn setKeySequence(self, this: &mut QKeySequenceEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QKeySequenceEdit14setKeySequenceERK12QKeySequence()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN16QKeySequenceEdit14setKeySequenceERK12QKeySequence(arg0)};
    return 1;
  }
}

impl /*struct*/ QKeySequenceEdit {
  pub fn keySequence<T: QKeySequenceEdit_keySequence>(&mut self, value: T) -> i32 {
    value.keySequence(self);
    return 1;
  }
}

pub trait QKeySequenceEdit_keySequence {
  fn keySequence(self, this: &mut QKeySequenceEdit) -> i32;
}

// proto: QKeySequence QKeySequenceEdit::keySequence();
impl<'a> /*trait*/ QKeySequenceEdit_keySequence for () {
  fn keySequence(self, this: &mut QKeySequenceEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QKeySequenceEdit11keySequenceEv()};
    unsafe {_ZNK16QKeySequenceEdit11keySequenceEv()};
    return 1;
  }
}

impl /*struct*/ QKeySequenceEdit {
  pub fn FreeQKeySequenceEdit<T: QKeySequenceEdit_FreeQKeySequenceEdit>(&mut self, value: T) -> i32 {
    value.FreeQKeySequenceEdit(self);
    return 1;
  }
}

pub trait QKeySequenceEdit_FreeQKeySequenceEdit {
  fn FreeQKeySequenceEdit(self, this: &mut QKeySequenceEdit) -> i32;
}

// proto: void QKeySequenceEdit::FreeQKeySequenceEdit();
impl<'a> /*trait*/ QKeySequenceEdit_FreeQKeySequenceEdit for () {
  fn FreeQKeySequenceEdit(self, this: &mut QKeySequenceEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QKeySequenceEditD0Ev()};
    unsafe {_ZN16QKeySequenceEditD0Ev()};
    return 1;
  }
}

impl /*struct*/ QKeySequenceEdit {
  pub fn keySequenceChanged<T: QKeySequenceEdit_keySequenceChanged>(&mut self, value: T) -> i32 {
    value.keySequenceChanged(self);
    return 1;
  }
}

pub trait QKeySequenceEdit_keySequenceChanged {
  fn keySequenceChanged(self, this: &mut QKeySequenceEdit) -> i32;
}

// proto: void QKeySequenceEdit::keySequenceChanged(const QKeySequence & keySequence);
impl<'a> /*trait*/ QKeySequenceEdit_keySequenceChanged for (&'a  QKeySequence) {
  fn keySequenceChanged(self, this: &mut QKeySequenceEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QKeySequenceEdit18keySequenceChangedERK12QKeySequence()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN16QKeySequenceEdit18keySequenceChangedERK12QKeySequence(arg0)};
    return 1;
  }
}

impl /*struct*/ QKeySequenceEdit {
  pub fn editingFinished<T: QKeySequenceEdit_editingFinished>(&mut self, value: T) -> i32 {
    value.editingFinished(self);
    return 1;
  }
}

pub trait QKeySequenceEdit_editingFinished {
  fn editingFinished(self, this: &mut QKeySequenceEdit) -> i32;
}

// proto: void QKeySequenceEdit::editingFinished();
impl<'a> /*trait*/ QKeySequenceEdit_editingFinished for () {
  fn editingFinished(self, this: &mut QKeySequenceEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QKeySequenceEdit15editingFinishedEv()};
    unsafe {_ZN16QKeySequenceEdit15editingFinishedEv()};
    return 1;
  }
}

// proto: void QKeySequenceEdit::NewQKeySequenceEdit(QWidget * parent);
impl<'a> /*trait*/ QKeySequenceEdit_NewQKeySequenceEdit for (&'a mut QWidget) {
  fn NewQKeySequenceEdit(self) -> QKeySequenceEdit {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QKeySequenceEditC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN16QKeySequenceEditC1EP7QWidget(qthis, arg0)};
    let rsthis = QKeySequenceEdit{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QKeySequenceEdit {
  pub fn metaObject<T: QKeySequenceEdit_metaObject>(&mut self, value: T) -> i32 {
    value.metaObject(self);
    return 1;
  }
}

pub trait QKeySequenceEdit_metaObject {
  fn metaObject(self, this: &mut QKeySequenceEdit) -> i32;
}

// proto: const QMetaObject * QKeySequenceEdit::metaObject();
impl<'a> /*trait*/ QKeySequenceEdit_metaObject for () {
  fn metaObject(self, this: &mut QKeySequenceEdit) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QKeySequenceEdit10metaObjectEv()};
    unsafe {_ZNK16QKeySequenceEdit10metaObjectEv()};
    return 1;
  }
}

