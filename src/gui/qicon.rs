// auto generated, do not modify.
// created: Sun Jan 24 17:41:38 2016
// src-file: /QtGui/qicon.h
// dst-file: /src/gui/qicon.rs
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
use super::qpainter::QPainter; // 773
use super::super::core::qrect::QRect; // 771
use super::super::core::qstring::QString; // 771
use super::super::core::qsize::QSize; // 771
use super::qwindow::QWindow; // 773
use super::super::core::qstringlist::QStringList; // 771
use super::qpixmap::QPixmap; // 773
use super::qiconengine::QIconEngine; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QIcon_Class_Size() -> c_int;
  // proto:  void QIcon::QIcon(const QIcon & other);
  fn C_ZN5QIconC2ERKS_(arg0: *mut c_void) -> u64;
  // proto: static QStringList QIcon::themeSearchPaths();
  fn C_ZN5QIcon16themeSearchPathsEv();
  // proto:  void QIcon::detach();
  fn C_ZN5QIcon6detachEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QIcon::isNull();
  fn C_ZNK5QIcon6isNullEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto: static void QIcon::setThemeSearchPaths(const QStringList & searchpath);
  fn C_ZN5QIcon19setThemeSearchPathsERK11QStringList(arg0: *mut c_void);
  // proto: static bool QIcon::hasThemeIcon(const QString & name);
  fn C_ZN5QIcon12hasThemeIconERK7QString(arg0: *mut c_void) -> c_char;
  // proto:  void QIcon::QIcon(const QPixmap & pixmap);
  fn C_ZN5QIconC2ERK7QPixmap(arg0: *mut c_void) -> u64;
  // proto: static QIcon QIcon::fromTheme(const QString & name, const QIcon & fallback);
  fn C_ZN5QIcon9fromThemeERK7QStringRKS_(arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto: static QString QIcon::themeName();
  fn C_ZN5QIcon9themeNameEv() -> *mut c_void;
  // proto:  QString QIcon::name();
  fn C_ZNK5QIcon4nameEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QIcon::QIcon();
  fn C_ZN5QIconC2Ev() -> u64;
  // proto:  void QIcon::QIcon(QIconEngine * engine);
  fn C_ZN5QIconC2EP11QIconEngine(arg0: *mut c_void) -> u64;
  // proto:  void QIcon::~QIcon();
  fn C_ZN5QIconD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  bool QIcon::isDetached();
  fn C_ZNK5QIcon10isDetachedEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QIcon::QIcon(const QString & fileName);
  fn C_ZN5QIconC2ERK7QString(arg0: *mut c_void) -> u64;
  // proto:  qint64 QIcon::cacheKey();
  fn C_ZNK5QIcon8cacheKeyEv(qthis: u64 /* *mut c_void*/) -> c_longlong;
  // proto:  void QIcon::swap(QIcon & other);
  fn C_ZN5QIcon4swapERS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto: static void QIcon::setThemeName(const QString & path);
  fn C_ZN5QIcon12setThemeNameERK7QString(arg0: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QIcon)=8
#[derive(Default)]
pub struct QIcon {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QIcon {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QIcon {
    return QIcon{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QIcon::QIcon(const QIcon & other);
impl /*struct*/ QIcon {
  pub fn new<T: QIcon_new>(value: T) -> QIcon {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QIcon_new {
  fn new(self) -> QIcon;
}

  // proto:  void QIcon::QIcon(const QIcon & other);
impl<'a> /*trait*/ QIcon_new for (&'a QIcon) {
  fn new(self) -> QIcon {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QIconC2ERKS_()};
    let ctysz: c_int = unsafe{QIcon_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN5QIconC2ERKS_(arg0)};
    let rsthis = QIcon{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto: static QStringList QIcon::themeSearchPaths();
impl /*struct*/ QIcon {
  pub fn themeSearchPaths_s<RetType, T: QIcon_themeSearchPaths_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.themeSearchPaths_s();
    // return 1;
  }
}

pub trait QIcon_themeSearchPaths_s<RetType> {
  fn themeSearchPaths_s(self ) -> RetType;
}

  // proto: static QStringList QIcon::themeSearchPaths();
impl<'a> /*trait*/ QIcon_themeSearchPaths_s<()> for () {
  fn themeSearchPaths_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QIcon16themeSearchPathsEv()};
     unsafe {C_ZN5QIcon16themeSearchPathsEv()};
    // return 1;
  }
}

  // proto:  void QIcon::detach();
impl /*struct*/ QIcon {
  pub fn detach<RetType, T: QIcon_detach<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.detach(self);
    // return 1;
  }
}

pub trait QIcon_detach<RetType> {
  fn detach(self , rsthis: & QIcon) -> RetType;
}

  // proto:  void QIcon::detach();
impl<'a> /*trait*/ QIcon_detach<()> for () {
  fn detach(self , rsthis: & QIcon) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QIcon6detachEv()};
     unsafe {C_ZN5QIcon6detachEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QIcon::isNull();
impl /*struct*/ QIcon {
  pub fn isNull<RetType, T: QIcon_isNull<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isNull(self);
    // return 1;
  }
}

pub trait QIcon_isNull<RetType> {
  fn isNull(self , rsthis: & QIcon) -> RetType;
}

  // proto:  bool QIcon::isNull();
impl<'a> /*trait*/ QIcon_isNull<i8> for () {
  fn isNull(self , rsthis: & QIcon) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QIcon6isNullEv()};
    let mut ret = unsafe {C_ZNK5QIcon6isNullEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto: static void QIcon::setThemeSearchPaths(const QStringList & searchpath);
impl /*struct*/ QIcon {
  pub fn setThemeSearchPaths_s<RetType, T: QIcon_setThemeSearchPaths_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setThemeSearchPaths_s();
    // return 1;
  }
}

pub trait QIcon_setThemeSearchPaths_s<RetType> {
  fn setThemeSearchPaths_s(self ) -> RetType;
}

  // proto: static void QIcon::setThemeSearchPaths(const QStringList & searchpath);
impl<'a> /*trait*/ QIcon_setThemeSearchPaths_s<()> for (&'a QStringList) {
  fn setThemeSearchPaths_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QIcon19setThemeSearchPathsERK11QStringList()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN5QIcon19setThemeSearchPathsERK11QStringList(arg0)};
    // return 1;
  }
}

  // proto: static bool QIcon::hasThemeIcon(const QString & name);
