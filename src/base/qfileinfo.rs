// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;
use super::qfile::QFile;
use super::qdatetime::QDateTime;
use super::qdir::QDir;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  bool QFileInfo::isHidden();
  fn _ZNK9QFileInfo8isHiddenEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QFileInfo::NewQFileInfo();
  fn _ZN9QFileInfoC1Ev(qthis: *mut c_void) ;
  // proto:  QString QFileInfo::completeSuffix();
  fn _ZNK9QFileInfo14completeSuffixEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString QFileInfo::canonicalPath();
  fn _ZNK9QFileInfo13canonicalPathEv(qthis: *mut c_void) -> *mut c_void;
  // proto: static bool QFileInfo::exists(const QString & file);
  fn _ZN9QFileInfo6existsERK7QString(arg0: *mut c_void) -> int8_t;
  // proto:  bool QFileInfo::makeAbsolute();
  fn _ZN9QFileInfo12makeAbsoluteEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QFileInfo::isRoot();
  fn _ZNK9QFileInfo6isRootEv(qthis: *mut c_void) -> int8_t;
  // proto:  QString QFileInfo::canonicalFilePath();
  fn _ZNK9QFileInfo17canonicalFilePathEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QFileInfo::isDir();
  fn _ZNK9QFileInfo5isDirEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QFileInfo::NewQFileInfo(const QString & file);
  fn _ZN9QFileInfoC1ERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QString QFileInfo::symLinkTarget();
  fn _ZNK9QFileInfo13symLinkTargetEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QFileInfo::setFile(const QString & file);
  fn _ZN9QFileInfo7setFileERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QFileInfo::isFile();
  fn _ZNK9QFileInfo6isFileEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QFileInfo::NewQFileInfo(const QFile & file);
  fn _ZN9QFileInfoC1ERK5QFile(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QFileInfo::setFile(const QFile & file);
  fn _ZN9QFileInfo7setFileERK5QFile(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  unsigned int QFileInfo::ownerId();
  fn _ZNK9QFileInfo7ownerIdEv(qthis: *mut c_void) -> c_uint;
  // proto:  QString QFileInfo::readLink();
  fn _ZNK9QFileInfo8readLinkEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString QFileInfo::filePath();
  fn _ZNK9QFileInfo8filePathEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QFileInfo::NewQFileInfo(const QFileInfo & fileinfo);
  fn _ZN9QFileInfoC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QFileInfo::isSymLink();
  fn _ZNK9QFileInfo9isSymLinkEv(qthis: *mut c_void) -> int8_t;
  // proto:  QDateTime QFileInfo::lastRead();
  fn _ZNK9QFileInfo8lastReadEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QFileInfo::refresh();
  fn _ZN9QFileInfo7refreshEv(qthis: *mut c_void) ;
  // proto:  void QFileInfo::NewQFileInfo(const QDir & dir, const QString & file);
  fn _ZN9QFileInfoC1ERK4QDirRK7QString(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  QString QFileInfo::path();
  fn _ZNK9QFileInfo4pathEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QDir QFileInfo::absoluteDir();
  fn _ZNK9QFileInfo11absoluteDirEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QFileInfo::isBundle();
  fn _ZNK9QFileInfo8isBundleEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QFileInfo::setFile(const QDir & dir, const QString & file);
  fn _ZN9QFileInfo7setFileERK4QDirRK7QString(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  bool QFileInfo::isRelative();
  fn _ZNK9QFileInfo10isRelativeEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QFileInfo::setCaching(bool on);
  fn _ZN9QFileInfo10setCachingEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  QDateTime QFileInfo::created();
  fn _ZNK9QFileInfo7createdEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QFileInfo::caching();
  fn _ZNK9QFileInfo7cachingEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QFileInfo::FreeQFileInfo();
  fn _ZN9QFileInfoD0Ev(qthis: *mut c_void) ;
  // proto:  QString QFileInfo::completeBaseName();
  fn _ZNK9QFileInfo16completeBaseNameEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString QFileInfo::baseName();
  fn _ZNK9QFileInfo8baseNameEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QFileInfo::isExecutable();
  fn _ZNK9QFileInfo12isExecutableEv(qthis: *mut c_void) -> int8_t;
  // proto:  QString QFileInfo::bundleName();
  fn _ZNK9QFileInfo10bundleNameEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  unsigned int QFileInfo::groupId();
  fn _ZNK9QFileInfo7groupIdEv(qthis: *mut c_void) -> c_uint;
  // proto:  QString QFileInfo::fileName();
  fn _ZNK9QFileInfo8fileNameEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  long long QFileInfo::size();
  fn _ZNK9QFileInfo4sizeEv(qthis: *mut c_void) -> c_longlong;
  // proto:  QString QFileInfo::absoluteFilePath();
  fn _ZNK9QFileInfo16absoluteFilePathEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString QFileInfo::suffix();
  fn _ZNK9QFileInfo6suffixEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString QFileInfo::group();
  fn _ZNK9QFileInfo5groupEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QFileInfo::isAbsolute();
  fn _ZNK9QFileInfo10isAbsoluteEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QFileInfo::isNativePath();
  fn _ZNK9QFileInfo12isNativePathEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QFileInfo::isWritable();
  fn _ZNK9QFileInfo10isWritableEv(qthis: *mut c_void) -> int8_t;
  // proto:  QString QFileInfo::owner();
  fn _ZNK9QFileInfo5ownerEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QFileInfo::isReadable();
  fn _ZNK9QFileInfo10isReadableEv(qthis: *mut c_void) -> int8_t;
  // proto:  QDir QFileInfo::dir();
  fn _ZNK9QFileInfo3dirEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QFileInfo::swap(QFileInfo & other);
  fn _ZN9QFileInfo4swapERS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QFileInfo::exists();
  fn _ZNK9QFileInfo6existsEv(qthis: *mut c_void) -> int8_t;
  // proto:  QDateTime QFileInfo::lastModified();
  fn _ZNK9QFileInfo12lastModifiedEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString QFileInfo::absolutePath();
  fn _ZNK9QFileInfo12absolutePathEv(qthis: *mut c_void) -> *mut c_void;
}

// body block begin
// class sizeof(QFileInfo)=1
pub struct QFileInfo {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QFileInfo {
  pub fn isHidden<RetType, T: QFileInfo_isHidden<RetType>>(&mut self, value: T) -> RetType {
    return value.isHidden(self);
    // return 1;
  }
}

pub trait QFileInfo_isHidden<RetType> {
  fn isHidden(self, rsthis: &mut QFileInfo) -> RetType;
}

// proto:  bool QFileInfo::isHidden();
impl<'a> /*trait*/ QFileInfo_isHidden<i8> for () {
  fn isHidden(self, rsthis: &mut QFileInfo) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo8isHiddenEv()};
    let mut ret = unsafe {_ZNK9QFileInfo8isHiddenEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn NewQFileInfo<T: QFileInfo_NewQFileInfo>(value: T) -> QFileInfo {
    let rsthis = value.NewQFileInfo();
    return rsthis;
    // return 1;
  }
}

pub trait QFileInfo_NewQFileInfo {
  fn NewQFileInfo(self) -> QFileInfo;
}

// proto: void QFileInfo::NewQFileInfo();
impl<'a> /*trait*/ QFileInfo_NewQFileInfo for () {
  fn NewQFileInfo(self) -> QFileInfo {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QFileInfoC1Ev()};
    unsafe {_ZN9QFileInfoC1Ev(qthis)};
    let rsthis = QFileInfo{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn completeSuffix<RetType, T: QFileInfo_completeSuffix<RetType>>(&mut self, value: T) -> RetType {
    return value.completeSuffix(self);
    // return 1;
  }
}

pub trait QFileInfo_completeSuffix<RetType> {
  fn completeSuffix(self, rsthis: &mut QFileInfo) -> RetType;
}

// proto:  QString QFileInfo::completeSuffix();
impl<'a> /*trait*/ QFileInfo_completeSuffix<QString> for () {
  fn completeSuffix(self, rsthis: &mut QFileInfo) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo14completeSuffixEv()};
    let mut ret = unsafe {_ZNK9QFileInfo14completeSuffixEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn canonicalPath<RetType, T: QFileInfo_canonicalPath<RetType>>(&mut self, value: T) -> RetType {
    return value.canonicalPath(self);
    // return 1;
  }
}

pub trait QFileInfo_canonicalPath<RetType> {
  fn canonicalPath(self, rsthis: &mut QFileInfo) -> RetType;
}

// proto:  QString QFileInfo::canonicalPath();
impl<'a> /*trait*/ QFileInfo_canonicalPath<QString> for () {
  fn canonicalPath(self, rsthis: &mut QFileInfo) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo13canonicalPathEv()};
    let mut ret = unsafe {_ZNK9QFileInfo13canonicalPathEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn exists<RetType, T: QFileInfo_exists<RetType>>(&mut self, value: T) -> RetType {
    return value.exists(self);
    // return 1;
  }
}

pub trait QFileInfo_exists<RetType> {
  fn exists(self, rsthis: &mut QFileInfo) -> RetType;
}

// proto: static bool QFileInfo::exists(const QString & file);
impl<'a> /*trait*/ QFileInfo_exists<i8> for (&'a  QString) {
  fn exists(self, rsthis: &mut QFileInfo) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QFileInfo6existsERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN9QFileInfo6existsERK7QString(arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn makeAbsolute<RetType, T: QFileInfo_makeAbsolute<RetType>>(&mut self, value: T) -> RetType {
    return value.makeAbsolute(self);
    // return 1;
  }
}

pub trait QFileInfo_makeAbsolute<RetType> {
  fn makeAbsolute(self, rsthis: &mut QFileInfo) -> RetType;
}

// proto:  bool QFileInfo::makeAbsolute();
impl<'a> /*trait*/ QFileInfo_makeAbsolute<i8> for () {
  fn makeAbsolute(self, rsthis: &mut QFileInfo) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QFileInfo12makeAbsoluteEv()};
    let mut ret = unsafe {_ZN9QFileInfo12makeAbsoluteEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn isRoot<RetType, T: QFileInfo_isRoot<RetType>>(&mut self, value: T) -> RetType {
    return value.isRoot(self);
    // return 1;
  }
}

