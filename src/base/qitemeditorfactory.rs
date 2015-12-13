// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qbytearray::QByteArray;
use super::qitemeditorcreatorbase::QItemEditorCreatorBase;
use super::qwidget::QWidget;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QItemEditorFactory::NewQItemEditorFactory();
  fn _ZN18QItemEditorFactoryC1Ev(qthis: *mut c_void) ;
  // proto:  QByteArray QItemEditorFactory::valuePropertyName(int userType);
  fn _ZNK18QItemEditorFactory17valuePropertyNameEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto: static const QItemEditorFactory * QItemEditorFactory::defaultFactory();
  fn _ZN18QItemEditorFactory14defaultFactoryEv() -> *mut c_void;
  // proto:  void QItemEditorFactory::FreeQItemEditorFactory();
  fn _ZN18QItemEditorFactoryD0Ev(qthis: *mut c_void) ;
  // proto:  void QItemEditorFactory::registerEditor(int userType, QItemEditorCreatorBase * creator);
  fn _ZN18QItemEditorFactory14registerEditorEiP22QItemEditorCreatorBase(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) ;
  // proto: static void QItemEditorFactory::setDefaultFactory(QItemEditorFactory * factory);
  fn _ZN18QItemEditorFactory17setDefaultFactoryEPS_(arg0: *mut c_void) ;
  // proto:  QWidget * QItemEditorFactory::createEditor(int userType, QWidget * parent);
  fn _ZNK18QItemEditorFactory12createEditorEiP7QWidget(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) -> *mut c_void;
}

// body block begin
// class sizeof(QItemEditorFactory)=1
pub struct QItemEditorFactory {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QItemEditorFactory {
  pub fn NewQItemEditorFactory<T: QItemEditorFactory_NewQItemEditorFactory>(value: T) -> QItemEditorFactory {
    let rsthis = value.NewQItemEditorFactory();
    return rsthis;
    // return 1;
  }
}

pub trait QItemEditorFactory_NewQItemEditorFactory {
  fn NewQItemEditorFactory(self) -> QItemEditorFactory;
}

// proto: void QItemEditorFactory::NewQItemEditorFactory();
impl<'a> /*trait*/ QItemEditorFactory_NewQItemEditorFactory for () {
  fn NewQItemEditorFactory(self) -> QItemEditorFactory {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QItemEditorFactoryC1Ev()};
    unsafe {_ZN18QItemEditorFactoryC1Ev(qthis)};
    let rsthis = QItemEditorFactory{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QItemEditorFactory {
  pub fn valuePropertyName<T: QItemEditorFactory_valuePropertyName>(&mut self, value: T) -> QByteArray {
    return value.valuePropertyName(self);
    // return 1;
  }
}

pub trait QItemEditorFactory_valuePropertyName {
  fn valuePropertyName(self, rsthis: &mut QItemEditorFactory) -> QByteArray;
}

// proto:  QByteArray QItemEditorFactory::valuePropertyName(int userType);
impl<'a> /*trait*/ QItemEditorFactory_valuePropertyName for (i32) {
  fn valuePropertyName(self, rsthis: &mut QItemEditorFactory) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QItemEditorFactory17valuePropertyNameEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK18QItemEditorFactory17valuePropertyNameEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QByteArray{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QItemEditorFactory {
  pub fn defaultFactory<T: QItemEditorFactory_defaultFactory>(&mut self, value: T) -> QItemEditorFactory {
    return value.defaultFactory(self);
    // return 1;
  }
}

pub trait QItemEditorFactory_defaultFactory {
  fn defaultFactory(self, rsthis: &mut QItemEditorFactory) -> QItemEditorFactory;
}

// proto: static const QItemEditorFactory * QItemEditorFactory::defaultFactory();
impl<'a> /*trait*/ QItemEditorFactory_defaultFactory for () {
  fn defaultFactory(self, rsthis: &mut QItemEditorFactory) -> QItemEditorFactory {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QItemEditorFactory14defaultFactoryEv()};
    let mut ret = unsafe {_ZN18QItemEditorFactory14defaultFactoryEv()};
    let mut ret1 = QItemEditorFactory{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QItemEditorFactory {
  pub fn FreeQItemEditorFactory<T: QItemEditorFactory_FreeQItemEditorFactory>(&mut self, value: T)  {
     value.FreeQItemEditorFactory(self);
    // return 1;
  }
}

pub trait QItemEditorFactory_FreeQItemEditorFactory {
  fn FreeQItemEditorFactory(self, rsthis: &mut QItemEditorFactory) ;
}

// proto:  void QItemEditorFactory::FreeQItemEditorFactory();
impl<'a> /*trait*/ QItemEditorFactory_FreeQItemEditorFactory for () {
  fn FreeQItemEditorFactory(self, rsthis: &mut QItemEditorFactory)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QItemEditorFactoryD0Ev()};
     unsafe {_ZN18QItemEditorFactoryD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QItemEditorFactory {
  pub fn registerEditor<T: QItemEditorFactory_registerEditor>(&mut self, value: T)  {
     value.registerEditor(self);
    // return 1;
  }
}

pub trait QItemEditorFactory_registerEditor {
  fn registerEditor(self, rsthis: &mut QItemEditorFactory) ;
}

// proto:  void QItemEditorFactory::registerEditor(int userType, QItemEditorCreatorBase * creator);
impl<'a> /*trait*/ QItemEditorFactory_registerEditor for (i32, &'a mut QItemEditorCreatorBase) {
  fn registerEditor(self, rsthis: &mut QItemEditorFactory)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QItemEditorFactory14registerEditorEiP22QItemEditorCreatorBase()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN18QItemEditorFactory14registerEditorEiP22QItemEditorCreatorBase(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QItemEditorFactory {
  pub fn setDefaultFactory<T: QItemEditorFactory_setDefaultFactory>(&mut self, value: T)  {
     value.setDefaultFactory(self);
    // return 1;
  }
}

pub trait QItemEditorFactory_setDefaultFactory {
  fn setDefaultFactory(self, rsthis: &mut QItemEditorFactory) ;
}

// proto: static void QItemEditorFactory::setDefaultFactory(QItemEditorFactory * factory);
impl<'a> /*trait*/ QItemEditorFactory_setDefaultFactory for (&'a mut QItemEditorFactory) {
  fn setDefaultFactory(self, rsthis: &mut QItemEditorFactory)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QItemEditorFactory17setDefaultFactoryEPS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN18QItemEditorFactory17setDefaultFactoryEPS_(arg0)};
    // return 1;
  }
}

impl /*struct*/ QItemEditorFactory {
  pub fn createEditor<T: QItemEditorFactory_createEditor>(&mut self, value: T) -> QWidget {
    return value.createEditor(self);
    // return 1;
  }
}

pub trait QItemEditorFactory_createEditor {
  fn createEditor(self, rsthis: &mut QItemEditorFactory) -> QWidget;
}

// proto:  QWidget * QItemEditorFactory::createEditor(int userType, QWidget * parent);
impl<'a> /*trait*/ QItemEditorFactory_createEditor for (i32, &'a mut QWidget) {
  fn createEditor(self, rsthis: &mut QItemEditorFactory) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QItemEditorFactory12createEditorEiP7QWidget()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK18QItemEditorFactory12createEditorEiP7QWidget(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