impl /*struct*/ QIcon {
  pub fn hasThemeIcon_s<RetType, T: QIcon_hasThemeIcon_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.hasThemeIcon_s();
    // return 1;
  }
}

pub trait QIcon_hasThemeIcon_s<RetType> {
  fn hasThemeIcon_s(self ) -> RetType;
}

  // proto: static bool QIcon::hasThemeIcon(const QString & name);
impl<'a> /*trait*/ QIcon_hasThemeIcon_s<i8> for (&'a QString) {
  fn hasThemeIcon_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QIcon12hasThemeIconERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN5QIcon12hasThemeIconERK7QString(arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QIcon::QIcon(const QPixmap & pixmap);
impl<'a> /*trait*/ QIcon_new for (&'a QPixmap) {
  fn new(self) -> QIcon {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QIconC2ERK7QPixmap()};
    let ctysz: c_int = unsafe{QIcon_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN5QIconC2ERK7QPixmap(arg0)};
    let rsthis = QIcon{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto: static QIcon QIcon::fromTheme(const QString & name, const QIcon & fallback);
impl /*struct*/ QIcon {
  pub fn fromTheme_s<RetType, T: QIcon_fromTheme_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.fromTheme_s();
    // return 1;
  }
}

pub trait QIcon_fromTheme_s<RetType> {
  fn fromTheme_s(self ) -> RetType;
}

  // proto: static QIcon QIcon::fromTheme(const QString & name, const QIcon & fallback);
impl<'a> /*trait*/ QIcon_fromTheme_s<QIcon> for (&'a QString, &'a QIcon) {
  fn fromTheme_s(self ) -> QIcon {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QIcon9fromThemeERK7QStringRKS_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN5QIcon9fromThemeERK7QStringRKS_(arg0, arg1)};
    let mut ret1 = QIcon::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static QString QIcon::themeName();
impl /*struct*/ QIcon {
  pub fn themeName_s<RetType, T: QIcon_themeName_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.themeName_s();
    // return 1;
  }
}

pub trait QIcon_themeName_s<RetType> {
  fn themeName_s(self ) -> RetType;
}

  // proto: static QString QIcon::themeName();
impl<'a> /*trait*/ QIcon_themeName_s<QString> for () {
  fn themeName_s(self ) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QIcon9themeNameEv()};
    let mut ret = unsafe {C_ZN5QIcon9themeNameEv()};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QString QIcon::name();