pub trait QFileInfo_isRoot<RetType> {
  fn isRoot(self, rsthis: &mut QFileInfo) -> RetType;
}

// proto:  bool QFileInfo::isRoot();
impl<'a> /*trait*/ QFileInfo_isRoot<i8> for () {
  fn isRoot(self, rsthis: &mut QFileInfo) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo6isRootEv()};
    let mut ret = unsafe {_ZNK9QFileInfo6isRootEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn canonicalFilePath<RetType, T: QFileInfo_canonicalFilePath<RetType>>(&mut self, value: T) -> RetType {
    return value.canonicalFilePath(self);
    // return 1;
  }
}

pub trait QFileInfo_canonicalFilePath<RetType> {
  fn canonicalFilePath(self, rsthis: &mut QFileInfo) -> RetType;
}

// proto:  QString QFileInfo::canonicalFilePath();
impl<'a> /*trait*/ QFileInfo_canonicalFilePath<QString> for () {
  fn canonicalFilePath(self, rsthis: &mut QFileInfo) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo17canonicalFilePathEv()};
    let mut ret = unsafe {_ZNK9QFileInfo17canonicalFilePathEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn isDir<RetType, T: QFileInfo_isDir<RetType>>(&mut self, value: T) -> RetType {
    return value.isDir(self);
    // return 1;
  }
}

