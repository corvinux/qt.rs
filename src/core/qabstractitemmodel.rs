// auto generated, do not modify.
// created: Sun Dec 27 22:52:02 2015
// src-file: /QtCore/qabstractitemmodel.h
// dst-file: /src/core/qabstractitemmodel.rs
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
// use super::qabstractitemmodel::QAbstractItemModel; // 773
use super::qvariant::QVariant; // 773
// use super::qabstractitemmodel::QModelIndex; // 773
use super::qobject::QObject; // 773
use super::qmimedata::QMimeData; // 773
use super::qsize::QSize; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QModelIndex_Class_Size() -> c_int;
  // proto:  void QModelIndex::QModelIndex(int arow, int acolumn, void * ptr, const QAbstractItemModel * amodel);
  fn dector_ZN11QModelIndexC1EiiPvPK18QAbstractItemModel(arg0: c_int, arg1: c_int, arg2: *mut c_void, arg3: *mut c_void) -> *mut c_void;
  fn demth_ZN11QModelIndexC1EiiPvPK18QAbstractItemModel(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int, arg2: *mut c_void, arg3: *mut c_void);
  // proto:  void QModelIndex::QModelIndex();
  fn dector_ZN11QModelIndexC1Ev() -> *mut c_void;
  fn _ZN11QModelIndexC1Ev(qthis: u64 /* *mut c_void*/);
  // proto:  int QModelIndex::column();
  fn _ZNK11QModelIndex6columnEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  quintptr QModelIndex::internalId();
  fn _ZNK11QModelIndex10internalIdEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  QModelIndex QModelIndex::child(int row, int column);
  fn demth_ZNK11QModelIndex5childEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  void * QModelIndex::internalPointer();
  fn demth_ZNK11QModelIndex15internalPointerEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QModelIndex::QModelIndex(int arow, int acolumn, quintptr id, const QAbstractItemModel * amodel);
  fn dector_ZN11QModelIndexC1EiiiPK18QAbstractItemModel(arg0: c_int, arg1: c_int, arg2: c_int, arg3: *mut c_void) -> *mut c_void;
  fn _ZN11QModelIndexC1EiiiPK18QAbstractItemModel(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int, arg2: c_int, arg3: *mut c_void);
  // proto:  bool QModelIndex::isValid();
  fn _ZNK11QModelIndex7isValidEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QModelIndex QModelIndex::parent();
  fn demth_ZNK11QModelIndex6parentEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QModelIndex QModelIndex::sibling(int row, int column);
  fn demth_ZNK11QModelIndex7siblingEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  const QAbstractItemModel * QModelIndex::model();
  fn _ZNK11QModelIndex5modelEv(qthis: u64 /* *mut c_void*/);
  // proto:  QVariant QModelIndex::data(int role);
  fn demth_ZNK11QModelIndex4dataEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  int QModelIndex::row();
  fn _ZNK11QModelIndex3rowEv(qthis: u64 /* *mut c_void*/) -> c_int;
  fn QPersistentModelIndex_Class_Size() -> c_int;
  // proto:  QModelIndex QPersistentModelIndex::sibling(int row, int column);
  fn _ZNK21QPersistentModelIndex7siblingEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  QVariant QPersistentModelIndex::data(int role);
  fn _ZNK21QPersistentModelIndex4dataEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  QModelIndex QPersistentModelIndex::parent();
  fn _ZNK21QPersistentModelIndex6parentEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QPersistentModelIndex::QPersistentModelIndex(const QPersistentModelIndex & other);
  fn dector_ZN21QPersistentModelIndexC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN21QPersistentModelIndexC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void * QPersistentModelIndex::internalPointer();
  fn _ZNK21QPersistentModelIndex15internalPointerEv(qthis: u64 /* *mut c_void*/);
  // proto:  int QPersistentModelIndex::row();
  fn _ZNK21QPersistentModelIndex3rowEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  quintptr QPersistentModelIndex::internalId();
  fn _ZNK21QPersistentModelIndex10internalIdEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  const QAbstractItemModel * QPersistentModelIndex::model();
  fn _ZNK21QPersistentModelIndex5modelEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QPersistentModelIndex::~QPersistentModelIndex();
  fn _ZN21QPersistentModelIndexD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QPersistentModelIndex::QPersistentModelIndex(const QModelIndex & index);
  fn dector_ZN21QPersistentModelIndexC1ERK11QModelIndex(arg0: *mut c_void) -> *mut c_void;
  fn _ZN21QPersistentModelIndexC1ERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QPersistentModelIndex::QPersistentModelIndex();
  fn dector_ZN21QPersistentModelIndexC1Ev() -> *mut c_void;
  fn _ZN21QPersistentModelIndexC1Ev(qthis: u64 /* *mut c_void*/);
  // proto:  int QPersistentModelIndex::column();
  fn _ZNK21QPersistentModelIndex6columnEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QPersistentModelIndex::swap(QPersistentModelIndex & other);
  fn demth_ZN21QPersistentModelIndex4swapERS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QModelIndex QPersistentModelIndex::child(int row, int column);
  fn _ZNK21QPersistentModelIndex5childEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  bool QPersistentModelIndex::isValid();
  fn _ZNK21QPersistentModelIndex7isValidEv(qthis: u64 /* *mut c_void*/) -> c_char;
  fn QAbstractTableModel_Class_Size() -> c_int;
  // proto:  void QAbstractTableModel::QAbstractTableModel(const QAbstractTableModel & );
  fn dector_ZN19QAbstractTableModelC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN19QAbstractTableModelC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QAbstractTableModel::QAbstractTableModel(QObject * parent);
  fn dector_ZN19QAbstractTableModelC1EP7QObject(arg0: *mut c_void) -> *mut c_void;
  fn _ZN19QAbstractTableModelC1EP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QModelIndex QAbstractTableModel::index(int row, int column, const QModelIndex & parent);
  fn _ZNK19QAbstractTableModel5indexEiiRK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int, arg2: *mut c_void) -> *mut c_void;
  // proto:  QModelIndex QAbstractTableModel::sibling(int row, int column, const QModelIndex & idx);
  fn _ZNK19QAbstractTableModel7siblingEiiRK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int, arg2: *mut c_void) -> *mut c_void;
  // proto:  void QAbstractTableModel::~QAbstractTableModel();
  fn _ZN19QAbstractTableModelD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  const QMetaObject * QAbstractTableModel::metaObject();
  fn _ZNK19QAbstractTableModel10metaObjectEv(qthis: u64 /* *mut c_void*/);
  fn QAbstractItemModel_Class_Size() -> c_int;
  // proto:  bool QAbstractItemModel::removeColumns(int column, int count, const QModelIndex & parent);
  fn _ZN18QAbstractItemModel13removeColumnsEiiRK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int, arg2: *mut c_void) -> c_char;
  // proto:  void QAbstractItemModel::~QAbstractItemModel();
  fn _ZN18QAbstractItemModelD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  bool QAbstractItemModel::canFetchMore(const QModelIndex & parent);
  fn _ZNK18QAbstractItemModel12canFetchMoreERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  bool QAbstractItemModel::submit();
  fn _ZN18QAbstractItemModel6submitEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QAbstractItemModel::hasIndex(int row, int column, const QModelIndex & parent);
  fn _ZNK18QAbstractItemModel8hasIndexEiiRK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int, arg2: *mut c_void) -> c_char;
  // proto:  QHash<int, QByteArray> QAbstractItemModel::roleNames();
  fn _ZNK18QAbstractItemModel9roleNamesEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QAbstractItemModel::moveColumn(const QModelIndex & sourceParent, int sourceColumn, const QModelIndex & destinationParent, int destinationChild);
  fn demth_ZN18QAbstractItemModel10moveColumnERK11QModelIndexiS2_i(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int, arg2: *mut c_void, arg3: c_int) -> c_char;
  // proto:  void QAbstractItemModel::fetchMore(const QModelIndex & parent);
  fn _ZN18QAbstractItemModel9fetchMoreERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  bool QAbstractItemModel::insertRows(int row, int count, const QModelIndex & parent);
  fn _ZN18QAbstractItemModel10insertRowsEiiRK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int, arg2: *mut c_void) -> c_char;
  // proto:  QSize QAbstractItemModel::span(const QModelIndex & index);
  fn _ZNK18QAbstractItemModel4spanERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QAbstractItemModel::QAbstractItemModel(QObject * parent);
  fn dector_ZN18QAbstractItemModelC1EP7QObject(arg0: *mut c_void) -> *mut c_void;
  fn _ZN18QAbstractItemModelC1EP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QModelIndex QAbstractItemModel::index(int row, int column, const QModelIndex & parent);
  fn _ZNK18QAbstractItemModel5indexEiiRK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int, arg2: *mut c_void) -> *mut c_void;
  // proto:  bool QAbstractItemModel::insertRow(int row, const QModelIndex & parent);
  fn demth_ZN18QAbstractItemModel9insertRowEiRK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void) -> c_char;
  // proto:  void QAbstractItemModel::QAbstractItemModel(const QAbstractItemModel & );
  fn dector_ZN18QAbstractItemModelC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN18QAbstractItemModelC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  const QMetaObject * QAbstractItemModel::metaObject();
  fn _ZNK18QAbstractItemModel10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QAbstractItemModel::removeRow(int row, const QModelIndex & parent);
  fn demth_ZN18QAbstractItemModel9removeRowEiRK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void) -> c_char;
  // proto:  bool QAbstractItemModel::setData(const QModelIndex & index, const QVariant & value, int role);
  fn _ZN18QAbstractItemModel7setDataERK11QModelIndexRK8QVarianti(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void, arg2: c_int) -> c_char;
  // proto:  int QAbstractItemModel::rowCount(const QModelIndex & parent);
  fn _ZNK18QAbstractItemModel8rowCountERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_int;
  // proto:  bool QAbstractItemModel::removeRows(int row, int count, const QModelIndex & parent);
  fn _ZN18QAbstractItemModel10removeRowsEiiRK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int, arg2: *mut c_void) -> c_char;
  // proto:  bool QAbstractItemModel::hasChildren(const QModelIndex & parent);
  fn _ZNK18QAbstractItemModel11hasChildrenERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  bool QAbstractItemModel::moveRow(const QModelIndex & sourceParent, int sourceRow, const QModelIndex & destinationParent, int destinationChild);
  fn demth_ZN18QAbstractItemModel7moveRowERK11QModelIndexiS2_i(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int, arg2: *mut c_void, arg3: c_int) -> c_char;
  // proto:  void QAbstractItemModel::revert();
  fn _ZN18QAbstractItemModel6revertEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QAbstractItemModel::removeColumn(int column, const QModelIndex & parent);
  fn demth_ZN18QAbstractItemModel12removeColumnEiRK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void) -> c_char;
  // proto:  bool QAbstractItemModel::insertColumns(int column, int count, const QModelIndex & parent);
  fn _ZN18QAbstractItemModel13insertColumnsEiiRK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int, arg2: *mut c_void) -> c_char;
  // proto:  bool QAbstractItemModel::insertColumn(int column, const QModelIndex & parent);
  fn demth_ZN18QAbstractItemModel12insertColumnEiRK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void) -> c_char;
  // proto:  bool QAbstractItemModel::moveColumns(const QModelIndex & sourceParent, int sourceColumn, int count, const QModelIndex & destinationParent, int destinationChild);
  fn _ZN18QAbstractItemModel11moveColumnsERK11QModelIndexiiS2_i(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int, arg2: c_int, arg3: *mut c_void, arg4: c_int) -> c_char;
  // proto:  QMap<int, QVariant> QAbstractItemModel::itemData(const QModelIndex & index);
  fn _ZNK18QAbstractItemModel8itemDataERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QStringList QAbstractItemModel::mimeTypes();
  fn _ZNK18QAbstractItemModel9mimeTypesEv(qthis: u64 /* *mut c_void*/);
  // proto:  QModelIndex QAbstractItemModel::parent(const QModelIndex & child);
  fn _ZNK18QAbstractItemModel6parentERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  QModelIndex QAbstractItemModel::buddy(const QModelIndex & index);
  fn _ZNK18QAbstractItemModel5buddyERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  int QAbstractItemModel::columnCount(const QModelIndex & parent);
  fn _ZNK18QAbstractItemModel11columnCountERK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_int;
  // proto:  QVariant QAbstractItemModel::data(const QModelIndex & index, int role);
  fn _ZNK18QAbstractItemModel4dataERK11QModelIndexi(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int) -> *mut c_void;
  // proto:  QModelIndex QAbstractItemModel::sibling(int row, int column, const QModelIndex & idx);
  fn _ZNK18QAbstractItemModel7siblingEiiRK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int, arg2: *mut c_void) -> *mut c_void;
  // proto:  bool QAbstractItemModel::moveRows(const QModelIndex & sourceParent, int sourceRow, int count, const QModelIndex & destinationParent, int destinationChild);
  fn _ZN18QAbstractItemModel8moveRowsERK11QModelIndexiiS2_i(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int, arg2: c_int, arg3: *mut c_void, arg4: c_int) -> c_char;
  fn QAbstractListModel_Class_Size() -> c_int;
  // proto:  QModelIndex QAbstractListModel::sibling(int row, int column, const QModelIndex & idx);
  fn _ZNK18QAbstractListModel7siblingEiiRK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int, arg2: *mut c_void) -> *mut c_void;
  // proto:  void QAbstractListModel::QAbstractListModel(QObject * parent);
  fn dector_ZN18QAbstractListModelC1EP7QObject(arg0: *mut c_void) -> *mut c_void;
  fn _ZN18QAbstractListModelC1EP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  const QMetaObject * QAbstractListModel::metaObject();
  fn _ZNK18QAbstractListModel10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QAbstractListModel::QAbstractListModel(const QAbstractListModel & );
  fn dector_ZN18QAbstractListModelC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN18QAbstractListModelC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QModelIndex QAbstractListModel::index(int row, int column, const QModelIndex & parent);
  fn _ZNK18QAbstractListModel5indexEiiRK11QModelIndex(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int, arg2: *mut c_void) -> *mut c_void;
  // proto:  void QAbstractListModel::~QAbstractListModel();
  fn _ZN18QAbstractListModelD0Ev(qthis: u64 /* *mut c_void*/);
  fn QAbstractItemModel_SlotProxy_connect__ZN18QAbstractItemModel17headerDataChangedEN2Qt11OrientationEii(qthis: *mut c_void, fptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QModelIndex)=24
