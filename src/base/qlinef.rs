// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qpointf::QPointF;
use super::qline::QLine;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QLineF::translate(qreal dx, qreal dy);
  fn _ZN6QLineF9translateEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double) ;
  // proto:  void QLineF::setPoints(const QPointF & p1, const QPointF & p2);
  fn _ZN6QLineF9setPointsERK7QPointFS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  void QLineF::setP2(const QPointF & p2);
  fn _ZN6QLineF5setP2ERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QLineF QLineF::translated(qreal dx, qreal dy);
  fn _ZNK6QLineF10translatedEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double) -> *mut c_void;
  // proto:  void QLineF::setLength(qreal len);
  fn _ZN6QLineF9setLengthEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  double QLineF::x1();
  fn _ZNK6QLineF2x1Ev(qthis: *mut c_void) -> c_double;
  // proto:  double QLineF::angle();
  fn _ZNK6QLineF5angleEv(qthis: *mut c_void) -> c_double;
  // proto:  void QLineF::NewQLineF(const QPointF & pt1, const QPointF & pt2);
  fn _ZN6QLineFC1ERK7QPointFS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  double QLineF::length();
  fn _ZNK6QLineF6lengthEv(qthis: *mut c_void) -> c_double;
  // proto:  void QLineF::NewQLineF(const QLine & line);
  fn _ZN6QLineFC1ERK5QLine(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QLineF::setAngle(qreal angle);
  fn _ZN6QLineF8setAngleEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  double QLineF::x2();
  fn _ZNK6QLineF2x2Ev(qthis: *mut c_void) -> c_double;
  // proto:  void QLineF::translate(const QPointF & p);
  fn _ZN6QLineF9translateERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  double QLineF::dx();
  fn _ZNK6QLineF2dxEv(qthis: *mut c_void) -> c_double;
  // proto:  void QLineF::NewQLineF();
  fn _ZN6QLineFC1Ev(qthis: *mut c_void) ;
  // proto:  QPointF QLineF::p1();
  fn _ZNK6QLineF2p1Ev(qthis: *mut c_void) -> *mut c_void;
  // proto:  QLineF QLineF::normalVector();
  fn _ZNK6QLineF12normalVectorEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QLine QLineF::toLine();
  fn _ZNK6QLineF6toLineEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QPointF QLineF::pointAt(qreal t);
  fn _ZNK6QLineF7pointAtEd(qthis: *mut c_void, arg0: c_double) -> *mut c_void;
  // proto:  QPointF QLineF::p2();
  fn _ZNK6QLineF2p2Ev(qthis: *mut c_void) -> *mut c_void;
  // proto:  double QLineF::y2();
  fn _ZNK6QLineF2y2Ev(qthis: *mut c_void) -> c_double;
  // proto:  void QLineF::NewQLineF(qreal x1, qreal y1, qreal x2, qreal y2);
  fn _ZN6QLineFC1Edddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) ;
  // proto:  double QLineF::dy();
  fn _ZNK6QLineF2dyEv(qthis: *mut c_void) -> c_double;
  // proto:  QLineF QLineF::unitVector();
  fn _ZNK6QLineF10unitVectorEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QLineF::isNull();
  fn _ZNK6QLineF6isNullEv(qthis: *mut c_void) -> int8_t;
  // proto:  double QLineF::y1();
  fn _ZNK6QLineF2y1Ev(qthis: *mut c_void) -> c_double;
  // proto:  double QLineF::angleTo(const QLineF & l);
  fn _ZNK6QLineF7angleToERKS_(qthis: *mut c_void, arg0: *mut c_void) -> c_double;
  // proto:  QLineF QLineF::translated(const QPointF & p);
  fn _ZNK6QLineF10translatedERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QLineF::setLine(qreal x1, qreal y1, qreal x2, qreal y2);
  fn _ZN6QLineF7setLineEdddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double) ;
  // proto: static QLineF QLineF::fromPolar(qreal length, qreal angle);
  fn _ZN6QLineF9fromPolarEdd(arg0: c_double, arg1: c_double) -> *mut c_void;
  // proto:  void QLineF::setP1(const QPointF & p1);
  fn _ZN6QLineF5setP1ERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  double QLineF::angle(const QLineF & l);
  fn _ZNK6QLineF5angleERKS_(qthis: *mut c_void, arg0: *mut c_void) -> c_double;
}

