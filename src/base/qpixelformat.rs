// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  unsigned char QPixelFormat::blackSize();
  fn _ZNK12QPixelFormat9blackSizeEv(qthis: *mut c_void) -> c_uchar;
  // proto:  void QPixelFormat::NewQPixelFormat();
  fn _ZN12QPixelFormatC1Ev(qthis: *mut c_void) ;
  // proto:  unsigned char QPixelFormat::subEnum();
  fn _ZNK12QPixelFormat7subEnumEv(qthis: *mut c_void) -> c_uchar;
  // proto:  unsigned char QPixelFormat::greenSize();
  fn _ZNK12QPixelFormat9greenSizeEv(qthis: *mut c_void) -> c_uchar;
  // proto:  unsigned char QPixelFormat::lightnessSize();
  fn _ZNK12QPixelFormat13lightnessSizeEv(qthis: *mut c_void) -> c_uchar;
  // proto:  unsigned char QPixelFormat::bitsPerPixel();
  fn _ZNK12QPixelFormat12bitsPerPixelEv(qthis: *mut c_void) -> c_uchar;
  // proto:  unsigned char QPixelFormat::alphaSize();
  fn _ZNK12QPixelFormat9alphaSizeEv(qthis: *mut c_void) -> c_uchar;
  // proto:  unsigned char QPixelFormat::magentaSize();
  fn _ZNK12QPixelFormat11magentaSizeEv(qthis: *mut c_void) -> c_uchar;
  // proto:  unsigned char QPixelFormat::hueSize();
  fn _ZNK12QPixelFormat7hueSizeEv(qthis: *mut c_void) -> c_uchar;
  // proto:  unsigned char QPixelFormat::saturationSize();
  fn _ZNK12QPixelFormat14saturationSizeEv(qthis: *mut c_void) -> c_uchar;
  // proto:  unsigned char QPixelFormat::brightnessSize();
  fn _ZNK12QPixelFormat14brightnessSizeEv(qthis: *mut c_void) -> c_uchar;
  // proto:  unsigned char QPixelFormat::yellowSize();
  fn _ZNK12QPixelFormat10yellowSizeEv(qthis: *mut c_void) -> c_uchar;
  // proto:  unsigned char QPixelFormat::redSize();
  fn _ZNK12QPixelFormat7redSizeEv(qthis: *mut c_void) -> c_uchar;
  // proto:  unsigned char QPixelFormat::blueSize();
  fn _ZNK12QPixelFormat8blueSizeEv(qthis: *mut c_void) -> c_uchar;
  // proto:  unsigned char QPixelFormat::cyanSize();
  fn _ZNK12QPixelFormat8cyanSizeEv(qthis: *mut c_void) -> c_uchar;
  // proto:  unsigned char QPixelFormat::channelCount();
  fn _ZNK12QPixelFormat12channelCountEv(qthis: *mut c_void) -> c_uchar;
}

// body block begin
// class sizeof(QPixelFormat)=8
pub struct QPixelFormat {
  pub qclsinst: *mut c_void,
}

// proto:  unsigned char QPixelFormat::blackSize();
impl /*struct*/ QPixelFormat {
  pub fn blackSize<RetType, T: QPixelFormat_blackSize<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.blackSize(self);
    // return 1;
  }
}

pub trait QPixelFormat_blackSize<RetType> {
  fn blackSize(self , rsthis: &mut QPixelFormat) -> RetType;
}

// proto:  unsigned char QPixelFormat::blackSize();
impl<'a> /*trait*/ QPixelFormat_blackSize<u8> for () {
  fn blackSize(self , rsthis: &mut QPixelFormat) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPixelFormat9blackSizeEv()};
    let mut ret = unsafe {_ZNK12QPixelFormat9blackSizeEv(rsthis.qclsinst)};
    return ret as u8;
    // return 1;
  }
}

impl /*struct*/ QPixelFormat {
  pub fn NewQPixelFormat<T: QPixelFormat_NewQPixelFormat>(value: T) -> QPixelFormat {
    let rsthis = value.NewQPixelFormat();
    return rsthis;
    // return 1;
  }
}

