// auto generated, do not modify.
// created: Thu Jan 28 22:38:45 2016
// src-file: /QtWidgets/qgesture.h
// dst-file: /src/widgets/qgesture.rs
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
// use super::qgesture::QGesture; // 773
use std::ops::Deref;
use super::super::core::qobjectdefs::*; // 771
use super::super::core::qobject::*; // 771
use super::super::core::qpoint::*; // 771
use super::super::core::qcoreevent::*; // 771
use super::qwidget::*; // 773
// use super::qlist::*; // 775
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QSwipeGesture_Class_Size() -> c_int;
  // proto:  void QSwipeGesture::~QSwipeGesture();
  fn C_ZN13QSwipeGestureD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  const QMetaObject * QSwipeGesture::metaObject();
  fn C_ZNK13QSwipeGesture10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QSwipeGesture::setSwipeAngle(qreal value);
  fn C_ZN13QSwipeGesture13setSwipeAngleEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  void QSwipeGesture::QSwipeGesture(QObject * parent);
  fn C_ZN13QSwipeGestureC2EP7QObject(arg0: *mut c_void) -> u64;
  // proto:  qreal QSwipeGesture::swipeAngle();
  fn C_ZNK13QSwipeGesture10swipeAngleEv(qthis: u64 /* *mut c_void*/) -> c_double;
  fn QGesture_Class_Size() -> c_int;
  // proto:  QPointF QGesture::hotSpot();
  fn C_ZNK8QGesture7hotSpotEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QGesture::hasHotSpot();
  fn C_ZNK8QGesture10hasHotSpotEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QGesture::unsetHotSpot();
  fn C_ZN8QGesture12unsetHotSpotEv(qthis: u64 /* *mut c_void*/);
  // proto:  const QMetaObject * QGesture::metaObject();
  fn C_ZNK8QGesture10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QGesture::QGesture(QObject * parent);
  fn C_ZN8QGestureC2EP7QObject(arg0: *mut c_void) -> u64;
  // proto:  void QGesture::setHotSpot(const QPointF & value);
  fn C_ZN8QGesture10setHotSpotERK7QPointF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QGesture::~QGesture();
  fn C_ZN8QGestureD2Ev(qthis: u64 /* *mut c_void*/);
  fn QGestureEvent_Class_Size() -> c_int;
  // proto:  bool QGestureEvent::isAccepted(QGesture * );
  fn C_ZNK13QGestureEvent10isAcceptedEP8QGesture(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_char;
  // proto:  QWidget * QGestureEvent::widget();
  fn C_ZNK13QGestureEvent6widgetEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QGestureEvent::ignore(QGesture * );
  fn C_ZN13QGestureEvent6ignoreEP8QGesture(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QGestureEvent::accept(QGesture * );
  fn C_ZN13QGestureEvent6acceptEP8QGesture(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QList<QGesture *> QGestureEvent::activeGestures();
  fn C_ZNK13QGestureEvent14activeGesturesEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QList<QGesture *> QGestureEvent::gestures();
  fn C_ZNK13QGestureEvent8gesturesEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QGestureEvent::setAccepted(QGesture * , bool );
  fn C_ZN13QGestureEvent11setAcceptedEP8QGestureb(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_char);
  // proto:  void QGestureEvent::setWidget(QWidget * widget);
  fn C_ZN13QGestureEvent9setWidgetEP7QWidget(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QGestureEvent::~QGestureEvent();
  fn C_ZN13QGestureEventD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QList<QGesture *> QGestureEvent::canceledGestures();
  fn C_ZNK13QGestureEvent16canceledGesturesEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QPointF QGestureEvent::mapToGraphicsScene(const QPointF & gesturePoint);
  fn C_ZNK13QGestureEvent18mapToGraphicsSceneERK7QPointF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  fn QPanGesture_Class_Size() -> c_int;
  // proto:  QPointF QPanGesture::offset();
  fn C_ZNK11QPanGesture6offsetEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QPointF QPanGesture::delta();
  fn C_ZNK11QPanGesture5deltaEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QPanGesture::setOffset(const QPointF & value);
  fn C_ZN11QPanGesture9setOffsetERK7QPointF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  qreal QPanGesture::acceleration();
  fn C_ZNK11QPanGesture12accelerationEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  void QPanGesture::~QPanGesture();
  fn C_ZN11QPanGestureD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QPanGesture::QPanGesture(QObject * parent);
  fn C_ZN11QPanGestureC2EP7QObject(arg0: *mut c_void) -> u64;
  // proto:  const QMetaObject * QPanGesture::metaObject();
  fn C_ZNK11QPanGesture10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QPanGesture::setAcceleration(qreal value);
  fn C_ZN11QPanGesture15setAccelerationEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  QPointF QPanGesture::lastOffset();
  fn C_ZNK11QPanGesture10lastOffsetEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QPanGesture::setLastOffset(const QPointF & value);
  fn C_ZN11QPanGesture13setLastOffsetERK7QPointF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  fn QTapAndHoldGesture_Class_Size() -> c_int;
  // proto:  void QTapAndHoldGesture::QTapAndHoldGesture(QObject * parent);
  fn C_ZN18QTapAndHoldGestureC2EP7QObject(arg0: *mut c_void) -> u64;
  // proto:  void QTapAndHoldGesture::~QTapAndHoldGesture();
  fn C_ZN18QTapAndHoldGestureD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  QPointF QTapAndHoldGesture::position();
  fn C_ZNK18QTapAndHoldGesture8positionEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto: static void QTapAndHoldGesture::setTimeout(int msecs);
  fn C_ZN18QTapAndHoldGesture10setTimeoutEi(arg0: c_int);
  // proto: static int QTapAndHoldGesture::timeout();
  fn C_ZN18QTapAndHoldGesture7timeoutEv() -> c_int;
  // proto:  const QMetaObject * QTapAndHoldGesture::metaObject();
  fn C_ZNK18QTapAndHoldGesture10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QTapAndHoldGesture::setPosition(const QPointF & pos);
  fn C_ZN18QTapAndHoldGesture11setPositionERK7QPointF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  fn QTapGesture_Class_Size() -> c_int;
  // proto:  QPointF QTapGesture::position();
  fn C_ZNK11QTapGesture8positionEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QTapGesture::setPosition(const QPointF & pos);
  fn C_ZN11QTapGesture11setPositionERK7QPointF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QTapGesture::QTapGesture(QObject * parent);
  fn C_ZN11QTapGestureC2EP7QObject(arg0: *mut c_void) -> u64;
  // proto:  const QMetaObject * QTapGesture::metaObject();
  fn C_ZNK11QTapGesture10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QTapGesture::~QTapGesture();
  fn C_ZN11QTapGestureD2Ev(qthis: u64 /* *mut c_void*/);
  fn QPinchGesture_Class_Size() -> c_int;
  // proto:  void QPinchGesture::setRotationAngle(qreal value);
  fn C_ZN13QPinchGesture16setRotationAngleEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  qreal QPinchGesture::lastScaleFactor();
  fn C_ZNK13QPinchGesture15lastScaleFactorEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  qreal QPinchGesture::lastRotationAngle();
  fn C_ZNK13QPinchGesture17lastRotationAngleEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  QPointF QPinchGesture::startCenterPoint();
  fn C_ZNK13QPinchGesture16startCenterPointEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  qreal QPinchGesture::rotationAngle();
  fn C_ZNK13QPinchGesture13rotationAngleEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  QPointF QPinchGesture::lastCenterPoint();
  fn C_ZNK13QPinchGesture15lastCenterPointEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QPinchGesture::QPinchGesture(QObject * parent);
  fn C_ZN13QPinchGestureC2EP7QObject(arg0: *mut c_void) -> u64;
  // proto:  qreal QPinchGesture::totalScaleFactor();
  fn C_ZNK13QPinchGesture16totalScaleFactorEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  void QPinchGesture::setTotalScaleFactor(qreal value);
  fn C_ZN13QPinchGesture19setTotalScaleFactorEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  qreal QPinchGesture::totalRotationAngle();
  fn C_ZNK13QPinchGesture18totalRotationAngleEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  void QPinchGesture::setLastScaleFactor(qreal value);
  fn C_ZN13QPinchGesture18setLastScaleFactorEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  void QPinchGesture::setLastCenterPoint(const QPointF & value);
  fn C_ZN13QPinchGesture18setLastCenterPointERK7QPointF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  const QMetaObject * QPinchGesture::metaObject();
  fn C_ZNK13QPinchGesture10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QPinchGesture::setLastRotationAngle(qreal value);
  fn C_ZN13QPinchGesture20setLastRotationAngleEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  QPointF QPinchGesture::centerPoint();
  fn C_ZNK13QPinchGesture11centerPointEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QPinchGesture::setCenterPoint(const QPointF & value);
  fn C_ZN13QPinchGesture14setCenterPointERK7QPointF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QPinchGesture::setTotalRotationAngle(qreal value);
  fn C_ZN13QPinchGesture21setTotalRotationAngleEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  void QPinchGesture::setScaleFactor(qreal value);
  fn C_ZN13QPinchGesture14setScaleFactorEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  void QPinchGesture::~QPinchGesture();
  fn C_ZN13QPinchGestureD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QPinchGesture::setStartCenterPoint(const QPointF & value);
  fn C_ZN13QPinchGesture19setStartCenterPointERK7QPointF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  qreal QPinchGesture::scaleFactor();
  fn C_ZNK13QPinchGesture11scaleFactorEv(qthis: u64 /* *mut c_void*/) -> c_double;
} // <= ext block end

// body block begin =>
// class sizeof(QSwipeGesture)=1
#[derive(Default)]
pub struct QSwipeGesture {
  qbase: QGesture,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QGesture)=1
#[derive(Default)]
pub struct QGesture {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QGestureEvent)=1
#[derive(Default)]
pub struct QGestureEvent {
  qbase: QEvent,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QPanGesture)=1
#[derive(Default)]
pub struct QPanGesture {
  qbase: QGesture,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QTapAndHoldGesture)=1
#[derive(Default)]
pub struct QTapAndHoldGesture {
  qbase: QGesture,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QTapGesture)=1
#[derive(Default)]
pub struct QTapGesture {
  qbase: QGesture,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QPinchGesture)=1
#[derive(Default)]
pub struct QPinchGesture {
  qbase: QGesture,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QSwipeGesture {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QSwipeGesture {
    return QSwipeGesture{qbase: QGesture::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QSwipeGesture {
  type Target = QGesture;

  fn deref(&self) -> &QGesture {
    return & self.qbase;
  }
}
impl AsRef<QGesture> for QSwipeGesture {
  fn as_ref(& self) -> & QGesture {
    return & self.qbase;
  }
}
  // proto:  void QSwipeGesture::~QSwipeGesture();
impl /*struct*/ QSwipeGesture {
  pub fn free<RetType, T: QSwipeGesture_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QSwipeGesture_free<RetType> {
  fn free(self , rsthis: & QSwipeGesture) -> RetType;
}

  // proto:  void QSwipeGesture::~QSwipeGesture();
impl<'a> /*trait*/ QSwipeGesture_free<()> for () {
  fn free(self , rsthis: & QSwipeGesture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSwipeGestureD2Ev()};
     unsafe {C_ZN13QSwipeGestureD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QSwipeGesture::metaObject();
impl /*struct*/ QSwipeGesture {
  pub fn metaObject<RetType, T: QSwipeGesture_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QSwipeGesture_metaObject<RetType> {
  fn metaObject(self , rsthis: & QSwipeGesture) -> RetType;
}

  // proto:  const QMetaObject * QSwipeGesture::metaObject();
impl<'a> /*trait*/ QSwipeGesture_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QSwipeGesture) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QSwipeGesture10metaObjectEv()};
    let mut ret = unsafe {C_ZNK13QSwipeGesture10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QSwipeGesture::setSwipeAngle(qreal value);
impl /*struct*/ QSwipeGesture {
  pub fn setSwipeAngle<RetType, T: QSwipeGesture_setSwipeAngle<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSwipeAngle(self);
    // return 1;
  }
}

pub trait QSwipeGesture_setSwipeAngle<RetType> {
  fn setSwipeAngle(self , rsthis: & QSwipeGesture) -> RetType;
}

  // proto:  void QSwipeGesture::setSwipeAngle(qreal value);
impl<'a> /*trait*/ QSwipeGesture_setSwipeAngle<()> for (f64) {
  fn setSwipeAngle(self , rsthis: & QSwipeGesture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSwipeGesture13setSwipeAngleEd()};
    let arg0 = self  as c_double;
     unsafe {C_ZN13QSwipeGesture13setSwipeAngleEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QSwipeGesture::QSwipeGesture(QObject * parent);
impl /*struct*/ QSwipeGesture {
  pub fn new<T: QSwipeGesture_new>(value: T) -> QSwipeGesture {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QSwipeGesture_new {
  fn new(self) -> QSwipeGesture;
}

  // proto:  void QSwipeGesture::QSwipeGesture(QObject * parent);
impl<'a> /*trait*/ QSwipeGesture_new for (&'a QObject) {
  fn new(self) -> QSwipeGesture {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QSwipeGestureC2EP7QObject()};
    let ctysz: c_int = unsafe{QSwipeGesture_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN13QSwipeGestureC2EP7QObject(arg0)};
    let rsthis = QSwipeGesture{qbase: QGesture::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  qreal QSwipeGesture::swipeAngle();
impl /*struct*/ QSwipeGesture {
  pub fn swipeAngle<RetType, T: QSwipeGesture_swipeAngle<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.swipeAngle(self);
    // return 1;
  }
}

pub trait QSwipeGesture_swipeAngle<RetType> {
  fn swipeAngle(self , rsthis: & QSwipeGesture) -> RetType;
}

  // proto:  qreal QSwipeGesture::swipeAngle();
impl<'a> /*trait*/ QSwipeGesture_swipeAngle<f64> for () {
  fn swipeAngle(self , rsthis: & QSwipeGesture) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QSwipeGesture10swipeAngleEv()};
    let mut ret = unsafe {C_ZNK13QSwipeGesture10swipeAngleEv(rsthis.qclsinst)};
    return ret as f64; // 1
    // return 1;
  }
}

impl /*struct*/ QGesture {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QGesture {
    return QGesture{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QGesture {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QGesture {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  QPointF QGesture::hotSpot();
impl /*struct*/ QGesture {
  pub fn hotSpot<RetType, T: QGesture_hotSpot<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hotSpot(self);
    // return 1;
  }
}

pub trait QGesture_hotSpot<RetType> {
  fn hotSpot(self , rsthis: & QGesture) -> RetType;
}

  // proto:  QPointF QGesture::hotSpot();
impl<'a> /*trait*/ QGesture_hotSpot<QPointF> for () {
  fn hotSpot(self , rsthis: & QGesture) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QGesture7hotSpotEv()};
    let mut ret = unsafe {C_ZNK8QGesture7hotSpotEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QGesture::hasHotSpot();
impl /*struct*/ QGesture {
  pub fn hasHotSpot<RetType, T: QGesture_hasHotSpot<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hasHotSpot(self);
    // return 1;
  }
}

pub trait QGesture_hasHotSpot<RetType> {
  fn hasHotSpot(self , rsthis: & QGesture) -> RetType;
}

  // proto:  bool QGesture::hasHotSpot();
impl<'a> /*trait*/ QGesture_hasHotSpot<i8> for () {
  fn hasHotSpot(self , rsthis: & QGesture) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QGesture10hasHotSpotEv()};
    let mut ret = unsafe {C_ZNK8QGesture10hasHotSpotEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QGesture::unsetHotSpot();
impl /*struct*/ QGesture {
  pub fn unsetHotSpot<RetType, T: QGesture_unsetHotSpot<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.unsetHotSpot(self);
    // return 1;
  }
}

pub trait QGesture_unsetHotSpot<RetType> {
  fn unsetHotSpot(self , rsthis: & QGesture) -> RetType;
}

  // proto:  void QGesture::unsetHotSpot();
impl<'a> /*trait*/ QGesture_unsetHotSpot<()> for () {
  fn unsetHotSpot(self , rsthis: & QGesture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QGesture12unsetHotSpotEv()};
     unsafe {C_ZN8QGesture12unsetHotSpotEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QGesture::metaObject();
impl /*struct*/ QGesture {
  pub fn metaObject<RetType, T: QGesture_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QGesture_metaObject<RetType> {
  fn metaObject(self , rsthis: & QGesture) -> RetType;
}

  // proto:  const QMetaObject * QGesture::metaObject();
impl<'a> /*trait*/ QGesture_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QGesture) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK8QGesture10metaObjectEv()};
    let mut ret = unsafe {C_ZNK8QGesture10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGesture::QGesture(QObject * parent);
impl /*struct*/ QGesture {
  pub fn new<T: QGesture_new>(value: T) -> QGesture {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QGesture_new {
  fn new(self) -> QGesture;
}

  // proto:  void QGesture::QGesture(QObject * parent);
impl<'a> /*trait*/ QGesture_new for (&'a QObject) {
  fn new(self) -> QGesture {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QGestureC2EP7QObject()};
    let ctysz: c_int = unsafe{QGesture_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN8QGestureC2EP7QObject(arg0)};
    let rsthis = QGesture{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QGesture::setHotSpot(const QPointF & value);
impl /*struct*/ QGesture {
  pub fn setHotSpot<RetType, T: QGesture_setHotSpot<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setHotSpot(self);
    // return 1;
  }
}

pub trait QGesture_setHotSpot<RetType> {
  fn setHotSpot(self , rsthis: & QGesture) -> RetType;
}

  // proto:  void QGesture::setHotSpot(const QPointF & value);
impl<'a> /*trait*/ QGesture_setHotSpot<()> for (&'a QPointF) {
  fn setHotSpot(self , rsthis: & QGesture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QGesture10setHotSpotERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN8QGesture10setHotSpotERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGesture::~QGesture();
impl /*struct*/ QGesture {
  pub fn free<RetType, T: QGesture_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QGesture_free<RetType> {
  fn free(self , rsthis: & QGesture) -> RetType;
}

  // proto:  void QGesture::~QGesture();
impl<'a> /*trait*/ QGesture_free<()> for () {
  fn free(self , rsthis: & QGesture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN8QGestureD2Ev()};
     unsafe {C_ZN8QGestureD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGestureEvent {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QGestureEvent {
    return QGestureEvent{qbase: QEvent::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QGestureEvent {
  type Target = QEvent;

  fn deref(&self) -> &QEvent {
    return & self.qbase;
  }
}
impl AsRef<QEvent> for QGestureEvent {
  fn as_ref(& self) -> & QEvent {
    return & self.qbase;
  }
}
  // proto:  bool QGestureEvent::isAccepted(QGesture * );
impl /*struct*/ QGestureEvent {
  pub fn isAccepted<RetType, T: QGestureEvent_isAccepted<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isAccepted(self);
    // return 1;
  }
}

pub trait QGestureEvent_isAccepted<RetType> {
  fn isAccepted(self , rsthis: & QGestureEvent) -> RetType;
}

  // proto:  bool QGestureEvent::isAccepted(QGesture * );
impl<'a> /*trait*/ QGestureEvent_isAccepted<i8> for (&'a QGesture) {
  fn isAccepted(self , rsthis: & QGestureEvent) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGestureEvent10isAcceptedEP8QGesture()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK13QGestureEvent10isAcceptedEP8QGesture(rsthis.qclsinst, arg0)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  QWidget * QGestureEvent::widget();
impl /*struct*/ QGestureEvent {
  pub fn widget<RetType, T: QGestureEvent_widget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.widget(self);
    // return 1;
  }
}

pub trait QGestureEvent_widget<RetType> {
  fn widget(self , rsthis: & QGestureEvent) -> RetType;
}

  // proto:  QWidget * QGestureEvent::widget();
impl<'a> /*trait*/ QGestureEvent_widget<QWidget> for () {
  fn widget(self , rsthis: & QGestureEvent) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGestureEvent6widgetEv()};
    let mut ret = unsafe {C_ZNK13QGestureEvent6widgetEv(rsthis.qclsinst)};
    let mut ret1 = QWidget::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGestureEvent::ignore(QGesture * );
impl /*struct*/ QGestureEvent {
  pub fn ignore<RetType, T: QGestureEvent_ignore<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.ignore(self);
    // return 1;
  }
}

pub trait QGestureEvent_ignore<RetType> {
  fn ignore(self , rsthis: & QGestureEvent) -> RetType;
}

  // proto:  void QGestureEvent::ignore(QGesture * );
impl<'a> /*trait*/ QGestureEvent_ignore<()> for (&'a QGesture) {
  fn ignore(self , rsthis: & QGestureEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGestureEvent6ignoreEP8QGesture()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN13QGestureEvent6ignoreEP8QGesture(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGestureEvent::accept(QGesture * );
impl /*struct*/ QGestureEvent {
  pub fn accept<RetType, T: QGestureEvent_accept<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.accept(self);
    // return 1;
  }
}

pub trait QGestureEvent_accept<RetType> {
  fn accept(self , rsthis: & QGestureEvent) -> RetType;
}

  // proto:  void QGestureEvent::accept(QGesture * );
impl<'a> /*trait*/ QGestureEvent_accept<()> for (&'a QGesture) {
  fn accept(self , rsthis: & QGestureEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGestureEvent6acceptEP8QGesture()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN13QGestureEvent6acceptEP8QGesture(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QList<QGesture *> QGestureEvent::activeGestures();
impl /*struct*/ QGestureEvent {
  pub fn activeGestures<RetType, T: QGestureEvent_activeGestures<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.activeGestures(self);
    // return 1;
  }
}

pub trait QGestureEvent_activeGestures<RetType> {
  fn activeGestures(self , rsthis: & QGestureEvent) -> RetType;
}

  // proto:  QList<QGesture *> QGestureEvent::activeGestures();
impl<'a> /*trait*/ QGestureEvent_activeGestures<u64> for () {
  fn activeGestures(self , rsthis: & QGestureEvent) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGestureEvent14activeGesturesEv()};
    let mut ret = unsafe {C_ZNK13QGestureEvent14activeGesturesEv(rsthis.qclsinst)};
    return ret as u64; // 5
    // return 1;
  }
}

  // proto:  QList<QGesture *> QGestureEvent::gestures();
impl /*struct*/ QGestureEvent {
  pub fn gestures<RetType, T: QGestureEvent_gestures<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.gestures(self);
    // return 1;
  }
}

pub trait QGestureEvent_gestures<RetType> {
  fn gestures(self , rsthis: & QGestureEvent) -> RetType;
}

  // proto:  QList<QGesture *> QGestureEvent::gestures();
impl<'a> /*trait*/ QGestureEvent_gestures<u64> for () {
  fn gestures(self , rsthis: & QGestureEvent) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGestureEvent8gesturesEv()};
    let mut ret = unsafe {C_ZNK13QGestureEvent8gesturesEv(rsthis.qclsinst)};
    return ret as u64; // 5
    // return 1;
  }
}

  // proto:  void QGestureEvent::setAccepted(QGesture * , bool );
impl /*struct*/ QGestureEvent {
  pub fn setAccepted<RetType, T: QGestureEvent_setAccepted<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setAccepted(self);
    // return 1;
  }
}

pub trait QGestureEvent_setAccepted<RetType> {
  fn setAccepted(self , rsthis: & QGestureEvent) -> RetType;
}

  // proto:  void QGestureEvent::setAccepted(QGesture * , bool );
impl<'a> /*trait*/ QGestureEvent_setAccepted<()> for (&'a QGesture, i8) {
  fn setAccepted(self , rsthis: & QGestureEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGestureEvent11setAcceptedEP8QGestureb()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_char;
     unsafe {C_ZN13QGestureEvent11setAcceptedEP8QGestureb(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QGestureEvent::setWidget(QWidget * widget);
impl /*struct*/ QGestureEvent {
  pub fn setWidget<RetType, T: QGestureEvent_setWidget<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setWidget(self);
    // return 1;
  }
}

pub trait QGestureEvent_setWidget<RetType> {
  fn setWidget(self , rsthis: & QGestureEvent) -> RetType;
}

  // proto:  void QGestureEvent::setWidget(QWidget * widget);
impl<'a> /*trait*/ QGestureEvent_setWidget<()> for (&'a QWidget) {
  fn setWidget(self , rsthis: & QGestureEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGestureEvent9setWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN13QGestureEvent9setWidgetEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGestureEvent::~QGestureEvent();
impl /*struct*/ QGestureEvent {
  pub fn free<RetType, T: QGestureEvent_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QGestureEvent_free<RetType> {
  fn free(self , rsthis: & QGestureEvent) -> RetType;
}

  // proto:  void QGestureEvent::~QGestureEvent();
impl<'a> /*trait*/ QGestureEvent_free<()> for () {
  fn free(self , rsthis: & QGestureEvent) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QGestureEventD2Ev()};
     unsafe {C_ZN13QGestureEventD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QList<QGesture *> QGestureEvent::canceledGestures();
impl /*struct*/ QGestureEvent {
  pub fn canceledGestures<RetType, T: QGestureEvent_canceledGestures<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.canceledGestures(self);
    // return 1;
  }
}

pub trait QGestureEvent_canceledGestures<RetType> {
  fn canceledGestures(self , rsthis: & QGestureEvent) -> RetType;
}

  // proto:  QList<QGesture *> QGestureEvent::canceledGestures();
impl<'a> /*trait*/ QGestureEvent_canceledGestures<u64> for () {
  fn canceledGestures(self , rsthis: & QGestureEvent) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGestureEvent16canceledGesturesEv()};
    let mut ret = unsafe {C_ZNK13QGestureEvent16canceledGesturesEv(rsthis.qclsinst)};
    return ret as u64; // 5
    // return 1;
  }
}

  // proto:  QPointF QGestureEvent::mapToGraphicsScene(const QPointF & gesturePoint);
impl /*struct*/ QGestureEvent {
  pub fn mapToGraphicsScene<RetType, T: QGestureEvent_mapToGraphicsScene<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mapToGraphicsScene(self);
    // return 1;
  }
}

pub trait QGestureEvent_mapToGraphicsScene<RetType> {
  fn mapToGraphicsScene(self , rsthis: & QGestureEvent) -> RetType;
}

  // proto:  QPointF QGestureEvent::mapToGraphicsScene(const QPointF & gesturePoint);
impl<'a> /*trait*/ QGestureEvent_mapToGraphicsScene<QPointF> for (&'a QPointF) {
  fn mapToGraphicsScene(self , rsthis: & QGestureEvent) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QGestureEvent18mapToGraphicsSceneERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK13QGestureEvent18mapToGraphicsSceneERK7QPointF(rsthis.qclsinst, arg0)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QPanGesture {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QPanGesture {
    return QPanGesture{qbase: QGesture::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QPanGesture {
  type Target = QGesture;

  fn deref(&self) -> &QGesture {
    return & self.qbase;
  }
}
impl AsRef<QGesture> for QPanGesture {
  fn as_ref(& self) -> & QGesture {
    return & self.qbase;
  }
}
  // proto:  QPointF QPanGesture::offset();
impl /*struct*/ QPanGesture {
  pub fn offset<RetType, T: QPanGesture_offset<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.offset(self);
    // return 1;
  }
}

pub trait QPanGesture_offset<RetType> {
  fn offset(self , rsthis: & QPanGesture) -> RetType;
}

  // proto:  QPointF QPanGesture::offset();
impl<'a> /*trait*/ QPanGesture_offset<QPointF> for () {
  fn offset(self , rsthis: & QPanGesture) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QPanGesture6offsetEv()};
    let mut ret = unsafe {C_ZNK11QPanGesture6offsetEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QPointF QPanGesture::delta();
impl /*struct*/ QPanGesture {
  pub fn delta<RetType, T: QPanGesture_delta<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.delta(self);
    // return 1;
  }
}

pub trait QPanGesture_delta<RetType> {
  fn delta(self , rsthis: & QPanGesture) -> RetType;
}

  // proto:  QPointF QPanGesture::delta();
impl<'a> /*trait*/ QPanGesture_delta<QPointF> for () {
  fn delta(self , rsthis: & QPanGesture) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QPanGesture5deltaEv()};
    let mut ret = unsafe {C_ZNK11QPanGesture5deltaEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPanGesture::setOffset(const QPointF & value);
impl /*struct*/ QPanGesture {
  pub fn setOffset<RetType, T: QPanGesture_setOffset<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setOffset(self);
    // return 1;
  }
}

pub trait QPanGesture_setOffset<RetType> {
  fn setOffset(self , rsthis: & QPanGesture) -> RetType;
}

  // proto:  void QPanGesture::setOffset(const QPointF & value);
impl<'a> /*trait*/ QPanGesture_setOffset<()> for (&'a QPointF) {
  fn setOffset(self , rsthis: & QPanGesture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QPanGesture9setOffsetERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN11QPanGesture9setOffsetERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  qreal QPanGesture::acceleration();
impl /*struct*/ QPanGesture {
  pub fn acceleration<RetType, T: QPanGesture_acceleration<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.acceleration(self);
    // return 1;
  }
}

pub trait QPanGesture_acceleration<RetType> {
  fn acceleration(self , rsthis: & QPanGesture) -> RetType;
}

  // proto:  qreal QPanGesture::acceleration();
impl<'a> /*trait*/ QPanGesture_acceleration<f64> for () {
  fn acceleration(self , rsthis: & QPanGesture) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QPanGesture12accelerationEv()};
    let mut ret = unsafe {C_ZNK11QPanGesture12accelerationEv(rsthis.qclsinst)};
    return ret as f64; // 1
    // return 1;
  }
}

  // proto:  void QPanGesture::~QPanGesture();
impl /*struct*/ QPanGesture {
  pub fn free<RetType, T: QPanGesture_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QPanGesture_free<RetType> {
  fn free(self , rsthis: & QPanGesture) -> RetType;
}

  // proto:  void QPanGesture::~QPanGesture();
impl<'a> /*trait*/ QPanGesture_free<()> for () {
  fn free(self , rsthis: & QPanGesture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QPanGestureD2Ev()};
     unsafe {C_ZN11QPanGestureD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QPanGesture::QPanGesture(QObject * parent);
impl /*struct*/ QPanGesture {
  pub fn new<T: QPanGesture_new>(value: T) -> QPanGesture {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QPanGesture_new {
  fn new(self) -> QPanGesture;
}

  // proto:  void QPanGesture::QPanGesture(QObject * parent);
impl<'a> /*trait*/ QPanGesture_new for (&'a QObject) {
  fn new(self) -> QPanGesture {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QPanGestureC2EP7QObject()};
    let ctysz: c_int = unsafe{QPanGesture_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN11QPanGestureC2EP7QObject(arg0)};
    let rsthis = QPanGesture{qbase: QGesture::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QMetaObject * QPanGesture::metaObject();
impl /*struct*/ QPanGesture {
  pub fn metaObject<RetType, T: QPanGesture_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QPanGesture_metaObject<RetType> {
  fn metaObject(self , rsthis: & QPanGesture) -> RetType;
}

  // proto:  const QMetaObject * QPanGesture::metaObject();
impl<'a> /*trait*/ QPanGesture_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QPanGesture) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QPanGesture10metaObjectEv()};
    let mut ret = unsafe {C_ZNK11QPanGesture10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPanGesture::setAcceleration(qreal value);
impl /*struct*/ QPanGesture {
  pub fn setAcceleration<RetType, T: QPanGesture_setAcceleration<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setAcceleration(self);
    // return 1;
  }
}

pub trait QPanGesture_setAcceleration<RetType> {
  fn setAcceleration(self , rsthis: & QPanGesture) -> RetType;
}

  // proto:  void QPanGesture::setAcceleration(qreal value);
impl<'a> /*trait*/ QPanGesture_setAcceleration<()> for (f64) {
  fn setAcceleration(self , rsthis: & QPanGesture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QPanGesture15setAccelerationEd()};
    let arg0 = self  as c_double;
     unsafe {C_ZN11QPanGesture15setAccelerationEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QPointF QPanGesture::lastOffset();
impl /*struct*/ QPanGesture {
  pub fn lastOffset<RetType, T: QPanGesture_lastOffset<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.lastOffset(self);
    // return 1;
  }
}

pub trait QPanGesture_lastOffset<RetType> {
  fn lastOffset(self , rsthis: & QPanGesture) -> RetType;
}

  // proto:  QPointF QPanGesture::lastOffset();
impl<'a> /*trait*/ QPanGesture_lastOffset<QPointF> for () {
  fn lastOffset(self , rsthis: & QPanGesture) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QPanGesture10lastOffsetEv()};
    let mut ret = unsafe {C_ZNK11QPanGesture10lastOffsetEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPanGesture::setLastOffset(const QPointF & value);
impl /*struct*/ QPanGesture {
  pub fn setLastOffset<RetType, T: QPanGesture_setLastOffset<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setLastOffset(self);
    // return 1;
  }
}

pub trait QPanGesture_setLastOffset<RetType> {
  fn setLastOffset(self , rsthis: & QPanGesture) -> RetType;
}

  // proto:  void QPanGesture::setLastOffset(const QPointF & value);
impl<'a> /*trait*/ QPanGesture_setLastOffset<()> for (&'a QPointF) {
  fn setLastOffset(self , rsthis: & QPanGesture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QPanGesture13setLastOffsetERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN11QPanGesture13setLastOffsetERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTapAndHoldGesture {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QTapAndHoldGesture {
    return QTapAndHoldGesture{qbase: QGesture::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QTapAndHoldGesture {
  type Target = QGesture;

  fn deref(&self) -> &QGesture {
    return & self.qbase;
  }
}
impl AsRef<QGesture> for QTapAndHoldGesture {
  fn as_ref(& self) -> & QGesture {
    return & self.qbase;
  }
}
  // proto:  void QTapAndHoldGesture::QTapAndHoldGesture(QObject * parent);
impl /*struct*/ QTapAndHoldGesture {
  pub fn new<T: QTapAndHoldGesture_new>(value: T) -> QTapAndHoldGesture {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QTapAndHoldGesture_new {
  fn new(self) -> QTapAndHoldGesture;
}

  // proto:  void QTapAndHoldGesture::QTapAndHoldGesture(QObject * parent);
impl<'a> /*trait*/ QTapAndHoldGesture_new for (&'a QObject) {
  fn new(self) -> QTapAndHoldGesture {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QTapAndHoldGestureC2EP7QObject()};
    let ctysz: c_int = unsafe{QTapAndHoldGesture_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN18QTapAndHoldGestureC2EP7QObject(arg0)};
    let rsthis = QTapAndHoldGesture{qbase: QGesture::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QTapAndHoldGesture::~QTapAndHoldGesture();
impl /*struct*/ QTapAndHoldGesture {
  pub fn free<RetType, T: QTapAndHoldGesture_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QTapAndHoldGesture_free<RetType> {
  fn free(self , rsthis: & QTapAndHoldGesture) -> RetType;
}

  // proto:  void QTapAndHoldGesture::~QTapAndHoldGesture();
impl<'a> /*trait*/ QTapAndHoldGesture_free<()> for () {
  fn free(self , rsthis: & QTapAndHoldGesture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QTapAndHoldGestureD2Ev()};
     unsafe {C_ZN18QTapAndHoldGestureD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QPointF QTapAndHoldGesture::position();
impl /*struct*/ QTapAndHoldGesture {
  pub fn position<RetType, T: QTapAndHoldGesture_position<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.position(self);
    // return 1;
  }
}

pub trait QTapAndHoldGesture_position<RetType> {
  fn position(self , rsthis: & QTapAndHoldGesture) -> RetType;
}

  // proto:  QPointF QTapAndHoldGesture::position();
impl<'a> /*trait*/ QTapAndHoldGesture_position<QPointF> for () {
  fn position(self , rsthis: & QTapAndHoldGesture) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QTapAndHoldGesture8positionEv()};
    let mut ret = unsafe {C_ZNK18QTapAndHoldGesture8positionEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto: static void QTapAndHoldGesture::setTimeout(int msecs);
impl /*struct*/ QTapAndHoldGesture {
  pub fn setTimeout_s<RetType, T: QTapAndHoldGesture_setTimeout_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.setTimeout_s();
    // return 1;
  }
}

pub trait QTapAndHoldGesture_setTimeout_s<RetType> {
  fn setTimeout_s(self ) -> RetType;
}

  // proto: static void QTapAndHoldGesture::setTimeout(int msecs);
impl<'a> /*trait*/ QTapAndHoldGesture_setTimeout_s<()> for (i32) {
  fn setTimeout_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QTapAndHoldGesture10setTimeoutEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN18QTapAndHoldGesture10setTimeoutEi(arg0)};
    // return 1;
  }
}

  // proto: static int QTapAndHoldGesture::timeout();
impl /*struct*/ QTapAndHoldGesture {
  pub fn timeout_s<RetType, T: QTapAndHoldGesture_timeout_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.timeout_s();
    // return 1;
  }
}

pub trait QTapAndHoldGesture_timeout_s<RetType> {
  fn timeout_s(self ) -> RetType;
}

  // proto: static int QTapAndHoldGesture::timeout();
impl<'a> /*trait*/ QTapAndHoldGesture_timeout_s<i32> for () {
  fn timeout_s(self ) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QTapAndHoldGesture7timeoutEv()};
    let mut ret = unsafe {C_ZN18QTapAndHoldGesture7timeoutEv()};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  const QMetaObject * QTapAndHoldGesture::metaObject();
impl /*struct*/ QTapAndHoldGesture {
  pub fn metaObject<RetType, T: QTapAndHoldGesture_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QTapAndHoldGesture_metaObject<RetType> {
  fn metaObject(self , rsthis: & QTapAndHoldGesture) -> RetType;
}

  // proto:  const QMetaObject * QTapAndHoldGesture::metaObject();
impl<'a> /*trait*/ QTapAndHoldGesture_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QTapAndHoldGesture) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QTapAndHoldGesture10metaObjectEv()};
    let mut ret = unsafe {C_ZNK18QTapAndHoldGesture10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTapAndHoldGesture::setPosition(const QPointF & pos);
impl /*struct*/ QTapAndHoldGesture {
  pub fn setPosition<RetType, T: QTapAndHoldGesture_setPosition<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPosition(self);
    // return 1;
  }
}

pub trait QTapAndHoldGesture_setPosition<RetType> {
  fn setPosition(self , rsthis: & QTapAndHoldGesture) -> RetType;
}

  // proto:  void QTapAndHoldGesture::setPosition(const QPointF & pos);
impl<'a> /*trait*/ QTapAndHoldGesture_setPosition<()> for (&'a QPointF) {
  fn setPosition(self , rsthis: & QTapAndHoldGesture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QTapAndHoldGesture11setPositionERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN18QTapAndHoldGesture11setPositionERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QTapGesture {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QTapGesture {
    return QTapGesture{qbase: QGesture::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QTapGesture {
  type Target = QGesture;

  fn deref(&self) -> &QGesture {
    return & self.qbase;
  }
}
impl AsRef<QGesture> for QTapGesture {
  fn as_ref(& self) -> & QGesture {
    return & self.qbase;
  }
}
  // proto:  QPointF QTapGesture::position();
impl /*struct*/ QTapGesture {
  pub fn position<RetType, T: QTapGesture_position<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.position(self);
    // return 1;
  }
}

pub trait QTapGesture_position<RetType> {
  fn position(self , rsthis: & QTapGesture) -> RetType;
}

  // proto:  QPointF QTapGesture::position();
impl<'a> /*trait*/ QTapGesture_position<QPointF> for () {
  fn position(self , rsthis: & QTapGesture) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTapGesture8positionEv()};
    let mut ret = unsafe {C_ZNK11QTapGesture8positionEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTapGesture::setPosition(const QPointF & pos);
impl /*struct*/ QTapGesture {
  pub fn setPosition<RetType, T: QTapGesture_setPosition<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPosition(self);
    // return 1;
  }
}

pub trait QTapGesture_setPosition<RetType> {
  fn setPosition(self , rsthis: & QTapGesture) -> RetType;
}

  // proto:  void QTapGesture::setPosition(const QPointF & pos);
impl<'a> /*trait*/ QTapGesture_setPosition<()> for (&'a QPointF) {
  fn setPosition(self , rsthis: & QTapGesture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTapGesture11setPositionERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN11QTapGesture11setPositionERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTapGesture::QTapGesture(QObject * parent);
impl /*struct*/ QTapGesture {
  pub fn new<T: QTapGesture_new>(value: T) -> QTapGesture {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QTapGesture_new {
  fn new(self) -> QTapGesture;
}

  // proto:  void QTapGesture::QTapGesture(QObject * parent);
impl<'a> /*trait*/ QTapGesture_new for (&'a QObject) {
  fn new(self) -> QTapGesture {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTapGestureC2EP7QObject()};
    let ctysz: c_int = unsafe{QTapGesture_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN11QTapGestureC2EP7QObject(arg0)};
    let rsthis = QTapGesture{qbase: QGesture::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QMetaObject * QTapGesture::metaObject();
impl /*struct*/ QTapGesture {
  pub fn metaObject<RetType, T: QTapGesture_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QTapGesture_metaObject<RetType> {
  fn metaObject(self , rsthis: & QTapGesture) -> RetType;
}

  // proto:  const QMetaObject * QTapGesture::metaObject();
impl<'a> /*trait*/ QTapGesture_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QTapGesture) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QTapGesture10metaObjectEv()};
    let mut ret = unsafe {C_ZNK11QTapGesture10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTapGesture::~QTapGesture();
impl /*struct*/ QTapGesture {
  pub fn free<RetType, T: QTapGesture_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QTapGesture_free<RetType> {
  fn free(self , rsthis: & QTapGesture) -> RetType;
}

  // proto:  void QTapGesture::~QTapGesture();
impl<'a> /*trait*/ QTapGesture_free<()> for () {
  fn free(self , rsthis: & QTapGesture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QTapGestureD2Ev()};
     unsafe {C_ZN11QTapGestureD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QPinchGesture {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QPinchGesture {
    return QPinchGesture{qbase: QGesture::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QPinchGesture {
  type Target = QGesture;

  fn deref(&self) -> &QGesture {
    return & self.qbase;
  }
}
impl AsRef<QGesture> for QPinchGesture {
  fn as_ref(& self) -> & QGesture {
    return & self.qbase;
  }
}
  // proto:  void QPinchGesture::setRotationAngle(qreal value);
impl /*struct*/ QPinchGesture {
  pub fn setRotationAngle<RetType, T: QPinchGesture_setRotationAngle<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setRotationAngle(self);
    // return 1;
  }
}

pub trait QPinchGesture_setRotationAngle<RetType> {
  fn setRotationAngle(self , rsthis: & QPinchGesture) -> RetType;
}

  // proto:  void QPinchGesture::setRotationAngle(qreal value);
impl<'a> /*trait*/ QPinchGesture_setRotationAngle<()> for (f64) {
  fn setRotationAngle(self , rsthis: & QPinchGesture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QPinchGesture16setRotationAngleEd()};
    let arg0 = self  as c_double;
     unsafe {C_ZN13QPinchGesture16setRotationAngleEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  qreal QPinchGesture::lastScaleFactor();
impl /*struct*/ QPinchGesture {
  pub fn lastScaleFactor<RetType, T: QPinchGesture_lastScaleFactor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.lastScaleFactor(self);
    // return 1;
  }
}

pub trait QPinchGesture_lastScaleFactor<RetType> {
  fn lastScaleFactor(self , rsthis: & QPinchGesture) -> RetType;
}

  // proto:  qreal QPinchGesture::lastScaleFactor();
impl<'a> /*trait*/ QPinchGesture_lastScaleFactor<f64> for () {
  fn lastScaleFactor(self , rsthis: & QPinchGesture) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QPinchGesture15lastScaleFactorEv()};
    let mut ret = unsafe {C_ZNK13QPinchGesture15lastScaleFactorEv(rsthis.qclsinst)};
    return ret as f64; // 1
    // return 1;
  }
}

  // proto:  qreal QPinchGesture::lastRotationAngle();
impl /*struct*/ QPinchGesture {
  pub fn lastRotationAngle<RetType, T: QPinchGesture_lastRotationAngle<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.lastRotationAngle(self);
    // return 1;
  }
}

pub trait QPinchGesture_lastRotationAngle<RetType> {
  fn lastRotationAngle(self , rsthis: & QPinchGesture) -> RetType;
}

  // proto:  qreal QPinchGesture::lastRotationAngle();
impl<'a> /*trait*/ QPinchGesture_lastRotationAngle<f64> for () {
  fn lastRotationAngle(self , rsthis: & QPinchGesture) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QPinchGesture17lastRotationAngleEv()};
    let mut ret = unsafe {C_ZNK13QPinchGesture17lastRotationAngleEv(rsthis.qclsinst)};
    return ret as f64; // 1
    // return 1;
  }
}

  // proto:  QPointF QPinchGesture::startCenterPoint();
impl /*struct*/ QPinchGesture {
  pub fn startCenterPoint<RetType, T: QPinchGesture_startCenterPoint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.startCenterPoint(self);
    // return 1;
  }
}

pub trait QPinchGesture_startCenterPoint<RetType> {
  fn startCenterPoint(self , rsthis: & QPinchGesture) -> RetType;
}

  // proto:  QPointF QPinchGesture::startCenterPoint();
impl<'a> /*trait*/ QPinchGesture_startCenterPoint<QPointF> for () {
  fn startCenterPoint(self , rsthis: & QPinchGesture) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QPinchGesture16startCenterPointEv()};
    let mut ret = unsafe {C_ZNK13QPinchGesture16startCenterPointEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  qreal QPinchGesture::rotationAngle();
impl /*struct*/ QPinchGesture {
  pub fn rotationAngle<RetType, T: QPinchGesture_rotationAngle<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rotationAngle(self);
    // return 1;
  }
}

pub trait QPinchGesture_rotationAngle<RetType> {
  fn rotationAngle(self , rsthis: & QPinchGesture) -> RetType;
}

  // proto:  qreal QPinchGesture::rotationAngle();
impl<'a> /*trait*/ QPinchGesture_rotationAngle<f64> for () {
  fn rotationAngle(self , rsthis: & QPinchGesture) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QPinchGesture13rotationAngleEv()};
    let mut ret = unsafe {C_ZNK13QPinchGesture13rotationAngleEv(rsthis.qclsinst)};
    return ret as f64; // 1
    // return 1;
  }
}

  // proto:  QPointF QPinchGesture::lastCenterPoint();
impl /*struct*/ QPinchGesture {
  pub fn lastCenterPoint<RetType, T: QPinchGesture_lastCenterPoint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.lastCenterPoint(self);
    // return 1;
  }
}

pub trait QPinchGesture_lastCenterPoint<RetType> {
  fn lastCenterPoint(self , rsthis: & QPinchGesture) -> RetType;
}

  // proto:  QPointF QPinchGesture::lastCenterPoint();
impl<'a> /*trait*/ QPinchGesture_lastCenterPoint<QPointF> for () {
  fn lastCenterPoint(self , rsthis: & QPinchGesture) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QPinchGesture15lastCenterPointEv()};
    let mut ret = unsafe {C_ZNK13QPinchGesture15lastCenterPointEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPinchGesture::QPinchGesture(QObject * parent);
impl /*struct*/ QPinchGesture {
  pub fn new<T: QPinchGesture_new>(value: T) -> QPinchGesture {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QPinchGesture_new {
  fn new(self) -> QPinchGesture;
}

  // proto:  void QPinchGesture::QPinchGesture(QObject * parent);
impl<'a> /*trait*/ QPinchGesture_new for (&'a QObject) {
  fn new(self) -> QPinchGesture {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QPinchGestureC2EP7QObject()};
    let ctysz: c_int = unsafe{QPinchGesture_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN13QPinchGestureC2EP7QObject(arg0)};
    let rsthis = QPinchGesture{qbase: QGesture::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  qreal QPinchGesture::totalScaleFactor();
impl /*struct*/ QPinchGesture {
  pub fn totalScaleFactor<RetType, T: QPinchGesture_totalScaleFactor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.totalScaleFactor(self);
    // return 1;
  }
}

pub trait QPinchGesture_totalScaleFactor<RetType> {
  fn totalScaleFactor(self , rsthis: & QPinchGesture) -> RetType;
}

  // proto:  qreal QPinchGesture::totalScaleFactor();
impl<'a> /*trait*/ QPinchGesture_totalScaleFactor<f64> for () {
  fn totalScaleFactor(self , rsthis: & QPinchGesture) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QPinchGesture16totalScaleFactorEv()};
    let mut ret = unsafe {C_ZNK13QPinchGesture16totalScaleFactorEv(rsthis.qclsinst)};
    return ret as f64; // 1
    // return 1;
  }
}

  // proto:  void QPinchGesture::setTotalScaleFactor(qreal value);
impl /*struct*/ QPinchGesture {
  pub fn setTotalScaleFactor<RetType, T: QPinchGesture_setTotalScaleFactor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTotalScaleFactor(self);
    // return 1;
  }
}

pub trait QPinchGesture_setTotalScaleFactor<RetType> {
  fn setTotalScaleFactor(self , rsthis: & QPinchGesture) -> RetType;
}

  // proto:  void QPinchGesture::setTotalScaleFactor(qreal value);
impl<'a> /*trait*/ QPinchGesture_setTotalScaleFactor<()> for (f64) {
  fn setTotalScaleFactor(self , rsthis: & QPinchGesture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QPinchGesture19setTotalScaleFactorEd()};
    let arg0 = self  as c_double;
     unsafe {C_ZN13QPinchGesture19setTotalScaleFactorEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  qreal QPinchGesture::totalRotationAngle();
impl /*struct*/ QPinchGesture {
  pub fn totalRotationAngle<RetType, T: QPinchGesture_totalRotationAngle<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.totalRotationAngle(self);
    // return 1;
  }
}

pub trait QPinchGesture_totalRotationAngle<RetType> {
  fn totalRotationAngle(self , rsthis: & QPinchGesture) -> RetType;
}

  // proto:  qreal QPinchGesture::totalRotationAngle();
impl<'a> /*trait*/ QPinchGesture_totalRotationAngle<f64> for () {
  fn totalRotationAngle(self , rsthis: & QPinchGesture) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QPinchGesture18totalRotationAngleEv()};
    let mut ret = unsafe {C_ZNK13QPinchGesture18totalRotationAngleEv(rsthis.qclsinst)};
    return ret as f64; // 1
    // return 1;
  }
}

  // proto:  void QPinchGesture::setLastScaleFactor(qreal value);
impl /*struct*/ QPinchGesture {
  pub fn setLastScaleFactor<RetType, T: QPinchGesture_setLastScaleFactor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setLastScaleFactor(self);
    // return 1;
  }
}

pub trait QPinchGesture_setLastScaleFactor<RetType> {
  fn setLastScaleFactor(self , rsthis: & QPinchGesture) -> RetType;
}

  // proto:  void QPinchGesture::setLastScaleFactor(qreal value);
impl<'a> /*trait*/ QPinchGesture_setLastScaleFactor<()> for (f64) {
  fn setLastScaleFactor(self , rsthis: & QPinchGesture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QPinchGesture18setLastScaleFactorEd()};
    let arg0 = self  as c_double;
     unsafe {C_ZN13QPinchGesture18setLastScaleFactorEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPinchGesture::setLastCenterPoint(const QPointF & value);
impl /*struct*/ QPinchGesture {
  pub fn setLastCenterPoint<RetType, T: QPinchGesture_setLastCenterPoint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setLastCenterPoint(self);
    // return 1;
  }
}

pub trait QPinchGesture_setLastCenterPoint<RetType> {
  fn setLastCenterPoint(self , rsthis: & QPinchGesture) -> RetType;
}

  // proto:  void QPinchGesture::setLastCenterPoint(const QPointF & value);
impl<'a> /*trait*/ QPinchGesture_setLastCenterPoint<()> for (&'a QPointF) {
  fn setLastCenterPoint(self , rsthis: & QPinchGesture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QPinchGesture18setLastCenterPointERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN13QPinchGesture18setLastCenterPointERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QPinchGesture::metaObject();
impl /*struct*/ QPinchGesture {
  pub fn metaObject<RetType, T: QPinchGesture_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QPinchGesture_metaObject<RetType> {
  fn metaObject(self , rsthis: & QPinchGesture) -> RetType;
}

  // proto:  const QMetaObject * QPinchGesture::metaObject();
impl<'a> /*trait*/ QPinchGesture_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QPinchGesture) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QPinchGesture10metaObjectEv()};
    let mut ret = unsafe {C_ZNK13QPinchGesture10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPinchGesture::setLastRotationAngle(qreal value);
impl /*struct*/ QPinchGesture {
  pub fn setLastRotationAngle<RetType, T: QPinchGesture_setLastRotationAngle<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setLastRotationAngle(self);
    // return 1;
  }
}

pub trait QPinchGesture_setLastRotationAngle<RetType> {
  fn setLastRotationAngle(self , rsthis: & QPinchGesture) -> RetType;
}

  // proto:  void QPinchGesture::setLastRotationAngle(qreal value);
impl<'a> /*trait*/ QPinchGesture_setLastRotationAngle<()> for (f64) {
  fn setLastRotationAngle(self , rsthis: & QPinchGesture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QPinchGesture20setLastRotationAngleEd()};
    let arg0 = self  as c_double;
     unsafe {C_ZN13QPinchGesture20setLastRotationAngleEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QPointF QPinchGesture::centerPoint();
impl /*struct*/ QPinchGesture {
  pub fn centerPoint<RetType, T: QPinchGesture_centerPoint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.centerPoint(self);
    // return 1;
  }
}

pub trait QPinchGesture_centerPoint<RetType> {
  fn centerPoint(self , rsthis: & QPinchGesture) -> RetType;
}

  // proto:  QPointF QPinchGesture::centerPoint();
impl<'a> /*trait*/ QPinchGesture_centerPoint<QPointF> for () {
  fn centerPoint(self , rsthis: & QPinchGesture) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QPinchGesture11centerPointEv()};
    let mut ret = unsafe {C_ZNK13QPinchGesture11centerPointEv(rsthis.qclsinst)};
    let mut ret1 = QPointF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QPinchGesture::setCenterPoint(const QPointF & value);
impl /*struct*/ QPinchGesture {
  pub fn setCenterPoint<RetType, T: QPinchGesture_setCenterPoint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCenterPoint(self);
    // return 1;
  }
}

pub trait QPinchGesture_setCenterPoint<RetType> {
  fn setCenterPoint(self , rsthis: & QPinchGesture) -> RetType;
}

  // proto:  void QPinchGesture::setCenterPoint(const QPointF & value);
impl<'a> /*trait*/ QPinchGesture_setCenterPoint<()> for (&'a QPointF) {
  fn setCenterPoint(self , rsthis: & QPinchGesture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QPinchGesture14setCenterPointERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN13QPinchGesture14setCenterPointERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPinchGesture::setTotalRotationAngle(qreal value);
impl /*struct*/ QPinchGesture {
  pub fn setTotalRotationAngle<RetType, T: QPinchGesture_setTotalRotationAngle<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTotalRotationAngle(self);
    // return 1;
  }
}

pub trait QPinchGesture_setTotalRotationAngle<RetType> {
  fn setTotalRotationAngle(self , rsthis: & QPinchGesture) -> RetType;
}

  // proto:  void QPinchGesture::setTotalRotationAngle(qreal value);
impl<'a> /*trait*/ QPinchGesture_setTotalRotationAngle<()> for (f64) {
  fn setTotalRotationAngle(self , rsthis: & QPinchGesture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QPinchGesture21setTotalRotationAngleEd()};
    let arg0 = self  as c_double;
     unsafe {C_ZN13QPinchGesture21setTotalRotationAngleEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPinchGesture::setScaleFactor(qreal value);
impl /*struct*/ QPinchGesture {
  pub fn setScaleFactor<RetType, T: QPinchGesture_setScaleFactor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setScaleFactor(self);
    // return 1;
  }
}

pub trait QPinchGesture_setScaleFactor<RetType> {
  fn setScaleFactor(self , rsthis: & QPinchGesture) -> RetType;
}

  // proto:  void QPinchGesture::setScaleFactor(qreal value);
impl<'a> /*trait*/ QPinchGesture_setScaleFactor<()> for (f64) {
  fn setScaleFactor(self , rsthis: & QPinchGesture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QPinchGesture14setScaleFactorEd()};
    let arg0 = self  as c_double;
     unsafe {C_ZN13QPinchGesture14setScaleFactorEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPinchGesture::~QPinchGesture();
impl /*struct*/ QPinchGesture {
  pub fn free<RetType, T: QPinchGesture_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QPinchGesture_free<RetType> {
  fn free(self , rsthis: & QPinchGesture) -> RetType;
}

  // proto:  void QPinchGesture::~QPinchGesture();
impl<'a> /*trait*/ QPinchGesture_free<()> for () {
  fn free(self , rsthis: & QPinchGesture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QPinchGestureD2Ev()};
     unsafe {C_ZN13QPinchGestureD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QPinchGesture::setStartCenterPoint(const QPointF & value);
impl /*struct*/ QPinchGesture {
  pub fn setStartCenterPoint<RetType, T: QPinchGesture_setStartCenterPoint<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setStartCenterPoint(self);
    // return 1;
  }
}

pub trait QPinchGesture_setStartCenterPoint<RetType> {
  fn setStartCenterPoint(self , rsthis: & QPinchGesture) -> RetType;
}

  // proto:  void QPinchGesture::setStartCenterPoint(const QPointF & value);
impl<'a> /*trait*/ QPinchGesture_setStartCenterPoint<()> for (&'a QPointF) {
  fn setStartCenterPoint(self , rsthis: & QPinchGesture) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QPinchGesture19setStartCenterPointERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN13QPinchGesture19setStartCenterPointERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  qreal QPinchGesture::scaleFactor();
impl /*struct*/ QPinchGesture {
  pub fn scaleFactor<RetType, T: QPinchGesture_scaleFactor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.scaleFactor(self);
    // return 1;
  }
}

pub trait QPinchGesture_scaleFactor<RetType> {
  fn scaleFactor(self , rsthis: & QPinchGesture) -> RetType;
}

  // proto:  qreal QPinchGesture::scaleFactor();
impl<'a> /*trait*/ QPinchGesture_scaleFactor<f64> for () {
  fn scaleFactor(self , rsthis: & QPinchGesture) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QPinchGesture11scaleFactorEv()};
    let mut ret = unsafe {C_ZNK13QPinchGesture11scaleFactorEv(rsthis.qclsinst)};
    return ret as f64; // 1
    // return 1;
  }
}

// <= body block end

