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
use super::qtextcodec::QTextCodec;
use super::qchar::QChar;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QTextEncoder::~QTextEncoder();
  fn _ZN12QTextEncoderD0Ev(qthis: *mut c_void);
  // proto:  QByteArray QTextEncoder::fromUnicode(const QString & str);
  fn _ZN12QTextEncoder11fromUnicodeERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QTextEncoder::hasFailure();
  fn _ZNK12QTextEncoder10hasFailureEv(qthis: *mut c_void) -> c_char;
  // proto:  void QTextEncoder::QTextEncoder(const QTextCodec * codec);
  fn _ZN12QTextEncoderC1EPK10QTextCodec(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QTextEncoder::QTextEncoder(const QTextEncoder & );
  fn _ZN12QTextEncoderC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QByteArray QTextEncoder::fromUnicode(const QChar * uc, int len);
  fn _ZN12QTextEncoder11fromUnicodeEPK5QChari(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) -> *mut c_void;
}

// body block begin
// class sizeof(QTextEncoder)=1
pub struct QTextEncoder {
  pub qclsinst: *mut c_void,
}

  // proto:  void QTextEncoder::~QTextEncoder();
impl /*struct*/ QTextEncoder {
  pub fn FreeQTextEncoder<RetType, T: QTextEncoder_FreeQTextEncoder<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQTextEncoder(self);
    // return 1;
  }
}

pub trait QTextEncoder_FreeQTextEncoder<RetType> {
  fn FreeQTextEncoder(self , rsthis: &mut QTextEncoder) -> RetType;
}

  // proto:  void QTextEncoder::~QTextEncoder();
impl<'a> /*trait*/ QTextEncoder_FreeQTextEncoder<()> for () {
  fn FreeQTextEncoder(self , rsthis: &mut QTextEncoder) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextEncoderD0Ev()};
     unsafe {_ZN12QTextEncoderD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QByteArray QTextEncoder::fromUnicode(const QString & str);
impl /*struct*/ QTextEncoder {
  pub fn fromUnicode<RetType, T: QTextEncoder_fromUnicode<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.fromUnicode(self);
    // return 1;
  }
}

pub trait QTextEncoder_fromUnicode<RetType> {
  fn fromUnicode(self , rsthis: &mut QTextEncoder) -> RetType;
}

  // proto:  QByteArray QTextEncoder::fromUnicode(const QString & str);
impl<'a> /*trait*/ QTextEncoder_fromUnicode<QByteArray> for (QString) {
  fn fromUnicode(self , rsthis: &mut QTextEncoder) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextEncoder11fromUnicodeERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN12QTextEncoder11fromUnicodeERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QByteArray{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  bool QTextEncoder::hasFailure();
impl /*struct*/ QTextEncoder {
  pub fn hasFailure<RetType, T: QTextEncoder_hasFailure<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.hasFailure(self);
    // return 1;
  }
}

pub trait QTextEncoder_hasFailure<RetType> {
  fn hasFailure(self , rsthis: &mut QTextEncoder) -> RetType;
}

  // proto:  bool QTextEncoder::hasFailure();
impl<'a> /*trait*/ QTextEncoder_hasFailure<i8> for () {
  fn hasFailure(self , rsthis: &mut QTextEncoder) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTextEncoder10hasFailureEv()};
    let mut ret = unsafe {_ZNK12QTextEncoder10hasFailureEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QTextEncoder::QTextEncoder(const QTextCodec * codec);
impl /*struct*/ QTextEncoder {
  pub fn NewQTextEncoder<T: QTextEncoder_NewQTextEncoder>(value: T) -> QTextEncoder {
    let rsthis = value.NewQTextEncoder();
    return rsthis;
    // return 1;
  }
}

pub trait QTextEncoder_NewQTextEncoder {
  fn NewQTextEncoder(self) -> QTextEncoder;
}

  // proto:  void QTextEncoder::QTextEncoder(const QTextCodec * codec);
impl<'a> /*trait*/ QTextEncoder_NewQTextEncoder for (QTextCodec) {
  fn NewQTextEncoder(self) -> QTextEncoder {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextEncoderC1EPK10QTextCodec()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QTextEncoderC1EPK10QTextCodec(qthis, arg0)};
    let rsthis = QTextEncoder{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QTextEncoder::QTextEncoder(const QTextEncoder & );
impl<'a> /*trait*/ QTextEncoder_NewQTextEncoder for (QTextEncoder) {
  fn NewQTextEncoder(self) -> QTextEncoder {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextEncoderC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QTextEncoderC1ERKS_(qthis, arg0)};
    let rsthis = QTextEncoder{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QByteArray QTextEncoder::fromUnicode(const QChar * uc, int len);
impl<'a> /*trait*/ QTextEncoder_fromUnicode<QByteArray> for (QChar, i32) {
  fn fromUnicode(self , rsthis: &mut QTextEncoder) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextEncoder11fromUnicodeEPK5QChari()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZN12QTextEncoder11fromUnicodeEPK5QChari(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QByteArray{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

