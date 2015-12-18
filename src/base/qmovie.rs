// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qobject::QObject;
use super::qsize::QSize;
use super::qiodevice::QIODevice;
use super::qimage::QImage;
use super::qstring::QString;
use super::qbytearray::QByteArray;
use super::qrect::QRect;
use super::qcolor::QColor;
use super::qpixmap::QPixmap;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QMovie::NewQMovie(QObject * parent);
  fn _ZN6QMovieC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QMovie::speed();
  fn _ZNK6QMovie5speedEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QMovie::jumpToNextFrame();
  fn _ZN6QMovie15jumpToNextFrameEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QMovie::frameChanged(int frameNumber);
  fn _ZN6QMovie12frameChangedEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  int QMovie::frameCount();
  fn _ZNK6QMovie10frameCountEv(qthis: *mut c_void) -> c_int;
  // proto:  void QMovie::setScaledSize(const QSize & size);
  fn _ZN6QMovie13setScaledSizeERK5QSize(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QMovie::setDevice(QIODevice * device);
  fn _ZN6QMovie9setDeviceEP9QIODevice(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QImage QMovie::currentImage();
  fn _ZNK6QMovie12currentImageEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QMovie::jumpToFrame(int frameNumber);
  fn _ZN6QMovie11jumpToFrameEi(qthis: *mut c_void, arg0: c_int) -> int8_t;
  // proto:  void QMovie::started();
  fn _ZN6QMovie7startedEv(qthis: *mut c_void) ;
  // proto:  void QMovie::NewQMovie(const QString & fileName, const QByteArray & format, QObject * parent);
  fn _ZN6QMovieC1ERK7QStringRK10QByteArrayP7QObject(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) ;
  // proto:  void QMovie::finished();
  fn _ZN6QMovie8finishedEv(qthis: *mut c_void) ;
  // proto:  const QMetaObject * QMovie::metaObject();
  fn _ZNK6QMovie10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QMovie::FreeQMovie();
  fn _ZN6QMovieD0Ev(qthis: *mut c_void) ;
  // proto:  void QMovie::start();
  fn _ZN6QMovie5startEv(qthis: *mut c_void) ;
  // proto:  int QMovie::loopCount();
  fn _ZNK6QMovie9loopCountEv(qthis: *mut c_void) -> c_int;
  // proto:  void QMovie::NewQMovie(QIODevice * device, const QByteArray & format, QObject * parent);
  fn _ZN6QMovieC1EP9QIODeviceRK10QByteArrayP7QObject(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) ;
  // proto:  void QMovie::setFormat(const QByteArray & format);
  fn _ZN6QMovie9setFormatERK10QByteArray(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QMovie::resized(const QSize & size);
  fn _ZN6QMovie7resizedERK5QSize(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto: static QList<QByteArray> QMovie::supportedFormats();
  fn _ZN6QMovie16supportedFormatsEv() ;
  // proto:  QRect QMovie::frameRect();
  fn _ZNK6QMovie9frameRectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QMovie::setPaused(bool paused);
  fn _ZN6QMovie9setPausedEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  QSize QMovie::scaledSize();
  fn _ZN6QMovie10scaledSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QIODevice * QMovie::device();
  fn _ZNK6QMovie6deviceEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QMovie::setBackgroundColor(const QColor & color);
  fn _ZN6QMovie18setBackgroundColorERK6QColor(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QMovie::isValid();
  fn _ZNK6QMovie7isValidEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QMovie::setSpeed(int percentSpeed);
  fn _ZN6QMovie8setSpeedEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QMovie::NewQMovie(const QMovie & );
  fn _ZN6QMovieC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QMovie::stop();
  fn _ZN6QMovie4stopEv(qthis: *mut c_void) ;
  // proto:  int QMovie::currentFrameNumber();
  fn _ZNK6QMovie18currentFrameNumberEv(qthis: *mut c_void) -> c_int;
  // proto:  int QMovie::nextFrameDelay();
  fn _ZNK6QMovie14nextFrameDelayEv(qthis: *mut c_void) -> c_int;
  // proto:  QPixmap QMovie::currentPixmap();
  fn _ZNK6QMovie13currentPixmapEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QByteArray QMovie::format();
  fn _ZNK6QMovie6formatEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString QMovie::fileName();
  fn _ZNK6QMovie8fileNameEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QMovie::updated(const QRect & rect);
  fn _ZN6QMovie7updatedERK5QRect(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QColor QMovie::backgroundColor();
  fn _ZNK6QMovie15backgroundColorEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QMovie::setFileName(const QString & fileName);
  fn _ZN6QMovie11setFileNameERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
}

// body block begin
// class sizeof(QMovie)=1
pub struct QMovie {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QMovie {
  pub fn NewQMovie<T: QMovie_NewQMovie>(value: T) -> QMovie {
    let rsthis = value.NewQMovie();
    return rsthis;
    // return 1;
  }
}

pub trait QMovie_NewQMovie {
  fn NewQMovie(self) -> QMovie;
}

// proto: void QMovie::NewQMovie(QObject * parent);
impl<'a> /*trait*/ QMovie_NewQMovie for (&'a mut QObject) {
  fn NewQMovie(self) -> QMovie {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMovieC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN6QMovieC1EP7QObject(qthis, arg0)};
    let rsthis = QMovie{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn speed<RetType, T: QMovie_speed<RetType>>(&mut self, value: T) -> RetType {
    return value.speed(self);
    // return 1;
  }
}

pub trait QMovie_speed<RetType> {
  fn speed(self, rsthis: &mut QMovie) -> RetType;
}

// proto:  int QMovie::speed();
impl<'a> /*trait*/ QMovie_speed<i32> for () {
  fn speed(self, rsthis: &mut QMovie) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QMovie5speedEv()};
    let mut ret = unsafe {_ZNK6QMovie5speedEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn jumpToNextFrame<RetType, T: QMovie_jumpToNextFrame<RetType>>(&mut self, value: T) -> RetType {
    return value.jumpToNextFrame(self);
    // return 1;
  }
}

pub trait QMovie_jumpToNextFrame<RetType> {
  fn jumpToNextFrame(self, rsthis: &mut QMovie) -> RetType;
}

// proto:  bool QMovie::jumpToNextFrame();
impl<'a> /*trait*/ QMovie_jumpToNextFrame<i8> for () {
  fn jumpToNextFrame(self, rsthis: &mut QMovie) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMovie15jumpToNextFrameEv()};
    let mut ret = unsafe {_ZN6QMovie15jumpToNextFrameEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn frameChanged<RetType, T: QMovie_frameChanged<RetType>>(&mut self, value: T) -> RetType {
    return value.frameChanged(self);
    // return 1;
  }
}

pub trait QMovie_frameChanged<RetType> {
  fn frameChanged(self, rsthis: &mut QMovie) -> RetType;
}

// proto:  void QMovie::frameChanged(int frameNumber);
impl<'a> /*trait*/ QMovie_frameChanged<()> for (i32) {
  fn frameChanged(self, rsthis: &mut QMovie) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMovie12frameChangedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN6QMovie12frameChangedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn frameCount<RetType, T: QMovie_frameCount<RetType>>(&mut self, value: T) -> RetType {
    return value.frameCount(self);
    // return 1;
  }
}

pub trait QMovie_frameCount<RetType> {
  fn frameCount(self, rsthis: &mut QMovie) -> RetType;
}

// proto:  int QMovie::frameCount();
impl<'a> /*trait*/ QMovie_frameCount<i32> for () {
  fn frameCount(self, rsthis: &mut QMovie) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QMovie10frameCountEv()};
    let mut ret = unsafe {_ZNK6QMovie10frameCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn setScaledSize<RetType, T: QMovie_setScaledSize<RetType>>(&mut self, value: T) -> RetType {
    return value.setScaledSize(self);
    // return 1;
  }
}

pub trait QMovie_setScaledSize<RetType> {
  fn setScaledSize(self, rsthis: &mut QMovie) -> RetType;
}

// proto:  void QMovie::setScaledSize(const QSize & size);
impl<'a> /*trait*/ QMovie_setScaledSize<()> for (&'a  QSize) {
  fn setScaledSize(self, rsthis: &mut QMovie) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMovie13setScaledSizeERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QMovie13setScaledSizeERK5QSize(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn setDevice<RetType, T: QMovie_setDevice<RetType>>(&mut self, value: T) -> RetType {
    return value.setDevice(self);
    // return 1;
  }
}

pub trait QMovie_setDevice<RetType> {
  fn setDevice(self, rsthis: &mut QMovie) -> RetType;
}

// proto:  void QMovie::setDevice(QIODevice * device);
impl<'a> /*trait*/ QMovie_setDevice<()> for (&'a mut QIODevice) {
  fn setDevice(self, rsthis: &mut QMovie) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMovie9setDeviceEP9QIODevice()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QMovie9setDeviceEP9QIODevice(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn currentImage<RetType, T: QMovie_currentImage<RetType>>(&mut self, value: T) -> RetType {
    return value.currentImage(self);
    // return 1;
  }
}

pub trait QMovie_currentImage<RetType> {
  fn currentImage(self, rsthis: &mut QMovie) -> RetType;
}

// proto:  QImage QMovie::currentImage();
impl<'a> /*trait*/ QMovie_currentImage<QImage> for () {
  fn currentImage(self, rsthis: &mut QMovie) -> QImage {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QMovie12currentImageEv()};
    let mut ret = unsafe {_ZNK6QMovie12currentImageEv(rsthis.qclsinst)};
    let mut ret1 = QImage{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn jumpToFrame<RetType, T: QMovie_jumpToFrame<RetType>>(&mut self, value: T) -> RetType {
    return value.jumpToFrame(self);
    // return 1;
  }
}

pub trait QMovie_jumpToFrame<RetType> {
  fn jumpToFrame(self, rsthis: &mut QMovie) -> RetType;
}

// proto:  bool QMovie::jumpToFrame(int frameNumber);
impl<'a> /*trait*/ QMovie_jumpToFrame<i8> for (i32) {
  fn jumpToFrame(self, rsthis: &mut QMovie) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMovie11jumpToFrameEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN6QMovie11jumpToFrameEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn started<RetType, T: QMovie_started<RetType>>(&mut self, value: T) -> RetType {
    return value.started(self);
    // return 1;
  }
}

pub trait QMovie_started<RetType> {
  fn started(self, rsthis: &mut QMovie) -> RetType;
}

// proto:  void QMovie::started();
impl<'a> /*trait*/ QMovie_started<()> for () {
  fn started(self, rsthis: &mut QMovie) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMovie7startedEv()};
     unsafe {_ZN6QMovie7startedEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto: void QMovie::NewQMovie(const QString & fileName, const QByteArray & format, QObject * parent);
impl<'a> /*trait*/ QMovie_NewQMovie for (&'a  QString, &'a  QByteArray, &'a mut QObject) {
  fn NewQMovie(self) -> QMovie {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMovieC1ERK7QStringRK10QByteArrayP7QObject()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZN6QMovieC1ERK7QStringRK10QByteArrayP7QObject(qthis, arg0, arg1, arg2)};
    let rsthis = QMovie{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn finished<RetType, T: QMovie_finished<RetType>>(&mut self, value: T) -> RetType {
    return value.finished(self);
    // return 1;
  }
}

pub trait QMovie_finished<RetType> {
  fn finished(self, rsthis: &mut QMovie) -> RetType;
}

// proto:  void QMovie::finished();
impl<'a> /*trait*/ QMovie_finished<()> for () {
  fn finished(self, rsthis: &mut QMovie) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMovie8finishedEv()};
     unsafe {_ZN6QMovie8finishedEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn metaObject<RetType, T: QMovie_metaObject<RetType>>(&mut self, value: T) -> RetType {
    return value.metaObject(self);
    // return 1;
  }
}