#[derive(Default)]
pub struct QModelIndex {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QPersistentModelIndex)=8
#[derive(Default)]
pub struct QPersistentModelIndex {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QAbstractTableModel)=1
#[derive(Default)]
pub struct QAbstractTableModel {
  qbase: QAbstractItemModel,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QAbstractItemModel)=1
#[derive(Default)]
pub struct QAbstractItemModel {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _modelReset_1: QAbstractItemModel_modelReset_signal,
  pub _columnsAboutToBeMoved_1: QAbstractItemModel_columnsAboutToBeMoved_signal,
  pub _columnsInserted_1: QAbstractItemModel_columnsInserted_signal,
  pub _rowsAboutToBeRemoved_1: QAbstractItemModel_rowsAboutToBeRemoved_signal,
  pub _columnsAboutToBeRemoved_1: QAbstractItemModel_columnsAboutToBeRemoved_signal,
  pub _rowsAboutToBeMoved_1: QAbstractItemModel_rowsAboutToBeMoved_signal,
  pub _layoutChanged_1: QAbstractItemModel_layoutChanged_signal,
  pub _columnsRemoved_1: QAbstractItemModel_columnsRemoved_signal,
  pub _rowsInserted_1: QAbstractItemModel_rowsInserted_signal,
  pub _columnsAboutToBeInserted_1: QAbstractItemModel_columnsAboutToBeInserted_signal,
  pub _layoutAboutToBeChanged_1: QAbstractItemModel_layoutAboutToBeChanged_signal,
  pub _rowsRemoved_1: QAbstractItemModel_rowsRemoved_signal,
  pub _rowsMoved_1: QAbstractItemModel_rowsMoved_signal,
  pub _headerDataChanged_1: QAbstractItemModel_headerDataChanged_signal,
  pub _columnsMoved_1: QAbstractItemModel_columnsMoved_signal,
  pub _rowsAboutToBeInserted_1: QAbstractItemModel_rowsAboutToBeInserted_signal,
  pub _modelAboutToBeReset_1: QAbstractItemModel_modelAboutToBeReset_signal,
  pub _dataChanged_1: QAbstractItemModel_dataChanged_signal,
}

