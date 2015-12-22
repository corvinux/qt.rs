// auto generated, do not modify.
// created: Tue Dec 22 23:21:28 2015
// src-file: /QtGui/qimageiohandler.h
// dst-file: /src/gui/qimageiohandler.rs
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
use super::super::core::qrect::QRect; // 771
use super::super::core::qbytearray::QByteArray; // 771
use super::qimage::QImage; // 773
use super::super::core::qvariant::QVariant; // 771
use super::super::core::qiodevice::QIODevice; // 771
use super::super::core::qobject::QObject; // 771
// use super::qimageiohandler::QImageIOHandler; // 773
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  void QImageIOHandler::QImageIOHandler(const QImageIOHandler & );
  fn _ZN15QImageIOHandlerC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  int QImageIOHandler::imageCount();
  fn _ZNK15QImageIOHandler10imageCountEv(qthis: *mut c_void) -> c_int;
  // proto:  QRect QImageIOHandler::currentImageRect();
  fn _ZNK15QImageIOHandler16currentImageRectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QImageIOHandler::jumpToImage(int imageNumber);
  fn _ZN15QImageIOHandler11jumpToImageEi(qthis: *mut c_void, arg0: c_int) -> c_char;
  // proto:  int QImageIOHandler::currentImageNumber();
  fn _ZNK15QImageIOHandler18currentImageNumberEv(qthis: *mut c_void) -> c_int;
  // proto:  void QImageIOHandler::setFormat(const QByteArray & format);
  fn _ZN15QImageIOHandler9setFormatERK10QByteArray(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QImageIOHandler::jumpToNextImage();
  fn _ZN15QImageIOHandler15jumpToNextImageEv(qthis: *mut c_void) -> c_char;
  // proto:  void QImageIOHandler::~QImageIOHandler();
  fn _ZN15QImageIOHandlerD0Ev(qthis: *mut c_void);
  // proto:  int QImageIOHandler::loopCount();
  fn _ZNK15QImageIOHandler9loopCountEv(qthis: *mut c_void) -> c_int;
  // proto:  void QImageIOHandler::QImageIOHandler();
  fn _ZN15QImageIOHandlerC1Ev(qthis: *mut c_void);
  // proto:  bool QImageIOHandler::read(QImage * image);
  fn _ZN15QImageIOHandler4readEP6QImage(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  QByteArray QImageIOHandler::name();
  fn _ZNK15QImageIOHandler4nameEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QByteArray QImageIOHandler::format();
  fn _ZNK15QImageIOHandler6formatEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QImageIOHandler::nextImageDelay();
  fn _ZNK15QImageIOHandler14nextImageDelayEv(qthis: *mut c_void) -> c_int;
  // proto:  void QImageIOHandler::setDevice(QIODevice * device);
  fn _ZN15QImageIOHandler9setDeviceEP9QIODevice(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QImageIOHandler::canRead();
  fn _ZNK15QImageIOHandler7canReadEv(qthis: *mut c_void) -> c_char;
  // proto:  QIODevice * QImageIOHandler::device();
  fn _ZNK15QImageIOHandler6deviceEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QImageIOHandler::write(const QImage & image);
  fn _ZN15QImageIOHandler5writeERK6QImage(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  const QMetaObject * QImageIOPlugin::metaObject();
  fn _ZNK14QImageIOPlugin10metaObjectEv(qthis: *mut c_void);
  // proto:  void QImageIOPlugin::~QImageIOPlugin();
  fn _ZN14QImageIOPluginD0Ev(qthis: *mut c_void);
  // proto:  QImageIOHandler * QImageIOPlugin::create(QIODevice * device, const QByteArray & format);
  fn _ZNK14QImageIOPlugin6createEP9QIODeviceRK10QByteArray(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  void QImageIOPlugin::QImageIOPlugin(QObject * parent);
  fn _ZN14QImageIOPluginC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QImageIOHandler)=1
pub struct QImageIOHandler {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

// class sizeof(QImageIOPlugin)=1
pub struct QImageIOPlugin {
  qbase: QObject,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QImageIOHandler {
  pub fn inheritFrom(qthis: *mut c_void) -> QImageIOHandler {
    return QImageIOHandler{qclsinst: qthis};
  }
}
  // proto:  void QImageIOHandler::QImageIOHandler(const QImageIOHandler & );
impl /*struct*/ QImageIOHandler {
  pub fn NewQImageIOHandler<T: QImageIOHandler_NewQImageIOHandler>(value: T) -> QImageIOHandler {
    let rsthis = value.NewQImageIOHandler();
    return rsthis;
    // return 1;
  }
}

pub trait QImageIOHandler_NewQImageIOHandler {
  fn NewQImageIOHandler(self) -> QImageIOHandler;
}

  // proto:  void QImageIOHandler::QImageIOHandler(const QImageIOHandler & );
impl<'a> /*trait*/ QImageIOHandler_NewQImageIOHandler for (QImageIOHandler) {
  fn NewQImageIOHandler(self) -> QImageIOHandler {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QImageIOHandlerC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN15QImageIOHandlerC1ERKS_(qthis, arg0)};
    let rsthis = QImageIOHandler{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QImageIOHandler::imageCount();
impl /*struct*/ QImageIOHandler {
  pub fn imageCount<RetType, T: QImageIOHandler_imageCount<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.imageCount(self);
    // return 1;
  }
}

pub trait QImageIOHandler_imageCount<RetType> {
  fn imageCount(self , rsthis: &mut QImageIOHandler) -> RetType;
}

  // proto:  int QImageIOHandler::imageCount();
impl<'a> /*trait*/ QImageIOHandler_imageCount<i32> for () {
  fn imageCount(self , rsthis: &mut QImageIOHandler) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QImageIOHandler10imageCountEv()};
    let mut ret = unsafe {_ZNK15QImageIOHandler10imageCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QRect QImageIOHandler::currentImageRect();
impl /*struct*/ QImageIOHandler {
  pub fn currentImageRect<RetType, T: QImageIOHandler_currentImageRect<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.currentImageRect(self);
    // return 1;
  }
}

pub trait QImageIOHandler_currentImageRect<RetType> {
  fn currentImageRect(self , rsthis: &mut QImageIOHandler) -> RetType;
}

  // proto:  QRect QImageIOHandler::currentImageRect();
impl<'a> /*trait*/ QImageIOHandler_currentImageRect<QRect> for () {
  fn currentImageRect(self , rsthis: &mut QImageIOHandler) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QImageIOHandler16currentImageRectEv()};
    let mut ret = unsafe {_ZNK15QImageIOHandler16currentImageRectEv(rsthis.qclsinst)};
    let mut ret1 = QRect::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QImageIOHandler::jumpToImage(int imageNumber);
impl /*struct*/ QImageIOHandler {
  pub fn jumpToImage<RetType, T: QImageIOHandler_jumpToImage<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.jumpToImage(self);
    // return 1;
  }
}

pub trait QImageIOHandler_jumpToImage<RetType> {
  fn jumpToImage(self , rsthis: &mut QImageIOHandler) -> RetType;
}

  // proto:  bool QImageIOHandler::jumpToImage(int imageNumber);
impl<'a> /*trait*/ QImageIOHandler_jumpToImage<i8> for (i32) {
  fn jumpToImage(self , rsthis: &mut QImageIOHandler) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QImageIOHandler11jumpToImageEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN15QImageIOHandler11jumpToImageEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  int QImageIOHandler::currentImageNumber();
impl /*struct*/ QImageIOHandler {
  pub fn currentImageNumber<RetType, T: QImageIOHandler_currentImageNumber<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.currentImageNumber(self);
    // return 1;
  }
}

pub trait QImageIOHandler_currentImageNumber<RetType> {
  fn currentImageNumber(self , rsthis: &mut QImageIOHandler) -> RetType;
}

  // proto:  int QImageIOHandler::currentImageNumber();
impl<'a> /*trait*/ QImageIOHandler_currentImageNumber<i32> for () {
  fn currentImageNumber(self , rsthis: &mut QImageIOHandler) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QImageIOHandler18currentImageNumberEv()};
    let mut ret = unsafe {_ZNK15QImageIOHandler18currentImageNumberEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QImageIOHandler::setFormat(const QByteArray & format);
impl /*struct*/ QImageIOHandler {
  pub fn setFormat<RetType, T: QImageIOHandler_setFormat<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setFormat(self);
    // return 1;
  }
}

pub trait QImageIOHandler_setFormat<RetType> {
  fn setFormat(self , rsthis: &mut QImageIOHandler) -> RetType;
}

  // proto:  void QImageIOHandler::setFormat(const QByteArray & format);
impl<'a> /*trait*/ QImageIOHandler_setFormat<()> for (QByteArray) {
  fn setFormat(self , rsthis: &mut QImageIOHandler) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QImageIOHandler9setFormatERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QImageIOHandler9setFormatERK10QByteArray(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QImageIOHandler::jumpToNextImage();
impl /*struct*/ QImageIOHandler {
  pub fn jumpToNextImage<RetType, T: QImageIOHandler_jumpToNextImage<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.jumpToNextImage(self);
    // return 1;
  }
}

pub trait QImageIOHandler_jumpToNextImage<RetType> {
  fn jumpToNextImage(self , rsthis: &mut QImageIOHandler) -> RetType;
}

  // proto:  bool QImageIOHandler::jumpToNextImage();
impl<'a> /*trait*/ QImageIOHandler_jumpToNextImage<i8> for () {
  fn jumpToNextImage(self , rsthis: &mut QImageIOHandler) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QImageIOHandler15jumpToNextImageEv()};
    let mut ret = unsafe {_ZN15QImageIOHandler15jumpToNextImageEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QImageIOHandler::~QImageIOHandler();
impl /*struct*/ QImageIOHandler {
  pub fn FreeQImageIOHandler<RetType, T: QImageIOHandler_FreeQImageIOHandler<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQImageIOHandler(self);
    // return 1;
  }
}