pub trait QMovie_metaObject<RetType> {
  fn metaObject(self, rsthis: &mut QMovie) -> RetType;
}

// proto:  const QMetaObject * QMovie::metaObject();
impl<'a> /*trait*/ QMovie_metaObject<()> for () {
  fn metaObject(self, rsthis: &mut QMovie) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QMovie10metaObjectEv()};
     unsafe {_ZNK6QMovie10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn FreeQMovie<RetType, T: QMovie_FreeQMovie<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQMovie(self);
    // return 1;
  }
}

pub trait QMovie_FreeQMovie<RetType> {
  fn FreeQMovie(self, rsthis: &mut QMovie) -> RetType;
}

// proto:  void QMovie::FreeQMovie();
impl<'a> /*trait*/ QMovie_FreeQMovie<()> for () {
  fn FreeQMovie(self, rsthis: &mut QMovie) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMovieD0Ev()};
     unsafe {_ZN6QMovieD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn start<RetType, T: QMovie_start<RetType>>(&mut self, value: T) -> RetType {
    return value.start(self);
    // return 1;
  }
}

pub trait QMovie_start<RetType> {
  fn start(self, rsthis: &mut QMovie) -> RetType;
}

// proto:  void QMovie::start();
impl<'a> /*trait*/ QMovie_start<()> for () {
  fn start(self, rsthis: &mut QMovie) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMovie5startEv()};
     unsafe {_ZN6QMovie5startEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn loopCount<RetType, T: QMovie_loopCount<RetType>>(&mut self, value: T) -> RetType {
    return value.loopCount(self);
    // return 1;
  }
}

