// auto generated, do not modify.
// created: Sun Jan 24 17:41:38 2016
// src-file: /QtCore/qlocale.h
// dst-file: /src/core/qlocale.rs
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
use super::qstring::QString; // 773
use super::qchar::QChar; // 773
use super::qdatetime::QTime; // 773
use super::qstringlist::QStringList; // 773
use super::qdatetime::QDateTime; // 773
use super::qdatetime::QDate; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QLocale_Class_Size() -> c_int;
  // proto:  QString QLocale::pmText();
  fn C_ZNK7QLocale6pmTextEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QString QLocale::nativeLanguageName();
  fn C_ZNK7QLocale18nativeLanguageNameEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QString QLocale::toLower(const QString & str);
  fn C_ZNK7QLocale7toLowerERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  QChar QLocale::zeroDigit();
  fn C_ZNK7QLocale9zeroDigitEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QString QLocale::name();
  fn C_ZNK7QLocale4nameEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QString QLocale::toCurrencyString(qlonglong , const QString & symbol);
  fn C_ZNK7QLocale16toCurrencyStringExRK7QString(qthis: u64 /* *mut c_void*/, arg0: c_longlong, arg1: *mut c_void) -> *mut c_void;
  // proto:  float QLocale::toFloat(const QString & s, bool * ok);
  fn C_ZNK7QLocale7toFloatERK7QStringPb(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_char) -> c_float;
  // proto: static QLocale QLocale::c();
  fn C_ZN7QLocale1cEv() -> *mut c_void;
  // proto:  QString QLocale::toCurrencyString(uint , const QString & symbol);
  fn C_ZNK7QLocale16toCurrencyStringEjRK7QString(qthis: u64 /* *mut c_void*/, arg0: c_uint, arg1: *mut c_void) -> *mut c_void;
  // proto:  QString QLocale::createSeparatedList(const QStringList & strl);
  fn C_ZNK7QLocale19createSeparatedListERK11QStringList(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  uint QLocale::toUInt(const QString & s, bool * ok);
  fn C_ZNK7QLocale6toUIntERK7QStringPb(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_char) -> c_uint;
  // proto:  QChar QLocale::decimalPoint();
  fn C_ZNK7QLocale12decimalPointEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QChar QLocale::positiveSign();
  fn C_ZNK7QLocale12positiveSignEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  qlonglong QLocale::toLongLong(const QString & s, bool * ok);
  fn C_ZNK7QLocale10toLongLongERK7QStringPb(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_char) -> c_longlong;
  // proto:  short QLocale::toShort(const QString & s, bool * ok);
  fn C_ZNK7QLocale7toShortERK7QStringPb(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_char) -> c_short;
  // proto:  QString QLocale::toString(float i, char f, int prec);
  fn C_ZNK7QLocale8toStringEfci(qthis: u64 /* *mut c_void*/, arg0: c_float, arg1: c_char, arg2: c_int) -> *mut c_void;
  // proto:  QString QLocale::toString(const QDateTime & dateTime, const QString & format);
  fn C_ZNK7QLocale8toStringERK9QDateTimeRK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  QDateTime QLocale::toDateTime(const QString & string, const QString & format);
  fn C_ZNK7QLocale10toDateTimeERK7QStringS2_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  QString QLocale::toCurrencyString(short , const QString & symbol);
  fn C_ZNK7QLocale16toCurrencyStringEsRK7QString(qthis: u64 /* *mut c_void*/, arg0: c_short, arg1: *mut c_void) -> *mut c_void;
  // proto:  QChar QLocale::groupSeparator();
  fn C_ZNK7QLocale14groupSeparatorEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QString QLocale::toCurrencyString(double , const QString & symbol);
  fn C_ZNK7QLocale16toCurrencyStringEdRK7QString(qthis: u64 /* *mut c_void*/, arg0: c_double, arg1: *mut c_void) -> *mut c_void;
  // proto:  QString QLocale::toCurrencyString(qulonglong , const QString & symbol);
  fn C_ZNK7QLocale16toCurrencyStringEyRK7QString(qthis: u64 /* *mut c_void*/, arg0: c_ulonglong, arg1: *mut c_void) -> *mut c_void;
  // proto:  void QLocale::QLocale(const QString & name);
  fn C_ZN7QLocaleC2ERK7QString(arg0: *mut c_void) -> u64;
  // proto:  QString QLocale::toString(const QTime & time, const QString & formatStr);
  fn C_ZNK7QLocale8toStringERK5QTimeRK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  QDate QLocale::toDate(const QString & string, const QString & format);
  fn C_ZNK7QLocale6toDateERK7QStringS2_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  QString QLocale::nativeCountryName();
  fn C_ZNK7QLocale17nativeCountryNameEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QChar QLocale::negativeSign();
  fn C_ZNK7QLocale12negativeSignEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QLocale::~QLocale();
  fn C_ZN7QLocaleD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QLocale::QLocale(const QLocale & other);
  fn C_ZN7QLocaleC2ERKS_(arg0: *mut c_void) -> u64;
  // proto:  QString QLocale::toString(const QDate & date, const QString & formatStr);
  fn C_ZNK7QLocale8toStringERK5QDateRK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  QString QLocale::toUpper(const QString & str);
  fn C_ZNK7QLocale7toUpperERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  QChar QLocale::percent();
  fn C_ZNK7QLocale7percentEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  qulonglong QLocale::toULongLong(const QString & s, bool * ok);
  fn C_ZNK7QLocale11toULongLongERK7QStringPb(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_char) -> c_ulonglong;
  // proto:  QString QLocale::toString(double i, char f, int prec);
  fn C_ZNK7QLocale8toStringEdci(qthis: u64 /* *mut c_void*/, arg0: c_double, arg1: c_char, arg2: c_int) -> *mut c_void;
  // proto:  QStringList QLocale::uiLanguages();
  fn C_ZNK7QLocale11uiLanguagesEv(qthis: u64 /* *mut c_void*/);
  // proto:  QString QLocale::bcp47Name();
  fn C_ZNK7QLocale9bcp47NameEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QTime QLocale::toTime(const QString & string, const QString & format);
  fn C_ZNK7QLocale6toTimeERK7QStringS2_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  ushort QLocale::toUShort(const QString & s, bool * ok);
  fn C_ZNK7QLocale8toUShortERK7QStringPb(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_char) -> c_ushort;
  // proto:  QString QLocale::toCurrencyString(ushort , const QString & symbol);
  fn C_ZNK7QLocale16toCurrencyStringEtRK7QString(qthis: u64 /* *mut c_void*/, arg0: c_ushort, arg1: *mut c_void) -> *mut c_void;
  // proto:  double QLocale::toDouble(const QString & s, bool * ok);
  fn C_ZNK7QLocale8toDoubleERK7QStringPb(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_char) -> c_double;
  // proto: static QLocale QLocale::system();
  fn C_ZN7QLocale6systemEv() -> *mut c_void;
  // proto: static void QLocale::setDefault(const QLocale & locale);
  fn C_ZN7QLocale10setDefaultERKS_(arg0: *mut c_void);
  // proto:  QChar QLocale::exponential();
  fn C_ZNK7QLocale11exponentialEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QString QLocale::toCurrencyString(float , const QString & symbol);
  fn C_ZNK7QLocale16toCurrencyStringEfRK7QString(qthis: u64 /* *mut c_void*/, arg0: c_float, arg1: *mut c_void) -> *mut c_void;
  // proto:  QString QLocale::toString(int i);
  fn C_ZNK7QLocale8toStringEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  QString QLocale::toString(uint i);
  fn C_ZNK7QLocale8toStringEj(qthis: u64 /* *mut c_void*/, arg0: c_uint) -> *mut c_void;
  // proto:  QString QLocale::toString(qlonglong i);
  fn C_ZNK7QLocale8toStringEx(qthis: u64 /* *mut c_void*/, arg0: c_longlong) -> *mut c_void;
  // proto:  QString QLocale::toString(qulonglong i);
  fn C_ZNK7QLocale8toStringEy(qthis: u64 /* *mut c_void*/, arg0: c_ulonglong) -> *mut c_void;
  // proto:  QString QLocale::toString(ushort i);
  fn C_ZNK7QLocale8toStringEt(qthis: u64 /* *mut c_void*/, arg0: c_ushort) -> *mut c_void;
  // proto:  QString QLocale::amText();
  fn C_ZNK7QLocale6amTextEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QString QLocale::toCurrencyString(int , const QString & symbol);
  fn C_ZNK7QLocale16toCurrencyStringEiRK7QString(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void) -> *mut c_void;
  // proto:  QString QLocale::toString(short i);
  fn C_ZNK7QLocale8toStringEs(qthis: u64 /* *mut c_void*/, arg0: c_short) -> *mut c_void;
  // proto:  void QLocale::QLocale();
  fn C_ZN7QLocaleC2Ev() -> u64;
  // proto:  int QLocale::toInt(const QString & s, bool * ok);
  fn C_ZNK7QLocale5toIntERK7QStringPb(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_char) -> c_int;
} // <= ext block end