pub trait QImageIOHandler_FreeQImageIOHandler<RetType> {
  fn FreeQImageIOHandler(self , rsthis: &mut QImageIOHandler) -> RetType;
}

  // proto:  void QImageIOHandler::~QImageIOHandler();
impl<'a> /*trait*/ QImageIOHandler_FreeQImageIOHandler<()> for () {
  fn FreeQImageIOHandler(self , rsthis: &mut QImageIOHandler) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QImageIOHandlerD0Ev()};
     unsafe {_ZN15QImageIOHandlerD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QImageIOHandler::loopCount();
impl /*struct*/ QImageIOHandler {
  pub fn loopCount<RetType, T: QImageIOHandler_loopCount<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.loopCount(self);
    // return 1;
  }
}

pub trait QImageIOHandler_loopCount<RetType> {
  fn loopCount(self , rsthis: &mut QImageIOHandler) -> RetType;
}

  // proto:  int QImageIOHandler::loopCount();
impl<'a> /*trait*/ QImageIOHandler_loopCount<i32> for () {
  fn loopCount(self , rsthis: &mut QImageIOHandler) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QImageIOHandler9loopCountEv()};
    let mut ret = unsafe {_ZNK15QImageIOHandler9loopCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QImageIOHandler::QImageIOHandler();
impl<'a> /*trait*/ QImageIOHandler_NewQImageIOHandler for () {
  fn NewQImageIOHandler(self) -> QImageIOHandler {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QImageIOHandlerC1Ev()};
    unsafe {_ZN15QImageIOHandlerC1Ev(qthis)};
    let rsthis = QImageIOHandler{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QImageIOHandler::read(QImage * image);
impl /*struct*/ QImageIOHandler {
  pub fn read<RetType, T: QImageIOHandler_read<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.read(self);
    // return 1;
  }
}

pub trait QImageIOHandler_read<RetType> {
  fn read(self , rsthis: &mut QImageIOHandler) -> RetType;
}

  // proto:  bool QImageIOHandler::read(QImage * image);
impl<'a> /*trait*/ QImageIOHandler_read<i8> for (QImage) {
  fn read(self , rsthis: &mut QImageIOHandler) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QImageIOHandler4readEP6QImage()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN15QImageIOHandler4readEP6QImage(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QByteArray QImageIOHandler::name();
impl /*struct*/ QImageIOHandler {
  pub fn name<RetType, T: QImageIOHandler_name<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.name(self);
    // return 1;
  }
}

pub trait QImageIOHandler_name<RetType> {
  fn name(self , rsthis: &mut QImageIOHandler) -> RetType;
}

  // proto:  QByteArray QImageIOHandler::name();
impl<'a> /*trait*/ QImageIOHandler_name<QByteArray> for () {
  fn name(self , rsthis: &mut QImageIOHandler) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QImageIOHandler4nameEv()};
    let mut ret = unsafe {_ZNK15QImageIOHandler4nameEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QByteArray QImageIOHandler::format();
impl /*struct*/ QImageIOHandler {
  pub fn format<RetType, T: QImageIOHandler_format<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.format(self);
    // return 1;
  }
}

