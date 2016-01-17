// auto generated, do not modify.
// created: Sun Jan 17 17:37:11 2016
// src-file: /QtWidgets/qstatusbar.h
// dst-file: /src/widgets/qstatusbar.rs
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
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QStatusBar_Class_Size() -> c_int;
  // proto:  void QStatusBar::~QStatusBar();
  fn _ZN10QStatusBarD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  int QStatusBar::insertPermanentWidget(int index, QWidget * widget, int stretch);
  fn _ZN10QStatusBar21insertPermanentWidgetEiP7QWidgeti(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void, arg2: c_int) -> c_int;
  // proto:  void QStatusBar::removeWidget(QWidget * widget);
  fn _ZN10QStatusBar12removeWidgetEP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QStatusBar::QStatusBar(const QStatusBar & );
  fn _ZN10QStatusBarC2ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QStatusBar::setSizeGripEnabled(bool );
  fn _ZN10QStatusBar18setSizeGripEnabledEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QStatusBar::addPermanentWidget(QWidget * widget, int stretch);
  fn _ZN10QStatusBar18addPermanentWidgetEP7QWidgeti(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int);
  // proto:  bool QStatusBar::isSizeGripEnabled();
  fn _ZNK10QStatusBar17isSizeGripEnabledEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QStatusBar::clearMessage();
  fn _ZN10QStatusBar12clearMessageEv(qthis: u64 /* *mut c_void*/);
  // proto:  QString QStatusBar::currentMessage();
  fn _ZNK10QStatusBar14currentMessageEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  const QMetaObject * QStatusBar::metaObject();
  fn _ZNK10QStatusBar10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QStatusBar::showMessage(const QString & text, int timeout);
  fn _ZN10QStatusBar11showMessageERK7QStringi(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int);
  // proto:  int QStatusBar::insertWidget(int index, QWidget * widget, int stretch);
  fn _ZN10QStatusBar12insertWidgetEiP7QWidgeti(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void, arg2: c_int) -> c_int;
  // proto:  void QStatusBar::addWidget(QWidget * widget, int stretch);
  fn _ZN10QStatusBar9addWidgetEP7QWidgeti(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int);
  // proto:  void QStatusBar::QStatusBar(QWidget * parent);
  fn _ZN10QStatusBarC2EP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  fn QStatusBar_SlotProxy_connect__ZN10QStatusBar14messageChangedERK7QString(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QStatusBar)=1
#[derive(Default)]
pub struct QStatusBar {
  qbase: QWidget,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _messageChanged: QStatusBar_messageChanged_signal,
}

