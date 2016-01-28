// auto generated, do not modify.
// created: Thu Jan 28 22:38:45 2016
// src-file: /QtCore/qstringlistmodel.h
// dst-file: /src/core/qstringlistmodel.rs
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
use super::qabstractitemmodel::*; // 773
use std::ops::Deref;
use super::qstringlist::*; // 773
use super::qobject::*; // 773
use super::qvariant::*; // 773
use super::qobjectdefs::*; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QStringListModel_Class_Size() -> c_int;
  // proto:  void QStringListModel::QStringListModel(const QStringList & strings, QObject * parent);
  fn C_ZN16QStringListModelC2ERK11QStringListP7QObject(arg0: *mut c_void, arg1: *mut c_void) -> u64;
  // proto:  bool QStringListModel::insertRows(int row, int count, const QModelIndex & parent);
  fn C_ZN16QStringListModel10insertRowsEiiRK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int, arg2: *mut c_void) -> c_char;
  // proto:  QVariant QStringListModel::data(const QModelIndex & index, int role);
  fn C_ZNK16QStringListModel4dataERK11QModelIndexi(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int) -> *mut c_void;
  // proto:  QStringList QStringListModel::stringList();
  fn C_ZNK16QStringListModel10stringListEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  const QMetaObject * QStringListModel::metaObject();
  fn C_ZNK16QStringListModel10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QStringListModel::removeRows(int row, int count, const QModelIndex & parent);
  fn C_ZN16QStringListModel10removeRowsEiiRK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int, arg2: *mut c_void) -> c_char;
  // proto:  void QStringListModel::QStringListModel(QObject * parent);
  fn C_ZN16QStringListModelC2EP7QObject(arg0: *mut c_void) -> u64;
  // proto:  bool QStringListModel::setData(const QModelIndex & index, const QVariant & value, int role);
  fn C_ZN16QStringListModel7setDataERK11QModelIndexRK8QVarianti(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: c_int) -> c_char;
  // proto:  int QStringListModel::rowCount(const QModelIndex & parent);
  fn C_ZNK16QStringListModel8rowCountERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_int;
  // proto:  QModelIndex QStringListModel::sibling(int row, int column, const QModelIndex & idx);
  fn C_ZNK16QStringListModel7siblingEiiRK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int, arg2: *mut c_void) -> *mut c_void;
  // proto:  void QStringListModel::setStringList(const QStringList & strings);
  fn C_ZN16QStringListModel13setStringListERK11QStringList(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QStringListModel)=1