// body block begin =>
// class sizeof(QLocale)=1
#[derive(Default)]
pub struct QLocale {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QLocale {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QLocale {
    return QLocale{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  QString QLocale::pmText();
impl /*struct*/ QLocale {
  pub fn pmText<RetType, T: QLocale_pmText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.pmText(self);
    // return 1;
  }
}

pub trait QLocale_pmText<RetType> {
  fn pmText(self , rsthis: & QLocale) -> RetType;
}

  // proto:  QString QLocale::pmText();
impl<'a> /*trait*/ QLocale_pmText<QString> for () {
  fn pmText(self , rsthis: & QLocale) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale6pmTextEv()};
    let mut ret = unsafe {C_ZNK7QLocale6pmTextEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QLocale::nativeLanguageName();
impl /*struct*/ QLocale {
  pub fn nativeLanguageName<RetType, T: QLocale_nativeLanguageName<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.nativeLanguageName(self);
    // return 1;
  }
}

pub trait QLocale_nativeLanguageName<RetType> {
  fn nativeLanguageName(self , rsthis: & QLocale) -> RetType;
}

  // proto:  QString QLocale::nativeLanguageName();
impl<'a> /*trait*/ QLocale_nativeLanguageName<QString> for () {
  fn nativeLanguageName(self , rsthis: & QLocale) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale18nativeLanguageNameEv()};
    let mut ret = unsafe {C_ZNK7QLocale18nativeLanguageNameEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QLocale::toLower(const QString & str);
impl /*struct*/ QLocale {
  pub fn toLower<RetType, T: QLocale_toLower<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toLower(self);
    // return 1;
  }
}

pub trait QLocale_toLower<RetType> {
  fn toLower(self , rsthis: & QLocale) -> RetType;
}