pub trait QImageIOHandler_format<RetType> {
  fn format(self , rsthis: &mut QImageIOHandler) -> RetType;
}

  // proto:  QByteArray QImageIOHandler::format();
impl<'a> /*trait*/ QImageIOHandler_format<QByteArray> for () {
  fn format(self , rsthis: &mut QImageIOHandler) -> QByteArray {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QImageIOHandler6formatEv()};
    let mut ret = unsafe {_ZNK15QImageIOHandler6formatEv(rsthis.qclsinst)};
    let mut ret1 = QByteArray::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  int QImageIOHandler::nextImageDelay();
impl /*struct*/ QImageIOHandler {
  pub fn nextImageDelay<RetType, T: QImageIOHandler_nextImageDelay<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.nextImageDelay(self);
    // return 1;
  }
}

pub trait QImageIOHandler_nextImageDelay<RetType> {
  fn nextImageDelay(self , rsthis: &mut QImageIOHandler) -> RetType;
}

  // proto:  int QImageIOHandler::nextImageDelay();
impl<'a> /*trait*/ QImageIOHandler_nextImageDelay<i32> for () {
  fn nextImageDelay(self , rsthis: &mut QImageIOHandler) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QImageIOHandler14nextImageDelayEv()};
    let mut ret = unsafe {_ZNK15QImageIOHandler14nextImageDelayEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QImageIOHandler::setDevice(QIODevice * device);
impl /*struct*/ QImageIOHandler {
  pub fn setDevice<RetType, T: QImageIOHandler_setDevice<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setDevice(self);
    // return 1;
  }
}

