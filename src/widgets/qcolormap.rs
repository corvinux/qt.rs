// auto generated, do not modify.
// created: Sat Dec 26 10:52:38 2015
// src-file: /QtWidgets/qcolormap.h
// dst-file: /src/widgets/qcolormap.rs
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
use super::super::gui::qcolor::QColor; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QColormap_Class_Size() -> c_int;
  // proto:  uint QColormap::pixel(const QColor & color);
  fn _ZNK9QColormap5pixelERK6QColor(qthis: *mut c_void, arg0: *mut c_void) -> c_uint;
  // proto:  const QVector<QColor> QColormap::colormap();
  fn _ZNK9QColormap8colormapEv(qthis: *mut c_void);
  // proto:  const QColor QColormap::colorAt(uint pixel);
  fn _ZNK9QColormap7colorAtEj(qthis: *mut c_void, arg0: c_uint) -> *mut c_void;
  // proto:  void QColormap::~QColormap();
  fn _ZN9QColormapD0Ev(qthis: *mut c_void);
  // proto:  void QColormap::QColormap();
  fn dector_ZN9QColormapC1Ev() -> *mut c_void;
  fn _ZN9QColormapC1Ev(qthis: *mut c_void);
  // proto: static QColormap QColormap::instance(int screen);
  fn _ZN9QColormap8instanceEi(arg0: c_int) -> *mut c_void;
  // proto:  int QColormap::size();
  fn _ZNK9QColormap4sizeEv(qthis: *mut c_void) -> c_int;
  // proto:  void QColormap::QColormap(const QColormap & colormap);
  fn dector_ZN9QColormapC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN9QColormapC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto: static void QColormap::initialize();
  fn _ZN9QColormap10initializeEv();
  // proto:  int QColormap::depth();
  fn _ZNK9QColormap5depthEv(qthis: *mut c_void) -> c_int;
  // proto: static void QColormap::cleanup();
  fn _ZN9QColormap7cleanupEv();
} // <= ext block end

// body block begin =>
// class sizeof(QColormap)=8
pub struct QColormap {
  // qbase: None,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QColormap {
  pub fn inheritFrom(qthis: *mut c_void) -> QColormap {
    return QColormap{qclsinst: qthis};
  }
}
  // proto:  uint QColormap::pixel(const QColor & color);
impl /*struct*/ QColormap {
  pub fn pixel<RetType, T: QColormap_pixel<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.pixel(self);
    // return 1;
  }
}

pub trait QColormap_pixel<RetType> {
  fn pixel(self , rsthis: & QColormap) -> RetType;
}

  // proto:  uint QColormap::pixel(const QColor & color);
impl<'a> /*trait*/ QColormap_pixel<u32> for (&'a QColor) {
  fn pixel(self , rsthis: & QColormap) -> u32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QColormap5pixelERK6QColor()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK9QColormap5pixelERK6QColor(rsthis.qclsinst, arg0)};
    return ret as u32;
    // return 1;
  }
}

  // proto:  const QVector<QColor> QColormap::colormap();
impl /*struct*/ QColormap {
  pub fn colormap<RetType, T: QColormap_colormap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.colormap(self);
    // return 1;
  }
}

pub trait QColormap_colormap<RetType> {
  fn colormap(self , rsthis: & QColormap) -> RetType;
}

  // proto:  const QVector<QColor> QColormap::colormap();
impl<'a> /*trait*/ QColormap_colormap<()> for () {
  fn colormap(self , rsthis: & QColormap) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QColormap8colormapEv()};
     unsafe {_ZNK9QColormap8colormapEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  const QColor QColormap::colorAt(uint pixel);
impl /*struct*/ QColormap {
  pub fn colorAt<RetType, T: QColormap_colorAt<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.colorAt(self);
    // return 1;
  }
}

pub trait QColormap_colorAt<RetType> {
  fn colorAt(self , rsthis: & QColormap) -> RetType;
}

  // proto:  const QColor QColormap::colorAt(uint pixel);
impl<'a> /*trait*/ QColormap_colorAt<QColor> for (u32) {
  fn colorAt(self , rsthis: & QColormap) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QColormap7colorAtEj()};
    let arg0 = self  as c_uint;
    let mut ret = unsafe {_ZNK9QColormap7colorAtEj(rsthis.qclsinst, arg0)};
    let mut ret1 = QColor::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QColormap::~QColormap();