  // proto:  QString QLocale::toLower(const QString & str);
impl<'a> /*trait*/ QLocale_toLower<QString> for (&'a QString) {
  fn toLower(self , rsthis: & QLocale) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale7toLowerERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK7QLocale7toLowerERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QChar QLocale::zeroDigit();
impl /*struct*/ QLocale {
  pub fn zeroDigit<RetType, T: QLocale_zeroDigit<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.zeroDigit(self);
    // return 1;
  }
}

pub trait QLocale_zeroDigit<RetType> {
  fn zeroDigit(self , rsthis: & QLocale) -> RetType;
}

  // proto:  QChar QLocale::zeroDigit();
impl<'a> /*trait*/ QLocale_zeroDigit<QChar> for () {
  fn zeroDigit(self , rsthis: & QLocale) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale9zeroDigitEv()};
    let mut ret = unsafe {C_ZNK7QLocale9zeroDigitEv(rsthis.qclsinst)};
    let mut ret1 = QChar::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QLocale::name();
impl /*struct*/ QLocale {
  pub fn name<RetType, T: QLocale_name<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.name(self);
    // return 1;
  }
}

pub trait QLocale_name<RetType> {
  fn name(self , rsthis: & QLocale) -> RetType;
}

  // proto:  QString QLocale::name();
impl<'a> /*trait*/ QLocale_name<QString> for () {
  fn name(self , rsthis: & QLocale) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale4nameEv()};
    let mut ret = unsafe {C_ZNK7QLocale4nameEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QLocale::toCurrencyString(qlonglong , const QString & symbol);
impl /*struct*/ QLocale {
  pub fn toCurrencyString<RetType, T: QLocale_toCurrencyString<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toCurrencyString(self);
    // return 1;
  }
}

pub trait QLocale_toCurrencyString<RetType> {
  fn toCurrencyString(self , rsthis: & QLocale) -> RetType;
}

  // proto:  QString QLocale::toCurrencyString(qlonglong , const QString & symbol);
impl<'a> /*trait*/ QLocale_toCurrencyString<QString> for (i64, &'a QString) {
  fn toCurrencyString(self , rsthis: & QLocale) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale16toCurrencyStringExRK7QString()};
    let arg0 = self.0  as c_longlong;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK7QLocale16toCurrencyStringExRK7QString(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  float QLocale::toFloat(const QString & s, bool * ok);
impl /*struct*/ QLocale {
  pub fn toFloat<RetType, T: QLocale_toFloat<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toFloat(self);
    // return 1;
  }
}

pub trait QLocale_toFloat<RetType> {
  fn toFloat(self , rsthis: & QLocale) -> RetType;
}

  // proto:  float QLocale::toFloat(const QString & s, bool * ok);
impl<'a> /*trait*/ QLocale_toFloat<f32> for (&'a QString, &'a mut Vec<i8>) {
  fn toFloat(self , rsthis: & QLocale) -> f32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale7toFloatERK7QStringPb()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *mut c_char;
    let mut ret = unsafe {C_ZNK7QLocale7toFloatERK7QStringPb(rsthis.qclsinst, arg0, arg1)};
    return ret as f32;
    // return 1;
  }
}

  // proto: static QLocale QLocale::c();
impl /*struct*/ QLocale {
  pub fn c_s<RetType, T: QLocale_c_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.c_s();
    // return 1;
  }
}

pub trait QLocale_c_s<RetType> {
  fn c_s(self ) -> RetType;
}

  // proto: static QLocale QLocale::c();
impl<'a> /*trait*/ QLocale_c_s<QLocale> for () {
  fn c_s(self ) -> QLocale {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QLocale1cEv()};
    let mut ret = unsafe {C_ZN7QLocale1cEv()};
    let mut ret1 = QLocale::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QLocale::toCurrencyString(uint , const QString & symbol);
impl<'a> /*trait*/ QLocale_toCurrencyString<QString> for (u32, &'a QString) {
  fn toCurrencyString(self , rsthis: & QLocale) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale16toCurrencyStringEjRK7QString()};
    let arg0 = self.0  as c_uint;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK7QLocale16toCurrencyStringEjRK7QString(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QLocale::createSeparatedList(const QStringList & strl);
impl /*struct*/ QLocale {
  pub fn createSeparatedList<RetType, T: QLocale_createSeparatedList<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.createSeparatedList(self);
    // return 1;
  }
}

