// auto generated, do not modify.
// created: Wed Dec 23 22:29:56 2015
// src-file: /QtCore/qtimeline.h
// dst-file: /src/core/qtimeline.rs
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
use super::qobject::QObject; // 773
use std::ops::Deref;
use super::qeasingcurve::QEasingCurve; // 773
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  void QTimeLine::start();
  fn _ZN9QTimeLine5startEv(qthis: *mut c_void);
  // proto:  void QTimeLine::QTimeLine(const QTimeLine & );
  fn _ZN9QTimeLineC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  int QTimeLine::duration();
  fn _ZNK9QTimeLine8durationEv(qthis: *mut c_void) -> c_int;
  // proto:  int QTimeLine::currentFrame();
  fn _ZNK9QTimeLine12currentFrameEv(qthis: *mut c_void) -> c_int;
  // proto:  const QMetaObject * QTimeLine::metaObject();
  fn _ZNK9QTimeLine10metaObjectEv(qthis: *mut c_void);
  // proto:  void QTimeLine::stop();
  fn _ZN9QTimeLine4stopEv(qthis: *mut c_void);
  // proto:  void QTimeLine::~QTimeLine();
  fn _ZN9QTimeLineD0Ev(qthis: *mut c_void);
  // proto:  void QTimeLine::setUpdateInterval(int interval);
  fn _ZN9QTimeLine17setUpdateIntervalEi(qthis: *mut c_void, arg0: c_int);
  // proto:  QEasingCurve QTimeLine::easingCurve();
  fn _ZNK9QTimeLine11easingCurveEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int QTimeLine::loopCount();
  fn _ZNK9QTimeLine9loopCountEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTimeLine::setStartFrame(int frame);
  fn _ZN9QTimeLine13setStartFrameEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QTimeLine::QTimeLine(int duration, QObject * parent);
  fn _ZN9QTimeLineC1EiP7QObject(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void);
  // proto:  void QTimeLine::resume();
  fn _ZN9QTimeLine6resumeEv(qthis: *mut c_void);
  // proto:  void QTimeLine::setEasingCurve(const QEasingCurve & curve);
  fn _ZN9QTimeLine14setEasingCurveERK12QEasingCurve(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  int QTimeLine::startFrame();
  fn _ZNK9QTimeLine10startFrameEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTimeLine::setEndFrame(int frame);
  fn _ZN9QTimeLine11setEndFrameEi(qthis: *mut c_void, arg0: c_int);
  // proto:  int QTimeLine::updateInterval();
  fn _ZNK9QTimeLine14updateIntervalEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTimeLine::setLoopCount(int count);
  fn _ZN9QTimeLine12setLoopCountEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QTimeLine::setCurrentTime(int msec);
  fn _ZN9QTimeLine14setCurrentTimeEi(qthis: *mut c_void, arg0: c_int);
  // proto:  int QTimeLine::currentTime();
  fn _ZNK9QTimeLine11currentTimeEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTimeLine::setDuration(int duration);
  fn _ZN9QTimeLine11setDurationEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QTimeLine::toggleDirection();
  fn _ZN9QTimeLine15toggleDirectionEv(qthis: *mut c_void);
  // proto:  int QTimeLine::endFrame();
  fn _ZNK9QTimeLine8endFrameEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTimeLine::setPaused(bool paused);
  fn _ZN9QTimeLine9setPausedEb(qthis: *mut c_void, arg0: c_char);
  // proto:  int QTimeLine::frameForTime(int msec);
  fn _ZNK9QTimeLine12frameForTimeEi(qthis: *mut c_void, arg0: c_int) -> c_int;
  // proto:  void QTimeLine::setFrameRange(int startFrame, int endFrame);
  fn _ZN9QTimeLine13setFrameRangeEii(qthis: *mut c_void, arg0: c_int, arg1: c_int);
  // proto:  qreal QTimeLine::valueForTime(int msec);
  fn _ZNK9QTimeLine12valueForTimeEi(qthis: *mut c_void, arg0: c_int) -> c_double;
  // proto:  qreal QTimeLine::currentValue();
  fn _ZNK9QTimeLine12currentValueEv(qthis: *mut c_void) -> c_double;
} // <= ext block end

