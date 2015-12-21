// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qmodelindex::QModelIndex;
use super::qitemselection::QItemSelection;
use super::qobject::QObject;
use super::qvariant::QVariant;
use super::qmimedata::QMimeData;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  bool QIdentityProxyModel::removeRows(int row, int count, const QModelIndex & parent);
  fn _ZN19QIdentityProxyModel10removeRowsEiiRK11QModelIndex(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_void) -> c_char;
  // proto:  bool QIdentityProxyModel::removeColumns(int column, int count, const QModelIndex & parent);
  fn _ZN19QIdentityProxyModel13removeColumnsEiiRK11QModelIndex(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_void) -> c_char;
  // proto:  QItemSelection QIdentityProxyModel::mapSelectionFromSource(const QItemSelection & selection);
  fn _ZNK19QIdentityProxyModel22mapSelectionFromSourceERK14QItemSelection(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QModelIndex QIdentityProxyModel::index(int row, int column, const QModelIndex & parent);
  fn _ZNK19QIdentityProxyModel5indexEiiRK11QModelIndex(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_void) -> *mut c_void;
  // proto:  bool QIdentityProxyModel::insertRows(int row, int count, const QModelIndex & parent);
  fn _ZN19QIdentityProxyModel10insertRowsEiiRK11QModelIndex(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_void) -> c_char;
  // proto:  bool QIdentityProxyModel::insertColumns(int column, int count, const QModelIndex & parent);
  fn _ZN19QIdentityProxyModel13insertColumnsEiiRK11QModelIndex(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_void) -> c_char;
  // proto:  QModelIndex QIdentityProxyModel::sibling(int row, int column, const QModelIndex & idx);
  fn _ZNK19QIdentityProxyModel7siblingEiiRK11QModelIndex(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_void) -> *mut c_void;
  // proto:  const QMetaObject * QIdentityProxyModel::metaObject();
  fn _ZNK19QIdentityProxyModel10metaObjectEv(qthis: *mut c_void);
  // proto:  void QIdentityProxyModel::~QIdentityProxyModel();
  fn _ZN19QIdentityProxyModelD0Ev(qthis: *mut c_void);
  // proto:  QModelIndex QIdentityProxyModel::parent(const QModelIndex & child);
  fn _ZNK19QIdentityProxyModel6parentERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QModelIndex QIdentityProxyModel::mapToSource(const QModelIndex & proxyIndex);
  fn _ZNK19QIdentityProxyModel11mapToSourceERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QModelIndex QIdentityProxyModel::mapFromSource(const QModelIndex & sourceIndex);
  fn _ZNK19QIdentityProxyModel13mapFromSourceERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QIdentityProxyModel::QIdentityProxyModel(QObject * parent);
  fn _ZN19QIdentityProxyModelC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  int QIdentityProxyModel::columnCount(const QModelIndex & parent);
  fn _ZNK19QIdentityProxyModel11columnCountERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto:  void QIdentityProxyModel::QIdentityProxyModel(const QIdentityProxyModel & );
  fn _ZN19QIdentityProxyModelC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QItemSelection QIdentityProxyModel::mapSelectionToSource(const QItemSelection & selection);
  fn _ZNK19QIdentityProxyModel20mapSelectionToSourceERK14QItemSelection(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  int QIdentityProxyModel::rowCount(const QModelIndex & parent);
  fn _ZNK19QIdentityProxyModel8rowCountERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
}

// body block begin
// class sizeof(QIdentityProxyModel)=1
pub struct QIdentityProxyModel {
  pub qclsinst: *mut c_void,
}

  // proto:  bool QIdentityProxyModel::removeRows(int row, int count, const QModelIndex & parent);
impl /*struct*/ QIdentityProxyModel {
  pub fn removeRows<RetType, T: QIdentityProxyModel_removeRows<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.removeRows(self);
    // return 1;
  }
}

pub trait QIdentityProxyModel_removeRows<RetType> {
  fn removeRows(self , rsthis: &mut QIdentityProxyModel) -> RetType;
}

  // proto:  bool QIdentityProxyModel::removeRows(int row, int count, const QModelIndex & parent);
impl<'a> /*trait*/ QIdentityProxyModel_removeRows<i8> for (i32, i32, QModelIndex) {
  fn removeRows(self , rsthis: &mut QIdentityProxyModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QIdentityProxyModel10removeRowsEiiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN19QIdentityProxyModel10removeRowsEiiRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QIdentityProxyModel::removeColumns(int column, int count, const QModelIndex & parent);
impl /*struct*/ QIdentityProxyModel {
  pub fn removeColumns<RetType, T: QIdentityProxyModel_removeColumns<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.removeColumns(self);
    // return 1;
  }
}

