// auto generated, do not modify.
// created: Mon Feb 22 23:57:02 2016
// src-file: /QtCore/qpoint.h
// dst-file: /src/core/qpoint.rs
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
// use super::qpoint::QPoint; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QPoint_Class_Size() -> c_int;
  // proto:  int & QPoint::ry();
  fn C_ZN6QPoint2ryEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto: static int QPoint::dotProduct(const QPoint & p1, const QPoint & p2);
  fn C_ZN6QPoint10dotProductERKS_S1_(arg0: *mut c_void, arg1: *mut c_void) -> c_int;
  // proto:  int QPoint::x();
  fn C_ZNK6QPoint1xEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QPoint::QPoint(int xpos, int ypos);
  fn C_ZN6QPointC2Eii(arg0: c_int, arg1: c_int) -> u64;
  // proto:  int QPoint::y();
  fn C_ZNK6QPoint1yEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QPoint::setX(int x);
  fn C_ZN6QPoint4setXEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  bool QPoint::isNull();
  fn C_ZNK6QPoint6isNullEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QPoint::QPoint();
  fn C_ZN6QPointC2Ev() -> u64;
  // proto:  void QPoint::setY(int y);
  fn C_ZN6QPoint4setYEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  int & QPoint::rx();
  fn C_ZN6QPoint2rxEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  int QPoint::manhattanLength();
  fn C_ZNK6QPoint15manhattanLengthEv(qthis: u64 /* *mut c_void*/) -> c_int;
  fn QPointF_Class_Size() -> c_int;
  // proto:  void QPointF::QPointF(qreal xpos, qreal ypos);
  fn C_ZN7QPointFC2Edd(arg0: c_double, arg1: c_double) -> u64;
  // proto:  void QPointF::QPointF();
  fn C_ZN7QPointFC2Ev() -> u64;
  // proto:  qreal QPointF::manhattanLength();
  fn C_ZNK7QPointF15manhattanLengthEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  QPoint QPointF::toPoint();
  fn C_ZNK7QPointF7toPointEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  qreal & QPointF::rx();
  fn C_ZN7QPointF2rxEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  qreal QPointF::y();
  fn C_ZNK7QPointF1yEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  bool QPointF::isNull();
  fn C_ZNK7QPointF6isNullEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  qreal QPointF::x();
  fn C_ZNK7QPointF1xEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  void QPointF::QPointF(const QPoint & p);
  fn C_ZN7QPointFC2ERK6QPoint(arg0: *mut c_void) -> u64;
  // proto:  void QPointF::setX(qreal x);
  fn C_ZN7QPointF4setXEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  qreal & QPointF::ry();
  fn C_ZN7QPointF2ryEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto: static qreal QPointF::dotProduct(const QPointF & p1, const QPointF & p2);
  fn C_ZN7QPointF10dotProductERKS_S1_(arg0: *mut c_void, arg1: *mut c_void) -> c_double;
  // proto:  void QPointF::setY(qreal y);
  fn C_ZN7QPointF4setYEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
} // <= ext block end

// body block begin =>
// class sizeof(QPoint)=8
#[derive(Default)]
pub struct QPoint {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QPointF)=16
#[derive(Default)]
pub struct QPointF {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QPoint {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QPoint {
    return QPoint{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  int & QPoint::ry();
impl /*struct*/ QPoint {
  pub fn ry<RetType, T: QPoint_ry<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.ry(self);
    // return 1;
  }
}

pub trait QPoint_ry<RetType> {
  fn ry(self , rsthis: & QPoint) -> RetType;
}

  // proto:  int & QPoint::ry();
impl<'a> /*trait*/ QPoint_ry<i32> for () {
  fn ry(self , rsthis: & QPoint) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QPoint2ryEv()};
    let mut ret = unsafe {C_ZN6QPoint2ryEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto: static int QPoint::dotProduct(const QPoint & p1, const QPoint & p2);
impl /*struct*/ QPoint {
  pub fn dotProduct_s<RetType, T: QPoint_dotProduct_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.dotProduct_s();
    // return 1;
  }
}

pub trait QPoint_dotProduct_s<RetType> {
  fn dotProduct_s(self ) -> RetType;
}