pub trait QFileInfo_isDir<RetType> {
  fn isDir(self, rsthis: &mut QFileInfo) -> RetType;
}

// proto:  bool QFileInfo::isDir();
impl<'a> /*trait*/ QFileInfo_isDir<i8> for () {
  fn isDir(self, rsthis: &mut QFileInfo) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo5isDirEv()};
    let mut ret = unsafe {_ZNK9QFileInfo5isDirEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto: void QFileInfo::NewQFileInfo(const QString & file);
impl<'a> /*trait*/ QFileInfo_NewQFileInfo for (&'a  QString) {
  fn NewQFileInfo(self) -> QFileInfo {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QFileInfoC1ERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QFileInfoC1ERK7QString(qthis, arg0)};
    let rsthis = QFileInfo{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn symLinkTarget<RetType, T: QFileInfo_symLinkTarget<RetType>>(&mut self, value: T) -> RetType {
    return value.symLinkTarget(self);
    // return 1;
  }
}

pub trait QFileInfo_symLinkTarget<RetType> {
  fn symLinkTarget(self, rsthis: &mut QFileInfo) -> RetType;
}

// proto:  QString QFileInfo::symLinkTarget();
impl<'a> /*trait*/ QFileInfo_symLinkTarget<QString> for () {
  fn symLinkTarget(self, rsthis: &mut QFileInfo) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo13symLinkTargetEv()};
    let mut ret = unsafe {_ZNK9QFileInfo13symLinkTargetEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn setFile<RetType, T: QFileInfo_setFile<RetType>>(&mut self, value: T) -> RetType {
    return value.setFile(self);
    // return 1;
  }
}

pub trait QFileInfo_setFile<RetType> {
  fn setFile(self, rsthis: &mut QFileInfo) -> RetType;
}

// proto:  void QFileInfo::setFile(const QString & file);
impl<'a> /*trait*/ QFileInfo_setFile<()> for (&'a  QString) {
  fn setFile(self, rsthis: &mut QFileInfo) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QFileInfo7setFileERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QFileInfo7setFileERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn isFile<RetType, T: QFileInfo_isFile<RetType>>(&mut self, value: T) -> RetType {
    return value.isFile(self);
    // return 1;
  }
}

