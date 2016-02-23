// auto generated, do not modify.
// created: Mon Feb 22 23:57:02 2016
// src-file: /QtWidgets/qsystemtrayicon.h
// dst-file: /src/widgets/qsystemtrayicon.rs
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
use super::super::core::qobject::*; // 771
use std::ops::Deref;
use super::super::core::qstring::*; // 771
use super::super::gui::qicon::*; // 771
use super::super::core::qobjectdefs::*; // 771
use super::qmenu::*; // 773
use super::super::core::qrect::*; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QSystemTrayIcon_Class_Size() -> c_int;
  // proto:  void QSystemTrayIcon::~QSystemTrayIcon();
  fn C_ZN15QSystemTrayIconD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QSystemTrayIcon::setVisible(bool visible);
  fn C_ZN15QSystemTrayIcon10setVisibleEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  QString QSystemTrayIcon::toolTip();
  fn C_ZNK15QSystemTrayIcon7toolTipEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QSystemTrayIcon::QSystemTrayIcon(const QIcon & icon, QObject * parent);
  fn C_ZN15QSystemTrayIconC2ERK5QIconP7QObject(arg0: *mut c_void, arg1: *mut c_void) -> u64;
  // proto:  void QSystemTrayIcon::hide();
  fn C_ZN15QSystemTrayIcon4hideEv(qthis: u64 /* *mut c_void*/);
  // proto:  const QMetaObject * QSystemTrayIcon::metaObject();
  fn C_ZNK15QSystemTrayIcon10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QSystemTrayIcon::setIcon(const QIcon & icon);
  fn C_ZN15QSystemTrayIcon7setIconERK5QIcon(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QSystemTrayIcon::isVisible();
  fn C_ZNK15QSystemTrayIcon9isVisibleEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QSystemTrayIcon::QSystemTrayIcon(QObject * parent);
  fn C_ZN15QSystemTrayIconC2EP7QObject(arg0: *mut c_void) -> u64;
  // proto:  void QSystemTrayIcon::show();
  fn C_ZN15QSystemTrayIcon4showEv(qthis: u64 /* *mut c_void*/);
  // proto: static bool QSystemTrayIcon::supportsMessages();
  fn C_ZN15QSystemTrayIcon16supportsMessagesEv() -> c_char;
  // proto:  void QSystemTrayIcon::setContextMenu(QMenu * menu);
  fn C_ZN15QSystemTrayIcon14setContextMenuEP5QMenu(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QRect QSystemTrayIcon::geometry();
  fn C_ZNK15QSystemTrayIcon8geometryEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QSystemTrayIcon::setToolTip(const QString & tip);
  fn C_ZN15QSystemTrayIcon10setToolTipERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QIcon QSystemTrayIcon::icon();
  fn C_ZNK15QSystemTrayIcon4iconEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QMenu * QSystemTrayIcon::contextMenu();
  fn C_ZNK15QSystemTrayIcon11contextMenuEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto: static bool QSystemTrayIcon::isSystemTrayAvailable();
  fn C_ZN15QSystemTrayIcon21isSystemTrayAvailableEv() -> c_char;
  fn QSystemTrayIcon_SlotProxy_connect__ZN15QSystemTrayIcon14messageClickedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QSystemTrayIcon)=1
#[derive(Default)]
pub struct QSystemTrayIcon {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _activated: QSystemTrayIcon_activated_signal,
  pub _messageClicked: QSystemTrayIcon_messageClicked_signal,
}

impl /*struct*/ QSystemTrayIcon {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QSystemTrayIcon {
    return QSystemTrayIcon{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QSystemTrayIcon {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QSystemTrayIcon {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  void QSystemTrayIcon::~QSystemTrayIcon();
impl /*struct*/ QSystemTrayIcon {
  pub fn free<RetType, T: QSystemTrayIcon_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QSystemTrayIcon_free<RetType> {
  fn free(self , rsthis: & QSystemTrayIcon) -> RetType;
}

  // proto:  void QSystemTrayIcon::~QSystemTrayIcon();
impl<'a> /*trait*/ QSystemTrayIcon_free<()> for () {
  fn free(self , rsthis: & QSystemTrayIcon) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSystemTrayIconD2Ev()};
     unsafe {C_ZN15QSystemTrayIconD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QSystemTrayIcon::setVisible(bool visible);
impl /*struct*/ QSystemTrayIcon {
  pub fn setVisible<RetType, T: QSystemTrayIcon_setVisible<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setVisible(self);
    // return 1;
  }
}

pub trait QSystemTrayIcon_setVisible<RetType> {
  fn setVisible(self , rsthis: & QSystemTrayIcon) -> RetType;
}

  // proto:  void QSystemTrayIcon::setVisible(bool visible);
impl<'a> /*trait*/ QSystemTrayIcon_setVisible<()> for (i8) {
  fn setVisible(self , rsthis: & QSystemTrayIcon) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSystemTrayIcon10setVisibleEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN15QSystemTrayIcon10setVisibleEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QSystemTrayIcon::toolTip();
impl /*struct*/ QSystemTrayIcon {
  pub fn toolTip<RetType, T: QSystemTrayIcon_toolTip<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toolTip(self);
    // return 1;
  }
}

pub trait QSystemTrayIcon_toolTip<RetType> {
  fn toolTip(self , rsthis: & QSystemTrayIcon) -> RetType;
}

  // proto:  QString QSystemTrayIcon::toolTip();
impl<'a> /*trait*/ QSystemTrayIcon_toolTip<QString> for () {
  fn toolTip(self , rsthis: & QSystemTrayIcon) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSystemTrayIcon7toolTipEv()};
    let mut ret = unsafe {C_ZNK15QSystemTrayIcon7toolTipEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QSystemTrayIcon::QSystemTrayIcon(const QIcon & icon, QObject * parent);
impl /*struct*/ QSystemTrayIcon {
  pub fn new<T: QSystemTrayIcon_new>(value: T) -> QSystemTrayIcon {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QSystemTrayIcon_new {
  fn new(self) -> QSystemTrayIcon;
}

  // proto:  void QSystemTrayIcon::QSystemTrayIcon(const QIcon & icon, QObject * parent);
impl<'a> /*trait*/ QSystemTrayIcon_new for (&'a QIcon, Option<&'a QObject>) {
  fn new(self) -> QSystemTrayIcon {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSystemTrayIconC2ERK5QIconP7QObject()};
    let ctysz: c_int = unsafe{QSystemTrayIcon_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = (if self.1.is_none() {0} else {self.1.unwrap().qclsinst})  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN15QSystemTrayIconC2ERK5QIconP7QObject(arg0, arg1)};
    let rsthis = QSystemTrayIcon{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QSystemTrayIcon::hide();
impl /*struct*/ QSystemTrayIcon {
  pub fn hide<RetType, T: QSystemTrayIcon_hide<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hide(self);
    // return 1;
  }
}

pub trait QSystemTrayIcon_hide<RetType> {
  fn hide(self , rsthis: & QSystemTrayIcon) -> RetType;
}

  // proto:  void QSystemTrayIcon::hide();
impl<'a> /*trait*/ QSystemTrayIcon_hide<()> for () {
  fn hide(self , rsthis: & QSystemTrayIcon) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSystemTrayIcon4hideEv()};
     unsafe {C_ZN15QSystemTrayIcon4hideEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QSystemTrayIcon::metaObject();
impl /*struct*/ QSystemTrayIcon {
  pub fn metaObject<RetType, T: QSystemTrayIcon_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QSystemTrayIcon_metaObject<RetType> {
  fn metaObject(self , rsthis: & QSystemTrayIcon) -> RetType;
}

  // proto:  const QMetaObject * QSystemTrayIcon::metaObject();
impl<'a> /*trait*/ QSystemTrayIcon_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QSystemTrayIcon) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSystemTrayIcon10metaObjectEv()};
    let mut ret = unsafe {C_ZNK15QSystemTrayIcon10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QSystemTrayIcon::setIcon(const QIcon & icon);
impl /*struct*/ QSystemTrayIcon {
  pub fn setIcon<RetType, T: QSystemTrayIcon_setIcon<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setIcon(self);
    // return 1;
  }
}

pub trait QSystemTrayIcon_setIcon<RetType> {
  fn setIcon(self , rsthis: & QSystemTrayIcon) -> RetType;
}

  // proto:  void QSystemTrayIcon::setIcon(const QIcon & icon);
impl<'a> /*trait*/ QSystemTrayIcon_setIcon<()> for (&'a QIcon) {
  fn setIcon(self , rsthis: & QSystemTrayIcon) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSystemTrayIcon7setIconERK5QIcon()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN15QSystemTrayIcon7setIconERK5QIcon(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QSystemTrayIcon::isVisible();
impl /*struct*/ QSystemTrayIcon {
  pub fn isVisible<RetType, T: QSystemTrayIcon_isVisible<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isVisible(self);
    // return 1;
  }
}

pub trait QSystemTrayIcon_isVisible<RetType> {
  fn isVisible(self , rsthis: & QSystemTrayIcon) -> RetType;
}

  // proto:  bool QSystemTrayIcon::isVisible();
impl<'a> /*trait*/ QSystemTrayIcon_isVisible<i8> for () {
  fn isVisible(self , rsthis: & QSystemTrayIcon) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSystemTrayIcon9isVisibleEv()};
    let mut ret = unsafe {C_ZNK15QSystemTrayIcon9isVisibleEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QSystemTrayIcon::QSystemTrayIcon(QObject * parent);
impl<'a> /*trait*/ QSystemTrayIcon_new for (Option<&'a QObject>) {
  fn new(self) -> QSystemTrayIcon {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSystemTrayIconC2EP7QObject()};
    let ctysz: c_int = unsafe{QSystemTrayIcon_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = (if self.is_none() {0} else {self.unwrap().qclsinst})  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN15QSystemTrayIconC2EP7QObject(arg0)};
    let rsthis = QSystemTrayIcon{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QSystemTrayIcon::show();
impl /*struct*/ QSystemTrayIcon {
  pub fn show<RetType, T: QSystemTrayIcon_show<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.show(self);
    // return 1;
  }
}

pub trait QSystemTrayIcon_show<RetType> {
  fn show(self , rsthis: & QSystemTrayIcon) -> RetType;
}

  // proto:  void QSystemTrayIcon::show();
impl<'a> /*trait*/ QSystemTrayIcon_show<()> for () {
  fn show(self , rsthis: & QSystemTrayIcon) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSystemTrayIcon4showEv()};
     unsafe {C_ZN15QSystemTrayIcon4showEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto: static bool QSystemTrayIcon::supportsMessages();
impl /*struct*/ QSystemTrayIcon {
  pub fn supportsMessages_s<RetType, T: QSystemTrayIcon_supportsMessages_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.supportsMessages_s();
    // return 1;
  }
}

pub trait QSystemTrayIcon_supportsMessages_s<RetType> {
  fn supportsMessages_s(self ) -> RetType;
}

  // proto: static bool QSystemTrayIcon::supportsMessages();
impl<'a> /*trait*/ QSystemTrayIcon_supportsMessages_s<i8> for () {
  fn supportsMessages_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSystemTrayIcon16supportsMessagesEv()};
    let mut ret = unsafe {C_ZN15QSystemTrayIcon16supportsMessagesEv()};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QSystemTrayIcon::setContextMenu(QMenu * menu);
impl /*struct*/ QSystemTrayIcon {
  pub fn setContextMenu<RetType, T: QSystemTrayIcon_setContextMenu<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setContextMenu(self);
    // return 1;
  }
}

pub trait QSystemTrayIcon_setContextMenu<RetType> {
  fn setContextMenu(self , rsthis: & QSystemTrayIcon) -> RetType;
}

  // proto:  void QSystemTrayIcon::setContextMenu(QMenu * menu);
impl<'a> /*trait*/ QSystemTrayIcon_setContextMenu<()> for (&'a QMenu) {
  fn setContextMenu(self , rsthis: & QSystemTrayIcon) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSystemTrayIcon14setContextMenuEP5QMenu()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN15QSystemTrayIcon14setContextMenuEP5QMenu(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QRect QSystemTrayIcon::geometry();
impl /*struct*/ QSystemTrayIcon {
  pub fn geometry<RetType, T: QSystemTrayIcon_geometry<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.geometry(self);
    // return 1;
  }
}

pub trait QSystemTrayIcon_geometry<RetType> {
  fn geometry(self , rsthis: & QSystemTrayIcon) -> RetType;
}

  // proto:  QRect QSystemTrayIcon::geometry();
impl<'a> /*trait*/ QSystemTrayIcon_geometry<QRect> for () {
  fn geometry(self , rsthis: & QSystemTrayIcon) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSystemTrayIcon8geometryEv()};
    let mut ret = unsafe {C_ZNK15QSystemTrayIcon8geometryEv(rsthis.qclsinst)};
    let mut ret1 = QRect::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QSystemTrayIcon::setToolTip(const QString & tip);
impl /*struct*/ QSystemTrayIcon {
  pub fn setToolTip<RetType, T: QSystemTrayIcon_setToolTip<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setToolTip(self);
    // return 1;
  }
}

pub trait QSystemTrayIcon_setToolTip<RetType> {
  fn setToolTip(self , rsthis: & QSystemTrayIcon) -> RetType;
}

  // proto:  void QSystemTrayIcon::setToolTip(const QString & tip);
impl<'a> /*trait*/ QSystemTrayIcon_setToolTip<()> for (&'a QString) {
  fn setToolTip(self , rsthis: & QSystemTrayIcon) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSystemTrayIcon10setToolTipERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN15QSystemTrayIcon10setToolTipERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QIcon QSystemTrayIcon::icon();
impl /*struct*/ QSystemTrayIcon {
  pub fn icon<RetType, T: QSystemTrayIcon_icon<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.icon(self);
    // return 1;
  }
}

pub trait QSystemTrayIcon_icon<RetType> {
  fn icon(self , rsthis: & QSystemTrayIcon) -> RetType;
}

  // proto:  QIcon QSystemTrayIcon::icon();
impl<'a> /*trait*/ QSystemTrayIcon_icon<QIcon> for () {
  fn icon(self , rsthis: & QSystemTrayIcon) -> QIcon {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSystemTrayIcon4iconEv()};
    let mut ret = unsafe {C_ZNK15QSystemTrayIcon4iconEv(rsthis.qclsinst)};
    let mut ret1 = QIcon::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QMenu * QSystemTrayIcon::contextMenu();
impl /*struct*/ QSystemTrayIcon {
  pub fn contextMenu<RetType, T: QSystemTrayIcon_contextMenu<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.contextMenu(self);
    // return 1;
  }
}

pub trait QSystemTrayIcon_contextMenu<RetType> {
  fn contextMenu(self , rsthis: & QSystemTrayIcon) -> RetType;
}

  // proto:  QMenu * QSystemTrayIcon::contextMenu();
impl<'a> /*trait*/ QSystemTrayIcon_contextMenu<QMenu> for () {
  fn contextMenu(self , rsthis: & QSystemTrayIcon) -> QMenu {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QSystemTrayIcon11contextMenuEv()};
    let mut ret = unsafe {C_ZNK15QSystemTrayIcon11contextMenuEv(rsthis.qclsinst)};
    let mut ret1 = QMenu::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static bool QSystemTrayIcon::isSystemTrayAvailable();
impl /*struct*/ QSystemTrayIcon {
  pub fn isSystemTrayAvailable_s<RetType, T: QSystemTrayIcon_isSystemTrayAvailable_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.isSystemTrayAvailable_s();
    // return 1;
  }
}