  // proto: static int QPoint::dotProduct(const QPoint & p1, const QPoint & p2);
impl<'a> /*trait*/ QPoint_dotProduct_s<i32> for (&'a QPoint, &'a QPoint) {
  fn dotProduct_s(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QPoint10dotProductERKS_S1_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN6QPoint10dotProductERKS_S1_(arg0, arg1)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  int QPoint::x();
impl /*struct*/ QPoint {
  pub fn x<RetType, T: QPoint_x<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.x(self);
    // return 1;
  }
}

pub trait QPoint_x<RetType> {
  fn x(self , rsthis: & QPoint) -> RetType;
}

  // proto:  int QPoint::x();
impl<'a> /*trait*/ QPoint_x<i32> for () {
  fn x(self , rsthis: & QPoint) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QPoint1xEv()};
    let mut ret = unsafe {C_ZNK6QPoint1xEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  void QPoint::QPoint(int xpos, int ypos);
impl /*struct*/ QPoint {
  pub fn new<T: QPoint_new>(value: T) -> QPoint {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QPoint_new {
  fn new(self) -> QPoint;
}

  // proto:  void QPoint::QPoint(int xpos, int ypos);
impl<'a> /*trait*/ QPoint_new for (i32, i32) {
  fn new(self) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QPointC2Eii()};
    let ctysz: c_int = unsafe{QPoint_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let qthis: u64 = unsafe {C_ZN6QPointC2Eii(arg0, arg1)};
    let rsthis = QPoint{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QPoint::y();
impl /*struct*/ QPoint {
  pub fn y<RetType, T: QPoint_y<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.y(self);
    // return 1;
  }
}

pub trait QPoint_y<RetType> {
  fn y(self , rsthis: & QPoint) -> RetType;
}

  // proto:  int QPoint::y();
impl<'a> /*trait*/ QPoint_y<i32> for () {
  fn y(self , rsthis: & QPoint) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QPoint1yEv()};
    let mut ret = unsafe {C_ZNK6QPoint1yEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  void QPoint::setX(int x);
impl /*struct*/ QPoint {
  pub fn setX<RetType, T: QPoint_setX<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setX(self);
    // return 1;
  }
}

pub trait QPoint_setX<RetType> {
  fn setX(self , rsthis: & QPoint) -> RetType;
}

  // proto:  void QPoint::setX(int x);
impl<'a> /*trait*/ QPoint_setX<()> for (i32) {
  fn setX(self , rsthis: & QPoint) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QPoint4setXEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN6QPoint4setXEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QPoint::isNull();
impl /*struct*/ QPoint {
  pub fn isNull<RetType, T: QPoint_isNull<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isNull(self);
    // return 1;
  }
}

pub trait QPoint_isNull<RetType> {
  fn isNull(self , rsthis: & QPoint) -> RetType;
}

  // proto:  bool QPoint::isNull();
impl<'a> /*trait*/ QPoint_isNull<i8> for () {
  fn isNull(self , rsthis: & QPoint) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QPoint6isNullEv()};
    let mut ret = unsafe {C_ZNK6QPoint6isNullEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QPoint::QPoint();
impl<'a> /*trait*/ QPoint_new for () {
  fn new(self) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QPointC2Ev()};
    let ctysz: c_int = unsafe{QPoint_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN6QPointC2Ev()};
    let rsthis = QPoint{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QPoint::setY(int y);
impl /*struct*/ QPoint {
  pub fn setY<RetType, T: QPoint_setY<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setY(self);
    // return 1;
  }
}

pub trait QPoint_setY<RetType> {
  fn setY(self , rsthis: & QPoint) -> RetType;
}

  // proto:  void QPoint::setY(int y);
impl<'a> /*trait*/ QPoint_setY<()> for (i32) {
  fn setY(self , rsthis: & QPoint) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QPoint4setYEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN6QPoint4setYEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int & QPoint::rx();
impl /*struct*/ QPoint {
  pub fn rx<RetType, T: QPoint_rx<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rx(self);
    // return 1;
  }
}

pub trait QPoint_rx<RetType> {
  fn rx(self , rsthis: & QPoint) -> RetType;
}

  // proto:  int & QPoint::rx();
impl<'a> /*trait*/ QPoint_rx<i32> for () {
  fn rx(self , rsthis: & QPoint) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QPoint2rxEv()};
    let mut ret = unsafe {C_ZN6QPoint2rxEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  int QPoint::manhattanLength();
impl /*struct*/ QPoint {
  pub fn manhattanLength<RetType, T: QPoint_manhattanLength<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.manhattanLength(self);
    // return 1;
  }
}