pub trait QIdentityProxyModel_removeColumns<RetType> {
  fn removeColumns(self , rsthis: &mut QIdentityProxyModel) -> RetType;
}

  // proto:  bool QIdentityProxyModel::removeColumns(int column, int count, const QModelIndex & parent);
impl<'a> /*trait*/ QIdentityProxyModel_removeColumns<i8> for (i32, i32, QModelIndex) {
  fn removeColumns(self , rsthis: &mut QIdentityProxyModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QIdentityProxyModel13removeColumnsEiiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN19QIdentityProxyModel13removeColumnsEiiRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QItemSelection QIdentityProxyModel::mapSelectionFromSource(const QItemSelection & selection);
impl /*struct*/ QIdentityProxyModel {
  pub fn mapSelectionFromSource<RetType, T: QIdentityProxyModel_mapSelectionFromSource<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.mapSelectionFromSource(self);
    // return 1;
  }
}

pub trait QIdentityProxyModel_mapSelectionFromSource<RetType> {
  fn mapSelectionFromSource(self , rsthis: &mut QIdentityProxyModel) -> RetType;
}

  // proto:  QItemSelection QIdentityProxyModel::mapSelectionFromSource(const QItemSelection & selection);
impl<'a> /*trait*/ QIdentityProxyModel_mapSelectionFromSource<QItemSelection> for (QItemSelection) {
  fn mapSelectionFromSource(self , rsthis: &mut QIdentityProxyModel) -> QItemSelection {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QIdentityProxyModel22mapSelectionFromSourceERK14QItemSelection()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK19QIdentityProxyModel22mapSelectionFromSourceERK14QItemSelection(rsthis.qclsinst, arg0)};
    let mut ret1 = QItemSelection{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QModelIndex QIdentityProxyModel::index(int row, int column, const QModelIndex & parent);
impl /*struct*/ QIdentityProxyModel {
  pub fn index<RetType, T: QIdentityProxyModel_index<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.index(self);
    // return 1;
  }
}

pub trait QIdentityProxyModel_index<RetType> {
  fn index(self , rsthis: &mut QIdentityProxyModel) -> RetType;
}

  // proto:  QModelIndex QIdentityProxyModel::index(int row, int column, const QModelIndex & parent);
impl<'a> /*trait*/ QIdentityProxyModel_index<QModelIndex> for (i32, i32, QModelIndex) {
  fn index(self , rsthis: &mut QIdentityProxyModel) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QIdentityProxyModel5indexEiiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK19QIdentityProxyModel5indexEiiRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QModelIndex{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  bool QIdentityProxyModel::insertRows(int row, int count, const QModelIndex & parent);
impl /*struct*/ QIdentityProxyModel {
  pub fn insertRows<RetType, T: QIdentityProxyModel_insertRows<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.insertRows(self);
    // return 1;
  }
}

pub trait QIdentityProxyModel_insertRows<RetType> {
  fn insertRows(self , rsthis: &mut QIdentityProxyModel) -> RetType;
}

  // proto:  bool QIdentityProxyModel::insertRows(int row, int count, const QModelIndex & parent);
impl<'a> /*trait*/ QIdentityProxyModel_insertRows<i8> for (i32, i32, QModelIndex) {
  fn insertRows(self , rsthis: &mut QIdentityProxyModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QIdentityProxyModel10insertRowsEiiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN19QIdentityProxyModel10insertRowsEiiRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QIdentityProxyModel::insertColumns(int column, int count, const QModelIndex & parent);
impl /*struct*/ QIdentityProxyModel {
  pub fn insertColumns<RetType, T: QIdentityProxyModel_insertColumns<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.insertColumns(self);
    // return 1;
  }
}

pub trait QIdentityProxyModel_insertColumns<RetType> {
  fn insertColumns(self , rsthis: &mut QIdentityProxyModel) -> RetType;
}

  // proto:  bool QIdentityProxyModel::insertColumns(int column, int count, const QModelIndex & parent);
impl<'a> /*trait*/ QIdentityProxyModel_insertColumns<i8> for (i32, i32, QModelIndex) {
  fn insertColumns(self , rsthis: &mut QIdentityProxyModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QIdentityProxyModel13insertColumnsEiiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN19QIdentityProxyModel13insertColumnsEiiRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QModelIndex QIdentityProxyModel::sibling(int row, int column, const QModelIndex & idx);
impl /*struct*/ QIdentityProxyModel {
  pub fn sibling<RetType, T: QIdentityProxyModel_sibling<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.sibling(self);
    // return 1;
  }
}