#[derive(Default)]
pub struct QStringListModel {
  qbase: QAbstractListModel,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QStringListModel {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QStringListModel {
    return QStringListModel{qbase: QAbstractListModel::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QStringListModel {
  type Target = QAbstractListModel;

  fn deref(&self) -> &QAbstractListModel {
    return & self.qbase;
  }
}
impl AsRef<QAbstractListModel> for QStringListModel {
  fn as_ref(& self) -> & QAbstractListModel {
    return & self.qbase;
  }
}
  // proto:  void QStringListModel::QStringListModel(const QStringList & strings, QObject * parent);
impl /*struct*/ QStringListModel {
  pub fn new<T: QStringListModel_new>(value: T) -> QStringListModel {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QStringListModel_new {
  fn new(self) -> QStringListModel;
}

  // proto:  void QStringListModel::QStringListModel(const QStringList & strings, QObject * parent);
impl<'a> /*trait*/ QStringListModel_new for (&'a QStringList, &'a QObject) {
  fn new(self) -> QStringListModel {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QStringListModelC2ERK11QStringListP7QObject()};
    let ctysz: c_int = unsafe{QStringListModel_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN16QStringListModelC2ERK11QStringListP7QObject(arg0, arg1)};
    let rsthis = QStringListModel{qbase: QAbstractListModel::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QStringListModel::insertRows(int row, int count, const QModelIndex & parent);
impl /*struct*/ QStringListModel {
  pub fn insertRows<RetType, T: QStringListModel_insertRows<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.insertRows(self);
    // return 1;
  }
}

pub trait QStringListModel_insertRows<RetType> {
  fn insertRows(self , rsthis: & QStringListModel) -> RetType;
}

  // proto:  bool QStringListModel::insertRows(int row, int count, const QModelIndex & parent);
impl<'a> /*trait*/ QStringListModel_insertRows<i8> for (i32, i32, &'a QModelIndex) {
  fn insertRows(self , rsthis: & QStringListModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QStringListModel10insertRowsEiiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN16QStringListModel10insertRowsEiiRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  QVariant QStringListModel::data(const QModelIndex & index, int role);
impl /*struct*/ QStringListModel {
  pub fn data<RetType, T: QStringListModel_data<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.data(self);
    // return 1;
  }
}

pub trait QStringListModel_data<RetType> {
  fn data(self , rsthis: & QStringListModel) -> RetType;
}

  // proto:  QVariant QStringListModel::data(const QModelIndex & index, int role);
impl<'a> /*trait*/ QStringListModel_data<QVariant> for (&'a QModelIndex, i32) {
  fn data(self , rsthis: & QStringListModel) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QStringListModel4dataERK11QModelIndexi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {C_ZNK16QStringListModel4dataERK11QModelIndexi(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QVariant::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QStringList QStringListModel::stringList();
impl /*struct*/ QStringListModel {
  pub fn stringList<RetType, T: QStringListModel_stringList<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.stringList(self);
    // return 1;
  }
}

pub trait QStringListModel_stringList<RetType> {
  fn stringList(self , rsthis: & QStringListModel) -> RetType;
}

  // proto:  QStringList QStringListModel::stringList();
impl<'a> /*trait*/ QStringListModel_stringList<QStringList> for () {
  fn stringList(self , rsthis: & QStringListModel) -> QStringList {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QStringListModel10stringListEv()};
    let mut ret = unsafe {C_ZNK16QStringListModel10stringListEv(rsthis.qclsinst)};
    let mut ret1 = QStringList::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QMetaObject * QStringListModel::metaObject();
impl /*struct*/ QStringListModel {
  pub fn metaObject<RetType, T: QStringListModel_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QStringListModel_metaObject<RetType> {
  fn metaObject(self , rsthis: & QStringListModel) -> RetType;
}

  // proto:  const QMetaObject * QStringListModel::metaObject();
impl<'a> /*trait*/ QStringListModel_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QStringListModel) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QStringListModel10metaObjectEv()};
    let mut ret = unsafe {C_ZNK16QStringListModel10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QStringListModel::removeRows(int row, int count, const QModelIndex & parent);
impl /*struct*/ QStringListModel {
  pub fn removeRows<RetType, T: QStringListModel_removeRows<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.removeRows(self);
    // return 1;
  }
}

pub trait QStringListModel_removeRows<RetType> {
  fn removeRows(self , rsthis: & QStringListModel) -> RetType;
}

  // proto:  bool QStringListModel::removeRows(int row, int count, const QModelIndex & parent);
impl<'a> /*trait*/ QStringListModel_removeRows<i8> for (i32, i32, &'a QModelIndex) {
  fn removeRows(self , rsthis: & QStringListModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QStringListModel10removeRowsEiiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN16QStringListModel10removeRowsEiiRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QStringListModel::QStringListModel(QObject * parent);
impl<'a> /*trait*/ QStringListModel_new for (&'a QObject) {
  fn new(self) -> QStringListModel {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QStringListModelC2EP7QObject()};
    let ctysz: c_int = unsafe{QStringListModel_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN16QStringListModelC2EP7QObject(arg0)};
    let rsthis = QStringListModel{qbase: QAbstractListModel::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QStringListModel::setData(const QModelIndex & index, const QVariant & value, int role);
impl /*struct*/ QStringListModel {
  pub fn setData<RetType, T: QStringListModel_setData<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setData(self);
    // return 1;
  }
}

pub trait QStringListModel_setData<RetType> {
  fn setData(self , rsthis: & QStringListModel) -> RetType;
}

  // proto:  bool QStringListModel::setData(const QModelIndex & index, const QVariant & value, int role);
impl<'a> /*trait*/ QStringListModel_setData<i8> for (&'a QModelIndex, &'a QVariant, i32) {
  fn setData(self , rsthis: & QStringListModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QStringListModel7setDataERK11QModelIndexRK8QVarianti()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
    let mut ret = unsafe {C_ZN16QStringListModel7setDataERK11QModelIndexRK8QVarianti(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  int QStringListModel::rowCount(const QModelIndex & parent);
impl /*struct*/ QStringListModel {
  pub fn rowCount<RetType, T: QStringListModel_rowCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rowCount(self);
    // return 1;
  }
}

pub trait QStringListModel_rowCount<RetType> {
  fn rowCount(self , rsthis: & QStringListModel) -> RetType;
}

  // proto:  int QStringListModel::rowCount(const QModelIndex & parent);
impl<'a> /*trait*/ QStringListModel_rowCount<i32> for (&'a QModelIndex) {
  fn rowCount(self , rsthis: & QStringListModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QStringListModel8rowCountERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK16QStringListModel8rowCountERK11QModelIndex(rsthis.qclsinst, arg0)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  QModelIndex QStringListModel::sibling(int row, int column, const QModelIndex & idx);
impl /*struct*/ QStringListModel {
  pub fn sibling<RetType, T: QStringListModel_sibling<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sibling(self);
    // return 1;
  }
}

pub trait QStringListModel_sibling<RetType> {
  fn sibling(self , rsthis: & QStringListModel) -> RetType;
}

  // proto:  QModelIndex QStringListModel::sibling(int row, int column, const QModelIndex & idx);
impl<'a> /*trait*/ QStringListModel_sibling<QModelIndex> for (i32, i32, &'a QModelIndex) {
  fn sibling(self , rsthis: & QStringListModel) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK16QStringListModel7siblingEiiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK16QStringListModel7siblingEiiRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QModelIndex::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QStringListModel::setStringList(const QStringList & strings);
impl /*struct*/ QStringListModel {
  pub fn setStringList<RetType, T: QStringListModel_setStringList<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setStringList(self);
    // return 1;
  }
}

pub trait QStringListModel_setStringList<RetType> {
  fn setStringList(self , rsthis: & QStringListModel) -> RetType;
}

  // proto:  void QStringListModel::setStringList(const QStringList & strings);
impl<'a> /*trait*/ QStringListModel_setStringList<()> for (&'a QStringList) {
  fn setStringList(self , rsthis: & QStringListModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN16QStringListModel13setStringListERK11QStringList()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN16QStringListModel13setStringListERK11QStringList(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// <= body block end