pub trait QMovie_loopCount<RetType> {
  fn loopCount(self, rsthis: &mut QMovie) -> RetType;
}

// proto:  int QMovie::loopCount();
impl<'a> /*trait*/ QMovie_loopCount<i32> for () {
  fn loopCount(self, rsthis: &mut QMovie) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QMovie9loopCountEv()};
    let mut ret = unsafe {_ZNK6QMovie9loopCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto: void QMovie::NewQMovie(QIODevice * device, const QByteArray & format, QObject * parent);
impl<'a> /*trait*/ QMovie_NewQMovie for (&'a mut QIODevice, &'a  QByteArray, &'a mut QObject) {
  fn NewQMovie(self) -> QMovie {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMovieC1EP9QIODeviceRK10QByteArrayP7QObject()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    unsafe {_ZN6QMovieC1EP9QIODeviceRK10QByteArrayP7QObject(qthis, arg0, arg1, arg2)};
    let rsthis = QMovie{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn setFormat<RetType, T: QMovie_setFormat<RetType>>(&mut self, value: T) -> RetType {
    return value.setFormat(self);
    // return 1;
  }
}

pub trait QMovie_setFormat<RetType> {
  fn setFormat(self, rsthis: &mut QMovie) -> RetType;
}

// proto:  void QMovie::setFormat(const QByteArray & format);
impl<'a> /*trait*/ QMovie_setFormat<()> for (&'a  QByteArray) {
  fn setFormat(self, rsthis: &mut QMovie) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMovie9setFormatERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QMovie9setFormatERK10QByteArray(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn resized<RetType, T: QMovie_resized<RetType>>(&mut self, value: T) -> RetType {
    return value.resized(self);
    // return 1;
  }
}

pub trait QMovie_resized<RetType> {
  fn resized(self, rsthis: &mut QMovie) -> RetType;
}

// proto:  void QMovie::resized(const QSize & size);
impl<'a> /*trait*/ QMovie_resized<()> for (&'a  QSize) {
  fn resized(self, rsthis: &mut QMovie) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMovie7resizedERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QMovie7resizedERK5QSize(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn supportedFormats<RetType, T: QMovie_supportedFormats<RetType>>(&mut self, value: T) -> RetType {
    return value.supportedFormats(self);
    // return 1;
  }
}

pub trait QMovie_supportedFormats<RetType> {
  fn supportedFormats(self, rsthis: &mut QMovie) -> RetType;
}

// proto: static QList<QByteArray> QMovie::supportedFormats();
impl<'a> /*trait*/ QMovie_supportedFormats<()> for () {
  fn supportedFormats(self, rsthis: &mut QMovie) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMovie16supportedFormatsEv()};
     unsafe {_ZN6QMovie16supportedFormatsEv()};
    // return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn frameRect<RetType, T: QMovie_frameRect<RetType>>(&mut self, value: T) -> RetType {
    return value.frameRect(self);
    // return 1;
  }
}