pub trait QPixelFormat_NewQPixelFormat {
  fn NewQPixelFormat(self) -> QPixelFormat;
}

// proto: void QPixelFormat::NewQPixelFormat();
impl<'a> /*trait*/ QPixelFormat_NewQPixelFormat for () {
  fn NewQPixelFormat(self) -> QPixelFormat {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPixelFormatC1Ev()};
    unsafe {_ZN12QPixelFormatC1Ev(qthis)};
    let rsthis = QPixelFormat{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  unsigned char QPixelFormat::subEnum();
impl /*struct*/ QPixelFormat {
  pub fn subEnum<RetType, T: QPixelFormat_subEnum<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.subEnum(self);
    // return 1;
  }
}

pub trait QPixelFormat_subEnum<RetType> {
  fn subEnum(self , rsthis: &mut QPixelFormat) -> RetType;
}

// proto:  unsigned char QPixelFormat::subEnum();
impl<'a> /*trait*/ QPixelFormat_subEnum<u8> for () {
  fn subEnum(self , rsthis: &mut QPixelFormat) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPixelFormat7subEnumEv()};
    let mut ret = unsafe {_ZNK12QPixelFormat7subEnumEv(rsthis.qclsinst)};
    return ret as u8;
    // return 1;
  }
}

// proto:  unsigned char QPixelFormat::greenSize();
impl /*struct*/ QPixelFormat {
  pub fn greenSize<RetType, T: QPixelFormat_greenSize<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.greenSize(self);
    // return 1;
  }
}

pub trait QPixelFormat_greenSize<RetType> {
  fn greenSize(self , rsthis: &mut QPixelFormat) -> RetType;
}

// proto:  unsigned char QPixelFormat::greenSize();
impl<'a> /*trait*/ QPixelFormat_greenSize<u8> for () {
  fn greenSize(self , rsthis: &mut QPixelFormat) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPixelFormat9greenSizeEv()};
    let mut ret = unsafe {_ZNK12QPixelFormat9greenSizeEv(rsthis.qclsinst)};
    return ret as u8;
    // return 1;
  }
}

// proto:  unsigned char QPixelFormat::lightnessSize();
impl /*struct*/ QPixelFormat {
  pub fn lightnessSize<RetType, T: QPixelFormat_lightnessSize<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.lightnessSize(self);
    // return 1;
  }
}

pub trait QPixelFormat_lightnessSize<RetType> {
  fn lightnessSize(self , rsthis: &mut QPixelFormat) -> RetType;
}

// proto:  unsigned char QPixelFormat::lightnessSize();
impl<'a> /*trait*/ QPixelFormat_lightnessSize<u8> for () {
  fn lightnessSize(self , rsthis: &mut QPixelFormat) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPixelFormat13lightnessSizeEv()};
    let mut ret = unsafe {_ZNK12QPixelFormat13lightnessSizeEv(rsthis.qclsinst)};
    return ret as u8;
    // return 1;
  }
}

// proto:  unsigned char QPixelFormat::bitsPerPixel();
impl /*struct*/ QPixelFormat {
  pub fn bitsPerPixel<RetType, T: QPixelFormat_bitsPerPixel<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.bitsPerPixel(self);
    // return 1;
  }
}

pub trait QPixelFormat_bitsPerPixel<RetType> {
  fn bitsPerPixel(self , rsthis: &mut QPixelFormat) -> RetType;
}

// proto:  unsigned char QPixelFormat::bitsPerPixel();
impl<'a> /*trait*/ QPixelFormat_bitsPerPixel<u8> for () {
  fn bitsPerPixel(self , rsthis: &mut QPixelFormat) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPixelFormat12bitsPerPixelEv()};
    let mut ret = unsafe {_ZNK12QPixelFormat12bitsPerPixelEv(rsthis.qclsinst)};
    return ret as u8;
    // return 1;
  }
}

// proto:  unsigned char QPixelFormat::alphaSize();
impl /*struct*/ QPixelFormat {
  pub fn alphaSize<RetType, T: QPixelFormat_alphaSize<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.alphaSize(self);
    // return 1;
  }
}