pub trait QImageIOHandler_setDevice<RetType> {
  fn setDevice(self , rsthis: &mut QImageIOHandler) -> RetType;
}

  // proto:  void QImageIOHandler::setDevice(QIODevice * device);
impl<'a> /*trait*/ QImageIOHandler_setDevice<()> for (QIODevice) {
  fn setDevice(self , rsthis: &mut QImageIOHandler) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QImageIOHandler9setDeviceEP9QIODevice()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN15QImageIOHandler9setDeviceEP9QIODevice(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QImageIOHandler::canRead();
impl /*struct*/ QImageIOHandler {
  pub fn canRead<RetType, T: QImageIOHandler_canRead<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.canRead(self);
    // return 1;
  }
}

pub trait QImageIOHandler_canRead<RetType> {
  fn canRead(self , rsthis: &mut QImageIOHandler) -> RetType;
}

  // proto:  bool QImageIOHandler::canRead();
impl<'a> /*trait*/ QImageIOHandler_canRead<i8> for () {
  fn canRead(self , rsthis: &mut QImageIOHandler) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QImageIOHandler7canReadEv()};
    let mut ret = unsafe {_ZNK15QImageIOHandler7canReadEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QIODevice * QImageIOHandler::device();
impl /*struct*/ QImageIOHandler {
  pub fn device<RetType, T: QImageIOHandler_device<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.device(self);
    // return 1;
  }
}

pub trait QImageIOHandler_device<RetType> {
  fn device(self , rsthis: &mut QImageIOHandler) -> RetType;
}

  // proto:  QIODevice * QImageIOHandler::device();
impl<'a> /*trait*/ QImageIOHandler_device<QIODevice> for () {
  fn device(self , rsthis: &mut QImageIOHandler) -> QIODevice {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QImageIOHandler6deviceEv()};
    let mut ret = unsafe {_ZNK15QImageIOHandler6deviceEv(rsthis.qclsinst)};
    let mut ret1 = QIODevice::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QImageIOHandler::write(const QImage & image);