pub trait QIdentityProxyModel_sibling<RetType> {
  fn sibling(self , rsthis: &mut QIdentityProxyModel) -> RetType;
}

  // proto:  QModelIndex QIdentityProxyModel::sibling(int row, int column, const QModelIndex & idx);
impl<'a> /*trait*/ QIdentityProxyModel_sibling<QModelIndex> for (i32, i32, QModelIndex) {
  fn sibling(self , rsthis: &mut QIdentityProxyModel) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QIdentityProxyModel7siblingEiiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK19QIdentityProxyModel7siblingEiiRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QModelIndex{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  const QMetaObject * QIdentityProxyModel::metaObject();
impl /*struct*/ QIdentityProxyModel {
  pub fn metaObject<RetType, T: QIdentityProxyModel_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QIdentityProxyModel_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QIdentityProxyModel) -> RetType;
}

  // proto:  const QMetaObject * QIdentityProxyModel::metaObject();
impl<'a> /*trait*/ QIdentityProxyModel_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QIdentityProxyModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QIdentityProxyModel10metaObjectEv()};
     unsafe {_ZNK19QIdentityProxyModel10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QIdentityProxyModel::~QIdentityProxyModel();
impl /*struct*/ QIdentityProxyModel {
  pub fn FreeQIdentityProxyModel<RetType, T: QIdentityProxyModel_FreeQIdentityProxyModel<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQIdentityProxyModel(self);
    // return 1;
  }
}

pub trait QIdentityProxyModel_FreeQIdentityProxyModel<RetType> {
  fn FreeQIdentityProxyModel(self , rsthis: &mut QIdentityProxyModel) -> RetType;
}

  // proto:  void QIdentityProxyModel::~QIdentityProxyModel();
impl<'a> /*trait*/ QIdentityProxyModel_FreeQIdentityProxyModel<()> for () {
  fn FreeQIdentityProxyModel(self , rsthis: &mut QIdentityProxyModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QIdentityProxyModelD0Ev()};
     unsafe {_ZN19QIdentityProxyModelD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QModelIndex QIdentityProxyModel::parent(const QModelIndex & child);
impl /*struct*/ QIdentityProxyModel {
  pub fn parent<RetType, T: QIdentityProxyModel_parent<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.parent(self);
    // return 1;
  }
}

pub trait QIdentityProxyModel_parent<RetType> {
  fn parent(self , rsthis: &mut QIdentityProxyModel) -> RetType;
}

  // proto:  QModelIndex QIdentityProxyModel::parent(const QModelIndex & child);
impl<'a> /*trait*/ QIdentityProxyModel_parent<QModelIndex> for (QModelIndex) {
  fn parent(self , rsthis: &mut QIdentityProxyModel) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QIdentityProxyModel6parentERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK19QIdentityProxyModel6parentERK11QModelIndex(rsthis.qclsinst, arg0)};
    let mut ret1 = QModelIndex{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QModelIndex QIdentityProxyModel::mapToSource(const QModelIndex & proxyIndex);
impl /*struct*/ QIdentityProxyModel {
  pub fn mapToSource<RetType, T: QIdentityProxyModel_mapToSource<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.mapToSource(self);
    // return 1;
  }
}

pub trait QIdentityProxyModel_mapToSource<RetType> {
  fn mapToSource(self , rsthis: &mut QIdentityProxyModel) -> RetType;
}

  // proto:  QModelIndex QIdentityProxyModel::mapToSource(const QModelIndex & proxyIndex);
impl<'a> /*trait*/ QIdentityProxyModel_mapToSource<QModelIndex> for (QModelIndex) {
  fn mapToSource(self , rsthis: &mut QIdentityProxyModel) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QIdentityProxyModel11mapToSourceERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK19QIdentityProxyModel11mapToSourceERK11QModelIndex(rsthis.qclsinst, arg0)};
    let mut ret1 = QModelIndex{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QModelIndex QIdentityProxyModel::mapFromSource(const QModelIndex & sourceIndex);
impl /*struct*/ QIdentityProxyModel {
  pub fn mapFromSource<RetType, T: QIdentityProxyModel_mapFromSource<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.mapFromSource(self);
    // return 1;
  }
}

pub trait QIdentityProxyModel_mapFromSource<RetType> {
  fn mapFromSource(self , rsthis: &mut QIdentityProxyModel) -> RetType;
}

  // proto:  QModelIndex QIdentityProxyModel::mapFromSource(const QModelIndex & sourceIndex);
