// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qscreen::QScreen;
use super::qsurfaceformat::QSurfaceFormat;
use super::qsize::QSize;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QOffscreenSurface::FreeQOffscreenSurface();
  fn _ZN17QOffscreenSurfaceD0Ev(qthis: *mut c_void) ;
  // proto:  void QOffscreenSurface::screenChanged(QScreen * screen);
  fn _ZN17QOffscreenSurface13screenChangedEP7QScreen(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QOffscreenSurface::NewQOffscreenSurface(const QOffscreenSurface & );
  fn _ZN17QOffscreenSurfaceC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QOffscreenSurface::NewQOffscreenSurface(QScreen * screen);
  fn _ZN17QOffscreenSurfaceC1EP7QScreen(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QScreen * QOffscreenSurface::screen();
  fn _ZNK17QOffscreenSurface6screenEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QOffscreenSurface::setFormat(const QSurfaceFormat & format);
  fn _ZN17QOffscreenSurface9setFormatERK14QSurfaceFormat(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QOffscreenSurface::setScreen(QScreen * screen);
  fn _ZN17QOffscreenSurface9setScreenEP7QScreen(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QSurfaceFormat QOffscreenSurface::requestedFormat();
  fn _ZNK17QOffscreenSurface15requestedFormatEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QSurfaceFormat QOffscreenSurface::format();
  fn _ZNK17QOffscreenSurface6formatEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QPlatformOffscreenSurface * QOffscreenSurface::handle();
  fn _ZNK17QOffscreenSurface6handleEv(qthis: *mut c_void) ;
  // proto:  const QMetaObject * QOffscreenSurface::metaObject();
  fn _ZNK17QOffscreenSurface10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QOffscreenSurface::destroy();
  fn _ZN17QOffscreenSurface7destroyEv(qthis: *mut c_void) ;
  // proto:  bool QOffscreenSurface::isValid();
  fn _ZNK17QOffscreenSurface7isValidEv(qthis: *mut c_void) -> int8_t;
  // proto:  QSize QOffscreenSurface::size();
  fn _ZNK17QOffscreenSurface4sizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QOffscreenSurface::create();
  fn _ZN17QOffscreenSurface6createEv(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QOffscreenSurface)=1
pub struct QOffscreenSurface {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QOffscreenSurface {
  pub fn FreeQOffscreenSurface<RetType, T: QOffscreenSurface_FreeQOffscreenSurface<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQOffscreenSurface(self);
    // return 1;
  }
}

pub trait QOffscreenSurface_FreeQOffscreenSurface<RetType> {
  fn FreeQOffscreenSurface(self, rsthis: &mut QOffscreenSurface) -> RetType;
}

// proto:  void QOffscreenSurface::FreeQOffscreenSurface();
impl<'a> /*trait*/ QOffscreenSurface_FreeQOffscreenSurface<()> for () {
  fn FreeQOffscreenSurface(self, rsthis: &mut QOffscreenSurface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QOffscreenSurfaceD0Ev()};
     unsafe {_ZN17QOffscreenSurfaceD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOffscreenSurface {
  pub fn screenChanged<RetType, T: QOffscreenSurface_screenChanged<RetType>>(&mut self, value: T) -> RetType {
    return value.screenChanged(self);
    // return 1;
  }
}

pub trait QOffscreenSurface_screenChanged<RetType> {
  fn screenChanged(self, rsthis: &mut QOffscreenSurface) -> RetType;
}

// proto:  void QOffscreenSurface::screenChanged(QScreen * screen);
impl<'a> /*trait*/ QOffscreenSurface_screenChanged<()> for (&'a mut QScreen) {
  fn screenChanged(self, rsthis: &mut QOffscreenSurface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QOffscreenSurface13screenChangedEP7QScreen()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN17QOffscreenSurface13screenChangedEP7QScreen(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QOffscreenSurface {
  pub fn NewQOffscreenSurface<T: QOffscreenSurface_NewQOffscreenSurface>(value: T) -> QOffscreenSurface {
    let rsthis = value.NewQOffscreenSurface();
    return rsthis;
    // return 1;
  }
}

pub trait QOffscreenSurface_NewQOffscreenSurface {
  fn NewQOffscreenSurface(self) -> QOffscreenSurface;
}

// proto: void QOffscreenSurface::NewQOffscreenSurface(const QOffscreenSurface & );
impl<'a> /*trait*/ QOffscreenSurface_NewQOffscreenSurface for (&'a  QOffscreenSurface) {
  fn NewQOffscreenSurface(self) -> QOffscreenSurface {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QOffscreenSurfaceC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN17QOffscreenSurfaceC1ERKS_(qthis, arg0)};
    let rsthis = QOffscreenSurface{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QOffscreenSurface::NewQOffscreenSurface(QScreen * screen);
impl<'a> /*trait*/ QOffscreenSurface_NewQOffscreenSurface for (&'a mut QScreen) {
  fn NewQOffscreenSurface(self) -> QOffscreenSurface {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QOffscreenSurfaceC1EP7QScreen()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN17QOffscreenSurfaceC1EP7QScreen(qthis, arg0)};
    let rsthis = QOffscreenSurface{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QOffscreenSurface {
  pub fn screen<RetType, T: QOffscreenSurface_screen<RetType>>(&mut self, value: T) -> RetType {
    return value.screen(self);
    // return 1;
  }
}

pub trait QOffscreenSurface_screen<RetType> {
  fn screen(self, rsthis: &mut QOffscreenSurface) -> RetType;
}

// proto:  QScreen * QOffscreenSurface::screen();
impl<'a> /*trait*/ QOffscreenSurface_screen<QScreen> for () {
  fn screen(self, rsthis: &mut QOffscreenSurface) -> QScreen {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QOffscreenSurface6screenEv()};
    let mut ret = unsafe {_ZNK17QOffscreenSurface6screenEv(rsthis.qclsinst)};
    let mut ret1 = QScreen{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QOffscreenSurface {
  pub fn setFormat<RetType, T: QOffscreenSurface_setFormat<RetType>>(&mut self, value: T) -> RetType {
    return value.setFormat(self);
    // return 1;
  }
}

pub trait QOffscreenSurface_setFormat<RetType> {
  fn setFormat(self, rsthis: &mut QOffscreenSurface) -> RetType;
}

// proto:  void QOffscreenSurface::setFormat(const QSurfaceFormat & format);
impl<'a> /*trait*/ QOffscreenSurface_setFormat<()> for (&'a  QSurfaceFormat) {
  fn setFormat(self, rsthis: &mut QOffscreenSurface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QOffscreenSurface9setFormatERK14QSurfaceFormat()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN17QOffscreenSurface9setFormatERK14QSurfaceFormat(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QOffscreenSurface {
  pub fn setScreen<RetType, T: QOffscreenSurface_setScreen<RetType>>(&mut self, value: T) -> RetType {
    return value.setScreen(self);
    // return 1;
  }
}

pub trait QOffscreenSurface_setScreen<RetType> {
  fn setScreen(self, rsthis: &mut QOffscreenSurface) -> RetType;
}

// proto:  void QOffscreenSurface::setScreen(QScreen * screen);
impl<'a> /*trait*/ QOffscreenSurface_setScreen<()> for (&'a mut QScreen) {
  fn setScreen(self, rsthis: &mut QOffscreenSurface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QOffscreenSurface9setScreenEP7QScreen()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN17QOffscreenSurface9setScreenEP7QScreen(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QOffscreenSurface {
  pub fn requestedFormat<RetType, T: QOffscreenSurface_requestedFormat<RetType>>(&mut self, value: T) -> RetType {
    return value.requestedFormat(self);
    // return 1;
  }
}

pub trait QOffscreenSurface_requestedFormat<RetType> {
  fn requestedFormat(self, rsthis: &mut QOffscreenSurface) -> RetType;
}

// proto:  QSurfaceFormat QOffscreenSurface::requestedFormat();
impl<'a> /*trait*/ QOffscreenSurface_requestedFormat<QSurfaceFormat> for () {
  fn requestedFormat(self, rsthis: &mut QOffscreenSurface) -> QSurfaceFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QOffscreenSurface15requestedFormatEv()};
    let mut ret = unsafe {_ZNK17QOffscreenSurface15requestedFormatEv(rsthis.qclsinst)};
    let mut ret1 = QSurfaceFormat{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QOffscreenSurface {
  pub fn format<RetType, T: QOffscreenSurface_format<RetType>>(&mut self, value: T) -> RetType {
    return value.format(self);
    // return 1;
  }
}

pub trait QOffscreenSurface_format<RetType> {
  fn format(self, rsthis: &mut QOffscreenSurface) -> RetType;
}

// proto:  QSurfaceFormat QOffscreenSurface::format();
impl<'a> /*trait*/ QOffscreenSurface_format<QSurfaceFormat> for () {
  fn format(self, rsthis: &mut QOffscreenSurface) -> QSurfaceFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QOffscreenSurface6formatEv()};
    let mut ret = unsafe {_ZNK17QOffscreenSurface6formatEv(rsthis.qclsinst)};
    let mut ret1 = QSurfaceFormat{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QOffscreenSurface {
  pub fn handle<RetType, T: QOffscreenSurface_handle<RetType>>(&mut self, value: T) -> RetType {
    return value.handle(self);
    // return 1;
  }
}

pub trait QOffscreenSurface_handle<RetType> {
  fn handle(self, rsthis: &mut QOffscreenSurface) -> RetType;
}

// proto:  QPlatformOffscreenSurface * QOffscreenSurface::handle();
impl<'a> /*trait*/ QOffscreenSurface_handle<()> for () {
  fn handle(self, rsthis: &mut QOffscreenSurface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QOffscreenSurface6handleEv()};
     unsafe {_ZNK17QOffscreenSurface6handleEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOffscreenSurface {
  pub fn metaObject<RetType, T: QOffscreenSurface_metaObject<RetType>>(&mut self, value: T) -> RetType {
    return value.metaObject(self);
    // return 1;
  }
}

pub trait QOffscreenSurface_metaObject<RetType> {
  fn metaObject(self, rsthis: &mut QOffscreenSurface) -> RetType;
}

// proto:  const QMetaObject * QOffscreenSurface::metaObject();
impl<'a> /*trait*/ QOffscreenSurface_metaObject<()> for () {
  fn metaObject(self, rsthis: &mut QOffscreenSurface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QOffscreenSurface10metaObjectEv()};
     unsafe {_ZNK17QOffscreenSurface10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOffscreenSurface {
  pub fn destroy<RetType, T: QOffscreenSurface_destroy<RetType>>(&mut self, value: T) -> RetType {
    return value.destroy(self);
    // return 1;
  }
}

pub trait QOffscreenSurface_destroy<RetType> {
  fn destroy(self, rsthis: &mut QOffscreenSurface) -> RetType;
}

// proto:  void QOffscreenSurface::destroy();
impl<'a> /*trait*/ QOffscreenSurface_destroy<()> for () {
  fn destroy(self, rsthis: &mut QOffscreenSurface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QOffscreenSurface7destroyEv()};
     unsafe {_ZN17QOffscreenSurface7destroyEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QOffscreenSurface {
  pub fn isValid<RetType, T: QOffscreenSurface_isValid<RetType>>(&mut self, value: T) -> RetType {
    return value.isValid(self);
    // return 1;
  }
}

pub trait QOffscreenSurface_isValid<RetType> {
  fn isValid(self, rsthis: &mut QOffscreenSurface) -> RetType;
}

// proto:  bool QOffscreenSurface::isValid();
impl<'a> /*trait*/ QOffscreenSurface_isValid<i8> for () {
  fn isValid(self, rsthis: &mut QOffscreenSurface) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QOffscreenSurface7isValidEv()};
    let mut ret = unsafe {_ZNK17QOffscreenSurface7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QOffscreenSurface {
  pub fn size<RetType, T: QOffscreenSurface_size<RetType>>(&mut self, value: T) -> RetType {
    return value.size(self);
    // return 1;
  }
}

pub trait QOffscreenSurface_size<RetType> {
  fn size(self, rsthis: &mut QOffscreenSurface) -> RetType;
}

// proto:  QSize QOffscreenSurface::size();
impl<'a> /*trait*/ QOffscreenSurface_size<QSize> for () {
  fn size(self, rsthis: &mut QOffscreenSurface) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QOffscreenSurface4sizeEv()};
    let mut ret = unsafe {_ZNK17QOffscreenSurface4sizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QOffscreenSurface {
  pub fn create<RetType, T: QOffscreenSurface_create<RetType>>(&mut self, value: T) -> RetType {
    return value.create(self);
    // return 1;
  }
}

pub trait QOffscreenSurface_create<RetType> {
  fn create(self, rsthis: &mut QOffscreenSurface) -> RetType;
}

// proto:  void QOffscreenSurface::create();
impl<'a> /*trait*/ QOffscreenSurface_create<()> for () {
  fn create(self, rsthis: &mut QOffscreenSurface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QOffscreenSurface6createEv()};
     unsafe {_ZN17QOffscreenSurface6createEv(rsthis.qclsinst)};
    // return 1;
  }
}

