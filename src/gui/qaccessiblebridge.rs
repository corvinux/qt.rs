// auto generated, do not modify.
// created: Sun Jan 24 17:41:38 2016
// src-file: /QtGui/qaccessiblebridge.h
// dst-file: /src/gui/qaccessiblebridge.rs
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
use super::super::core::qobject::QObject; // 771
use std::ops::Deref;
use super::super::core::qstring::QString; // 771
// use super::qaccessiblebridge::QAccessibleBridge; // 773
use super::super::core::qobjectdefs::QMetaObject; // 771
use super::qaccessible::QAccessibleEvent; // 773
use super::qaccessible::QAccessibleInterface; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QAccessibleBridgePlugin_Class_Size() -> c_int;
  // proto:  void QAccessibleBridgePlugin::QAccessibleBridgePlugin(QObject * parent);
  fn C_ZN23QAccessibleBridgePluginC2EP7QObject(arg0: *mut c_void) -> u64;
  // proto:  QAccessibleBridge * QAccessibleBridgePlugin::create(const QString & key);
  fn C_ZN23QAccessibleBridgePlugin6createERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QAccessibleBridgePlugin::~QAccessibleBridgePlugin();
  fn C_ZN23QAccessibleBridgePluginD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  const QMetaObject * QAccessibleBridgePlugin::metaObject();
  fn C_ZNK23QAccessibleBridgePlugin10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  fn QAccessibleBridge_Class_Size() -> c_int;
  // proto:  void QAccessibleBridge::~QAccessibleBridge();
  fn C_ZN17QAccessibleBridgeD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QAccessibleBridge::notifyAccessibilityUpdate(QAccessibleEvent * event);
  fn C_ZN17QAccessibleBridge25notifyAccessibilityUpdateEP16QAccessibleEvent(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QAccessibleBridge::setRootObject(QAccessibleInterface * );
  fn C_ZN17QAccessibleBridge13setRootObjectEP20QAccessibleInterface(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QAccessibleBridgePlugin)=1
#[derive(Default)]
pub struct QAccessibleBridgePlugin {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QAccessibleBridge)=8
#[derive(Default)]
pub struct QAccessibleBridge {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QAccessibleBridgePlugin {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QAccessibleBridgePlugin {
    return QAccessibleBridgePlugin{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QAccessibleBridgePlugin {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QAccessibleBridgePlugin {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  void QAccessibleBridgePlugin::QAccessibleBridgePlugin(QObject * parent);
impl /*struct*/ QAccessibleBridgePlugin {
  pub fn new<T: QAccessibleBridgePlugin_new>(value: T) -> QAccessibleBridgePlugin {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QAccessibleBridgePlugin_new {
  fn new(self) -> QAccessibleBridgePlugin;
}

  // proto:  void QAccessibleBridgePlugin::QAccessibleBridgePlugin(QObject * parent);
impl<'a> /*trait*/ QAccessibleBridgePlugin_new for (&'a QObject) {
  fn new(self) -> QAccessibleBridgePlugin {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QAccessibleBridgePluginC2EP7QObject()};
    let ctysz: c_int = unsafe{QAccessibleBridgePlugin_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN23QAccessibleBridgePluginC2EP7QObject(arg0)};
    let rsthis = QAccessibleBridgePlugin{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QAccessibleBridge * QAccessibleBridgePlugin::create(const QString & key);
impl /*struct*/ QAccessibleBridgePlugin {
  pub fn create<RetType, T: QAccessibleBridgePlugin_create<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.create(self);
    // return 1;
  }
}

pub trait QAccessibleBridgePlugin_create<RetType> {
  fn create(self , rsthis: & QAccessibleBridgePlugin) -> RetType;
}

  // proto:  QAccessibleBridge * QAccessibleBridgePlugin::create(const QString & key);
impl<'a> /*trait*/ QAccessibleBridgePlugin_create<QAccessibleBridge> for (&'a QString) {
  fn create(self , rsthis: & QAccessibleBridgePlugin) -> QAccessibleBridge {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QAccessibleBridgePlugin6createERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN23QAccessibleBridgePlugin6createERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QAccessibleBridge::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QAccessibleBridgePlugin::~QAccessibleBridgePlugin();
impl /*struct*/ QAccessibleBridgePlugin {
  pub fn free<RetType, T: QAccessibleBridgePlugin_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QAccessibleBridgePlugin_free<RetType> {
  fn free(self , rsthis: & QAccessibleBridgePlugin) -> RetType;
}

  // proto:  void QAccessibleBridgePlugin::~QAccessibleBridgePlugin();
impl<'a> /*trait*/ QAccessibleBridgePlugin_free<()> for () {
  fn free(self , rsthis: & QAccessibleBridgePlugin) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QAccessibleBridgePluginD2Ev()};
     unsafe {C_ZN23QAccessibleBridgePluginD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QAccessibleBridgePlugin::metaObject();
impl /*struct*/ QAccessibleBridgePlugin {
  pub fn metaObject<RetType, T: QAccessibleBridgePlugin_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QAccessibleBridgePlugin_metaObject<RetType> {
  fn metaObject(self , rsthis: & QAccessibleBridgePlugin) -> RetType;
}

  // proto:  const QMetaObject * QAccessibleBridgePlugin::metaObject();
impl<'a> /*trait*/ QAccessibleBridgePlugin_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QAccessibleBridgePlugin) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QAccessibleBridgePlugin10metaObjectEv()};
    let mut ret = unsafe {C_ZNK23QAccessibleBridgePlugin10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QAccessibleBridge {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QAccessibleBridge {
    return QAccessibleBridge{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QAccessibleBridge::~QAccessibleBridge();
impl /*struct*/ QAccessibleBridge {
  pub fn free<RetType, T: QAccessibleBridge_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QAccessibleBridge_free<RetType> {
  fn free(self , rsthis: & QAccessibleBridge) -> RetType;
}

  // proto:  void QAccessibleBridge::~QAccessibleBridge();
impl<'a> /*trait*/ QAccessibleBridge_free<()> for () {
  fn free(self , rsthis: & QAccessibleBridge) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QAccessibleBridgeD2Ev()};
     unsafe {C_ZN17QAccessibleBridgeD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QAccessibleBridge::notifyAccessibilityUpdate(QAccessibleEvent * event);
impl /*struct*/ QAccessibleBridge {
  pub fn notifyAccessibilityUpdate<RetType, T: QAccessibleBridge_notifyAccessibilityUpdate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.notifyAccessibilityUpdate(self);
    // return 1;
  }
}

pub trait QAccessibleBridge_notifyAccessibilityUpdate<RetType> {
  fn notifyAccessibilityUpdate(self , rsthis: & QAccessibleBridge) -> RetType;
}

  // proto:  void QAccessibleBridge::notifyAccessibilityUpdate(QAccessibleEvent * event);
impl<'a> /*trait*/ QAccessibleBridge_notifyAccessibilityUpdate<()> for (&'a QAccessibleEvent) {
  fn notifyAccessibilityUpdate(self , rsthis: & QAccessibleBridge) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QAccessibleBridge25notifyAccessibilityUpdateEP16QAccessibleEvent()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN17QAccessibleBridge25notifyAccessibilityUpdateEP16QAccessibleEvent(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QAccessibleBridge::setRootObject(QAccessibleInterface * );
impl /*struct*/ QAccessibleBridge {
  pub fn setRootObject<RetType, T: QAccessibleBridge_setRootObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setRootObject(self);
    // return 1;
  }
}

pub trait QAccessibleBridge_setRootObject<RetType> {
  fn setRootObject(self , rsthis: & QAccessibleBridge) -> RetType;
}

  // proto:  void QAccessibleBridge::setRootObject(QAccessibleInterface * );
impl<'a> /*trait*/ QAccessibleBridge_setRootObject<()> for (&'a QAccessibleInterface) {
  fn setRootObject(self , rsthis: & QAccessibleBridge) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QAccessibleBridge13setRootObjectEP20QAccessibleInterface()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN17QAccessibleBridge13setRootObjectEP20QAccessibleInterface(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// <= body block end