pub trait QPoint_manhattanLength<RetType> {
  fn manhattanLength(self , rsthis: & QPoint) -> RetType;
}

  // proto:  int QPoint::manhattanLength();
impl<'a> /*trait*/ QPoint_manhattanLength<i32> for () {
  fn manhattanLength(self , rsthis: & QPoint) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QPoint15manhattanLengthEv()};
    let mut ret = unsafe {C_ZNK6QPoint15manhattanLengthEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

impl /*struct*/ QPointF {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QPointF {
    return QPointF{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QPointF::QPointF(qreal xpos, qreal ypos);
impl /*struct*/ QPointF {
  pub fn new<T: QPointF_new>(value: T) -> QPointF {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QPointF_new {
  fn new(self) -> QPointF;
}

  // proto:  void QPointF::QPointF(qreal xpos, qreal ypos);
impl<'a> /*trait*/ QPointF_new for (f64, f64) {
  fn new(self) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPointFC2Edd()};
    let ctysz: c_int = unsafe{QPointF_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let qthis: u64 = unsafe {C_ZN7QPointFC2Edd(arg0, arg1)};
    let rsthis = QPointF{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QPointF::QPointF();
impl<'a> /*trait*/ QPointF_new for () {
  fn new(self) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPointFC2Ev()};
    let ctysz: c_int = unsafe{QPointF_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN7QPointFC2Ev()};
    let rsthis = QPointF{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  qreal QPointF::manhattanLength();
impl /*struct*/ QPointF {
  pub fn manhattanLength<RetType, T: QPointF_manhattanLength<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.manhattanLength(self);
    // return 1;
  }
}

pub trait QPointF_manhattanLength<RetType> {
  fn manhattanLength(self , rsthis: & QPointF) -> RetType;
}

  // proto:  qreal QPointF::manhattanLength();
impl<'a> /*trait*/ QPointF_manhattanLength<f64> for () {
  fn manhattanLength(self , rsthis: & QPointF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPointF15manhattanLengthEv()};
    let mut ret = unsafe {C_ZNK7QPointF15manhattanLengthEv(rsthis.qclsinst)};
    return ret as f64; // 1
    // return 1;
  }
}

  // proto:  QPoint QPointF::toPoint();
impl /*struct*/ QPointF {
  pub fn toPoint<RetType, T: QPointF_toPoint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toPoint(self);
    // return 1;
  }
}

pub trait QPointF_toPoint<RetType> {
  fn toPoint(self , rsthis: & QPointF) -> RetType;
}

  // proto:  QPoint QPointF::toPoint();
impl<'a> /*trait*/ QPointF_toPoint<QPoint> for () {
  fn toPoint(self , rsthis: & QPointF) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPointF7toPointEv()};
    let mut ret = unsafe {C_ZNK7QPointF7toPointEv(rsthis.qclsinst)};
    let mut ret1 = QPoint::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  qreal & QPointF::rx();
impl /*struct*/ QPointF {
  pub fn rx<RetType, T: QPointF_rx<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rx(self);
    // return 1;
  }
}

pub trait QPointF_rx<RetType> {
  fn rx(self , rsthis: & QPointF) -> RetType;
}

  // proto:  qreal & QPointF::rx();
impl<'a> /*trait*/ QPointF_rx<f64> for () {
  fn rx(self , rsthis: & QPointF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPointF2rxEv()};
    let mut ret = unsafe {C_ZN7QPointF2rxEv(rsthis.qclsinst)};
    return ret as f64; // 1
    // return 1;
  }
}

  // proto:  qreal QPointF::y();
impl /*struct*/ QPointF {
  pub fn y<RetType, T: QPointF_y<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.y(self);
    // return 1;
  }
}

pub trait QPointF_y<RetType> {
  fn y(self , rsthis: & QPointF) -> RetType;
}

  // proto:  qreal QPointF::y();
impl<'a> /*trait*/ QPointF_y<f64> for () {
  fn y(self , rsthis: & QPointF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPointF1yEv()};
    let mut ret = unsafe {C_ZNK7QPointF1yEv(rsthis.qclsinst)};
    return ret as f64; // 1
    // return 1;
  }
}

  // proto:  bool QPointF::isNull();