impl /*struct*/ QStatusBar {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QStatusBar {
    return QStatusBar{qbase: QWidget::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QStatusBar {
  type Target = QWidget;

  fn deref(&self) -> &QWidget {
    return & self.qbase;
  }
}
impl AsRef<QWidget> for QStatusBar {
  fn as_ref(& self) -> & QWidget {
    return & self.qbase;
  }
}
  // proto:  void QStatusBar::~QStatusBar();
impl /*struct*/ QStatusBar {
  pub fn free<RetType, T: QStatusBar_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QStatusBar_free<RetType> {
  fn free(self , rsthis: & QStatusBar) -> RetType;
}

  // proto:  void QStatusBar::~QStatusBar();
impl<'a> /*trait*/ QStatusBar_free<()> for () {
  fn free(self , rsthis: & QStatusBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QStatusBarD2Ev()};
     unsafe {_ZN10QStatusBarD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QStatusBar::insertPermanentWidget(int index, QWidget * widget, int stretch);
impl /*struct*/ QStatusBar {
  pub fn insertPermanentWidget<RetType, T: QStatusBar_insertPermanentWidget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.insertPermanentWidget(self);
    // return 1;
  }
}

pub trait QStatusBar_insertPermanentWidget<RetType> {
  fn insertPermanentWidget(self , rsthis: & QStatusBar) -> RetType;
}

  // proto:  int QStatusBar::insertPermanentWidget(int index, QWidget * widget, int stretch);
impl<'a> /*trait*/ QStatusBar_insertPermanentWidget<i32> for (i32, &'a QWidget, i32) {
  fn insertPermanentWidget(self , rsthis: & QStatusBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QStatusBar21insertPermanentWidgetEiP7QWidgeti()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
    let mut ret = unsafe {_ZN10QStatusBar21insertPermanentWidgetEiP7QWidgeti(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QStatusBar::removeWidget(QWidget * widget);
impl /*struct*/ QStatusBar {
  pub fn removeWidget<RetType, T: QStatusBar_removeWidget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.removeWidget(self);
    // return 1;
  }
}

pub trait QStatusBar_removeWidget<RetType> {
  fn removeWidget(self , rsthis: & QStatusBar) -> RetType;
}

  // proto:  void QStatusBar::removeWidget(QWidget * widget);
impl<'a> /*trait*/ QStatusBar_removeWidget<()> for (&'a QWidget) {
  fn removeWidget(self , rsthis: & QStatusBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QStatusBar12removeWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN10QStatusBar12removeWidgetEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QStatusBar::QStatusBar(const QStatusBar & );
impl /*struct*/ QStatusBar {
  pub fn new<T: QStatusBar_new>(value: T) -> QStatusBar {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QStatusBar_new {
  fn new(self) -> QStatusBar;
}

  // proto:  void QStatusBar::QStatusBar(const QStatusBar & );
impl<'a> /*trait*/ QStatusBar_new for (&'a QStatusBar) {
  fn new(self) -> QStatusBar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QStatusBarC2ERKS_()};
    let ctysz: c_int = unsafe{QStatusBar_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN10QStatusBarC2ERKS_(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QStatusBar{qbase: QWidget::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStatusBar::setSizeGripEnabled(bool );
impl /*struct*/ QStatusBar {
  pub fn setSizeGripEnabled<RetType, T: QStatusBar_setSizeGripEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSizeGripEnabled(self);
    // return 1;
  }
}

pub trait QStatusBar_setSizeGripEnabled<RetType> {
  fn setSizeGripEnabled(self , rsthis: & QStatusBar) -> RetType;
}

  // proto:  void QStatusBar::setSizeGripEnabled(bool );
impl<'a> /*trait*/ QStatusBar_setSizeGripEnabled<()> for (i8) {
  fn setSizeGripEnabled(self , rsthis: & QStatusBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QStatusBar18setSizeGripEnabledEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN10QStatusBar18setSizeGripEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QStatusBar::addPermanentWidget(QWidget * widget, int stretch);
impl /*struct*/ QStatusBar {
  pub fn addPermanentWidget<RetType, T: QStatusBar_addPermanentWidget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addPermanentWidget(self);
    // return 1;
  }
}

pub trait QStatusBar_addPermanentWidget<RetType> {
  fn addPermanentWidget(self , rsthis: & QStatusBar) -> RetType;
}

  // proto:  void QStatusBar::addPermanentWidget(QWidget * widget, int stretch);
impl<'a> /*trait*/ QStatusBar_addPermanentWidget<()> for (&'a QWidget, i32) {
  fn addPermanentWidget(self , rsthis: & QStatusBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QStatusBar18addPermanentWidgetEP7QWidgeti()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN10QStatusBar18addPermanentWidgetEP7QWidgeti(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  bool QStatusBar::isSizeGripEnabled();
impl /*struct*/ QStatusBar {
  pub fn isSizeGripEnabled<RetType, T: QStatusBar_isSizeGripEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isSizeGripEnabled(self);
    // return 1;
  }
}

pub trait QStatusBar_isSizeGripEnabled<RetType> {
  fn isSizeGripEnabled(self , rsthis: & QStatusBar) -> RetType;
}

  // proto:  bool QStatusBar::isSizeGripEnabled();
impl<'a> /*trait*/ QStatusBar_isSizeGripEnabled<i8> for () {
  fn isSizeGripEnabled(self , rsthis: & QStatusBar) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QStatusBar17isSizeGripEnabledEv()};
    let mut ret = unsafe {_ZNK10QStatusBar17isSizeGripEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QStatusBar::clearMessage();
impl /*struct*/ QStatusBar {
  pub fn clearMessage<RetType, T: QStatusBar_clearMessage<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clearMessage(self);
    // return 1;
  }
}

pub trait QStatusBar_clearMessage<RetType> {
  fn clearMessage(self , rsthis: & QStatusBar) -> RetType;
}

  // proto:  void QStatusBar::clearMessage();
impl<'a> /*trait*/ QStatusBar_clearMessage<()> for () {
  fn clearMessage(self , rsthis: & QStatusBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QStatusBar12clearMessageEv()};
     unsafe {_ZN10QStatusBar12clearMessageEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QString QStatusBar::currentMessage();
impl /*struct*/ QStatusBar {
  pub fn currentMessage<RetType, T: QStatusBar_currentMessage<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.currentMessage(self);
    // return 1;
  }
}

pub trait QStatusBar_currentMessage<RetType> {
  fn currentMessage(self , rsthis: & QStatusBar) -> RetType;
}

  // proto:  QString QStatusBar::currentMessage();
impl<'a> /*trait*/ QStatusBar_currentMessage<QString> for () {
  fn currentMessage(self , rsthis: & QStatusBar) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QStatusBar14currentMessageEv()};
    let mut ret = unsafe {_ZNK10QStatusBar14currentMessageEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QMetaObject * QStatusBar::metaObject();
impl /*struct*/ QStatusBar {
  pub fn metaObject<RetType, T: QStatusBar_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QStatusBar_metaObject<RetType> {
  fn metaObject(self , rsthis: & QStatusBar) -> RetType;
}

  // proto:  const QMetaObject * QStatusBar::metaObject();
impl<'a> /*trait*/ QStatusBar_metaObject<()> for () {
  fn metaObject(self , rsthis: & QStatusBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QStatusBar10metaObjectEv()};
     unsafe {_ZNK10QStatusBar10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QStatusBar::showMessage(const QString & text, int timeout);
impl /*struct*/ QStatusBar {
  pub fn showMessage<RetType, T: QStatusBar_showMessage<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.showMessage(self);
    // return 1;
  }
}

pub trait QStatusBar_showMessage<RetType> {
  fn showMessage(self , rsthis: & QStatusBar) -> RetType;
}

  // proto:  void QStatusBar::showMessage(const QString & text, int timeout);
impl<'a> /*trait*/ QStatusBar_showMessage<()> for (&'a QString, i32) {
  fn showMessage(self , rsthis: & QStatusBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QStatusBar11showMessageERK7QStringi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN10QStatusBar11showMessageERK7QStringi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  int QStatusBar::insertWidget(int index, QWidget * widget, int stretch);
impl /*struct*/ QStatusBar {
  pub fn insertWidget<RetType, T: QStatusBar_insertWidget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.insertWidget(self);
    // return 1;
  }
}

pub trait QStatusBar_insertWidget<RetType> {
  fn insertWidget(self , rsthis: & QStatusBar) -> RetType;
}

  // proto:  int QStatusBar::insertWidget(int index, QWidget * widget, int stretch);
impl<'a> /*trait*/ QStatusBar_insertWidget<i32> for (i32, &'a QWidget, i32) {
  fn insertWidget(self , rsthis: & QStatusBar) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QStatusBar12insertWidgetEiP7QWidgeti()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
    let mut ret = unsafe {_ZN10QStatusBar12insertWidgetEiP7QWidgeti(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QStatusBar::addWidget(QWidget * widget, int stretch);
impl /*struct*/ QStatusBar {
  pub fn addWidget<RetType, T: QStatusBar_addWidget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addWidget(self);
    // return 1;
  }
}

pub trait QStatusBar_addWidget<RetType> {
  fn addWidget(self , rsthis: & QStatusBar) -> RetType;
}

  // proto:  void QStatusBar::addWidget(QWidget * widget, int stretch);
impl<'a> /*trait*/ QStatusBar_addWidget<()> for (&'a QWidget, i32) {
  fn addWidget(self , rsthis: & QStatusBar) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QStatusBar9addWidgetEP7QWidgeti()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN10QStatusBar9addWidgetEP7QWidgeti(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QStatusBar::QStatusBar(QWidget * parent);
impl<'a> /*trait*/ QStatusBar_new for (&'a QWidget) {
  fn new(self) -> QStatusBar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QStatusBarC2EP7QWidget()};
    let ctysz: c_int = unsafe{QStatusBar_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN10QStatusBarC2EP7QWidget(qthis_ph, arg0)};
    let qthis: u64 = qthis_ph;
    let rsthis = QStatusBar{qbase: QWidget::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

#[derive(Default)] // for QStatusBar_messageChanged
pub struct QStatusBar_messageChanged_signal{poi:u64}
impl /* struct */ QStatusBar {
  pub fn messageChanged(&self) -> QStatusBar_messageChanged_signal {
     return QStatusBar_messageChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QStatusBar_messageChanged_signal {
  pub fn connect<T: QStatusBar_messageChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QStatusBar_messageChanged_signal_connect {
  fn connect(self, sigthis: QStatusBar_messageChanged_signal);
}

// messageChanged(const class QString &)
extern fn QStatusBar_messageChanged_signal_connect_cb_0(rsfptr:fn(QString), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QString::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QStatusBar_messageChanged_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn(QString)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QString::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QStatusBar_messageChanged_signal_connect for fn(QString) {
  fn connect(self, sigthis: QStatusBar_messageChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QStatusBar_messageChanged_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QStatusBar_SlotProxy_connect__ZN10QStatusBar14messageChangedERK7QString(arg0, arg1, arg2)};
  }
}
impl /* trait */ QStatusBar_messageChanged_signal_connect for Box<Fn(QString)> {
  fn connect(self, sigthis: QStatusBar_messageChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QStatusBar_messageChanged_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QStatusBar_SlotProxy_connect__ZN10QStatusBar14messageChangedERK7QString(arg0, arg1, arg2)};
  }
}
// <= body block end

