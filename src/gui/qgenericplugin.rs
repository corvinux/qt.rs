// auto generated, do not modify.
// created: Tue Dec 22 23:21:28 2015
// src-file: /QtGui/qgenericplugin.h
// dst-file: /src/gui/qgenericplugin.rs
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
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  void QGenericPlugin::QGenericPlugin(QObject * parent);
  fn _ZN14QGenericPluginC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QObject * QGenericPlugin::create(const QString & name, const QString & spec);
  fn _ZN14QGenericPlugin6createERK7QStringS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  void QGenericPlugin::~QGenericPlugin();
  fn _ZN14QGenericPluginD0Ev(qthis: *mut c_void);
  // proto:  const QMetaObject * QGenericPlugin::metaObject();
  fn _ZNK14QGenericPlugin10metaObjectEv(qthis: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QGenericPlugin)=1
pub struct QGenericPlugin {
  qbase: QObject,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QGenericPlugin {
  pub fn inheritFrom(qthis: *mut c_void) -> QGenericPlugin {
    return QGenericPlugin{qbase: QObject::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QGenericPlugin {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return &self.qbase;
  }
}
impl AsRef<QObject> for QGenericPlugin {
  fn as_ref(&self) -> &QObject {
    return &self.qbase;
  }
}
  // proto:  void QGenericPlugin::QGenericPlugin(QObject * parent);
impl /*struct*/ QGenericPlugin {
  pub fn NewQGenericPlugin<T: QGenericPlugin_NewQGenericPlugin>(value: T) -> QGenericPlugin {
    let rsthis = value.NewQGenericPlugin();
    return rsthis;
    // return 1;
  }
}

pub trait QGenericPlugin_NewQGenericPlugin {
  fn NewQGenericPlugin(self) -> QGenericPlugin;
}

  // proto:  void QGenericPlugin::QGenericPlugin(QObject * parent);
impl<'a> /*trait*/ QGenericPlugin_NewQGenericPlugin for (QObject) {
  fn NewQGenericPlugin(self) -> QGenericPlugin {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGenericPluginC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN14QGenericPluginC1EP7QObject(qthis, arg0)};
    let rsthis = QGenericPlugin{/**/qbase: QObject::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QObject * QGenericPlugin::create(const QString & name, const QString & spec);
impl /*struct*/ QGenericPlugin {
  pub fn create<RetType, T: QGenericPlugin_create<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.create(self);
    // return 1;
  }
}

pub trait QGenericPlugin_create<RetType> {
  fn create(self , rsthis: &mut QGenericPlugin) -> RetType;
}

  // proto:  QObject * QGenericPlugin::create(const QString & name, const QString & spec);
impl<'a> /*trait*/ QGenericPlugin_create<QObject> for (QString, QString) {
  fn create(self , rsthis: &mut QGenericPlugin) -> QObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGenericPlugin6createERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN14QGenericPlugin6createERK7QStringS2_(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QObject::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGenericPlugin::~QGenericPlugin();
impl /*struct*/ QGenericPlugin {
  pub fn FreeQGenericPlugin<RetType, T: QGenericPlugin_FreeQGenericPlugin<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQGenericPlugin(self);
    // return 1;
  }
}

pub trait QGenericPlugin_FreeQGenericPlugin<RetType> {
  fn FreeQGenericPlugin(self , rsthis: &mut QGenericPlugin) -> RetType;
}

  // proto:  void QGenericPlugin::~QGenericPlugin();
impl<'a> /*trait*/ QGenericPlugin_FreeQGenericPlugin<()> for () {
  fn FreeQGenericPlugin(self , rsthis: &mut QGenericPlugin) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QGenericPluginD0Ev()};
     unsafe {_ZN14QGenericPluginD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QGenericPlugin::metaObject();
impl /*struct*/ QGenericPlugin {
  pub fn metaObject<RetType, T: QGenericPlugin_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QGenericPlugin_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QGenericPlugin) -> RetType;
}

  // proto:  const QMetaObject * QGenericPlugin::metaObject();
impl<'a> /*trait*/ QGenericPlugin_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QGenericPlugin) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QGenericPlugin10metaObjectEv()};
     unsafe {_ZNK14QGenericPlugin10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

