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
use super::qrect::QRect;
use super::qiodevice::QIODevice;
use super::qbytearray::QByteArray;
use super::qcolor::QColor;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  QString QImageReader::errorString();
  fn _ZNK12QImageReader11errorStringEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QImageReader::canRead();
  fn _ZNK12QImageReader7canReadEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QImageReader::FreeQImageReader();
  fn _ZN12QImageReaderD0Ev(qthis: *mut c_void) ;
  // proto:  void QImageReader::setScaledSize(const QSize & size);
  fn _ZN12QImageReader13setScaledSizeERK5QSize(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QImageReader::setScaledClipRect(const QRect & rect);
  fn _ZN12QImageReader17setScaledClipRectERK5QRect(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QImageReader::imageCount();
  fn _ZNK12QImageReader10imageCountEv(qthis: *mut c_void) -> c_int;
  // proto:  QStringList QImageReader::textKeys();
  fn _ZNK12QImageReader8textKeysEv(qthis: *mut c_void) ;
  // proto:  bool QImageReader::decideFormatFromContent();
  fn _ZNK12QImageReader23decideFormatFromContentEv(qthis: *mut c_void) -> int8_t;
  // proto:  QIODevice * QImageReader::device();
  fn _ZNK12QImageReader6deviceEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QImageReader::autoTransform();
  fn _ZNK12QImageReader13autoTransformEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QImageReader::jumpToNextImage();
  fn _ZN12QImageReader15jumpToNextImageEv(qthis: *mut c_void) -> int8_t;
  // proto: static QByteArray QImageReader::imageFormat(const QString & fileName);
  fn _ZN12QImageReader11imageFormatERK7QString(arg0: *mut c_void) -> *mut c_void;
  // proto:  QList<QByteArray> QImageReader::supportedSubTypes();
  fn _ZNK12QImageReader17supportedSubTypesEv(qthis: *mut c_void) ;
  // proto:  QSize QImageReader::size();
  fn _ZNK12QImageReader4sizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QColor QImageReader::backgroundColor();
  fn _ZNK12QImageReader15backgroundColorEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QByteArray QImageReader::subType();
  fn _ZNK12QImageReader7subTypeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QImageReader::currentImageNumber();
  fn _ZNK12QImageReader18currentImageNumberEv(qthis: *mut c_void) -> c_int;
  // proto: static QList<QByteArray> QImageReader::supportedImageFormats();
  fn _ZN12QImageReader21supportedImageFormatsEv() ;
  // proto:  int QImageReader::loopCount();
  fn _ZNK12QImageReader9loopCountEv(qthis: *mut c_void) -> c_int;
  // proto:  void QImageReader::setDecideFormatFromContent(bool ignored);
  fn _ZN12QImageReader26setDecideFormatFromContentEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  QRect QImageReader::scaledClipRect();
  fn _ZNK12QImageReader14scaledClipRectEv(qthis: *mut c_void) -> *mut c_void;
  // proto: static QList<QByteArray> QImageReader::supportedMimeTypes();
  fn _ZN12QImageReader18supportedMimeTypesEv() ;
  // proto:  QString QImageReader::text(const QString & key);
  fn _ZNK12QImageReader4textERK7QString(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  int QImageReader::nextImageDelay();
  fn _ZNK12QImageReader14nextImageDelayEv(qthis: *mut c_void) -> c_int;
  // proto:  bool QImageReader::supportsAnimation();
  fn _ZNK12QImageReader17supportsAnimationEv(qthis: *mut c_void) -> int8_t;
  // proto:  bool QImageReader::jumpToImage(int imageNumber);
  fn _ZN12QImageReader11jumpToImageEi(qthis: *mut c_void, arg0: c_int) -> int8_t;
  // proto:  void QImageReader::setFileName(const QString & fileName);
  fn _ZN12QImageReader11setFileNameERK7QString(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QImageReader::NewQImageReader(const QImageReader & );
  fn _ZN12QImageReaderC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QSize QImageReader::scaledSize();
  fn _ZNK12QImageReader10scaledSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QImageReader::setAutoTransform(bool enabled);
  fn _ZN12QImageReader16setAutoTransformEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QImageReader::setClipRect(const QRect & rect);
  fn _ZN12QImageReader11setClipRectERK5QRect(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  bool QImageReader::autoDetectImageFormat();
  fn _ZNK12QImageReader21autoDetectImageFormatEv(qthis: *mut c_void) -> int8_t;
  // proto:  QRect QImageReader::currentImageRect();
  fn _ZNK12QImageReader16currentImageRectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QImageReader::NewQImageReader(const QString & fileName, const QByteArray & format);
  fn _ZN12QImageReaderC1ERK7QStringRK10QByteArray(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto: static QByteArray QImageReader::imageFormat(QIODevice * device);
  fn _ZN12QImageReader11imageFormatEP9QIODevice(arg0: *mut c_void) -> *mut c_void;
  // proto:  int QImageReader::quality();
  fn _ZNK12QImageReader7qualityEv(qthis: *mut c_void) -> c_int;
  // proto:  void QImageReader::setDevice(QIODevice * device);
  fn _ZN12QImageReader9setDeviceEP9QIODevice(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QImageReader::setBackgroundColor(const QColor & color);
  fn _ZN12QImageReader18setBackgroundColorERK6QColor(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QImageReader::setQuality(int quality);
  fn _ZN12QImageReader10setQualityEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QImageReader::NewQImageReader(QIODevice * device, const QByteArray & format);
  fn _ZN12QImageReaderC1EP9QIODeviceRK10QByteArray(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QImageReader::setAutoDetectImageFormat(bool enabled);
  fn _ZN12QImageReader24setAutoDetectImageFormatEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QImageReader::NewQImageReader();
  fn _ZN12QImageReaderC1Ev(qthis: *mut c_void) ;
  // proto:  void QImageReader::setFormat(const QByteArray & format);
  fn _ZN12QImageReader9setFormatERK10QByteArray(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QString QImageReader::fileName();
  fn _ZNK12QImageReader8fileNameEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QRect QImageReader::clipRect();
  fn _ZNK12QImageReader8clipRectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QByteArray QImageReader::format();
  fn _ZNK12QImageReader6formatEv(qthis: *mut c_void) -> *mut c_void;
}

// body block begin
// class sizeof(QImageReader)=8
pub struct QImageReader {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QImageReader {
  pub fn errorString<RetType, T: QImageReader_errorString<RetType>>(&mut self, value: T) -> RetType {
    return value.errorString(self);
    // return 1;
  }
}

pub trait QImageReader_errorString<RetType> {
  fn errorString(self, rsthis: &mut QImageReader) -> RetType;
}

// proto:  QString QImageReader::errorString();
impl<'a> /*trait*/ QImageReader_errorString<QString> for () {
  fn errorString(self, rsthis: &mut QImageReader) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader11errorStringEv()};
    let mut ret = unsafe {_ZNK12QImageReader11errorStringEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn canRead<RetType, T: QImageReader_canRead<RetType>>(&mut self, value: T) -> RetType {
    return value.canRead(self);
    // return 1;
  }
}

pub trait QImageReader_canRead<RetType> {
  fn canRead(self, rsthis: &mut QImageReader) -> RetType;
}

// proto:  bool QImageReader::canRead();
impl<'a> /*trait*/ QImageReader_canRead<i8> for () {
  fn canRead(self, rsthis: &mut QImageReader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader7canReadEv()};
    let mut ret = unsafe {_ZNK12QImageReader7canReadEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn FreeQImageReader<RetType, T: QImageReader_FreeQImageReader<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQImageReader(self);
    // return 1;
  }
}

pub trait QImageReader_FreeQImageReader<RetType> {
  fn FreeQImageReader(self, rsthis: &mut QImageReader) -> RetType;
}

// proto:  void QImageReader::FreeQImageReader();
impl<'a> /*trait*/ QImageReader_FreeQImageReader<()> for () {
  fn FreeQImageReader(self, rsthis: &mut QImageReader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReaderD0Ev()};
     unsafe {_ZN12QImageReaderD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn setScaledSize<RetType, T: QImageReader_setScaledSize<RetType>>(&mut self, value: T) -> RetType {
    return value.setScaledSize(self);
    // return 1;
  }
}

pub trait QImageReader_setScaledSize<RetType> {
  fn setScaledSize(self, rsthis: &mut QImageReader) -> RetType;
}

// proto:  void QImageReader::setScaledSize(const QSize & size);
impl<'a> /*trait*/ QImageReader_setScaledSize<()> for (&'a  QSize) {
  fn setScaledSize(self, rsthis: &mut QImageReader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReader13setScaledSizeERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QImageReader13setScaledSizeERK5QSize(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn setScaledClipRect<RetType, T: QImageReader_setScaledClipRect<RetType>>(&mut self, value: T) -> RetType {
    return value.setScaledClipRect(self);
    // return 1;
  }
}

pub trait QImageReader_setScaledClipRect<RetType> {
  fn setScaledClipRect(self, rsthis: &mut QImageReader) -> RetType;
}

// proto:  void QImageReader::setScaledClipRect(const QRect & rect);
impl<'a> /*trait*/ QImageReader_setScaledClipRect<()> for (&'a  QRect) {
  fn setScaledClipRect(self, rsthis: &mut QImageReader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReader17setScaledClipRectERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QImageReader17setScaledClipRectERK5QRect(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn imageCount<RetType, T: QImageReader_imageCount<RetType>>(&mut self, value: T) -> RetType {
    return value.imageCount(self);
    // return 1;
  }
}

pub trait QImageReader_imageCount<RetType> {
  fn imageCount(self, rsthis: &mut QImageReader) -> RetType;
}

// proto:  int QImageReader::imageCount();
impl<'a> /*trait*/ QImageReader_imageCount<i32> for () {
  fn imageCount(self, rsthis: &mut QImageReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader10imageCountEv()};
    let mut ret = unsafe {_ZNK12QImageReader10imageCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn textKeys<RetType, T: QImageReader_textKeys<RetType>>(&mut self, value: T) -> RetType {
    return value.textKeys(self);
    // return 1;
  }
}

pub trait QImageReader_textKeys<RetType> {
  fn textKeys(self, rsthis: &mut QImageReader) -> RetType;
}

// proto:  QStringList QImageReader::textKeys();
impl<'a> /*trait*/ QImageReader_textKeys<()> for () {
  fn textKeys(self, rsthis: &mut QImageReader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader8textKeysEv()};
     unsafe {_ZNK12QImageReader8textKeysEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn decideFormatFromContent<RetType, T: QImageReader_decideFormatFromContent<RetType>>(&mut self, value: T) -> RetType {
    return value.decideFormatFromContent(self);
    // return 1;
  }
}

pub trait QImageReader_decideFormatFromContent<RetType> {
  fn decideFormatFromContent(self, rsthis: &mut QImageReader) -> RetType;
}

// proto:  bool QImageReader::decideFormatFromContent();
impl<'a> /*trait*/ QImageReader_decideFormatFromContent<i8> for () {
  fn decideFormatFromContent(self, rsthis: &mut QImageReader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader23decideFormatFromContentEv()};
    let mut ret = unsafe {_ZNK12QImageReader23decideFormatFromContentEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn device<RetType, T: QImageReader_device<RetType>>(&mut self, value: T) -> RetType {
    return value.device(self);
    // return 1;
  }
}

pub trait QImageReader_device<RetType> {
  fn device(self, rsthis: &mut QImageReader) -> RetType;
}

// proto:  QIODevice * QImageReader::device();
impl<'a> /*trait*/ QImageReader_device<QIODevice> for () {
  fn device(self, rsthis: &mut QImageReader) -> QIODevice {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader6deviceEv()};
    let mut ret = unsafe {_ZNK12QImageReader6deviceEv(rsthis.qclsinst)};
    let mut ret1 = QIODevice{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn autoTransform<RetType, T: QImageReader_autoTransform<RetType>>(&mut self, value: T) -> RetType {
    return value.autoTransform(self);
    // return 1;
  }
}

pub trait QImageReader_autoTransform<RetType> {
  fn autoTransform(self, rsthis: &mut QImageReader) -> RetType;
}

// proto:  bool QImageReader::autoTransform();
impl<'a> /*trait*/ QImageReader_autoTransform<i8> for () {
  fn autoTransform(self, rsthis: &mut QImageReader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader13autoTransformEv()};
    let mut ret = unsafe {_ZNK12QImageReader13autoTransformEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn jumpToNextImage<RetType, T: QImageReader_jumpToNextImage<RetType>>(&mut self, value: T) -> RetType {
    return value.jumpToNextImage(self);
    // return 1;
  }
}

pub trait QImageReader_jumpToNextImage<RetType> {
  fn jumpToNextImage(self, rsthis: &mut QImageReader) -> RetType;
}

// proto:  bool QImageReader::jumpToNextImage();
impl<'a> /*trait*/ QImageReader_jumpToNextImage<i8> for () {
  fn jumpToNextImage(self, rsthis: &mut QImageReader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReader15jumpToNextImageEv()};
    let mut ret = unsafe {_ZN12QImageReader15jumpToNextImageEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn imageFormat<RetType, T: QImageReader_imageFormat<RetType>>(&mut self, value: T) -> RetType {
    return value.imageFormat(self);
    // return 1;
  }
}

pub trait QImageReader_imageFormat<RetType> {
  fn imageFormat(self, rsthis: &mut QImageReader) -> RetType;
}

// proto: static QByteArray QImageReader::imageFormat(const QString & fileName);
impl<'a> /*trait*/ QImageReader_imageFormat<QByteArray> for (&'a  QString) {
  fn imageFormat(self, rsthis: &mut QImageReader) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReader11imageFormatERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN12QImageReader11imageFormatERK7QString(arg0)};
    let mut ret1 = QByteArray{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn supportedSubTypes<RetType, T: QImageReader_supportedSubTypes<RetType>>(&mut self, value: T) -> RetType {
    return value.supportedSubTypes(self);
    // return 1;
  }
}

pub trait QImageReader_supportedSubTypes<RetType> {
  fn supportedSubTypes(self, rsthis: &mut QImageReader) -> RetType;
}

// proto:  QList<QByteArray> QImageReader::supportedSubTypes();
impl<'a> /*trait*/ QImageReader_supportedSubTypes<()> for () {
  fn supportedSubTypes(self, rsthis: &mut QImageReader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader17supportedSubTypesEv()};
     unsafe {_ZNK12QImageReader17supportedSubTypesEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn size<RetType, T: QImageReader_size<RetType>>(&mut self, value: T) -> RetType {
    return value.size(self);
    // return 1;
  }
}

pub trait QImageReader_size<RetType> {
  fn size(self, rsthis: &mut QImageReader) -> RetType;
}

// proto:  QSize QImageReader::size();
impl<'a> /*trait*/ QImageReader_size<QSize> for () {
  fn size(self, rsthis: &mut QImageReader) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader4sizeEv()};
    let mut ret = unsafe {_ZNK12QImageReader4sizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn backgroundColor<RetType, T: QImageReader_backgroundColor<RetType>>(&mut self, value: T) -> RetType {
    return value.backgroundColor(self);
    // return 1;
  }
}

pub trait QImageReader_backgroundColor<RetType> {
  fn backgroundColor(self, rsthis: &mut QImageReader) -> RetType;
}

// proto:  QColor QImageReader::backgroundColor();
impl<'a> /*trait*/ QImageReader_backgroundColor<QColor> for () {
  fn backgroundColor(self, rsthis: &mut QImageReader) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader15backgroundColorEv()};
    let mut ret = unsafe {_ZNK12QImageReader15backgroundColorEv(rsthis.qclsinst)};
    let mut ret1 = QColor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn subType<RetType, T: QImageReader_subType<RetType>>(&mut self, value: T) -> RetType {
    return value.subType(self);
    // return 1;
  }
}

pub trait QImageReader_subType<RetType> {
  fn subType(self, rsthis: &mut QImageReader) -> RetType;
}

// proto:  QByteArray QImageReader::subType();
impl<'a> /*trait*/ QImageReader_subType<QByteArray> for () {
  fn subType(self, rsthis: &mut QImageReader) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader7subTypeEv()};
    let mut ret = unsafe {_ZNK12QImageReader7subTypeEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn currentImageNumber<RetType, T: QImageReader_currentImageNumber<RetType>>(&mut self, value: T) -> RetType {
    return value.currentImageNumber(self);
    // return 1;
  }
}

pub trait QImageReader_currentImageNumber<RetType> {
  fn currentImageNumber(self, rsthis: &mut QImageReader) -> RetType;
}

// proto:  int QImageReader::currentImageNumber();
impl<'a> /*trait*/ QImageReader_currentImageNumber<i32> for () {
  fn currentImageNumber(self, rsthis: &mut QImageReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader18currentImageNumberEv()};
    let mut ret = unsafe {_ZNK12QImageReader18currentImageNumberEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn supportedImageFormats<RetType, T: QImageReader_supportedImageFormats<RetType>>(&mut self, value: T) -> RetType {
    return value.supportedImageFormats(self);
    // return 1;
  }
}

pub trait QImageReader_supportedImageFormats<RetType> {
  fn supportedImageFormats(self, rsthis: &mut QImageReader) -> RetType;
}

// proto: static QList<QByteArray> QImageReader::supportedImageFormats();
impl<'a> /*trait*/ QImageReader_supportedImageFormats<()> for () {
  fn supportedImageFormats(self, rsthis: &mut QImageReader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReader21supportedImageFormatsEv()};
     unsafe {_ZN12QImageReader21supportedImageFormatsEv()};
    // return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn loopCount<RetType, T: QImageReader_loopCount<RetType>>(&mut self, value: T) -> RetType {
    return value.loopCount(self);
    // return 1;
  }
}

pub trait QImageReader_loopCount<RetType> {
  fn loopCount(self, rsthis: &mut QImageReader) -> RetType;
}

// proto:  int QImageReader::loopCount();
impl<'a> /*trait*/ QImageReader_loopCount<i32> for () {
  fn loopCount(self, rsthis: &mut QImageReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader9loopCountEv()};
    let mut ret = unsafe {_ZNK12QImageReader9loopCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn setDecideFormatFromContent<RetType, T: QImageReader_setDecideFormatFromContent<RetType>>(&mut self, value: T) -> RetType {
    return value.setDecideFormatFromContent(self);
    // return 1;
  }
}

pub trait QImageReader_setDecideFormatFromContent<RetType> {
  fn setDecideFormatFromContent(self, rsthis: &mut QImageReader) -> RetType;
}

// proto:  void QImageReader::setDecideFormatFromContent(bool ignored);
impl<'a> /*trait*/ QImageReader_setDecideFormatFromContent<()> for (i8) {
  fn setDecideFormatFromContent(self, rsthis: &mut QImageReader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReader26setDecideFormatFromContentEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN12QImageReader26setDecideFormatFromContentEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn scaledClipRect<RetType, T: QImageReader_scaledClipRect<RetType>>(&mut self, value: T) -> RetType {
    return value.scaledClipRect(self);
    // return 1;
  }
}

pub trait QImageReader_scaledClipRect<RetType> {
  fn scaledClipRect(self, rsthis: &mut QImageReader) -> RetType;
}

// proto:  QRect QImageReader::scaledClipRect();
impl<'a> /*trait*/ QImageReader_scaledClipRect<QRect> for () {
  fn scaledClipRect(self, rsthis: &mut QImageReader) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader14scaledClipRectEv()};
    let mut ret = unsafe {_ZNK12QImageReader14scaledClipRectEv(rsthis.qclsinst)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn supportedMimeTypes<RetType, T: QImageReader_supportedMimeTypes<RetType>>(&mut self, value: T) -> RetType {
    return value.supportedMimeTypes(self);
    // return 1;
  }
}

pub trait QImageReader_supportedMimeTypes<RetType> {
  fn supportedMimeTypes(self, rsthis: &mut QImageReader) -> RetType;
}

// proto: static QList<QByteArray> QImageReader::supportedMimeTypes();
impl<'a> /*trait*/ QImageReader_supportedMimeTypes<()> for () {
  fn supportedMimeTypes(self, rsthis: &mut QImageReader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReader18supportedMimeTypesEv()};
     unsafe {_ZN12QImageReader18supportedMimeTypesEv()};
    // return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn text<RetType, T: QImageReader_text<RetType>>(&mut self, value: T) -> RetType {
    return value.text(self);
    // return 1;
  }
}

