// auto generated, do not modify.
// created: Sun Jan 24 17:41:38 2016
// src-file: /QtWidgets/qundoview.h
// dst-file: /src/widgets/qundoview.rs
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
use super::qlistview::QListView; // 773
use std::ops::Deref;
use super::qundogroup::QUndoGroup; // 773
use super::qwidget::QWidget; // 773
use super::qundostack::QUndoStack; // 773
use super::super::core::qstring::QString; // 771
use super::super::gui::qicon::QIcon; // 771
use super::super::core::qobjectdefs::QMetaObject; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QUndoView_Class_Size() -> c_int;
  // proto:  void QUndoView::QUndoView(QUndoGroup * group, QWidget * parent);
  fn C_ZN9QUndoViewC2EP10QUndoGroupP7QWidget(arg0: *mut c_void, arg1: *mut c_void) -> u64;
  // proto:  void QUndoView::setStack(QUndoStack * stack);
  fn C_ZN9QUndoView8setStackEP10QUndoStack(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QUndoView::setEmptyLabel(const QString & label);
  fn C_ZN9QUndoView13setEmptyLabelERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QUndoView::setCleanIcon(const QIcon & icon);
  fn C_ZN9QUndoView12setCleanIconERK5QIcon(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QUndoView::setGroup(QUndoGroup * group);
  fn C_ZN9QUndoView8setGroupEP10QUndoGroup(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QUndoGroup * QUndoView::group();
  fn C_ZNK9QUndoView5groupEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  const QMetaObject * QUndoView::metaObject();
  fn C_ZNK9QUndoView10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QUndoStack * QUndoView::stack();
  fn C_ZNK9QUndoView5stackEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QIcon QUndoView::cleanIcon();
  fn C_ZNK9QUndoView9cleanIconEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QString QUndoView::emptyLabel();
  fn C_ZNK9QUndoView10emptyLabelEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QUndoView::QUndoView(QWidget * parent);
  fn C_ZN9QUndoViewC2EP7QWidget(arg0: *mut c_void) -> u64;
  // proto:  void QUndoView::~QUndoView();
  fn C_ZN9QUndoViewD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QUndoView::QUndoView(QUndoStack * stack, QWidget * parent);
  fn C_ZN9QUndoViewC2EP10QUndoStackP7QWidget(arg0: *mut c_void, arg1: *mut c_void) -> u64;
} // <= ext block end

// body block begin =>
// class sizeof(QUndoView)=1
#[derive(Default)]
pub struct QUndoView {
  qbase: QListView,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QUndoView {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QUndoView {
    return QUndoView{qbase: QListView::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QUndoView {
  type Target = QListView;

  fn deref(&self) -> &QListView {
    return & self.qbase;
  }
}
impl AsRef<QListView> for QUndoView {
  fn as_ref(& self) -> & QListView {
    return & self.qbase;
  }
}
  // proto:  void QUndoView::QUndoView(QUndoGroup * group, QWidget * parent);
impl /*struct*/ QUndoView {
  pub fn new<T: QUndoView_new>(value: T) -> QUndoView {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QUndoView_new {
  fn new(self) -> QUndoView;
}

  // proto:  void QUndoView::QUndoView(QUndoGroup * group, QWidget * parent);
impl<'a> /*trait*/ QUndoView_new for (&'a QUndoGroup, &'a QWidget) {
  fn new(self) -> QUndoView {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QUndoViewC2EP10QUndoGroupP7QWidget()};
    let ctysz: c_int = unsafe{QUndoView_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN9QUndoViewC2EP10QUndoGroupP7QWidget(arg0, arg1)};
    let rsthis = QUndoView{qbase: QListView::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QUndoView::setStack(QUndoStack * stack);
impl /*struct*/ QUndoView {
  pub fn setStack<RetType, T: QUndoView_setStack<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setStack(self);
    // return 1;
  }
}

pub trait QUndoView_setStack<RetType> {
  fn setStack(self , rsthis: & QUndoView) -> RetType;
}

  // proto:  void QUndoView::setStack(QUndoStack * stack);
impl<'a> /*trait*/ QUndoView_setStack<()> for (&'a QUndoStack) {
  fn setStack(self , rsthis: & QUndoView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QUndoView8setStackEP10QUndoStack()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN9QUndoView8setStackEP10QUndoStack(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QUndoView::setEmptyLabel(const QString & label);
impl /*struct*/ QUndoView {
  pub fn setEmptyLabel<RetType, T: QUndoView_setEmptyLabel<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setEmptyLabel(self);
    // return 1;
  }
}

pub trait QUndoView_setEmptyLabel<RetType> {
  fn setEmptyLabel(self , rsthis: & QUndoView) -> RetType;
}

  // proto:  void QUndoView::setEmptyLabel(const QString & label);
impl<'a> /*trait*/ QUndoView_setEmptyLabel<()> for (&'a QString) {
  fn setEmptyLabel(self , rsthis: & QUndoView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QUndoView13setEmptyLabelERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN9QUndoView13setEmptyLabelERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QUndoView::setCleanIcon(const QIcon & icon);
impl /*struct*/ QUndoView {
  pub fn setCleanIcon<RetType, T: QUndoView_setCleanIcon<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCleanIcon(self);
    // return 1;
  }
}

pub trait QUndoView_setCleanIcon<RetType> {
  fn setCleanIcon(self , rsthis: & QUndoView) -> RetType;
}

  // proto:  void QUndoView::setCleanIcon(const QIcon & icon);
impl<'a> /*trait*/ QUndoView_setCleanIcon<()> for (&'a QIcon) {
  fn setCleanIcon(self , rsthis: & QUndoView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QUndoView12setCleanIconERK5QIcon()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN9QUndoView12setCleanIconERK5QIcon(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QUndoView::setGroup(QUndoGroup * group);
impl /*struct*/ QUndoView {
  pub fn setGroup<RetType, T: QUndoView_setGroup<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setGroup(self);
    // return 1;
  }
}

pub trait QUndoView_setGroup<RetType> {
  fn setGroup(self , rsthis: & QUndoView) -> RetType;
}

  // proto:  void QUndoView::setGroup(QUndoGroup * group);
impl<'a> /*trait*/ QUndoView_setGroup<()> for (&'a QUndoGroup) {
  fn setGroup(self , rsthis: & QUndoView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QUndoView8setGroupEP10QUndoGroup()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN9QUndoView8setGroupEP10QUndoGroup(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QUndoGroup * QUndoView::group();
impl /*struct*/ QUndoView {
  pub fn group<RetType, T: QUndoView_group<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.group(self);
    // return 1;
  }
}

pub trait QUndoView_group<RetType> {
  fn group(self , rsthis: & QUndoView) -> RetType;
}

  // proto:  QUndoGroup * QUndoView::group();
impl<'a> /*trait*/ QUndoView_group<QUndoGroup> for () {
  fn group(self , rsthis: & QUndoView) -> QUndoGroup {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QUndoView5groupEv()};
    let mut ret = unsafe {C_ZNK9QUndoView5groupEv(rsthis.qclsinst)};
    let mut ret1 = QUndoGroup::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QMetaObject * QUndoView::metaObject();
impl /*struct*/ QUndoView {
  pub fn metaObject<RetType, T: QUndoView_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QUndoView_metaObject<RetType> {
  fn metaObject(self , rsthis: & QUndoView) -> RetType;
}

  // proto:  const QMetaObject * QUndoView::metaObject();
impl<'a> /*trait*/ QUndoView_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QUndoView) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QUndoView10metaObjectEv()};
    let mut ret = unsafe {C_ZNK9QUndoView10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QUndoStack * QUndoView::stack();
impl /*struct*/ QUndoView {
  pub fn stack<RetType, T: QUndoView_stack<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.stack(self);
    // return 1;
  }
}

pub trait QUndoView_stack<RetType> {
  fn stack(self , rsthis: & QUndoView) -> RetType;
}

  // proto:  QUndoStack * QUndoView::stack();
impl<'a> /*trait*/ QUndoView_stack<QUndoStack> for () {
  fn stack(self , rsthis: & QUndoView) -> QUndoStack {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QUndoView5stackEv()};
    let mut ret = unsafe {C_ZNK9QUndoView5stackEv(rsthis.qclsinst)};
    let mut ret1 = QUndoStack::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QIcon QUndoView::cleanIcon();
impl /*struct*/ QUndoView {
  pub fn cleanIcon<RetType, T: QUndoView_cleanIcon<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.cleanIcon(self);
    // return 1;
  }
}

pub trait QUndoView_cleanIcon<RetType> {
  fn cleanIcon(self , rsthis: & QUndoView) -> RetType;
}

  // proto:  QIcon QUndoView::cleanIcon();
impl<'a> /*trait*/ QUndoView_cleanIcon<QIcon> for () {
  fn cleanIcon(self , rsthis: & QUndoView) -> QIcon {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QUndoView9cleanIconEv()};
    let mut ret = unsafe {C_ZNK9QUndoView9cleanIconEv(rsthis.qclsinst)};
    let mut ret1 = QIcon::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QUndoView::emptyLabel();
impl /*struct*/ QUndoView {
  pub fn emptyLabel<RetType, T: QUndoView_emptyLabel<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.emptyLabel(self);
    // return 1;
  }
}

pub trait QUndoView_emptyLabel<RetType> {
  fn emptyLabel(self , rsthis: & QUndoView) -> RetType;
}

  // proto:  QString QUndoView::emptyLabel();
impl<'a> /*trait*/ QUndoView_emptyLabel<QString> for () {
  fn emptyLabel(self , rsthis: & QUndoView) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QUndoView10emptyLabelEv()};
    let mut ret = unsafe {C_ZNK9QUndoView10emptyLabelEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QUndoView::QUndoView(QWidget * parent);
impl<'a> /*trait*/ QUndoView_new for (&'a QWidget) {
  fn new(self) -> QUndoView {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QUndoViewC2EP7QWidget()};
    let ctysz: c_int = unsafe{QUndoView_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN9QUndoViewC2EP7QWidget(arg0)};
    let rsthis = QUndoView{qbase: QListView::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QUndoView::~QUndoView();
impl /*struct*/ QUndoView {
  pub fn free<RetType, T: QUndoView_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QUndoView_free<RetType> {
  fn free(self , rsthis: & QUndoView) -> RetType;
}

  // proto:  void QUndoView::~QUndoView();
impl<'a> /*trait*/ QUndoView_free<()> for () {
  fn free(self , rsthis: & QUndoView) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QUndoViewD2Ev()};
     unsafe {C_ZN9QUndoViewD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QUndoView::QUndoView(QUndoStack * stack, QWidget * parent);
impl<'a> /*trait*/ QUndoView_new for (&'a QUndoStack, &'a QWidget) {
  fn new(self) -> QUndoView {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QUndoViewC2EP10QUndoStackP7QWidget()};
    let ctysz: c_int = unsafe{QUndoView_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN9QUndoViewC2EP10QUndoStackP7QWidget(arg0, arg1)};
    let rsthis = QUndoView{qbase: QListView::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// <= body block end