pub trait QLocale_createSeparatedList<RetType> {
  fn createSeparatedList(self , rsthis: & QLocale) -> RetType;
}

  // proto:  QString QLocale::createSeparatedList(const QStringList & strl);
impl<'a> /*trait*/ QLocale_createSeparatedList<QString> for (&'a QStringList) {
  fn createSeparatedList(self , rsthis: & QLocale) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale19createSeparatedListERK11QStringList()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK7QLocale19createSeparatedListERK11QStringList(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  uint QLocale::toUInt(const QString & s, bool * ok);
impl /*struct*/ QLocale {
  pub fn toUInt<RetType, T: QLocale_toUInt<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toUInt(self);
    // return 1;
  }
}

pub trait QLocale_toUInt<RetType> {
  fn toUInt(self , rsthis: & QLocale) -> RetType;
}

  // proto:  uint QLocale::toUInt(const QString & s, bool * ok);
impl<'a> /*trait*/ QLocale_toUInt<u32> for (&'a QString, &'a mut Vec<i8>) {
  fn toUInt(self , rsthis: & QLocale) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale6toUIntERK7QStringPb()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *mut c_char;
    let mut ret = unsafe {C_ZNK7QLocale6toUIntERK7QStringPb(rsthis.qclsinst, arg0, arg1)};
    return ret as u32;
    // return 1;
  }
}

  // proto:  QChar QLocale::decimalPoint();
impl /*struct*/ QLocale {
  pub fn decimalPoint<RetType, T: QLocale_decimalPoint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.decimalPoint(self);
    // return 1;
  }
}

pub trait QLocale_decimalPoint<RetType> {
  fn decimalPoint(self , rsthis: & QLocale) -> RetType;
}

  // proto:  QChar QLocale::decimalPoint();
impl<'a> /*trait*/ QLocale_decimalPoint<QChar> for () {
  fn decimalPoint(self , rsthis: & QLocale) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale12decimalPointEv()};
    let mut ret = unsafe {C_ZNK7QLocale12decimalPointEv(rsthis.qclsinst)};
    let mut ret1 = QChar::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QChar QLocale::positiveSign();
impl /*struct*/ QLocale {
  pub fn positiveSign<RetType, T: QLocale_positiveSign<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.positiveSign(self);
    // return 1;
  }
}

pub trait QLocale_positiveSign<RetType> {
  fn positiveSign(self , rsthis: & QLocale) -> RetType;
}

  // proto:  QChar QLocale::positiveSign();
impl<'a> /*trait*/ QLocale_positiveSign<QChar> for () {
  fn positiveSign(self , rsthis: & QLocale) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale12positiveSignEv()};
    let mut ret = unsafe {C_ZNK7QLocale12positiveSignEv(rsthis.qclsinst)};
    let mut ret1 = QChar::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  qlonglong QLocale::toLongLong(const QString & s, bool * ok);
impl /*struct*/ QLocale {
  pub fn toLongLong<RetType, T: QLocale_toLongLong<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toLongLong(self);
    // return 1;
  }
}

pub trait QLocale_toLongLong<RetType> {
  fn toLongLong(self , rsthis: & QLocale) -> RetType;
}

  // proto:  qlonglong QLocale::toLongLong(const QString & s, bool * ok);
impl<'a> /*trait*/ QLocale_toLongLong<i64> for (&'a QString, &'a mut Vec<i8>) {
  fn toLongLong(self , rsthis: & QLocale) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale10toLongLongERK7QStringPb()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *mut c_char;
    let mut ret = unsafe {C_ZNK7QLocale10toLongLongERK7QStringPb(rsthis.qclsinst, arg0, arg1)};
    return ret as i64;
    // return 1;
  }
}

  // proto:  short QLocale::toShort(const QString & s, bool * ok);
impl /*struct*/ QLocale {
  pub fn toShort<RetType, T: QLocale_toShort<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toShort(self);
    // return 1;
  }
}

pub trait QLocale_toShort<RetType> {
  fn toShort(self , rsthis: & QLocale) -> RetType;
}

  // proto:  short QLocale::toShort(const QString & s, bool * ok);
impl<'a> /*trait*/ QLocale_toShort<i16> for (&'a QString, &'a mut Vec<i8>) {
  fn toShort(self , rsthis: & QLocale) -> i16 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale7toShortERK7QStringPb()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *mut c_char;
    let mut ret = unsafe {C_ZNK7QLocale7toShortERK7QStringPb(rsthis.qclsinst, arg0, arg1)};
    return ret as i16;
    // return 1;
  }
}

  // proto:  QString QLocale::toString(float i, char f, int prec);
impl /*struct*/ QLocale {
  pub fn toString<RetType, T: QLocale_toString<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toString(self);
    // return 1;
  }
}