pub trait QSystemTrayIcon_isSystemTrayAvailable_s<RetType> {
  fn isSystemTrayAvailable_s(self ) -> RetType;
}

  // proto: static bool QSystemTrayIcon::isSystemTrayAvailable();
impl<'a> /*trait*/ QSystemTrayIcon_isSystemTrayAvailable_s<i8> for () {
  fn isSystemTrayAvailable_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QSystemTrayIcon21isSystemTrayAvailableEv()};
    let mut ret = unsafe {C_ZN15QSystemTrayIcon21isSystemTrayAvailableEv()};
    return ret as i8; // 1
    // return 1;
  }
}

#[derive(Default)] // for QSystemTrayIcon_activated
pub struct QSystemTrayIcon_activated_signal{poi:u64}
impl /* struct */ QSystemTrayIcon {
  pub fn activated(&self) -> QSystemTrayIcon_activated_signal {
     return QSystemTrayIcon_activated_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QSystemTrayIcon_activated_signal {
  pub fn connect<T: QSystemTrayIcon_activated_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QSystemTrayIcon_activated_signal_connect {
  fn connect(self, sigthis: QSystemTrayIcon_activated_signal);
}

#[derive(Default)] // for QSystemTrayIcon_messageClicked
pub struct QSystemTrayIcon_messageClicked_signal{poi:u64}
impl /* struct */ QSystemTrayIcon {
  pub fn messageClicked(&self) -> QSystemTrayIcon_messageClicked_signal {
     return QSystemTrayIcon_messageClicked_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QSystemTrayIcon_messageClicked_signal {
  pub fn connect<T: QSystemTrayIcon_messageClicked_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QSystemTrayIcon_messageClicked_signal_connect {
  fn connect(self, sigthis: QSystemTrayIcon_messageClicked_signal);
}

// messageClicked()
extern fn QSystemTrayIcon_messageClicked_signal_connect_cb_0(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QSystemTrayIcon_messageClicked_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QSystemTrayIcon_messageClicked_signal_connect for fn() {
  fn connect(self, sigthis: QSystemTrayIcon_messageClicked_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QSystemTrayIcon_messageClicked_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QSystemTrayIcon_SlotProxy_connect__ZN15QSystemTrayIcon14messageClickedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QSystemTrayIcon_messageClicked_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QSystemTrayIcon_messageClicked_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QSystemTrayIcon_messageClicked_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QSystemTrayIcon_SlotProxy_connect__ZN15QSystemTrayIcon14messageClickedEv(arg0, arg1, arg2)};
  }
}
// <= body block end