pub trait QMovie_frameRect<RetType> {
  fn frameRect(self, rsthis: &mut QMovie) -> RetType;
}

// proto:  QRect QMovie::frameRect();
impl<'a> /*trait*/ QMovie_frameRect<QRect> for () {
  fn frameRect(self, rsthis: &mut QMovie) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QMovie9frameRectEv()};
    let mut ret = unsafe {_ZNK6QMovie9frameRectEv(rsthis.qclsinst)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn setPaused<RetType, T: QMovie_setPaused<RetType>>(&mut self, value: T) -> RetType {
    return value.setPaused(self);
    // return 1;
  }
}

pub trait QMovie_setPaused<RetType> {
  fn setPaused(self, rsthis: &mut QMovie) -> RetType;
}

// proto:  void QMovie::setPaused(bool paused);
impl<'a> /*trait*/ QMovie_setPaused<()> for (i8) {
  fn setPaused(self, rsthis: &mut QMovie) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMovie9setPausedEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN6QMovie9setPausedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn scaledSize<RetType, T: QMovie_scaledSize<RetType>>(&mut self, value: T) -> RetType {
    return value.scaledSize(self);
    // return 1;
  }
}

pub trait QMovie_scaledSize<RetType> {
  fn scaledSize(self, rsthis: &mut QMovie) -> RetType;
}