pub trait QImageReader_text<RetType> {
  fn text(self, rsthis: &mut QImageReader) -> RetType;
}

// proto:  QString QImageReader::text(const QString & key);
impl<'a> /*trait*/ QImageReader_text<QString> for (&'a  QString) {
  fn text(self, rsthis: &mut QImageReader) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader4textERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK12QImageReader4textERK7QString(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn nextImageDelay<RetType, T: QImageReader_nextImageDelay<RetType>>(&mut self, value: T) -> RetType {
    return value.nextImageDelay(self);
    // return 1;
  }
}

pub trait QImageReader_nextImageDelay<RetType> {
  fn nextImageDelay(self, rsthis: &mut QImageReader) -> RetType;
}

// proto:  int QImageReader::nextImageDelay();
impl<'a> /*trait*/ QImageReader_nextImageDelay<i32> for () {
  fn nextImageDelay(self, rsthis: &mut QImageReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader14nextImageDelayEv()};
    let mut ret = unsafe {_ZNK12QImageReader14nextImageDelayEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn supportsAnimation<RetType, T: QImageReader_supportsAnimation<RetType>>(&mut self, value: T) -> RetType {
    return value.supportsAnimation(self);
    // return 1;
  }
}

pub trait QImageReader_supportsAnimation<RetType> {
  fn supportsAnimation(self, rsthis: &mut QImageReader) -> RetType;
}

// proto:  bool QImageReader::supportsAnimation();
impl<'a> /*trait*/ QImageReader_supportsAnimation<i8> for () {
  fn supportsAnimation(self, rsthis: &mut QImageReader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader17supportsAnimationEv()};
    let mut ret = unsafe {_ZNK12QImageReader17supportsAnimationEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn jumpToImage<RetType, T: QImageReader_jumpToImage<RetType>>(&mut self, value: T) -> RetType {
    return value.jumpToImage(self);
    // return 1;
  }
}

pub trait QImageReader_jumpToImage<RetType> {
  fn jumpToImage(self, rsthis: &mut QImageReader) -> RetType;
}

// proto:  bool QImageReader::jumpToImage(int imageNumber);
impl<'a> /*trait*/ QImageReader_jumpToImage<i8> for (i32) {
  fn jumpToImage(self, rsthis: &mut QImageReader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReader11jumpToImageEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN12QImageReader11jumpToImageEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn setFileName<RetType, T: QImageReader_setFileName<RetType>>(&mut self, value: T) -> RetType {
    return value.setFileName(self);
    // return 1;
  }
}

pub trait QImageReader_setFileName<RetType> {
  fn setFileName(self, rsthis: &mut QImageReader) -> RetType;
}

// proto:  void QImageReader::setFileName(const QString & fileName);
impl<'a> /*trait*/ QImageReader_setFileName<()> for (&'a  QString) {
  fn setFileName(self, rsthis: &mut QImageReader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReader11setFileNameERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QImageReader11setFileNameERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn NewQImageReader<T: QImageReader_NewQImageReader>(value: T) -> QImageReader {
    let rsthis = value.NewQImageReader();
    return rsthis;
    // return 1;
  }
}

pub trait QImageReader_NewQImageReader {
  fn NewQImageReader(self) -> QImageReader;
}

// proto: void QImageReader::NewQImageReader(const QImageReader & );
impl<'a> /*trait*/ QImageReader_NewQImageReader for (&'a  QImageReader) {
  fn NewQImageReader(self) -> QImageReader {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReaderC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QImageReaderC1ERKS_(qthis, arg0)};
    let rsthis = QImageReader{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn scaledSize<RetType, T: QImageReader_scaledSize<RetType>>(&mut self, value: T) -> RetType {
    return value.scaledSize(self);
    // return 1;
  }
}

pub trait QImageReader_scaledSize<RetType> {
  fn scaledSize(self, rsthis: &mut QImageReader) -> RetType;
}

// proto:  QSize QImageReader::scaledSize();
impl<'a> /*trait*/ QImageReader_scaledSize<QSize> for () {
  fn scaledSize(self, rsthis: &mut QImageReader) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader10scaledSizeEv()};
    let mut ret = unsafe {_ZNK12QImageReader10scaledSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn setAutoTransform<RetType, T: QImageReader_setAutoTransform<RetType>>(&mut self, value: T) -> RetType {
    return value.setAutoTransform(self);
    // return 1;
  }
}