pub trait QFileInfo_isFile<RetType> {
  fn isFile(self, rsthis: &mut QFileInfo) -> RetType;
}

// proto:  bool QFileInfo::isFile();
impl<'a> /*trait*/ QFileInfo_isFile<i8> for () {
  fn isFile(self, rsthis: &mut QFileInfo) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo6isFileEv()};
    let mut ret = unsafe {_ZNK9QFileInfo6isFileEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto: void QFileInfo::NewQFileInfo(const QFile & file);
impl<'a> /*trait*/ QFileInfo_NewQFileInfo for (&'a  QFile) {
  fn NewQFileInfo(self) -> QFileInfo {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QFileInfoC1ERK5QFile()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QFileInfoC1ERK5QFile(qthis, arg0)};
    let rsthis = QFileInfo{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  void QFileInfo::setFile(const QFile & file);
impl<'a> /*trait*/ QFileInfo_setFile<()> for (&'a  QFile) {
  fn setFile(self, rsthis: &mut QFileInfo) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QFileInfo7setFileERK5QFile()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QFileInfo7setFileERK5QFile(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn ownerId<RetType, T: QFileInfo_ownerId<RetType>>(&mut self, value: T) -> RetType {
    return value.ownerId(self);
    // return 1;
  }
}

pub trait QFileInfo_ownerId<RetType> {
  fn ownerId(self, rsthis: &mut QFileInfo) -> RetType;
}

// proto:  unsigned int QFileInfo::ownerId();
impl<'a> /*trait*/ QFileInfo_ownerId<u32> for () {
  fn ownerId(self, rsthis: &mut QFileInfo) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo7ownerIdEv()};
    let mut ret = unsafe {_ZNK9QFileInfo7ownerIdEv(rsthis.qclsinst)};
    return ret as u32;
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn readLink<RetType, T: QFileInfo_readLink<RetType>>(&mut self, value: T) -> RetType {
    return value.readLink(self);
    // return 1;
  }
}

pub trait QFileInfo_readLink<RetType> {
  fn readLink(self, rsthis: &mut QFileInfo) -> RetType;
}

// proto:  QString QFileInfo::readLink();
impl<'a> /*trait*/ QFileInfo_readLink<QString> for () {
  fn readLink(self, rsthis: &mut QFileInfo) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo8readLinkEv()};
    let mut ret = unsafe {_ZNK9QFileInfo8readLinkEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn filePath<RetType, T: QFileInfo_filePath<RetType>>(&mut self, value: T) -> RetType {
    return value.filePath(self);
    // return 1;
  }
}

pub trait QFileInfo_filePath<RetType> {
  fn filePath(self, rsthis: &mut QFileInfo) -> RetType;
}

// proto:  QString QFileInfo::filePath();
impl<'a> /*trait*/ QFileInfo_filePath<QString> for () {
  fn filePath(self, rsthis: &mut QFileInfo) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo8filePathEv()};
    let mut ret = unsafe {_ZNK9QFileInfo8filePathEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QFileInfo::NewQFileInfo(const QFileInfo & fileinfo);
impl<'a> /*trait*/ QFileInfo_NewQFileInfo for (&'a  QFileInfo) {
  fn NewQFileInfo(self) -> QFileInfo {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QFileInfoC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QFileInfoC1ERKS_(qthis, arg0)};
    let rsthis = QFileInfo{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn isSymLink<RetType, T: QFileInfo_isSymLink<RetType>>(&mut self, value: T) -> RetType {
    return value.isSymLink(self);
    // return 1;
  }
}

pub trait QFileInfo_isSymLink<RetType> {
  fn isSymLink(self, rsthis: &mut QFileInfo) -> RetType;
}

// proto:  bool QFileInfo::isSymLink();
impl<'a> /*trait*/ QFileInfo_isSymLink<i8> for () {
  fn isSymLink(self, rsthis: &mut QFileInfo) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo9isSymLinkEv()};
    let mut ret = unsafe {_ZNK9QFileInfo9isSymLinkEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn lastRead<RetType, T: QFileInfo_lastRead<RetType>>(&mut self, value: T) -> RetType {
    return value.lastRead(self);
    // return 1;
  }
}

pub trait QFileInfo_lastRead<RetType> {
  fn lastRead(self, rsthis: &mut QFileInfo) -> RetType;
}

// proto:  QDateTime QFileInfo::lastRead();
impl<'a> /*trait*/ QFileInfo_lastRead<QDateTime> for () {
  fn lastRead(self, rsthis: &mut QFileInfo) -> QDateTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo8lastReadEv()};
    let mut ret = unsafe {_ZNK9QFileInfo8lastReadEv(rsthis.qclsinst)};
    let mut ret1 = QDateTime{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn refresh<RetType, T: QFileInfo_refresh<RetType>>(&mut self, value: T) -> RetType {
    return value.refresh(self);
    // return 1;
  }
}