impl<'a> /*trait*/ QIdentityProxyModel_mapFromSource<QModelIndex> for (QModelIndex) {
  fn mapFromSource(self , rsthis: &mut QIdentityProxyModel) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QIdentityProxyModel13mapFromSourceERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK19QIdentityProxyModel13mapFromSourceERK11QModelIndex(rsthis.qclsinst, arg0)};
    let mut ret1 = QModelIndex{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QIdentityProxyModel::QIdentityProxyModel(QObject * parent);
impl /*struct*/ QIdentityProxyModel {
  pub fn NewQIdentityProxyModel<T: QIdentityProxyModel_NewQIdentityProxyModel>(value: T) -> QIdentityProxyModel {
    let rsthis = value.NewQIdentityProxyModel();
    return rsthis;
    // return 1;
  }
}

pub trait QIdentityProxyModel_NewQIdentityProxyModel {
  fn NewQIdentityProxyModel(self) -> QIdentityProxyModel;
}

  // proto:  void QIdentityProxyModel::QIdentityProxyModel(QObject * parent);
impl<'a> /*trait*/ QIdentityProxyModel_NewQIdentityProxyModel for (QObject) {
  fn NewQIdentityProxyModel(self) -> QIdentityProxyModel {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QIdentityProxyModelC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN19QIdentityProxyModelC1EP7QObject(qthis, arg0)};
    let rsthis = QIdentityProxyModel{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QIdentityProxyModel::columnCount(const QModelIndex & parent);
impl /*struct*/ QIdentityProxyModel {
  pub fn columnCount<RetType, T: QIdentityProxyModel_columnCount<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.columnCount(self);
    // return 1;
  }
}

pub trait QIdentityProxyModel_columnCount<RetType> {
  fn columnCount(self , rsthis: &mut QIdentityProxyModel) -> RetType;
}

  // proto:  int QIdentityProxyModel::columnCount(const QModelIndex & parent);
impl<'a> /*trait*/ QIdentityProxyModel_columnCount<i32> for (QModelIndex) {
  fn columnCount(self , rsthis: &mut QIdentityProxyModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QIdentityProxyModel11columnCountERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK19QIdentityProxyModel11columnCountERK11QModelIndex(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QIdentityProxyModel::QIdentityProxyModel(const QIdentityProxyModel & );
impl<'a> /*trait*/ QIdentityProxyModel_NewQIdentityProxyModel for (QIdentityProxyModel) {
  fn NewQIdentityProxyModel(self) -> QIdentityProxyModel {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QIdentityProxyModelC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN19QIdentityProxyModelC1ERKS_(qthis, arg0)};
    let rsthis = QIdentityProxyModel{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QItemSelection QIdentityProxyModel::mapSelectionToSource(const QItemSelection & selection);
impl /*struct*/ QIdentityProxyModel {
  pub fn mapSelectionToSource<RetType, T: QIdentityProxyModel_mapSelectionToSource<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.mapSelectionToSource(self);
    // return 1;
  }
}

pub trait QIdentityProxyModel_mapSelectionToSource<RetType> {
  fn mapSelectionToSource(self , rsthis: &mut QIdentityProxyModel) -> RetType;
}

  // proto:  QItemSelection QIdentityProxyModel::mapSelectionToSource(const QItemSelection & selection);
impl<'a> /*trait*/ QIdentityProxyModel_mapSelectionToSource<QItemSelection> for (QItemSelection) {
  fn mapSelectionToSource(self , rsthis: &mut QIdentityProxyModel) -> QItemSelection {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QIdentityProxyModel20mapSelectionToSourceERK14QItemSelection()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK19QIdentityProxyModel20mapSelectionToSourceERK14QItemSelection(rsthis.qclsinst, arg0)};
    let mut ret1 = QItemSelection{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  int QIdentityProxyModel::rowCount(const QModelIndex & parent);
impl /*struct*/ QIdentityProxyModel {
  pub fn rowCount<RetType, T: QIdentityProxyModel_rowCount<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.rowCount(self);
    // return 1;
  }
}

pub trait QIdentityProxyModel_rowCount<RetType> {
  fn rowCount(self , rsthis: &mut QIdentityProxyModel) -> RetType;
}

  // proto:  int QIdentityProxyModel::rowCount(const QModelIndex & parent);
impl<'a> /*trait*/ QIdentityProxyModel_rowCount<i32> for (QModelIndex) {
  fn rowCount(self , rsthis: &mut QIdentityProxyModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QIdentityProxyModel8rowCountERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK19QIdentityProxyModel8rowCountERK11QModelIndex(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

