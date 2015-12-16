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

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  bool QIdentityProxyModel::removeRows(int row, int count, const QModelIndex & parent);
  fn _ZN19QIdentityProxyModel10removeRowsEiiRK11QModelIndex(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_void) -> int8_t;
  // proto:  bool QIdentityProxyModel::removeColumns(int column, int count, const QModelIndex & parent);
  fn _ZN19QIdentityProxyModel13removeColumnsEiiRK11QModelIndex(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_void) -> int8_t;
  // proto:  QItemSelection QIdentityProxyModel::mapSelectionFromSource(const QItemSelection & selection);
  fn _ZNK19QIdentityProxyModel22mapSelectionFromSourceERK14QItemSelection(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QModelIndex QIdentityProxyModel::index(int row, int column, const QModelIndex & parent);
  fn _ZNK19QIdentityProxyModel5indexEiiRK11QModelIndex(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_void) -> *mut c_void;
  // proto:  bool QIdentityProxyModel::insertRows(int row, int count, const QModelIndex & parent);
  fn _ZN19QIdentityProxyModel10insertRowsEiiRK11QModelIndex(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_void) -> int8_t;
  // proto:  bool QIdentityProxyModel::insertColumns(int column, int count, const QModelIndex & parent);
  fn _ZN19QIdentityProxyModel13insertColumnsEiiRK11QModelIndex(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_void) -> int8_t;
  // proto:  QModelIndex QIdentityProxyModel::sibling(int row, int column, const QModelIndex & idx);
  fn _ZNK19QIdentityProxyModel7siblingEiiRK11QModelIndex(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_void) -> *mut c_void;
  // proto:  const QMetaObject * QIdentityProxyModel::metaObject();
  fn _ZNK19QIdentityProxyModel10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QIdentityProxyModel::FreeQIdentityProxyModel();
  fn _ZN19QIdentityProxyModelD0Ev(qthis: *mut c_void) ;
  // proto:  QModelIndex QIdentityProxyModel::parent(const QModelIndex & child);
  fn _ZNK19QIdentityProxyModel6parentERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QModelIndex QIdentityProxyModel::mapToSource(const QModelIndex & proxyIndex);
  fn _ZNK19QIdentityProxyModel11mapToSourceERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QModelIndex QIdentityProxyModel::mapFromSource(const QModelIndex & sourceIndex);
  fn _ZNK19QIdentityProxyModel13mapFromSourceERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QIdentityProxyModel::NewQIdentityProxyModel(QObject * parent);
  fn _ZN19QIdentityProxyModelC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QIdentityProxyModel::columnCount(const QModelIndex & parent);
  fn _ZNK19QIdentityProxyModel11columnCountERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto:  void QIdentityProxyModel::NewQIdentityProxyModel(const QIdentityProxyModel & );
  fn _ZN19QIdentityProxyModelC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
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

impl /*struct*/ QIdentityProxyModel {
  pub fn removeRows<T: QIdentityProxyModel_removeRows>(&mut self, value: T) -> i8 {
    return value.removeRows(self);
    // return 1;
  }
}

pub trait QIdentityProxyModel_removeRows {
  fn removeRows(self, rsthis: &mut QIdentityProxyModel) -> i8;
}

// proto:  bool QIdentityProxyModel::removeRows(int row, int count, const QModelIndex & parent);
impl<'a> /*trait*/ QIdentityProxyModel_removeRows for (i32, i32, &'a  QModelIndex) {
  fn removeRows(self, rsthis: &mut QIdentityProxyModel) -> i8 {
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

impl /*struct*/ QIdentityProxyModel {
  pub fn removeColumns<T: QIdentityProxyModel_removeColumns>(&mut self, value: T) -> i8 {
    return value.removeColumns(self);
    // return 1;
  }
}

pub trait QIdentityProxyModel_removeColumns {
  fn removeColumns(self, rsthis: &mut QIdentityProxyModel) -> i8;
}

// proto:  bool QIdentityProxyModel::removeColumns(int column, int count, const QModelIndex & parent);
impl<'a> /*trait*/ QIdentityProxyModel_removeColumns for (i32, i32, &'a  QModelIndex) {
  fn removeColumns(self, rsthis: &mut QIdentityProxyModel) -> i8 {
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

impl /*struct*/ QIdentityProxyModel {
  pub fn mapSelectionFromSource<T: QIdentityProxyModel_mapSelectionFromSource>(&mut self, value: T) -> QItemSelection {
    return value.mapSelectionFromSource(self);
    // return 1;
  }
}

pub trait QIdentityProxyModel_mapSelectionFromSource {
  fn mapSelectionFromSource(self, rsthis: &mut QIdentityProxyModel) -> QItemSelection;
}

// proto:  QItemSelection QIdentityProxyModel::mapSelectionFromSource(const QItemSelection & selection);
impl<'a> /*trait*/ QIdentityProxyModel_mapSelectionFromSource for (&'a  QItemSelection) {
  fn mapSelectionFromSource(self, rsthis: &mut QIdentityProxyModel) -> QItemSelection {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QIdentityProxyModel22mapSelectionFromSourceERK14QItemSelection()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK19QIdentityProxyModel22mapSelectionFromSourceERK14QItemSelection(rsthis.qclsinst, arg0)};
    let mut ret1 = QItemSelection{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QIdentityProxyModel {
  pub fn index<T: QIdentityProxyModel_index>(&mut self, value: T) -> QModelIndex {
    return value.index(self);
    // return 1;
  }
}

pub trait QIdentityProxyModel_index {
  fn index(self, rsthis: &mut QIdentityProxyModel) -> QModelIndex;
}

// proto:  QModelIndex QIdentityProxyModel::index(int row, int column, const QModelIndex & parent);
impl<'a> /*trait*/ QIdentityProxyModel_index for (i32, i32, &'a  QModelIndex) {
  fn index(self, rsthis: &mut QIdentityProxyModel) -> QModelIndex {
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

impl /*struct*/ QIdentityProxyModel {
  pub fn insertRows<T: QIdentityProxyModel_insertRows>(&mut self, value: T) -> i8 {
    return value.insertRows(self);
    // return 1;
  }
}

pub trait QIdentityProxyModel_insertRows {
  fn insertRows(self, rsthis: &mut QIdentityProxyModel) -> i8;
}

// proto:  bool QIdentityProxyModel::insertRows(int row, int count, const QModelIndex & parent);
impl<'a> /*trait*/ QIdentityProxyModel_insertRows for (i32, i32, &'a  QModelIndex) {
  fn insertRows(self, rsthis: &mut QIdentityProxyModel) -> i8 {
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

impl /*struct*/ QIdentityProxyModel {
  pub fn insertColumns<T: QIdentityProxyModel_insertColumns>(&mut self, value: T) -> i8 {
    return value.insertColumns(self);
    // return 1;
  }
}

pub trait QIdentityProxyModel_insertColumns {
  fn insertColumns(self, rsthis: &mut QIdentityProxyModel) -> i8;
}

// proto:  bool QIdentityProxyModel::insertColumns(int column, int count, const QModelIndex & parent);
impl<'a> /*trait*/ QIdentityProxyModel_insertColumns for (i32, i32, &'a  QModelIndex) {
  fn insertColumns(self, rsthis: &mut QIdentityProxyModel) -> i8 {
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

impl /*struct*/ QIdentityProxyModel {
  pub fn sibling<T: QIdentityProxyModel_sibling>(&mut self, value: T) -> QModelIndex {
    return value.sibling(self);
    // return 1;
  }
}

pub trait QIdentityProxyModel_sibling {
  fn sibling(self, rsthis: &mut QIdentityProxyModel) -> QModelIndex;
}

// proto:  QModelIndex QIdentityProxyModel::sibling(int row, int column, const QModelIndex & idx);
impl<'a> /*trait*/ QIdentityProxyModel_sibling for (i32, i32, &'a  QModelIndex) {
  fn sibling(self, rsthis: &mut QIdentityProxyModel) -> QModelIndex {
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

impl /*struct*/ QIdentityProxyModel {
  pub fn metaObject<T: QIdentityProxyModel_metaObject>(&mut self, value: T)  {
     value.metaObject(self);
    // return 1;
  }
}

pub trait QIdentityProxyModel_metaObject {
  fn metaObject(self, rsthis: &mut QIdentityProxyModel) ;
}

// proto:  const QMetaObject * QIdentityProxyModel::metaObject();
impl<'a> /*trait*/ QIdentityProxyModel_metaObject for () {
  fn metaObject(self, rsthis: &mut QIdentityProxyModel)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QIdentityProxyModel10metaObjectEv()};
     unsafe {_ZNK19QIdentityProxyModel10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QIdentityProxyModel {
  pub fn FreeQIdentityProxyModel<T: QIdentityProxyModel_FreeQIdentityProxyModel>(&mut self, value: T)  {
     value.FreeQIdentityProxyModel(self);
    // return 1;
  }
}

pub trait QIdentityProxyModel_FreeQIdentityProxyModel {
  fn FreeQIdentityProxyModel(self, rsthis: &mut QIdentityProxyModel) ;
}

// proto:  void QIdentityProxyModel::FreeQIdentityProxyModel();
impl<'a> /*trait*/ QIdentityProxyModel_FreeQIdentityProxyModel for () {
  fn FreeQIdentityProxyModel(self, rsthis: &mut QIdentityProxyModel)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QIdentityProxyModelD0Ev()};
     unsafe {_ZN19QIdentityProxyModelD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QIdentityProxyModel {
  pub fn parent<T: QIdentityProxyModel_parent>(&mut self, value: T) -> QModelIndex {
    return value.parent(self);
    // return 1;
  }
}

pub trait QIdentityProxyModel_parent {
  fn parent(self, rsthis: &mut QIdentityProxyModel) -> QModelIndex;
}

// proto:  QModelIndex QIdentityProxyModel::parent(const QModelIndex & child);
impl<'a> /*trait*/ QIdentityProxyModel_parent for (&'a  QModelIndex) {
  fn parent(self, rsthis: &mut QIdentityProxyModel) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QIdentityProxyModel6parentERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK19QIdentityProxyModel6parentERK11QModelIndex(rsthis.qclsinst, arg0)};
    let mut ret1 = QModelIndex{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QIdentityProxyModel {
  pub fn mapToSource<T: QIdentityProxyModel_mapToSource>(&mut self, value: T) -> QModelIndex {
    return value.mapToSource(self);
    // return 1;
  }
}

pub trait QIdentityProxyModel_mapToSource {
  fn mapToSource(self, rsthis: &mut QIdentityProxyModel) -> QModelIndex;
}

// proto:  QModelIndex QIdentityProxyModel::mapToSource(const QModelIndex & proxyIndex);
impl<'a> /*trait*/ QIdentityProxyModel_mapToSource for (&'a  QModelIndex) {
  fn mapToSource(self, rsthis: &mut QIdentityProxyModel) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QIdentityProxyModel11mapToSourceERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK19QIdentityProxyModel11mapToSourceERK11QModelIndex(rsthis.qclsinst, arg0)};
    let mut ret1 = QModelIndex{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QIdentityProxyModel {
  pub fn mapFromSource<T: QIdentityProxyModel_mapFromSource>(&mut self, value: T) -> QModelIndex {
    return value.mapFromSource(self);
    // return 1;
  }
}

pub trait QIdentityProxyModel_mapFromSource {
  fn mapFromSource(self, rsthis: &mut QIdentityProxyModel) -> QModelIndex;
}

// proto:  QModelIndex QIdentityProxyModel::mapFromSource(const QModelIndex & sourceIndex);
impl<'a> /*trait*/ QIdentityProxyModel_mapFromSource for (&'a  QModelIndex) {
  fn mapFromSource(self, rsthis: &mut QIdentityProxyModel) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QIdentityProxyModel13mapFromSourceERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK19QIdentityProxyModel13mapFromSourceERK11QModelIndex(rsthis.qclsinst, arg0)};
    let mut ret1 = QModelIndex{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

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

// proto: void QIdentityProxyModel::NewQIdentityProxyModel(QObject * parent);
impl<'a> /*trait*/ QIdentityProxyModel_NewQIdentityProxyModel for (&'a mut QObject) {
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

impl /*struct*/ QIdentityProxyModel {
  pub fn columnCount<T: QIdentityProxyModel_columnCount>(&mut self, value: T) -> i32 {
    return value.columnCount(self);
    // return 1;
  }
}

pub trait QIdentityProxyModel_columnCount {
  fn columnCount(self, rsthis: &mut QIdentityProxyModel) -> i32;
}

// proto:  int QIdentityProxyModel::columnCount(const QModelIndex & parent);
impl<'a> /*trait*/ QIdentityProxyModel_columnCount for (&'a  QModelIndex) {
  fn columnCount(self, rsthis: &mut QIdentityProxyModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QIdentityProxyModel11columnCountERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK19QIdentityProxyModel11columnCountERK11QModelIndex(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

// proto: void QIdentityProxyModel::NewQIdentityProxyModel(const QIdentityProxyModel & );
impl<'a> /*trait*/ QIdentityProxyModel_NewQIdentityProxyModel for (&'a  QIdentityProxyModel) {
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

impl /*struct*/ QIdentityProxyModel {
  pub fn mapSelectionToSource<T: QIdentityProxyModel_mapSelectionToSource>(&mut self, value: T) -> QItemSelection {
    return value.mapSelectionToSource(self);
    // return 1;
  }
}

pub trait QIdentityProxyModel_mapSelectionToSource {
  fn mapSelectionToSource(self, rsthis: &mut QIdentityProxyModel) -> QItemSelection;
}

// proto:  QItemSelection QIdentityProxyModel::mapSelectionToSource(const QItemSelection & selection);
impl<'a> /*trait*/ QIdentityProxyModel_mapSelectionToSource for (&'a  QItemSelection) {
  fn mapSelectionToSource(self, rsthis: &mut QIdentityProxyModel) -> QItemSelection {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QIdentityProxyModel20mapSelectionToSourceERK14QItemSelection()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK19QIdentityProxyModel20mapSelectionToSourceERK14QItemSelection(rsthis.qclsinst, arg0)};
    let mut ret1 = QItemSelection{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QIdentityProxyModel {
  pub fn rowCount<T: QIdentityProxyModel_rowCount>(&mut self, value: T) -> i32 {
    return value.rowCount(self);
    // return 1;
  }
}

pub trait QIdentityProxyModel_rowCount {
  fn rowCount(self, rsthis: &mut QIdentityProxyModel) -> i32;
}

// proto:  int QIdentityProxyModel::rowCount(const QModelIndex & parent);
impl<'a> /*trait*/ QIdentityProxyModel_rowCount for (&'a  QModelIndex) {
  fn rowCount(self, rsthis: &mut QIdentityProxyModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QIdentityProxyModel8rowCountERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK19QIdentityProxyModel8rowCountERK11QModelIndex(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