pub trait QFileInfo_refresh<RetType> {
  fn refresh(self, rsthis: &mut QFileInfo) -> RetType;
}

// proto:  void QFileInfo::refresh();
impl<'a> /*trait*/ QFileInfo_refresh<()> for () {
  fn refresh(self, rsthis: &mut QFileInfo) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QFileInfo7refreshEv()};
     unsafe {_ZN9QFileInfo7refreshEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto: void QFileInfo::NewQFileInfo(const QDir & dir, const QString & file);
impl<'a> /*trait*/ QFileInfo_NewQFileInfo for (&'a  QDir, &'a  QString) {
  fn NewQFileInfo(self) -> QFileInfo {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QFileInfoC1ERK4QDirRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN9QFileInfoC1ERK4QDirRK7QString(qthis, arg0, arg1)};
    let rsthis = QFileInfo{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn path<RetType, T: QFileInfo_path<RetType>>(&mut self, value: T) -> RetType {
    return value.path(self);
    // return 1;
  }
}

pub trait QFileInfo_path<RetType> {
  fn path(self, rsthis: &mut QFileInfo) -> RetType;
}

// proto:  QString QFileInfo::path();
impl<'a> /*trait*/ QFileInfo_path<QString> for () {
  fn path(self, rsthis: &mut QFileInfo) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo4pathEv()};
    let mut ret = unsafe {_ZNK9QFileInfo4pathEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn absoluteDir<RetType, T: QFileInfo_absoluteDir<RetType>>(&mut self, value: T) -> RetType {
    return value.absoluteDir(self);
    // return 1;
  }
}

pub trait QFileInfo_absoluteDir<RetType> {
  fn absoluteDir(self, rsthis: &mut QFileInfo) -> RetType;
}

// proto:  QDir QFileInfo::absoluteDir();
impl<'a> /*trait*/ QFileInfo_absoluteDir<QDir> for () {
  fn absoluteDir(self, rsthis: &mut QFileInfo) -> QDir {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo11absoluteDirEv()};
    let mut ret = unsafe {_ZNK9QFileInfo11absoluteDirEv(rsthis.qclsinst)};
    let mut ret1 = QDir{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn isBundle<RetType, T: QFileInfo_isBundle<RetType>>(&mut self, value: T) -> RetType {
    return value.isBundle(self);
    // return 1;
  }
}

pub trait QFileInfo_isBundle<RetType> {
  fn isBundle(self, rsthis: &mut QFileInfo) -> RetType;
}

// proto:  bool QFileInfo::isBundle();
impl<'a> /*trait*/ QFileInfo_isBundle<i8> for () {
  fn isBundle(self, rsthis: &mut QFileInfo) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo8isBundleEv()};
    let mut ret = unsafe {_ZNK9QFileInfo8isBundleEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  void QFileInfo::setFile(const QDir & dir, const QString & file);
impl<'a> /*trait*/ QFileInfo_setFile<()> for (&'a  QDir, &'a  QString) {
  fn setFile(self, rsthis: &mut QFileInfo) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QFileInfo7setFileERK4QDirRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN9QFileInfo7setFileERK4QDirRK7QString(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn isRelative<RetType, T: QFileInfo_isRelative<RetType>>(&mut self, value: T) -> RetType {
    return value.isRelative(self);
    // return 1;
  }
}

pub trait QFileInfo_isRelative<RetType> {
  fn isRelative(self, rsthis: &mut QFileInfo) -> RetType;
}

// proto:  bool QFileInfo::isRelative();
impl<'a> /*trait*/ QFileInfo_isRelative<i8> for () {
  fn isRelative(self, rsthis: &mut QFileInfo) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo10isRelativeEv()};
    let mut ret = unsafe {_ZNK9QFileInfo10isRelativeEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn setCaching<RetType, T: QFileInfo_setCaching<RetType>>(&mut self, value: T) -> RetType {
    return value.setCaching(self);
    // return 1;
  }
}

pub trait QFileInfo_setCaching<RetType> {
  fn setCaching(self, rsthis: &mut QFileInfo) -> RetType;
}

// proto:  void QFileInfo::setCaching(bool on);
impl<'a> /*trait*/ QFileInfo_setCaching<()> for (i8) {
  fn setCaching(self, rsthis: &mut QFileInfo) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QFileInfo10setCachingEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN9QFileInfo10setCachingEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn created<RetType, T: QFileInfo_created<RetType>>(&mut self, value: T) -> RetType {
    return value.created(self);
    // return 1;
  }
}

