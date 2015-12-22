// auto generated, do not modify.
// created: Tue Dec 22 23:21:28 2015
// src-file: /QtGui/qiconengineplugin.h
// dst-file: /src/gui/qiconengineplugin.rs
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
use super::qiconengine::QIconEngine; // 773
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  const QMetaObject * QIconEnginePlugin::metaObject();
  fn _ZNK17QIconEnginePlugin10metaObjectEv(qthis: *mut c_void);
  // proto:  void QIconEnginePlugin::QIconEnginePlugin(QObject * parent);
  fn _ZN17QIconEnginePluginC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QIconEngine * QIconEnginePlugin::create(const QString & filename);
  fn _ZN17QIconEnginePlugin6createERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QIconEnginePlugin::~QIconEnginePlugin();
  fn _ZN17QIconEnginePluginD0Ev(qthis: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QIconEnginePlugin)=1
pub struct QIconEnginePlugin {
  qbase: QObject,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QIconEnginePlugin {
  pub fn inheritFrom(qthis: *mut c_void) -> QIconEnginePlugin {
    return QIconEnginePlugin{qbase: QObject::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QIconEnginePlugin {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return &self.qbase;
  }
}
impl AsRef<QObject> for QIconEnginePlugin {
  fn as_ref(&self) -> &QObject {
    return &self.qbase;
  }
}
  // proto:  const QMetaObject * QIconEnginePlugin::metaObject();
impl /*struct*/ QIconEnginePlugin {
  pub fn metaObject<RetType, T: QIconEnginePlugin_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QIconEnginePlugin_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QIconEnginePlugin) -> RetType;
}

  // proto:  const QMetaObject * QIconEnginePlugin::metaObject();
impl<'a> /*trait*/ QIconEnginePlugin_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QIconEnginePlugin) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QIconEnginePlugin10metaObjectEv()};
     unsafe {_ZNK17QIconEnginePlugin10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QIconEnginePlugin::QIconEnginePlugin(QObject * parent);
impl /*struct*/ QIconEnginePlugin {
  pub fn NewQIconEnginePlugin<T: QIconEnginePlugin_NewQIconEnginePlugin>(value: T) -> QIconEnginePlugin {
    let rsthis = value.NewQIconEnginePlugin();
    return rsthis;
    // return 1;
  }
}

pub trait QIconEnginePlugin_NewQIconEnginePlugin {
  fn NewQIconEnginePlugin(self) -> QIconEnginePlugin;
}

  // proto:  void QIconEnginePlugin::QIconEnginePlugin(QObject * parent);
impl<'a> /*trait*/ QIconEnginePlugin_NewQIconEnginePlugin for (QObject) {
  fn NewQIconEnginePlugin(self) -> QIconEnginePlugin {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QIconEnginePluginC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN17QIconEnginePluginC1EP7QObject(qthis, arg0)};
    let rsthis = QIconEnginePlugin{/**/qbase: QObject::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QIconEngine * QIconEnginePlugin::create(const QString & filename);
impl /*struct*/ QIconEnginePlugin {
  pub fn create<RetType, T: QIconEnginePlugin_create<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.create(self);
    // return 1;
  }
}

pub trait QIconEnginePlugin_create<RetType> {
  fn create(self , rsthis: &mut QIconEnginePlugin) -> RetType;
}

  // proto:  QIconEngine * QIconEnginePlugin::create(const QString & filename);
impl<'a> /*trait*/ QIconEnginePlugin_create<QIconEngine> for (QString) {
  fn create(self , rsthis: &mut QIconEnginePlugin) -> QIconEngine {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QIconEnginePlugin6createERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN17QIconEnginePlugin6createERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QIconEngine::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QIconEnginePlugin::~QIconEnginePlugin();
impl /*struct*/ QIconEnginePlugin {
  pub fn FreeQIconEnginePlugin<RetType, T: QIconEnginePlugin_FreeQIconEnginePlugin<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQIconEnginePlugin(self);
    // return 1;
  }
}

pub trait QIconEnginePlugin_FreeQIconEnginePlugin<RetType> {
  fn FreeQIconEnginePlugin(self , rsthis: &mut QIconEnginePlugin) -> RetType;
}

  // proto:  void QIconEnginePlugin::~QIconEnginePlugin();
impl<'a> /*trait*/ QIconEnginePlugin_FreeQIconEnginePlugin<()> for () {
  fn FreeQIconEnginePlugin(self , rsthis: &mut QIconEnginePlugin) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QIconEnginePluginD0Ev()};
     unsafe {_ZN17QIconEnginePluginD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

