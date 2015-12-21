// auto generated, do not modify.
// created: Mon Dec 21 22:54:38 2015
// src-file: /QtCore/qeasingcurve.h
// dst-file: /src/core/qeasingcurve.rs
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
use super::qpoint::QPointF; // 773
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  void QEasingCurve::QEasingCurve(const QEasingCurve & other);
  fn _ZN12QEasingCurveC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QEasingCurve::~QEasingCurve();
  fn _ZN12QEasingCurveD0Ev(qthis: *mut c_void);
  // proto:  EasingFunction QEasingCurve::customType();
  fn _ZNK12QEasingCurve10customTypeEv(qthis: *mut c_void);
  // proto:  qreal QEasingCurve::overshoot();
  fn _ZNK12QEasingCurve9overshootEv(qthis: *mut c_void) -> c_double;
  // proto:  void QEasingCurve::setPeriod(qreal period);
  fn _ZN12QEasingCurve9setPeriodEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QEasingCurve::addTCBSegment(const QPointF & nextPoint, qreal t, qreal c, qreal b);
  fn _ZN12QEasingCurve13addTCBSegmentERK7QPointFddd(qthis: *mut c_void, arg0: *mut c_void, arg1: c_double, arg2: c_double, arg3: c_double);
  // proto:  void QEasingCurve::addCubicBezierSegment(const QPointF & c1, const QPointF & c2, const QPointF & endPoint);
  fn _ZN12QEasingCurve21addCubicBezierSegmentERK7QPointFS2_S2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  qreal QEasingCurve::period();
  fn _ZNK12QEasingCurve6periodEv(qthis: *mut c_void) -> c_double;
  // proto:  qreal QEasingCurve::valueForProgress(qreal progress);
  fn _ZNK12QEasingCurve16valueForProgressEd(qthis: *mut c_void, arg0: c_double) -> c_double;
  // proto:  void QEasingCurve::setAmplitude(qreal amplitude);
  fn _ZN12QEasingCurve12setAmplitudeEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QEasingCurve::swap(QEasingCurve & other);
  fn _ZN12QEasingCurve4swapERS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QEasingCurve::setOvershoot(qreal overshoot);
  fn _ZN12QEasingCurve12setOvershootEd(qthis: *mut c_void, arg0: c_double);
  // proto:  QVector<QPointF> QEasingCurve::toCubicSpline();
  fn _ZNK12QEasingCurve13toCubicSplineEv(qthis: *mut c_void);
  // proto:  qreal QEasingCurve::amplitude();
  fn _ZNK12QEasingCurve9amplitudeEv(qthis: *mut c_void) -> c_double;
} // <= ext block end

// body block begin =>
// class sizeof(QEasingCurve)=8
pub struct QEasingCurve {
  pub qclsinst: *mut c_void,
}

  // proto:  void QEasingCurve::QEasingCurve(const QEasingCurve & other);
impl /*struct*/ QEasingCurve {
  pub fn NewQEasingCurve<T: QEasingCurve_NewQEasingCurve>(value: T) -> QEasingCurve {
    let rsthis = value.NewQEasingCurve();
    return rsthis;
    // return 1;
  }
}

pub trait QEasingCurve_NewQEasingCurve {
  fn NewQEasingCurve(self) -> QEasingCurve;
}

  // proto:  void QEasingCurve::QEasingCurve(const QEasingCurve & other);
impl<'a> /*trait*/ QEasingCurve_NewQEasingCurve for (QEasingCurve) {
  fn NewQEasingCurve(self) -> QEasingCurve {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QEasingCurveC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QEasingCurveC1ERKS_(qthis, arg0)};
    let rsthis = QEasingCurve{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QEasingCurve::~QEasingCurve();
impl /*struct*/ QEasingCurve {
  pub fn FreeQEasingCurve<RetType, T: QEasingCurve_FreeQEasingCurve<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQEasingCurve(self);
    // return 1;
  }
}

pub trait QEasingCurve_FreeQEasingCurve<RetType> {
  fn FreeQEasingCurve(self , rsthis: &mut QEasingCurve) -> RetType;
}

  // proto:  void QEasingCurve::~QEasingCurve();
impl<'a> /*trait*/ QEasingCurve_FreeQEasingCurve<()> for () {
  fn FreeQEasingCurve(self , rsthis: &mut QEasingCurve) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QEasingCurveD0Ev()};
     unsafe {_ZN12QEasingCurveD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  EasingFunction QEasingCurve::customType();
impl /*struct*/ QEasingCurve {
  pub fn customType<RetType, T: QEasingCurve_customType<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.customType(self);
    // return 1;
  }
}