pub trait QFileInfo_created<RetType> {
  fn created(self, rsthis: &mut QFileInfo) -> RetType;
}

// proto:  QDateTime QFileInfo::created();
impl<'a> /*trait*/ QFileInfo_created<QDateTime> for () {
  fn created(self, rsthis: &mut QFileInfo) -> QDateTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo7createdEv()};
    let mut ret = unsafe {_ZNK9QFileInfo7createdEv(rsthis.qclsinst)};
    let mut ret1 = QDateTime{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn caching<RetType, T: QFileInfo_caching<RetType>>(&mut self, value: T) -> RetType {
    return value.caching(self);
    // return 1;
  }
}

pub trait QFileInfo_caching<RetType> {
  fn caching(self, rsthis: &mut QFileInfo) -> RetType;
}

// proto:  bool QFileInfo::caching();
impl<'a> /*trait*/ QFileInfo_caching<i8> for () {
  fn caching(self, rsthis: &mut QFileInfo) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo7cachingEv()};
    let mut ret = unsafe {_ZNK9QFileInfo7cachingEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn FreeQFileInfo<RetType, T: QFileInfo_FreeQFileInfo<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQFileInfo(self);
    // return 1;
  }
}

pub trait QFileInfo_FreeQFileInfo<RetType> {
  fn FreeQFileInfo(self, rsthis: &mut QFileInfo) -> RetType;
}

// proto:  void QFileInfo::FreeQFileInfo();
impl<'a> /*trait*/ QFileInfo_FreeQFileInfo<()> for () {
  fn FreeQFileInfo(self, rsthis: &mut QFileInfo) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QFileInfoD0Ev()};
     unsafe {_ZN9QFileInfoD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn completeBaseName<RetType, T: QFileInfo_completeBaseName<RetType>>(&mut self, value: T) -> RetType {
    return value.completeBaseName(self);
    // return 1;
  }
}

pub trait QFileInfo_completeBaseName<RetType> {
  fn completeBaseName(self, rsthis: &mut QFileInfo) -> RetType;
}

// proto:  QString QFileInfo::completeBaseName();
impl<'a> /*trait*/ QFileInfo_completeBaseName<QString> for () {
  fn completeBaseName(self, rsthis: &mut QFileInfo) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo16completeBaseNameEv()};
    let mut ret = unsafe {_ZNK9QFileInfo16completeBaseNameEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn baseName<RetType, T: QFileInfo_baseName<RetType>>(&mut self, value: T) -> RetType {
    return value.baseName(self);
    // return 1;
  }
}

pub trait QFileInfo_baseName<RetType> {
  fn baseName(self, rsthis: &mut QFileInfo) -> RetType;
}

// proto:  QString QFileInfo::baseName();
impl<'a> /*trait*/ QFileInfo_baseName<QString> for () {
  fn baseName(self, rsthis: &mut QFileInfo) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo8baseNameEv()};
    let mut ret = unsafe {_ZNK9QFileInfo8baseNameEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn isExecutable<RetType, T: QFileInfo_isExecutable<RetType>>(&mut self, value: T) -> RetType {
    return value.isExecutable(self);
    // return 1;
  }
}

pub trait QFileInfo_isExecutable<RetType> {
  fn isExecutable(self, rsthis: &mut QFileInfo) -> RetType;
}

// proto:  bool QFileInfo::isExecutable();
impl<'a> /*trait*/ QFileInfo_isExecutable<i8> for () {
  fn isExecutable(self, rsthis: &mut QFileInfo) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo12isExecutableEv()};
    let mut ret = unsafe {_ZNK9QFileInfo12isExecutableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn bundleName<RetType, T: QFileInfo_bundleName<RetType>>(&mut self, value: T) -> RetType {
    return value.bundleName(self);
    // return 1;
  }
}

pub trait QFileInfo_bundleName<RetType> {
  fn bundleName(self, rsthis: &mut QFileInfo) -> RetType;
}

// proto:  QString QFileInfo::bundleName();
impl<'a> /*trait*/ QFileInfo_bundleName<QString> for () {
  fn bundleName(self, rsthis: &mut QFileInfo) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo10bundleNameEv()};
    let mut ret = unsafe {_ZNK9QFileInfo10bundleNameEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn groupId<RetType, T: QFileInfo_groupId<RetType>>(&mut self, value: T) -> RetType {
    return value.groupId(self);
    // return 1;
  }
}

pub trait QFileInfo_groupId<RetType> {
  fn groupId(self, rsthis: &mut QFileInfo) -> RetType;
}

