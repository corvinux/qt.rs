// auto generated, do not modify.
// created: Wed Dec 30 23:22:52 2015
// src-file: /QtGui/qregion.h
// dst-file: /src/gui/qregion.rs
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
use super::super::core::qpoint::QPoint; // 771
use super::qbitmap::QBitmap; // 773
use super::qpolygon::QPolygon; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QRegion_Class_Size() -> c_int;
  // proto:  bool QRegion::isNull();
  fn _ZNK7QRegion6isNullEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QRect QRegion::boundingRect();
  fn _ZNK7QRegion12boundingRectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QRegion::QRegion(const QRegion & region);
  fn dector_ZN7QRegionC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN7QRegionC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  int QRegion::rectCount();
  fn _ZNK7QRegion9rectCountEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QRegion::translate(int dx, int dy);
  fn _ZN7QRegion9translateEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int);
  // proto:  QRegion QRegion::united(const QRegion & r);
  fn _ZNK7QRegion6unitedERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  QRegion QRegion::translated(const QPoint & p);
  fn demth_ZNK7QRegion10translatedERK6QPoint(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QRegion::swap(QRegion & other);
  fn demth_ZN7QRegion4swapERS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QRegion::QRegion(const QBitmap & bitmap);
  fn dector_ZN7QRegionC1ERK7QBitmap(arg0: *mut c_void) -> *mut c_void;
  fn _ZN7QRegionC1ERK7QBitmap(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QRegion::~QRegion();
  fn _ZN7QRegionD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QRegion::translate(const QPoint & p);
  fn demth_ZN7QRegion9translateERK6QPoint(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QRegion::QRegion();
  fn dector_ZN7QRegionC1Ev() -> *mut c_void;
  fn _ZN7QRegionC1Ev(qthis: u64 /* *mut c_void*/);
  // proto:  bool QRegion::contains(const QRect & r);
  fn _ZNK7QRegion8containsERK5QRect(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  bool QRegion::isEmpty();
  fn _ZNK7QRegion7isEmptyEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QRegion QRegion::intersected(const QRect & r);
  fn _ZNK7QRegion11intersectedERK5QRect(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QRegion::setRects(const QRect * rect, int num);
  fn _ZN7QRegion8setRectsEPK5QRecti(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int);
  // proto:  QVector<QRect> QRegion::rects();
  fn _ZNK7QRegion5rectsEv(qthis: u64 /* *mut c_void*/);
  // proto:  QRegion QRegion::subtracted(const QRegion & r);
  fn _ZNK7QRegion10subtractedERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QRegion::intersects(const QRect & r);
  fn _ZNK7QRegion10intersectsERK5QRect(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  QRegion QRegion::translated(int dx, int dy);
  fn _ZNK7QRegion10translatedEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  QRegion QRegion::intersected(const QRegion & r);
  fn _ZNK7QRegion11intersectedERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  QRegion QRegion::united(const QRect & r);
  fn _ZNK7QRegion6unitedERK5QRect(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  QRegion QRegion::xored(const QRegion & r);
  fn _ZNK7QRegion5xoredERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QRegion::intersects(const QRegion & r);
  fn _ZNK7QRegion10intersectsERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  bool QRegion::contains(const QPoint & p);
  fn _ZNK7QRegion8containsERK6QPoint(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
} // <= ext block end

// body block begin =>
// class sizeof(QRegion)=8
#[derive(Default)]
pub struct QRegion {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QRegion {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QRegion {
    return QRegion{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  bool QRegion::isNull();
impl /*struct*/ QRegion {
  pub fn isNull<RetType, T: QRegion_isNull<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isNull(self);
    // return 1;
  }
}

pub trait QRegion_isNull<RetType> {
  fn isNull(self , rsthis: & QRegion) -> RetType;
}

  // proto:  bool QRegion::isNull();
impl<'a> /*trait*/ QRegion_isNull<i8> for () {
  fn isNull(self , rsthis: & QRegion) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QRegion6isNullEv()};
    let mut ret = unsafe {_ZNK7QRegion6isNullEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QRect QRegion::boundingRect();
impl /*struct*/ QRegion {
  pub fn boundingRect<RetType, T: QRegion_boundingRect<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.boundingRect(self);
    // return 1;
  }
}

pub trait QRegion_boundingRect<RetType> {
  fn boundingRect(self , rsthis: & QRegion) -> RetType;
}

  // proto:  QRect QRegion::boundingRect();
impl<'a> /*trait*/ QRegion_boundingRect<QRect> for () {
  fn boundingRect(self , rsthis: & QRegion) -> QRect {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QRegion12boundingRectEv()};
    let mut ret = unsafe {_ZNK7QRegion12boundingRectEv(rsthis.qclsinst)};
    let mut ret1 = QRect::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QRegion::QRegion(const QRegion & region);
impl /*struct*/ QRegion {
  pub fn New<T: QRegion_New>(value: T) -> QRegion {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QRegion_New {
  fn New(self) -> QRegion;
}

  // proto:  void QRegion::QRegion(const QRegion & region);
impl<'a> /*trait*/ QRegion_New for (&'a QRegion) {
  fn New(self) -> QRegion {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QRegionC1ERKS_()};
    let ctysz: c_int = unsafe{QRegion_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN7QRegionC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN7QRegionC1ERKS_(arg0)} as u64;
    let rsthis = QRegion{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QRegion::rectCount();
impl /*struct*/ QRegion {
  pub fn rectCount<RetType, T: QRegion_rectCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rectCount(self);
    // return 1;
  }
}

pub trait QRegion_rectCount<RetType> {
  fn rectCount(self , rsthis: & QRegion) -> RetType;
}

  // proto:  int QRegion::rectCount();
impl<'a> /*trait*/ QRegion_rectCount<i32> for () {
  fn rectCount(self , rsthis: & QRegion) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QRegion9rectCountEv()};
    let mut ret = unsafe {_ZNK7QRegion9rectCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QRegion::translate(int dx, int dy);
impl /*struct*/ QRegion {
  pub fn translate<RetType, T: QRegion_translate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.translate(self);
    // return 1;
  }
}

pub trait QRegion_translate<RetType> {
  fn translate(self , rsthis: & QRegion) -> RetType;
}

  // proto:  void QRegion::translate(int dx, int dy);
impl<'a> /*trait*/ QRegion_translate<()> for (i32, i32) {
  fn translate(self , rsthis: & QRegion) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QRegion9translateEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN7QRegion9translateEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QRegion QRegion::united(const QRegion & r);
impl /*struct*/ QRegion {
  pub fn united<RetType, T: QRegion_united<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.united(self);
    // return 1;
  }
}

pub trait QRegion_united<RetType> {
  fn united(self , rsthis: & QRegion) -> RetType;
}

  // proto:  QRegion QRegion::united(const QRegion & r);
impl<'a> /*trait*/ QRegion_united<QRegion> for (&'a QRegion) {
  fn united(self , rsthis: & QRegion) -> QRegion {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QRegion6unitedERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QRegion6unitedERKS_(rsthis.qclsinst, arg0)};
    let mut ret1 = QRegion::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QRegion QRegion::translated(const QPoint & p);
impl /*struct*/ QRegion {
  pub fn translated<RetType, T: QRegion_translated<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.translated(self);
    // return 1;
  }
}

pub trait QRegion_translated<RetType> {
  fn translated(self , rsthis: & QRegion) -> RetType;
}

  // proto:  QRegion QRegion::translated(const QPoint & p);
impl<'a> /*trait*/ QRegion_translated<QRegion> for (&'a QPoint) {
  fn translated(self , rsthis: & QRegion) -> QRegion {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QRegion10translatedERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {demth_ZNK7QRegion10translatedERK6QPoint(rsthis.qclsinst, arg0)};
    let mut ret1 = QRegion::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QRegion::swap(QRegion & other);
impl /*struct*/ QRegion {
  pub fn swap<RetType, T: QRegion_swap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QRegion_swap<RetType> {
  fn swap(self , rsthis: & QRegion) -> RetType;
}

  // proto:  void QRegion::swap(QRegion & other);
impl<'a> /*trait*/ QRegion_swap<()> for (&'a QRegion) {
  fn swap(self , rsthis: & QRegion) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QRegion4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {demth_ZN7QRegion4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QRegion::QRegion(const QBitmap & bitmap);
impl<'a> /*trait*/ QRegion_New for (&'a QBitmap) {
  fn New(self) -> QRegion {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QRegionC1ERK7QBitmap()};
    let ctysz: c_int = unsafe{QRegion_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN7QRegionC1ERK7QBitmap(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN7QRegionC1ERK7QBitmap(arg0)} as u64;
    let rsthis = QRegion{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QRegion::~QRegion();
impl /*struct*/ QRegion {
  pub fn Free<RetType, T: QRegion_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QRegion_Free<RetType> {
  fn Free(self , rsthis: & QRegion) -> RetType;
}

  // proto:  void QRegion::~QRegion();
impl<'a> /*trait*/ QRegion_Free<()> for () {
  fn Free(self , rsthis: & QRegion) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QRegionD0Ev()};
     unsafe {_ZN7QRegionD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QRegion::translate(const QPoint & p);
impl<'a> /*trait*/ QRegion_translate<()> for (&'a QPoint) {
  fn translate(self , rsthis: & QRegion) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QRegion9translateERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {demth_ZN7QRegion9translateERK6QPoint(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QRegion::QRegion();
impl<'a> /*trait*/ QRegion_New for () {
  fn New(self) -> QRegion {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QRegionC1Ev()};
    let ctysz: c_int = unsafe{QRegion_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    // unsafe {_ZN7QRegionC1Ev(qthis)};
    let qthis: u64 = unsafe {dector_ZN7QRegionC1Ev()} as u64;
    let rsthis = QRegion{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QRegion::contains(const QRect & r);
impl /*struct*/ QRegion {
  pub fn contains<RetType, T: QRegion_contains<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.contains(self);
    // return 1;
  }
}

pub trait QRegion_contains<RetType> {
  fn contains(self , rsthis: & QRegion) -> RetType;
}

  // proto:  bool QRegion::contains(const QRect & r);
impl<'a> /*trait*/ QRegion_contains<i8> for (&'a QRect) {
  fn contains(self , rsthis: & QRegion) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QRegion8containsERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QRegion8containsERK5QRect(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QRegion::isEmpty();
impl /*struct*/ QRegion {
  pub fn isEmpty<RetType, T: QRegion_isEmpty<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isEmpty(self);
    // return 1;
  }
}

pub trait QRegion_isEmpty<RetType> {
  fn isEmpty(self , rsthis: & QRegion) -> RetType;
}

  // proto:  bool QRegion::isEmpty();
impl<'a> /*trait*/ QRegion_isEmpty<i8> for () {
  fn isEmpty(self , rsthis: & QRegion) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QRegion7isEmptyEv()};
    let mut ret = unsafe {_ZNK7QRegion7isEmptyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QRegion QRegion::intersected(const QRect & r);
impl /*struct*/ QRegion {
  pub fn intersected<RetType, T: QRegion_intersected<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.intersected(self);
    // return 1;
  }
}

pub trait QRegion_intersected<RetType> {
  fn intersected(self , rsthis: & QRegion) -> RetType;
}

  // proto:  QRegion QRegion::intersected(const QRect & r);
impl<'a> /*trait*/ QRegion_intersected<QRegion> for (&'a QRect) {
  fn intersected(self , rsthis: & QRegion) -> QRegion {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QRegion11intersectedERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QRegion11intersectedERK5QRect(rsthis.qclsinst, arg0)};
    let mut ret1 = QRegion::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QRegion::setRects(const QRect * rect, int num);
impl /*struct*/ QRegion {
  pub fn setRects<RetType, T: QRegion_setRects<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setRects(self);
    // return 1;
  }
}

pub trait QRegion_setRects<RetType> {
  fn setRects(self , rsthis: & QRegion) -> RetType;
}

  // proto:  void QRegion::setRects(const QRect * rect, int num);
impl<'a> /*trait*/ QRegion_setRects<()> for (&'a QRect, i32) {
  fn setRects(self , rsthis: & QRegion) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QRegion8setRectsEPK5QRecti()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN7QRegion8setRectsEPK5QRecti(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QVector<QRect> QRegion::rects();
impl /*struct*/ QRegion {
  pub fn rects<RetType, T: QRegion_rects<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rects(self);
    // return 1;
  }
}

pub trait QRegion_rects<RetType> {
  fn rects(self , rsthis: & QRegion) -> RetType;
}

  // proto:  QVector<QRect> QRegion::rects();
impl<'a> /*trait*/ QRegion_rects<()> for () {
  fn rects(self , rsthis: & QRegion) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QRegion5rectsEv()};
     unsafe {_ZNK7QRegion5rectsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QRegion QRegion::subtracted(const QRegion & r);
impl /*struct*/ QRegion {
  pub fn subtracted<RetType, T: QRegion_subtracted<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.subtracted(self);
    // return 1;
  }
}

pub trait QRegion_subtracted<RetType> {
  fn subtracted(self , rsthis: & QRegion) -> RetType;
}

  // proto:  QRegion QRegion::subtracted(const QRegion & r);
impl<'a> /*trait*/ QRegion_subtracted<QRegion> for (&'a QRegion) {
  fn subtracted(self , rsthis: & QRegion) -> QRegion {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QRegion10subtractedERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QRegion10subtractedERKS_(rsthis.qclsinst, arg0)};
    let mut ret1 = QRegion::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QRegion::intersects(const QRect & r);
impl /*struct*/ QRegion {
  pub fn intersects<RetType, T: QRegion_intersects<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.intersects(self);
    // return 1;
  }
}

pub trait QRegion_intersects<RetType> {
  fn intersects(self , rsthis: & QRegion) -> RetType;
}

  // proto:  bool QRegion::intersects(const QRect & r);
impl<'a> /*trait*/ QRegion_intersects<i8> for (&'a QRect) {
  fn intersects(self , rsthis: & QRegion) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QRegion10intersectsERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QRegion10intersectsERK5QRect(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QRegion QRegion::translated(int dx, int dy);
impl<'a> /*trait*/ QRegion_translated<QRegion> for (i32, i32) {
  fn translated(self , rsthis: & QRegion) -> QRegion {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QRegion10translatedEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {_ZNK7QRegion10translatedEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QRegion::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QRegion QRegion::intersected(const QRegion & r);
impl<'a> /*trait*/ QRegion_intersected<QRegion> for (&'a QRegion) {
  fn intersected(self , rsthis: & QRegion) -> QRegion {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QRegion11intersectedERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QRegion11intersectedERKS_(rsthis.qclsinst, arg0)};
    let mut ret1 = QRegion::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QRegion QRegion::united(const QRect & r);
impl<'a> /*trait*/ QRegion_united<QRegion> for (&'a QRect) {
  fn united(self , rsthis: & QRegion) -> QRegion {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QRegion6unitedERK5QRect()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QRegion6unitedERK5QRect(rsthis.qclsinst, arg0)};
    let mut ret1 = QRegion::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QRegion QRegion::xored(const QRegion & r);
impl /*struct*/ QRegion {
  pub fn xored<RetType, T: QRegion_xored<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.xored(self);
    // return 1;
  }
}

pub trait QRegion_xored<RetType> {
  fn xored(self , rsthis: & QRegion) -> RetType;
}

  // proto:  QRegion QRegion::xored(const QRegion & r);
impl<'a> /*trait*/ QRegion_xored<QRegion> for (&'a QRegion) {
  fn xored(self , rsthis: & QRegion) -> QRegion {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QRegion5xoredERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QRegion5xoredERKS_(rsthis.qclsinst, arg0)};
    let mut ret1 = QRegion::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QRegion::intersects(const QRegion & r);
impl<'a> /*trait*/ QRegion_intersects<i8> for (&'a QRegion) {
  fn intersects(self , rsthis: & QRegion) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QRegion10intersectsERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QRegion10intersectsERKS_(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QRegion::contains(const QPoint & p);
impl<'a> /*trait*/ QRegion_contains<i8> for (&'a QPoint) {
  fn contains(self , rsthis: & QRegion) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QRegion8containsERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK7QRegion8containsERK6QPoint(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

// <= body block end