impl /*struct*/ QImageIOHandler {
  pub fn write<RetType, T: QImageIOHandler_write<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.write(self);
    // return 1;
  }
}

pub trait QImageIOHandler_write<RetType> {
  fn write(self , rsthis: &mut QImageIOHandler) -> RetType;
}

  // proto:  bool QImageIOHandler::write(const QImage & image);
impl<'a> /*trait*/ QImageIOHandler_write<i8> for (QImage) {
  fn write(self , rsthis: &mut QImageIOHandler) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QImageIOHandler5writeERK6QImage()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN15QImageIOHandler5writeERK6QImage(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QImageIOPlugin {
  pub fn inheritFrom(qthis: *mut c_void) -> QImageIOPlugin {
    return QImageIOPlugin{qbase: QObject::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QImageIOPlugin {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return &self.qbase;
  }
}
impl AsRef<QObject> for QImageIOPlugin {
  fn as_ref(&self) -> &QObject {
    return &self.qbase;
  }
}
  // proto:  const QMetaObject * QImageIOPlugin::metaObject();
impl /*struct*/ QImageIOPlugin {
  pub fn metaObject<RetType, T: QImageIOPlugin_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QImageIOPlugin_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QImageIOPlugin) -> RetType;
}

  // proto:  const QMetaObject * QImageIOPlugin::metaObject();
impl<'a> /*trait*/ QImageIOPlugin_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QImageIOPlugin) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QImageIOPlugin10metaObjectEv()};
     unsafe {_ZNK14QImageIOPlugin10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QImageIOPlugin::~QImageIOPlugin();
impl /*struct*/ QImageIOPlugin {
  pub fn FreeQImageIOPlugin<RetType, T: QImageIOPlugin_FreeQImageIOPlugin<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQImageIOPlugin(self);
    // return 1;
  }
}

pub trait QImageIOPlugin_FreeQImageIOPlugin<RetType> {
  fn FreeQImageIOPlugin(self , rsthis: &mut QImageIOPlugin) -> RetType;
}

  // proto:  void QImageIOPlugin::~QImageIOPlugin();
impl<'a> /*trait*/ QImageIOPlugin_FreeQImageIOPlugin<()> for () {
  fn FreeQImageIOPlugin(self , rsthis: &mut QImageIOPlugin) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QImageIOPluginD0Ev()};
     unsafe {_ZN14QImageIOPluginD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QImageIOHandler * QImageIOPlugin::create(QIODevice * device, const QByteArray & format);
impl /*struct*/ QImageIOPlugin {
  pub fn create<RetType, T: QImageIOPlugin_create<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.create(self);
    // return 1;
  }
}

pub trait QImageIOPlugin_create<RetType> {
  fn create(self , rsthis: &mut QImageIOPlugin) -> RetType;
}

  // proto:  QImageIOHandler * QImageIOPlugin::create(QIODevice * device, const QByteArray & format);
impl<'a> /*trait*/ QImageIOPlugin_create<QImageIOHandler> for (QIODevice, QByteArray) {
  fn create(self , rsthis: &mut QImageIOPlugin) -> QImageIOHandler {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QImageIOPlugin6createEP9QIODeviceRK10QByteArray()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK14QImageIOPlugin6createEP9QIODeviceRK10QByteArray(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QImageIOHandler::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QImageIOPlugin::QImageIOPlugin(QObject * parent);
impl /*struct*/ QImageIOPlugin {
  pub fn NewQImageIOPlugin<T: QImageIOPlugin_NewQImageIOPlugin>(value: T) -> QImageIOPlugin {
    let rsthis = value.NewQImageIOPlugin();
    return rsthis;
    // return 1;
  }
}

pub trait QImageIOPlugin_NewQImageIOPlugin {
  fn NewQImageIOPlugin(self) -> QImageIOPlugin;
}

  // proto:  void QImageIOPlugin::QImageIOPlugin(QObject * parent);
impl<'a> /*trait*/ QImageIOPlugin_NewQImageIOPlugin for (QObject) {
  fn NewQImageIOPlugin(self) -> QImageIOPlugin {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QImageIOPluginC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN14QImageIOPluginC1EP7QObject(qthis, arg0)};
    let rsthis = QImageIOPlugin{/**/qbase: QObject::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// <= body block end