// proto:  unsigned int QFileInfo::groupId();
impl<'a> /*trait*/ QFileInfo_groupId<u32> for () {
  fn groupId(self, rsthis: &mut QFileInfo) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo7groupIdEv()};
    let mut ret = unsafe {_ZNK9QFileInfo7groupIdEv(rsthis.qclsinst)};
    return ret as u32;
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn fileName<RetType, T: QFileInfo_fileName<RetType>>(&mut self, value: T) -> RetType {
    return value.fileName(self);
    // return 1;
  }
}

pub trait QFileInfo_fileName<RetType> {
  fn fileName(self, rsthis: &mut QFileInfo) -> RetType;
}

// proto:  QString QFileInfo::fileName();
impl<'a> /*trait*/ QFileInfo_fileName<QString> for () {
  fn fileName(self, rsthis: &mut QFileInfo) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo8fileNameEv()};
    let mut ret = unsafe {_ZNK9QFileInfo8fileNameEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn size<RetType, T: QFileInfo_size<RetType>>(&mut self, value: T) -> RetType {
    return value.size(self);
    // return 1;
  }
}

pub trait QFileInfo_size<RetType> {
  fn size(self, rsthis: &mut QFileInfo) -> RetType;
}

// proto:  long long QFileInfo::size();
impl<'a> /*trait*/ QFileInfo_size<i64> for () {
  fn size(self, rsthis: &mut QFileInfo) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo4sizeEv()};
    let mut ret = unsafe {_ZNK9QFileInfo4sizeEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn absoluteFilePath<RetType, T: QFileInfo_absoluteFilePath<RetType>>(&mut self, value: T) -> RetType {
    return value.absoluteFilePath(self);
    // return 1;
  }
}

pub trait QFileInfo_absoluteFilePath<RetType> {
  fn absoluteFilePath(self, rsthis: &mut QFileInfo) -> RetType;
}

// proto:  QString QFileInfo::absoluteFilePath();
impl<'a> /*trait*/ QFileInfo_absoluteFilePath<QString> for () {
  fn absoluteFilePath(self, rsthis: &mut QFileInfo) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo16absoluteFilePathEv()};
    let mut ret = unsafe {_ZNK9QFileInfo16absoluteFilePathEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn suffix<RetType, T: QFileInfo_suffix<RetType>>(&mut self, value: T) -> RetType {
    return value.suffix(self);
    // return 1;
  }
}

pub trait QFileInfo_suffix<RetType> {
  fn suffix(self, rsthis: &mut QFileInfo) -> RetType;
}

// proto:  QString QFileInfo::suffix();
impl<'a> /*trait*/ QFileInfo_suffix<QString> for () {
  fn suffix(self, rsthis: &mut QFileInfo) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo6suffixEv()};
    let mut ret = unsafe {_ZNK9QFileInfo6suffixEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn group<RetType, T: QFileInfo_group<RetType>>(&mut self, value: T) -> RetType {
    return value.group(self);
    // return 1;
  }
}

pub trait QFileInfo_group<RetType> {
  fn group(self, rsthis: &mut QFileInfo) -> RetType;
}

// proto:  QString QFileInfo::group();
impl<'a> /*trait*/ QFileInfo_group<QString> for () {
  fn group(self, rsthis: &mut QFileInfo) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo5groupEv()};
    let mut ret = unsafe {_ZNK9QFileInfo5groupEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn isAbsolute<RetType, T: QFileInfo_isAbsolute<RetType>>(&mut self, value: T) -> RetType {
    return value.isAbsolute(self);
    // return 1;
  }
}

pub trait QFileInfo_isAbsolute<RetType> {
  fn isAbsolute(self, rsthis: &mut QFileInfo) -> RetType;
}

// proto:  bool QFileInfo::isAbsolute();
impl<'a> /*trait*/ QFileInfo_isAbsolute<i8> for () {
  fn isAbsolute(self, rsthis: &mut QFileInfo) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo10isAbsoluteEv()};
    let mut ret = unsafe {_ZNK9QFileInfo10isAbsoluteEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn isNativePath<RetType, T: QFileInfo_isNativePath<RetType>>(&mut self, value: T) -> RetType {
    return value.isNativePath(self);
    // return 1;
  }
}

pub trait QFileInfo_isNativePath<RetType> {
  fn isNativePath(self, rsthis: &mut QFileInfo) -> RetType;
}

// proto:  bool QFileInfo::isNativePath();
impl<'a> /*trait*/ QFileInfo_isNativePath<i8> for () {
  fn isNativePath(self, rsthis: &mut QFileInfo) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo12isNativePathEv()};
    let mut ret = unsafe {_ZNK9QFileInfo12isNativePathEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn isWritable<RetType, T: QFileInfo_isWritable<RetType>>(&mut self, value: T) -> RetType {
    return value.isWritable(self);
    // return 1;
  }
}

