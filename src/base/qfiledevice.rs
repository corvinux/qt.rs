// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;
use super::qobject::QObject;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  long long QFileDevice::size();
  fn _ZNK11QFileDevice4sizeEv(qthis: *mut c_void) -> c_longlong;
  // proto:  bool QFileDevice::seek(qint64 offset);
  fn _ZN11QFileDevice4seekEx(qthis: *mut c_void, arg0: c_longlong) -> int8_t;
  // proto:  bool QFileDevice::unmap(uchar * address);
  fn _ZN11QFileDevice5unmapEPh(qthis: *mut c_void, arg0: *mut c_uchar) -> int8_t;
  // proto:  void QFileDevice::close();
  fn _ZN11QFileDevice5closeEv(qthis: *mut c_void) ;
  // proto:  long long QFileDevice::pos();
  fn _ZNK11QFileDevice3posEv(qthis: *mut c_void) -> c_longlong;
  // proto:  int QFileDevice::handle();
  fn _ZNK11QFileDevice6handleEv(qthis: *mut c_void) -> c_int;
  // proto:  QString QFileDevice::fileName();
  fn _ZNK11QFileDevice8fileNameEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QFileDevice::NewQFileDevice(QObject * parent);
  fn _ZN11QFileDeviceC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QFileDevice::FreeQFileDevice();
  fn _ZN11QFileDeviceD0Ev(qthis: *mut c_void) ;
  // proto:  bool QFileDevice::atEnd();
  fn _ZNK11QFileDevice5atEndEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QFileDevice::isSequential();
  fn _ZNK11QFileDevice12isSequentialEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QFileDevice::flush();
  fn _ZN11QFileDevice5flushEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QFileDevice::NewQFileDevice();
  fn _ZN11QFileDeviceC1Ev(qthis: *mut c_void) ;
  // proto:  void QFileDevice::unsetError();
  fn _ZN11QFileDevice10unsetErrorEv(qthis: *mut c_void) ;
  // proto:  void QFileDevice::NewQFileDevice(const QFileDevice & );
  fn _ZN11QFileDeviceC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  const QMetaObject * QFileDevice::metaObject();
  fn _ZNK11QFileDevice10metaObjectEv(qthis: *mut c_void) ;
  // proto:  bool QFileDevice::resize(qint64 sz);
  fn _ZN11QFileDevice6resizeEx(qthis: *mut c_void, arg0: c_longlong) -> int8_t;
}

