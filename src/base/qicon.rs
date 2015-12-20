// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qpainter::QPainter;
use super::qrect::QRect;
use super::qstring::QString;
use super::qsize::QSize;
use super::qwindow::QWindow;
use super::qstringlist::QStringList;
use super::qpixmap::QPixmap;
use super::qiconengine::QIconEngine;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QIcon::QIcon(const QIcon & other);
  fn _ZN5QIconC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto: static QStringList QIcon::themeSearchPaths();
  fn _ZN5QIcon16themeSearchPathsEv();
  // proto:  void QIcon::detach();
  fn _ZN5QIcon6detachEv(qthis: *mut c_void);
  // proto:  bool QIcon::isNull();
  fn _ZNK5QIcon6isNullEv(qthis: *mut c_void) -> c_char;
  // proto: static void QIcon::setThemeSearchPaths(const QStringList & searchpath);
  fn _ZN5QIcon19setThemeSearchPathsERK11QStringList(arg0: *mut c_void);
  // proto: static bool QIcon::hasThemeIcon(const QString & name);
  fn _ZN5QIcon12hasThemeIconERK7QString(arg0: *mut c_void) -> c_char;
  // proto:  void QIcon::QIcon(const QPixmap & pixmap);
  fn _ZN5QIconC1ERK7QPixmap(qthis: *mut c_void, arg0: *mut c_void);
  // proto: static QIcon QIcon::fromTheme(const QString & name, const QIcon & fallback);
  fn _ZN5QIcon9fromThemeERK7QStringRKS_(arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto: static QString QIcon::themeName();
  fn _ZN5QIcon9themeNameEv() -> *mut c_void;
  // proto:  QString QIcon::name();
  fn _ZNK5QIcon4nameEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QIcon::QIcon();
  fn _ZN5QIconC1Ev(qthis: *mut c_void);
  // proto:  void QIcon::QIcon(QIconEngine * engine);
  fn _ZN5QIconC1EP11QIconEngine(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QIcon::~QIcon();
  fn _ZN5QIconD0Ev(qthis: *mut c_void);
  // proto:  bool QIcon::isDetached();
  fn _ZNK5QIcon10isDetachedEv(qthis: *mut c_void) -> c_char;
  // proto:  void QIcon::QIcon(const QString & fileName);
  fn _ZN5QIconC1ERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  qint64 QIcon::cacheKey();
  fn _ZNK5QIcon8cacheKeyEv(qthis: *mut c_void) -> c_longlong;
  // proto:  void QIcon::swap(QIcon & other);
  fn _ZN5QIcon4swapERS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto: static void QIcon::setThemeName(const QString & path);
  fn _ZN5QIcon12setThemeNameERK7QString(arg0: *mut c_void);
}

// body block begin
// class sizeof(QIcon)=8
pub struct QIcon {
  pub qclsinst: *mut c_void,
}

  // proto:  void QIcon::QIcon(const QIcon & other);
impl /*struct*/ QIcon {
  pub fn NewQIcon<T: QIcon_NewQIcon>(value: T) -> QIcon {
    let rsthis = value.NewQIcon();
    return rsthis;
    // return 1;
  }
}

pub trait QIcon_NewQIcon {
  fn NewQIcon(self) -> QIcon;
}

  // proto:  void QIcon::QIcon(const QIcon & other);
impl<'a> /*trait*/ QIcon_NewQIcon for (QIcon) {
  fn NewQIcon(self) -> QIcon {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QIconC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN5QIconC1ERKS_(qthis, arg0)};
    let rsthis = QIcon{qclsinst: qthis};
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
     unsafe {_ZN5QIcon16themeSearchPathsEv()};
    // return 1;
  }
}

  // proto:  void QIcon::detach();
impl /*struct*/ QIcon {
  pub fn detach<RetType, T: QIcon_detach<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.detach(self);
    // return 1;
  }
}

pub trait QIcon_detach<RetType> {
  fn detach(self , rsthis: &mut QIcon) -> RetType;
}

  // proto:  void QIcon::detach();
impl<'a> /*trait*/ QIcon_detach<()> for () {
  fn detach(self , rsthis: &mut QIcon) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QIcon6detachEv()};
     unsafe {_ZN5QIcon6detachEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QIcon::isNull();
impl /*struct*/ QIcon {
  pub fn isNull<RetType, T: QIcon_isNull<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isNull(self);
    // return 1;
  }
}

pub trait QIcon_isNull<RetType> {
  fn isNull(self , rsthis: &mut QIcon) -> RetType;
}

  // proto:  bool QIcon::isNull();
