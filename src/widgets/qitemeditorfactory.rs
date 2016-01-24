// auto generated, do not modify.
// created: Sun Jan 24 17:41:38 2016
// src-file: /QtWidgets/qitemeditorfactory.h
// dst-file: /src/widgets/qitemeditorfactory.rs
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
use std::ops::Deref;
use super::super::core::qbytearray::QByteArray; // 771
use super::qwidget::QWidget; // 773
// use super::qitemeditorfactory::QItemEditorCreatorBase; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QItemEditorCreatorBase_Class_Size() -> c_int;
  // proto:  QByteArray QItemEditorCreatorBase::valuePropertyName();
  fn C_ZNK22QItemEditorCreatorBase17valuePropertyNameEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QWidget * QItemEditorCreatorBase::createWidget(QWidget * parent);
  fn C_ZNK22QItemEditorCreatorBase12createWidgetEP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QItemEditorCreatorBase::~QItemEditorCreatorBase();
  fn C_ZN22QItemEditorCreatorBaseD2Ev(qthis: u64 /* *mut c_void*/);
  fn QItemEditorFactory_Class_Size() -> c_int;
  // proto:  void QItemEditorFactory::QItemEditorFactory();
  fn C_ZN18QItemEditorFactoryC2Ev() -> u64;
  // proto:  QByteArray QItemEditorFactory::valuePropertyName(int userType);
  fn C_ZNK18QItemEditorFactory17valuePropertyNameEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto: static const QItemEditorFactory * QItemEditorFactory::defaultFactory();
  fn C_ZN18QItemEditorFactory14defaultFactoryEv() -> *mut c_void;
  // proto:  void QItemEditorFactory::~QItemEditorFactory();
  fn C_ZN18QItemEditorFactoryD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QItemEditorFactory::registerEditor(int userType, QItemEditorCreatorBase * creator);
  fn C_ZN18QItemEditorFactory14registerEditorEiP22QItemEditorCreatorBase(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void);
  // proto: static void QItemEditorFactory::setDefaultFactory(QItemEditorFactory * factory);
  fn C_ZN18QItemEditorFactory17setDefaultFactoryEPS_(arg0: *mut c_void);
  // proto:  QWidget * QItemEditorFactory::createEditor(int userType, QWidget * parent);
  fn C_ZNK18QItemEditorFactory12createEditorEiP7QWidget(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void) -> *mut c_void;
} // <= ext block end

// body block begin =>
// class sizeof(QItemEditorCreatorBase)=8
#[derive(Default)]
pub struct QItemEditorCreatorBase {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QItemEditorFactory)=1
#[derive(Default)]
pub struct QItemEditorFactory {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QItemEditorCreatorBase {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QItemEditorCreatorBase {
    return QItemEditorCreatorBase{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  QByteArray QItemEditorCreatorBase::valuePropertyName();
impl /*struct*/ QItemEditorCreatorBase {
  pub fn valuePropertyName<RetType, T: QItemEditorCreatorBase_valuePropertyName<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.valuePropertyName(self);
    // return 1;
  }
}

pub trait QItemEditorCreatorBase_valuePropertyName<RetType> {
  fn valuePropertyName(self , rsthis: & QItemEditorCreatorBase) -> RetType;
}

  // proto:  QByteArray QItemEditorCreatorBase::valuePropertyName();
impl<'a> /*trait*/ QItemEditorCreatorBase_valuePropertyName<QByteArray> for () {
  fn valuePropertyName(self , rsthis: & QItemEditorCreatorBase) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QItemEditorCreatorBase17valuePropertyNameEv()};
    let mut ret = unsafe {C_ZNK22QItemEditorCreatorBase17valuePropertyNameEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QWidget * QItemEditorCreatorBase::createWidget(QWidget * parent);
impl /*struct*/ QItemEditorCreatorBase {
  pub fn createWidget<RetType, T: QItemEditorCreatorBase_createWidget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.createWidget(self);
    // return 1;
  }
}

pub trait QItemEditorCreatorBase_createWidget<RetType> {
  fn createWidget(self , rsthis: & QItemEditorCreatorBase) -> RetType;
}

  // proto:  QWidget * QItemEditorCreatorBase::createWidget(QWidget * parent);
impl<'a> /*trait*/ QItemEditorCreatorBase_createWidget<QWidget> for (&'a QWidget) {
  fn createWidget(self , rsthis: & QItemEditorCreatorBase) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QItemEditorCreatorBase12createWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK22QItemEditorCreatorBase12createWidgetEP7QWidget(rsthis.qclsinst, arg0)};
    let mut ret1 = QWidget::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QItemEditorCreatorBase::~QItemEditorCreatorBase();
impl /*struct*/ QItemEditorCreatorBase {
  pub fn free<RetType, T: QItemEditorCreatorBase_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QItemEditorCreatorBase_free<RetType> {
  fn free(self , rsthis: & QItemEditorCreatorBase) -> RetType;
}

