// auto generated, do not modify.
// created: Thu Jan 28 22:38:45 2016
// src-file: /QtCore/qobjectdefs.h
// dst-file: /src/core/qobjectdefs.rs
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
// use super::qobjectdefs::QGenericArgument; // 773
use super::qbytearray::*; // 773
use super::qobject::*; // 773
use super::qmetaobject::*; // 773
// use super::qobjectdefs::QGenericReturnArgument; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QMetaObject__Connection_Class_Size() -> c_int;
  // proto:  void QMetaObject::Connection::Connection();
  fn C_ZN11QMetaObject10ConnectionC2Ev() -> u64;
  // proto:  void QMetaObject::Connection::~Connection();
  fn C_ZN11QMetaObject10ConnectionD2Ev(qthis: u64 /* *mut c_void*/);
  fn QGenericReturnArgument_Class_Size() -> c_int;
  // proto:  void QGenericReturnArgument::QGenericReturnArgument(const char * aName, void * aData);
  fn C_ZN22QGenericReturnArgumentC2EPKcPv(arg0: *mut c_char, arg1: *mut c_void) -> u64;
  fn QMetaObject_Class_Size() -> c_int;
  // proto: static QByteArray QMetaObject::normalizedSignature(const char * method);
  fn C_ZN11QMetaObject19normalizedSignatureEPKc(arg0: *mut c_char) -> *mut c_void;
  // proto: static bool QMetaObject::disconnectOne(const QObject * sender, int signal_index, const QObject * receiver, int method_index);
  fn C_ZN11QMetaObject13disconnectOneEPK7QObjectiS2_i(arg0: *mut c_void, arg1: c_int, arg2: *mut c_void, arg3: c_int) -> c_char;
  // proto:  int QMetaObject::indexOfSlot(const char * slot);
  fn C_ZNK11QMetaObject11indexOfSlotEPKc(qthis: u64 /* *mut c_void*/, arg0: *mut c_char) -> c_int;
  // proto:  int QMetaObject::indexOfConstructor(const char * constructor);
  fn C_ZNK11QMetaObject18indexOfConstructorEPKc(qthis: u64 /* *mut c_void*/, arg0: *mut c_char) -> c_int;
  // proto:  QMetaEnum QMetaObject::enumerator(int index);
  fn C_ZNK11QMetaObject10enumeratorEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  int QMetaObject::indexOfMethod(const char * method);
  fn C_ZNK11QMetaObject13indexOfMethodEPKc(qthis: u64 /* *mut c_void*/, arg0: *mut c_char) -> c_int;
  // proto:  QMetaMethod QMetaObject::constructor(int index);
  fn C_ZNK11QMetaObject11constructorEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto: static bool QMetaObject::checkConnectArgs(const char * signal, const char * method);
  fn C_ZN11QMetaObject16checkConnectArgsEPKcS1_(arg0: *mut c_char, arg1: *mut c_char) -> c_char;
  // proto:  int QMetaObject::enumeratorOffset();
  fn C_ZNK11QMetaObject16enumeratorOffsetEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  QMetaProperty QMetaObject::property(int index);
  fn C_ZNK11QMetaObject8propertyEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto: static void QMetaObject::connectSlotsByName(QObject * o);
  fn C_ZN11QMetaObject18connectSlotsByNameEP7QObject(arg0: *mut c_void);
  // proto:  QMetaProperty QMetaObject::userProperty();
  fn C_ZNK11QMetaObject12userPropertyEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QMetaObject::indexOfProperty(const char * name);
  fn C_ZNK11QMetaObject15indexOfPropertyEPKc(qthis: u64 /* *mut c_void*/, arg0: *mut c_char) -> c_int;
  // proto:  int QMetaObject::indexOfClassInfo(const char * name);
  fn C_ZNK11QMetaObject16indexOfClassInfoEPKc(qthis: u64 /* *mut c_void*/, arg0: *mut c_char) -> c_int;
  // proto: static void QMetaObject::activate(QObject * sender, const QMetaObject * , int local_signal_index, void ** argv);
  fn C_ZN11QMetaObject8activateEP7QObjectPKS_iPPv(arg0: *mut c_void, arg1: *mut c_void, arg2: c_int, arg3: *mut c_void);
  // proto:  const QObject * QMetaObject::cast(const QObject * obj);
  fn C_ZNK11QMetaObject4castEPK7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  QMetaMethod QMetaObject::method(int index);
  fn C_ZNK11QMetaObject6methodEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  const QMetaObject * QMetaObject::superClass();
  fn C_ZNK11QMetaObject10superClassEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QObject * QMetaObject::cast(QObject * obj);
  fn C_ZNK11QMetaObject4castEP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto: static void QMetaObject::activate(QObject * sender, int signal_offset, int local_signal_index, void ** argv);
  fn C_ZN11QMetaObject8activateEP7QObjectiiPPv(arg0: *mut c_void, arg1: c_int, arg2: c_int, arg3: *mut c_void);
  // proto:  int QMetaObject::propertyCount();
  fn C_ZNK11QMetaObject13propertyCountEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  QMetaClassInfo QMetaObject::classInfo(int index);
  fn C_ZNK11QMetaObject9classInfoEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto: static bool QMetaObject::checkConnectArgs(const QMetaMethod & signal, const QMetaMethod & method);
  fn C_ZN11QMetaObject16checkConnectArgsERK11QMetaMethodS2_(arg0: *mut c_void, arg1: *mut c_void) -> c_char;
  // proto:  const char * QMetaObject::className();
  fn C_ZNK11QMetaObject9classNameEv(qthis: u64 /* *mut c_void*/) -> *mut c_char;
  // proto:  int QMetaObject::indexOfSignal(const char * signal);
  fn C_ZNK11QMetaObject13indexOfSignalEPKc(qthis: u64 /* *mut c_void*/, arg0: *mut c_char) -> c_int;
  // proto: static QByteArray QMetaObject::normalizedType(const char * type);
  fn C_ZN11QMetaObject14normalizedTypeEPKc(arg0: *mut c_char) -> *mut c_void;
  // proto:  int QMetaObject::constructorCount();
  fn C_ZNK11QMetaObject16constructorCountEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  int QMetaObject::propertyOffset();
  fn C_ZNK11QMetaObject14propertyOffsetEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto: static bool QMetaObject::disconnect(const QObject * sender, int signal_index, const QObject * receiver, int method_index);
  fn C_ZN11QMetaObject10disconnectEPK7QObjectiS2_i(arg0: *mut c_void, arg1: c_int, arg2: *mut c_void, arg3: c_int) -> c_char;
  // proto: static void QMetaObject::activate(QObject * sender, int signal_index, void ** argv);
  fn C_ZN11QMetaObject8activateEP7QObjectiPPv(arg0: *mut c_void, arg1: c_int, arg2: *mut c_void);
  // proto:  int QMetaObject::enumeratorCount();
  fn C_ZNK11QMetaObject15enumeratorCountEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  int QMetaObject::classInfoOffset();
  fn C_ZNK11QMetaObject15classInfoOffsetEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  int QMetaObject::methodOffset();
  fn C_ZNK11QMetaObject12methodOffsetEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  int QMetaObject::indexOfEnumerator(const char * name);
  fn C_ZNK11QMetaObject17indexOfEnumeratorEPKc(qthis: u64 /* *mut c_void*/, arg0: *mut c_char) -> c_int;
  // proto:  int QMetaObject::methodCount();
  fn C_ZNK11QMetaObject11methodCountEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  int QMetaObject::classInfoCount();
  fn C_ZNK11QMetaObject14classInfoCountEv(qthis: u64 /* *mut c_void*/) -> c_int;
  fn QGenericArgument_Class_Size() -> c_int;
  // proto:  const char * QGenericArgument::name();
  fn C_ZNK16QGenericArgument4nameEv(qthis: u64 /* *mut c_void*/) -> *mut c_char;
  // proto:  void * QGenericArgument::data();
  fn C_ZNK16QGenericArgument4dataEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QGenericArgument::QGenericArgument(const char * aName, const void * aData);
  fn C_ZN16QGenericArgumentC2EPKcPKv(arg0: *mut c_char, arg1: *mut c_void) -> u64;
} // <= ext block end