// body block begin =>
// class sizeof(QTimeLine)=1
pub struct QTimeLine {
  qbase: QObject,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTimeLine {
  pub fn inheritFrom(qthis: *mut c_void) -> QTimeLine {
    return QTimeLine{qbase: QObject::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QTimeLine {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QTimeLine {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  void QTimeLine::start();
impl /*struct*/ QTimeLine {
  pub fn start<RetType, T: QTimeLine_start<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.start(self);
    // return 1;
  }
}

pub trait QTimeLine_start<RetType> {
  fn start(self , rsthis: & QTimeLine) -> RetType;
}

  // proto:  void QTimeLine::start();
impl<'a> /*trait*/ QTimeLine_start<()> for () {
  fn start(self , rsthis: & QTimeLine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeLine5startEv()};
     unsafe {_ZN9QTimeLine5startEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTimeLine::QTimeLine(const QTimeLine & );
impl /*struct*/ QTimeLine {
  pub fn New<T: QTimeLine_New>(value: T) -> QTimeLine {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QTimeLine_New {
  fn New(self) -> QTimeLine;
}

  // proto:  void QTimeLine::QTimeLine(const QTimeLine & );
impl<'a> /*trait*/ QTimeLine_New for (&'a QTimeLine) {
  fn New(self) -> QTimeLine {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeLineC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN9QTimeLineC1ERKS_(qthis, arg0)};
    let rsthis = QTimeLine{/**/qbase: QObject::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QTimeLine::duration();
impl /*struct*/ QTimeLine {
  pub fn duration<RetType, T: QTimeLine_duration<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.duration(self);
    // return 1;
  }
}

pub trait QTimeLine_duration<RetType> {
  fn duration(self , rsthis: & QTimeLine) -> RetType;
}

  // proto:  int QTimeLine::duration();
impl<'a> /*trait*/ QTimeLine_duration<i32> for () {
  fn duration(self , rsthis: & QTimeLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTimeLine8durationEv()};
    let mut ret = unsafe {_ZNK9QTimeLine8durationEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int QTimeLine::currentFrame();
impl /*struct*/ QTimeLine {
  pub fn currentFrame<RetType, T: QTimeLine_currentFrame<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.currentFrame(self);
    // return 1;
  }
}

pub trait QTimeLine_currentFrame<RetType> {
  fn currentFrame(self , rsthis: & QTimeLine) -> RetType;
}

  // proto:  int QTimeLine::currentFrame();
impl<'a> /*trait*/ QTimeLine_currentFrame<i32> for () {
  fn currentFrame(self , rsthis: & QTimeLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTimeLine12currentFrameEv()};
    let mut ret = unsafe {_ZNK9QTimeLine12currentFrameEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  const QMetaObject * QTimeLine::metaObject();
impl /*struct*/ QTimeLine {
  pub fn metaObject<RetType, T: QTimeLine_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QTimeLine_metaObject<RetType> {
  fn metaObject(self , rsthis: & QTimeLine) -> RetType;
}

  // proto:  const QMetaObject * QTimeLine::metaObject();
impl<'a> /*trait*/ QTimeLine_metaObject<()> for () {
  fn metaObject(self , rsthis: & QTimeLine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTimeLine10metaObjectEv()};
     unsafe {_ZNK9QTimeLine10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTimeLine::stop();
impl /*struct*/ QTimeLine {
  pub fn stop<RetType, T: QTimeLine_stop<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.stop(self);
    // return 1;
  }
}

pub trait QTimeLine_stop<RetType> {
  fn stop(self , rsthis: & QTimeLine) -> RetType;
}

  // proto:  void QTimeLine::stop();
impl<'a> /*trait*/ QTimeLine_stop<()> for () {
  fn stop(self , rsthis: & QTimeLine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeLine4stopEv()};
     unsafe {_ZN9QTimeLine4stopEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTimeLine::~QTimeLine();
impl /*struct*/ QTimeLine {
  pub fn Free<RetType, T: QTimeLine_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QTimeLine_Free<RetType> {
  fn Free(self , rsthis: & QTimeLine) -> RetType;
}

  // proto:  void QTimeLine::~QTimeLine();
impl<'a> /*trait*/ QTimeLine_Free<()> for () {
  fn Free(self , rsthis: & QTimeLine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeLineD0Ev()};
     unsafe {_ZN9QTimeLineD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTimeLine::setUpdateInterval(int interval);
impl /*struct*/ QTimeLine {
  pub fn setUpdateInterval<RetType, T: QTimeLine_setUpdateInterval<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setUpdateInterval(self);
    // return 1;
  }
}

pub trait QTimeLine_setUpdateInterval<RetType> {
  fn setUpdateInterval(self , rsthis: & QTimeLine) -> RetType;
}

  // proto:  void QTimeLine::setUpdateInterval(int interval);
impl<'a> /*trait*/ QTimeLine_setUpdateInterval<()> for (i32) {
  fn setUpdateInterval(self , rsthis: & QTimeLine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeLine17setUpdateIntervalEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QTimeLine17setUpdateIntervalEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QEasingCurve QTimeLine::easingCurve();
impl /*struct*/ QTimeLine {
  pub fn easingCurve<RetType, T: QTimeLine_easingCurve<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.easingCurve(self);
    // return 1;
  }
}

pub trait QTimeLine_easingCurve<RetType> {
  fn easingCurve(self , rsthis: & QTimeLine) -> RetType;
}

  // proto:  QEasingCurve QTimeLine::easingCurve();
impl<'a> /*trait*/ QTimeLine_easingCurve<QEasingCurve> for () {
  fn easingCurve(self , rsthis: & QTimeLine) -> QEasingCurve {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTimeLine11easingCurveEv()};
    let mut ret = unsafe {_ZNK9QTimeLine11easingCurveEv(rsthis.qclsinst)};
    let mut ret1 = QEasingCurve::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  int QTimeLine::loopCount();
impl /*struct*/ QTimeLine {
  pub fn loopCount<RetType, T: QTimeLine_loopCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.loopCount(self);
    // return 1;
  }
}

pub trait QTimeLine_loopCount<RetType> {
  fn loopCount(self , rsthis: & QTimeLine) -> RetType;
}

  // proto:  int QTimeLine::loopCount();
impl<'a> /*trait*/ QTimeLine_loopCount<i32> for () {
  fn loopCount(self , rsthis: & QTimeLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTimeLine9loopCountEv()};
    let mut ret = unsafe {_ZNK9QTimeLine9loopCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QTimeLine::setStartFrame(int frame);
impl /*struct*/ QTimeLine {
  pub fn setStartFrame<RetType, T: QTimeLine_setStartFrame<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setStartFrame(self);
    // return 1;
  }
}

pub trait QTimeLine_setStartFrame<RetType> {
  fn setStartFrame(self , rsthis: & QTimeLine) -> RetType;
}

  // proto:  void QTimeLine::setStartFrame(int frame);
impl<'a> /*trait*/ QTimeLine_setStartFrame<()> for (i32) {
  fn setStartFrame(self , rsthis: & QTimeLine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeLine13setStartFrameEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QTimeLine13setStartFrameEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTimeLine::QTimeLine(int duration, QObject * parent);
impl<'a> /*trait*/ QTimeLine_New for (i32, &'a QObject) {
  fn New(self) -> QTimeLine {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeLineC1EiP7QObject()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN9QTimeLineC1EiP7QObject(qthis, arg0, arg1)};
    let rsthis = QTimeLine{/**/qbase: QObject::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QTimeLine::resume();
impl /*struct*/ QTimeLine {
  pub fn resume<RetType, T: QTimeLine_resume<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.resume(self);
    // return 1;
  }
}

pub trait QTimeLine_resume<RetType> {
  fn resume(self , rsthis: & QTimeLine) -> RetType;
}

  // proto:  void QTimeLine::resume();
impl<'a> /*trait*/ QTimeLine_resume<()> for () {
  fn resume(self , rsthis: & QTimeLine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeLine6resumeEv()};
     unsafe {_ZN9QTimeLine6resumeEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTimeLine::setEasingCurve(const QEasingCurve & curve);
impl /*struct*/ QTimeLine {
  pub fn setEasingCurve<RetType, T: QTimeLine_setEasingCurve<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setEasingCurve(self);
    // return 1;
  }
}

pub trait QTimeLine_setEasingCurve<RetType> {
  fn setEasingCurve(self , rsthis: & QTimeLine) -> RetType;
}

  // proto:  void QTimeLine::setEasingCurve(const QEasingCurve & curve);
impl<'a> /*trait*/ QTimeLine_setEasingCurve<()> for (&'a QEasingCurve) {
  fn setEasingCurve(self , rsthis: & QTimeLine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeLine14setEasingCurveERK12QEasingCurve()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN9QTimeLine14setEasingCurveERK12QEasingCurve(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QTimeLine::startFrame();
impl /*struct*/ QTimeLine {
  pub fn startFrame<RetType, T: QTimeLine_startFrame<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.startFrame(self);
    // return 1;
  }
}

pub trait QTimeLine_startFrame<RetType> {
  fn startFrame(self , rsthis: & QTimeLine) -> RetType;
}

  // proto:  int QTimeLine::startFrame();
impl<'a> /*trait*/ QTimeLine_startFrame<i32> for () {
  fn startFrame(self , rsthis: & QTimeLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTimeLine10startFrameEv()};
    let mut ret = unsafe {_ZNK9QTimeLine10startFrameEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QTimeLine::setEndFrame(int frame);
impl /*struct*/ QTimeLine {
  pub fn setEndFrame<RetType, T: QTimeLine_setEndFrame<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setEndFrame(self);
    // return 1;
  }
}

pub trait QTimeLine_setEndFrame<RetType> {
  fn setEndFrame(self , rsthis: & QTimeLine) -> RetType;
}

  // proto:  void QTimeLine::setEndFrame(int frame);
impl<'a> /*trait*/ QTimeLine_setEndFrame<()> for (i32) {
  fn setEndFrame(self , rsthis: & QTimeLine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeLine11setEndFrameEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QTimeLine11setEndFrameEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QTimeLine::updateInterval();
impl /*struct*/ QTimeLine {
  pub fn updateInterval<RetType, T: QTimeLine_updateInterval<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.updateInterval(self);
    // return 1;
  }
}

pub trait QTimeLine_updateInterval<RetType> {
  fn updateInterval(self , rsthis: & QTimeLine) -> RetType;
}

  // proto:  int QTimeLine::updateInterval();
impl<'a> /*trait*/ QTimeLine_updateInterval<i32> for () {
  fn updateInterval(self , rsthis: & QTimeLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTimeLine14updateIntervalEv()};
    let mut ret = unsafe {_ZNK9QTimeLine14updateIntervalEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QTimeLine::setLoopCount(int count);
impl /*struct*/ QTimeLine {
  pub fn setLoopCount<RetType, T: QTimeLine_setLoopCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setLoopCount(self);
    // return 1;
  }
}

pub trait QTimeLine_setLoopCount<RetType> {
  fn setLoopCount(self , rsthis: & QTimeLine) -> RetType;
}

  // proto:  void QTimeLine::setLoopCount(int count);
impl<'a> /*trait*/ QTimeLine_setLoopCount<()> for (i32) {
  fn setLoopCount(self , rsthis: & QTimeLine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeLine12setLoopCountEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QTimeLine12setLoopCountEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTimeLine::setCurrentTime(int msec);
impl /*struct*/ QTimeLine {
  pub fn setCurrentTime<RetType, T: QTimeLine_setCurrentTime<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCurrentTime(self);
    // return 1;
  }
}

pub trait QTimeLine_setCurrentTime<RetType> {
  fn setCurrentTime(self , rsthis: & QTimeLine) -> RetType;
}

  // proto:  void QTimeLine::setCurrentTime(int msec);
impl<'a> /*trait*/ QTimeLine_setCurrentTime<()> for (i32) {
  fn setCurrentTime(self , rsthis: & QTimeLine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeLine14setCurrentTimeEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QTimeLine14setCurrentTimeEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QTimeLine::currentTime();
impl /*struct*/ QTimeLine {
  pub fn currentTime<RetType, T: QTimeLine_currentTime<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.currentTime(self);
    // return 1;
  }
}

pub trait QTimeLine_currentTime<RetType> {
  fn currentTime(self , rsthis: & QTimeLine) -> RetType;
}

  // proto:  int QTimeLine::currentTime();
impl<'a> /*trait*/ QTimeLine_currentTime<i32> for () {
  fn currentTime(self , rsthis: & QTimeLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTimeLine11currentTimeEv()};
    let mut ret = unsafe {_ZNK9QTimeLine11currentTimeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QTimeLine::setDuration(int duration);
impl /*struct*/ QTimeLine {
  pub fn setDuration<RetType, T: QTimeLine_setDuration<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDuration(self);
    // return 1;
  }
}

pub trait QTimeLine_setDuration<RetType> {
  fn setDuration(self , rsthis: & QTimeLine) -> RetType;
}

  // proto:  void QTimeLine::setDuration(int duration);
impl<'a> /*trait*/ QTimeLine_setDuration<()> for (i32) {
  fn setDuration(self , rsthis: & QTimeLine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeLine11setDurationEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN9QTimeLine11setDurationEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTimeLine::toggleDirection();
impl /*struct*/ QTimeLine {
  pub fn toggleDirection<RetType, T: QTimeLine_toggleDirection<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toggleDirection(self);
    // return 1;
  }
}

pub trait QTimeLine_toggleDirection<RetType> {
  fn toggleDirection(self , rsthis: & QTimeLine) -> RetType;
}

  // proto:  void QTimeLine::toggleDirection();
impl<'a> /*trait*/ QTimeLine_toggleDirection<()> for () {
  fn toggleDirection(self , rsthis: & QTimeLine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeLine15toggleDirectionEv()};
     unsafe {_ZN9QTimeLine15toggleDirectionEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QTimeLine::endFrame();
impl /*struct*/ QTimeLine {
  pub fn endFrame<RetType, T: QTimeLine_endFrame<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.endFrame(self);
    // return 1;
  }
}

pub trait QTimeLine_endFrame<RetType> {
  fn endFrame(self , rsthis: & QTimeLine) -> RetType;
}

  // proto:  int QTimeLine::endFrame();
impl<'a> /*trait*/ QTimeLine_endFrame<i32> for () {
  fn endFrame(self , rsthis: & QTimeLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTimeLine8endFrameEv()};
    let mut ret = unsafe {_ZNK9QTimeLine8endFrameEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QTimeLine::setPaused(bool paused);
impl /*struct*/ QTimeLine {
  pub fn setPaused<RetType, T: QTimeLine_setPaused<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPaused(self);
    // return 1;
  }
}

pub trait QTimeLine_setPaused<RetType> {
  fn setPaused(self , rsthis: & QTimeLine) -> RetType;
}

  // proto:  void QTimeLine::setPaused(bool paused);
impl<'a> /*trait*/ QTimeLine_setPaused<()> for (i8) {
  fn setPaused(self , rsthis: & QTimeLine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeLine9setPausedEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN9QTimeLine9setPausedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QTimeLine::frameForTime(int msec);
impl /*struct*/ QTimeLine {
  pub fn frameForTime<RetType, T: QTimeLine_frameForTime<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.frameForTime(self);
    // return 1;
  }
}

pub trait QTimeLine_frameForTime<RetType> {
  fn frameForTime(self , rsthis: & QTimeLine) -> RetType;
}

  // proto:  int QTimeLine::frameForTime(int msec);
impl<'a> /*trait*/ QTimeLine_frameForTime<i32> for (i32) {
  fn frameForTime(self , rsthis: & QTimeLine) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTimeLine12frameForTimeEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK9QTimeLine12frameForTimeEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QTimeLine::setFrameRange(int startFrame, int endFrame);
impl /*struct*/ QTimeLine {
  pub fn setFrameRange<RetType, T: QTimeLine_setFrameRange<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFrameRange(self);
    // return 1;
  }
}

pub trait QTimeLine_setFrameRange<RetType> {
  fn setFrameRange(self , rsthis: & QTimeLine) -> RetType;
}

  // proto:  void QTimeLine::setFrameRange(int startFrame, int endFrame);
impl<'a> /*trait*/ QTimeLine_setFrameRange<()> for (i32, i32) {
  fn setFrameRange(self , rsthis: & QTimeLine) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN9QTimeLine13setFrameRangeEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN9QTimeLine13setFrameRangeEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  qreal QTimeLine::valueForTime(int msec);
impl /*struct*/ QTimeLine {
  pub fn valueForTime<RetType, T: QTimeLine_valueForTime<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.valueForTime(self);
    // return 1;
  }
}

pub trait QTimeLine_valueForTime<RetType> {
  fn valueForTime(self , rsthis: & QTimeLine) -> RetType;
}

  // proto:  qreal QTimeLine::valueForTime(int msec);
impl<'a> /*trait*/ QTimeLine_valueForTime<f64> for (i32) {
  fn valueForTime(self , rsthis: & QTimeLine) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTimeLine12valueForTimeEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK9QTimeLine12valueForTimeEi(rsthis.qclsinst, arg0)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  qreal QTimeLine::currentValue();
impl /*struct*/ QTimeLine {
  pub fn currentValue<RetType, T: QTimeLine_currentValue<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.currentValue(self);
    // return 1;
  }
}

pub trait QTimeLine_currentValue<RetType> {
  fn currentValue(self , rsthis: & QTimeLine) -> RetType;
}

  // proto:  qreal QTimeLine::currentValue();
impl<'a> /*trait*/ QTimeLine_currentValue<f64> for () {
  fn currentValue(self , rsthis: & QTimeLine) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK9QTimeLine12currentValueEv()};
    let mut ret = unsafe {_ZNK9QTimeLine12currentValueEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

// <= body block end