  // proto:  void QItemEditorCreatorBase::~QItemEditorCreatorBase();
impl<'a> /*trait*/ QItemEditorCreatorBase_free<()> for () {
  fn free(self , rsthis: & QItemEditorCreatorBase) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QItemEditorCreatorBaseD2Ev()};
     unsafe {C_ZN22QItemEditorCreatorBaseD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QItemEditorFactory {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QItemEditorFactory {
    return QItemEditorFactory{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QItemEditorFactory::QItemEditorFactory();
impl /*struct*/ QItemEditorFactory {
  pub fn new<T: QItemEditorFactory_new>(value: T) -> QItemEditorFactory {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QItemEditorFactory_new {
  fn new(self) -> QItemEditorFactory;
}

  // proto:  void QItemEditorFactory::QItemEditorFactory();
impl<'a> /*trait*/ QItemEditorFactory_new for () {
  fn new(self) -> QItemEditorFactory {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QItemEditorFactoryC2Ev()};
    let ctysz: c_int = unsafe{QItemEditorFactory_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN18QItemEditorFactoryC2Ev()};
    let rsthis = QItemEditorFactory{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QByteArray QItemEditorFactory::valuePropertyName(int userType);
impl /*struct*/ QItemEditorFactory {
  pub fn valuePropertyName<RetType, T: QItemEditorFactory_valuePropertyName<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.valuePropertyName(self);
    // return 1;
  }
}

pub trait QItemEditorFactory_valuePropertyName<RetType> {
  fn valuePropertyName(self , rsthis: & QItemEditorFactory) -> RetType;
}

  // proto:  QByteArray QItemEditorFactory::valuePropertyName(int userType);
impl<'a> /*trait*/ QItemEditorFactory_valuePropertyName<QByteArray> for (i32) {
  fn valuePropertyName(self , rsthis: & QItemEditorFactory) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QItemEditorFactory17valuePropertyNameEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZNK18QItemEditorFactory17valuePropertyNameEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QByteArray::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static const QItemEditorFactory * QItemEditorFactory::defaultFactory();
impl /*struct*/ QItemEditorFactory {
  pub fn defaultFactory_s<RetType, T: QItemEditorFactory_defaultFactory_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.defaultFactory_s();
    // return 1;
  }
}

pub trait QItemEditorFactory_defaultFactory_s<RetType> {
  fn defaultFactory_s(self ) -> RetType;
}

  // proto: static const QItemEditorFactory * QItemEditorFactory::defaultFactory();
impl<'a> /*trait*/ QItemEditorFactory_defaultFactory_s<QItemEditorFactory> for () {
  fn defaultFactory_s(self ) -> QItemEditorFactory {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QItemEditorFactory14defaultFactoryEv()};
    let mut ret = unsafe {C_ZN18QItemEditorFactory14defaultFactoryEv()};
    let mut ret1 = QItemEditorFactory::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QItemEditorFactory::~QItemEditorFactory();
impl /*struct*/ QItemEditorFactory {
  pub fn free<RetType, T: QItemEditorFactory_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QItemEditorFactory_free<RetType> {
  fn free(self , rsthis: & QItemEditorFactory) -> RetType;
}

  // proto:  void QItemEditorFactory::~QItemEditorFactory();
impl<'a> /*trait*/ QItemEditorFactory_free<()> for () {
  fn free(self , rsthis: & QItemEditorFactory) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QItemEditorFactoryD2Ev()};
     unsafe {C_ZN18QItemEditorFactoryD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QItemEditorFactory::registerEditor(int userType, QItemEditorCreatorBase * creator);
impl /*struct*/ QItemEditorFactory {
  pub fn registerEditor<RetType, T: QItemEditorFactory_registerEditor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.registerEditor(self);
    // return 1;
  }
}

pub trait QItemEditorFactory_registerEditor<RetType> {
  fn registerEditor(self , rsthis: & QItemEditorFactory) -> RetType;
}

  // proto:  void QItemEditorFactory::registerEditor(int userType, QItemEditorCreatorBase * creator);
impl<'a> /*trait*/ QItemEditorFactory_registerEditor<()> for (i32, &'a QItemEditorCreatorBase) {
  fn registerEditor(self , rsthis: & QItemEditorFactory) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QItemEditorFactory14registerEditorEiP22QItemEditorCreatorBase()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {C_ZN18QItemEditorFactory14registerEditorEiP22QItemEditorCreatorBase(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto: static void QItemEditorFactory::setDefaultFactory(QItemEditorFactory * factory);
impl /*struct*/ QItemEditorFactory {
  pub fn setDefaultFactory_s<RetType, T: QItemEditorFactory_setDefaultFactory_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setDefaultFactory_s();
    // return 1;
  }
}

pub trait QItemEditorFactory_setDefaultFactory_s<RetType> {
  fn setDefaultFactory_s(self ) -> RetType;
}

  // proto: static void QItemEditorFactory::setDefaultFactory(QItemEditorFactory * factory);
impl<'a> /*trait*/ QItemEditorFactory_setDefaultFactory_s<()> for (&'a QItemEditorFactory) {
  fn setDefaultFactory_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QItemEditorFactory17setDefaultFactoryEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN18QItemEditorFactory17setDefaultFactoryEPS_(arg0)};
    // return 1;
  }
}

  // proto:  QWidget * QItemEditorFactory::createEditor(int userType, QWidget * parent);
impl /*struct*/ QItemEditorFactory {
  pub fn createEditor<RetType, T: QItemEditorFactory_createEditor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.createEditor(self);
    // return 1;
  }
}

pub trait QItemEditorFactory_createEditor<RetType> {
  fn createEditor(self , rsthis: & QItemEditorFactory) -> RetType;
}

  // proto:  QWidget * QItemEditorFactory::createEditor(int userType, QWidget * parent);
impl<'a> /*trait*/ QItemEditorFactory_createEditor<QWidget> for (i32, &'a QWidget) {
  fn createEditor(self , rsthis: & QItemEditorFactory) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QItemEditorFactory12createEditorEiP7QWidget()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK18QItemEditorFactory12createEditorEiP7QWidget(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QWidget::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

// <= body block end