// body block begin =>
// class sizeof(QMetaObject__Connection)=8
#[derive(Default)]
pub struct QMetaObject__Connection {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QGenericReturnArgument)=16
#[derive(Default)]
pub struct QGenericReturnArgument {
  qbase: QGenericArgument,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QMetaObject)=48
#[derive(Default)]
pub struct QMetaObject {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QGenericArgument)=16
#[derive(Default)]
pub struct QGenericArgument {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QMetaObject__Connection {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QMetaObject__Connection {
    return QMetaObject__Connection{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QMetaObject::Connection::Connection();
impl /*struct*/ QMetaObject__Connection {
  pub fn new<T: QMetaObject__Connection_new>(value: T) -> QMetaObject__Connection {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QMetaObject__Connection_new {
  fn new(self) -> QMetaObject__Connection;
}

  // proto:  void QMetaObject::Connection::Connection();
impl<'a> /*trait*/ QMetaObject__Connection_new for () {
  fn new(self) -> QMetaObject__Connection {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMetaObject10ConnectionC2Ev()};
    let ctysz: c_int = unsafe{QMetaObject__Connection_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN11QMetaObject10ConnectionC2Ev()};
    let rsthis = QMetaObject__Connection{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QMetaObject::Connection::~Connection();
impl /*struct*/ QMetaObject__Connection {
  pub fn free<RetType, T: QMetaObject__Connection_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QMetaObject__Connection_free<RetType> {
  fn free(self , rsthis: & QMetaObject__Connection) -> RetType;
}

  // proto:  void QMetaObject::Connection::~Connection();
impl<'a> /*trait*/ QMetaObject__Connection_free<()> for () {
  fn free(self , rsthis: & QMetaObject__Connection) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QMetaObject10ConnectionD2Ev()};
     unsafe {C_ZN11QMetaObject10ConnectionD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGenericReturnArgument {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QGenericReturnArgument {
    return QGenericReturnArgument{qbase: QGenericArgument::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QGenericReturnArgument {
  type Target = QGenericArgument;

  fn deref(&self) -> &QGenericArgument {
    return & self.qbase;
  }
}
impl AsRef<QGenericArgument> for QGenericReturnArgument {
  fn as_ref(& self) -> & QGenericArgument {
    return & self.qbase;
  }
}
  // proto:  void QGenericReturnArgument::QGenericReturnArgument(const char * aName, void * aData);
impl /*struct*/ QGenericReturnArgument {
  pub fn new<T: QGenericReturnArgument_new>(value: T) -> QGenericReturnArgument {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QGenericReturnArgument_new {
  fn new(self) -> QGenericReturnArgument;
}

  // proto:  void QGenericReturnArgument::QGenericReturnArgument(const char * aName, void * aData);
impl<'a> /*trait*/ QGenericReturnArgument_new for (&'a  String, *mut c_void) {
  fn new(self) -> QGenericReturnArgument {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QGenericReturnArgumentC2EPKcPv()};
    let ctysz: c_int = unsafe{QGenericReturnArgument_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN22QGenericReturnArgumentC2EPKcPv(arg0, arg1)};
    let rsthis = QGenericReturnArgument{qbase: QGenericArgument::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QMetaObject {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QMetaObject {
    return QMetaObject{qclsinst: qthis, ..Default::default()};
  }
}
  // proto: static QByteArray QMetaObject::normalizedSignature(const char * method);
impl /*struct*/ QMetaObject {
  pub fn normalizedSignature_s<RetType, T: QMetaObject_normalizedSignature_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.normalizedSignature_s();
    // return 1;
  }
}

pub trait QMetaObject_normalizedSignature_s<RetType> {
  fn normalizedSignature_s(self ) -> RetType;
}

  // proto: static QByteArray QMetaObject::normalizedSignature(const char * method);
impl<'a> /*trait*/ QMetaObject_normalizedSignature_s<QByteArray> for (&'a  String) {
  fn normalizedSignature_s(self ) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN11QMetaObject19normalizedSignatureEPKc()};
    let arg0 = self.as_ptr()  as *mut c_char;
    let mut ret = unsafe {C_ZN11QMetaObject19normalizedSignatureEPKc(arg0)};
    let mut ret1 = QByteArray::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static bool QMetaObject::disconnectOne(const QObject * sender, int signal_index, const QObject * receiver, int method_index);
impl /*struct*/ QMetaObject {
  pub fn disconnectOne_s<RetType, T: QMetaObject_disconnectOne_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.disconnectOne_s();
    // return 1;
  }
}

pub trait QMetaObject_disconnectOne_s<RetType> {
  fn disconnectOne_s(self ) -> RetType;
}

  // proto: static bool QMetaObject::disconnectOne(const QObject * sender, int signal_index, const QObject * receiver, int method_index);
impl<'a> /*trait*/ QMetaObject_disconnectOne_s<i8> for (&'a QObject, i32, &'a QObject, i32) {
  fn disconnectOne_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN11QMetaObject13disconnectOneEPK7QObjectiS2_i()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3  as c_int;
    let mut ret = unsafe {C_ZN11QMetaObject13disconnectOneEPK7QObjectiS2_i(arg0, arg1, arg2, arg3)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  int QMetaObject::indexOfSlot(const char * slot);
impl /*struct*/ QMetaObject {
  pub fn indexOfSlot<RetType, T: QMetaObject_indexOfSlot<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.indexOfSlot(self);
    // return 1;
  }
}

pub trait QMetaObject_indexOfSlot<RetType> {
  fn indexOfSlot(self , rsthis: & QMetaObject) -> RetType;
}

  // proto:  int QMetaObject::indexOfSlot(const char * slot);
impl<'a> /*trait*/ QMetaObject_indexOfSlot<i32> for (&'a  String) {
  fn indexOfSlot(self , rsthis: & QMetaObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK11QMetaObject11indexOfSlotEPKc()};
    let arg0 = self.as_ptr()  as *mut c_char;
    let mut ret = unsafe {C_ZNK11QMetaObject11indexOfSlotEPKc(rsthis.qclsinst, arg0)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  int QMetaObject::indexOfConstructor(const char * constructor);
impl /*struct*/ QMetaObject {
  pub fn indexOfConstructor<RetType, T: QMetaObject_indexOfConstructor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.indexOfConstructor(self);
    // return 1;
  }
}

pub trait QMetaObject_indexOfConstructor<RetType> {
  fn indexOfConstructor(self , rsthis: & QMetaObject) -> RetType;
}

  // proto:  int QMetaObject::indexOfConstructor(const char * constructor);
impl<'a> /*trait*/ QMetaObject_indexOfConstructor<i32> for (&'a  String) {
  fn indexOfConstructor(self , rsthis: & QMetaObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK11QMetaObject18indexOfConstructorEPKc()};
    let arg0 = self.as_ptr()  as *mut c_char;
    let mut ret = unsafe {C_ZNK11QMetaObject18indexOfConstructorEPKc(rsthis.qclsinst, arg0)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  QMetaEnum QMetaObject::enumerator(int index);
impl /*struct*/ QMetaObject {
  pub fn enumerator<RetType, T: QMetaObject_enumerator<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.enumerator(self);
    // return 1;
  }
}

pub trait QMetaObject_enumerator<RetType> {
  fn enumerator(self , rsthis: & QMetaObject) -> RetType;
}

  // proto:  QMetaEnum QMetaObject::enumerator(int index);
impl<'a> /*trait*/ QMetaObject_enumerator<QMetaEnum> for (i32) {
  fn enumerator(self , rsthis: & QMetaObject) -> QMetaEnum {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK11QMetaObject10enumeratorEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZNK11QMetaObject10enumeratorEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QMetaEnum::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QMetaObject::indexOfMethod(const char * method);
impl /*struct*/ QMetaObject {
  pub fn indexOfMethod<RetType, T: QMetaObject_indexOfMethod<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.indexOfMethod(self);
    // return 1;
  }
}

pub trait QMetaObject_indexOfMethod<RetType> {
  fn indexOfMethod(self , rsthis: & QMetaObject) -> RetType;
}

  // proto:  int QMetaObject::indexOfMethod(const char * method);
impl<'a> /*trait*/ QMetaObject_indexOfMethod<i32> for (&'a  String) {
  fn indexOfMethod(self , rsthis: & QMetaObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK11QMetaObject13indexOfMethodEPKc()};
    let arg0 = self.as_ptr()  as *mut c_char;
    let mut ret = unsafe {C_ZNK11QMetaObject13indexOfMethodEPKc(rsthis.qclsinst, arg0)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  QMetaMethod QMetaObject::constructor(int index);
impl /*struct*/ QMetaObject {
  pub fn constructor<RetType, T: QMetaObject_constructor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.constructor(self);
    // return 1;
  }
}

pub trait QMetaObject_constructor<RetType> {
  fn constructor(self , rsthis: & QMetaObject) -> RetType;
}

  // proto:  QMetaMethod QMetaObject::constructor(int index);
impl<'a> /*trait*/ QMetaObject_constructor<QMetaMethod> for (i32) {
  fn constructor(self , rsthis: & QMetaObject) -> QMetaMethod {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK11QMetaObject11constructorEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZNK11QMetaObject11constructorEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QMetaMethod::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static bool QMetaObject::checkConnectArgs(const char * signal, const char * method);
impl /*struct*/ QMetaObject {
  pub fn checkConnectArgs_s<RetType, T: QMetaObject_checkConnectArgs_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.checkConnectArgs_s();
    // return 1;
  }
}

pub trait QMetaObject_checkConnectArgs_s<RetType> {
  fn checkConnectArgs_s(self ) -> RetType;
}

  // proto: static bool QMetaObject::checkConnectArgs(const char * signal, const char * method);
impl<'a> /*trait*/ QMetaObject_checkConnectArgs_s<i8> for (&'a  String, &'a  String) {
  fn checkConnectArgs_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN11QMetaObject16checkConnectArgsEPKcS1_()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1.as_ptr()  as *mut c_char;
    let mut ret = unsafe {C_ZN11QMetaObject16checkConnectArgsEPKcS1_(arg0, arg1)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  int QMetaObject::enumeratorOffset();
impl /*struct*/ QMetaObject {
  pub fn enumeratorOffset<RetType, T: QMetaObject_enumeratorOffset<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.enumeratorOffset(self);
    // return 1;
  }
}

pub trait QMetaObject_enumeratorOffset<RetType> {
  fn enumeratorOffset(self , rsthis: & QMetaObject) -> RetType;
}

  // proto:  int QMetaObject::enumeratorOffset();
impl<'a> /*trait*/ QMetaObject_enumeratorOffset<i32> for () {
  fn enumeratorOffset(self , rsthis: & QMetaObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK11QMetaObject16enumeratorOffsetEv()};
    let mut ret = unsafe {C_ZNK11QMetaObject16enumeratorOffsetEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  QMetaProperty QMetaObject::property(int index);
impl /*struct*/ QMetaObject {
  pub fn property<RetType, T: QMetaObject_property<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.property(self);
    // return 1;
  }
}

pub trait QMetaObject_property<RetType> {
  fn property(self , rsthis: & QMetaObject) -> RetType;
}

  // proto:  QMetaProperty QMetaObject::property(int index);
impl<'a> /*trait*/ QMetaObject_property<QMetaProperty> for (i32) {
  fn property(self , rsthis: & QMetaObject) -> QMetaProperty {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK11QMetaObject8propertyEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZNK11QMetaObject8propertyEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QMetaProperty::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static void QMetaObject::connectSlotsByName(QObject * o);
impl /*struct*/ QMetaObject {
  pub fn connectSlotsByName_s<RetType, T: QMetaObject_connectSlotsByName_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.connectSlotsByName_s();
    // return 1;
  }
}

pub trait QMetaObject_connectSlotsByName_s<RetType> {
  fn connectSlotsByName_s(self ) -> RetType;
}

  // proto: static void QMetaObject::connectSlotsByName(QObject * o);
impl<'a> /*trait*/ QMetaObject_connectSlotsByName_s<()> for (&'a QObject) {
  fn connectSlotsByName_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN11QMetaObject18connectSlotsByNameEP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN11QMetaObject18connectSlotsByNameEP7QObject(arg0)};
    // return 1;
  }
}

  // proto:  QMetaProperty QMetaObject::userProperty();
impl /*struct*/ QMetaObject {
  pub fn userProperty<RetType, T: QMetaObject_userProperty<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.userProperty(self);
    // return 1;
  }
}

pub trait QMetaObject_userProperty<RetType> {
  fn userProperty(self , rsthis: & QMetaObject) -> RetType;
}

  // proto:  QMetaProperty QMetaObject::userProperty();
impl<'a> /*trait*/ QMetaObject_userProperty<QMetaProperty> for () {
  fn userProperty(self , rsthis: & QMetaObject) -> QMetaProperty {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK11QMetaObject12userPropertyEv()};
    let mut ret = unsafe {C_ZNK11QMetaObject12userPropertyEv(rsthis.qclsinst)};
    let mut ret1 = QMetaProperty::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QMetaObject::indexOfProperty(const char * name);
impl /*struct*/ QMetaObject {
  pub fn indexOfProperty<RetType, T: QMetaObject_indexOfProperty<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.indexOfProperty(self);
    // return 1;
  }
}

pub trait QMetaObject_indexOfProperty<RetType> {
  fn indexOfProperty(self , rsthis: & QMetaObject) -> RetType;
}

  // proto:  int QMetaObject::indexOfProperty(const char * name);
impl<'a> /*trait*/ QMetaObject_indexOfProperty<i32> for (&'a  String) {
  fn indexOfProperty(self , rsthis: & QMetaObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK11QMetaObject15indexOfPropertyEPKc()};
    let arg0 = self.as_ptr()  as *mut c_char;
    let mut ret = unsafe {C_ZNK11QMetaObject15indexOfPropertyEPKc(rsthis.qclsinst, arg0)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  int QMetaObject::indexOfClassInfo(const char * name);
impl /*struct*/ QMetaObject {
  pub fn indexOfClassInfo<RetType, T: QMetaObject_indexOfClassInfo<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.indexOfClassInfo(self);
    // return 1;
  }
}

pub trait QMetaObject_indexOfClassInfo<RetType> {
  fn indexOfClassInfo(self , rsthis: & QMetaObject) -> RetType;
}

  // proto:  int QMetaObject::indexOfClassInfo(const char * name);
impl<'a> /*trait*/ QMetaObject_indexOfClassInfo<i32> for (&'a  String) {
  fn indexOfClassInfo(self , rsthis: & QMetaObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK11QMetaObject16indexOfClassInfoEPKc()};
    let arg0 = self.as_ptr()  as *mut c_char;
    let mut ret = unsafe {C_ZNK11QMetaObject16indexOfClassInfoEPKc(rsthis.qclsinst, arg0)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto: static void QMetaObject::activate(QObject * sender, const QMetaObject * , int local_signal_index, void ** argv);
impl /*struct*/ QMetaObject {
  pub fn activate_s<RetType, T: QMetaObject_activate_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.activate_s();
    // return 1;
  }
}

pub trait QMetaObject_activate_s<RetType> {
  fn activate_s(self ) -> RetType;
}

  // proto: static void QMetaObject::activate(QObject * sender, const QMetaObject * , int local_signal_index, void ** argv);
impl<'a> /*trait*/ QMetaObject_activate_s<()> for (&'a QObject, &'a QMetaObject, i32, *mut c_void) {
  fn activate_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN11QMetaObject8activateEP7QObjectPKS_iPPv()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as *mut c_void;
     unsafe {C_ZN11QMetaObject8activateEP7QObjectPKS_iPPv(arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  const QObject * QMetaObject::cast(const QObject * obj);
impl /*struct*/ QMetaObject {
  pub fn cast<RetType, T: QMetaObject_cast<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.cast(self);
    // return 1;
  }
}

pub trait QMetaObject_cast<RetType> {
  fn cast(self , rsthis: & QMetaObject) -> RetType;
}

  // proto:  const QObject * QMetaObject::cast(const QObject * obj);
impl<'a> /*trait*/ QMetaObject_cast<QObject> for (&'a QObject) {
  fn cast(self , rsthis: & QMetaObject) -> QObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK11QMetaObject4castEPK7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK11QMetaObject4castEPK7QObject(rsthis.qclsinst, arg0)};
    let mut ret1 = QObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QMetaMethod QMetaObject::method(int index);
impl /*struct*/ QMetaObject {
  pub fn method<RetType, T: QMetaObject_method<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.method(self);
    // return 1;
  }
}

pub trait QMetaObject_method<RetType> {
  fn method(self , rsthis: & QMetaObject) -> RetType;
}

  // proto:  QMetaMethod QMetaObject::method(int index);
impl<'a> /*trait*/ QMetaObject_method<QMetaMethod> for (i32) {
  fn method(self , rsthis: & QMetaObject) -> QMetaMethod {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK11QMetaObject6methodEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZNK11QMetaObject6methodEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QMetaMethod::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QMetaObject * QMetaObject::superClass();
impl /*struct*/ QMetaObject {
  pub fn superClass<RetType, T: QMetaObject_superClass<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.superClass(self);
    // return 1;
  }
}

pub trait QMetaObject_superClass<RetType> {
  fn superClass(self , rsthis: & QMetaObject) -> RetType;
}

  // proto:  const QMetaObject * QMetaObject::superClass();
impl<'a> /*trait*/ QMetaObject_superClass<QMetaObject> for () {
  fn superClass(self , rsthis: & QMetaObject) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK11QMetaObject10superClassEv()};
    let mut ret = unsafe {C_ZNK11QMetaObject10superClassEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static void QMetaObject::activate(QObject * sender, int signal_offset, int local_signal_index, void ** argv);
impl<'a> /*trait*/ QMetaObject_activate_s<()> for (&'a QObject, i32, i32, *mut c_void) {
  fn activate_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN11QMetaObject8activateEP7QObjectiiPPv()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as *mut c_void;
     unsafe {C_ZN11QMetaObject8activateEP7QObjectiiPPv(arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  int QMetaObject::propertyCount();
impl /*struct*/ QMetaObject {
  pub fn propertyCount<RetType, T: QMetaObject_propertyCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.propertyCount(self);
    // return 1;
  }
}

pub trait QMetaObject_propertyCount<RetType> {
  fn propertyCount(self , rsthis: & QMetaObject) -> RetType;
}

  // proto:  int QMetaObject::propertyCount();
impl<'a> /*trait*/ QMetaObject_propertyCount<i32> for () {
  fn propertyCount(self , rsthis: & QMetaObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK11QMetaObject13propertyCountEv()};
    let mut ret = unsafe {C_ZNK11QMetaObject13propertyCountEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  QMetaClassInfo QMetaObject::classInfo(int index);
impl /*struct*/ QMetaObject {
  pub fn classInfo<RetType, T: QMetaObject_classInfo<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.classInfo(self);
    // return 1;
  }
}

pub trait QMetaObject_classInfo<RetType> {
  fn classInfo(self , rsthis: & QMetaObject) -> RetType;
}

  // proto:  QMetaClassInfo QMetaObject::classInfo(int index);
impl<'a> /*trait*/ QMetaObject_classInfo<QMetaClassInfo> for (i32) {
  fn classInfo(self , rsthis: & QMetaObject) -> QMetaClassInfo {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK11QMetaObject9classInfoEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZNK11QMetaObject9classInfoEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QMetaClassInfo::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static bool QMetaObject::checkConnectArgs(const QMetaMethod & signal, const QMetaMethod & method);
impl<'a> /*trait*/ QMetaObject_checkConnectArgs_s<i8> for (&'a QMetaMethod, &'a QMetaMethod) {
  fn checkConnectArgs_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN11QMetaObject16checkConnectArgsERK11QMetaMethodS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN11QMetaObject16checkConnectArgsERK11QMetaMethodS2_(arg0, arg1)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  const char * QMetaObject::className();
impl /*struct*/ QMetaObject {
  pub fn className<RetType, T: QMetaObject_className<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.className(self);
    // return 1;
  }
}

pub trait QMetaObject_className<RetType> {
  fn className(self , rsthis: & QMetaObject) -> RetType;
}

  // proto:  const char * QMetaObject::className();
impl<'a> /*trait*/ QMetaObject_className<String> for () {
  fn className(self , rsthis: & QMetaObject) -> String {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK11QMetaObject9classNameEv()};
    let mut ret = unsafe {C_ZNK11QMetaObject9classNameEv(rsthis.qclsinst)};
    let slen = unsafe {strlen(ret as *const i8)} as usize;
    return unsafe{String::from_raw_parts(ret as *mut u8, slen, slen+1)};
    // return 1;
  }
}

  // proto:  int QMetaObject::indexOfSignal(const char * signal);
impl /*struct*/ QMetaObject {
  pub fn indexOfSignal<RetType, T: QMetaObject_indexOfSignal<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.indexOfSignal(self);
    // return 1;
  }
}

pub trait QMetaObject_indexOfSignal<RetType> {
  fn indexOfSignal(self , rsthis: & QMetaObject) -> RetType;
}