// proto:  QSize QMovie::scaledSize();
impl<'a> /*trait*/ QMovie_scaledSize<QSize> for () {
  fn scaledSize(self, rsthis: &mut QMovie) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMovie10scaledSizeEv()};
    let mut ret = unsafe {_ZN6QMovie10scaledSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn device<RetType, T: QMovie_device<RetType>>(&mut self, value: T) -> RetType {
    return value.device(self);
    // return 1;
  }
}

pub trait QMovie_device<RetType> {
  fn device(self, rsthis: &mut QMovie) -> RetType;
}

// proto:  QIODevice * QMovie::device();
impl<'a> /*trait*/ QMovie_device<QIODevice> for () {
  fn device(self, rsthis: &mut QMovie) -> QIODevice {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QMovie6deviceEv()};
    let mut ret = unsafe {_ZNK6QMovie6deviceEv(rsthis.qclsinst)};
    let mut ret1 = QIODevice{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn setBackgroundColor<RetType, T: QMovie_setBackgroundColor<RetType>>(&mut self, value: T) -> RetType {
    return value.setBackgroundColor(self);
    // return 1;
  }
}

pub trait QMovie_setBackgroundColor<RetType> {
  fn setBackgroundColor(self, rsthis: &mut QMovie) -> RetType;
}

// proto:  void QMovie::setBackgroundColor(const QColor & color);
impl<'a> /*trait*/ QMovie_setBackgroundColor<()> for (&'a  QColor) {
  fn setBackgroundColor(self, rsthis: &mut QMovie) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMovie18setBackgroundColorERK6QColor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QMovie18setBackgroundColorERK6QColor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn isValid<RetType, T: QMovie_isValid<RetType>>(&mut self, value: T) -> RetType {
    return value.isValid(self);
    // return 1;
  }
}

pub trait QMovie_isValid<RetType> {
  fn isValid(self, rsthis: &mut QMovie) -> RetType;
}

// proto:  bool QMovie::isValid();
impl<'a> /*trait*/ QMovie_isValid<i8> for () {
  fn isValid(self, rsthis: &mut QMovie) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QMovie7isValidEv()};
    let mut ret = unsafe {_ZNK6QMovie7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn setSpeed<RetType, T: QMovie_setSpeed<RetType>>(&mut self, value: T) -> RetType {
    return value.setSpeed(self);
    // return 1;
  }
}

pub trait QMovie_setSpeed<RetType> {
  fn setSpeed(self, rsthis: &mut QMovie) -> RetType;
}

// proto:  void QMovie::setSpeed(int percentSpeed);
impl<'a> /*trait*/ QMovie_setSpeed<()> for (i32) {
  fn setSpeed(self, rsthis: &mut QMovie) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMovie8setSpeedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN6QMovie8setSpeedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto: void QMovie::NewQMovie(const QMovie & );
impl<'a> /*trait*/ QMovie_NewQMovie for (&'a  QMovie) {
  fn NewQMovie(self) -> QMovie {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMovieC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN6QMovieC1ERKS_(qthis, arg0)};
    let rsthis = QMovie{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn stop<RetType, T: QMovie_stop<RetType>>(&mut self, value: T) -> RetType {
    return value.stop(self);
    // return 1;
  }
}

pub trait QMovie_stop<RetType> {
  fn stop(self, rsthis: &mut QMovie) -> RetType;
}

// proto:  void QMovie::stop();
impl<'a> /*trait*/ QMovie_stop<()> for () {
  fn stop(self, rsthis: &mut QMovie) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMovie4stopEv()};
     unsafe {_ZN6QMovie4stopEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn currentFrameNumber<RetType, T: QMovie_currentFrameNumber<RetType>>(&mut self, value: T) -> RetType {
    return value.currentFrameNumber(self);
    // return 1;
  }
}

pub trait QMovie_currentFrameNumber<RetType> {
  fn currentFrameNumber(self, rsthis: &mut QMovie) -> RetType;
}

// proto:  int QMovie::currentFrameNumber();
impl<'a> /*trait*/ QMovie_currentFrameNumber<i32> for () {
  fn currentFrameNumber(self, rsthis: &mut QMovie) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QMovie18currentFrameNumberEv()};
    let mut ret = unsafe {_ZNK6QMovie18currentFrameNumberEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn nextFrameDelay<RetType, T: QMovie_nextFrameDelay<RetType>>(&mut self, value: T) -> RetType {
    return value.nextFrameDelay(self);
    // return 1;
  }
}

