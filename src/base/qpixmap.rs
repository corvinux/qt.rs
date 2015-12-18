// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qstring::QString;
use super::qsize::QSize;
use super::qpaintdevice::QPaintDevice;
use super::qcolor::QColor;
use super::qregion::QRegion;
use super::qrect::QRect;
use super::qpoint::QPoint;
use super::qbitmap::QBitmap;
use super::qobject::QObject;
use super::qimage::QImage;
use super::qpaintengine::QPaintEngine;
use super::qiodevice::QIODevice;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  bool QPixmap::save(const QString & fileName, const char * format, int quality);
  fn _ZNK7QPixmap4saveERK7QStringPKci(qthis: *mut c_void, arg0: *mut c_void, arg1: *const c_char, arg2: c_int) -> int8_t;
  // proto:  void QPixmap::swap(QPixmap & other);
  fn _ZN7QPixmap4swapERS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QPixmap::isQBitmap();
  fn _ZNK7QPixmap9isQBitmapEv(qthis: *mut c_void) -> int8_t;
  // proto:  double QPixmap::devicePixelRatio();
  fn _ZNK7QPixmap16devicePixelRatioEv(qthis: *mut c_void) -> c_double;
  // proto:  void QPixmap::NewQPixmap(const QSize & );
  fn _ZN7QPixmapC1ERK5QSize(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QPixmap::fill(const QPaintDevice * device, int xofs, int yofs);
  fn _ZN7QPixmap4fillEPK12QPaintDeviceii(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: c_int) ;
  // proto:  void QPixmap::NewQPixmap(const QSize & s, int type);
  fn _ZN7QPixmapC1ERK5QSizei(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) ;
  // proto:  void QPixmap::fill(const QColor & fillColor);
  fn _ZN7QPixmap4fillERK6QColor(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QPixmap::devType();
  fn _ZNK7QPixmap7devTypeEv(qthis: *mut c_void) -> c_int;
  // proto:  void QPixmap::scroll(int dx, int dy, int x, int y, int width, int height, QRegion * exposed);
  fn _ZN7QPixmap6scrollEiiiiiiP7QRegion(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_int, arg5: c_int, arg6: *mut c_void) ;
  // proto:  QPixmap QPixmap::copy(const QRect & rect);
  fn _ZNK7QPixmap4copyERK5QRect(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QPixmap::NewQPixmap(int w, int h);
  fn _ZN7QPixmapC1Eii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto: static QPixmap QPixmap::grabWindow(WId , int x, int y, int w, int h);
  fn _ZN7QPixmap10grabWindowEiiiii(arg0: *mut c_uint, arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_int) -> *mut c_void;
  // proto:  void QPixmap::fill(const QPaintDevice * device, const QPoint & ofs);
  fn _ZN7QPixmap4fillEPK12QPaintDeviceRK6QPoint(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  bool QPixmap::isDetached();
  fn _ZNK7QPixmap10isDetachedEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QPixmap::isNull();
  fn _ZNK7QPixmap6isNullEv(qthis: *mut c_void) -> int8_t;
  // proto:  QPixmap QPixmap::copy(int x, int y, int width, int height);
  fn _ZNK7QPixmap4copyEiiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) -> *mut c_void;
  // proto: static int QPixmap::defaultDepth();
  fn _ZN7QPixmap12defaultDepthEv() -> c_int;
  // proto:  void QPixmap::detach();
  fn _ZN7QPixmap6detachEv(qthis: *mut c_void) ;
  // proto:  void QPixmap::scroll(int dx, int dy, const QRect & rect, QRegion * exposed);
  fn _ZN7QPixmap6scrollEiiRK5QRectP7QRegion(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: *mut c_void, arg3: *mut c_void) ;
  // proto:  void QPixmap::setMask(const QBitmap & );
  fn _ZN7QPixmap7setMaskERK7QBitmap(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QPixmap::NewQPixmap();
  fn _ZN7QPixmapC1Ev(qthis: *mut c_void) ;
  // proto: static QPixmap QPixmap::grabWidget(QObject * widget, const QRect & rect);
  fn _ZN7QPixmap10grabWidgetEP7QObjectRK5QRect(arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  void QPixmap::NewQPixmap(const QPixmap & );
  fn _ZN7QPixmapC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QPixmap::setDevicePixelRatio(qreal scaleFactor);
  fn _ZN7QPixmap19setDevicePixelRatioEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  void QPixmap::NewQPixmap(const char *const [] xpm);
  fn _ZN7QPixmapC1EPKPKc(qthis: *mut c_void, arg0: *mut *mut c_char) ;
  // proto:  long long QPixmap::cacheKey();
  fn _ZNK7QPixmap8cacheKeyEv(qthis: *mut c_void) -> c_longlong;
  // proto:  QBitmap QPixmap::createHeuristicMask(bool clipTight);
  fn _ZNK7QPixmap19createHeuristicMaskEb(qthis: *mut c_void, arg0: int8_t) -> *mut c_void;
  // proto:  int QPixmap::depth();
  fn _ZNK7QPixmap5depthEv(qthis: *mut c_void) -> c_int;
  // proto:  QImage QPixmap::toImage();
  fn _ZNK7QPixmap7toImageEv(qthis: *mut c_void) -> *mut c_void;
  // proto: static QPixmap QPixmap::grabWidget(QObject * widget, int x, int y, int w, int h);
  fn _ZN7QPixmap10grabWidgetEP7QObjectiiii(arg0: *mut c_void, arg1: c_int, arg2: c_int, arg3: c_int, arg4: c_int) -> *mut c_void;
  // proto:  QPlatformPixmap * QPixmap::handle();
  fn _ZNK7QPixmap6handleEv(qthis: *mut c_void) ;
  // proto:  bool QPixmap::hasAlphaChannel();
  fn _ZNK7QPixmap15hasAlphaChannelEv(qthis: *mut c_void) -> int8_t;
  // proto:  QRect QPixmap::rect();
  fn _ZNK7QPixmap4rectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QBitmap QPixmap::mask();
  fn _ZNK7QPixmap4maskEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QPixmap::width();
  fn _ZNK7QPixmap5widthEv(qthis: *mut c_void) -> c_int;
  // proto:  QPaintEngine * QPixmap::paintEngine();
  fn _ZNK7QPixmap11paintEngineEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPixmap::FreeQPixmap();
  fn _ZN7QPixmapD0Ev(qthis: *mut c_void) ;
  // proto:  int QPixmap::height();
  fn _ZNK7QPixmap6heightEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QPixmap::save(QIODevice * device, const char * format, int quality);
  fn _ZNK7QPixmap4saveEP9QIODevicePKci(qthis: *mut c_void, arg0: *mut c_void, arg1: *const c_char, arg2: c_int) -> int8_t;
  // proto:  QSize QPixmap::size();
  fn _ZNK7QPixmap4sizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QPixmap::hasAlpha();
  fn _ZNK7QPixmap8hasAlphaEv(qthis: *mut c_void) -> int8_t;
}

// body block begin
// class sizeof(QPixmap)=1
pub struct QPixmap {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QPixmap {
  pub fn save<RetType, T: QPixmap_save<RetType>>(&mut self, value: T) -> RetType {
    return value.save(self);
    // return 1;
  }
}

pub trait QPixmap_save<RetType> {
  fn save(self, rsthis: &mut QPixmap) -> RetType;
}

// proto:  bool QPixmap::save(const QString & fileName, const char * format, int quality);
impl<'a> /*trait*/ QPixmap_save<i8> for (&'a  QString, &'a  String, i32) {
  fn save(self, rsthis: &mut QPixmap) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPixmap4saveERK7QStringPKci()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *const c_char;
    let arg2 = self.2  as c_int;
    let mut ret = unsafe {_ZNK7QPixmap4saveERK7QStringPKci(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QPixmap {
  pub fn swap<RetType, T: QPixmap_swap<RetType>>(&mut self, value: T) -> RetType {
    return value.swap(self);
    // return 1;
  }
}

pub trait QPixmap_swap<RetType> {
  fn swap(self, rsthis: &mut QPixmap) -> RetType;
}

// proto:  void QPixmap::swap(QPixmap & other);
impl<'a> /*trait*/ QPixmap_swap<()> for (&'a mut QPixmap) {
  fn swap(self, rsthis: &mut QPixmap) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPixmap4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QPixmap4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPixmap {
  pub fn isQBitmap<RetType, T: QPixmap_isQBitmap<RetType>>(&mut self, value: T) -> RetType {
    return value.isQBitmap(self);
    // return 1;
  }
}

pub trait QPixmap_isQBitmap<RetType> {
  fn isQBitmap(self, rsthis: &mut QPixmap) -> RetType;
}

// proto:  bool QPixmap::isQBitmap();
impl<'a> /*trait*/ QPixmap_isQBitmap<i8> for () {
  fn isQBitmap(self, rsthis: &mut QPixmap) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPixmap9isQBitmapEv()};
    let mut ret = unsafe {_ZNK7QPixmap9isQBitmapEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QPixmap {
  pub fn devicePixelRatio<RetType, T: QPixmap_devicePixelRatio<RetType>>(&mut self, value: T) -> RetType {
    return value.devicePixelRatio(self);
    // return 1;
  }
}

pub trait QPixmap_devicePixelRatio<RetType> {
  fn devicePixelRatio(self, rsthis: &mut QPixmap) -> RetType;
}

// proto:  double QPixmap::devicePixelRatio();
impl<'a> /*trait*/ QPixmap_devicePixelRatio<f64> for () {
  fn devicePixelRatio(self, rsthis: &mut QPixmap) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPixmap16devicePixelRatioEv()};
    let mut ret = unsafe {_ZNK7QPixmap16devicePixelRatioEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QPixmap {
  pub fn NewQPixmap<T: QPixmap_NewQPixmap>(value: T) -> QPixmap {
    let rsthis = value.NewQPixmap();
    return rsthis;
    // return 1;
  }
}

pub trait QPixmap_NewQPixmap {
  fn NewQPixmap(self) -> QPixmap;
}

// proto: void QPixmap::NewQPixmap(const QSize & );
impl<'a> /*trait*/ QPixmap_NewQPixmap for (&'a  QSize) {
  fn NewQPixmap(self) -> QPixmap {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPixmapC1ERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QPixmapC1ERK5QSize(qthis, arg0)};
    let rsthis = QPixmap{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPixmap {
  pub fn fill<RetType, T: QPixmap_fill<RetType>>(&mut self, value: T) -> RetType {
    return value.fill(self);
    // return 1;
  }
}

pub trait QPixmap_fill<RetType> {
  fn fill(self, rsthis: &mut QPixmap) -> RetType;
}

// proto:  void QPixmap::fill(const QPaintDevice * device, int xofs, int yofs);
impl<'a> /*trait*/ QPixmap_fill<()> for (&'a  QPaintDevice, i32, i32) {
  fn fill(self, rsthis: &mut QPixmap) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPixmap4fillEPK12QPaintDeviceii()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
     unsafe {_ZN7QPixmap4fillEPK12QPaintDeviceii(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

// proto: void QPixmap::NewQPixmap(const QSize & s, int type);
impl<'a> /*trait*/ QPixmap_NewQPixmap for (&'a  QSize, i32) {
  fn NewQPixmap(self) -> QPixmap {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPixmapC1ERK5QSizei()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    unsafe {_ZN7QPixmapC1ERK5QSizei(qthis, arg0, arg1)};
    let rsthis = QPixmap{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  void QPixmap::fill(const QColor & fillColor);
impl<'a> /*trait*/ QPixmap_fill<()> for (&'a  QColor) {
  fn fill(self, rsthis: &mut QPixmap) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPixmap4fillERK6QColor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QPixmap4fillERK6QColor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPixmap {
  pub fn devType<RetType, T: QPixmap_devType<RetType>>(&mut self, value: T) -> RetType {
    return value.devType(self);
    // return 1;
  }
}

pub trait QPixmap_devType<RetType> {
  fn devType(self, rsthis: &mut QPixmap) -> RetType;
}

// proto:  int QPixmap::devType();
impl<'a> /*trait*/ QPixmap_devType<i32> for () {
  fn devType(self, rsthis: &mut QPixmap) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPixmap7devTypeEv()};
    let mut ret = unsafe {_ZNK7QPixmap7devTypeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QPixmap {
  pub fn scroll<RetType, T: QPixmap_scroll<RetType>>(&mut self, value: T) -> RetType {
    return value.scroll(self);
    // return 1;
  }
}

pub trait QPixmap_scroll<RetType> {
  fn scroll(self, rsthis: &mut QPixmap) -> RetType;
}

// proto:  void QPixmap::scroll(int dx, int dy, int x, int y, int width, int height, QRegion * exposed);
impl<'a> /*trait*/ QPixmap_scroll<()> for (i32, i32, i32, i32, i32, i32, &'a mut QRegion) {
  fn scroll(self, rsthis: &mut QPixmap) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPixmap6scrollEiiiiiiP7QRegion()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    let arg4 = self.4  as c_int;
    let arg5 = self.5  as c_int;
    let arg6 = self.6.qclsinst  as *mut c_void;
     unsafe {_ZN7QPixmap6scrollEiiiiiiP7QRegion(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4, arg5, arg6)};
    // return 1;
  }
}

impl /*struct*/ QPixmap {
  pub fn copy<RetType, T: QPixmap_copy<RetType>>(&mut self, value: T) -> RetType {
    return value.copy(self);
    // return 1;
  }
}

pub trait QPixmap_copy<RetType> {
  fn copy(self, rsthis: &mut QPixmap) -> RetType;
}

// proto:  QPixmap QPixmap::copy(const QRect & rect);
impl<'a> /*trait*/ QPixmap_copy<QPixmap> for (&'a  QRect) {
  fn copy(self, rsthis: &mut QPixmap) -> QPixmap {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPixmap4copyERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QPixmap4copyERK5QRect(rsthis.qclsinst, arg0)};
    let mut ret1 = QPixmap{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QPixmap::NewQPixmap(int w, int h);
impl<'a> /*trait*/ QPixmap_NewQPixmap for (i32, i32) {
  fn NewQPixmap(self) -> QPixmap {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPixmapC1Eii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN7QPixmapC1Eii(qthis, arg0, arg1)};
    let rsthis = QPixmap{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPixmap {
  pub fn grabWindow<RetType, T: QPixmap_grabWindow<RetType>>(&mut self, value: T) -> RetType {
    return value.grabWindow(self);
    // return 1;
  }
}

pub trait QPixmap_grabWindow<RetType> {
  fn grabWindow(self, rsthis: &mut QPixmap) -> RetType;
}

// proto: static QPixmap QPixmap::grabWindow(WId , int x, int y, int w, int h);
impl<'a> /*trait*/ QPixmap_grabWindow<QPixmap> for (*mut i32, i32, i32, i32, i32) {
  fn grabWindow(self, rsthis: &mut QPixmap) -> QPixmap {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPixmap10grabWindowEiiiii()};
    let arg0 = self.0  as *mut c_uint;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    let arg4 = self.4  as c_int;
    let mut ret = unsafe {_ZN7QPixmap10grabWindowEiiiii(arg0, arg1, arg2, arg3, arg4)};
    let mut ret1 = QPixmap{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QPixmap::fill(const QPaintDevice * device, const QPoint & ofs);
impl<'a> /*trait*/ QPixmap_fill<()> for (&'a  QPaintDevice, &'a  QPoint) {
  fn fill(self, rsthis: &mut QPixmap) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPixmap4fillEPK12QPaintDeviceRK6QPoint()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN7QPixmap4fillEPK12QPaintDeviceRK6QPoint(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QPixmap {
  pub fn isDetached<RetType, T: QPixmap_isDetached<RetType>>(&mut self, value: T) -> RetType {
    return value.isDetached(self);
    // return 1;
  }
}

pub trait QPixmap_isDetached<RetType> {
  fn isDetached(self, rsthis: &mut QPixmap) -> RetType;
}

// proto:  bool QPixmap::isDetached();
impl<'a> /*trait*/ QPixmap_isDetached<i8> for () {
  fn isDetached(self, rsthis: &mut QPixmap) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPixmap10isDetachedEv()};
    let mut ret = unsafe {_ZNK7QPixmap10isDetachedEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QPixmap {
  pub fn isNull<RetType, T: QPixmap_isNull<RetType>>(&mut self, value: T) -> RetType {
    return value.isNull(self);
    // return 1;
  }
}

pub trait QPixmap_isNull<RetType> {
  fn isNull(self, rsthis: &mut QPixmap) -> RetType;
}

// proto:  bool QPixmap::isNull();
impl<'a> /*trait*/ QPixmap_isNull<i8> for () {
  fn isNull(self, rsthis: &mut QPixmap) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPixmap6isNullEv()};
    let mut ret = unsafe {_ZNK7QPixmap6isNullEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  QPixmap QPixmap::copy(int x, int y, int width, int height);
impl<'a> /*trait*/ QPixmap_copy<QPixmap> for (i32, i32, i32, i32) {
  fn copy(self, rsthis: &mut QPixmap) -> QPixmap {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPixmap4copyEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    let mut ret = unsafe {_ZNK7QPixmap4copyEiiii(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    let mut ret1 = QPixmap{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPixmap {
  pub fn defaultDepth<RetType, T: QPixmap_defaultDepth<RetType>>(&mut self, value: T) -> RetType {
    return value.defaultDepth(self);
    // return 1;
  }
}

pub trait QPixmap_defaultDepth<RetType> {
  fn defaultDepth(self, rsthis: &mut QPixmap) -> RetType;
}

// proto: static int QPixmap::defaultDepth();
impl<'a> /*trait*/ QPixmap_defaultDepth<i32> for () {
  fn defaultDepth(self, rsthis: &mut QPixmap) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPixmap12defaultDepthEv()};
    let mut ret = unsafe {_ZN7QPixmap12defaultDepthEv()};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QPixmap {
  pub fn detach<RetType, T: QPixmap_detach<RetType>>(&mut self, value: T) -> RetType {
    return value.detach(self);
    // return 1;
  }
}

pub trait QPixmap_detach<RetType> {
  fn detach(self, rsthis: &mut QPixmap) -> RetType;
}

// proto:  void QPixmap::detach();
impl<'a> /*trait*/ QPixmap_detach<()> for () {
  fn detach(self, rsthis: &mut QPixmap) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPixmap6detachEv()};
     unsafe {_ZN7QPixmap6detachEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  void QPixmap::scroll(int dx, int dy, const QRect & rect, QRegion * exposed);
impl<'a> /*trait*/ QPixmap_scroll<()> for (i32, i32, &'a  QRect, &'a mut QRegion) {
  fn scroll(self, rsthis: &mut QPixmap) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPixmap6scrollEiiRK5QRectP7QRegion()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3.qclsinst  as *mut c_void;
     unsafe {_ZN7QPixmap6scrollEiiRK5QRectP7QRegion(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

impl /*struct*/ QPixmap {
  pub fn setMask<RetType, T: QPixmap_setMask<RetType>>(&mut self, value: T) -> RetType {
    return value.setMask(self);
    // return 1;
  }
}

pub trait QPixmap_setMask<RetType> {
  fn setMask(self, rsthis: &mut QPixmap) -> RetType;
}

// proto:  void QPixmap::setMask(const QBitmap & );
impl<'a> /*trait*/ QPixmap_setMask<()> for (&'a  QBitmap) {
  fn setMask(self, rsthis: &mut QPixmap) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPixmap7setMaskERK7QBitmap()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN7QPixmap7setMaskERK7QBitmap(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto: void QPixmap::NewQPixmap();
impl<'a> /*trait*/ QPixmap_NewQPixmap for () {
  fn NewQPixmap(self) -> QPixmap {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPixmapC1Ev()};
    unsafe {_ZN7QPixmapC1Ev(qthis)};
    let rsthis = QPixmap{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPixmap {
  pub fn grabWidget<RetType, T: QPixmap_grabWidget<RetType>>(&mut self, value: T) -> RetType {
    return value.grabWidget(self);
    // return 1;
  }
}

pub trait QPixmap_grabWidget<RetType> {
  fn grabWidget(self, rsthis: &mut QPixmap) -> RetType;
}

// proto: static QPixmap QPixmap::grabWidget(QObject * widget, const QRect & rect);
impl<'a> /*trait*/ QPixmap_grabWidget<QPixmap> for (&'a mut QObject, &'a  QRect) {
  fn grabWidget(self, rsthis: &mut QPixmap) -> QPixmap {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPixmap10grabWidgetEP7QObjectRK5QRect()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN7QPixmap10grabWidgetEP7QObjectRK5QRect(arg0, arg1)};
    let mut ret1 = QPixmap{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QPixmap::NewQPixmap(const QPixmap & );
impl<'a> /*trait*/ QPixmap_NewQPixmap for (&'a  QPixmap) {
  fn NewQPixmap(self) -> QPixmap {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPixmapC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN7QPixmapC1ERKS_(qthis, arg0)};
    let rsthis = QPixmap{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPixmap {
  pub fn setDevicePixelRatio<RetType, T: QPixmap_setDevicePixelRatio<RetType>>(&mut self, value: T) -> RetType {
    return value.setDevicePixelRatio(self);
    // return 1;
  }
}

pub trait QPixmap_setDevicePixelRatio<RetType> {
  fn setDevicePixelRatio(self, rsthis: &mut QPixmap) -> RetType;
}

// proto:  void QPixmap::setDevicePixelRatio(qreal scaleFactor);
impl<'a> /*trait*/ QPixmap_setDevicePixelRatio<()> for (f64) {
  fn setDevicePixelRatio(self, rsthis: &mut QPixmap) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPixmap19setDevicePixelRatioEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN7QPixmap19setDevicePixelRatioEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto: void QPixmap::NewQPixmap(const char *const [] xpm);
impl<'a> /*trait*/ QPixmap_NewQPixmap for (&'a  Vec<&'a  i8>) {
  fn NewQPixmap(self) -> QPixmap {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPixmapC1EPKPKc()};
    let arg0 = 0  as *mut *mut c_char;
    unsafe {_ZN7QPixmapC1EPKPKc(qthis, arg0)};
    let rsthis = QPixmap{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPixmap {
  pub fn cacheKey<RetType, T: QPixmap_cacheKey<RetType>>(&mut self, value: T) -> RetType {
    return value.cacheKey(self);
    // return 1;
  }
}

pub trait QPixmap_cacheKey<RetType> {
  fn cacheKey(self, rsthis: &mut QPixmap) -> RetType;
}

// proto:  long long QPixmap::cacheKey();
impl<'a> /*trait*/ QPixmap_cacheKey<i64> for () {
  fn cacheKey(self, rsthis: &mut QPixmap) -> i64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPixmap8cacheKeyEv()};
    let mut ret = unsafe {_ZNK7QPixmap8cacheKeyEv(rsthis.qclsinst)};
    return ret as i64;
    // return 1;
  }
}

impl /*struct*/ QPixmap {
  pub fn createHeuristicMask<RetType, T: QPixmap_createHeuristicMask<RetType>>(&mut self, value: T) -> RetType {
    return value.createHeuristicMask(self);
    // return 1;
  }
}

pub trait QPixmap_createHeuristicMask<RetType> {
  fn createHeuristicMask(self, rsthis: &mut QPixmap) -> RetType;
}

// proto:  QBitmap QPixmap::createHeuristicMask(bool clipTight);
impl<'a> /*trait*/ QPixmap_createHeuristicMask<QBitmap> for (i8) {
  fn createHeuristicMask(self, rsthis: &mut QPixmap) -> QBitmap {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPixmap19createHeuristicMaskEb()};
    let arg0 = self  as int8_t;
    let mut ret = unsafe {_ZNK7QPixmap19createHeuristicMaskEb(rsthis.qclsinst, arg0)};
    let mut ret1 = QBitmap{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPixmap {
  pub fn depth<RetType, T: QPixmap_depth<RetType>>(&mut self, value: T) -> RetType {
    return value.depth(self);
    // return 1;
  }
}

pub trait QPixmap_depth<RetType> {
  fn depth(self, rsthis: &mut QPixmap) -> RetType;
}

// proto:  int QPixmap::depth();
impl<'a> /*trait*/ QPixmap_depth<i32> for () {
  fn depth(self, rsthis: &mut QPixmap) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPixmap5depthEv()};
    let mut ret = unsafe {_ZNK7QPixmap5depthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QPixmap {
  pub fn toImage<RetType, T: QPixmap_toImage<RetType>>(&mut self, value: T) -> RetType {
    return value.toImage(self);
    // return 1;
  }
}

pub trait QPixmap_toImage<RetType> {
  fn toImage(self, rsthis: &mut QPixmap) -> RetType;
}

// proto:  QImage QPixmap::toImage();
impl<'a> /*trait*/ QPixmap_toImage<QImage> for () {
  fn toImage(self, rsthis: &mut QPixmap) -> QImage {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPixmap7toImageEv()};
    let mut ret = unsafe {_ZNK7QPixmap7toImageEv(rsthis.qclsinst)};
    let mut ret1 = QImage{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: static QPixmap QPixmap::grabWidget(QObject * widget, int x, int y, int w, int h);
impl<'a> /*trait*/ QPixmap_grabWidget<QPixmap> for (&'a mut QObject, i32, i32, i32, i32) {
  fn grabWidget(self, rsthis: &mut QPixmap) -> QPixmap {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPixmap10grabWidgetEP7QObjectiiii()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
    let arg4 = self.4  as c_int;
    let mut ret = unsafe {_ZN7QPixmap10grabWidgetEP7QObjectiiii(arg0, arg1, arg2, arg3, arg4)};
    let mut ret1 = QPixmap{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPixmap {
  pub fn handle<RetType, T: QPixmap_handle<RetType>>(&mut self, value: T) -> RetType {
    return value.handle(self);
    // return 1;
  }
}

pub trait QPixmap_handle<RetType> {
  fn handle(self, rsthis: &mut QPixmap) -> RetType;
}

// proto:  QPlatformPixmap * QPixmap::handle();
impl<'a> /*trait*/ QPixmap_handle<()> for () {
  fn handle(self, rsthis: &mut QPixmap) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPixmap6handleEv()};
     unsafe {_ZNK7QPixmap6handleEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QPixmap {
  pub fn hasAlphaChannel<RetType, T: QPixmap_hasAlphaChannel<RetType>>(&mut self, value: T) -> RetType {
    return value.hasAlphaChannel(self);
    // return 1;
  }
}

pub trait QPixmap_hasAlphaChannel<RetType> {
  fn hasAlphaChannel(self, rsthis: &mut QPixmap) -> RetType;
}

// proto:  bool QPixmap::hasAlphaChannel();
impl<'a> /*trait*/ QPixmap_hasAlphaChannel<i8> for () {
  fn hasAlphaChannel(self, rsthis: &mut QPixmap) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPixmap15hasAlphaChannelEv()};
    let mut ret = unsafe {_ZNK7QPixmap15hasAlphaChannelEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QPixmap {
  pub fn rect<RetType, T: QPixmap_rect<RetType>>(&mut self, value: T) -> RetType {
    return value.rect(self);
    // return 1;
  }
}

pub trait QPixmap_rect<RetType> {
  fn rect(self, rsthis: &mut QPixmap) -> RetType;
}

// proto:  QRect QPixmap::rect();
impl<'a> /*trait*/ QPixmap_rect<QRect> for () {
  fn rect(self, rsthis: &mut QPixmap) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPixmap4rectEv()};
    let mut ret = unsafe {_ZNK7QPixmap4rectEv(rsthis.qclsinst)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPixmap {
  pub fn mask<RetType, T: QPixmap_mask<RetType>>(&mut self, value: T) -> RetType {
    return value.mask(self);
    // return 1;
  }
}

pub trait QPixmap_mask<RetType> {
  fn mask(self, rsthis: &mut QPixmap) -> RetType;
}

// proto:  QBitmap QPixmap::mask();
impl<'a> /*trait*/ QPixmap_mask<QBitmap> for () {
  fn mask(self, rsthis: &mut QPixmap) -> QBitmap {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPixmap4maskEv()};
    let mut ret = unsafe {_ZNK7QPixmap4maskEv(rsthis.qclsinst)};
    let mut ret1 = QBitmap{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPixmap {
  pub fn width<RetType, T: QPixmap_width<RetType>>(&mut self, value: T) -> RetType {
    return value.width(self);
    // return 1;
  }
}

pub trait QPixmap_width<RetType> {
  fn width(self, rsthis: &mut QPixmap) -> RetType;
}

// proto:  int QPixmap::width();
impl<'a> /*trait*/ QPixmap_width<i32> for () {
  fn width(self, rsthis: &mut QPixmap) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPixmap5widthEv()};
    let mut ret = unsafe {_ZNK7QPixmap5widthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QPixmap {
  pub fn paintEngine<RetType, T: QPixmap_paintEngine<RetType>>(&mut self, value: T) -> RetType {
    return value.paintEngine(self);
    // return 1;
  }
}

pub trait QPixmap_paintEngine<RetType> {
  fn paintEngine(self, rsthis: &mut QPixmap) -> RetType;
}

// proto:  QPaintEngine * QPixmap::paintEngine();
impl<'a> /*trait*/ QPixmap_paintEngine<QPaintEngine> for () {
  fn paintEngine(self, rsthis: &mut QPixmap) -> QPaintEngine {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPixmap11paintEngineEv()};
    let mut ret = unsafe {_ZNK7QPixmap11paintEngineEv(rsthis.qclsinst)};
    let mut ret1 = QPaintEngine{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPixmap {
  pub fn FreeQPixmap<RetType, T: QPixmap_FreeQPixmap<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQPixmap(self);
    // return 1;
  }
}

pub trait QPixmap_FreeQPixmap<RetType> {
  fn FreeQPixmap(self, rsthis: &mut QPixmap) -> RetType;
}

// proto:  void QPixmap::FreeQPixmap();
impl<'a> /*trait*/ QPixmap_FreeQPixmap<()> for () {
  fn FreeQPixmap(self, rsthis: &mut QPixmap) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPixmapD0Ev()};
     unsafe {_ZN7QPixmapD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QPixmap {
  pub fn height<RetType, T: QPixmap_height<RetType>>(&mut self, value: T) -> RetType {
    return value.height(self);
    // return 1;
  }
}

pub trait QPixmap_height<RetType> {
  fn height(self, rsthis: &mut QPixmap) -> RetType;
}

// proto:  int QPixmap::height();
impl<'a> /*trait*/ QPixmap_height<i32> for () {
  fn height(self, rsthis: &mut QPixmap) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPixmap6heightEv()};
    let mut ret = unsafe {_ZNK7QPixmap6heightEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto:  bool QPixmap::save(QIODevice * device, const char * format, int quality);
impl<'a> /*trait*/ QPixmap_save<i8> for (&'a mut QIODevice, &'a  String, i32) {
  fn save(self, rsthis: &mut QPixmap) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPixmap4saveEP9QIODevicePKci()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *const c_char;
    let arg2 = self.2  as c_int;
    let mut ret = unsafe {_ZNK7QPixmap4saveEP9QIODevicePKci(rsthis.qclsinst, arg0, arg1, arg2)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QPixmap {
  pub fn size<RetType, T: QPixmap_size<RetType>>(&mut self, value: T) -> RetType {
    return value.size(self);
    // return 1;
  }
}

pub trait QPixmap_size<RetType> {
  fn size(self, rsthis: &mut QPixmap) -> RetType;
}

// proto:  QSize QPixmap::size();
impl<'a> /*trait*/ QPixmap_size<QSize> for () {
  fn size(self, rsthis: &mut QPixmap) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPixmap4sizeEv()};
    let mut ret = unsafe {_ZNK7QPixmap4sizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPixmap {
  pub fn hasAlpha<RetType, T: QPixmap_hasAlpha<RetType>>(&mut self, value: T) -> RetType {
    return value.hasAlpha(self);
    // return 1;
  }
}

pub trait QPixmap_hasAlpha<RetType> {
  fn hasAlpha(self, rsthis: &mut QPixmap) -> RetType;
}

// proto:  bool QPixmap::hasAlpha();
impl<'a> /*trait*/ QPixmap_hasAlpha<i8> for () {
  fn hasAlpha(self, rsthis: &mut QPixmap) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPixmap8hasAlphaEv()};
    let mut ret = unsafe {_ZNK7QPixmap8hasAlphaEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