impl /*struct*/ QColormap {
  pub fn Free<RetType, T: QColormap_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QColormap_Free<RetType> {
  fn Free(self , rsthis: & QColormap) -> RetType;
}

  // proto:  void QColormap::~QColormap();
impl<'a> /*trait*/ QColormap_Free<()> for () {
  fn Free(self , rsthis: & QColormap) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QColormapD0Ev()};
     unsafe {_ZN9QColormapD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QColormap::QColormap();
impl /*struct*/ QColormap {
  pub fn New<T: QColormap_New>(value: T) -> QColormap {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QColormap_New {
  fn New(self) -> QColormap;
}

  // proto:  void QColormap::QColormap();
impl<'a> /*trait*/ QColormap_New for () {
  fn New(self) -> QColormap {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QColormapC1Ev()};
    let ctysz: c_int = unsafe{QColormap_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    // unsafe {_ZN9QColormapC1Ev(qthis)};
    let qthis: *mut c_void = unsafe {dector_ZN9QColormapC1Ev()};
    let rsthis = QColormap{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto: static QColormap QColormap::instance(int screen);
impl /*struct*/ QColormap {
  pub fn instance_s<RetType, T: QColormap_instance_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.instance_s();
    // return 1;
  }
}

pub trait QColormap_instance_s<RetType> {
  fn instance_s(self ) -> RetType;
}

  // proto: static QColormap QColormap::instance(int screen);
impl<'a> /*trait*/ QColormap_instance_s<QColormap> for (i32) {
  fn instance_s(self ) -> QColormap {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QColormap8instanceEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN9QColormap8instanceEi(arg0)};
    let mut ret1 = QColormap::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  int QColormap::size();
impl /*struct*/ QColormap {
  pub fn size<RetType, T: QColormap_size<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.size(self);
    // return 1;
  }
}

pub trait QColormap_size<RetType> {
  fn size(self , rsthis: & QColormap) -> RetType;
}

  // proto:  int QColormap::size();
impl<'a> /*trait*/ QColormap_size<i32> for () {
  fn size(self , rsthis: & QColormap) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QColormap4sizeEv()};
    let mut ret = unsafe {_ZNK9QColormap4sizeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QColormap::QColormap(const QColormap & colormap);
impl<'a> /*trait*/ QColormap_New for (&'a QColormap) {
  fn New(self) -> QColormap {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QColormapC1ERKS_()};
    let ctysz: c_int = unsafe{QColormap_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN9QColormapC1ERKS_(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN9QColormapC1ERKS_(arg0)};
    let rsthis = QColormap{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto: static void QColormap::initialize();
impl /*struct*/ QColormap {
  pub fn initialize_s<RetType, T: QColormap_initialize_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.initialize_s();
    // return 1;
  }
}

pub trait QColormap_initialize_s<RetType> {
  fn initialize_s(self ) -> RetType;
}

  // proto: static void QColormap::initialize();
impl<'a> /*trait*/ QColormap_initialize_s<()> for () {
  fn initialize_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QColormap10initializeEv()};
     unsafe {_ZN9QColormap10initializeEv()};
    // return 1;
  }
}

  // proto:  int QColormap::depth();
impl /*struct*/ QColormap {
  pub fn depth<RetType, T: QColormap_depth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.depth(self);
    // return 1;
  }
}

pub trait QColormap_depth<RetType> {
  fn depth(self , rsthis: & QColormap) -> RetType;
}

  // proto:  int QColormap::depth();
impl<'a> /*trait*/ QColormap_depth<i32> for () {
  fn depth(self , rsthis: & QColormap) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QColormap5depthEv()};
    let mut ret = unsafe {_ZNK9QColormap5depthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto: static void QColormap::cleanup();
impl /*struct*/ QColormap {
  pub fn cleanup_s<RetType, T: QColormap_cleanup_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.cleanup_s();
    // return 1;
  }
}

pub trait QColormap_cleanup_s<RetType> {
  fn cleanup_s(self ) -> RetType;
}

  // proto: static void QColormap::cleanup();
impl<'a> /*trait*/ QColormap_cleanup_s<()> for () {
  fn cleanup_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QColormap7cleanupEv()};
     unsafe {_ZN9QColormap7cleanupEv()};
    // return 1;
  }
}

// <= body block end