// body block begin
// class sizeof(QLineF)=32
pub struct QLineF {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QLineF {
  pub fn translate<T: QLineF_translate>(&mut self, value: T)  {
     value.translate(self);
    // return 1;
  }
}

pub trait QLineF_translate {
  fn translate(self, rsthis: &mut QLineF) ;
}

// proto:  void QLineF::translate(qreal dx, qreal dy);
impl<'a> /*trait*/ QLineF_translate for (f64, f64) {
  fn translate(self, rsthis: &mut QLineF)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLineF9translateEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
     unsafe {_ZN6QLineF9translateEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QLineF {
  pub fn setPoints<T: QLineF_setPoints>(&mut self, value: T)  {
     value.setPoints(self);
    // return 1;
  }
}

pub trait QLineF_setPoints {
  fn setPoints(self, rsthis: &mut QLineF) ;
}

// proto:  void QLineF::setPoints(const QPointF & p1, const QPointF & p2);
impl<'a> /*trait*/ QLineF_setPoints for (&'a  QPointF, &'a  QPointF) {
  fn setPoints(self, rsthis: &mut QLineF)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLineF9setPointsERK7QPointFS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN6QLineF9setPointsERK7QPointFS2_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QLineF {
  pub fn setP2<T: QLineF_setP2>(&mut self, value: T)  {
     value.setP2(self);
    // return 1;
  }
}

pub trait QLineF_setP2 {
  fn setP2(self, rsthis: &mut QLineF) ;
}

// proto:  void QLineF::setP2(const QPointF & p2);
impl<'a> /*trait*/ QLineF_setP2 for (&'a  QPointF) {
  fn setP2(self, rsthis: &mut QLineF)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLineF5setP2ERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QLineF5setP2ERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QLineF {
  pub fn translated<T: QLineF_translated>(&mut self, value: T) -> QLineF {
    return value.translated(self);
    // return 1;
  }
}

pub trait QLineF_translated {
  fn translated(self, rsthis: &mut QLineF) -> QLineF;
}

// proto:  QLineF QLineF::translated(qreal dx, qreal dy);
impl<'a> /*trait*/ QLineF_translated for (f64, f64) {
  fn translated(self, rsthis: &mut QLineF) -> QLineF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLineF10translatedEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let mut ret = unsafe {_ZNK6QLineF10translatedEdd(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QLineF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QLineF {
  pub fn setLength<T: QLineF_setLength>(&mut self, value: T)  {
     value.setLength(self);
    // return 1;
  }
}

pub trait QLineF_setLength {
  fn setLength(self, rsthis: &mut QLineF) ;
}

// proto:  void QLineF::setLength(qreal len);
impl<'a> /*trait*/ QLineF_setLength for (f64) {
  fn setLength(self, rsthis: &mut QLineF)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLineF9setLengthEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN6QLineF9setLengthEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QLineF {
  pub fn x1<T: QLineF_x1>(&mut self, value: T) -> f64 {
    return value.x1(self);
    // return 1;
  }
}

pub trait QLineF_x1 {
  fn x1(self, rsthis: &mut QLineF) -> f64;
}

// proto:  double QLineF::x1();
impl<'a> /*trait*/ QLineF_x1 for () {
  fn x1(self, rsthis: &mut QLineF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLineF2x1Ev()};
    let mut ret = unsafe {_ZNK6QLineF2x1Ev(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QLineF {
  pub fn angle<T: QLineF_angle>(&mut self, value: T) -> f64 {
    return value.angle(self);
    // return 1;
  }
}

pub trait QLineF_angle {
  fn angle(self, rsthis: &mut QLineF) -> f64;
}

// proto:  double QLineF::angle();
impl<'a> /*trait*/ QLineF_angle for () {
  fn angle(self, rsthis: &mut QLineF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLineF5angleEv()};
    let mut ret = unsafe {_ZNK6QLineF5angleEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QLineF {
  pub fn NewQLineF<T: QLineF_NewQLineF>(value: T) -> QLineF {
    let rsthis = value.NewQLineF();
    return rsthis;
    // return 1;
  }
}

pub trait QLineF_NewQLineF {
  fn NewQLineF(self) -> QLineF;
}

// proto: void QLineF::NewQLineF(const QPointF & pt1, const QPointF & pt2);
impl<'a> /*trait*/ QLineF_NewQLineF for (&'a  QPointF, &'a  QPointF) {
  fn NewQLineF(self) -> QLineF {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLineFC1ERK7QPointFS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN6QLineFC1ERK7QPointFS2_(qthis, arg0, arg1)};
    let rsthis = QLineF{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QLineF {
  pub fn length<T: QLineF_length>(&mut self, value: T) -> f64 {
    return value.length(self);
    // return 1;
  }
}

pub trait QLineF_length {
  fn length(self, rsthis: &mut QLineF) -> f64;
}

// proto:  double QLineF::length();
impl<'a> /*trait*/ QLineF_length for () {
  fn length(self, rsthis: &mut QLineF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLineF6lengthEv()};
    let mut ret = unsafe {_ZNK6QLineF6lengthEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

// proto: void QLineF::NewQLineF(const QLine & line);
impl<'a> /*trait*/ QLineF_NewQLineF for (&'a  QLine) {
  fn NewQLineF(self) -> QLineF {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLineFC1ERK5QLine()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN6QLineFC1ERK5QLine(qthis, arg0)};
    let rsthis = QLineF{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QLineF {
  pub fn setAngle<T: QLineF_setAngle>(&mut self, value: T)  {
     value.setAngle(self);
    // return 1;
  }
}

pub trait QLineF_setAngle {
  fn setAngle(self, rsthis: &mut QLineF) ;
}

// proto:  void QLineF::setAngle(qreal angle);
impl<'a> /*trait*/ QLineF_setAngle for (f64) {
  fn setAngle(self, rsthis: &mut QLineF)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLineF8setAngleEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN6QLineF8setAngleEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QLineF {
  pub fn x2<T: QLineF_x2>(&mut self, value: T) -> f64 {
    return value.x2(self);
    // return 1;
  }
}

pub trait QLineF_x2 {
  fn x2(self, rsthis: &mut QLineF) -> f64;
}

// proto:  double QLineF::x2();
impl<'a> /*trait*/ QLineF_x2 for () {
  fn x2(self, rsthis: &mut QLineF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLineF2x2Ev()};
    let mut ret = unsafe {_ZNK6QLineF2x2Ev(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

// proto:  void QLineF::translate(const QPointF & p);
impl<'a> /*trait*/ QLineF_translate for (&'a  QPointF) {
  fn translate(self, rsthis: &mut QLineF)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLineF9translateERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QLineF9translateERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QLineF {
  pub fn dx<T: QLineF_dx>(&mut self, value: T) -> f64 {
    return value.dx(self);
    // return 1;
  }
}

pub trait QLineF_dx {
  fn dx(self, rsthis: &mut QLineF) -> f64;
}

// proto:  double QLineF::dx();
impl<'a> /*trait*/ QLineF_dx for () {
  fn dx(self, rsthis: &mut QLineF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLineF2dxEv()};
    let mut ret = unsafe {_ZNK6QLineF2dxEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

// proto: void QLineF::NewQLineF();
impl<'a> /*trait*/ QLineF_NewQLineF for () {
  fn NewQLineF(self) -> QLineF {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLineFC1Ev()};
    unsafe {_ZN6QLineFC1Ev(qthis)};
    let rsthis = QLineF{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QLineF {
  pub fn p1<T: QLineF_p1>(&mut self, value: T) -> QPointF {
    return value.p1(self);
    // return 1;
  }
}

pub trait QLineF_p1 {
  fn p1(self, rsthis: &mut QLineF) -> QPointF;
}

// proto:  QPointF QLineF::p1();
impl<'a> /*trait*/ QLineF_p1 for () {
  fn p1(self, rsthis: &mut QLineF) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLineF2p1Ev()};
    let mut ret = unsafe {_ZNK6QLineF2p1Ev(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QLineF {
  pub fn normalVector<T: QLineF_normalVector>(&mut self, value: T) -> QLineF {
    return value.normalVector(self);
    // return 1;
  }
}

pub trait QLineF_normalVector {
  fn normalVector(self, rsthis: &mut QLineF) -> QLineF;
}

// proto:  QLineF QLineF::normalVector();
impl<'a> /*trait*/ QLineF_normalVector for () {
  fn normalVector(self, rsthis: &mut QLineF) -> QLineF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLineF12normalVectorEv()};
    let mut ret = unsafe {_ZNK6QLineF12normalVectorEv(rsthis.qclsinst)};
    let mut ret1 = QLineF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QLineF {
  pub fn toLine<T: QLineF_toLine>(&mut self, value: T) -> QLine {
    return value.toLine(self);
    // return 1;
  }
}

pub trait QLineF_toLine {
  fn toLine(self, rsthis: &mut QLineF) -> QLine;
}

// proto:  QLine QLineF::toLine();
impl<'a> /*trait*/ QLineF_toLine for () {
  fn toLine(self, rsthis: &mut QLineF) -> QLine {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLineF6toLineEv()};
    let mut ret = unsafe {_ZNK6QLineF6toLineEv(rsthis.qclsinst)};
    let mut ret1 = QLine{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QLineF {
  pub fn pointAt<T: QLineF_pointAt>(&mut self, value: T) -> QPointF {
    return value.pointAt(self);
    // return 1;
  }
}

pub trait QLineF_pointAt {
  fn pointAt(self, rsthis: &mut QLineF) -> QPointF;
}

// proto:  QPointF QLineF::pointAt(qreal t);
impl<'a> /*trait*/ QLineF_pointAt for (f64) {
  fn pointAt(self, rsthis: &mut QLineF) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLineF7pointAtEd()};
    let arg0 = self  as c_double;
    let mut ret = unsafe {_ZNK6QLineF7pointAtEd(rsthis.qclsinst, arg0)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QLineF {
  pub fn p2<T: QLineF_p2>(&mut self, value: T) -> QPointF {
    return value.p2(self);
    // return 1;
  }
}

pub trait QLineF_p2 {
  fn p2(self, rsthis: &mut QLineF) -> QPointF;
}

// proto:  QPointF QLineF::p2();
impl<'a> /*trait*/ QLineF_p2 for () {
  fn p2(self, rsthis: &mut QLineF) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLineF2p2Ev()};
    let mut ret = unsafe {_ZNK6QLineF2p2Ev(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QLineF {
  pub fn y2<T: QLineF_y2>(&mut self, value: T) -> f64 {
    return value.y2(self);
    // return 1;
  }
}

pub trait QLineF_y2 {
  fn y2(self, rsthis: &mut QLineF) -> f64;
}

// proto:  double QLineF::y2();
impl<'a> /*trait*/ QLineF_y2 for () {
  fn y2(self, rsthis: &mut QLineF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLineF2y2Ev()};
    let mut ret = unsafe {_ZNK6QLineF2y2Ev(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

// proto: void QLineF::NewQLineF(qreal x1, qreal y1, qreal x2, qreal y2);
impl<'a> /*trait*/ QLineF_NewQLineF for (f64, f64, f64, f64) {
  fn NewQLineF(self) -> QLineF {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLineFC1Edddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    unsafe {_ZN6QLineFC1Edddd(qthis, arg0, arg1, arg2, arg3)};
    let rsthis = QLineF{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QLineF {
  pub fn dy<T: QLineF_dy>(&mut self, value: T) -> f64 {
    return value.dy(self);
    // return 1;
  }
}

pub trait QLineF_dy {
  fn dy(self, rsthis: &mut QLineF) -> f64;
}

// proto:  double QLineF::dy();
impl<'a> /*trait*/ QLineF_dy for () {
  fn dy(self, rsthis: &mut QLineF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLineF2dyEv()};
    let mut ret = unsafe {_ZNK6QLineF2dyEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QLineF {
  pub fn unitVector<T: QLineF_unitVector>(&mut self, value: T) -> QLineF {
    return value.unitVector(self);
    // return 1;
  }
}

pub trait QLineF_unitVector {
  fn unitVector(self, rsthis: &mut QLineF) -> QLineF;
}

// proto:  QLineF QLineF::unitVector();
impl<'a> /*trait*/ QLineF_unitVector for () {
  fn unitVector(self, rsthis: &mut QLineF) -> QLineF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLineF10unitVectorEv()};
    let mut ret = unsafe {_ZNK6QLineF10unitVectorEv(rsthis.qclsinst)};
    let mut ret1 = QLineF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QLineF {
  pub fn isNull<T: QLineF_isNull>(&mut self, value: T) -> i8 {
    return value.isNull(self);
    // return 1;
  }
}

pub trait QLineF_isNull {
  fn isNull(self, rsthis: &mut QLineF) -> i8;
}

// proto:  bool QLineF::isNull();
impl<'a> /*trait*/ QLineF_isNull for () {
  fn isNull(self, rsthis: &mut QLineF) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLineF6isNullEv()};
    let mut ret = unsafe {_ZNK6QLineF6isNullEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QLineF {
  pub fn y1<T: QLineF_y1>(&mut self, value: T) -> f64 {
    return value.y1(self);
    // return 1;
  }
}

pub trait QLineF_y1 {
  fn y1(self, rsthis: &mut QLineF) -> f64;
}

// proto:  double QLineF::y1();
impl<'a> /*trait*/ QLineF_y1 for () {
  fn y1(self, rsthis: &mut QLineF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLineF2y1Ev()};
    let mut ret = unsafe {_ZNK6QLineF2y1Ev(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QLineF {
  pub fn angleTo<T: QLineF_angleTo>(&mut self, value: T) -> f64 {
    return value.angleTo(self);
    // return 1;
  }
}

pub trait QLineF_angleTo {
  fn angleTo(self, rsthis: &mut QLineF) -> f64;
}

// proto:  double QLineF::angleTo(const QLineF & l);
impl<'a> /*trait*/ QLineF_angleTo for (&'a  QLineF) {
  fn angleTo(self, rsthis: &mut QLineF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLineF7angleToERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK6QLineF7angleToERKS_(rsthis.qclsinst, arg0)};
    return ret as f64;
    // return 1;
  }
}

// proto:  QLineF QLineF::translated(const QPointF & p);
impl<'a> /*trait*/ QLineF_translated for (&'a  QPointF) {
  fn translated(self, rsthis: &mut QLineF) -> QLineF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLineF10translatedERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK6QLineF10translatedERK7QPointF(rsthis.qclsinst, arg0)};
    let mut ret1 = QLineF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QLineF {
  pub fn setLine<T: QLineF_setLine>(&mut self, value: T)  {
     value.setLine(self);
    // return 1;
  }
}

pub trait QLineF_setLine {
  fn setLine(self, rsthis: &mut QLineF) ;
}

// proto:  void QLineF::setLine(qreal x1, qreal y1, qreal x2, qreal y2);
impl<'a> /*trait*/ QLineF_setLine for (f64, f64, f64, f64) {
  fn setLine(self, rsthis: &mut QLineF)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLineF7setLineEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
     unsafe {_ZN6QLineF7setLineEdddd(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

impl /*struct*/ QLineF {
  pub fn fromPolar<T: QLineF_fromPolar>(&mut self, value: T) -> QLineF {
    return value.fromPolar(self);
    // return 1;
  }
}

pub trait QLineF_fromPolar {
  fn fromPolar(self, rsthis: &mut QLineF) -> QLineF;
}

// proto: static QLineF QLineF::fromPolar(qreal length, qreal angle);
impl<'a> /*trait*/ QLineF_fromPolar for (f64, f64) {
  fn fromPolar(self, rsthis: &mut QLineF) -> QLineF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLineF9fromPolarEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let mut ret = unsafe {_ZN6QLineF9fromPolarEdd(arg0, arg1)};
    let mut ret1 = QLineF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QLineF {
  pub fn setP1<T: QLineF_setP1>(&mut self, value: T)  {
     value.setP1(self);
    // return 1;
  }
}

pub trait QLineF_setP1 {
  fn setP1(self, rsthis: &mut QLineF) ;
}

// proto:  void QLineF::setP1(const QPointF & p1);
impl<'a> /*trait*/ QLineF_setP1 for (&'a  QPointF) {
  fn setP1(self, rsthis: &mut QLineF)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QLineF5setP1ERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN6QLineF5setP1ERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  double QLineF::angle(const QLineF & l);
impl<'a> /*trait*/ QLineF_angle for (&'a  QLineF) {
  fn angle(self, rsthis: &mut QLineF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QLineF5angleERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK6QLineF5angleERKS_(rsthis.qclsinst, arg0)};
    return ret as f64;
    // return 1;
  }
}