pub trait QPixelFormat_alphaSize<RetType> {
  fn alphaSize(self , rsthis: &mut QPixelFormat) -> RetType;
}

// proto:  unsigned char QPixelFormat::alphaSize();
impl<'a> /*trait*/ QPixelFormat_alphaSize<u8> for () {
  fn alphaSize(self , rsthis: &mut QPixelFormat) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPixelFormat9alphaSizeEv()};
    let mut ret = unsafe {_ZNK12QPixelFormat9alphaSizeEv(rsthis.qclsinst)};
    return ret as u8;
    // return 1;
  }
}

// proto:  unsigned char QPixelFormat::magentaSize();
impl /*struct*/ QPixelFormat {
  pub fn magentaSize<RetType, T: QPixelFormat_magentaSize<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.magentaSize(self);
    // return 1;
  }
}

pub trait QPixelFormat_magentaSize<RetType> {
  fn magentaSize(self , rsthis: &mut QPixelFormat) -> RetType;
}

// proto:  unsigned char QPixelFormat::magentaSize();
impl<'a> /*trait*/ QPixelFormat_magentaSize<u8> for () {
  fn magentaSize(self , rsthis: &mut QPixelFormat) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPixelFormat11magentaSizeEv()};
    let mut ret = unsafe {_ZNK12QPixelFormat11magentaSizeEv(rsthis.qclsinst)};
    return ret as u8;
    // return 1;
  }
}

// proto:  unsigned char QPixelFormat::hueSize();
impl /*struct*/ QPixelFormat {
  pub fn hueSize<RetType, T: QPixelFormat_hueSize<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.hueSize(self);
    // return 1;
  }
}

pub trait QPixelFormat_hueSize<RetType> {
  fn hueSize(self , rsthis: &mut QPixelFormat) -> RetType;
}

// proto:  unsigned char QPixelFormat::hueSize();
impl<'a> /*trait*/ QPixelFormat_hueSize<u8> for () {
  fn hueSize(self , rsthis: &mut QPixelFormat) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPixelFormat7hueSizeEv()};
    let mut ret = unsafe {_ZNK12QPixelFormat7hueSizeEv(rsthis.qclsinst)};
    return ret as u8;
    // return 1;
  }
}

// proto:  unsigned char QPixelFormat::saturationSize();
impl /*struct*/ QPixelFormat {
  pub fn saturationSize<RetType, T: QPixelFormat_saturationSize<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.saturationSize(self);
    // return 1;
  }
}

pub trait QPixelFormat_saturationSize<RetType> {
  fn saturationSize(self , rsthis: &mut QPixelFormat) -> RetType;
}

// proto:  unsigned char QPixelFormat::saturationSize();
impl<'a> /*trait*/ QPixelFormat_saturationSize<u8> for () {
  fn saturationSize(self , rsthis: &mut QPixelFormat) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPixelFormat14saturationSizeEv()};
    let mut ret = unsafe {_ZNK12QPixelFormat14saturationSizeEv(rsthis.qclsinst)};
    return ret as u8;
    // return 1;
  }
}

// proto:  unsigned char QPixelFormat::brightnessSize();
impl /*struct*/ QPixelFormat {
  pub fn brightnessSize<RetType, T: QPixelFormat_brightnessSize<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.brightnessSize(self);
    // return 1;
  }
}

pub trait QPixelFormat_brightnessSize<RetType> {
  fn brightnessSize(self , rsthis: &mut QPixelFormat) -> RetType;
}

// proto:  unsigned char QPixelFormat::brightnessSize();
impl<'a> /*trait*/ QPixelFormat_brightnessSize<u8> for () {
  fn brightnessSize(self , rsthis: &mut QPixelFormat) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPixelFormat14brightnessSizeEv()};
    let mut ret = unsafe {_ZNK12QPixelFormat14brightnessSizeEv(rsthis.qclsinst)};
    return ret as u8;
    // return 1;
  }
}