pub trait QLocale_toString<RetType> {
  fn toString(self , rsthis: & QLocale) -> RetType;
}

  // proto:  QString QLocale::toString(float i, char f, int prec);
impl<'a> /*trait*/ QLocale_toString<QString> for (f32, i8, i32) {
  fn toString(self , rsthis: & QLocale) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale8toStringEfci()};
    let arg0 = self.0  as c_float;
    let arg1 = self.1  as c_char;
    let arg2 = self.2  as c_int;
    let mut ret = unsafe {C_ZNK7QLocale8toStringEfci(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QLocale::toString(const QDateTime & dateTime, const QString & format);
impl<'a> /*trait*/ QLocale_toString<QString> for (&'a QDateTime, &'a QString) {
  fn toString(self , rsthis: & QLocale) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale8toStringERK9QDateTimeRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK7QLocale8toStringERK9QDateTimeRK7QString(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QDateTime QLocale::toDateTime(const QString & string, const QString & format);
impl /*struct*/ QLocale {
  pub fn toDateTime<RetType, T: QLocale_toDateTime<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toDateTime(self);
    // return 1;
  }
}

pub trait QLocale_toDateTime<RetType> {
  fn toDateTime(self , rsthis: & QLocale) -> RetType;
}

  // proto:  QDateTime QLocale::toDateTime(const QString & string, const QString & format);
impl<'a> /*trait*/ QLocale_toDateTime<QDateTime> for (&'a QString, &'a QString) {
  fn toDateTime(self , rsthis: & QLocale) -> QDateTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale10toDateTimeERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK7QLocale10toDateTimeERK7QStringS2_(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QDateTime::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QLocale::toCurrencyString(short , const QString & symbol);
impl<'a> /*trait*/ QLocale_toCurrencyString<QString> for (i16, &'a QString) {
  fn toCurrencyString(self , rsthis: & QLocale) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale16toCurrencyStringEsRK7QString()};
    let arg0 = self.0  as c_short;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK7QLocale16toCurrencyStringEsRK7QString(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QChar QLocale::groupSeparator();
impl /*struct*/ QLocale {
  pub fn groupSeparator<RetType, T: QLocale_groupSeparator<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.groupSeparator(self);
    // return 1;
  }
}

pub trait QLocale_groupSeparator<RetType> {
  fn groupSeparator(self , rsthis: & QLocale) -> RetType;
}

  // proto:  QChar QLocale::groupSeparator();
impl<'a> /*trait*/ QLocale_groupSeparator<QChar> for () {
  fn groupSeparator(self , rsthis: & QLocale) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale14groupSeparatorEv()};
    let mut ret = unsafe {C_ZNK7QLocale14groupSeparatorEv(rsthis.qclsinst)};
    let mut ret1 = QChar::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QLocale::toCurrencyString(double , const QString & symbol);
impl<'a> /*trait*/ QLocale_toCurrencyString<QString> for (f64, &'a QString) {
  fn toCurrencyString(self , rsthis: & QLocale) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale16toCurrencyStringEdRK7QString()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK7QLocale16toCurrencyStringEdRK7QString(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QLocale::toCurrencyString(qulonglong , const QString & symbol);
impl<'a> /*trait*/ QLocale_toCurrencyString<QString> for (u64, &'a QString) {
  fn toCurrencyString(self , rsthis: & QLocale) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale16toCurrencyStringEyRK7QString()};
    let arg0 = self.0  as c_ulonglong;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK7QLocale16toCurrencyStringEyRK7QString(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QLocale::QLocale(const QString & name);
impl /*struct*/ QLocale {
  pub fn new<T: QLocale_new>(value: T) -> QLocale {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QLocale_new {
  fn new(self) -> QLocale;
}

  // proto:  void QLocale::QLocale(const QString & name);
impl<'a> /*trait*/ QLocale_new for (&'a QString) {
  fn new(self) -> QLocale {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QLocaleC2ERK7QString()};
    let ctysz: c_int = unsafe{QLocale_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN7QLocaleC2ERK7QString(arg0)};
    let rsthis = QLocale{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QString QLocale::toString(const QTime & time, const QString & formatStr);
impl<'a> /*trait*/ QLocale_toString<QString> for (&'a QTime, &'a QString) {
  fn toString(self , rsthis: & QLocale) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale8toStringERK5QTimeRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK7QLocale8toStringERK5QTimeRK7QString(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QDate QLocale::toDate(const QString & string, const QString & format);
impl /*struct*/ QLocale {
  pub fn toDate<RetType, T: QLocale_toDate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toDate(self);
    // return 1;
  }
}

pub trait QLocale_toDate<RetType> {
  fn toDate(self , rsthis: & QLocale) -> RetType;
}

  // proto:  QDate QLocale::toDate(const QString & string, const QString & format);
impl<'a> /*trait*/ QLocale_toDate<QDate> for (&'a QString, &'a QString) {
  fn toDate(self , rsthis: & QLocale) -> QDate {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale6toDateERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK7QLocale6toDateERK7QStringS2_(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QDate::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QLocale::nativeCountryName();
impl /*struct*/ QLocale {
  pub fn nativeCountryName<RetType, T: QLocale_nativeCountryName<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.nativeCountryName(self);
    // return 1;
  }
}

pub trait QLocale_nativeCountryName<RetType> {
  fn nativeCountryName(self , rsthis: & QLocale) -> RetType;
}

  // proto:  QString QLocale::nativeCountryName();
impl<'a> /*trait*/ QLocale_nativeCountryName<QString> for () {
  fn nativeCountryName(self , rsthis: & QLocale) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale17nativeCountryNameEv()};
    let mut ret = unsafe {C_ZNK7QLocale17nativeCountryNameEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QChar QLocale::negativeSign();
impl /*struct*/ QLocale {
  pub fn negativeSign<RetType, T: QLocale_negativeSign<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.negativeSign(self);
    // return 1;
  }
}

pub trait QLocale_negativeSign<RetType> {
  fn negativeSign(self , rsthis: & QLocale) -> RetType;
}

  // proto:  QChar QLocale::negativeSign();
impl<'a> /*trait*/ QLocale_negativeSign<QChar> for () {
  fn negativeSign(self , rsthis: & QLocale) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale12negativeSignEv()};
    let mut ret = unsafe {C_ZNK7QLocale12negativeSignEv(rsthis.qclsinst)};
    let mut ret1 = QChar::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QLocale::~QLocale();
impl /*struct*/ QLocale {
  pub fn free<RetType, T: QLocale_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QLocale_free<RetType> {
  fn free(self , rsthis: & QLocale) -> RetType;
}

  // proto:  void QLocale::~QLocale();
impl<'a> /*trait*/ QLocale_free<()> for () {
  fn free(self , rsthis: & QLocale) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QLocaleD2Ev()};
     unsafe {C_ZN7QLocaleD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QLocale::QLocale(const QLocale & other);
impl<'a> /*trait*/ QLocale_new for (&'a QLocale) {
  fn new(self) -> QLocale {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QLocaleC2ERKS_()};
    let ctysz: c_int = unsafe{QLocale_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN7QLocaleC2ERKS_(arg0)};
    let rsthis = QLocale{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QString QLocale::toString(const QDate & date, const QString & formatStr);
impl<'a> /*trait*/ QLocale_toString<QString> for (&'a QDate, &'a QString) {
  fn toString(self , rsthis: & QLocale) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale8toStringERK5QDateRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK7QLocale8toStringERK5QDateRK7QString(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QLocale::toUpper(const QString & str);
impl /*struct*/ QLocale {
  pub fn toUpper<RetType, T: QLocale_toUpper<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toUpper(self);
    // return 1;
  }
}

pub trait QLocale_toUpper<RetType> {
  fn toUpper(self , rsthis: & QLocale) -> RetType;
}

  // proto:  QString QLocale::toUpper(const QString & str);
impl<'a> /*trait*/ QLocale_toUpper<QString> for (&'a QString) {
  fn toUpper(self , rsthis: & QLocale) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale7toUpperERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK7QLocale7toUpperERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QChar QLocale::percent();
impl /*struct*/ QLocale {
  pub fn percent<RetType, T: QLocale_percent<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.percent(self);
    // return 1;
  }
}

pub trait QLocale_percent<RetType> {
  fn percent(self , rsthis: & QLocale) -> RetType;
}

  // proto:  QChar QLocale::percent();
impl<'a> /*trait*/ QLocale_percent<QChar> for () {
  fn percent(self , rsthis: & QLocale) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale7percentEv()};
    let mut ret = unsafe {C_ZNK7QLocale7percentEv(rsthis.qclsinst)};
    let mut ret1 = QChar::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  qulonglong QLocale::toULongLong(const QString & s, bool * ok);
impl /*struct*/ QLocale {
  pub fn toULongLong<RetType, T: QLocale_toULongLong<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toULongLong(self);
    // return 1;
  }
}

pub trait QLocale_toULongLong<RetType> {
  fn toULongLong(self , rsthis: & QLocale) -> RetType;
}

  // proto:  qulonglong QLocale::toULongLong(const QString & s, bool * ok);
impl<'a> /*trait*/ QLocale_toULongLong<u64> for (&'a QString, &'a mut Vec<i8>) {
  fn toULongLong(self , rsthis: & QLocale) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale11toULongLongERK7QStringPb()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *mut c_char;
    let mut ret = unsafe {C_ZNK7QLocale11toULongLongERK7QStringPb(rsthis.qclsinst, arg0, arg1)};
    return ret as u64;
    // return 1;
  }
}