  // proto:  int QMetaObject::indexOfSignal(const char * signal);
impl<'a> /*trait*/ QMetaObject_indexOfSignal<i32> for (&'a  String) {
  fn indexOfSignal(self , rsthis: & QMetaObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK11QMetaObject13indexOfSignalEPKc()};
    let arg0 = self.as_ptr()  as *mut c_char;
    let mut ret = unsafe {C_ZNK11QMetaObject13indexOfSignalEPKc(rsthis.qclsinst, arg0)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto: static QByteArray QMetaObject::normalizedType(const char * type);
impl /*struct*/ QMetaObject {
  pub fn normalizedType_s<RetType, T: QMetaObject_normalizedType_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.normalizedType_s();
    // return 1;
  }
}

pub trait QMetaObject_normalizedType_s<RetType> {
  fn normalizedType_s(self ) -> RetType;
}

  // proto: static QByteArray QMetaObject::normalizedType(const char * type);
impl<'a> /*trait*/ QMetaObject_normalizedType_s<QByteArray> for (&'a  String) {
  fn normalizedType_s(self ) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN11QMetaObject14normalizedTypeEPKc()};
    let arg0 = self.as_ptr()  as *mut c_char;
    let mut ret = unsafe {C_ZN11QMetaObject14normalizedTypeEPKc(arg0)};
    let mut ret1 = QByteArray::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QMetaObject::constructorCount();
impl /*struct*/ QMetaObject {
  pub fn constructorCount<RetType, T: QMetaObject_constructorCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.constructorCount(self);
    // return 1;
  }
}

pub trait QMetaObject_constructorCount<RetType> {
  fn constructorCount(self , rsthis: & QMetaObject) -> RetType;
}

