// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qobject::QObject;
use super::qstring::QString;
use super::qwidget::QWidget;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QSignalMapper::removeMappings(QObject * sender);
  fn _ZN13QSignalMapper14removeMappingsEP7QObject(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QSignalMapper::mapped(int );
  fn _ZN13QSignalMapper6mappedEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QSignalMapper::NewQSignalMapper(const QSignalMapper & );
  fn _ZN13QSignalMapperC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  const QMetaObject * QSignalMapper::metaObject();
  fn _ZNK13QSignalMapper10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QSignalMapper::setMapping(QObject * sender, QObject * object);
  fn _ZN13QSignalMapper10setMappingEP7QObjectS1_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QSignalMapper::mapped(QObject * );
  fn _ZN13QSignalMapper6mappedEP7QObject(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QObject * QSignalMapper::mapping(int id);
  fn _ZNK13QSignalMapper7mappingEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  void QSignalMapper::NewQSignalMapper(QObject * parent);
  fn _ZN13QSignalMapperC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QSignalMapper::FreeQSignalMapper();
  fn _ZN13QSignalMapperD0Ev(qthis: *mut c_void) ;
  // proto:  void QSignalMapper::mapped(const QString & );
  fn _ZN13QSignalMapper6mappedERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QSignalMapper::setMapping(QObject * sender, int id);
  fn _ZN13QSignalMapper10setMappingEP7QObjecti(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) ;
  // proto:  QObject * QSignalMapper::mapping(const QString & text);
  fn _ZNK13QSignalMapper7mappingERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QObject * QSignalMapper::mapping(QObject * object);
  fn _ZNK13QSignalMapper7mappingEP7QObject(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QSignalMapper::mapped(QWidget * );
  fn _ZN13QSignalMapper6mappedEP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QSignalMapper::setMapping(QObject * sender, const QString & text);
  fn _ZN13QSignalMapper10setMappingEP7QObjectRK7QString(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  QObject * QSignalMapper::mapping(QWidget * widget);
  fn _ZNK13QSignalMapper7mappingEP7QWidget(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QSignalMapper::setMapping(QObject * sender, QWidget * widget);
  fn _ZN13QSignalMapper10setMappingEP7QObjectP7QWidget(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
}

// body block begin
// class sizeof(QSignalMapper)=1
pub struct QSignalMapper {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QSignalMapper {
  pub fn removeMappings<T: QSignalMapper_removeMappings>(&mut self, value: T)  {
     value.removeMappings(self);
    // return 1;
  }
}

pub trait QSignalMapper_removeMappings {
  fn removeMappings(self, rsthis: &mut QSignalMapper) ;
}

// proto:  void QSignalMapper::removeMappings(QObject * sender);
impl<'a> /*trait*/ QSignalMapper_removeMappings for (&'a mut QObject) {
  fn removeMappings(self, rsthis: &mut QSignalMapper)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSignalMapper14removeMappingsEP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QSignalMapper14removeMappingsEP7QObject(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QSignalMapper {
  pub fn mapped<T: QSignalMapper_mapped>(&mut self, value: T)  {
     value.mapped(self);
    // return 1;
  }
}

pub trait QSignalMapper_mapped {
  fn mapped(self, rsthis: &mut QSignalMapper) ;
}

// proto:  void QSignalMapper::mapped(int );
impl<'a> /*trait*/ QSignalMapper_mapped for (i32) {
  fn mapped(self, rsthis: &mut QSignalMapper)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSignalMapper6mappedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN13QSignalMapper6mappedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QSignalMapper {
  pub fn NewQSignalMapper<T: QSignalMapper_NewQSignalMapper>(value: T) -> QSignalMapper {
    let rsthis = value.NewQSignalMapper();
    return rsthis;
    // return 1;
  }
}

pub trait QSignalMapper_NewQSignalMapper {
  fn NewQSignalMapper(self) -> QSignalMapper;
}

// proto: void QSignalMapper::NewQSignalMapper(const QSignalMapper & );
impl<'a> /*trait*/ QSignalMapper_NewQSignalMapper for (&'a  QSignalMapper) {
  fn NewQSignalMapper(self) -> QSignalMapper {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSignalMapperC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QSignalMapperC1ERKS_(qthis, arg0)};
    let rsthis = QSignalMapper{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QSignalMapper {
  pub fn metaObject<T: QSignalMapper_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QSignalMapper_metaObject {
  fn metaObject(self, rsthis: &mut QSignalMapper) ;
}

// proto:  const QMetaObject * QSignalMapper::metaObject();
impl<'a> /*trait*/ QSignalMapper_metaObject for () {
  fn metaObject(self, rsthis: &mut QSignalMapper)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QSignalMapper10metaObjectEv()};
     unsafe {_ZNK13QSignalMapper10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QSignalMapper {
  pub fn setMapping<T: QSignalMapper_setMapping>(&mut self, value: T)  {
     value.setMapping(self);
    // return 1;
  }
}

pub trait QSignalMapper_setMapping {
  fn setMapping(self, rsthis: &mut QSignalMapper) ;
}

// proto:  void QSignalMapper::setMapping(QObject * sender, QObject * object);
impl<'a> /*trait*/ QSignalMapper_setMapping for (&'a mut QObject, &'a mut QObject) {
  fn setMapping(self, rsthis: &mut QSignalMapper)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSignalMapper10setMappingEP7QObjectS1_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN13QSignalMapper10setMappingEP7QObjectS1_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  void QSignalMapper::mapped(QObject * );
impl<'a> /*trait*/ QSignalMapper_mapped for (&'a mut QObject) {
  fn mapped(self, rsthis: &mut QSignalMapper)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSignalMapper6mappedEP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QSignalMapper6mappedEP7QObject(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QSignalMapper {
  pub fn mapping<T: QSignalMapper_mapping>(&mut self, value: T) -> QObject {
    return value.mapping(self);
    // return 1;
  }
}

pub trait QSignalMapper_mapping {
  fn mapping(self, rsthis: &mut QSignalMapper) -> QObject;
}

// proto:  QObject * QSignalMapper::mapping(int id);
impl<'a> /*trait*/ QSignalMapper_mapping for (i32) {
  fn mapping(self, rsthis: &mut QSignalMapper) -> QObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QSignalMapper7mappingEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK13QSignalMapper7mappingEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QObject{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QSignalMapper::NewQSignalMapper(QObject * parent);
impl<'a> /*trait*/ QSignalMapper_NewQSignalMapper for (&'a mut QObject) {
  fn NewQSignalMapper(self) -> QSignalMapper {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSignalMapperC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN13QSignalMapperC1EP7QObject(qthis, arg0)};
    let rsthis = QSignalMapper{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QSignalMapper {
  pub fn FreeQSignalMapper<T: QSignalMapper_FreeQSignalMapper>(&mut self, value: T)  {
     value.FreeQSignalMapper(self);
    // return 1;
  }
}

pub trait QSignalMapper_FreeQSignalMapper {
  fn FreeQSignalMapper(self, rsthis: &mut QSignalMapper) ;
}

// proto:  void QSignalMapper::FreeQSignalMapper();
impl<'a> /*trait*/ QSignalMapper_FreeQSignalMapper for () {
  fn FreeQSignalMapper(self, rsthis: &mut QSignalMapper)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSignalMapperD0Ev()};
     unsafe {_ZN13QSignalMapperD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  void QSignalMapper::mapped(const QString & );
impl<'a> /*trait*/ QSignalMapper_mapped for (&'a  QString) {
  fn mapped(self, rsthis: &mut QSignalMapper)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSignalMapper6mappedERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QSignalMapper6mappedERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QSignalMapper::setMapping(QObject * sender, int id);
impl<'a> /*trait*/ QSignalMapper_setMapping for (&'a mut QObject, i32) {
  fn setMapping(self, rsthis: &mut QSignalMapper)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSignalMapper10setMappingEP7QObjecti()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN13QSignalMapper10setMappingEP7QObjecti(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  QObject * QSignalMapper::mapping(const QString & text);
impl<'a> /*trait*/ QSignalMapper_mapping for (&'a  QString) {
  fn mapping(self, rsthis: &mut QSignalMapper) -> QObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QSignalMapper7mappingERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QSignalMapper7mappingERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QObject{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QObject * QSignalMapper::mapping(QObject * object);
impl<'a> /*trait*/ QSignalMapper_mapping for (&'a mut QObject) {
  fn mapping(self, rsthis: &mut QSignalMapper) -> QObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QSignalMapper7mappingEP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QSignalMapper7mappingEP7QObject(rsthis.qclsinst, arg0)};
    let mut ret1 = QObject{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QSignalMapper::mapped(QWidget * );
impl<'a> /*trait*/ QSignalMapper_mapped for (&'a mut QWidget) {
  fn mapped(self, rsthis: &mut QSignalMapper)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSignalMapper6mappedEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN13QSignalMapper6mappedEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  void QSignalMapper::setMapping(QObject * sender, const QString & text);
impl<'a> /*trait*/ QSignalMapper_setMapping for (&'a mut QObject, &'a  QString) {
  fn setMapping(self, rsthis: &mut QSignalMapper)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSignalMapper10setMappingEP7QObjectRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN13QSignalMapper10setMappingEP7QObjectRK7QString(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

// proto:  QObject * QSignalMapper::mapping(QWidget * widget);
impl<'a> /*trait*/ QSignalMapper_mapping for (&'a mut QWidget) {
  fn mapping(self, rsthis: &mut QSignalMapper) -> QObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QSignalMapper7mappingEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK13QSignalMapper7mappingEP7QWidget(rsthis.qclsinst, arg0)};
    let mut ret1 = QObject{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QSignalMapper::setMapping(QObject * sender, QWidget * widget);
impl<'a> /*trait*/ QSignalMapper_setMapping for (&'a mut QObject, &'a mut QWidget) {
  fn setMapping(self, rsthis: &mut QSignalMapper)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSignalMapper10setMappingEP7QObjectP7QWidget()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN13QSignalMapper10setMappingEP7QObjectP7QWidget(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