  // proto:  QString QLocale::toString(double i, char f, int prec);
impl<'a> /*trait*/ QLocale_toString<QString> for (f64, i8, i32) {
  fn toString(self , rsthis: & QLocale) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale8toStringEdci()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_char;
    let arg2 = self.2  as c_int;
    let mut ret = unsafe {C_ZNK7QLocale8toStringEdci(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QStringList QLocale::uiLanguages();
impl /*struct*/ QLocale {
  pub fn uiLanguages<RetType, T: QLocale_uiLanguages<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.uiLanguages(self);
    // return 1;
  }
}

pub trait QLocale_uiLanguages<RetType> {
  fn uiLanguages(self , rsthis: & QLocale) -> RetType;
}

  // proto:  QStringList QLocale::uiLanguages();
impl<'a> /*trait*/ QLocale_uiLanguages<()> for () {
  fn uiLanguages(self , rsthis: & QLocale) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale11uiLanguagesEv()};
     unsafe {C_ZNK7QLocale11uiLanguagesEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QString QLocale::bcp47Name();
impl /*struct*/ QLocale {
  pub fn bcp47Name<RetType, T: QLocale_bcp47Name<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.bcp47Name(self);
    // return 1;
  }
}

pub trait QLocale_bcp47Name<RetType> {
  fn bcp47Name(self , rsthis: & QLocale) -> RetType;
}