  // proto:  int QMetaObject::constructorCount();
impl<'a> /*trait*/ QMetaObject_constructorCount<i32> for () {
  fn constructorCount(self , rsthis: & QMetaObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK11QMetaObject16constructorCountEv()};
    let mut ret = unsafe {C_ZNK11QMetaObject16constructorCountEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  int QMetaObject::propertyOffset();
impl /*struct*/ QMetaObject {
  pub fn propertyOffset<RetType, T: QMetaObject_propertyOffset<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.propertyOffset(self);
    // return 1;
  }
}

pub trait QMetaObject_propertyOffset<RetType> {
  fn propertyOffset(self , rsthis: & QMetaObject) -> RetType;
}

  // proto:  int QMetaObject::propertyOffset();
impl<'a> /*trait*/ QMetaObject_propertyOffset<i32> for () {
  fn propertyOffset(self , rsthis: & QMetaObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK11QMetaObject14propertyOffsetEv()};
    let mut ret = unsafe {C_ZNK11QMetaObject14propertyOffsetEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto: static bool QMetaObject::disconnect(const QObject * sender, int signal_index, const QObject * receiver, int method_index);
impl /*struct*/ QMetaObject {
  pub fn disconnect_s<RetType, T: QMetaObject_disconnect_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.disconnect_s();
    // return 1;
  }
}

pub trait QMetaObject_disconnect_s<RetType> {
  fn disconnect_s(self ) -> RetType;
}

