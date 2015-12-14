// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qdatastream::QDataStream;
use super::qstring::QString;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  bool QIconEngine::read(QDataStream & in);
  fn _ZN11QIconEngine4readER11QDataStream(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  QString QIconEngine::iconName();
  fn _ZNK11QIconEngine8iconNameEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QIconEngine::write(QDataStream & out);
  fn _ZNK11QIconEngine5writeER11QDataStream(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  void QIconEngine::virtual_hook(int id, void * data);
  fn _ZN11QIconEngine12virtual_hookEiPv(qthis: *mut c_void, arg0: c_int, arg1: *mut uint8_t) ;
  // proto:  QIconEngine * QIconEngine::clone();
  fn _ZNK11QIconEngine5cloneEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString QIconEngine::key();
  fn _ZNK11QIconEngine3keyEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QIconEngine::FreeQIconEngine();
  fn _ZN11QIconEngineD0Ev(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QIconEngine)=8
pub struct QIconEngine {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QIconEngine {
  pub fn read<T: QIconEngine_read>(&mut self, value: T) -> i8 {
    return value.read(self);
    // return 1;
  }
}

pub trait QIconEngine_read {
  fn read(self, rsthis: &mut QIconEngine) -> i8;
}

// proto:  bool QIconEngine::read(QDataStream & in);
impl<'a> /*trait*/ QIconEngine_read for (&'a mut QDataStream) {
  fn read(self, rsthis: &mut QIconEngine) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QIconEngine4readER11QDataStream()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN11QIconEngine4readER11QDataStream(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QIconEngine {
  pub fn iconName<T: QIconEngine_iconName>(&mut self, value: T) -> QString {
    return value.iconName(self);
    // return 1;
  }
}

pub trait QIconEngine_iconName {
  fn iconName(self, rsthis: &mut QIconEngine) -> QString;
}

// proto:  QString QIconEngine::iconName();
impl<'a> /*trait*/ QIconEngine_iconName for () {
  fn iconName(self, rsthis: &mut QIconEngine) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QIconEngine8iconNameEv()};
    let mut ret = unsafe {_ZNK11QIconEngine8iconNameEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QIconEngine {
  pub fn write<T: QIconEngine_write>(&mut self, value: T) -> i8 {
    return value.write(self);
    // return 1;
  }
}

pub trait QIconEngine_write {
  fn write(self, rsthis: &mut QIconEngine) -> i8;
}

// proto:  bool QIconEngine::write(QDataStream & out);
impl<'a> /*trait*/ QIconEngine_write for (&'a mut QDataStream) {
  fn write(self, rsthis: &mut QIconEngine) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QIconEngine5writeER11QDataStream()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK11QIconEngine5writeER11QDataStream(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QIconEngine {
  pub fn virtual_hook<T: QIconEngine_virtual_hook>(&mut self, value: T)  {
     value.virtual_hook(self);
    // return 1;
  }
}

pub trait QIconEngine_virtual_hook {
  fn virtual_hook(self, rsthis: &mut QIconEngine) ;
}

// proto:  void QIconEngine::virtual_hook(int id, void * data);
impl<'a> /*trait*/ QIconEngine_virtual_hook for (i32, &'a mut u8) {
  fn virtual_hook(self, rsthis: &mut QIconEngine)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QIconEngine12virtual_hookEiPv()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as *mut uint8_t;
     unsafe {_ZN11QIconEngine12virtual_hookEiPv(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QIconEngine {
  pub fn clone<T: QIconEngine_clone>(&mut self, value: T) -> QIconEngine {
    return value.clone(self);
    // return 1;
  }
}

pub trait QIconEngine_clone {
  fn clone(self, rsthis: &mut QIconEngine) -> QIconEngine;
}

// proto:  QIconEngine * QIconEngine::clone();
impl<'a> /*trait*/ QIconEngine_clone for () {
  fn clone(self, rsthis: &mut QIconEngine) -> QIconEngine {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QIconEngine5cloneEv()};
    let mut ret = unsafe {_ZNK11QIconEngine5cloneEv(rsthis.qclsinst)};
    let mut ret1 = QIconEngine{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QIconEngine {
  pub fn key<T: QIconEngine_key>(&mut self, value: T) -> QString {
    return value.key(self);
    // return 1;
  }
}

pub trait QIconEngine_key {
  fn key(self, rsthis: &mut QIconEngine) -> QString;
}

// proto:  QString QIconEngine::key();
impl<'a> /*trait*/ QIconEngine_key for () {
  fn key(self, rsthis: &mut QIconEngine) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QIconEngine3keyEv()};
    let mut ret = unsafe {_ZNK11QIconEngine3keyEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QIconEngine {
  pub fn FreeQIconEngine<T: QIconEngine_FreeQIconEngine>(&mut self, value: T)  {
     value.FreeQIconEngine(self);
    // return 1;
  }
}

pub trait QIconEngine_FreeQIconEngine {
  fn FreeQIconEngine(self, rsthis: &mut QIconEngine) ;
}

// proto:  void QIconEngine::FreeQIconEngine();
impl<'a> /*trait*/ QIconEngine_FreeQIconEngine for () {
  fn FreeQIconEngine(self, rsthis: &mut QIconEngine)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QIconEngineD0Ev()};
     unsafe {_ZN11QIconEngineD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