  // proto:  QString QLocale::bcp47Name();
impl<'a> /*trait*/ QLocale_bcp47Name<QString> for () {
  fn bcp47Name(self , rsthis: & QLocale) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale9bcp47NameEv()};
    let mut ret = unsafe {C_ZNK7QLocale9bcp47NameEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QTime QLocale::toTime(const QString & string, const QString & format);
impl /*struct*/ QLocale {
  pub fn toTime<RetType, T: QLocale_toTime<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toTime(self);
    // return 1;
  }
}

pub trait QLocale_toTime<RetType> {
  fn toTime(self , rsthis: & QLocale) -> RetType;
}

  // proto:  QTime QLocale::toTime(const QString & string, const QString & format);
impl<'a> /*trait*/ QLocale_toTime<QTime> for (&'a QString, &'a QString) {
  fn toTime(self , rsthis: & QLocale) -> QTime {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale6toTimeERK7QStringS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK7QLocale6toTimeERK7QStringS2_(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QTime::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  ushort QLocale::toUShort(const QString & s, bool * ok);
impl /*struct*/ QLocale {
  pub fn toUShort<RetType, T: QLocale_toUShort<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toUShort(self);
    // return 1;
  }
}

pub trait QLocale_toUShort<RetType> {
  fn toUShort(self , rsthis: & QLocale) -> RetType;
}

  // proto:  ushort QLocale::toUShort(const QString & s, bool * ok);
impl<'a> /*trait*/ QLocale_toUShort<u16> for (&'a QString, &'a mut Vec<i8>) {
  fn toUShort(self , rsthis: & QLocale) -> u16 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale8toUShortERK7QStringPb()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *mut c_char;
    let mut ret = unsafe {C_ZNK7QLocale8toUShortERK7QStringPb(rsthis.qclsinst, arg0, arg1)};
    return ret as u16;
    // return 1;
  }
}

  // proto:  QString QLocale::toCurrencyString(ushort , const QString & symbol);
impl<'a> /*trait*/ QLocale_toCurrencyString<QString> for (u16, &'a QString) {
  fn toCurrencyString(self , rsthis: & QLocale) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale16toCurrencyStringEtRK7QString()};
    let arg0 = self.0  as c_ushort;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK7QLocale16toCurrencyStringEtRK7QString(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  double QLocale::toDouble(const QString & s, bool * ok);
impl /*struct*/ QLocale {
  pub fn toDouble<RetType, T: QLocale_toDouble<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toDouble(self);
    // return 1;
  }
}

pub trait QLocale_toDouble<RetType> {
  fn toDouble(self , rsthis: & QLocale) -> RetType;
}

  // proto:  double QLocale::toDouble(const QString & s, bool * ok);
impl<'a> /*trait*/ QLocale_toDouble<f64> for (&'a QString, &'a mut Vec<i8>) {
  fn toDouble(self , rsthis: & QLocale) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale8toDoubleERK7QStringPb()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *mut c_char;
    let mut ret = unsafe {C_ZNK7QLocale8toDoubleERK7QStringPb(rsthis.qclsinst, arg0, arg1)};
    return ret as f64;
    // return 1;
  }
}

  // proto: static QLocale QLocale::system();
impl /*struct*/ QLocale {
  pub fn system_s<RetType, T: QLocale_system_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.system_s();
    // return 1;
  }
}

pub trait QLocale_system_s<RetType> {
  fn system_s(self ) -> RetType;
}

  // proto: static QLocale QLocale::system();
impl<'a> /*trait*/ QLocale_system_s<QLocale> for () {
  fn system_s(self ) -> QLocale {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QLocale6systemEv()};
    let mut ret = unsafe {C_ZN7QLocale6systemEv()};
    let mut ret1 = QLocale::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static void QLocale::setDefault(const QLocale & locale);
impl /*struct*/ QLocale {
  pub fn setDefault_s<RetType, T: QLocale_setDefault_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setDefault_s();
    // return 1;
  }
}

pub trait QLocale_setDefault_s<RetType> {
  fn setDefault_s(self ) -> RetType;
}

  // proto: static void QLocale::setDefault(const QLocale & locale);