// class sizeof(QAbstractListModel)=1
#[derive(Default)]
pub struct QAbstractListModel {
  qbase: QAbstractItemModel,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QModelIndex {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QModelIndex {
    return QModelIndex{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QModelIndex::QModelIndex(int arow, int acolumn, void * ptr, const QAbstractItemModel * amodel);
impl /*struct*/ QModelIndex {
  pub fn New<T: QModelIndex_New>(value: T) -> QModelIndex {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QModelIndex_New {
  fn New(self) -> QModelIndex;
}

  // proto:  void QModelIndex::QModelIndex(int arow, int acolumn, void * ptr, const QAbstractItemModel * amodel);
impl<'a> /*trait*/ QModelIndex_New for (i32, i32, *mut c_void, &'a QAbstractItemModel) {
  fn New(self) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QModelIndexC1EiiPvPK18QAbstractItemModel()};
    let ctysz: c_int = unsafe{QModelIndex_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as *mut c_void;
    let arg3 = self.3.qclsinst  as *mut c_void;
    // unsafe {_ZN11QModelIndexC1EiiPvPK18QAbstractItemModel(qthis, arg0, arg1, arg2, arg3)};
    let qthis: u64 = unsafe {dector_ZN11QModelIndexC1EiiPvPK18QAbstractItemModel(arg0, arg1, arg2, arg3)} as u64;
    let rsthis = QModelIndex{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QModelIndex::QModelIndex();
impl<'a> /*trait*/ QModelIndex_New for () {
  fn New(self) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QModelIndexC1Ev()};
    let ctysz: c_int = unsafe{QModelIndex_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    // unsafe {_ZN11QModelIndexC1Ev(qthis)};
    let qthis: u64 = unsafe {dector_ZN11QModelIndexC1Ev()} as u64;
    let rsthis = QModelIndex{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QModelIndex::column();
impl /*struct*/ QModelIndex {
  pub fn column<RetType, T: QModelIndex_column<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.column(self);
    // return 1;
  }
}

pub trait QModelIndex_column<RetType> {
  fn column(self , rsthis: & QModelIndex) -> RetType;
}

  // proto:  int QModelIndex::column();
impl<'a> /*trait*/ QModelIndex_column<i32> for () {
  fn column(self , rsthis: & QModelIndex) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QModelIndex6columnEv()};
    let mut ret = unsafe {_ZNK11QModelIndex6columnEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  quintptr QModelIndex::internalId();
impl /*struct*/ QModelIndex {
  pub fn internalId<RetType, T: QModelIndex_internalId<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.internalId(self);
    // return 1;
  }
}

pub trait QModelIndex_internalId<RetType> {
  fn internalId(self , rsthis: & QModelIndex) -> RetType;
}

  // proto:  quintptr QModelIndex::internalId();
impl<'a> /*trait*/ QModelIndex_internalId<i32> for () {
  fn internalId(self , rsthis: & QModelIndex) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QModelIndex10internalIdEv()};
    let mut ret = unsafe {_ZNK11QModelIndex10internalIdEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QModelIndex QModelIndex::child(int row, int column);
impl /*struct*/ QModelIndex {
  pub fn child<RetType, T: QModelIndex_child<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.child(self);
    // return 1;
  }
}

pub trait QModelIndex_child<RetType> {
  fn child(self , rsthis: & QModelIndex) -> RetType;
}

  // proto:  QModelIndex QModelIndex::child(int row, int column);
impl<'a> /*trait*/ QModelIndex_child<QModelIndex> for (i32, i32) {
  fn child(self , rsthis: & QModelIndex) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QModelIndex5childEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {demth_ZNK11QModelIndex5childEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QModelIndex::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void * QModelIndex::internalPointer();
impl /*struct*/ QModelIndex {
  pub fn internalPointer<RetType, T: QModelIndex_internalPointer<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.internalPointer(self);
    // return 1;
  }
}

pub trait QModelIndex_internalPointer<RetType> {
  fn internalPointer(self , rsthis: & QModelIndex) -> RetType;
}

  // proto:  void * QModelIndex::internalPointer();
impl<'a> /*trait*/ QModelIndex_internalPointer<()> for () {
  fn internalPointer(self , rsthis: & QModelIndex) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QModelIndex15internalPointerEv()};
     unsafe {demth_ZNK11QModelIndex15internalPointerEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QModelIndex::QModelIndex(int arow, int acolumn, quintptr id, const QAbstractItemModel * amodel);
impl<'a> /*trait*/ QModelIndex_New for (i32, i32, i32, &'a QAbstractItemModel) {
  fn New(self) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QModelIndexC1EiiiPK18QAbstractItemModel()};
    let ctysz: c_int = unsafe{QModelIndex_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3.qclsinst  as *mut c_void;
    // unsafe {_ZN11QModelIndexC1EiiiPK18QAbstractItemModel(qthis, arg0, arg1, arg2, arg3)};
    let qthis: u64 = unsafe {dector_ZN11QModelIndexC1EiiiPK18QAbstractItemModel(arg0, arg1, arg2, arg3)} as u64;
    let rsthis = QModelIndex{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QModelIndex::isValid();
impl /*struct*/ QModelIndex {
  pub fn isValid<RetType, T: QModelIndex_isValid<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QModelIndex_isValid<RetType> {
  fn isValid(self , rsthis: & QModelIndex) -> RetType;
}

  // proto:  bool QModelIndex::isValid();
impl<'a> /*trait*/ QModelIndex_isValid<i8> for () {
  fn isValid(self , rsthis: & QModelIndex) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QModelIndex7isValidEv()};
    let mut ret = unsafe {_ZNK11QModelIndex7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QModelIndex QModelIndex::parent();
impl /*struct*/ QModelIndex {
  pub fn parent<RetType, T: QModelIndex_parent<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.parent(self);
    // return 1;
  }
}

pub trait QModelIndex_parent<RetType> {
  fn parent(self , rsthis: & QModelIndex) -> RetType;
}

  // proto:  QModelIndex QModelIndex::parent();
impl<'a> /*trait*/ QModelIndex_parent<QModelIndex> for () {
  fn parent(self , rsthis: & QModelIndex) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QModelIndex6parentEv()};
    let mut ret = unsafe {demth_ZNK11QModelIndex6parentEv(rsthis.qclsinst)};
    let mut ret1 = QModelIndex::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QModelIndex QModelIndex::sibling(int row, int column);
impl /*struct*/ QModelIndex {
  pub fn sibling<RetType, T: QModelIndex_sibling<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sibling(self);
    // return 1;
  }
}

pub trait QModelIndex_sibling<RetType> {
  fn sibling(self , rsthis: & QModelIndex) -> RetType;
}

  // proto:  QModelIndex QModelIndex::sibling(int row, int column);
impl<'a> /*trait*/ QModelIndex_sibling<QModelIndex> for (i32, i32) {
  fn sibling(self , rsthis: & QModelIndex) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QModelIndex7siblingEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {demth_ZNK11QModelIndex7siblingEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QModelIndex::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QAbstractItemModel * QModelIndex::model();
impl /*struct*/ QModelIndex {
  pub fn model<RetType, T: QModelIndex_model<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.model(self);
    // return 1;
  }
}

pub trait QModelIndex_model<RetType> {
  fn model(self , rsthis: & QModelIndex) -> RetType;
}

  // proto:  const QAbstractItemModel * QModelIndex::model();
impl<'a> /*trait*/ QModelIndex_model<()> for () {
  fn model(self , rsthis: & QModelIndex) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QModelIndex5modelEv()};
     unsafe {_ZNK11QModelIndex5modelEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QVariant QModelIndex::data(int role);
impl /*struct*/ QModelIndex {
  pub fn data<RetType, T: QModelIndex_data<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.data(self);
    // return 1;
  }
}

pub trait QModelIndex_data<RetType> {
  fn data(self , rsthis: & QModelIndex) -> RetType;
}

  // proto:  QVariant QModelIndex::data(int role);
impl<'a> /*trait*/ QModelIndex_data<QVariant> for (i32) {
  fn data(self , rsthis: & QModelIndex) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QModelIndex4dataEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {demth_ZNK11QModelIndex4dataEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QVariant::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QModelIndex::row();
impl /*struct*/ QModelIndex {
  pub fn row<RetType, T: QModelIndex_row<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.row(self);
    // return 1;
  }
}

pub trait QModelIndex_row<RetType> {
  fn row(self , rsthis: & QModelIndex) -> RetType;
}

  // proto:  int QModelIndex::row();
impl<'a> /*trait*/ QModelIndex_row<i32> for () {
  fn row(self , rsthis: & QModelIndex) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QModelIndex3rowEv()};
    let mut ret = unsafe {_ZNK11QModelIndex3rowEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QPersistentModelIndex {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QPersistentModelIndex {
    return QPersistentModelIndex{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  QModelIndex QPersistentModelIndex::sibling(int row, int column);
impl /*struct*/ QPersistentModelIndex {
  pub fn sibling<RetType, T: QPersistentModelIndex_sibling<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sibling(self);
    // return 1;
  }
}

pub trait QPersistentModelIndex_sibling<RetType> {
  fn sibling(self , rsthis: & QPersistentModelIndex) -> RetType;
}

  // proto:  QModelIndex QPersistentModelIndex::sibling(int row, int column);
impl<'a> /*trait*/ QPersistentModelIndex_sibling<QModelIndex> for (i32, i32) {
  fn sibling(self , rsthis: & QPersistentModelIndex) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QPersistentModelIndex7siblingEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK21QPersistentModelIndex7siblingEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QModelIndex::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QVariant QPersistentModelIndex::data(int role);
impl /*struct*/ QPersistentModelIndex {
  pub fn data<RetType, T: QPersistentModelIndex_data<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.data(self);
    // return 1;
  }
}

pub trait QPersistentModelIndex_data<RetType> {
  fn data(self , rsthis: & QPersistentModelIndex) -> RetType;
}

  // proto:  QVariant QPersistentModelIndex::data(int role);
impl<'a> /*trait*/ QPersistentModelIndex_data<QVariant> for (i32) {
  fn data(self , rsthis: & QPersistentModelIndex) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QPersistentModelIndex4dataEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK21QPersistentModelIndex4dataEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QVariant::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QModelIndex QPersistentModelIndex::parent();
impl /*struct*/ QPersistentModelIndex {
  pub fn parent<RetType, T: QPersistentModelIndex_parent<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.parent(self);
    // return 1;
  }
}

pub trait QPersistentModelIndex_parent<RetType> {
  fn parent(self , rsthis: & QPersistentModelIndex) -> RetType;
}

  // proto:  QModelIndex QPersistentModelIndex::parent();
impl<'a> /*trait*/ QPersistentModelIndex_parent<QModelIndex> for () {
  fn parent(self , rsthis: & QPersistentModelIndex) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QPersistentModelIndex6parentEv()};
    let mut ret = unsafe {_ZNK21QPersistentModelIndex6parentEv(rsthis.qclsinst)};
    let mut ret1 = QModelIndex::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPersistentModelIndex::QPersistentModelIndex(const QPersistentModelIndex & other);
impl /*struct*/ QPersistentModelIndex {
  pub fn New<T: QPersistentModelIndex_New>(value: T) -> QPersistentModelIndex {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QPersistentModelIndex_New {
  fn New(self) -> QPersistentModelIndex;
}

  // proto:  void QPersistentModelIndex::QPersistentModelIndex(const QPersistentModelIndex & other);
impl<'a> /*trait*/ QPersistentModelIndex_New for (&'a QPersistentModelIndex) {
  fn New(self) -> QPersistentModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QPersistentModelIndexC1ERKS_()};
    let ctysz: c_int = unsafe{QPersistentModelIndex_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN21QPersistentModelIndexC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN21QPersistentModelIndexC1ERKS_(arg0)} as u64;
    let rsthis = QPersistentModelIndex{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void * QPersistentModelIndex::internalPointer();
impl /*struct*/ QPersistentModelIndex {
  pub fn internalPointer<RetType, T: QPersistentModelIndex_internalPointer<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.internalPointer(self);
    // return 1;
  }
}

pub trait QPersistentModelIndex_internalPointer<RetType> {
  fn internalPointer(self , rsthis: & QPersistentModelIndex) -> RetType;
}

  // proto:  void * QPersistentModelIndex::internalPointer();
impl<'a> /*trait*/ QPersistentModelIndex_internalPointer<()> for () {
  fn internalPointer(self , rsthis: & QPersistentModelIndex) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QPersistentModelIndex15internalPointerEv()};
     unsafe {_ZNK21QPersistentModelIndex15internalPointerEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QPersistentModelIndex::row();
impl /*struct*/ QPersistentModelIndex {
  pub fn row<RetType, T: QPersistentModelIndex_row<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.row(self);
    // return 1;
  }
}

pub trait QPersistentModelIndex_row<RetType> {
  fn row(self , rsthis: & QPersistentModelIndex) -> RetType;
}

  // proto:  int QPersistentModelIndex::row();
impl<'a> /*trait*/ QPersistentModelIndex_row<i32> for () {
  fn row(self , rsthis: & QPersistentModelIndex) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QPersistentModelIndex3rowEv()};
    let mut ret = unsafe {_ZNK21QPersistentModelIndex3rowEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  quintptr QPersistentModelIndex::internalId();
impl /*struct*/ QPersistentModelIndex {
  pub fn internalId<RetType, T: QPersistentModelIndex_internalId<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.internalId(self);
    // return 1;
  }
}

pub trait QPersistentModelIndex_internalId<RetType> {
  fn internalId(self , rsthis: & QPersistentModelIndex) -> RetType;
}

  // proto:  quintptr QPersistentModelIndex::internalId();
impl<'a> /*trait*/ QPersistentModelIndex_internalId<i32> for () {
  fn internalId(self , rsthis: & QPersistentModelIndex) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QPersistentModelIndex10internalIdEv()};
    let mut ret = unsafe {_ZNK21QPersistentModelIndex10internalIdEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  const QAbstractItemModel * QPersistentModelIndex::model();
impl /*struct*/ QPersistentModelIndex {
  pub fn model<RetType, T: QPersistentModelIndex_model<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.model(self);
    // return 1;
  }
}

pub trait QPersistentModelIndex_model<RetType> {
  fn model(self , rsthis: & QPersistentModelIndex) -> RetType;
}

  // proto:  const QAbstractItemModel * QPersistentModelIndex::model();