pub trait QMovie_nextFrameDelay<RetType> {
  fn nextFrameDelay(self, rsthis: &mut QMovie) -> RetType;
}

// proto:  int QMovie::nextFrameDelay();
impl<'a> /*trait*/ QMovie_nextFrameDelay<i32> for () {
  fn nextFrameDelay(self, rsthis: &mut QMovie) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QMovie14nextFrameDelayEv()};
    let mut ret = unsafe {_ZNK6QMovie14nextFrameDelayEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn currentPixmap<RetType, T: QMovie_currentPixmap<RetType>>(&mut self, value: T) -> RetType {
    return value.currentPixmap(self);
    // return 1;
  }
}

pub trait QMovie_currentPixmap<RetType> {
  fn currentPixmap(self, rsthis: &mut QMovie) -> RetType;
}

// proto:  QPixmap QMovie::currentPixmap();
impl<'a> /*trait*/ QMovie_currentPixmap<QPixmap> for () {
  fn currentPixmap(self, rsthis: &mut QMovie) -> QPixmap {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QMovie13currentPixmapEv()};
    let mut ret = unsafe {_ZNK6QMovie13currentPixmapEv(rsthis.qclsinst)};
    let mut ret1 = QPixmap{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn format<RetType, T: QMovie_format<RetType>>(&mut self, value: T) -> RetType {
    return value.format(self);
    // return 1;
  }
}

pub trait QMovie_format<RetType> {
  fn format(self, rsthis: &mut QMovie) -> RetType;
}

// proto:  QByteArray QMovie::format();
impl<'a> /*trait*/ QMovie_format<QByteArray> for () {
  fn format(self, rsthis: &mut QMovie) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QMovie6formatEv()};
    let mut ret = unsafe {_ZNK6QMovie6formatEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn fileName<RetType, T: QMovie_fileName<RetType>>(&mut self, value: T) -> RetType {
    return value.fileName(self);
    // return 1;
  }
}

pub trait QMovie_fileName<RetType> {
  fn fileName(self, rsthis: &mut QMovie) -> RetType;
}

// proto:  QString QMovie::fileName();
impl<'a> /*trait*/ QMovie_fileName<QString> for () {
  fn fileName(self, rsthis: &mut QMovie) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QMovie8fileNameEv()};
    let mut ret = unsafe {_ZNK6QMovie8fileNameEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn updated<RetType, T: QMovie_updated<RetType>>(&mut self, value: T) -> RetType {
    return value.updated(self);
    // return 1;
  }
}

pub trait QMovie_updated<RetType> {
  fn updated(self, rsthis: &mut QMovie) -> RetType;
}

// proto:  void QMovie::updated(const QRect & rect);
impl<'a> /*trait*/ QMovie_updated<()> for (&'a  QRect) {
  fn updated(self, rsthis: &mut QMovie) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMovie7updatedERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QMovie7updatedERK5QRect(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn backgroundColor<RetType, T: QMovie_backgroundColor<RetType>>(&mut self, value: T) -> RetType {
    return value.backgroundColor(self);
    // return 1;
  }
}

pub trait QMovie_backgroundColor<RetType> {
  fn backgroundColor(self, rsthis: &mut QMovie) -> RetType;
}

// proto:  QColor QMovie::backgroundColor();
impl<'a> /*trait*/ QMovie_backgroundColor<QColor> for () {
  fn backgroundColor(self, rsthis: &mut QMovie) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QMovie15backgroundColorEv()};
    let mut ret = unsafe {_ZNK6QMovie15backgroundColorEv(rsthis.qclsinst)};
    let mut ret1 = QColor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QMovie {
  pub fn setFileName<RetType, T: QMovie_setFileName<RetType>>(&mut self, value: T) -> RetType {
    return value.setFileName(self);
    // return 1;
  }
}

pub trait QMovie_setFileName<RetType> {
  fn setFileName(self, rsthis: &mut QMovie) -> RetType;
}

// proto:  void QMovie::setFileName(const QString & fileName);
impl<'a> /*trait*/ QMovie_setFileName<()> for (&'a  QString) {
  fn setFileName(self, rsthis: &mut QMovie) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMovie11setFileNameERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QMovie11setFileNameERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

