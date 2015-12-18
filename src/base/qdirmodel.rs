// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qfileiconprovider::QFileIconProvider;
use super::qmodelindex::QModelIndex;
use super::qvariant::QVariant;
use super::qstring::QString;
use super::qstringlist::QStringList;
use super::qobject::QObject;
use super::qicon::QIcon;
use super::qfileinfo::QFileInfo;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  QFileIconProvider * QDirModel::iconProvider();
  fn _ZNK9QDirModel12iconProviderEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QModelIndex QDirModel::parent(const QModelIndex & child);
  fn _ZNK9QDirModel6parentERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QVariant QDirModel::data(const QModelIndex & index, int role);
  fn _ZNK9QDirModel4dataERK11QModelIndexi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) -> *mut c_void;
  // proto:  QStringList QDirModel::nameFilters();
  fn _ZNK9QDirModel11nameFiltersEv(qthis: *mut c_void) ;
  // proto:  bool QDirModel::isReadOnly();
  fn _ZNK9QDirModel10isReadOnlyEv(qthis: *mut c_void) -> int8_t;
  // proto:  int QDirModel::columnCount(const QModelIndex & parent);
  fn _ZNK9QDirModel11columnCountERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto:  QStringList QDirModel::mimeTypes();
  fn _ZNK9QDirModel9mimeTypesEv(qthis: *mut c_void) ;
  // proto:  void QDirModel::FreeQDirModel();
  fn _ZN9QDirModelD0Ev(qthis: *mut c_void) ;
  // proto:  bool QDirModel::remove(const QModelIndex & index);
  fn _ZN9QDirModel6removeERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  QString QDirModel::fileName(const QModelIndex & index);
  fn _ZNK9QDirModel8fileNameERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  const QMetaObject * QDirModel::metaObject();
  fn _ZNK9QDirModel10metaObjectEv(qthis: *mut c_void) ;
  // proto:  bool QDirModel::resolveSymlinks();
  fn _ZNK9QDirModel15resolveSymlinksEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QDirModel::refresh(const QModelIndex & parent);
  fn _ZN9QDirModel7refreshERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QDirModel::setNameFilters(const QStringList & filters);
  fn _ZN9QDirModel14setNameFiltersERK11QStringList(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QDirModel::setIconProvider(QFileIconProvider * provider);
  fn _ZN9QDirModel15setIconProviderEP17QFileIconProvider(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QModelIndex QDirModel::index(int row, int column, const QModelIndex & parent);
  fn _ZNK9QDirModel5indexEiiRK11QModelIndex(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_void) -> *mut c_void;
  // proto:  void QDirModel::NewQDirModel(QObject * parent);
  fn _ZN9QDirModelC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QModelIndex QDirModel::index(const QString & path, int column);
  fn _ZNK9QDirModel5indexERK7QStringi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) -> *mut c_void;
  // proto:  bool QDirModel::setData(const QModelIndex & index, const QVariant & value, int role);
  fn _ZN9QDirModel7setDataERK11QModelIndexRK8QVarianti(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: c_int) -> int8_t;
  // proto:  void QDirModel::setLazyChildCount(bool enable);
  fn _ZN9QDirModel17setLazyChildCountEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  QIcon QDirModel::fileIcon(const QModelIndex & index);
  fn _ZNK9QDirModel8fileIconERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QDirModel::hasChildren(const QModelIndex & index);
  fn _ZNK9QDirModel11hasChildrenERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  bool QDirModel::isDir(const QModelIndex & index);
  fn _ZNK9QDirModel5isDirERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  QModelIndex QDirModel::mkdir(const QModelIndex & parent, const QString & name);
  fn _ZN9QDirModel5mkdirERK11QModelIndexRK7QString(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  bool QDirModel::rmdir(const QModelIndex & index);
  fn _ZN9QDirModel5rmdirERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  QString QDirModel::filePath(const QModelIndex & index);
  fn _ZNK9QDirModel8filePathERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  int QDirModel::rowCount(const QModelIndex & parent);
  fn _ZNK9QDirModel8rowCountERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
  // proto:  void QDirModel::setReadOnly(bool enable);
  fn _ZN9QDirModel11setReadOnlyEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QDirModel::NewQDirModel(const QDirModel & );
  fn _ZN9QDirModelC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QDirModel::setResolveSymlinks(bool enable);
  fn _ZN9QDirModel18setResolveSymlinksEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  bool QDirModel::lazyChildCount();
  fn _ZNK9QDirModel14lazyChildCountEv(qthis: *mut c_void) -> int8_t;
  // proto:  QFileInfo QDirModel::fileInfo(const QModelIndex & index);
  fn _ZNK9QDirModel8fileInfoERK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
}

// body block begin
// class sizeof(QDirModel)=1
pub struct QDirModel {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QDirModel {
  pub fn iconProvider<RetType, T: QDirModel_iconProvider<RetType>>(&mut self, value: T) -> RetType {
    return value.iconProvider(self);
    // return 1;
  }
}

pub trait QDirModel_iconProvider<RetType> {
  fn iconProvider(self, rsthis: &mut QDirModel) -> RetType;
}

// proto:  QFileIconProvider * QDirModel::iconProvider();
impl<'a> /*trait*/ QDirModel_iconProvider<QFileIconProvider> for () {
  fn iconProvider(self, rsthis: &mut QDirModel) -> QFileIconProvider {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDirModel12iconProviderEv()};
    let mut ret = unsafe {_ZNK9QDirModel12iconProviderEv(rsthis.qclsinst)};
    let mut ret1 = QFileIconProvider{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDirModel {
  pub fn parent<RetType, T: QDirModel_parent<RetType>>(&mut self, value: T) -> RetType {
    return value.parent(self);
    // return 1;
  }
}

pub trait QDirModel_parent<RetType> {
  fn parent(self, rsthis: &mut QDirModel) -> RetType;
}

// proto:  QModelIndex QDirModel::parent(const QModelIndex & child);
impl<'a> /*trait*/ QDirModel_parent<QModelIndex> for (&'a  QModelIndex) {
  fn parent(self, rsthis: &mut QDirModel) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDirModel6parentERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QDirModel6parentERK11QModelIndex(rsthis.qclsinst, arg0)};
    let mut ret1 = QModelIndex{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDirModel {
  pub fn data<RetType, T: QDirModel_data<RetType>>(&mut self, value: T) -> RetType {
    return value.data(self);
    // return 1;
  }
}

pub trait QDirModel_data<RetType> {
  fn data(self, rsthis: &mut QDirModel) -> RetType;
}

// proto:  QVariant QDirModel::data(const QModelIndex & index, int role);
impl<'a> /*trait*/ QDirModel_data<QVariant> for (&'a  QModelIndex, i32) {
  fn data(self, rsthis: &mut QDirModel) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDirModel4dataERK11QModelIndexi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK9QDirModel4dataERK11QModelIndexi(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QVariant{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDirModel {
  pub fn nameFilters<RetType, T: QDirModel_nameFilters<RetType>>(&mut self, value: T) -> RetType {
    return value.nameFilters(self);
    // return 1;
  }
}

pub trait QDirModel_nameFilters<RetType> {
  fn nameFilters(self, rsthis: &mut QDirModel) -> RetType;
}

// proto:  QStringList QDirModel::nameFilters();
impl<'a> /*trait*/ QDirModel_nameFilters<()> for () {
  fn nameFilters(self, rsthis: &mut QDirModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDirModel11nameFiltersEv()};
     unsafe {_ZNK9QDirModel11nameFiltersEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QDirModel {
  pub fn isReadOnly<RetType, T: QDirModel_isReadOnly<RetType>>(&mut self, value: T) -> RetType {
    return value.isReadOnly(self);
    // return 1;
  }
}

pub trait QDirModel_isReadOnly<RetType> {
  fn isReadOnly(self, rsthis: &mut QDirModel) -> RetType;
}

// proto:  bool QDirModel::isReadOnly();
impl<'a> /*trait*/ QDirModel_isReadOnly<i8> for () {
  fn isReadOnly(self, rsthis: &mut QDirModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDirModel10isReadOnlyEv()};
    let mut ret = unsafe {_ZNK9QDirModel10isReadOnlyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QDirModel {
  pub fn columnCount<RetType, T: QDirModel_columnCount<RetType>>(&mut self, value: T) -> RetType {
    return value.columnCount(self);
    // return 1;
  }
}

pub trait QDirModel_columnCount<RetType> {
  fn columnCount(self, rsthis: &mut QDirModel) -> RetType;
}

// proto:  int QDirModel::columnCount(const QModelIndex & parent);
impl<'a> /*trait*/ QDirModel_columnCount<i32> for (&'a  QModelIndex) {
  fn columnCount(self, rsthis: &mut QDirModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDirModel11columnCountERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QDirModel11columnCountERK11QModelIndex(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QDirModel {
  pub fn mimeTypes<RetType, T: QDirModel_mimeTypes<RetType>>(&mut self, value: T) -> RetType {
    return value.mimeTypes(self);
    // return 1;
  }
}

pub trait QDirModel_mimeTypes<RetType> {
  fn mimeTypes(self, rsthis: &mut QDirModel) -> RetType;
}

// proto:  QStringList QDirModel::mimeTypes();
impl<'a> /*trait*/ QDirModel_mimeTypes<()> for () {
  fn mimeTypes(self, rsthis: &mut QDirModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDirModel9mimeTypesEv()};
     unsafe {_ZNK9QDirModel9mimeTypesEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QDirModel {
  pub fn FreeQDirModel<RetType, T: QDirModel_FreeQDirModel<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQDirModel(self);
    // return 1;
  }
}

pub trait QDirModel_FreeQDirModel<RetType> {
  fn FreeQDirModel(self, rsthis: &mut QDirModel) -> RetType;
}

// proto:  void QDirModel::FreeQDirModel();
impl<'a> /*trait*/ QDirModel_FreeQDirModel<()> for () {
  fn FreeQDirModel(self, rsthis: &mut QDirModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDirModelD0Ev()};
     unsafe {_ZN9QDirModelD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QDirModel {
  pub fn remove<RetType, T: QDirModel_remove<RetType>>(&mut self, value: T) -> RetType {
    return value.remove(self);
    // return 1;
  }
}

pub trait QDirModel_remove<RetType> {
  fn remove(self, rsthis: &mut QDirModel) -> RetType;
}

// proto:  bool QDirModel::remove(const QModelIndex & index);
impl<'a> /*trait*/ QDirModel_remove<i8> for (&'a  QModelIndex) {
  fn remove(self, rsthis: &mut QDirModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDirModel6removeERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN9QDirModel6removeERK11QModelIndex(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QDirModel {
  pub fn fileName<RetType, T: QDirModel_fileName<RetType>>(&mut self, value: T) -> RetType {
    return value.fileName(self);
    // return 1;
  }
}

pub trait QDirModel_fileName<RetType> {
  fn fileName(self, rsthis: &mut QDirModel) -> RetType;
}

// proto:  QString QDirModel::fileName(const QModelIndex & index);
impl<'a> /*trait*/ QDirModel_fileName<QString> for (&'a  QModelIndex) {
  fn fileName(self, rsthis: &mut QDirModel) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDirModel8fileNameERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QDirModel8fileNameERK11QModelIndex(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDirModel {
  pub fn metaObject<RetType, T: QDirModel_metaObject<RetType>>(&mut self, value: T) -> RetType {
    return value.metaObject(self);
    // return 1;
  }
}

pub trait QDirModel_metaObject<RetType> {
  fn metaObject(self, rsthis: &mut QDirModel) -> RetType;
}

// proto:  const QMetaObject * QDirModel::metaObject();
impl<'a> /*trait*/ QDirModel_metaObject<()> for () {
  fn metaObject(self, rsthis: &mut QDirModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDirModel10metaObjectEv()};
     unsafe {_ZNK9QDirModel10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QDirModel {
  pub fn resolveSymlinks<RetType, T: QDirModel_resolveSymlinks<RetType>>(&mut self, value: T) -> RetType {
    return value.resolveSymlinks(self);
    // return 1;
  }
}

pub trait QDirModel_resolveSymlinks<RetType> {
  fn resolveSymlinks(self, rsthis: &mut QDirModel) -> RetType;
}

// proto:  bool QDirModel::resolveSymlinks();
impl<'a> /*trait*/ QDirModel_resolveSymlinks<i8> for () {
  fn resolveSymlinks(self, rsthis: &mut QDirModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDirModel15resolveSymlinksEv()};
    let mut ret = unsafe {_ZNK9QDirModel15resolveSymlinksEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QDirModel {
  pub fn refresh<RetType, T: QDirModel_refresh<RetType>>(&mut self, value: T) -> RetType {
    return value.refresh(self);
    // return 1;
  }
}

pub trait QDirModel_refresh<RetType> {
  fn refresh(self, rsthis: &mut QDirModel) -> RetType;
}

// proto:  void QDirModel::refresh(const QModelIndex & parent);
impl<'a> /*trait*/ QDirModel_refresh<()> for (&'a  QModelIndex) {
  fn refresh(self, rsthis: &mut QDirModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDirModel7refreshERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QDirModel7refreshERK11QModelIndex(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QDirModel {
  pub fn setNameFilters<RetType, T: QDirModel_setNameFilters<RetType>>(&mut self, value: T) -> RetType {
    return value.setNameFilters(self);
    // return 1;
  }
}

pub trait QDirModel_setNameFilters<RetType> {
  fn setNameFilters(self, rsthis: &mut QDirModel) -> RetType;
}

// proto:  void QDirModel::setNameFilters(const QStringList & filters);
impl<'a> /*trait*/ QDirModel_setNameFilters<()> for (&'a  QStringList) {
  fn setNameFilters(self, rsthis: &mut QDirModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDirModel14setNameFiltersERK11QStringList()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QDirModel14setNameFiltersERK11QStringList(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QDirModel {
  pub fn setIconProvider<RetType, T: QDirModel_setIconProvider<RetType>>(&mut self, value: T) -> RetType {
    return value.setIconProvider(self);
    // return 1;
  }
}

pub trait QDirModel_setIconProvider<RetType> {
  fn setIconProvider(self, rsthis: &mut QDirModel) -> RetType;
}

// proto:  void QDirModel::setIconProvider(QFileIconProvider * provider);
impl<'a> /*trait*/ QDirModel_setIconProvider<()> for (&'a mut QFileIconProvider) {
  fn setIconProvider(self, rsthis: &mut QDirModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDirModel15setIconProviderEP17QFileIconProvider()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QDirModel15setIconProviderEP17QFileIconProvider(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QDirModel {
  pub fn index<RetType, T: QDirModel_index<RetType>>(&mut self, value: T) -> RetType {
    return value.index(self);
    // return 1;
  }
}

pub trait QDirModel_index<RetType> {
  fn index(self, rsthis: &mut QDirModel) -> RetType;
}

// proto:  QModelIndex QDirModel::index(int row, int column, const QModelIndex & parent);
impl<'a> /*trait*/ QDirModel_index<QModelIndex> for (i32, i32, &'a  QModelIndex) {
  fn index(self, rsthis: &mut QDirModel) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDirModel5indexEiiRK11QModelIndex()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QDirModel5indexEiiRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QModelIndex{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDirModel {
  pub fn NewQDirModel<T: QDirModel_NewQDirModel>(value: T) -> QDirModel {
    let rsthis = value.NewQDirModel();
    return rsthis;
    // return 1;
  }
}

pub trait QDirModel_NewQDirModel {
  fn NewQDirModel(self) -> QDirModel;
}

// proto: void QDirModel::NewQDirModel(QObject * parent);
impl<'a> /*trait*/ QDirModel_NewQDirModel for (&'a mut QObject) {
  fn NewQDirModel(self) -> QDirModel {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDirModelC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QDirModelC1EP7QObject(qthis, arg0)};
    let rsthis = QDirModel{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  QModelIndex QDirModel::index(const QString & path, int column);
impl<'a> /*trait*/ QDirModel_index<QModelIndex> for (&'a  QString, i32) {
  fn index(self, rsthis: &mut QDirModel) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDirModel5indexERK7QStringi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK9QDirModel5indexERK7QStringi(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QModelIndex{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDirModel {
  pub fn setData<RetType, T: QDirModel_setData<RetType>>(&mut self, value: T) -> RetType {
    return value.setData(self);
    // return 1;
  }
}

pub trait QDirModel_setData<RetType> {
  fn setData(self, rsthis: &mut QDirModel) -> RetType;
}

// proto:  bool QDirModel::setData(const QModelIndex & index, const QVariant & value, int role);
impl<'a> /*trait*/ QDirModel_setData<i8> for (&'a  QModelIndex, &'a  QVariant, i32) {
  fn setData(self, rsthis: &mut QDirModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDirModel7setDataERK11QModelIndexRK8QVarianti()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2  as c_int;
    let mut ret = unsafe {_ZN9QDirModel7setDataERK11QModelIndexRK8QVarianti(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QDirModel {
  pub fn setLazyChildCount<RetType, T: QDirModel_setLazyChildCount<RetType>>(&mut self, value: T) -> RetType {
    return value.setLazyChildCount(self);
    // return 1;
  }
}

pub trait QDirModel_setLazyChildCount<RetType> {
  fn setLazyChildCount(self, rsthis: &mut QDirModel) -> RetType;
}

// proto:  void QDirModel::setLazyChildCount(bool enable);
impl<'a> /*trait*/ QDirModel_setLazyChildCount<()> for (i8) {
  fn setLazyChildCount(self, rsthis: &mut QDirModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDirModel17setLazyChildCountEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN9QDirModel17setLazyChildCountEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QDirModel {
  pub fn fileIcon<RetType, T: QDirModel_fileIcon<RetType>>(&mut self, value: T) -> RetType {
    return value.fileIcon(self);
    // return 1;
  }
}

pub trait QDirModel_fileIcon<RetType> {
  fn fileIcon(self, rsthis: &mut QDirModel) -> RetType;
}

// proto:  QIcon QDirModel::fileIcon(const QModelIndex & index);
impl<'a> /*trait*/ QDirModel_fileIcon<QIcon> for (&'a  QModelIndex) {
  fn fileIcon(self, rsthis: &mut QDirModel) -> QIcon {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDirModel8fileIconERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QDirModel8fileIconERK11QModelIndex(rsthis.qclsinst, arg0)};
    let mut ret1 = QIcon{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDirModel {
  pub fn hasChildren<RetType, T: QDirModel_hasChildren<RetType>>(&mut self, value: T) -> RetType {
    return value.hasChildren(self);
    // return 1;
  }
}

pub trait QDirModel_hasChildren<RetType> {
  fn hasChildren(self, rsthis: &mut QDirModel) -> RetType;
}

// proto:  bool QDirModel::hasChildren(const QModelIndex & index);
impl<'a> /*trait*/ QDirModel_hasChildren<i8> for (&'a  QModelIndex) {
  fn hasChildren(self, rsthis: &mut QDirModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDirModel11hasChildrenERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QDirModel11hasChildrenERK11QModelIndex(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QDirModel {
  pub fn isDir<RetType, T: QDirModel_isDir<RetType>>(&mut self, value: T) -> RetType {
    return value.isDir(self);
    // return 1;
  }
}

pub trait QDirModel_isDir<RetType> {
  fn isDir(self, rsthis: &mut QDirModel) -> RetType;
}

// proto:  bool QDirModel::isDir(const QModelIndex & index);
impl<'a> /*trait*/ QDirModel_isDir<i8> for (&'a  QModelIndex) {
  fn isDir(self, rsthis: &mut QDirModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDirModel5isDirERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QDirModel5isDirERK11QModelIndex(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QDirModel {
  pub fn mkdir<RetType, T: QDirModel_mkdir<RetType>>(&mut self, value: T) -> RetType {
    return value.mkdir(self);
    // return 1;
  }
}

pub trait QDirModel_mkdir<RetType> {
  fn mkdir(self, rsthis: &mut QDirModel) -> RetType;
}

// proto:  QModelIndex QDirModel::mkdir(const QModelIndex & parent, const QString & name);
impl<'a> /*trait*/ QDirModel_mkdir<QModelIndex> for (&'a  QModelIndex, &'a  QString) {
  fn mkdir(self, rsthis: &mut QDirModel) -> QModelIndex {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDirModel5mkdirERK11QModelIndexRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN9QDirModel5mkdirERK11QModelIndexRK7QString(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QModelIndex{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDirModel {
  pub fn rmdir<RetType, T: QDirModel_rmdir<RetType>>(&mut self, value: T) -> RetType {
    return value.rmdir(self);
    // return 1;
  }
}

pub trait QDirModel_rmdir<RetType> {
  fn rmdir(self, rsthis: &mut QDirModel) -> RetType;
}

// proto:  bool QDirModel::rmdir(const QModelIndex & index);
impl<'a> /*trait*/ QDirModel_rmdir<i8> for (&'a  QModelIndex) {
  fn rmdir(self, rsthis: &mut QDirModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDirModel5rmdirERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN9QDirModel5rmdirERK11QModelIndex(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QDirModel {
  pub fn filePath<RetType, T: QDirModel_filePath<RetType>>(&mut self, value: T) -> RetType {
    return value.filePath(self);
    // return 1;
  }
}

pub trait QDirModel_filePath<RetType> {
  fn filePath(self, rsthis: &mut QDirModel) -> RetType;
}

// proto:  QString QDirModel::filePath(const QModelIndex & index);
impl<'a> /*trait*/ QDirModel_filePath<QString> for (&'a  QModelIndex) {
  fn filePath(self, rsthis: &mut QDirModel) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDirModel8filePathERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QDirModel8filePathERK11QModelIndex(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QDirModel {
  pub fn rowCount<RetType, T: QDirModel_rowCount<RetType>>(&mut self, value: T) -> RetType {
    return value.rowCount(self);
    // return 1;
  }
}

pub trait QDirModel_rowCount<RetType> {
  fn rowCount(self, rsthis: &mut QDirModel) -> RetType;
}

// proto:  int QDirModel::rowCount(const QModelIndex & parent);
impl<'a> /*trait*/ QDirModel_rowCount<i32> for (&'a  QModelIndex) {
  fn rowCount(self, rsthis: &mut QDirModel) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDirModel8rowCountERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QDirModel8rowCountERK11QModelIndex(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QDirModel {
  pub fn setReadOnly<RetType, T: QDirModel_setReadOnly<RetType>>(&mut self, value: T) -> RetType {
    return value.setReadOnly(self);
    // return 1;
  }
}

pub trait QDirModel_setReadOnly<RetType> {
  fn setReadOnly(self, rsthis: &mut QDirModel) -> RetType;
}

// proto:  void QDirModel::setReadOnly(bool enable);
impl<'a> /*trait*/ QDirModel_setReadOnly<()> for (i8) {
  fn setReadOnly(self, rsthis: &mut QDirModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDirModel11setReadOnlyEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN9QDirModel11setReadOnlyEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto: void QDirModel::NewQDirModel(const QDirModel & );
impl<'a> /*trait*/ QDirModel_NewQDirModel for (&'a  QDirModel) {
  fn NewQDirModel(self) -> QDirModel {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDirModelC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QDirModelC1ERKS_(qthis, arg0)};
    let rsthis = QDirModel{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QDirModel {
  pub fn setResolveSymlinks<RetType, T: QDirModel_setResolveSymlinks<RetType>>(&mut self, value: T) -> RetType {
    return value.setResolveSymlinks(self);
    // return 1;
  }
}

pub trait QDirModel_setResolveSymlinks<RetType> {
  fn setResolveSymlinks(self, rsthis: &mut QDirModel) -> RetType;
}

// proto:  void QDirModel::setResolveSymlinks(bool enable);
impl<'a> /*trait*/ QDirModel_setResolveSymlinks<()> for (i8) {
  fn setResolveSymlinks(self, rsthis: &mut QDirModel) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QDirModel18setResolveSymlinksEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN9QDirModel18setResolveSymlinksEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QDirModel {
  pub fn lazyChildCount<RetType, T: QDirModel_lazyChildCount<RetType>>(&mut self, value: T) -> RetType {
    return value.lazyChildCount(self);
    // return 1;
  }
}

pub trait QDirModel_lazyChildCount<RetType> {
  fn lazyChildCount(self, rsthis: &mut QDirModel) -> RetType;
}

// proto:  bool QDirModel::lazyChildCount();
impl<'a> /*trait*/ QDirModel_lazyChildCount<i8> for () {
  fn lazyChildCount(self, rsthis: &mut QDirModel) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDirModel14lazyChildCountEv()};
    let mut ret = unsafe {_ZNK9QDirModel14lazyChildCountEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QDirModel {
  pub fn fileInfo<RetType, T: QDirModel_fileInfo<RetType>>(&mut self, value: T) -> RetType {
    return value.fileInfo(self);
    // return 1;
  }
}

pub trait QDirModel_fileInfo<RetType> {
  fn fileInfo(self, rsthis: &mut QDirModel) -> RetType;
}

// proto:  QFileInfo QDirModel::fileInfo(const QModelIndex & index);
impl<'a> /*trait*/ QDirModel_fileInfo<QFileInfo> for (&'a  QModelIndex) {
  fn fileInfo(self, rsthis: &mut QDirModel) -> QFileInfo {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QDirModel8fileInfoERK11QModelIndex()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QDirModel8fileInfoERK11QModelIndex(rsthis.qclsinst, arg0)};
    let mut ret1 = QFileInfo{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