impl<'a> /*trait*/ QLocale_setDefault_s<()> for (&'a QLocale) {
  fn setDefault_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QLocale10setDefaultERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN7QLocale10setDefaultERKS_(arg0)};
    // return 1;
  }
}

  // proto:  QChar QLocale::exponential();
impl /*struct*/ QLocale {
  pub fn exponential<RetType, T: QLocale_exponential<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.exponential(self);
    // return 1;
  }
}

pub trait QLocale_exponential<RetType> {
  fn exponential(self , rsthis: & QLocale) -> RetType;
}

  // proto:  QChar QLocale::exponential();
impl<'a> /*trait*/ QLocale_exponential<QChar> for () {
  fn exponential(self , rsthis: & QLocale) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale11exponentialEv()};
    let mut ret = unsafe {C_ZNK7QLocale11exponentialEv(rsthis.qclsinst)};
    let mut ret1 = QChar::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QLocale::toCurrencyString(float , const QString & symbol);
impl<'a> /*trait*/ QLocale_toCurrencyString<QString> for (f32, &'a QString) {
  fn toCurrencyString(self , rsthis: & QLocale) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale16toCurrencyStringEfRK7QString()};
    let arg0 = self.0  as c_float;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK7QLocale16toCurrencyStringEfRK7QString(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QLocale::toString(int i);
impl<'a> /*trait*/ QLocale_toString<QString> for (i32) {
  fn toString(self , rsthis: & QLocale) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale8toStringEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZNK7QLocale8toStringEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QLocale::toString(uint i);
impl<'a> /*trait*/ QLocale_toString<QString> for (u32) {
  fn toString(self , rsthis: & QLocale) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale8toStringEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {C_ZNK7QLocale8toStringEj(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QLocale::toString(qlonglong i);
impl<'a> /*trait*/ QLocale_toString<QString> for (i64) {
  fn toString(self , rsthis: & QLocale) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale8toStringEx()};
    let arg0 = self  as c_longlong;
    let mut ret = unsafe {C_ZNK7QLocale8toStringEx(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QLocale::toString(qulonglong i);
impl<'a> /*trait*/ QLocale_toString<QString> for (u64) {
  fn toString(self , rsthis: & QLocale) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale8toStringEy()};
    let arg0 = self  as c_ulonglong;
    let mut ret = unsafe {C_ZNK7QLocale8toStringEy(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QLocale::toString(ushort i);
impl<'a> /*trait*/ QLocale_toString<QString> for (u16) {
  fn toString(self , rsthis: & QLocale) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale8toStringEt()};
    let arg0 = self  as c_ushort;
    let mut ret = unsafe {C_ZNK7QLocale8toStringEt(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QLocale::amText();
impl /*struct*/ QLocale {
  pub fn amText<RetType, T: QLocale_amText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.amText(self);
    // return 1;
  }
}

pub trait QLocale_amText<RetType> {
  fn amText(self , rsthis: & QLocale) -> RetType;
}

  // proto:  QString QLocale::amText();
impl<'a> /*trait*/ QLocale_amText<QString> for () {
  fn amText(self , rsthis: & QLocale) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale6amTextEv()};
    let mut ret = unsafe {C_ZNK7QLocale6amTextEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QLocale::toCurrencyString(int , const QString & symbol);
impl<'a> /*trait*/ QLocale_toCurrencyString<QString> for (i32, &'a QString) {
  fn toCurrencyString(self , rsthis: & QLocale) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale16toCurrencyStringEiRK7QString()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK7QLocale16toCurrencyStringEiRK7QString(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QLocale::toString(short i);
impl<'a> /*trait*/ QLocale_toString<QString> for (i16) {
  fn toString(self , rsthis: & QLocale) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale8toStringEs()};
    let arg0 = self  as c_short;
    let mut ret = unsafe {C_ZNK7QLocale8toStringEs(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QLocale::QLocale();
impl<'a> /*trait*/ QLocale_new for () {
  fn new(self) -> QLocale {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QLocaleC2Ev()};
    let ctysz: c_int = unsafe{QLocale_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN7QLocaleC2Ev()};
    let rsthis = QLocale{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QLocale::toInt(const QString & s, bool * ok);
impl /*struct*/ QLocale {
  pub fn toInt<RetType, T: QLocale_toInt<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toInt(self);
    // return 1;
  }
}

pub trait QLocale_toInt<RetType> {
  fn toInt(self , rsthis: & QLocale) -> RetType;
}

  // proto:  int QLocale::toInt(const QString & s, bool * ok);
impl<'a> /*trait*/ QLocale_toInt<i32> for (&'a QString, &'a mut Vec<i8>) {
  fn toInt(self , rsthis: & QLocale) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QLocale5toIntERK7QStringPb()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *mut c_char;
    let mut ret = unsafe {C_ZNK7QLocale5toIntERK7QStringPb(rsthis.qclsinst, arg0, arg1)};
    return ret as i32;
    // return 1;
  }
}

// <= body block end