impl /*struct*/ QPointF {
  pub fn isNull<RetType, T: QPointF_isNull<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isNull(self);
    // return 1;
  }
}

pub trait QPointF_isNull<RetType> {
  fn isNull(self , rsthis: & QPointF) -> RetType;
}

  // proto:  bool QPointF::isNull();
impl<'a> /*trait*/ QPointF_isNull<i8> for () {
  fn isNull(self , rsthis: & QPointF) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPointF6isNullEv()};
    let mut ret = unsafe {C_ZNK7QPointF6isNullEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  qreal QPointF::x();
impl /*struct*/ QPointF {
  pub fn x<RetType, T: QPointF_x<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.x(self);
    // return 1;
  }
}

pub trait QPointF_x<RetType> {
  fn x(self , rsthis: & QPointF) -> RetType;
}

  // proto:  qreal QPointF::x();
impl<'a> /*trait*/ QPointF_x<f64> for () {
  fn x(self , rsthis: & QPointF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK7QPointF1xEv()};
    let mut ret = unsafe {C_ZNK7QPointF1xEv(rsthis.qclsinst)};
    return ret as f64; // 1
    // return 1;
  }
}

  // proto:  void QPointF::QPointF(const QPoint & p);
impl<'a> /*trait*/ QPointF_new for (&'a QPoint) {
  fn new(self) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPointFC2ERK6QPoint()};
    let ctysz: c_int = unsafe{QPointF_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN7QPointFC2ERK6QPoint(arg0)};
    let rsthis = QPointF{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QPointF::setX(qreal x);
impl /*struct*/ QPointF {
  pub fn setX<RetType, T: QPointF_setX<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setX(self);
    // return 1;
  }
}

pub trait QPointF_setX<RetType> {
  fn setX(self , rsthis: & QPointF) -> RetType;
}

  // proto:  void QPointF::setX(qreal x);
impl<'a> /*trait*/ QPointF_setX<()> for (f64) {
  fn setX(self , rsthis: & QPointF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPointF4setXEd()};
    let arg0 = self  as c_double;
     unsafe {C_ZN7QPointF4setXEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  qreal & QPointF::ry();
impl /*struct*/ QPointF {
  pub fn ry<RetType, T: QPointF_ry<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.ry(self);
    // return 1;
  }
}

pub trait QPointF_ry<RetType> {
  fn ry(self , rsthis: & QPointF) -> RetType;
}

  // proto:  qreal & QPointF::ry();
impl<'a> /*trait*/ QPointF_ry<f64> for () {
  fn ry(self , rsthis: & QPointF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPointF2ryEv()};
    let mut ret = unsafe {C_ZN7QPointF2ryEv(rsthis.qclsinst)};
    return ret as f64; // 1
    // return 1;
  }
}

  // proto: static qreal QPointF::dotProduct(const QPointF & p1, const QPointF & p2);
impl /*struct*/ QPointF {
  pub fn dotProduct_s<RetType, T: QPointF_dotProduct_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.dotProduct_s();
    // return 1;
  }
}

pub trait QPointF_dotProduct_s<RetType> {
  fn dotProduct_s(self ) -> RetType;
}

  // proto: static qreal QPointF::dotProduct(const QPointF & p1, const QPointF & p2);
impl<'a> /*trait*/ QPointF_dotProduct_s<f64> for (&'a QPointF, &'a QPointF) {
  fn dotProduct_s(self ) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPointF10dotProductERKS_S1_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZN7QPointF10dotProductERKS_S1_(arg0, arg1)};
    return ret as f64; // 1
    // return 1;
  }
}

  // proto:  void QPointF::setY(qreal y);
impl /*struct*/ QPointF {
  pub fn setY<RetType, T: QPointF_setY<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setY(self);
    // return 1;
  }
}

pub trait QPointF_setY<RetType> {
  fn setY(self , rsthis: & QPointF) -> RetType;
}

  // proto:  void QPointF::setY(qreal y);
impl<'a> /*trait*/ QPointF_setY<()> for (f64) {
  fn setY(self , rsthis: & QPointF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN7QPointF4setYEd()};
    let arg0 = self  as c_double;
     unsafe {C_ZN7QPointF4setYEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// <= body block end

