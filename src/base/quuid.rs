// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;
use super::qbytearray::QByteArray;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QUuid::NewQUuid(const QString & );
  fn _ZN5QUuidC1ERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QByteArray QUuid::toRfc4122();
  fn _ZNK5QUuid9toRfc4122Ev(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString QUuid::toString();
  fn _ZNK5QUuid8toStringEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QUuid::isNull();
  fn _ZNK5QUuid6isNullEv(qthis: *mut c_void) -> int8_t;
  // proto: static QUuid QUuid::createUuidV5(const QUuid & ns, const QString & baseData);
  fn _ZN5QUuid12createUuidV5ERKS_RK7QString(arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto: static QUuid QUuid::createUuid();
  fn _ZN5QUuid10createUuidEv() -> *mut c_void;
  // proto:  void QUuid::NewQUuid(uint l, ushort w1, ushort w2, uchar b1, uchar b2, uchar b3, uchar b4, uchar b5, uchar b6, uchar b7, uchar b8);
  fn _ZN5QUuidC1Ejtthhhhhhhh(qthis: *mut c_void, arg0: c_uint, arg1: c_ushort, arg2: c_ushort, arg3: c_uchar, arg4: c_uchar, arg5: c_uchar, arg6: c_uchar, arg7: c_uchar, arg8: c_uchar, arg9: c_uchar, arg10: c_uchar) ;
  // proto:  void QUuid::NewQUuid(const QByteArray & );
  fn _ZN5QUuidC1ERK10QByteArray(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto: static QUuid QUuid::createUuidV3(const QUuid & ns, const QString & baseData);
  fn _ZN5QUuid12createUuidV3ERKS_RK7QString(arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  void QUuid::NewQUuid();
  fn _ZN5QUuidC1Ev(qthis: *mut c_void) ;
  // proto:  QByteArray QUuid::toByteArray();
  fn _ZNK5QUuid11toByteArrayEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QUuid::NewQUuid(const char * );
  fn _ZN5QUuidC1EPKc(qthis: *mut c_void, arg0: *const c_char) ;
  // proto: static QUuid QUuid::createUuidV5(const QUuid & ns, const QByteArray & baseData);
  fn _ZN5QUuid12createUuidV5ERKS_RK10QByteArray(arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto: static QUuid QUuid::fromRfc4122(const QByteArray & );
  fn _ZN5QUuid11fromRfc4122ERK10QByteArray(arg0: *mut c_void) -> *mut c_void;
  // proto: static QUuid QUuid::createUuidV3(const QUuid & ns, const QByteArray & baseData);
  fn _ZN5QUuid12createUuidV3ERKS_RK10QByteArray(arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
}

// body block begin
// class sizeof(QUuid)=16
pub struct QUuid {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QUuid {
  pub fn NewQUuid<T: QUuid_NewQUuid>(value: T) -> QUuid {
    let rsthis = value.NewQUuid();
    return rsthis;
    // return 1;
  }
}

pub trait QUuid_NewQUuid {
  fn NewQUuid(self) -> QUuid;
}

// proto: void QUuid::NewQUuid(const QString & );
impl<'a> /*trait*/ QUuid_NewQUuid for (&'a  QString) {
  fn NewQUuid(self) -> QUuid {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QUuidC1ERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN5QUuidC1ERK7QString(qthis, arg0)};
    let rsthis = QUuid{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QUuid {
  pub fn toRfc4122<RetType, T: QUuid_toRfc4122<RetType>>(&mut self, value: T) -> RetType {
    return value.toRfc4122(self);
    // return 1;
  }
}

pub trait QUuid_toRfc4122<RetType> {
  fn toRfc4122(self, rsthis: &mut QUuid) -> RetType;
}

// proto:  QByteArray QUuid::toRfc4122();
impl<'a> /*trait*/ QUuid_toRfc4122<QByteArray> for () {
  fn toRfc4122(self, rsthis: &mut QUuid) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QUuid9toRfc4122Ev()};
    let mut ret = unsafe {_ZNK5QUuid9toRfc4122Ev(rsthis.qclsinst)};
    let mut ret1 = QByteArray{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QUuid {
  pub fn toString<RetType, T: QUuid_toString<RetType>>(&mut self, value: T) -> RetType {
    return value.toString(self);
    // return 1;
  }
}

pub trait QUuid_toString<RetType> {
  fn toString(self, rsthis: &mut QUuid) -> RetType;
}

// proto:  QString QUuid::toString();
impl<'a> /*trait*/ QUuid_toString<QString> for () {
  fn toString(self, rsthis: &mut QUuid) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QUuid8toStringEv()};
    let mut ret = unsafe {_ZNK5QUuid8toStringEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QUuid {
  pub fn isNull<RetType, T: QUuid_isNull<RetType>>(&mut self, value: T) -> RetType {
    return value.isNull(self);
    // return 1;
  }
}

pub trait QUuid_isNull<RetType> {
  fn isNull(self, rsthis: &mut QUuid) -> RetType;
}

// proto:  bool QUuid::isNull();
impl<'a> /*trait*/ QUuid_isNull<i8> for () {
  fn isNull(self, rsthis: &mut QUuid) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QUuid6isNullEv()};
    let mut ret = unsafe {_ZNK5QUuid6isNullEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QUuid {
  pub fn createUuidV5<RetType, T: QUuid_createUuidV5<RetType>>(&mut self, value: T) -> RetType {
    return value.createUuidV5(self);
    // return 1;
  }
}

pub trait QUuid_createUuidV5<RetType> {
  fn createUuidV5(self, rsthis: &mut QUuid) -> RetType;
}

// proto: static QUuid QUuid::createUuidV5(const QUuid & ns, const QString & baseData);
impl<'a> /*trait*/ QUuid_createUuidV5<QUuid> for (&'a  QUuid, &'a  QString) {
  fn createUuidV5(self, rsthis: &mut QUuid) -> QUuid {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QUuid12createUuidV5ERKS_RK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN5QUuid12createUuidV5ERKS_RK7QString(arg0, arg1)};
    let mut ret1 = QUuid{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QUuid {
  pub fn createUuid<RetType, T: QUuid_createUuid<RetType>>(&mut self, value: T) -> RetType {
    return value.createUuid(self);
    // return 1;
  }
}

pub trait QUuid_createUuid<RetType> {
  fn createUuid(self, rsthis: &mut QUuid) -> RetType;
}

// proto: static QUuid QUuid::createUuid();
impl<'a> /*trait*/ QUuid_createUuid<QUuid> for () {
  fn createUuid(self, rsthis: &mut QUuid) -> QUuid {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QUuid10createUuidEv()};
    let mut ret = unsafe {_ZN5QUuid10createUuidEv()};
    let mut ret1 = QUuid{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QUuid::NewQUuid(uint l, ushort w1, ushort w2, uchar b1, uchar b2, uchar b3, uchar b4, uchar b5, uchar b6, uchar b7, uchar b8);
impl<'a> /*trait*/ QUuid_NewQUuid for (u32, u16, u16, u8, u8, u8, u8, u8, u8, u8, u8) {
  fn NewQUuid(self) -> QUuid {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QUuidC1Ejtthhhhhhhh()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1  as c_ushort;
    let arg2 = self.2  as c_ushort;
    let arg3 = self.3  as c_uchar;
    let arg4 = self.4  as c_uchar;
    let arg5 = self.5  as c_uchar;
    let arg6 = self.6  as c_uchar;
    let arg7 = self.7  as c_uchar;
    let arg8 = self.8  as c_uchar;
    let arg9 = self.9  as c_uchar;
    let arg10 = self.10  as c_uchar;
    unsafe {_ZN5QUuidC1Ejtthhhhhhhh(qthis, arg0, arg1, arg2, arg3, arg4, arg5, arg6, arg7, arg8, arg9, arg10)};
    let rsthis = QUuid{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QUuid::NewQUuid(const QByteArray & );
impl<'a> /*trait*/ QUuid_NewQUuid for (&'a  QByteArray) {
  fn NewQUuid(self) -> QUuid {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QUuidC1ERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN5QUuidC1ERK10QByteArray(qthis, arg0)};
    let rsthis = QUuid{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QUuid {
  pub fn createUuidV3<RetType, T: QUuid_createUuidV3<RetType>>(&mut self, value: T) -> RetType {
    return value.createUuidV3(self);
    // return 1;
  }
}

pub trait QUuid_createUuidV3<RetType> {
  fn createUuidV3(self, rsthis: &mut QUuid) -> RetType;
}

// proto: static QUuid QUuid::createUuidV3(const QUuid & ns, const QString & baseData);
impl<'a> /*trait*/ QUuid_createUuidV3<QUuid> for (&'a  QUuid, &'a  QString) {
  fn createUuidV3(self, rsthis: &mut QUuid) -> QUuid {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QUuid12createUuidV3ERKS_RK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN5QUuid12createUuidV3ERKS_RK7QString(arg0, arg1)};
    let mut ret1 = QUuid{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QUuid::NewQUuid();
impl<'a> /*trait*/ QUuid_NewQUuid for () {
  fn NewQUuid(self) -> QUuid {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QUuidC1Ev()};
    unsafe {_ZN5QUuidC1Ev(qthis)};
    let rsthis = QUuid{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QUuid {
  pub fn toByteArray<RetType, T: QUuid_toByteArray<RetType>>(&mut self, value: T) -> RetType {
    return value.toByteArray(self);
    // return 1;
  }
}

pub trait QUuid_toByteArray<RetType> {
  fn toByteArray(self, rsthis: &mut QUuid) -> RetType;
}

// proto:  QByteArray QUuid::toByteArray();
impl<'a> /*trait*/ QUuid_toByteArray<QByteArray> for () {
  fn toByteArray(self, rsthis: &mut QUuid) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QUuid11toByteArrayEv()};
    let mut ret = unsafe {_ZNK5QUuid11toByteArrayEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QUuid::NewQUuid(const char * );
impl<'a> /*trait*/ QUuid_NewQUuid for (&'a  String) {
  fn NewQUuid(self) -> QUuid {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QUuidC1EPKc()};
    let arg0 = self.as_ptr()  as *const c_char;
    unsafe {_ZN5QUuidC1EPKc(qthis, arg0)};
    let rsthis = QUuid{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: static QUuid QUuid::createUuidV5(const QUuid & ns, const QByteArray & baseData);
impl<'a> /*trait*/ QUuid_createUuidV5<QUuid> for (&'a  QUuid, &'a  QByteArray) {
  fn createUuidV5(self, rsthis: &mut QUuid) -> QUuid {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QUuid12createUuidV5ERKS_RK10QByteArray()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN5QUuid12createUuidV5ERKS_RK10QByteArray(arg0, arg1)};
    let mut ret1 = QUuid{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QUuid {
  pub fn fromRfc4122<RetType, T: QUuid_fromRfc4122<RetType>>(&mut self, value: T) -> RetType {
    return value.fromRfc4122(self);
    // return 1;
  }
}

pub trait QUuid_fromRfc4122<RetType> {
  fn fromRfc4122(self, rsthis: &mut QUuid) -> RetType;
}

// proto: static QUuid QUuid::fromRfc4122(const QByteArray & );
impl<'a> /*trait*/ QUuid_fromRfc4122<QUuid> for (&'a  QByteArray) {
  fn fromRfc4122(self, rsthis: &mut QUuid) -> QUuid {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QUuid11fromRfc4122ERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN5QUuid11fromRfc4122ERK10QByteArray(arg0)};
    let mut ret1 = QUuid{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: static QUuid QUuid::createUuidV3(const QUuid & ns, const QByteArray & baseData);
impl<'a> /*trait*/ QUuid_createUuidV3<QUuid> for (&'a  QUuid, &'a  QByteArray) {
  fn createUuidV3(self, rsthis: &mut QUuid) -> QUuid {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QUuid12createUuidV3ERKS_RK10QByteArray()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN5QUuid12createUuidV3ERKS_RK10QByteArray(arg0, arg1)};
    let mut ret1 = QUuid{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