pub trait QEasingCurve_customType<RetType> {
  fn customType(self , rsthis: &mut QEasingCurve) -> RetType;
}

  // proto:  EasingFunction QEasingCurve::customType();
impl<'a> /*trait*/ QEasingCurve_customType<()> for () {
  fn customType(self , rsthis: &mut QEasingCurve) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QEasingCurve10customTypeEv()};
     unsafe {_ZNK12QEasingCurve10customTypeEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  qreal QEasingCurve::overshoot();
impl /*struct*/ QEasingCurve {
  pub fn overshoot<RetType, T: QEasingCurve_overshoot<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.overshoot(self);
    // return 1;
  }
}

pub trait QEasingCurve_overshoot<RetType> {
  fn overshoot(self , rsthis: &mut QEasingCurve) -> RetType;
}

  // proto:  qreal QEasingCurve::overshoot();
impl<'a> /*trait*/ QEasingCurve_overshoot<f64> for () {
  fn overshoot(self , rsthis: &mut QEasingCurve) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QEasingCurve9overshootEv()};
    let mut ret = unsafe {_ZNK12QEasingCurve9overshootEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QEasingCurve::setPeriod(qreal period);
impl /*struct*/ QEasingCurve {
  pub fn setPeriod<RetType, T: QEasingCurve_setPeriod<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setPeriod(self);
    // return 1;
  }
}

pub trait QEasingCurve_setPeriod<RetType> {
  fn setPeriod(self , rsthis: &mut QEasingCurve) -> RetType;
}

  // proto:  void QEasingCurve::setPeriod(qreal period);
impl<'a> /*trait*/ QEasingCurve_setPeriod<()> for (f64) {
  fn setPeriod(self , rsthis: &mut QEasingCurve) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QEasingCurve9setPeriodEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN12QEasingCurve9setPeriodEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QEasingCurve::addTCBSegment(const QPointF & nextPoint, qreal t, qreal c, qreal b);
impl /*struct*/ QEasingCurve {
  pub fn addTCBSegment<RetType, T: QEasingCurve_addTCBSegment<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.addTCBSegment(self);
    // return 1;
  }
}

pub trait QEasingCurve_addTCBSegment<RetType> {
  fn addTCBSegment(self , rsthis: &mut QEasingCurve) -> RetType;
}

  // proto:  void QEasingCurve::addTCBSegment(const QPointF & nextPoint, qreal t, qreal c, qreal b);
impl<'a> /*trait*/ QEasingCurve_addTCBSegment<()> for (QPointF, f64, f64, f64) {
  fn addTCBSegment(self , rsthis: &mut QEasingCurve) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QEasingCurve13addTCBSegmentERK7QPointFddd()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
     unsafe {_ZN12QEasingCurve13addTCBSegmentERK7QPointFddd(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  void QEasingCurve::addCubicBezierSegment(const QPointF & c1, const QPointF & c2, const QPointF & endPoint);
impl /*struct*/ QEasingCurve {
  pub fn addCubicBezierSegment<RetType, T: QEasingCurve_addCubicBezierSegment<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.addCubicBezierSegment(self);
    // return 1;
  }
}

pub trait QEasingCurve_addCubicBezierSegment<RetType> {
  fn addCubicBezierSegment(self , rsthis: &mut QEasingCurve) -> RetType;
}

  // proto:  void QEasingCurve::addCubicBezierSegment(const QPointF & c1, const QPointF & c2, const QPointF & endPoint);
impl<'a> /*trait*/ QEasingCurve_addCubicBezierSegment<()> for (QPointF, QPointF, QPointF) {
  fn addCubicBezierSegment(self , rsthis: &mut QEasingCurve) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QEasingCurve21addCubicBezierSegmentERK7QPointFS2_S2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN12QEasingCurve21addCubicBezierSegmentERK7QPointFS2_S2_(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  qreal QEasingCurve::period();
impl /*struct*/ QEasingCurve {
  pub fn period<RetType, T: QEasingCurve_period<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.period(self);
    // return 1;
  }
}

pub trait QEasingCurve_period<RetType> {
  fn period(self , rsthis: &mut QEasingCurve) -> RetType;
}

  // proto:  qreal QEasingCurve::period();
impl<'a> /*trait*/ QEasingCurve_period<f64> for () {
  fn period(self , rsthis: &mut QEasingCurve) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QEasingCurve6periodEv()};
    let mut ret = unsafe {_ZNK12QEasingCurve6periodEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  qreal QEasingCurve::valueForProgress(qreal progress);
impl /*struct*/ QEasingCurve {
  pub fn valueForProgress<RetType, T: QEasingCurve_valueForProgress<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.valueForProgress(self);
    // return 1;
  }
}