impl<'a> /*trait*/ QPersistentModelIndex_model<()> for () {
  fn model(self , rsthis: & QPersistentModelIndex) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QPersistentModelIndex5modelEv()};
     unsafe {_ZNK21QPersistentModelIndex5modelEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QPersistentModelIndex::~QPersistentModelIndex();
impl /*struct*/ QPersistentModelIndex {
  pub fn Free<RetType, T: QPersistentModelIndex_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QPersistentModelIndex_Free<RetType> {
  fn Free(self , rsthis: & QPersistentModelIndex) -> RetType;
}

  // proto:  void QPersistentModelIndex::~QPersistentModelIndex();
impl<'a> /*trait*/ QPersistentModelIndex_Free<()> for () {
  fn Free(self , rsthis: & QPersistentModelIndex) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QPersistentModelIndexD0Ev()};
     unsafe {_ZN21QPersistentModelIndexD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QPersistentModelIndex::QPersistentModelIndex(const QModelIndex & index);
impl<'a> /*trait*/ QPersistentModelIndex_New for (&'a QModelIndex) {
  fn New(self) -> QPersistentModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QPersistentModelIndexC1ERK11QModelIndex()};
    let ctysz: c_int = unsafe{QPersistentModelIndex_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN21QPersistentModelIndexC1ERK11QModelIndex(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN21QPersistentModelIndexC1ERK11QModelIndex(arg0)} as u64;
    let rsthis = QPersistentModelIndex{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QPersistentModelIndex::QPersistentModelIndex();
impl<'a> /*trait*/ QPersistentModelIndex_New for () {
  fn New(self) -> QPersistentModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QPersistentModelIndexC1Ev()};
    let ctysz: c_int = unsafe{QPersistentModelIndex_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    // unsafe {_ZN21QPersistentModelIndexC1Ev(qthis)};
    let qthis: u64 = unsafe {dector_ZN21QPersistentModelIndexC1Ev()} as u64;
    let rsthis = QPersistentModelIndex{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QPersistentModelIndex::column();
impl /*struct*/ QPersistentModelIndex {
  pub fn column<RetType, T: QPersistentModelIndex_column<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.column(self);
    // return 1;
  }
}

pub trait QPersistentModelIndex_column<RetType> {
  fn column(self , rsthis: & QPersistentModelIndex) -> RetType;
}

  // proto:  int QPersistentModelIndex::column();
impl<'a> /*trait*/ QPersistentModelIndex_column<i32> for () {
  fn column(self , rsthis: & QPersistentModelIndex) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QPersistentModelIndex6columnEv()};
    let mut ret = unsafe {_ZNK21QPersistentModelIndex6columnEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QPersistentModelIndex::swap(QPersistentModelIndex & other);
impl /*struct*/ QPersistentModelIndex {
  pub fn swap<RetType, T: QPersistentModelIndex_swap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QPersistentModelIndex_swap<RetType> {
  fn swap(self , rsthis: & QPersistentModelIndex) -> RetType;
}

  // proto:  void QPersistentModelIndex::swap(QPersistentModelIndex & other);
impl<'a> /*trait*/ QPersistentModelIndex_swap<()> for (&'a QPersistentModelIndex) {
  fn swap(self , rsthis: & QPersistentModelIndex) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QPersistentModelIndex4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {demth_ZN21QPersistentModelIndex4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QModelIndex QPersistentModelIndex::child(int row, int column);
impl /*struct*/ QPersistentModelIndex {
  pub fn child<RetType, T: QPersistentModelIndex_child<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.child(self);
    // return 1;
  }
}

pub trait QPersistentModelIndex_child<RetType> {
  fn child(self , rsthis: & QPersistentModelIndex) -> RetType;
}

  // proto:  QModelIndex QPersistentModelIndex::child(int row, int column);
impl<'a> /*trait*/ QPersistentModelIndex_child<QModelIndex> for (i32, i32) {
  fn child(self , rsthis: & QPersistentModelIndex) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QPersistentModelIndex5childEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK21QPersistentModelIndex5childEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QModelIndex::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QPersistentModelIndex::isValid();
impl /*struct*/ QPersistentModelIndex {
  pub fn isValid<RetType, T: QPersistentModelIndex_isValid<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QPersistentModelIndex_isValid<RetType> {
  fn isValid(self , rsthis: & QPersistentModelIndex) -> RetType;
}

  // proto:  bool QPersistentModelIndex::isValid();
impl<'a> /*trait*/ QPersistentModelIndex_isValid<i8> for () {
  fn isValid(self , rsthis: & QPersistentModelIndex) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QPersistentModelIndex7isValidEv()};
    let mut ret = unsafe {_ZNK21QPersistentModelIndex7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QAbstractTableModel {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QAbstractTableModel {
    return QAbstractTableModel{qbase: QAbstractItemModel::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QAbstractTableModel {
  type Target = QAbstractItemModel;

  fn deref(&self) -> &QAbstractItemModel {
    return & self.qbase;
  }
}
impl AsRef<QAbstractItemModel> for QAbstractTableModel {
  fn as_ref(& self) -> & QAbstractItemModel {
    return & self.qbase;
  }
}
  // proto:  void QAbstractTableModel::QAbstractTableModel(const QAbstractTableModel & );
impl /*struct*/ QAbstractTableModel {
  pub fn New<T: QAbstractTableModel_New>(value: T) -> QAbstractTableModel {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QAbstractTableModel_New {
  fn New(self) -> QAbstractTableModel;
}

  // proto:  void QAbstractTableModel::QAbstractTableModel(const QAbstractTableModel & );
impl<'a> /*trait*/ QAbstractTableModel_New for (&'a QAbstractTableModel) {
  fn New(self) -> QAbstractTableModel {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QAbstractTableModelC1ERKS_()};
    let ctysz: c_int = unsafe{QAbstractTableModel_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN19QAbstractTableModelC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN19QAbstractTableModelC1ERKS_(arg0)} as u64;
    let rsthis = QAbstractTableModel{qbase: QAbstractItemModel::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QAbstractTableModel::QAbstractTableModel(QObject * parent);
impl<'a> /*trait*/ QAbstractTableModel_New for (&'a QObject) {
  fn New(self) -> QAbstractTableModel {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QAbstractTableModelC1EP7QObject()};
    let ctysz: c_int = unsafe{QAbstractTableModel_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN19QAbstractTableModelC1EP7QObject(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN19QAbstractTableModelC1EP7QObject(arg0)} as u64;
    let rsthis = QAbstractTableModel{qbase: QAbstractItemModel::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QModelIndex QAbstractTableModel::index(int row, int column, const QModelIndex & parent);
impl /*struct*/ QAbstractTableModel {
  pub fn index<RetType, T: QAbstractTableModel_index<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.index(self);
    // return 1;
  }
}

pub trait QAbstractTableModel_index<RetType> {
  fn index(self , rsthis: & QAbstractTableModel) -> RetType;
}

  // proto:  QModelIndex QAbstractTableModel::index(int row, int column, const QModelIndex & parent);
impl<'a> /*trait*/ QAbstractTableModel_index<QModelIndex> for (i32, i32, &'a QModelIndex) {
  fn index(self , rsthis: & QAbstractTableModel) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QAbstractTableModel5indexEiiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK19QAbstractTableModel5indexEiiRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QModelIndex::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QModelIndex QAbstractTableModel::sibling(int row, int column, const QModelIndex & idx);
impl /*struct*/ QAbstractTableModel {
  pub fn sibling<RetType, T: QAbstractTableModel_sibling<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sibling(self);
    // return 1;
  }
}

pub trait QAbstractTableModel_sibling<RetType> {
  fn sibling(self , rsthis: & QAbstractTableModel) -> RetType;
}

  // proto:  QModelIndex QAbstractTableModel::sibling(int row, int column, const QModelIndex & idx);
impl<'a> /*trait*/ QAbstractTableModel_sibling<QModelIndex> for (i32, i32, &'a QModelIndex) {
  fn sibling(self , rsthis: & QAbstractTableModel) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QAbstractTableModel7siblingEiiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK19QAbstractTableModel7siblingEiiRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QModelIndex::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QAbstractTableModel::~QAbstractTableModel();
impl /*struct*/ QAbstractTableModel {
  pub fn Free<RetType, T: QAbstractTableModel_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QAbstractTableModel_Free<RetType> {
  fn Free(self , rsthis: & QAbstractTableModel) -> RetType;
}

  // proto:  void QAbstractTableModel::~QAbstractTableModel();
impl<'a> /*trait*/ QAbstractTableModel_Free<()> for () {
  fn Free(self , rsthis: & QAbstractTableModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QAbstractTableModelD0Ev()};
     unsafe {_ZN19QAbstractTableModelD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QAbstractTableModel::metaObject();
impl /*struct*/ QAbstractTableModel {
  pub fn metaObject<RetType, T: QAbstractTableModel_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QAbstractTableModel_metaObject<RetType> {
  fn metaObject(self , rsthis: & QAbstractTableModel) -> RetType;
}

  // proto:  const QMetaObject * QAbstractTableModel::metaObject();
impl<'a> /*trait*/ QAbstractTableModel_metaObject<()> for () {
  fn metaObject(self , rsthis: & QAbstractTableModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QAbstractTableModel10metaObjectEv()};
     unsafe {_ZNK19QAbstractTableModel10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QAbstractItemModel {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QAbstractItemModel {
    return QAbstractItemModel{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QAbstractItemModel {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QAbstractItemModel {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  bool QAbstractItemModel::removeColumns(int column, int count, const QModelIndex & parent);
impl /*struct*/ QAbstractItemModel {
  pub fn removeColumns<RetType, T: QAbstractItemModel_removeColumns<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.removeColumns(self);
    // return 1;
  }
}

pub trait QAbstractItemModel_removeColumns<RetType> {
  fn removeColumns(self , rsthis: & QAbstractItemModel) -> RetType;
}

  // proto:  bool QAbstractItemModel::removeColumns(int column, int count, const QModelIndex & parent);
impl<'a> /*trait*/ QAbstractItemModel_removeColumns<i8> for (i32, i32, &'a QModelIndex) {
  fn removeColumns(self , rsthis: & QAbstractItemModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QAbstractItemModel13removeColumnsEiiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN18QAbstractItemModel13removeColumnsEiiRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QAbstractItemModel::~QAbstractItemModel();
impl /*struct*/ QAbstractItemModel {
  pub fn Free<RetType, T: QAbstractItemModel_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QAbstractItemModel_Free<RetType> {
  fn Free(self , rsthis: & QAbstractItemModel) -> RetType;
}

  // proto:  void QAbstractItemModel::~QAbstractItemModel();
impl<'a> /*trait*/ QAbstractItemModel_Free<()> for () {
  fn Free(self , rsthis: & QAbstractItemModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QAbstractItemModelD0Ev()};
     unsafe {_ZN18QAbstractItemModelD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QAbstractItemModel::canFetchMore(const QModelIndex & parent);
impl /*struct*/ QAbstractItemModel {
  pub fn canFetchMore<RetType, T: QAbstractItemModel_canFetchMore<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.canFetchMore(self);
    // return 1;
  }
}

pub trait QAbstractItemModel_canFetchMore<RetType> {
  fn canFetchMore(self , rsthis: & QAbstractItemModel) -> RetType;
}

  // proto:  bool QAbstractItemModel::canFetchMore(const QModelIndex & parent);
impl<'a> /*trait*/ QAbstractItemModel_canFetchMore<i8> for (&'a QModelIndex) {
  fn canFetchMore(self , rsthis: & QAbstractItemModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QAbstractItemModel12canFetchMoreERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK18QAbstractItemModel12canFetchMoreERK11QModelIndex(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QAbstractItemModel::submit();
impl /*struct*/ QAbstractItemModel {
  pub fn submit<RetType, T: QAbstractItemModel_submit<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.submit(self);
    // return 1;
  }
}

pub trait QAbstractItemModel_submit<RetType> {
  fn submit(self , rsthis: & QAbstractItemModel) -> RetType;
}

  // proto:  bool QAbstractItemModel::submit();
impl<'a> /*trait*/ QAbstractItemModel_submit<i8> for () {
  fn submit(self , rsthis: & QAbstractItemModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QAbstractItemModel6submitEv()};
    let mut ret = unsafe {_ZN18QAbstractItemModel6submitEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QAbstractItemModel::hasIndex(int row, int column, const QModelIndex & parent);
impl /*struct*/ QAbstractItemModel {
  pub fn hasIndex<RetType, T: QAbstractItemModel_hasIndex<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasIndex(self);
    // return 1;
  }
}

pub trait QAbstractItemModel_hasIndex<RetType> {
  fn hasIndex(self , rsthis: & QAbstractItemModel) -> RetType;
}

  // proto:  bool QAbstractItemModel::hasIndex(int row, int column, const QModelIndex & parent);
impl<'a> /*trait*/ QAbstractItemModel_hasIndex<i8> for (i32, i32, &'a QModelIndex) {
  fn hasIndex(self , rsthis: & QAbstractItemModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QAbstractItemModel8hasIndexEiiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK18QAbstractItemModel8hasIndexEiiRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QHash<int, QByteArray> QAbstractItemModel::roleNames();
impl /*struct*/ QAbstractItemModel {
  pub fn roleNames<RetType, T: QAbstractItemModel_roleNames<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.roleNames(self);
    // return 1;
  }
}

pub trait QAbstractItemModel_roleNames<RetType> {
  fn roleNames(self , rsthis: & QAbstractItemModel) -> RetType;
}

  // proto:  QHash<int, QByteArray> QAbstractItemModel::roleNames();
impl<'a> /*trait*/ QAbstractItemModel_roleNames<()> for () {
  fn roleNames(self , rsthis: & QAbstractItemModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QAbstractItemModel9roleNamesEv()};
     unsafe {_ZNK18QAbstractItemModel9roleNamesEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QAbstractItemModel::moveColumn(const QModelIndex & sourceParent, int sourceColumn, const QModelIndex & destinationParent, int destinationChild);
impl /*struct*/ QAbstractItemModel {
  pub fn moveColumn<RetType, T: QAbstractItemModel_moveColumn<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.moveColumn(self);
    // return 1;
  }
}

pub trait QAbstractItemModel_moveColumn<RetType> {
  fn moveColumn(self , rsthis: & QAbstractItemModel) -> RetType;
}

  // proto:  bool QAbstractItemModel::moveColumn(const QModelIndex & sourceParent, int sourceColumn, const QModelIndex & destinationParent, int destinationChild);
impl<'a> /*trait*/ QAbstractItemModel_moveColumn<i8> for (&'a QModelIndex, i32, &'a QModelIndex, i32) {
  fn moveColumn(self , rsthis: & QAbstractItemModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QAbstractItemModel10moveColumnERK11QModelIndexiS2_i()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3  as c_int;
    let mut ret = unsafe {demth_ZN18QAbstractItemModel10moveColumnERK11QModelIndexiS2_i(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QAbstractItemModel::fetchMore(const QModelIndex & parent);
impl /*struct*/ QAbstractItemModel {
  pub fn fetchMore<RetType, T: QAbstractItemModel_fetchMore<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.fetchMore(self);
    // return 1;
  }
}

pub trait QAbstractItemModel_fetchMore<RetType> {
  fn fetchMore(self , rsthis: & QAbstractItemModel) -> RetType;
}

  // proto:  void QAbstractItemModel::fetchMore(const QModelIndex & parent);
impl<'a> /*trait*/ QAbstractItemModel_fetchMore<()> for (&'a QModelIndex) {
  fn fetchMore(self , rsthis: & QAbstractItemModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QAbstractItemModel9fetchMoreERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN18QAbstractItemModel9fetchMoreERK11QModelIndex(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QAbstractItemModel::insertRows(int row, int count, const QModelIndex & parent);
impl /*struct*/ QAbstractItemModel {
  pub fn insertRows<RetType, T: QAbstractItemModel_insertRows<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.insertRows(self);
    // return 1;
  }
}

pub trait QAbstractItemModel_insertRows<RetType> {
  fn insertRows(self , rsthis: & QAbstractItemModel) -> RetType;
}

  // proto:  bool QAbstractItemModel::insertRows(int row, int count, const QModelIndex & parent);
impl<'a> /*trait*/ QAbstractItemModel_insertRows<i8> for (i32, i32, &'a QModelIndex) {
  fn insertRows(self , rsthis: & QAbstractItemModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QAbstractItemModel10insertRowsEiiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN18QAbstractItemModel10insertRowsEiiRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QSize QAbstractItemModel::span(const QModelIndex & index);
impl /*struct*/ QAbstractItemModel {
  pub fn span<RetType, T: QAbstractItemModel_span<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.span(self);
    // return 1;
  }
}

pub trait QAbstractItemModel_span<RetType> {
  fn span(self , rsthis: & QAbstractItemModel) -> RetType;
}

  // proto:  QSize QAbstractItemModel::span(const QModelIndex & index);
impl<'a> /*trait*/ QAbstractItemModel_span<QSize> for (&'a QModelIndex) {
  fn span(self , rsthis: & QAbstractItemModel) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QAbstractItemModel4spanERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK18QAbstractItemModel4spanERK11QModelIndex(rsthis.qclsinst, arg0)};
    let mut ret1 = QSize::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QAbstractItemModel::QAbstractItemModel(QObject * parent);
impl /*struct*/ QAbstractItemModel {
  pub fn New<T: QAbstractItemModel_New>(value: T) -> QAbstractItemModel {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QAbstractItemModel_New {
  fn New(self) -> QAbstractItemModel;
}

  // proto:  void QAbstractItemModel::QAbstractItemModel(QObject * parent);
impl<'a> /*trait*/ QAbstractItemModel_New for (&'a QObject) {
  fn New(self) -> QAbstractItemModel {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QAbstractItemModelC1EP7QObject()};
    let ctysz: c_int = unsafe{QAbstractItemModel_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN18QAbstractItemModelC1EP7QObject(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN18QAbstractItemModelC1EP7QObject(arg0)} as u64;
    let rsthis = QAbstractItemModel{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QModelIndex QAbstractItemModel::index(int row, int column, const QModelIndex & parent);
impl /*struct*/ QAbstractItemModel {
  pub fn index<RetType, T: QAbstractItemModel_index<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.index(self);
    // return 1;
  }
}

pub trait QAbstractItemModel_index<RetType> {
  fn index(self , rsthis: & QAbstractItemModel) -> RetType;
}

  // proto:  QModelIndex QAbstractItemModel::index(int row, int column, const QModelIndex & parent);
impl<'a> /*trait*/ QAbstractItemModel_index<QModelIndex> for (i32, i32, &'a QModelIndex) {
  fn index(self , rsthis: & QAbstractItemModel) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QAbstractItemModel5indexEiiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK18QAbstractItemModel5indexEiiRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QModelIndex::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QAbstractItemModel::insertRow(int row, const QModelIndex & parent);
impl /*struct*/ QAbstractItemModel {
  pub fn insertRow<RetType, T: QAbstractItemModel_insertRow<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.insertRow(self);
    // return 1;
  }
}

pub trait QAbstractItemModel_insertRow<RetType> {
  fn insertRow(self , rsthis: & QAbstractItemModel) -> RetType;
}

  // proto:  bool QAbstractItemModel::insertRow(int row, const QModelIndex & parent);
impl<'a> /*trait*/ QAbstractItemModel_insertRow<i8> for (i32, &'a QModelIndex) {
  fn insertRow(self , rsthis: & QAbstractItemModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QAbstractItemModel9insertRowEiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {demth_ZN18QAbstractItemModel9insertRowEiRK11QModelIndex(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QAbstractItemModel::QAbstractItemModel(const QAbstractItemModel & );
impl<'a> /*trait*/ QAbstractItemModel_New for (&'a QAbstractItemModel) {
  fn New(self) -> QAbstractItemModel {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QAbstractItemModelC1ERKS_()};
    let ctysz: c_int = unsafe{QAbstractItemModel_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN18QAbstractItemModelC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN18QAbstractItemModelC1ERKS_(arg0)} as u64;
    let rsthis = QAbstractItemModel{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QMetaObject * QAbstractItemModel::metaObject();
impl /*struct*/ QAbstractItemModel {
  pub fn metaObject<RetType, T: QAbstractItemModel_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QAbstractItemModel_metaObject<RetType> {
  fn metaObject(self , rsthis: & QAbstractItemModel) -> RetType;
}

  // proto:  const QMetaObject * QAbstractItemModel::metaObject();
impl<'a> /*trait*/ QAbstractItemModel_metaObject<()> for () {
  fn metaObject(self , rsthis: & QAbstractItemModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QAbstractItemModel10metaObjectEv()};
     unsafe {_ZNK18QAbstractItemModel10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QAbstractItemModel::removeRow(int row, const QModelIndex & parent);
impl /*struct*/ QAbstractItemModel {
  pub fn removeRow<RetType, T: QAbstractItemModel_removeRow<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.removeRow(self);
    // return 1;
  }
}

pub trait QAbstractItemModel_removeRow<RetType> {
  fn removeRow(self , rsthis: & QAbstractItemModel) -> RetType;
}

  // proto:  bool QAbstractItemModel::removeRow(int row, const QModelIndex & parent);
impl<'a> /*trait*/ QAbstractItemModel_removeRow<i8> for (i32, &'a QModelIndex) {
  fn removeRow(self , rsthis: & QAbstractItemModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QAbstractItemModel9removeRowEiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {demth_ZN18QAbstractItemModel9removeRowEiRK11QModelIndex(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QAbstractItemModel::setData(const QModelIndex & index, const QVariant & value, int role);
impl /*struct*/ QAbstractItemModel {
  pub fn setData<RetType, T: QAbstractItemModel_setData<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setData(self);
    // return 1;
  }
}

pub trait QAbstractItemModel_setData<RetType> {
  fn setData(self , rsthis: & QAbstractItemModel) -> RetType;
}

  // proto:  bool QAbstractItemModel::setData(const QModelIndex & index, const QVariant & value, int role);
impl<'a> /*trait*/ QAbstractItemModel_setData<i8> for (&'a QModelIndex, &'a QVariant, i32) {
  fn setData(self , rsthis: & QAbstractItemModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QAbstractItemModel7setDataERK11QModelIndexRK8QVarianti()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
    let mut ret = unsafe {_ZN18QAbstractItemModel7setDataERK11QModelIndexRK8QVarianti(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QAbstractItemModel::rowCount(const QModelIndex & parent);
impl /*struct*/ QAbstractItemModel {
  pub fn rowCount<RetType, T: QAbstractItemModel_rowCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rowCount(self);
    // return 1;
  }
}

pub trait QAbstractItemModel_rowCount<RetType> {
  fn rowCount(self , rsthis: & QAbstractItemModel) -> RetType;
}

  // proto:  int QAbstractItemModel::rowCount(const QModelIndex & parent);
impl<'a> /*trait*/ QAbstractItemModel_rowCount<i32> for (&'a QModelIndex) {
  fn rowCount(self , rsthis: & QAbstractItemModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QAbstractItemModel8rowCountERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK18QAbstractItemModel8rowCountERK11QModelIndex(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  bool QAbstractItemModel::removeRows(int row, int count, const QModelIndex & parent);
impl /*struct*/ QAbstractItemModel {
  pub fn removeRows<RetType, T: QAbstractItemModel_removeRows<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.removeRows(self);
    // return 1;
  }
}

pub trait QAbstractItemModel_removeRows<RetType> {
  fn removeRows(self , rsthis: & QAbstractItemModel) -> RetType;
}

  // proto:  bool QAbstractItemModel::removeRows(int row, int count, const QModelIndex & parent);
impl<'a> /*trait*/ QAbstractItemModel_removeRows<i8> for (i32, i32, &'a QModelIndex) {
  fn removeRows(self , rsthis: & QAbstractItemModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QAbstractItemModel10removeRowsEiiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN18QAbstractItemModel10removeRowsEiiRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QAbstractItemModel::hasChildren(const QModelIndex & parent);
impl /*struct*/ QAbstractItemModel {
  pub fn hasChildren<RetType, T: QAbstractItemModel_hasChildren<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasChildren(self);
    // return 1;
  }
}

pub trait QAbstractItemModel_hasChildren<RetType> {
  fn hasChildren(self , rsthis: & QAbstractItemModel) -> RetType;
}

  // proto:  bool QAbstractItemModel::hasChildren(const QModelIndex & parent);
impl<'a> /*trait*/ QAbstractItemModel_hasChildren<i8> for (&'a QModelIndex) {
  fn hasChildren(self , rsthis: & QAbstractItemModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QAbstractItemModel11hasChildrenERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK18QAbstractItemModel11hasChildrenERK11QModelIndex(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QAbstractItemModel::moveRow(const QModelIndex & sourceParent, int sourceRow, const QModelIndex & destinationParent, int destinationChild);
impl /*struct*/ QAbstractItemModel {
  pub fn moveRow<RetType, T: QAbstractItemModel_moveRow<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.moveRow(self);
    // return 1;
  }
}

pub trait QAbstractItemModel_moveRow<RetType> {
  fn moveRow(self , rsthis: & QAbstractItemModel) -> RetType;
}

  // proto:  bool QAbstractItemModel::moveRow(const QModelIndex & sourceParent, int sourceRow, const QModelIndex & destinationParent, int destinationChild);
impl<'a> /*trait*/ QAbstractItemModel_moveRow<i8> for (&'a QModelIndex, i32, &'a QModelIndex, i32) {
  fn moveRow(self , rsthis: & QAbstractItemModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QAbstractItemModel7moveRowERK11QModelIndexiS2_i()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3  as c_int;
    let mut ret = unsafe {demth_ZN18QAbstractItemModel7moveRowERK11QModelIndexiS2_i(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QAbstractItemModel::revert();
impl /*struct*/ QAbstractItemModel {
  pub fn revert<RetType, T: QAbstractItemModel_revert<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.revert(self);
    // return 1;
  }
}

pub trait QAbstractItemModel_revert<RetType> {
  fn revert(self , rsthis: & QAbstractItemModel) -> RetType;
}

  // proto:  void QAbstractItemModel::revert();
impl<'a> /*trait*/ QAbstractItemModel_revert<()> for () {
  fn revert(self , rsthis: & QAbstractItemModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QAbstractItemModel6revertEv()};
     unsafe {_ZN18QAbstractItemModel6revertEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QAbstractItemModel::removeColumn(int column, const QModelIndex & parent);
impl /*struct*/ QAbstractItemModel {
  pub fn removeColumn<RetType, T: QAbstractItemModel_removeColumn<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.removeColumn(self);
    // return 1;
  }
}

pub trait QAbstractItemModel_removeColumn<RetType> {
  fn removeColumn(self , rsthis: & QAbstractItemModel) -> RetType;
}

  // proto:  bool QAbstractItemModel::removeColumn(int column, const QModelIndex & parent);
impl<'a> /*trait*/ QAbstractItemModel_removeColumn<i8> for (i32, &'a QModelIndex) {
  fn removeColumn(self , rsthis: & QAbstractItemModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QAbstractItemModel12removeColumnEiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {demth_ZN18QAbstractItemModel12removeColumnEiRK11QModelIndex(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QAbstractItemModel::insertColumns(int column, int count, const QModelIndex & parent);
impl /*struct*/ QAbstractItemModel {
  pub fn insertColumns<RetType, T: QAbstractItemModel_insertColumns<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.insertColumns(self);
    // return 1;
  }
}

pub trait QAbstractItemModel_insertColumns<RetType> {
  fn insertColumns(self , rsthis: & QAbstractItemModel) -> RetType;
}

  // proto:  bool QAbstractItemModel::insertColumns(int column, int count, const QModelIndex & parent);
impl<'a> /*trait*/ QAbstractItemModel_insertColumns<i8> for (i32, i32, &'a QModelIndex) {
  fn insertColumns(self , rsthis: & QAbstractItemModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QAbstractItemModel13insertColumnsEiiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN18QAbstractItemModel13insertColumnsEiiRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QAbstractItemModel::insertColumn(int column, const QModelIndex & parent);
impl /*struct*/ QAbstractItemModel {
  pub fn insertColumn<RetType, T: QAbstractItemModel_insertColumn<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.insertColumn(self);
    // return 1;
  }
}

pub trait QAbstractItemModel_insertColumn<RetType> {
  fn insertColumn(self , rsthis: & QAbstractItemModel) -> RetType;
}

  // proto:  bool QAbstractItemModel::insertColumn(int column, const QModelIndex & parent);
impl<'a> /*trait*/ QAbstractItemModel_insertColumn<i8> for (i32, &'a QModelIndex) {
  fn insertColumn(self , rsthis: & QAbstractItemModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QAbstractItemModel12insertColumnEiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {demth_ZN18QAbstractItemModel12insertColumnEiRK11QModelIndex(rsthis.qclsinst, arg0, arg1)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QAbstractItemModel::moveColumns(const QModelIndex & sourceParent, int sourceColumn, int count, const QModelIndex & destinationParent, int destinationChild);
impl /*struct*/ QAbstractItemModel {
  pub fn moveColumns<RetType, T: QAbstractItemModel_moveColumns<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.moveColumns(self);
    // return 1;
  }
}

pub trait QAbstractItemModel_moveColumns<RetType> {
  fn moveColumns(self , rsthis: & QAbstractItemModel) -> RetType;
}

  // proto:  bool QAbstractItemModel::moveColumns(const QModelIndex & sourceParent, int sourceColumn, int count, const QModelIndex & destinationParent, int destinationChild);
impl<'a> /*trait*/ QAbstractItemModel_moveColumns<i8> for (&'a QModelIndex, i32, i32, &'a QModelIndex, i32) {
  fn moveColumns(self , rsthis: & QAbstractItemModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QAbstractItemModel11moveColumnsERK11QModelIndexiiS2_i()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3.qclsinst  as *mut c_void;
    let arg4 = self.4  as c_int;
    let mut ret = unsafe {_ZN18QAbstractItemModel11moveColumnsERK11QModelIndexiiS2_i(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QMap<int, QVariant> QAbstractItemModel::itemData(const QModelIndex & index);
impl /*struct*/ QAbstractItemModel {
  pub fn itemData<RetType, T: QAbstractItemModel_itemData<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.itemData(self);
    // return 1;
  }
}

pub trait QAbstractItemModel_itemData<RetType> {
  fn itemData(self , rsthis: & QAbstractItemModel) -> RetType;
}

  // proto:  QMap<int, QVariant> QAbstractItemModel::itemData(const QModelIndex & index);
impl<'a> /*trait*/ QAbstractItemModel_itemData<()> for (&'a QModelIndex) {
  fn itemData(self , rsthis: & QAbstractItemModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QAbstractItemModel8itemDataERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZNK18QAbstractItemModel8itemDataERK11QModelIndex(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QStringList QAbstractItemModel::mimeTypes();
impl /*struct*/ QAbstractItemModel {
  pub fn mimeTypes<RetType, T: QAbstractItemModel_mimeTypes<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mimeTypes(self);
    // return 1;
  }
}

pub trait QAbstractItemModel_mimeTypes<RetType> {
  fn mimeTypes(self , rsthis: & QAbstractItemModel) -> RetType;
}

  // proto:  QStringList QAbstractItemModel::mimeTypes();
impl<'a> /*trait*/ QAbstractItemModel_mimeTypes<()> for () {
  fn mimeTypes(self , rsthis: & QAbstractItemModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QAbstractItemModel9mimeTypesEv()};
     unsafe {_ZNK18QAbstractItemModel9mimeTypesEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QModelIndex QAbstractItemModel::parent(const QModelIndex & child);
impl /*struct*/ QAbstractItemModel {
  pub fn parent<RetType, T: QAbstractItemModel_parent<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.parent(self);
    // return 1;
  }
}

pub trait QAbstractItemModel_parent<RetType> {
  fn parent(self , rsthis: & QAbstractItemModel) -> RetType;
}

  // proto:  QModelIndex QAbstractItemModel::parent(const QModelIndex & child);
impl<'a> /*trait*/ QAbstractItemModel_parent<QModelIndex> for (&'a QModelIndex) {
  fn parent(self , rsthis: & QAbstractItemModel) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QAbstractItemModel6parentERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK18QAbstractItemModel6parentERK11QModelIndex(rsthis.qclsinst, arg0)};
    let mut ret1 = QModelIndex::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QModelIndex QAbstractItemModel::buddy(const QModelIndex & index);
impl /*struct*/ QAbstractItemModel {
  pub fn buddy<RetType, T: QAbstractItemModel_buddy<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.buddy(self);
    // return 1;
  }
}

pub trait QAbstractItemModel_buddy<RetType> {
  fn buddy(self , rsthis: & QAbstractItemModel) -> RetType;
}

  // proto:  QModelIndex QAbstractItemModel::buddy(const QModelIndex & index);
impl<'a> /*trait*/ QAbstractItemModel_buddy<QModelIndex> for (&'a QModelIndex) {
  fn buddy(self , rsthis: & QAbstractItemModel) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QAbstractItemModel5buddyERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK18QAbstractItemModel5buddyERK11QModelIndex(rsthis.qclsinst, arg0)};
    let mut ret1 = QModelIndex::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QAbstractItemModel::columnCount(const QModelIndex & parent);
impl /*struct*/ QAbstractItemModel {
  pub fn columnCount<RetType, T: QAbstractItemModel_columnCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.columnCount(self);
    // return 1;
  }
}

pub trait QAbstractItemModel_columnCount<RetType> {
  fn columnCount(self , rsthis: & QAbstractItemModel) -> RetType;
}

  // proto:  int QAbstractItemModel::columnCount(const QModelIndex & parent);
impl<'a> /*trait*/ QAbstractItemModel_columnCount<i32> for (&'a QModelIndex) {
  fn columnCount(self , rsthis: & QAbstractItemModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QAbstractItemModel11columnCountERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK18QAbstractItemModel11columnCountERK11QModelIndex(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QVariant QAbstractItemModel::data(const QModelIndex & index, int role);
impl /*struct*/ QAbstractItemModel {
  pub fn data<RetType, T: QAbstractItemModel_data<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.data(self);
    // return 1;
  }
}

pub trait QAbstractItemModel_data<RetType> {
  fn data(self , rsthis: & QAbstractItemModel) -> RetType;
}

  // proto:  QVariant QAbstractItemModel::data(const QModelIndex & index, int role);
impl<'a> /*trait*/ QAbstractItemModel_data<QVariant> for (&'a QModelIndex, i32) {
  fn data(self , rsthis: & QAbstractItemModel) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QAbstractItemModel4dataERK11QModelIndexi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK18QAbstractItemModel4dataERK11QModelIndexi(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QVariant::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QModelIndex QAbstractItemModel::sibling(int row, int column, const QModelIndex & idx);
impl /*struct*/ QAbstractItemModel {
  pub fn sibling<RetType, T: QAbstractItemModel_sibling<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sibling(self);
    // return 1;
  }
}

pub trait QAbstractItemModel_sibling<RetType> {
  fn sibling(self , rsthis: & QAbstractItemModel) -> RetType;
}

  // proto:  QModelIndex QAbstractItemModel::sibling(int row, int column, const QModelIndex & idx);
impl<'a> /*trait*/ QAbstractItemModel_sibling<QModelIndex> for (i32, i32, &'a QModelIndex) {
  fn sibling(self , rsthis: & QAbstractItemModel) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QAbstractItemModel7siblingEiiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK18QAbstractItemModel7siblingEiiRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QModelIndex::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QAbstractItemModel::moveRows(const QModelIndex & sourceParent, int sourceRow, int count, const QModelIndex & destinationParent, int destinationChild);
impl /*struct*/ QAbstractItemModel {
  pub fn moveRows<RetType, T: QAbstractItemModel_moveRows<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.moveRows(self);
    // return 1;
  }
}

pub trait QAbstractItemModel_moveRows<RetType> {
  fn moveRows(self , rsthis: & QAbstractItemModel) -> RetType;
}

  // proto:  bool QAbstractItemModel::moveRows(const QModelIndex & sourceParent, int sourceRow, int count, const QModelIndex & destinationParent, int destinationChild);
impl<'a> /*trait*/ QAbstractItemModel_moveRows<i8> for (&'a QModelIndex, i32, i32, &'a QModelIndex, i32) {
  fn moveRows(self , rsthis: & QAbstractItemModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QAbstractItemModel8moveRowsERK11QModelIndexiiS2_i()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3.qclsinst  as *mut c_void;
    let arg4 = self.4  as c_int;
    let mut ret = unsafe {_ZN18QAbstractItemModel8moveRowsERK11QModelIndexiiS2_i(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QAbstractListModel {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QAbstractListModel {
    return QAbstractListModel{qbase: QAbstractItemModel::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QAbstractListModel {
  type Target = QAbstractItemModel;

  fn deref(&self) -> &QAbstractItemModel {
    return & self.qbase;
  }
}
impl AsRef<QAbstractItemModel> for QAbstractListModel {
  fn as_ref(& self) -> & QAbstractItemModel {
    return & self.qbase;
  }
}
  // proto:  QModelIndex QAbstractListModel::sibling(int row, int column, const QModelIndex & idx);
impl /*struct*/ QAbstractListModel {
  pub fn sibling<RetType, T: QAbstractListModel_sibling<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.sibling(self);
    // return 1;
  }
}

pub trait QAbstractListModel_sibling<RetType> {
  fn sibling(self , rsthis: & QAbstractListModel) -> RetType;
}

  // proto:  QModelIndex QAbstractListModel::sibling(int row, int column, const QModelIndex & idx);
impl<'a> /*trait*/ QAbstractListModel_sibling<QModelIndex> for (i32, i32, &'a QModelIndex) {
  fn sibling(self , rsthis: & QAbstractListModel) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QAbstractListModel7siblingEiiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK18QAbstractListModel7siblingEiiRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QModelIndex::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QAbstractListModel::QAbstractListModel(QObject * parent);
impl /*struct*/ QAbstractListModel {
  pub fn New<T: QAbstractListModel_New>(value: T) -> QAbstractListModel {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QAbstractListModel_New {
  fn New(self) -> QAbstractListModel;
}

  // proto:  void QAbstractListModel::QAbstractListModel(QObject * parent);
impl<'a> /*trait*/ QAbstractListModel_New for (&'a QObject) {
  fn New(self) -> QAbstractListModel {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QAbstractListModelC1EP7QObject()};
    let ctysz: c_int = unsafe{QAbstractListModel_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN18QAbstractListModelC1EP7QObject(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN18QAbstractListModelC1EP7QObject(arg0)} as u64;
    let rsthis = QAbstractListModel{qbase: QAbstractItemModel::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QMetaObject * QAbstractListModel::metaObject();
impl /*struct*/ QAbstractListModel {
  pub fn metaObject<RetType, T: QAbstractListModel_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QAbstractListModel_metaObject<RetType> {
  fn metaObject(self , rsthis: & QAbstractListModel) -> RetType;
}

  // proto:  const QMetaObject * QAbstractListModel::metaObject();
impl<'a> /*trait*/ QAbstractListModel_metaObject<()> for () {
  fn metaObject(self , rsthis: & QAbstractListModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QAbstractListModel10metaObjectEv()};
     unsafe {_ZNK18QAbstractListModel10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QAbstractListModel::QAbstractListModel(const QAbstractListModel & );
impl<'a> /*trait*/ QAbstractListModel_New for (&'a QAbstractListModel) {
  fn New(self) -> QAbstractListModel {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QAbstractListModelC1ERKS_()};
    let ctysz: c_int = unsafe{QAbstractListModel_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN18QAbstractListModelC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN18QAbstractListModelC1ERKS_(arg0)} as u64;
    let rsthis = QAbstractListModel{qbase: QAbstractItemModel::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QModelIndex QAbstractListModel::index(int row, int column, const QModelIndex & parent);
impl /*struct*/ QAbstractListModel {
  pub fn index<RetType, T: QAbstractListModel_index<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.index(self);
    // return 1;
  }
}

pub trait QAbstractListModel_index<RetType> {
  fn index(self , rsthis: & QAbstractListModel) -> RetType;
}

  // proto:  QModelIndex QAbstractListModel::index(int row, int column, const QModelIndex & parent);
impl<'a> /*trait*/ QAbstractListModel_index<QModelIndex> for (i32, i32, &'a QModelIndex) {
  fn index(self , rsthis: & QAbstractListModel) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QAbstractListModel5indexEiiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK18QAbstractListModel5indexEiiRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QModelIndex::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QAbstractListModel::~QAbstractListModel();
impl /*struct*/ QAbstractListModel {
  pub fn Free<RetType, T: QAbstractListModel_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QAbstractListModel_Free<RetType> {
  fn Free(self , rsthis: & QAbstractListModel) -> RetType;
}

  // proto:  void QAbstractListModel::~QAbstractListModel();
impl<'a> /*trait*/ QAbstractListModel_Free<()> for () {
  fn Free(self , rsthis: & QAbstractListModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QAbstractListModelD0Ev()};
     unsafe {_ZN18QAbstractListModelD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

#[derive(Default)] // for QAbstractItemModel_modelReset
pub struct QAbstractItemModel_modelReset_signal{poi:u64}
impl /* struct */ QAbstractItemModel {
  pub fn modelReset_1(self) -> QAbstractItemModel_modelReset_signal {
     return QAbstractItemModel_modelReset_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QAbstractItemModel_modelReset_signal {
  pub fn connect<T: QAbstractItemModel_modelReset_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QAbstractItemModel_modelReset_signal_connect {
  fn connect(self, sigthis: QAbstractItemModel_modelReset_signal);
}

#[derive(Default)] // for QAbstractItemModel_columnsAboutToBeMoved
pub struct QAbstractItemModel_columnsAboutToBeMoved_signal{poi:u64}
impl /* struct */ QAbstractItemModel {
  pub fn columnsAboutToBeMoved_1(self) -> QAbstractItemModel_columnsAboutToBeMoved_signal {
     return QAbstractItemModel_columnsAboutToBeMoved_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QAbstractItemModel_columnsAboutToBeMoved_signal {
  pub fn connect<T: QAbstractItemModel_columnsAboutToBeMoved_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QAbstractItemModel_columnsAboutToBeMoved_signal_connect {
  fn connect(self, sigthis: QAbstractItemModel_columnsAboutToBeMoved_signal);
}

#[derive(Default)] // for QAbstractItemModel_columnsInserted
pub struct QAbstractItemModel_columnsInserted_signal{poi:u64}
impl /* struct */ QAbstractItemModel {
  pub fn columnsInserted_1(self) -> QAbstractItemModel_columnsInserted_signal {
     return QAbstractItemModel_columnsInserted_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QAbstractItemModel_columnsInserted_signal {
  pub fn connect<T: QAbstractItemModel_columnsInserted_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QAbstractItemModel_columnsInserted_signal_connect {
  fn connect(self, sigthis: QAbstractItemModel_columnsInserted_signal);
}

#[derive(Default)] // for QAbstractItemModel_rowsAboutToBeRemoved
pub struct QAbstractItemModel_rowsAboutToBeRemoved_signal{poi:u64}
impl /* struct */ QAbstractItemModel {
  pub fn rowsAboutToBeRemoved_1(self) -> QAbstractItemModel_rowsAboutToBeRemoved_signal {
     return QAbstractItemModel_rowsAboutToBeRemoved_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QAbstractItemModel_rowsAboutToBeRemoved_signal {
  pub fn connect<T: QAbstractItemModel_rowsAboutToBeRemoved_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QAbstractItemModel_rowsAboutToBeRemoved_signal_connect {
  fn connect(self, sigthis: QAbstractItemModel_rowsAboutToBeRemoved_signal);
}

#[derive(Default)] // for QAbstractItemModel_columnsAboutToBeRemoved
pub struct QAbstractItemModel_columnsAboutToBeRemoved_signal{poi:u64}
impl /* struct */ QAbstractItemModel {
  pub fn columnsAboutToBeRemoved_1(self) -> QAbstractItemModel_columnsAboutToBeRemoved_signal {
     return QAbstractItemModel_columnsAboutToBeRemoved_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QAbstractItemModel_columnsAboutToBeRemoved_signal {
  pub fn connect<T: QAbstractItemModel_columnsAboutToBeRemoved_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QAbstractItemModel_columnsAboutToBeRemoved_signal_connect {
  fn connect(self, sigthis: QAbstractItemModel_columnsAboutToBeRemoved_signal);
}

#[derive(Default)] // for QAbstractItemModel_rowsAboutToBeMoved
pub struct QAbstractItemModel_rowsAboutToBeMoved_signal{poi:u64}
impl /* struct */ QAbstractItemModel {
  pub fn rowsAboutToBeMoved_1(self) -> QAbstractItemModel_rowsAboutToBeMoved_signal {
     return QAbstractItemModel_rowsAboutToBeMoved_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QAbstractItemModel_rowsAboutToBeMoved_signal {
  pub fn connect<T: QAbstractItemModel_rowsAboutToBeMoved_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QAbstractItemModel_rowsAboutToBeMoved_signal_connect {
  fn connect(self, sigthis: QAbstractItemModel_rowsAboutToBeMoved_signal);
}

#[derive(Default)] // for QAbstractItemModel_layoutChanged
pub struct QAbstractItemModel_layoutChanged_signal{poi:u64}
impl /* struct */ QAbstractItemModel {
  pub fn layoutChanged_1(self) -> QAbstractItemModel_layoutChanged_signal {
     return QAbstractItemModel_layoutChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QAbstractItemModel_layoutChanged_signal {
  pub fn connect<T: QAbstractItemModel_layoutChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QAbstractItemModel_layoutChanged_signal_connect {
  fn connect(self, sigthis: QAbstractItemModel_layoutChanged_signal);
}

#[derive(Default)] // for QAbstractItemModel_columnsRemoved
pub struct QAbstractItemModel_columnsRemoved_signal{poi:u64}
impl /* struct */ QAbstractItemModel {
  pub fn columnsRemoved_1(self) -> QAbstractItemModel_columnsRemoved_signal {
     return QAbstractItemModel_columnsRemoved_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QAbstractItemModel_columnsRemoved_signal {
  pub fn connect<T: QAbstractItemModel_columnsRemoved_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QAbstractItemModel_columnsRemoved_signal_connect {
  fn connect(self, sigthis: QAbstractItemModel_columnsRemoved_signal);
}

#[derive(Default)] // for QAbstractItemModel_rowsInserted
pub struct QAbstractItemModel_rowsInserted_signal{poi:u64}
impl /* struct */ QAbstractItemModel {
  pub fn rowsInserted_1(self) -> QAbstractItemModel_rowsInserted_signal {
     return QAbstractItemModel_rowsInserted_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QAbstractItemModel_rowsInserted_signal {
  pub fn connect<T: QAbstractItemModel_rowsInserted_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QAbstractItemModel_rowsInserted_signal_connect {
  fn connect(self, sigthis: QAbstractItemModel_rowsInserted_signal);
}

#[derive(Default)] // for QAbstractItemModel_columnsAboutToBeInserted
pub struct QAbstractItemModel_columnsAboutToBeInserted_signal{poi:u64}
impl /* struct */ QAbstractItemModel {
  pub fn columnsAboutToBeInserted_1(self) -> QAbstractItemModel_columnsAboutToBeInserted_signal {
     return QAbstractItemModel_columnsAboutToBeInserted_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QAbstractItemModel_columnsAboutToBeInserted_signal {
  pub fn connect<T: QAbstractItemModel_columnsAboutToBeInserted_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QAbstractItemModel_columnsAboutToBeInserted_signal_connect {
  fn connect(self, sigthis: QAbstractItemModel_columnsAboutToBeInserted_signal);
}

#[derive(Default)] // for QAbstractItemModel_layoutAboutToBeChanged
pub struct QAbstractItemModel_layoutAboutToBeChanged_signal{poi:u64}
impl /* struct */ QAbstractItemModel {
  pub fn layoutAboutToBeChanged_1(self) -> QAbstractItemModel_layoutAboutToBeChanged_signal {
     return QAbstractItemModel_layoutAboutToBeChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QAbstractItemModel_layoutAboutToBeChanged_signal {
  pub fn connect<T: QAbstractItemModel_layoutAboutToBeChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QAbstractItemModel_layoutAboutToBeChanged_signal_connect {
  fn connect(self, sigthis: QAbstractItemModel_layoutAboutToBeChanged_signal);
}

#[derive(Default)] // for QAbstractItemModel_rowsRemoved
pub struct QAbstractItemModel_rowsRemoved_signal{poi:u64}
impl /* struct */ QAbstractItemModel {
  pub fn rowsRemoved_1(self) -> QAbstractItemModel_rowsRemoved_signal {
     return QAbstractItemModel_rowsRemoved_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QAbstractItemModel_rowsRemoved_signal {
  pub fn connect<T: QAbstractItemModel_rowsRemoved_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QAbstractItemModel_rowsRemoved_signal_connect {
  fn connect(self, sigthis: QAbstractItemModel_rowsRemoved_signal);
}

#[derive(Default)] // for QAbstractItemModel_rowsMoved
pub struct QAbstractItemModel_rowsMoved_signal{poi:u64}
impl /* struct */ QAbstractItemModel {
  pub fn rowsMoved_1(self) -> QAbstractItemModel_rowsMoved_signal {
     return QAbstractItemModel_rowsMoved_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QAbstractItemModel_rowsMoved_signal {
  pub fn connect<T: QAbstractItemModel_rowsMoved_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QAbstractItemModel_rowsMoved_signal_connect {
  fn connect(self, sigthis: QAbstractItemModel_rowsMoved_signal);
}

#[derive(Default)] // for QAbstractItemModel_headerDataChanged
pub struct QAbstractItemModel_headerDataChanged_signal{poi:u64}
impl /* struct */ QAbstractItemModel {
  pub fn headerDataChanged_1(self) -> QAbstractItemModel_headerDataChanged_signal {
     return QAbstractItemModel_headerDataChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QAbstractItemModel_headerDataChanged_signal {
  pub fn connect<T: QAbstractItemModel_headerDataChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QAbstractItemModel_headerDataChanged_signal_connect {
  fn connect(self, sigthis: QAbstractItemModel_headerDataChanged_signal);
}

#[derive(Default)] // for QAbstractItemModel_columnsMoved
pub struct QAbstractItemModel_columnsMoved_signal{poi:u64}
impl /* struct */ QAbstractItemModel {
  pub fn columnsMoved_1(self) -> QAbstractItemModel_columnsMoved_signal {
     return QAbstractItemModel_columnsMoved_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QAbstractItemModel_columnsMoved_signal {
  pub fn connect<T: QAbstractItemModel_columnsMoved_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QAbstractItemModel_columnsMoved_signal_connect {
  fn connect(self, sigthis: QAbstractItemModel_columnsMoved_signal);
}

#[derive(Default)] // for QAbstractItemModel_rowsAboutToBeInserted
pub struct QAbstractItemModel_rowsAboutToBeInserted_signal{poi:u64}
impl /* struct */ QAbstractItemModel {
  pub fn rowsAboutToBeInserted_1(self) -> QAbstractItemModel_rowsAboutToBeInserted_signal {
     return QAbstractItemModel_rowsAboutToBeInserted_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QAbstractItemModel_rowsAboutToBeInserted_signal {
  pub fn connect<T: QAbstractItemModel_rowsAboutToBeInserted_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QAbstractItemModel_rowsAboutToBeInserted_signal_connect {
  fn connect(self, sigthis: QAbstractItemModel_rowsAboutToBeInserted_signal);
}

#[derive(Default)] // for QAbstractItemModel_modelAboutToBeReset
pub struct QAbstractItemModel_modelAboutToBeReset_signal{poi:u64}
impl /* struct */ QAbstractItemModel {
  pub fn modelAboutToBeReset_1(self) -> QAbstractItemModel_modelAboutToBeReset_signal {
     return QAbstractItemModel_modelAboutToBeReset_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QAbstractItemModel_modelAboutToBeReset_signal {
  pub fn connect<T: QAbstractItemModel_modelAboutToBeReset_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QAbstractItemModel_modelAboutToBeReset_signal_connect {
  fn connect(self, sigthis: QAbstractItemModel_modelAboutToBeReset_signal);
}

#[derive(Default)] // for QAbstractItemModel_dataChanged
pub struct QAbstractItemModel_dataChanged_signal{poi:u64}
impl /* struct */ QAbstractItemModel {
  pub fn dataChanged_1(self) -> QAbstractItemModel_dataChanged_signal {
     return QAbstractItemModel_dataChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QAbstractItemModel_dataChanged_signal {
  pub fn connect<T: QAbstractItemModel_dataChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QAbstractItemModel_dataChanged_signal_connect {
  fn connect(self, sigthis: QAbstractItemModel_dataChanged_signal);
}

// headerDataChanged(Qt::Orientation, int, int)
extern fn QAbstractItemModel_headerDataChanged_signal_connect_cb_0(arg0: c_int, arg1: c_int, arg2: c_int) {
  println!("{}:{}", file!(), line!());
}
impl /* trait */ QAbstractItemModel_headerDataChanged_signal_connect for (extern fn(i32, i32, i32)) {
  fn connect(self, sigthis: QAbstractItemModel_headerDataChanged_signal) {
    // do smth...
    unsafe {QAbstractItemModel_SlotProxy_connect__ZN18QAbstractItemModel17headerDataChangedEN2Qt11OrientationEii(sigthis.poi as *mut c_void, QAbstractItemModel_headerDataChanged_signal_connect_cb_0 as *mut c_void)};
  }
}
// <= body block end