impl /*struct*/ QIcon {
  pub fn name<RetType, T: QIcon_name<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.name(self);
    // return 1;
  }
}

pub trait QIcon_name<RetType> {
  fn name(self , rsthis: & QIcon) -> RetType;
}

  // proto:  QString QIcon::name();
impl<'a> /*trait*/ QIcon_name<QString> for () {
  fn name(self , rsthis: & QIcon) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QIcon4nameEv()};
    let mut ret = unsafe {C_ZNK5QIcon4nameEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QIcon::QIcon();
impl<'a> /*trait*/ QIcon_new for () {
  fn new(self) -> QIcon {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QIconC2Ev()};
    let ctysz: c_int = unsafe{QIcon_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN5QIconC2Ev()};
    let rsthis = QIcon{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QIcon::QIcon(QIconEngine * engine);
impl<'a> /*trait*/ QIcon_new for (&'a QIconEngine) {
  fn new(self) -> QIcon {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QIconC2EP11QIconEngine()};
    let ctysz: c_int = unsafe{QIcon_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN5QIconC2EP11QIconEngine(arg0)};
    let rsthis = QIcon{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QIcon::~QIcon();
impl /*struct*/ QIcon {
  pub fn free<RetType, T: QIcon_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QIcon_free<RetType> {
  fn free(self , rsthis: & QIcon) -> RetType;
}

  // proto:  void QIcon::~QIcon();
impl<'a> /*trait*/ QIcon_free<()> for () {
  fn free(self , rsthis: & QIcon) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QIconD2Ev()};
     unsafe {C_ZN5QIconD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QIcon::isDetached();
impl /*struct*/ QIcon {
  pub fn isDetached<RetType, T: QIcon_isDetached<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isDetached(self);
    // return 1;
  }
}

pub trait QIcon_isDetached<RetType> {
  fn isDetached(self , rsthis: & QIcon) -> RetType;
}

  // proto:  bool QIcon::isDetached();
impl<'a> /*trait*/ QIcon_isDetached<i8> for () {
  fn isDetached(self , rsthis: & QIcon) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QIcon10isDetachedEv()};
    let mut ret = unsafe {C_ZNK5QIcon10isDetachedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QIcon::QIcon(const QString & fileName);
impl<'a> /*trait*/ QIcon_new for (&'a QString) {
  fn new(self) -> QIcon {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QIconC2ERK7QString()};
    let ctysz: c_int = unsafe{QIcon_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN5QIconC2ERK7QString(arg0)};
    let rsthis = QIcon{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  qint64 QIcon::cacheKey();
impl /*struct*/ QIcon {
  pub fn cacheKey<RetType, T: QIcon_cacheKey<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.cacheKey(self);
    // return 1;
  }
}

pub trait QIcon_cacheKey<RetType> {
  fn cacheKey(self , rsthis: & QIcon) -> RetType;
}

  // proto:  qint64 QIcon::cacheKey();
impl<'a> /*trait*/ QIcon_cacheKey<i64> for () {
  fn cacheKey(self , rsthis: & QIcon) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QIcon8cacheKeyEv()};
    let mut ret = unsafe {C_ZNK5QIcon8cacheKeyEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

  // proto:  void QIcon::swap(QIcon & other);
impl /*struct*/ QIcon {
  pub fn swap<RetType, T: QIcon_swap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QIcon_swap<RetType> {
  fn swap(self , rsthis: & QIcon) -> RetType;
}

  // proto:  void QIcon::swap(QIcon & other);
impl<'a> /*trait*/ QIcon_swap<()> for (&'a QIcon) {
  fn swap(self , rsthis: & QIcon) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QIcon4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN5QIcon4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto: static void QIcon::setThemeName(const QString & path);
impl /*struct*/ QIcon {
  pub fn setThemeName_s<RetType, T: QIcon_setThemeName_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setThemeName_s();
    // return 1;
  }
}

pub trait QIcon_setThemeName_s<RetType> {
  fn setThemeName_s(self ) -> RetType;
}

  // proto: static void QIcon::setThemeName(const QString & path);
impl<'a> /*trait*/ QIcon_setThemeName_s<()> for (&'a QString) {
  fn setThemeName_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QIcon12setThemeNameERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN5QIcon12setThemeNameERK7QString(arg0)};
    // return 1;
  }
}

// <= body block end