impl<'a> /*trait*/ QIcon_isNull<i8> for () {
  fn isNull(self , rsthis: &mut QIcon) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QIcon6isNullEv()};
    let mut ret = unsafe {_ZNK5QIcon6isNullEv(rsthis.qclsinst)};
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
impl<'a> /*trait*/ QIcon_setThemeSearchPaths_s<()> for (QStringList) {
  fn setThemeSearchPaths_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QIcon19setThemeSearchPathsERK11QStringList()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN5QIcon19setThemeSearchPathsERK11QStringList(arg0)};
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
impl<'a> /*trait*/ QIcon_hasThemeIcon_s<i8> for (QString) {
  fn hasThemeIcon_s(self ) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QIcon12hasThemeIconERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN5QIcon12hasThemeIconERK7QString(arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QIcon::QIcon(const QPixmap & pixmap);
impl<'a> /*trait*/ QIcon_NewQIcon for (QPixmap) {
  fn NewQIcon(self) -> QIcon {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QIconC1ERK7QPixmap()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN5QIconC1ERK7QPixmap(qthis, arg0)};
    let rsthis = QIcon{qclsinst: qthis};
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
impl<'a> /*trait*/ QIcon_fromTheme_s<QIcon> for (QString, QIcon) {
  fn fromTheme_s(self ) -> QIcon {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QIcon9fromThemeERK7QStringRKS_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN5QIcon9fromThemeERK7QStringRKS_(arg0, arg1)};
    let mut ret1 = QIcon{qclsinst: ret};
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
    let mut ret = unsafe {_ZN5QIcon9themeNameEv()};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QString QIcon::name();
impl /*struct*/ QIcon {
  pub fn name<RetType, T: QIcon_name<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.name(self);
    // return 1;
  }
}

pub trait QIcon_name<RetType> {
  fn name(self , rsthis: &mut QIcon) -> RetType;
}

  // proto:  QString QIcon::name();
impl<'a> /*trait*/ QIcon_name<QString> for () {
  fn name(self , rsthis: &mut QIcon) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QIcon4nameEv()};
    let mut ret = unsafe {_ZNK5QIcon4nameEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QIcon::QIcon();
impl<'a> /*trait*/ QIcon_NewQIcon for () {
  fn NewQIcon(self) -> QIcon {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QIconC1Ev()};
    unsafe {_ZN5QIconC1Ev(qthis)};
    let rsthis = QIcon{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QIcon::QIcon(QIconEngine * engine);
impl<'a> /*trait*/ QIcon_NewQIcon for (QIconEngine) {
  fn NewQIcon(self) -> QIcon {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QIconC1EP11QIconEngine()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN5QIconC1EP11QIconEngine(qthis, arg0)};
    let rsthis = QIcon{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QIcon::~QIcon();
impl /*struct*/ QIcon {
  pub fn FreeQIcon<RetType, T: QIcon_FreeQIcon<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQIcon(self);
    // return 1;
  }
}

pub trait QIcon_FreeQIcon<RetType> {
  fn FreeQIcon(self , rsthis: &mut QIcon) -> RetType;
}

  // proto:  void QIcon::~QIcon();
impl<'a> /*trait*/ QIcon_FreeQIcon<()> for () {
  fn FreeQIcon(self , rsthis: &mut QIcon) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QIconD0Ev()};
     unsafe {_ZN5QIconD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QIcon::isDetached();
impl /*struct*/ QIcon {
  pub fn isDetached<RetType, T: QIcon_isDetached<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isDetached(self);
    // return 1;
  }
}

pub trait QIcon_isDetached<RetType> {
  fn isDetached(self , rsthis: &mut QIcon) -> RetType;
}

  // proto:  bool QIcon::isDetached();
impl<'a> /*trait*/ QIcon_isDetached<i8> for () {
  fn isDetached(self , rsthis: &mut QIcon) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QIcon10isDetachedEv()};
    let mut ret = unsafe {_ZNK5QIcon10isDetachedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QIcon::QIcon(const QString & fileName);
impl<'a> /*trait*/ QIcon_NewQIcon for (QString) {
  fn NewQIcon(self) -> QIcon {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QIconC1ERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN5QIconC1ERK7QString(qthis, arg0)};
    let rsthis = QIcon{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  qint64 QIcon::cacheKey();
impl /*struct*/ QIcon {
  pub fn cacheKey<RetType, T: QIcon_cacheKey<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.cacheKey(self);
    // return 1;
  }
}

pub trait QIcon_cacheKey<RetType> {
  fn cacheKey(self , rsthis: &mut QIcon) -> RetType;
}

  // proto:  qint64 QIcon::cacheKey();
impl<'a> /*trait*/ QIcon_cacheKey<i64> for () {
  fn cacheKey(self , rsthis: &mut QIcon) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QIcon8cacheKeyEv()};
    let mut ret = unsafe {_ZNK5QIcon8cacheKeyEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

  // proto:  void QIcon::swap(QIcon & other);
impl /*struct*/ QIcon {
  pub fn swap<RetType, T: QIcon_swap<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QIcon_swap<RetType> {
  fn swap(self , rsthis: &mut QIcon) -> RetType;
}

  // proto:  void QIcon::swap(QIcon & other);
impl<'a> /*trait*/ QIcon_swap<()> for (QIcon) {
  fn swap(self , rsthis: &mut QIcon) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QIcon4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN5QIcon4swapERS_(rsthis.qclsinst, arg0)};
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
impl<'a> /*trait*/ QIcon_setThemeName_s<()> for (QString) {
  fn setThemeName_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QIcon12setThemeNameERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN5QIcon12setThemeNameERK7QString(arg0)};
    // return 1;
  }
}