// body block begin
// class sizeof(QFileDevice)=1
pub struct QFileDevice {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QFileDevice {
  pub fn size<RetType, T: QFileDevice_size<RetType>>(&mut self, value: T) -> RetType {
    return value.size(self);
    // return 1;
  }
}

pub trait QFileDevice_size<RetType> {
  fn size(self, rsthis: &mut QFileDevice) -> RetType;
}

// proto:  long long QFileDevice::size();
impl<'a> /*trait*/ QFileDevice_size<i64> for () {
  fn size(self, rsthis: &mut QFileDevice) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFileDevice4sizeEv()};
    let mut ret = unsafe {_ZNK11QFileDevice4sizeEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

impl /*struct*/ QFileDevice {
  pub fn seek<RetType, T: QFileDevice_seek<RetType>>(&mut self, value: T) -> RetType {
    return value.seek(self);
    // return 1;
  }
}

pub trait QFileDevice_seek<RetType> {
  fn seek(self, rsthis: &mut QFileDevice) -> RetType;
}

// proto:  bool QFileDevice::seek(qint64 offset);
impl<'a> /*trait*/ QFileDevice_seek<i8> for (i64) {
  fn seek(self, rsthis: &mut QFileDevice) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDevice4seekEx()};
    let arg0 = self  as c_longlong;
    let mut ret = unsafe {_ZN11QFileDevice4seekEx(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFileDevice {
  pub fn unmap<RetType, T: QFileDevice_unmap<RetType>>(&mut self, value: T) -> RetType {
    return value.unmap(self);
    // return 1;
  }
}

pub trait QFileDevice_unmap<RetType> {
  fn unmap(self, rsthis: &mut QFileDevice) -> RetType;
}

// proto:  bool QFileDevice::unmap(uchar * address);
impl<'a> /*trait*/ QFileDevice_unmap<i8> for (&'a mut String) {
  fn unmap(self, rsthis: &mut QFileDevice) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDevice5unmapEPh()};
    let arg0 = self.as_ptr()  as *mut c_uchar;
    let mut ret = unsafe {_ZN11QFileDevice5unmapEPh(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFileDevice {
  pub fn close<RetType, T: QFileDevice_close<RetType>>(&mut self, value: T) -> RetType {
    return value.close(self);
    // return 1;
  }
}

pub trait QFileDevice_close<RetType> {
  fn close(self, rsthis: &mut QFileDevice) -> RetType;
}

// proto:  void QFileDevice::close();
impl<'a> /*trait*/ QFileDevice_close<()> for () {
  fn close(self, rsthis: &mut QFileDevice) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDevice5closeEv()};
     unsafe {_ZN11QFileDevice5closeEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QFileDevice {
  pub fn pos<RetType, T: QFileDevice_pos<RetType>>(&mut self, value: T) -> RetType {
    return value.pos(self);
    // return 1;
  }
}

pub trait QFileDevice_pos<RetType> {
  fn pos(self, rsthis: &mut QFileDevice) -> RetType;
}

// proto:  long long QFileDevice::pos();
impl<'a> /*trait*/ QFileDevice_pos<i64> for () {
  fn pos(self, rsthis: &mut QFileDevice) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFileDevice3posEv()};
    let mut ret = unsafe {_ZNK11QFileDevice3posEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

impl /*struct*/ QFileDevice {
  pub fn handle<RetType, T: QFileDevice_handle<RetType>>(&mut self, value: T) -> RetType {
    return value.handle(self);
    // return 1;
  }
}

pub trait QFileDevice_handle<RetType> {
  fn handle(self, rsthis: &mut QFileDevice) -> RetType;
}

// proto:  int QFileDevice::handle();
impl<'a> /*trait*/ QFileDevice_handle<i32> for () {
  fn handle(self, rsthis: &mut QFileDevice) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFileDevice6handleEv()};
    let mut ret = unsafe {_ZNK11QFileDevice6handleEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QFileDevice {
  pub fn fileName<RetType, T: QFileDevice_fileName<RetType>>(&mut self, value: T) -> RetType {
    return value.fileName(self);
    // return 1;
  }
}

pub trait QFileDevice_fileName<RetType> {
  fn fileName(self, rsthis: &mut QFileDevice) -> RetType;
}

// proto:  QString QFileDevice::fileName();
impl<'a> /*trait*/ QFileDevice_fileName<QString> for () {
  fn fileName(self, rsthis: &mut QFileDevice) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFileDevice8fileNameEv()};
    let mut ret = unsafe {_ZNK11QFileDevice8fileNameEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QFileDevice {
  pub fn NewQFileDevice<T: QFileDevice_NewQFileDevice>(value: T) -> QFileDevice {
    let rsthis = value.NewQFileDevice();
    return rsthis;
    // return 1;
  }
}

pub trait QFileDevice_NewQFileDevice {
  fn NewQFileDevice(self) -> QFileDevice;
}

// proto: void QFileDevice::NewQFileDevice(QObject * parent);
impl<'a> /*trait*/ QFileDevice_NewQFileDevice for (&'a mut QObject) {
  fn NewQFileDevice(self) -> QFileDevice {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDeviceC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QFileDeviceC1EP7QObject(qthis, arg0)};
    let rsthis = QFileDevice{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QFileDevice {
  pub fn FreeQFileDevice<RetType, T: QFileDevice_FreeQFileDevice<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQFileDevice(self);
    // return 1;
  }
}

pub trait QFileDevice_FreeQFileDevice<RetType> {
  fn FreeQFileDevice(self, rsthis: &mut QFileDevice) -> RetType;
}

// proto:  void QFileDevice::FreeQFileDevice();
impl<'a> /*trait*/ QFileDevice_FreeQFileDevice<()> for () {
  fn FreeQFileDevice(self, rsthis: &mut QFileDevice) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDeviceD0Ev()};
     unsafe {_ZN11QFileDeviceD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QFileDevice {
  pub fn atEnd<RetType, T: QFileDevice_atEnd<RetType>>(&mut self, value: T) -> RetType {
    return value.atEnd(self);
    // return 1;
  }
}

pub trait QFileDevice_atEnd<RetType> {
  fn atEnd(self, rsthis: &mut QFileDevice) -> RetType;
}

// proto:  bool QFileDevice::atEnd();
impl<'a> /*trait*/ QFileDevice_atEnd<i8> for () {
  fn atEnd(self, rsthis: &mut QFileDevice) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFileDevice5atEndEv()};
    let mut ret = unsafe {_ZNK11QFileDevice5atEndEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFileDevice {
  pub fn isSequential<RetType, T: QFileDevice_isSequential<RetType>>(&mut self, value: T) -> RetType {
    return value.isSequential(self);
    // return 1;
  }
}

pub trait QFileDevice_isSequential<RetType> {
  fn isSequential(self, rsthis: &mut QFileDevice) -> RetType;
}

// proto:  bool QFileDevice::isSequential();
impl<'a> /*trait*/ QFileDevice_isSequential<i8> for () {
  fn isSequential(self, rsthis: &mut QFileDevice) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFileDevice12isSequentialEv()};
    let mut ret = unsafe {_ZNK11QFileDevice12isSequentialEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QFileDevice {
  pub fn flush<RetType, T: QFileDevice_flush<RetType>>(&mut self, value: T) -> RetType {
    return value.flush(self);
    // return 1;
  }
}

pub trait QFileDevice_flush<RetType> {
  fn flush(self, rsthis: &mut QFileDevice) -> RetType;
}

// proto:  bool QFileDevice::flush();
impl<'a> /*trait*/ QFileDevice_flush<i8> for () {
  fn flush(self, rsthis: &mut QFileDevice) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDevice5flushEv()};
    let mut ret = unsafe {_ZN11QFileDevice5flushEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto: void QFileDevice::NewQFileDevice();
impl<'a> /*trait*/ QFileDevice_NewQFileDevice for () {
  fn NewQFileDevice(self) -> QFileDevice {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDeviceC1Ev()};
    unsafe {_ZN11QFileDeviceC1Ev(qthis)};
    let rsthis = QFileDevice{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QFileDevice {
  pub fn unsetError<RetType, T: QFileDevice_unsetError<RetType>>(&mut self, value: T) -> RetType {
    return value.unsetError(self);
    // return 1;
  }
}

pub trait QFileDevice_unsetError<RetType> {
  fn unsetError(self, rsthis: &mut QFileDevice) -> RetType;
}

// proto:  void QFileDevice::unsetError();
impl<'a> /*trait*/ QFileDevice_unsetError<()> for () {
  fn unsetError(self, rsthis: &mut QFileDevice) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDevice10unsetErrorEv()};
     unsafe {_ZN11QFileDevice10unsetErrorEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto: void QFileDevice::NewQFileDevice(const QFileDevice & );
impl<'a> /*trait*/ QFileDevice_NewQFileDevice for (&'a  QFileDevice) {
  fn NewQFileDevice(self) -> QFileDevice {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDeviceC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QFileDeviceC1ERKS_(qthis, arg0)};
    let rsthis = QFileDevice{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QFileDevice {
  pub fn metaObject<RetType, T: QFileDevice_metaObject<RetType>>(&mut self, value: T) -> RetType {
    return value.metaObject(self);
    // return 1;
  }
}

pub trait QFileDevice_metaObject<RetType> {
  fn metaObject(self, rsthis: &mut QFileDevice) -> RetType;
}

// proto:  const QMetaObject * QFileDevice::metaObject();
impl<'a> /*trait*/ QFileDevice_metaObject<()> for () {
  fn metaObject(self, rsthis: &mut QFileDevice) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFileDevice10metaObjectEv()};
     unsafe {_ZNK11QFileDevice10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QFileDevice {
  pub fn resize<RetType, T: QFileDevice_resize<RetType>>(&mut self, value: T) -> RetType {
    return value.resize(self);
    // return 1;
  }
}

pub trait QFileDevice_resize<RetType> {
  fn resize(self, rsthis: &mut QFileDevice) -> RetType;
}

// proto:  bool QFileDevice::resize(qint64 sz);
impl<'a> /*trait*/ QFileDevice_resize<i8> for (i64) {
  fn resize(self, rsthis: &mut QFileDevice) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFileDevice6resizeEx()};
    let arg0 = self  as c_longlong;
    let mut ret = unsafe {_ZN11QFileDevice6resizeEx(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