pub trait QEasingCurve_valueForProgress<RetType> {
  fn valueForProgress(self , rsthis: &mut QEasingCurve) -> RetType;
}

  // proto:  qreal QEasingCurve::valueForProgress(qreal progress);
impl<'a> /*trait*/ QEasingCurve_valueForProgress<f64> for (f64) {
  fn valueForProgress(self , rsthis: &mut QEasingCurve) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QEasingCurve16valueForProgressEd()};
    let arg0 = self  as c_double;
    let mut ret = unsafe {_ZNK12QEasingCurve16valueForProgressEd(rsthis.qclsinst, arg0)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QEasingCurve::setAmplitude(qreal amplitude);
impl /*struct*/ QEasingCurve {
  pub fn setAmplitude<RetType, T: QEasingCurve_setAmplitude<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setAmplitude(self);
    // return 1;
  }
}

pub trait QEasingCurve_setAmplitude<RetType> {
  fn setAmplitude(self , rsthis: &mut QEasingCurve) -> RetType;
}

  // proto:  void QEasingCurve::setAmplitude(qreal amplitude);
impl<'a> /*trait*/ QEasingCurve_setAmplitude<()> for (f64) {
  fn setAmplitude(self , rsthis: &mut QEasingCurve) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QEasingCurve12setAmplitudeEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN12QEasingCurve12setAmplitudeEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QEasingCurve::swap(QEasingCurve & other);
impl /*struct*/ QEasingCurve {
  pub fn swap<RetType, T: QEasingCurve_swap<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QEasingCurve_swap<RetType> {
  fn swap(self , rsthis: &mut QEasingCurve) -> RetType;
}

  // proto:  void QEasingCurve::swap(QEasingCurve & other);
impl<'a> /*trait*/ QEasingCurve_swap<()> for (QEasingCurve) {
  fn swap(self , rsthis: &mut QEasingCurve) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QEasingCurve4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QEasingCurve4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QEasingCurve::setOvershoot(qreal overshoot);
impl /*struct*/ QEasingCurve {
  pub fn setOvershoot<RetType, T: QEasingCurve_setOvershoot<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setOvershoot(self);
    // return 1;
  }
}

pub trait QEasingCurve_setOvershoot<RetType> {
  fn setOvershoot(self , rsthis: &mut QEasingCurve) -> RetType;
}

  // proto:  void QEasingCurve::setOvershoot(qreal overshoot);
impl<'a> /*trait*/ QEasingCurve_setOvershoot<()> for (f64) {
  fn setOvershoot(self , rsthis: &mut QEasingCurve) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QEasingCurve12setOvershootEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN12QEasingCurve12setOvershootEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QVector<QPointF> QEasingCurve::toCubicSpline();
impl /*struct*/ QEasingCurve {
  pub fn toCubicSpline<RetType, T: QEasingCurve_toCubicSpline<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toCubicSpline(self);
    // return 1;
  }
}

pub trait QEasingCurve_toCubicSpline<RetType> {
  fn toCubicSpline(self , rsthis: &mut QEasingCurve) -> RetType;
}

  // proto:  QVector<QPointF> QEasingCurve::toCubicSpline();
impl<'a> /*trait*/ QEasingCurve_toCubicSpline<()> for () {
  fn toCubicSpline(self , rsthis: &mut QEasingCurve) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QEasingCurve13toCubicSplineEv()};
     unsafe {_ZNK12QEasingCurve13toCubicSplineEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  qreal QEasingCurve::amplitude();
impl /*struct*/ QEasingCurve {
  pub fn amplitude<RetType, T: QEasingCurve_amplitude<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.amplitude(self);
    // return 1;
  }
}

pub trait QEasingCurve_amplitude<RetType> {
  fn amplitude(self , rsthis: &mut QEasingCurve) -> RetType;
}

  // proto:  qreal QEasingCurve::amplitude();
impl<'a> /*trait*/ QEasingCurve_amplitude<f64> for () {
  fn amplitude(self , rsthis: &mut QEasingCurve) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QEasingCurve9amplitudeEv()};
    let mut ret = unsafe {_ZNK12QEasingCurve9amplitudeEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

// <= body block end