// proto:  unsigned char QPixelFormat::yellowSize();
impl /*struct*/ QPixelFormat {
  pub fn yellowSize<RetType, T: QPixelFormat_yellowSize<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.yellowSize(self);
    // return 1;
  }
}

pub trait QPixelFormat_yellowSize<RetType> {
  fn yellowSize(self , rsthis: &mut QPixelFormat) -> RetType;
}

// proto:  unsigned char QPixelFormat::yellowSize();
impl<'a> /*trait*/ QPixelFormat_yellowSize<u8> for () {
  fn yellowSize(self , rsthis: &mut QPixelFormat) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPixelFormat10yellowSizeEv()};
    let mut ret = unsafe {_ZNK12QPixelFormat10yellowSizeEv(rsthis.qclsinst)};
    return ret as u8;
    // return 1;
  }
}

// proto:  unsigned char QPixelFormat::redSize();
impl /*struct*/ QPixelFormat {
  pub fn redSize<RetType, T: QPixelFormat_redSize<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.redSize(self);
    // return 1;
  }
}

pub trait QPixelFormat_redSize<RetType> {
  fn redSize(self , rsthis: &mut QPixelFormat) -> RetType;
}

// proto:  unsigned char QPixelFormat::redSize();
impl<'a> /*trait*/ QPixelFormat_redSize<u8> for () {
  fn redSize(self , rsthis: &mut QPixelFormat) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPixelFormat7redSizeEv()};
    let mut ret = unsafe {_ZNK12QPixelFormat7redSizeEv(rsthis.qclsinst)};
    return ret as u8;
    // return 1;
  }
}

// proto:  unsigned char QPixelFormat::blueSize();
impl /*struct*/ QPixelFormat {
  pub fn blueSize<RetType, T: QPixelFormat_blueSize<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.blueSize(self);
    // return 1;
  }
}

pub trait QPixelFormat_blueSize<RetType> {
  fn blueSize(self , rsthis: &mut QPixelFormat) -> RetType;
}

// proto:  unsigned char QPixelFormat::blueSize();
impl<'a> /*trait*/ QPixelFormat_blueSize<u8> for () {
  fn blueSize(self , rsthis: &mut QPixelFormat) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPixelFormat8blueSizeEv()};
    let mut ret = unsafe {_ZNK12QPixelFormat8blueSizeEv(rsthis.qclsinst)};
    return ret as u8;
    // return 1;
  }
}

// proto:  unsigned char QPixelFormat::cyanSize();
impl /*struct*/ QPixelFormat {
  pub fn cyanSize<RetType, T: QPixelFormat_cyanSize<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.cyanSize(self);
    // return 1;
  }
}

pub trait QPixelFormat_cyanSize<RetType> {
  fn cyanSize(self , rsthis: &mut QPixelFormat) -> RetType;
}

// proto:  unsigned char QPixelFormat::cyanSize();
impl<'a> /*trait*/ QPixelFormat_cyanSize<u8> for () {
  fn cyanSize(self , rsthis: &mut QPixelFormat) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPixelFormat8cyanSizeEv()};
    let mut ret = unsafe {_ZNK12QPixelFormat8cyanSizeEv(rsthis.qclsinst)};
    return ret as u8;
    // return 1;
  }
}

// proto:  unsigned char QPixelFormat::channelCount();
impl /*struct*/ QPixelFormat {
  pub fn channelCount<RetType, T: QPixelFormat_channelCount<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.channelCount(self);
    // return 1;
  }
}

pub trait QPixelFormat_channelCount<RetType> {
  fn channelCount(self , rsthis: &mut QPixelFormat) -> RetType;
}

// proto:  unsigned char QPixelFormat::channelCount();
impl<'a> /*trait*/ QPixelFormat_channelCount<u8> for () {
  fn channelCount(self , rsthis: &mut QPixelFormat) -> u8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPixelFormat12channelCountEv()};
    let mut ret = unsafe {_ZNK12QPixelFormat12channelCountEv(rsthis.qclsinst)};
    return ret as u8;
    // return 1;
  }
}