  // proto: static bool QMetaObject::disconnect(const QObject * sender, int signal_index, const QObject * receiver, int method_index);
impl<'a> /*trait*/ QMetaObject_disconnect_s<i8> for (&'a QObject, i32, &'a QObject, i32) {
  fn disconnect_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN11QMetaObject10disconnectEPK7QObjectiS2_i()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3  as c_int;
    let mut ret = unsafe {C_ZN11QMetaObject10disconnectEPK7QObjectiS2_i(arg0, arg1, arg2, arg3)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto: static void QMetaObject::activate(QObject * sender, int signal_index, void ** argv);
impl<'a> /*trait*/ QMetaObject_activate_s<()> for (&'a QObject, i32, *mut c_void) {
  fn activate_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZN11QMetaObject8activateEP7QObjectiPPv()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as *mut c_void;
     unsafe {C_ZN11QMetaObject8activateEP7QObjectiPPv(arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  int QMetaObject::enumeratorCount();
impl /*struct*/ QMetaObject {
  pub fn enumeratorCount<RetType, T: QMetaObject_enumeratorCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.enumeratorCount(self);
    // return 1;
  }
}

pub trait QMetaObject_enumeratorCount<RetType> {
  fn enumeratorCount(self , rsthis: & QMetaObject) -> RetType;
}

  // proto:  int QMetaObject::enumeratorCount();
impl<'a> /*trait*/ QMetaObject_enumeratorCount<i32> for () {
  fn enumeratorCount(self , rsthis: & QMetaObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK11QMetaObject15enumeratorCountEv()};
    let mut ret = unsafe {C_ZNK11QMetaObject15enumeratorCountEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  int QMetaObject::classInfoOffset();
impl /*struct*/ QMetaObject {
  pub fn classInfoOffset<RetType, T: QMetaObject_classInfoOffset<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.classInfoOffset(self);
    // return 1;
  }
}

pub trait QMetaObject_classInfoOffset<RetType> {
  fn classInfoOffset(self , rsthis: & QMetaObject) -> RetType;
}

  // proto:  int QMetaObject::classInfoOffset();
impl<'a> /*trait*/ QMetaObject_classInfoOffset<i32> for () {
  fn classInfoOffset(self , rsthis: & QMetaObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK11QMetaObject15classInfoOffsetEv()};
    let mut ret = unsafe {C_ZNK11QMetaObject15classInfoOffsetEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  int QMetaObject::methodOffset();
impl /*struct*/ QMetaObject {
  pub fn methodOffset<RetType, T: QMetaObject_methodOffset<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.methodOffset(self);
    // return 1;
  }
}

pub trait QMetaObject_methodOffset<RetType> {
  fn methodOffset(self , rsthis: & QMetaObject) -> RetType;
}

  // proto:  int QMetaObject::methodOffset();
impl<'a> /*trait*/ QMetaObject_methodOffset<i32> for () {
  fn methodOffset(self , rsthis: & QMetaObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK11QMetaObject12methodOffsetEv()};
    let mut ret = unsafe {C_ZNK11QMetaObject12methodOffsetEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  int QMetaObject::indexOfEnumerator(const char * name);
impl /*struct*/ QMetaObject {
  pub fn indexOfEnumerator<RetType, T: QMetaObject_indexOfEnumerator<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.indexOfEnumerator(self);
    // return 1;
  }
}

pub trait QMetaObject_indexOfEnumerator<RetType> {
  fn indexOfEnumerator(self , rsthis: & QMetaObject) -> RetType;
}

  // proto:  int QMetaObject::indexOfEnumerator(const char * name);
impl<'a> /*trait*/ QMetaObject_indexOfEnumerator<i32> for (&'a  String) {
  fn indexOfEnumerator(self , rsthis: & QMetaObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK11QMetaObject17indexOfEnumeratorEPKc()};
    let arg0 = self.as_ptr()  as *mut c_char;
    let mut ret = unsafe {C_ZNK11QMetaObject17indexOfEnumeratorEPKc(rsthis.qclsinst, arg0)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  int QMetaObject::methodCount();
impl /*struct*/ QMetaObject {
  pub fn methodCount<RetType, T: QMetaObject_methodCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.methodCount(self);
    // return 1;
  }
}

pub trait QMetaObject_methodCount<RetType> {
  fn methodCount(self , rsthis: & QMetaObject) -> RetType;
}