pub trait QImageReader_setAutoTransform<RetType> {
  fn setAutoTransform(self, rsthis: &mut QImageReader) -> RetType;
}

// proto:  void QImageReader::setAutoTransform(bool enabled);
impl<'a> /*trait*/ QImageReader_setAutoTransform<()> for (i8) {
  fn setAutoTransform(self, rsthis: &mut QImageReader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReader16setAutoTransformEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN12QImageReader16setAutoTransformEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn setClipRect<RetType, T: QImageReader_setClipRect<RetType>>(&mut self, value: T) -> RetType {
    return value.setClipRect(self);
    // return 1;
  }
}

pub trait QImageReader_setClipRect<RetType> {
  fn setClipRect(self, rsthis: &mut QImageReader) -> RetType;
}

// proto:  void QImageReader::setClipRect(const QRect & rect);
impl<'a> /*trait*/ QImageReader_setClipRect<()> for (&'a  QRect) {
  fn setClipRect(self, rsthis: &mut QImageReader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReader11setClipRectERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QImageReader11setClipRectERK5QRect(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn autoDetectImageFormat<RetType, T: QImageReader_autoDetectImageFormat<RetType>>(&mut self, value: T) -> RetType {
    return value.autoDetectImageFormat(self);
    // return 1;
  }
}

pub trait QImageReader_autoDetectImageFormat<RetType> {
  fn autoDetectImageFormat(self, rsthis: &mut QImageReader) -> RetType;
}

// proto:  bool QImageReader::autoDetectImageFormat();
impl<'a> /*trait*/ QImageReader_autoDetectImageFormat<i8> for () {
  fn autoDetectImageFormat(self, rsthis: &mut QImageReader) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader21autoDetectImageFormatEv()};
    let mut ret = unsafe {_ZNK12QImageReader21autoDetectImageFormatEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn currentImageRect<RetType, T: QImageReader_currentImageRect<RetType>>(&mut self, value: T) -> RetType {
    return value.currentImageRect(self);
    // return 1;
  }
}

pub trait QImageReader_currentImageRect<RetType> {
  fn currentImageRect(self, rsthis: &mut QImageReader) -> RetType;
}

// proto:  QRect QImageReader::currentImageRect();
impl<'a> /*trait*/ QImageReader_currentImageRect<QRect> for () {
  fn currentImageRect(self, rsthis: &mut QImageReader) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader16currentImageRectEv()};
    let mut ret = unsafe {_ZNK12QImageReader16currentImageRectEv(rsthis.qclsinst)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QImageReader::NewQImageReader(const QString & fileName, const QByteArray & format);
impl<'a> /*trait*/ QImageReader_NewQImageReader for (&'a  QString, &'a  QByteArray) {
  fn NewQImageReader(self) -> QImageReader {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReaderC1ERK7QStringRK10QByteArray()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN12QImageReaderC1ERK7QStringRK10QByteArray(qthis, arg0, arg1)};
    let rsthis = QImageReader{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: static QByteArray QImageReader::imageFormat(QIODevice * device);
impl<'a> /*trait*/ QImageReader_imageFormat<QByteArray> for (&'a mut QIODevice) {
  fn imageFormat(self, rsthis: &mut QImageReader) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReader11imageFormatEP9QIODevice()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN12QImageReader11imageFormatEP9QIODevice(arg0)};
    let mut ret1 = QByteArray{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn quality<RetType, T: QImageReader_quality<RetType>>(&mut self, value: T) -> RetType {
    return value.quality(self);
    // return 1;
  }
}

pub trait QImageReader_quality<RetType> {
  fn quality(self, rsthis: &mut QImageReader) -> RetType;
}

// proto:  int QImageReader::quality();
impl<'a> /*trait*/ QImageReader_quality<i32> for () {
  fn quality(self, rsthis: &mut QImageReader) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader7qualityEv()};
    let mut ret = unsafe {_ZNK12QImageReader7qualityEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn setDevice<RetType, T: QImageReader_setDevice<RetType>>(&mut self, value: T) -> RetType {
    return value.setDevice(self);
    // return 1;
  }
}