pub trait QFileInfo_isWritable<RetType> {
  fn isWritable(self, rsthis: &mut QFileInfo) -> RetType;
}

// proto:  bool QFileInfo::isWritable();
impl<'a> /*trait*/ QFileInfo_isWritable<i8> for () {
  fn isWritable(self, rsthis: &mut QFileInfo) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo10isWritableEv()};
    let mut ret = unsafe {_ZNK9QFileInfo10isWritableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn owner<RetType, T: QFileInfo_owner<RetType>>(&mut self, value: T) -> RetType {
    return value.owner(self);
    // return 1;
  }
}

pub trait QFileInfo_owner<RetType> {
  fn owner(self, rsthis: &mut QFileInfo) -> RetType;
}

// proto:  QString QFileInfo::owner();
impl<'a> /*trait*/ QFileInfo_owner<QString> for () {
  fn owner(self, rsthis: &mut QFileInfo) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo5ownerEv()};
    let mut ret = unsafe {_ZNK9QFileInfo5ownerEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn isReadable<RetType, T: QFileInfo_isReadable<RetType>>(&mut self, value: T) -> RetType {
    return value.isReadable(self);
    // return 1;
  }
}

pub trait QFileInfo_isReadable<RetType> {
  fn isReadable(self, rsthis: &mut QFileInfo) -> RetType;
}

// proto:  bool QFileInfo::isReadable();
impl<'a> /*trait*/ QFileInfo_isReadable<i8> for () {
  fn isReadable(self, rsthis: &mut QFileInfo) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo10isReadableEv()};
    let mut ret = unsafe {_ZNK9QFileInfo10isReadableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn dir<RetType, T: QFileInfo_dir<RetType>>(&mut self, value: T) -> RetType {
    return value.dir(self);
    // return 1;
  }
}

pub trait QFileInfo_dir<RetType> {
  fn dir(self, rsthis: &mut QFileInfo) -> RetType;
}

// proto:  QDir QFileInfo::dir();
impl<'a> /*trait*/ QFileInfo_dir<QDir> for () {
  fn dir(self, rsthis: &mut QFileInfo) -> QDir {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo3dirEv()};
    let mut ret = unsafe {_ZNK9QFileInfo3dirEv(rsthis.qclsinst)};
    let mut ret1 = QDir{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn swap<RetType, T: QFileInfo_swap<RetType>>(&mut self, value: T) -> RetType {
    return value.swap(self);
    // return 1;
  }
}

pub trait QFileInfo_swap<RetType> {
  fn swap(self, rsthis: &mut QFileInfo) -> RetType;
}

// proto:  void QFileInfo::swap(QFileInfo & other);
impl<'a> /*trait*/ QFileInfo_swap<()> for (&'a mut QFileInfo) {
  fn swap(self, rsthis: &mut QFileInfo) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QFileInfo4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QFileInfo4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  bool QFileInfo::exists();
impl<'a> /*trait*/ QFileInfo_exists<i8> for () {
  fn exists(self, rsthis: &mut QFileInfo) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo6existsEv()};
    let mut ret = unsafe {_ZNK9QFileInfo6existsEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn lastModified<RetType, T: QFileInfo_lastModified<RetType>>(&mut self, value: T) -> RetType {
    return value.lastModified(self);
    // return 1;
  }
}

pub trait QFileInfo_lastModified<RetType> {
  fn lastModified(self, rsthis: &mut QFileInfo) -> RetType;
}

// proto:  QDateTime QFileInfo::lastModified();
impl<'a> /*trait*/ QFileInfo_lastModified<QDateTime> for () {
  fn lastModified(self, rsthis: &mut QFileInfo) -> QDateTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo12lastModifiedEv()};
    let mut ret = unsafe {_ZNK9QFileInfo12lastModifiedEv(rsthis.qclsinst)};
    let mut ret1 = QDateTime{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFileInfo {
  pub fn absolutePath<RetType, T: QFileInfo_absolutePath<RetType>>(&mut self, value: T) -> RetType {
    return value.absolutePath(self);
    // return 1;
  }
}

pub trait QFileInfo_absolutePath<RetType> {
  fn absolutePath(self, rsthis: &mut QFileInfo) -> RetType;
}

// proto:  QString QFileInfo::absolutePath();
impl<'a> /*trait*/ QFileInfo_absolutePath<QString> for () {
  fn absolutePath(self, rsthis: &mut QFileInfo) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QFileInfo12absolutePathEv()};
    let mut ret = unsafe {_ZNK9QFileInfo12absolutePathEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