  // proto:  int QMetaObject::methodCount();
impl<'a> /*trait*/ QMetaObject_methodCount<i32> for () {
  fn methodCount(self , rsthis: & QMetaObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK11QMetaObject11methodCountEv()};
    let mut ret = unsafe {C_ZNK11QMetaObject11methodCountEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  int QMetaObject::classInfoCount();
impl /*struct*/ QMetaObject {
  pub fn classInfoCount<RetType, T: QMetaObject_classInfoCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.classInfoCount(self);
    // return 1;
  }
}

pub trait QMetaObject_classInfoCount<RetType> {
  fn classInfoCount(self , rsthis: & QMetaObject) -> RetType;
}

  // proto:  int QMetaObject::classInfoCount();
impl<'a> /*trait*/ QMetaObject_classInfoCount<i32> for () {
  fn classInfoCount(self , rsthis: & QMetaObject) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 48)};
    // unsafe{_ZNK11QMetaObject14classInfoCountEv()};
    let mut ret = unsafe {C_ZNK11QMetaObject14classInfoCountEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

impl /*struct*/ QGenericArgument {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QGenericArgument {
    return QGenericArgument{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  const char * QGenericArgument::name();
impl /*struct*/ QGenericArgument {
  pub fn name<RetType, T: QGenericArgument_name<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.name(self);
    // return 1;
  }
}

pub trait QGenericArgument_name<RetType> {
  fn name(self , rsthis: & QGenericArgument) -> RetType;
}

  // proto:  const char * QGenericArgument::name();
impl<'a> /*trait*/ QGenericArgument_name<String> for () {
  fn name(self , rsthis: & QGenericArgument) -> String {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QGenericArgument4nameEv()};
    let mut ret = unsafe {C_ZNK16QGenericArgument4nameEv(rsthis.qclsinst)};
    let slen = unsafe {strlen(ret as *const i8)} as usize;
    return unsafe{String::from_raw_parts(ret as *mut u8, slen, slen+1)};
    // return 1;
  }
}

  // proto:  void * QGenericArgument::data();
impl /*struct*/ QGenericArgument {
  pub fn data<RetType, T: QGenericArgument_data<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.data(self);
    // return 1;
  }
}

pub trait QGenericArgument_data<RetType> {
  fn data(self , rsthis: & QGenericArgument) -> RetType;
}

  // proto:  void * QGenericArgument::data();
impl<'a> /*trait*/ QGenericArgument_data<*mut c_void> for () {
  fn data(self , rsthis: & QGenericArgument) -> *mut c_void {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QGenericArgument4dataEv()};
    let mut ret = unsafe {C_ZNK16QGenericArgument4dataEv(rsthis.qclsinst)};
    return ret as *mut c_void; // 1
    // return 1;
  }
}

  // proto:  void QGenericArgument::QGenericArgument(const char * aName, const void * aData);
impl /*struct*/ QGenericArgument {
  pub fn new<T: QGenericArgument_new>(value: T) -> QGenericArgument {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QGenericArgument_new {
  fn new(self) -> QGenericArgument;
}

  // proto:  void QGenericArgument::QGenericArgument(const char * aName, const void * aData);
impl<'a> /*trait*/ QGenericArgument_new for (&'a  String, *mut c_void) {
  fn new(self) -> QGenericArgument {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QGenericArgumentC2EPKcPKv()};
    let ctysz: c_int = unsafe{QGenericArgument_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN16QGenericArgumentC2EPKcPKv(arg0, arg1)};
    let rsthis = QGenericArgument{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

// <= body block end