pub trait QImageReader_setDevice<RetType> {
  fn setDevice(self, rsthis: &mut QImageReader) -> RetType;
}

// proto:  void QImageReader::setDevice(QIODevice * device);
impl<'a> /*trait*/ QImageReader_setDevice<()> for (&'a mut QIODevice) {
  fn setDevice(self, rsthis: &mut QImageReader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReader9setDeviceEP9QIODevice()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QImageReader9setDeviceEP9QIODevice(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn setBackgroundColor<RetType, T: QImageReader_setBackgroundColor<RetType>>(&mut self, value: T) -> RetType {
    return value.setBackgroundColor(self);
    // return 1;
  }
}

pub trait QImageReader_setBackgroundColor<RetType> {
  fn setBackgroundColor(self, rsthis: &mut QImageReader) -> RetType;
}

// proto:  void QImageReader::setBackgroundColor(const QColor & color);
impl<'a> /*trait*/ QImageReader_setBackgroundColor<()> for (&'a  QColor) {
  fn setBackgroundColor(self, rsthis: &mut QImageReader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReader18setBackgroundColorERK6QColor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QImageReader18setBackgroundColorERK6QColor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn setQuality<RetType, T: QImageReader_setQuality<RetType>>(&mut self, value: T) -> RetType {
    return value.setQuality(self);
    // return 1;
  }
}

pub trait QImageReader_setQuality<RetType> {
  fn setQuality(self, rsthis: &mut QImageReader) -> RetType;
}

// proto:  void QImageReader::setQuality(int quality);
impl<'a> /*trait*/ QImageReader_setQuality<()> for (i32) {
  fn setQuality(self, rsthis: &mut QImageReader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReader10setQualityEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN12QImageReader10setQualityEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto: void QImageReader::NewQImageReader(QIODevice * device, const QByteArray & format);
impl<'a> /*trait*/ QImageReader_NewQImageReader for (&'a mut QIODevice, &'a  QByteArray) {
  fn NewQImageReader(self) -> QImageReader {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReaderC1EP9QIODeviceRK10QByteArray()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN12QImageReaderC1EP9QIODeviceRK10QByteArray(qthis, arg0, arg1)};
    let rsthis = QImageReader{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn setAutoDetectImageFormat<RetType, T: QImageReader_setAutoDetectImageFormat<RetType>>(&mut self, value: T) -> RetType {
    return value.setAutoDetectImageFormat(self);
    // return 1;
  }
}

pub trait QImageReader_setAutoDetectImageFormat<RetType> {
  fn setAutoDetectImageFormat(self, rsthis: &mut QImageReader) -> RetType;
}

// proto:  void QImageReader::setAutoDetectImageFormat(bool enabled);
impl<'a> /*trait*/ QImageReader_setAutoDetectImageFormat<()> for (i8) {
  fn setAutoDetectImageFormat(self, rsthis: &mut QImageReader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReader24setAutoDetectImageFormatEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN12QImageReader24setAutoDetectImageFormatEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto: void QImageReader::NewQImageReader();
impl<'a> /*trait*/ QImageReader_NewQImageReader for () {
  fn NewQImageReader(self) -> QImageReader {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReaderC1Ev()};
    unsafe {_ZN12QImageReaderC1Ev(qthis)};
    let rsthis = QImageReader{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn setFormat<RetType, T: QImageReader_setFormat<RetType>>(&mut self, value: T) -> RetType {
    return value.setFormat(self);
    // return 1;
  }
}

pub trait QImageReader_setFormat<RetType> {
  fn setFormat(self, rsthis: &mut QImageReader) -> RetType;
}

// proto:  void QImageReader::setFormat(const QByteArray & format);
impl<'a> /*trait*/ QImageReader_setFormat<()> for (&'a  QByteArray) {
  fn setFormat(self, rsthis: &mut QImageReader) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QImageReader9setFormatERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QImageReader9setFormatERK10QByteArray(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn fileName<RetType, T: QImageReader_fileName<RetType>>(&mut self, value: T) -> RetType {
    return value.fileName(self);
    // return 1;
  }
}

pub trait QImageReader_fileName<RetType> {
  fn fileName(self, rsthis: &mut QImageReader) -> RetType;
}

// proto:  QString QImageReader::fileName();
impl<'a> /*trait*/ QImageReader_fileName<QString> for () {
  fn fileName(self, rsthis: &mut QImageReader) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader8fileNameEv()};
    let mut ret = unsafe {_ZNK12QImageReader8fileNameEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn clipRect<RetType, T: QImageReader_clipRect<RetType>>(&mut self, value: T) -> RetType {
    return value.clipRect(self);
    // return 1;
  }
}

pub trait QImageReader_clipRect<RetType> {
  fn clipRect(self, rsthis: &mut QImageReader) -> RetType;
}

// proto:  QRect QImageReader::clipRect();
impl<'a> /*trait*/ QImageReader_clipRect<QRect> for () {
  fn clipRect(self, rsthis: &mut QImageReader) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader8clipRectEv()};
    let mut ret = unsafe {_ZNK12QImageReader8clipRectEv(rsthis.qclsinst)};
    let mut ret1 = QRect{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QImageReader {
  pub fn format<RetType, T: QImageReader_format<RetType>>(&mut self, value: T) -> RetType {
    return value.format(self);
    // return 1;
  }
}

pub trait QImageReader_format<RetType> {
  fn format(self, rsthis: &mut QImageReader) -> RetType;
}

// proto:  QByteArray QImageReader::format();
impl<'a> /*trait*/ QImageReader_format<QByteArray> for () {
  fn format(self, rsthis: &mut QImageReader) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QImageReader6formatEv()};
    let mut ret = unsafe {_ZNK12QImageReader6formatEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

