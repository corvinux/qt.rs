// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qmatrix::QMatrix;
use super::qpolygonf::QPolygonF;
use super::qtransform::QTransform;
use super::qrectf::QRectF;
use super::qpointf::QPointF;
use super::qfont::QFont;
use super::qstring::QString;
use super::qregion::QRegion;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QPainterPath::setElementPositionAt(int i, qreal x, qreal y);
  fn _ZN12QPainterPath20setElementPositionAtEidd(qthis: *mut c_void, arg0: c_int, arg1: c_double, arg2: c_double);
  // proto:  QPolygonF QPainterPath::toFillPolygon(const QMatrix & matrix);
  fn _ZNK12QPainterPath13toFillPolygonERK7QMatrix(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QPainterPath QPainterPath::translated(qreal dx, qreal dy);
  fn _ZNK12QPainterPath10translatedEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double) -> *mut c_void;
  // proto:  void QPainterPath::~QPainterPath();
  fn _ZN12QPainterPathD0Ev(qthis: *mut c_void);
  // proto:  QList<QPolygonF> QPainterPath::toSubpathPolygons(const QTransform & matrix);
  fn _ZNK12QPainterPath17toSubpathPolygonsERK10QTransform(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QRectF QPainterPath::controlPointRect();
  fn _ZNK12QPainterPath16controlPointRectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QList<QPolygonF> QPainterPath::toFillPolygons(const QMatrix & matrix);
  fn _ZNK12QPainterPath14toFillPolygonsERK7QMatrix(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QPainterPath QPainterPath::translated(const QPointF & offset);
  fn _ZNK12QPainterPath10translatedERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QPainterPath::quadTo(const QPointF & ctrlPt, const QPointF & endPt);
  fn _ZN12QPainterPath6quadToERK7QPointFS2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  QList<QPolygonF> QPainterPath::toFillPolygons(const QTransform & matrix);
  fn _ZNK12QPainterPath14toFillPolygonsERK10QTransform(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QPainterPath::arcTo(qreal x, qreal y, qreal w, qreal h, qreal startAngle, qreal arcLength);
  fn _ZN12QPainterPath5arcToEdddddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double, arg4: c_double, arg5: c_double);
  // proto:  void QPainterPath::addRect(const QRectF & rect);
  fn _ZN12QPainterPath7addRectERK6QRectF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QPainterPath::addRoundRect(const QRectF & rect, int xRnd, int yRnd);
  fn _ZN12QPainterPath12addRoundRectERK6QRectFii(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: c_int);
  // proto:  void QPainterPath::addText(qreal x, qreal y, const QFont & f, const QString & text);
  fn _ZN12QPainterPath7addTextEddRK5QFontRK7QString(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: *mut c_void, arg3: *mut c_void);
  // proto:  bool QPainterPath::intersects(const QRectF & rect);
  fn _ZNK12QPainterPath10intersectsERK6QRectF(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  bool QPainterPath::contains(const QPointF & pt);
  fn _ZNK12QPainterPath8containsERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  void QPainterPath::arcTo(const QRectF & rect, qreal startAngle, qreal arcLength);
  fn _ZN12QPainterPath5arcToERK6QRectFdd(qthis: *mut c_void, arg0: *mut c_void, arg1: c_double, arg2: c_double);
  // proto:  void QPainterPath::addRoundRect(const QRectF & rect, int roundness);
  fn _ZN12QPainterPath12addRoundRectERK6QRectFi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int);
  // proto:  void QPainterPath::addEllipse(const QPointF & center, qreal rx, qreal ry);
  fn _ZN12QPainterPath10addEllipseERK7QPointFdd(qthis: *mut c_void, arg0: *mut c_void, arg1: c_double, arg2: c_double);
  // proto:  void QPainterPath::lineTo(qreal x, qreal y);
  fn _ZN12QPainterPath6lineToEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double);
  // proto:  void QPainterPath::cubicTo(const QPointF & ctrlPt1, const QPointF & ctrlPt2, const QPointF & endPt);
  fn _ZN12QPainterPath7cubicToERK7QPointFS2_S2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  qreal QPainterPath::slopeAtPercent(qreal t);
  fn _ZNK12QPainterPath14slopeAtPercentEd(qthis: *mut c_void, arg0: c_double) -> c_double;
  // proto:  void QPainterPath::addEllipse(qreal x, qreal y, qreal w, qreal h);
  fn _ZN12QPainterPath10addEllipseEdddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double);
  // proto:  bool QPainterPath::intersects(const QPainterPath & p);
  fn _ZNK12QPainterPath10intersectsERKS_(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  void QPainterPath::addRoundRect(qreal x, qreal y, qreal w, qreal h, int roundness);
  fn _ZN12QPainterPath12addRoundRectEddddi(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double, arg4: c_int);
  // proto:  void QPainterPath::QPainterPath(const QPointF & startPoint);
  fn _ZN12QPainterPathC1ERK7QPointF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QPainterPath QPainterPath::intersected(const QPainterPath & r);
  fn _ZNK12QPainterPath11intersectedERKS_(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QPainterPath::translate(qreal dx, qreal dy);
  fn _ZN12QPainterPath9translateEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double);
  // proto:  void QPainterPath::addPolygon(const QPolygonF & polygon);
  fn _ZN12QPainterPath10addPolygonERK9QPolygonF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QPainterPath::translate(const QPointF & offset);
  fn _ZN12QPainterPath9translateERK7QPointF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QPolygonF QPainterPath::toFillPolygon(const QTransform & matrix);
  fn _ZNK12QPainterPath13toFillPolygonERK10QTransform(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QPainterPath::addPath(const QPainterPath & path);
  fn _ZN12QPainterPath7addPathERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QPainterPath::quadTo(qreal ctrlPtx, qreal ctrlPty, qreal endPtx, qreal endPty);
  fn _ZN12QPainterPath6quadToEdddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double);
  // proto:  int QPainterPath::elementCount();
  fn _ZNK12QPainterPath12elementCountEv(qthis: *mut c_void) -> c_int;
  // proto:  QPainterPath QPainterPath::simplified();
  fn _ZNK12QPainterPath10simplifiedEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QPainterPath::contains(const QRectF & rect);
  fn _ZNK12QPainterPath8containsERK6QRectF(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  qreal QPainterPath::length();
  fn _ZNK12QPainterPath6lengthEv(qthis: *mut c_void) -> c_double;
  // proto:  void QPainterPath::connectPath(const QPainterPath & path);
  fn _ZN12QPainterPath11connectPathERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QPainterPath::addRegion(const QRegion & region);
  fn _ZN12QPainterPath9addRegionERK7QRegion(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QPointF QPainterPath::currentPosition();
  fn _ZNK12QPainterPath15currentPositionEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QPainterPath QPainterPath::toReversed();
  fn _ZNK12QPainterPath10toReversedEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPainterPath::addRoundRect(qreal x, qreal y, qreal w, qreal h, int xRnd, int yRnd);
  fn _ZN12QPainterPath12addRoundRectEddddii(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double, arg4: c_int, arg5: c_int);
  // proto:  QRectF QPainterPath::boundingRect();
  fn _ZNK12QPainterPath12boundingRectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QPainterPath::swap(QPainterPath & other);
  fn _ZN12QPainterPath4swapERS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QPainterPath::contains(const QPainterPath & p);
  fn _ZNK12QPainterPath8containsERKS_(qthis: *mut c_void, arg0: *mut c_void) -> c_char;
  // proto:  void QPainterPath::moveTo(qreal x, qreal y);
  fn _ZN12QPainterPath6moveToEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double);
  // proto:  QPainterPath QPainterPath::subtracted(const QPainterPath & r);
  fn _ZNK12QPainterPath10subtractedERKS_(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QPainterPath::QPainterPath();
  fn _ZN12QPainterPathC1Ev(qthis: *mut c_void);
  // proto:  void QPainterPath::addText(const QPointF & point, const QFont & f, const QString & text);
  fn _ZN12QPainterPath7addTextERK7QPointFRK5QFontRK7QString(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  void QPainterPath::QPainterPath(const QPainterPath & other);
  fn _ZN12QPainterPathC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QPointF QPainterPath::pointAtPercent(qreal t);
  fn _ZNK12QPainterPath14pointAtPercentEd(qthis: *mut c_void, arg0: c_double) -> *mut c_void;
  // proto:  qreal QPainterPath::percentAtLength(qreal t);
  fn _ZNK12QPainterPath15percentAtLengthEd(qthis: *mut c_void, arg0: c_double) -> c_double;
  // proto:  void QPainterPath::cubicTo(qreal ctrlPt1x, qreal ctrlPt1y, qreal ctrlPt2x, qreal ctrlPt2y, qreal endPtx, qreal endPty);
  fn _ZN12QPainterPath7cubicToEdddddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double, arg4: c_double, arg5: c_double);
  // proto:  void QPainterPath::lineTo(const QPointF & p);
  fn _ZN12QPainterPath6lineToERK7QPointF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QPainterPath QPainterPath::subtractedInverted(const QPainterPath & r);
  fn _ZNK12QPainterPath18subtractedInvertedERKS_(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QPainterPath::arcMoveTo(qreal x, qreal y, qreal w, qreal h, qreal angle);
  fn _ZN12QPainterPath9arcMoveToEddddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double, arg4: c_double);
  // proto:  bool QPainterPath::isEmpty();
  fn _ZNK12QPainterPath7isEmptyEv(qthis: *mut c_void) -> c_char;
  // proto:  void QPainterPath::addRect(qreal x, qreal y, qreal w, qreal h);
  fn _ZN12QPainterPath7addRectEdddd(qthis: *mut c_void, arg0: c_double, arg1: c_double, arg2: c_double, arg3: c_double);
  // proto:  void QPainterPath::arcMoveTo(const QRectF & rect, qreal angle);
  fn _ZN12QPainterPath9arcMoveToERK6QRectFd(qthis: *mut c_void, arg0: *mut c_void, arg1: c_double);
  // proto:  QList<QPolygonF> QPainterPath::toSubpathPolygons(const QMatrix & matrix);
  fn _ZNK12QPainterPath17toSubpathPolygonsERK7QMatrix(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QPainterPath QPainterPath::united(const QPainterPath & r);
  fn _ZNK12QPainterPath6unitedERKS_(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QPainterPath::addEllipse(const QRectF & rect);
  fn _ZN12QPainterPath10addEllipseERK6QRectF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QPainterPath::moveTo(const QPointF & p);
  fn _ZN12QPainterPath6moveToERK7QPointF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  qreal QPainterPath::angleAtPercent(qreal t);
  fn _ZNK12QPainterPath14angleAtPercentEd(qthis: *mut c_void, arg0: c_double) -> c_double;
  // proto:  void QPainterPath::closeSubpath();
  fn _ZN12QPainterPath12closeSubpathEv(qthis: *mut c_void);
}

// body block begin
// class sizeof(QPainterPath)=1
pub struct QPainterPath {
  pub qclsinst: *mut c_void,
}

  // proto:  void QPainterPath::setElementPositionAt(int i, qreal x, qreal y);
impl /*struct*/ QPainterPath {
  pub fn setElementPositionAt<RetType, T: QPainterPath_setElementPositionAt<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setElementPositionAt(self);
    // return 1;
  }
}

pub trait QPainterPath_setElementPositionAt<RetType> {
  fn setElementPositionAt(self , rsthis: &mut QPainterPath) -> RetType;
}

  // proto:  void QPainterPath::setElementPositionAt(int i, qreal x, qreal y);
impl<'a> /*trait*/ QPainterPath_setElementPositionAt<()> for (i32, f64, f64) {
  fn setElementPositionAt(self , rsthis: &mut QPainterPath) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPath20setElementPositionAtEidd()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
     unsafe {_ZN12QPainterPath20setElementPositionAtEidd(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  QPolygonF QPainterPath::toFillPolygon(const QMatrix & matrix);
impl /*struct*/ QPainterPath {
  pub fn toFillPolygon<RetType, T: QPainterPath_toFillPolygon<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toFillPolygon(self);
    // return 1;
  }
}

pub trait QPainterPath_toFillPolygon<RetType> {
  fn toFillPolygon(self , rsthis: &mut QPainterPath) -> RetType;
}

  // proto:  QPolygonF QPainterPath::toFillPolygon(const QMatrix & matrix);
impl<'a> /*trait*/ QPainterPath_toFillPolygon<QPolygonF> for (QMatrix) {
  fn toFillPolygon(self , rsthis: &mut QPainterPath) -> QPolygonF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath13toFillPolygonERK7QMatrix()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK12QPainterPath13toFillPolygonERK7QMatrix(rsthis.qclsinst, arg0)};
    let mut ret1 = QPolygonF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QPainterPath QPainterPath::translated(qreal dx, qreal dy);
impl /*struct*/ QPainterPath {
  pub fn translated<RetType, T: QPainterPath_translated<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.translated(self);
    // return 1;
  }
}

pub trait QPainterPath_translated<RetType> {
  fn translated(self , rsthis: &mut QPainterPath) -> RetType;
}

  // proto:  QPainterPath QPainterPath::translated(qreal dx, qreal dy);
impl<'a> /*trait*/ QPainterPath_translated<QPainterPath> for (f64, f64) {
  fn translated(self , rsthis: &mut QPainterPath) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath10translatedEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let mut ret = unsafe {_ZNK12QPainterPath10translatedEdd(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QPainterPath{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QPainterPath::~QPainterPath();
impl /*struct*/ QPainterPath {
  pub fn FreeQPainterPath<RetType, T: QPainterPath_FreeQPainterPath<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQPainterPath(self);
    // return 1;
  }
}

pub trait QPainterPath_FreeQPainterPath<RetType> {
  fn FreeQPainterPath(self , rsthis: &mut QPainterPath) -> RetType;
}

  // proto:  void QPainterPath::~QPainterPath();
impl<'a> /*trait*/ QPainterPath_FreeQPainterPath<()> for () {
  fn FreeQPainterPath(self , rsthis: &mut QPainterPath) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPathD0Ev()};
     unsafe {_ZN12QPainterPathD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QList<QPolygonF> QPainterPath::toSubpathPolygons(const QTransform & matrix);
impl /*struct*/ QPainterPath {
  pub fn toSubpathPolygons<RetType, T: QPainterPath_toSubpathPolygons<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toSubpathPolygons(self);
    // return 1;
  }
}

pub trait QPainterPath_toSubpathPolygons<RetType> {
  fn toSubpathPolygons(self , rsthis: &mut QPainterPath) -> RetType;
}

  // proto:  QList<QPolygonF> QPainterPath::toSubpathPolygons(const QTransform & matrix);
impl<'a> /*trait*/ QPainterPath_toSubpathPolygons<()> for (QTransform) {
  fn toSubpathPolygons(self , rsthis: &mut QPainterPath) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath17toSubpathPolygonsERK10QTransform()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZNK12QPainterPath17toSubpathPolygonsERK10QTransform(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QRectF QPainterPath::controlPointRect();
impl /*struct*/ QPainterPath {
  pub fn controlPointRect<RetType, T: QPainterPath_controlPointRect<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.controlPointRect(self);
    // return 1;
  }
}

pub trait QPainterPath_controlPointRect<RetType> {
  fn controlPointRect(self , rsthis: &mut QPainterPath) -> RetType;
}

  // proto:  QRectF QPainterPath::controlPointRect();
impl<'a> /*trait*/ QPainterPath_controlPointRect<QRectF> for () {
  fn controlPointRect(self , rsthis: &mut QPainterPath) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath16controlPointRectEv()};
    let mut ret = unsafe {_ZNK12QPainterPath16controlPointRectEv(rsthis.qclsinst)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QList<QPolygonF> QPainterPath::toFillPolygons(const QMatrix & matrix);
impl /*struct*/ QPainterPath {
  pub fn toFillPolygons<RetType, T: QPainterPath_toFillPolygons<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toFillPolygons(self);
    // return 1;
  }
}

pub trait QPainterPath_toFillPolygons<RetType> {
  fn toFillPolygons(self , rsthis: &mut QPainterPath) -> RetType;
}

  // proto:  QList<QPolygonF> QPainterPath::toFillPolygons(const QMatrix & matrix);
impl<'a> /*trait*/ QPainterPath_toFillPolygons<()> for (QMatrix) {
  fn toFillPolygons(self , rsthis: &mut QPainterPath) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath14toFillPolygonsERK7QMatrix()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZNK12QPainterPath14toFillPolygonsERK7QMatrix(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QPainterPath QPainterPath::translated(const QPointF & offset);
impl<'a> /*trait*/ QPainterPath_translated<QPainterPath> for (QPointF) {
  fn translated(self , rsthis: &mut QPainterPath) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath10translatedERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK12QPainterPath10translatedERK7QPointF(rsthis.qclsinst, arg0)};
    let mut ret1 = QPainterPath{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QPainterPath::quadTo(const QPointF & ctrlPt, const QPointF & endPt);
impl /*struct*/ QPainterPath {
  pub fn quadTo<RetType, T: QPainterPath_quadTo<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.quadTo(self);
    // return 1;
  }
}

pub trait QPainterPath_quadTo<RetType> {
  fn quadTo(self , rsthis: &mut QPainterPath) -> RetType;
}

  // proto:  void QPainterPath::quadTo(const QPointF & ctrlPt, const QPointF & endPt);
impl<'a> /*trait*/ QPainterPath_quadTo<()> for (QPointF, QPointF) {
  fn quadTo(self , rsthis: &mut QPainterPath) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPath6quadToERK7QPointFS2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN12QPainterPath6quadToERK7QPointFS2_(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QList<QPolygonF> QPainterPath::toFillPolygons(const QTransform & matrix);
impl<'a> /*trait*/ QPainterPath_toFillPolygons<()> for (QTransform) {
  fn toFillPolygons(self , rsthis: &mut QPainterPath) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath14toFillPolygonsERK10QTransform()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZNK12QPainterPath14toFillPolygonsERK10QTransform(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPainterPath::arcTo(qreal x, qreal y, qreal w, qreal h, qreal startAngle, qreal arcLength);
impl /*struct*/ QPainterPath {
  pub fn arcTo<RetType, T: QPainterPath_arcTo<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.arcTo(self);
    // return 1;
  }
}

pub trait QPainterPath_arcTo<RetType> {
  fn arcTo(self , rsthis: &mut QPainterPath) -> RetType;
}

  // proto:  void QPainterPath::arcTo(qreal x, qreal y, qreal w, qreal h, qreal startAngle, qreal arcLength);
impl<'a> /*trait*/ QPainterPath_arcTo<()> for (f64, f64, f64, f64, f64, f64) {
  fn arcTo(self , rsthis: &mut QPainterPath) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPath5arcToEdddddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let arg4 = self.4  as c_double;
    let arg5 = self.5  as c_double;
     unsafe {_ZN12QPainterPath5arcToEdddddd(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4, arg5)};
    // return 1;
  }
}

  // proto:  void QPainterPath::addRect(const QRectF & rect);
impl /*struct*/ QPainterPath {
  pub fn addRect<RetType, T: QPainterPath_addRect<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.addRect(self);
    // return 1;
  }
}

pub trait QPainterPath_addRect<RetType> {
  fn addRect(self , rsthis: &mut QPainterPath) -> RetType;
}

  // proto:  void QPainterPath::addRect(const QRectF & rect);
impl<'a> /*trait*/ QPainterPath_addRect<()> for (QRectF) {
  fn addRect(self , rsthis: &mut QPainterPath) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPath7addRectERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QPainterPath7addRectERK6QRectF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPainterPath::addRoundRect(const QRectF & rect, int xRnd, int yRnd);
impl /*struct*/ QPainterPath {
  pub fn addRoundRect<RetType, T: QPainterPath_addRoundRect<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.addRoundRect(self);
    // return 1;
  }
}

pub trait QPainterPath_addRoundRect<RetType> {
  fn addRoundRect(self , rsthis: &mut QPainterPath) -> RetType;
}

  // proto:  void QPainterPath::addRoundRect(const QRectF & rect, int xRnd, int yRnd);
impl<'a> /*trait*/ QPainterPath_addRoundRect<()> for (QRectF, i32, i32) {
  fn addRoundRect(self , rsthis: &mut QPainterPath) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPath12addRoundRectERK6QRectFii()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
     unsafe {_ZN12QPainterPath12addRoundRectERK6QRectFii(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QPainterPath::addText(qreal x, qreal y, const QFont & f, const QString & text);
impl /*struct*/ QPainterPath {
  pub fn addText<RetType, T: QPainterPath_addText<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.addText(self);
    // return 1;
  }
}

pub trait QPainterPath_addText<RetType> {
  fn addText(self , rsthis: &mut QPainterPath) -> RetType;
}

  // proto:  void QPainterPath::addText(qreal x, qreal y, const QFont & f, const QString & text);
impl<'a> /*trait*/ QPainterPath_addText<()> for (f64, f64, QFont, QString) {
  fn addText(self , rsthis: &mut QPainterPath) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPath7addTextEddRK5QFontRK7QString()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3.qclsinst  as *mut c_void;
     unsafe {_ZN12QPainterPath7addTextEddRK5QFontRK7QString(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  bool QPainterPath::intersects(const QRectF & rect);
impl /*struct*/ QPainterPath {
  pub fn intersects<RetType, T: QPainterPath_intersects<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.intersects(self);
    // return 1;
  }
}

pub trait QPainterPath_intersects<RetType> {
  fn intersects(self , rsthis: &mut QPainterPath) -> RetType;
}

  // proto:  bool QPainterPath::intersects(const QRectF & rect);
impl<'a> /*trait*/ QPainterPath_intersects<i8> for (QRectF) {
  fn intersects(self , rsthis: &mut QPainterPath) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath10intersectsERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK12QPainterPath10intersectsERK6QRectF(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QPainterPath::contains(const QPointF & pt);
impl /*struct*/ QPainterPath {
  pub fn contains<RetType, T: QPainterPath_contains<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.contains(self);
    // return 1;
  }
}

pub trait QPainterPath_contains<RetType> {
  fn contains(self , rsthis: &mut QPainterPath) -> RetType;
}

  // proto:  bool QPainterPath::contains(const QPointF & pt);
impl<'a> /*trait*/ QPainterPath_contains<i8> for (QPointF) {
  fn contains(self , rsthis: &mut QPainterPath) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath8containsERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK12QPainterPath8containsERK7QPointF(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QPainterPath::arcTo(const QRectF & rect, qreal startAngle, qreal arcLength);
impl<'a> /*trait*/ QPainterPath_arcTo<()> for (QRectF, f64, f64) {
  fn arcTo(self , rsthis: &mut QPainterPath) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPath5arcToERK6QRectFdd()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
     unsafe {_ZN12QPainterPath5arcToERK6QRectFdd(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QPainterPath::addRoundRect(const QRectF & rect, int roundness);
impl<'a> /*trait*/ QPainterPath_addRoundRect<()> for (QRectF, i32) {
  fn addRoundRect(self , rsthis: &mut QPainterPath) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPath12addRoundRectERK6QRectFi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN12QPainterPath12addRoundRectERK6QRectFi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QPainterPath::addEllipse(const QPointF & center, qreal rx, qreal ry);
impl /*struct*/ QPainterPath {
  pub fn addEllipse<RetType, T: QPainterPath_addEllipse<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.addEllipse(self);
    // return 1;
  }
}

pub trait QPainterPath_addEllipse<RetType> {
  fn addEllipse(self , rsthis: &mut QPainterPath) -> RetType;
}

  // proto:  void QPainterPath::addEllipse(const QPointF & center, qreal rx, qreal ry);
impl<'a> /*trait*/ QPainterPath_addEllipse<()> for (QPointF, f64, f64) {
  fn addEllipse(self , rsthis: &mut QPainterPath) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPath10addEllipseERK7QPointFdd()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
     unsafe {_ZN12QPainterPath10addEllipseERK7QPointFdd(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QPainterPath::lineTo(qreal x, qreal y);
impl /*struct*/ QPainterPath {
  pub fn lineTo<RetType, T: QPainterPath_lineTo<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.lineTo(self);
    // return 1;
  }
}

pub trait QPainterPath_lineTo<RetType> {
  fn lineTo(self , rsthis: &mut QPainterPath) -> RetType;
}

  // proto:  void QPainterPath::lineTo(qreal x, qreal y);
impl<'a> /*trait*/ QPainterPath_lineTo<()> for (f64, f64) {
  fn lineTo(self , rsthis: &mut QPainterPath) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPath6lineToEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
     unsafe {_ZN12QPainterPath6lineToEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QPainterPath::cubicTo(const QPointF & ctrlPt1, const QPointF & ctrlPt2, const QPointF & endPt);
impl /*struct*/ QPainterPath {
  pub fn cubicTo<RetType, T: QPainterPath_cubicTo<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.cubicTo(self);
    // return 1;
  }
}

pub trait QPainterPath_cubicTo<RetType> {
  fn cubicTo(self , rsthis: &mut QPainterPath) -> RetType;
}

  // proto:  void QPainterPath::cubicTo(const QPointF & ctrlPt1, const QPointF & ctrlPt2, const QPointF & endPt);
impl<'a> /*trait*/ QPainterPath_cubicTo<()> for (QPointF, QPointF, QPointF) {
  fn cubicTo(self , rsthis: &mut QPainterPath) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPath7cubicToERK7QPointFS2_S2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN12QPainterPath7cubicToERK7QPointFS2_S2_(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  qreal QPainterPath::slopeAtPercent(qreal t);
impl /*struct*/ QPainterPath {
  pub fn slopeAtPercent<RetType, T: QPainterPath_slopeAtPercent<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.slopeAtPercent(self);
    // return 1;
  }
}

pub trait QPainterPath_slopeAtPercent<RetType> {
  fn slopeAtPercent(self , rsthis: &mut QPainterPath) -> RetType;
}

  // proto:  qreal QPainterPath::slopeAtPercent(qreal t);
impl<'a> /*trait*/ QPainterPath_slopeAtPercent<f64> for (f64) {
  fn slopeAtPercent(self , rsthis: &mut QPainterPath) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath14slopeAtPercentEd()};
    let arg0 = self  as c_double;
    let mut ret = unsafe {_ZNK12QPainterPath14slopeAtPercentEd(rsthis.qclsinst, arg0)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QPainterPath::addEllipse(qreal x, qreal y, qreal w, qreal h);
impl<'a> /*trait*/ QPainterPath_addEllipse<()> for (f64, f64, f64, f64) {
  fn addEllipse(self , rsthis: &mut QPainterPath) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPath10addEllipseEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
     unsafe {_ZN12QPainterPath10addEllipseEdddd(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  bool QPainterPath::intersects(const QPainterPath & p);
impl<'a> /*trait*/ QPainterPath_intersects<i8> for (QPainterPath) {
  fn intersects(self , rsthis: &mut QPainterPath) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath10intersectsERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK12QPainterPath10intersectsERKS_(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QPainterPath::addRoundRect(qreal x, qreal y, qreal w, qreal h, int roundness);
impl<'a> /*trait*/ QPainterPath_addRoundRect<()> for (f64, f64, f64, f64, i32) {
  fn addRoundRect(self , rsthis: &mut QPainterPath) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPath12addRoundRectEddddi()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let arg4 = self.4  as c_int;
     unsafe {_ZN12QPainterPath12addRoundRectEddddi(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4)};
    // return 1;
  }
}

  // proto:  void QPainterPath::QPainterPath(const QPointF & startPoint);
impl /*struct*/ QPainterPath {
  pub fn NewQPainterPath<T: QPainterPath_NewQPainterPath>(value: T) -> QPainterPath {
    let rsthis = value.NewQPainterPath();
    return rsthis;
    // return 1;
  }
}

pub trait QPainterPath_NewQPainterPath {
  fn NewQPainterPath(self) -> QPainterPath;
}

  // proto:  void QPainterPath::QPainterPath(const QPointF & startPoint);
impl<'a> /*trait*/ QPainterPath_NewQPainterPath for (QPointF) {
  fn NewQPainterPath(self) -> QPainterPath {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPathC1ERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QPainterPathC1ERK7QPointF(qthis, arg0)};
    let rsthis = QPainterPath{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QPainterPath QPainterPath::intersected(const QPainterPath & r);
impl /*struct*/ QPainterPath {
  pub fn intersected<RetType, T: QPainterPath_intersected<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.intersected(self);
    // return 1;
  }
}

pub trait QPainterPath_intersected<RetType> {
  fn intersected(self , rsthis: &mut QPainterPath) -> RetType;
}

  // proto:  QPainterPath QPainterPath::intersected(const QPainterPath & r);
impl<'a> /*trait*/ QPainterPath_intersected<QPainterPath> for (QPainterPath) {
  fn intersected(self , rsthis: &mut QPainterPath) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath11intersectedERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK12QPainterPath11intersectedERKS_(rsthis.qclsinst, arg0)};
    let mut ret1 = QPainterPath{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QPainterPath::translate(qreal dx, qreal dy);
impl /*struct*/ QPainterPath {
  pub fn translate<RetType, T: QPainterPath_translate<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.translate(self);
    // return 1;
  }
}

pub trait QPainterPath_translate<RetType> {
  fn translate(self , rsthis: &mut QPainterPath) -> RetType;
}

  // proto:  void QPainterPath::translate(qreal dx, qreal dy);
impl<'a> /*trait*/ QPainterPath_translate<()> for (f64, f64) {
  fn translate(self , rsthis: &mut QPainterPath) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPath9translateEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
     unsafe {_ZN12QPainterPath9translateEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QPainterPath::addPolygon(const QPolygonF & polygon);
impl /*struct*/ QPainterPath {
  pub fn addPolygon<RetType, T: QPainterPath_addPolygon<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.addPolygon(self);
    // return 1;
  }
}

pub trait QPainterPath_addPolygon<RetType> {
  fn addPolygon(self , rsthis: &mut QPainterPath) -> RetType;
}

  // proto:  void QPainterPath::addPolygon(const QPolygonF & polygon);
impl<'a> /*trait*/ QPainterPath_addPolygon<()> for (QPolygonF) {
  fn addPolygon(self , rsthis: &mut QPainterPath) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPath10addPolygonERK9QPolygonF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QPainterPath10addPolygonERK9QPolygonF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPainterPath::translate(const QPointF & offset);
impl<'a> /*trait*/ QPainterPath_translate<()> for (QPointF) {
  fn translate(self , rsthis: &mut QPainterPath) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPath9translateERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QPainterPath9translateERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QPolygonF QPainterPath::toFillPolygon(const QTransform & matrix);
impl<'a> /*trait*/ QPainterPath_toFillPolygon<QPolygonF> for (QTransform) {
  fn toFillPolygon(self , rsthis: &mut QPainterPath) -> QPolygonF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath13toFillPolygonERK10QTransform()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK12QPainterPath13toFillPolygonERK10QTransform(rsthis.qclsinst, arg0)};
    let mut ret1 = QPolygonF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QPainterPath::addPath(const QPainterPath & path);
impl /*struct*/ QPainterPath {
  pub fn addPath<RetType, T: QPainterPath_addPath<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.addPath(self);
    // return 1;
  }
}

pub trait QPainterPath_addPath<RetType> {
  fn addPath(self , rsthis: &mut QPainterPath) -> RetType;
}

  // proto:  void QPainterPath::addPath(const QPainterPath & path);
impl<'a> /*trait*/ QPainterPath_addPath<()> for (QPainterPath) {
  fn addPath(self , rsthis: &mut QPainterPath) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPath7addPathERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QPainterPath7addPathERKS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPainterPath::quadTo(qreal ctrlPtx, qreal ctrlPty, qreal endPtx, qreal endPty);
impl<'a> /*trait*/ QPainterPath_quadTo<()> for (f64, f64, f64, f64) {
  fn quadTo(self , rsthis: &mut QPainterPath) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPath6quadToEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
     unsafe {_ZN12QPainterPath6quadToEdddd(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  int QPainterPath::elementCount();
impl /*struct*/ QPainterPath {
  pub fn elementCount<RetType, T: QPainterPath_elementCount<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.elementCount(self);
    // return 1;
  }
}

pub trait QPainterPath_elementCount<RetType> {
  fn elementCount(self , rsthis: &mut QPainterPath) -> RetType;
}

  // proto:  int QPainterPath::elementCount();
impl<'a> /*trait*/ QPainterPath_elementCount<i32> for () {
  fn elementCount(self , rsthis: &mut QPainterPath) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath12elementCountEv()};
    let mut ret = unsafe {_ZNK12QPainterPath12elementCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QPainterPath QPainterPath::simplified();
impl /*struct*/ QPainterPath {
  pub fn simplified<RetType, T: QPainterPath_simplified<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.simplified(self);
    // return 1;
  }
}

pub trait QPainterPath_simplified<RetType> {
  fn simplified(self , rsthis: &mut QPainterPath) -> RetType;
}

  // proto:  QPainterPath QPainterPath::simplified();
impl<'a> /*trait*/ QPainterPath_simplified<QPainterPath> for () {
  fn simplified(self , rsthis: &mut QPainterPath) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath10simplifiedEv()};
    let mut ret = unsafe {_ZNK12QPainterPath10simplifiedEv(rsthis.qclsinst)};
    let mut ret1 = QPainterPath{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  bool QPainterPath::contains(const QRectF & rect);
impl<'a> /*trait*/ QPainterPath_contains<i8> for (QRectF) {
  fn contains(self , rsthis: &mut QPainterPath) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath8containsERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK12QPainterPath8containsERK6QRectF(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  qreal QPainterPath::length();
impl /*struct*/ QPainterPath {
  pub fn length<RetType, T: QPainterPath_length<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.length(self);
    // return 1;
  }
}

pub trait QPainterPath_length<RetType> {
  fn length(self , rsthis: &mut QPainterPath) -> RetType;
}

  // proto:  qreal QPainterPath::length();
impl<'a> /*trait*/ QPainterPath_length<f64> for () {
  fn length(self , rsthis: &mut QPainterPath) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath6lengthEv()};
    let mut ret = unsafe {_ZNK12QPainterPath6lengthEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QPainterPath::connectPath(const QPainterPath & path);
impl /*struct*/ QPainterPath {
  pub fn connectPath<RetType, T: QPainterPath_connectPath<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.connectPath(self);
    // return 1;
  }
}

pub trait QPainterPath_connectPath<RetType> {
  fn connectPath(self , rsthis: &mut QPainterPath) -> RetType;
}

  // proto:  void QPainterPath::connectPath(const QPainterPath & path);
impl<'a> /*trait*/ QPainterPath_connectPath<()> for (QPainterPath) {
  fn connectPath(self , rsthis: &mut QPainterPath) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPath11connectPathERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QPainterPath11connectPathERKS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPainterPath::addRegion(const QRegion & region);
impl /*struct*/ QPainterPath {
  pub fn addRegion<RetType, T: QPainterPath_addRegion<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.addRegion(self);
    // return 1;
  }
}

pub trait QPainterPath_addRegion<RetType> {
  fn addRegion(self , rsthis: &mut QPainterPath) -> RetType;
}

  // proto:  void QPainterPath::addRegion(const QRegion & region);
impl<'a> /*trait*/ QPainterPath_addRegion<()> for (QRegion) {
  fn addRegion(self , rsthis: &mut QPainterPath) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPath9addRegionERK7QRegion()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QPainterPath9addRegionERK7QRegion(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QPointF QPainterPath::currentPosition();
impl /*struct*/ QPainterPath {
  pub fn currentPosition<RetType, T: QPainterPath_currentPosition<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.currentPosition(self);
    // return 1;
  }
}

pub trait QPainterPath_currentPosition<RetType> {
  fn currentPosition(self , rsthis: &mut QPainterPath) -> RetType;
}

  // proto:  QPointF QPainterPath::currentPosition();
impl<'a> /*trait*/ QPainterPath_currentPosition<QPointF> for () {
  fn currentPosition(self , rsthis: &mut QPainterPath) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath15currentPositionEv()};
    let mut ret = unsafe {_ZNK12QPainterPath15currentPositionEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QPainterPath QPainterPath::toReversed();
impl /*struct*/ QPainterPath {
  pub fn toReversed<RetType, T: QPainterPath_toReversed<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toReversed(self);
    // return 1;
  }
}

pub trait QPainterPath_toReversed<RetType> {
  fn toReversed(self , rsthis: &mut QPainterPath) -> RetType;
}

  // proto:  QPainterPath QPainterPath::toReversed();
impl<'a> /*trait*/ QPainterPath_toReversed<QPainterPath> for () {
  fn toReversed(self , rsthis: &mut QPainterPath) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath10toReversedEv()};
    let mut ret = unsafe {_ZNK12QPainterPath10toReversedEv(rsthis.qclsinst)};
    let mut ret1 = QPainterPath{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QPainterPath::addRoundRect(qreal x, qreal y, qreal w, qreal h, int xRnd, int yRnd);
impl<'a> /*trait*/ QPainterPath_addRoundRect<()> for (f64, f64, f64, f64, i32, i32) {
  fn addRoundRect(self , rsthis: &mut QPainterPath) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPath12addRoundRectEddddii()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let arg4 = self.4  as c_int;
    let arg5 = self.5  as c_int;
     unsafe {_ZN12QPainterPath12addRoundRectEddddii(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4, arg5)};
    // return 1;
  }
}

  // proto:  QRectF QPainterPath::boundingRect();
impl /*struct*/ QPainterPath {
  pub fn boundingRect<RetType, T: QPainterPath_boundingRect<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.boundingRect(self);
    // return 1;
  }
}

pub trait QPainterPath_boundingRect<RetType> {
  fn boundingRect(self , rsthis: &mut QPainterPath) -> RetType;
}

  // proto:  QRectF QPainterPath::boundingRect();
impl<'a> /*trait*/ QPainterPath_boundingRect<QRectF> for () {
  fn boundingRect(self , rsthis: &mut QPainterPath) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath12boundingRectEv()};
    let mut ret = unsafe {_ZNK12QPainterPath12boundingRectEv(rsthis.qclsinst)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QPainterPath::swap(QPainterPath & other);
impl /*struct*/ QPainterPath {
  pub fn swap<RetType, T: QPainterPath_swap<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QPainterPath_swap<RetType> {
  fn swap(self , rsthis: &mut QPainterPath) -> RetType;
}

  // proto:  void QPainterPath::swap(QPainterPath & other);
impl<'a> /*trait*/ QPainterPath_swap<()> for (QPainterPath) {
  fn swap(self , rsthis: &mut QPainterPath) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPath4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QPainterPath4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QPainterPath::contains(const QPainterPath & p);
impl<'a> /*trait*/ QPainterPath_contains<i8> for (QPainterPath) {
  fn contains(self , rsthis: &mut QPainterPath) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath8containsERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK12QPainterPath8containsERKS_(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QPainterPath::moveTo(qreal x, qreal y);
impl /*struct*/ QPainterPath {
  pub fn moveTo<RetType, T: QPainterPath_moveTo<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.moveTo(self);
    // return 1;
  }
}

pub trait QPainterPath_moveTo<RetType> {
  fn moveTo(self , rsthis: &mut QPainterPath) -> RetType;
}

  // proto:  void QPainterPath::moveTo(qreal x, qreal y);
impl<'a> /*trait*/ QPainterPath_moveTo<()> for (f64, f64) {
  fn moveTo(self , rsthis: &mut QPainterPath) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPath6moveToEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
     unsafe {_ZN12QPainterPath6moveToEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QPainterPath QPainterPath::subtracted(const QPainterPath & r);
impl /*struct*/ QPainterPath {
  pub fn subtracted<RetType, T: QPainterPath_subtracted<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.subtracted(self);
    // return 1;
  }
}

pub trait QPainterPath_subtracted<RetType> {
  fn subtracted(self , rsthis: &mut QPainterPath) -> RetType;
}

  // proto:  QPainterPath QPainterPath::subtracted(const QPainterPath & r);
impl<'a> /*trait*/ QPainterPath_subtracted<QPainterPath> for (QPainterPath) {
  fn subtracted(self , rsthis: &mut QPainterPath) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath10subtractedERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK12QPainterPath10subtractedERKS_(rsthis.qclsinst, arg0)};
    let mut ret1 = QPainterPath{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QPainterPath::QPainterPath();
impl<'a> /*trait*/ QPainterPath_NewQPainterPath for () {
  fn NewQPainterPath(self) -> QPainterPath {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPathC1Ev()};
    unsafe {_ZN12QPainterPathC1Ev(qthis)};
    let rsthis = QPainterPath{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QPainterPath::addText(const QPointF & point, const QFont & f, const QString & text);
impl<'a> /*trait*/ QPainterPath_addText<()> for (QPointF, QFont, QString) {
  fn addText(self , rsthis: &mut QPainterPath) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPath7addTextERK7QPointFRK5QFontRK7QString()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN12QPainterPath7addTextERK7QPointFRK5QFontRK7QString(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QPainterPath::QPainterPath(const QPainterPath & other);
impl<'a> /*trait*/ QPainterPath_NewQPainterPath for (QPainterPath) {
  fn NewQPainterPath(self) -> QPainterPath {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPathC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QPainterPathC1ERKS_(qthis, arg0)};
    let rsthis = QPainterPath{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QPointF QPainterPath::pointAtPercent(qreal t);
impl /*struct*/ QPainterPath {
  pub fn pointAtPercent<RetType, T: QPainterPath_pointAtPercent<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.pointAtPercent(self);
    // return 1;
  }
}

pub trait QPainterPath_pointAtPercent<RetType> {
  fn pointAtPercent(self , rsthis: &mut QPainterPath) -> RetType;
}

  // proto:  QPointF QPainterPath::pointAtPercent(qreal t);
impl<'a> /*trait*/ QPainterPath_pointAtPercent<QPointF> for (f64) {
  fn pointAtPercent(self , rsthis: &mut QPainterPath) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath14pointAtPercentEd()};
    let arg0 = self  as c_double;
    let mut ret = unsafe {_ZNK12QPainterPath14pointAtPercentEd(rsthis.qclsinst, arg0)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  qreal QPainterPath::percentAtLength(qreal t);
impl /*struct*/ QPainterPath {
  pub fn percentAtLength<RetType, T: QPainterPath_percentAtLength<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.percentAtLength(self);
    // return 1;
  }
}

pub trait QPainterPath_percentAtLength<RetType> {
  fn percentAtLength(self , rsthis: &mut QPainterPath) -> RetType;
}

  // proto:  qreal QPainterPath::percentAtLength(qreal t);
impl<'a> /*trait*/ QPainterPath_percentAtLength<f64> for (f64) {
  fn percentAtLength(self , rsthis: &mut QPainterPath) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath15percentAtLengthEd()};
    let arg0 = self  as c_double;
    let mut ret = unsafe {_ZNK12QPainterPath15percentAtLengthEd(rsthis.qclsinst, arg0)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QPainterPath::cubicTo(qreal ctrlPt1x, qreal ctrlPt1y, qreal ctrlPt2x, qreal ctrlPt2y, qreal endPtx, qreal endPty);
impl<'a> /*trait*/ QPainterPath_cubicTo<()> for (f64, f64, f64, f64, f64, f64) {
  fn cubicTo(self , rsthis: &mut QPainterPath) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPath7cubicToEdddddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let arg4 = self.4  as c_double;
    let arg5 = self.5  as c_double;
     unsafe {_ZN12QPainterPath7cubicToEdddddd(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4, arg5)};
    // return 1;
  }
}

  // proto:  void QPainterPath::lineTo(const QPointF & p);
impl<'a> /*trait*/ QPainterPath_lineTo<()> for (QPointF) {
  fn lineTo(self , rsthis: &mut QPainterPath) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPath6lineToERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QPainterPath6lineToERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QPainterPath QPainterPath::subtractedInverted(const QPainterPath & r);
impl /*struct*/ QPainterPath {
  pub fn subtractedInverted<RetType, T: QPainterPath_subtractedInverted<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.subtractedInverted(self);
    // return 1;
  }
}

pub trait QPainterPath_subtractedInverted<RetType> {
  fn subtractedInverted(self , rsthis: &mut QPainterPath) -> RetType;
}

  // proto:  QPainterPath QPainterPath::subtractedInverted(const QPainterPath & r);
impl<'a> /*trait*/ QPainterPath_subtractedInverted<QPainterPath> for (QPainterPath) {
  fn subtractedInverted(self , rsthis: &mut QPainterPath) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath18subtractedInvertedERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK12QPainterPath18subtractedInvertedERKS_(rsthis.qclsinst, arg0)};
    let mut ret1 = QPainterPath{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QPainterPath::arcMoveTo(qreal x, qreal y, qreal w, qreal h, qreal angle);
impl /*struct*/ QPainterPath {
  pub fn arcMoveTo<RetType, T: QPainterPath_arcMoveTo<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.arcMoveTo(self);
    // return 1;
  }
}

pub trait QPainterPath_arcMoveTo<RetType> {
  fn arcMoveTo(self , rsthis: &mut QPainterPath) -> RetType;
}

  // proto:  void QPainterPath::arcMoveTo(qreal x, qreal y, qreal w, qreal h, qreal angle);
impl<'a> /*trait*/ QPainterPath_arcMoveTo<()> for (f64, f64, f64, f64, f64) {
  fn arcMoveTo(self , rsthis: &mut QPainterPath) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPath9arcMoveToEddddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
    let arg4 = self.4  as c_double;
     unsafe {_ZN12QPainterPath9arcMoveToEddddd(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4)};
    // return 1;
  }
}

  // proto:  bool QPainterPath::isEmpty();
impl /*struct*/ QPainterPath {
  pub fn isEmpty<RetType, T: QPainterPath_isEmpty<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isEmpty(self);
    // return 1;
  }
}

pub trait QPainterPath_isEmpty<RetType> {
  fn isEmpty(self , rsthis: &mut QPainterPath) -> RetType;
}

  // proto:  bool QPainterPath::isEmpty();
impl<'a> /*trait*/ QPainterPath_isEmpty<i8> for () {
  fn isEmpty(self , rsthis: &mut QPainterPath) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath7isEmptyEv()};
    let mut ret = unsafe {_ZNK12QPainterPath7isEmptyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QPainterPath::addRect(qreal x, qreal y, qreal w, qreal h);
impl<'a> /*trait*/ QPainterPath_addRect<()> for (f64, f64, f64, f64) {
  fn addRect(self , rsthis: &mut QPainterPath) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPath7addRectEdddd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    let arg2 = self.2  as c_double;
    let arg3 = self.3  as c_double;
     unsafe {_ZN12QPainterPath7addRectEdddd(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  void QPainterPath::arcMoveTo(const QRectF & rect, qreal angle);
impl<'a> /*trait*/ QPainterPath_arcMoveTo<()> for (QRectF, f64) {
  fn arcMoveTo(self , rsthis: &mut QPainterPath) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPath9arcMoveToERK6QRectFd()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_double;
     unsafe {_ZN12QPainterPath9arcMoveToERK6QRectFd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QList<QPolygonF> QPainterPath::toSubpathPolygons(const QMatrix & matrix);
impl<'a> /*trait*/ QPainterPath_toSubpathPolygons<()> for (QMatrix) {
  fn toSubpathPolygons(self , rsthis: &mut QPainterPath) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath17toSubpathPolygonsERK7QMatrix()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZNK12QPainterPath17toSubpathPolygonsERK7QMatrix(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QPainterPath QPainterPath::united(const QPainterPath & r);
impl /*struct*/ QPainterPath {
  pub fn united<RetType, T: QPainterPath_united<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.united(self);
    // return 1;
  }
}

pub trait QPainterPath_united<RetType> {
  fn united(self , rsthis: &mut QPainterPath) -> RetType;
}

  // proto:  QPainterPath QPainterPath::united(const QPainterPath & r);
impl<'a> /*trait*/ QPainterPath_united<QPainterPath> for (QPainterPath) {
  fn united(self , rsthis: &mut QPainterPath) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath6unitedERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK12QPainterPath6unitedERKS_(rsthis.qclsinst, arg0)};
    let mut ret1 = QPainterPath{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QPainterPath::addEllipse(const QRectF & rect);
impl<'a> /*trait*/ QPainterPath_addEllipse<()> for (QRectF) {
  fn addEllipse(self , rsthis: &mut QPainterPath) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPath10addEllipseERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QPainterPath10addEllipseERK6QRectF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QPainterPath::moveTo(const QPointF & p);
impl<'a> /*trait*/ QPainterPath_moveTo<()> for (QPointF) {
  fn moveTo(self , rsthis: &mut QPainterPath) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPath6moveToERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QPainterPath6moveToERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  qreal QPainterPath::angleAtPercent(qreal t);
impl /*struct*/ QPainterPath {
  pub fn angleAtPercent<RetType, T: QPainterPath_angleAtPercent<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.angleAtPercent(self);
    // return 1;
  }
}

pub trait QPainterPath_angleAtPercent<RetType> {
  fn angleAtPercent(self , rsthis: &mut QPainterPath) -> RetType;
}

  // proto:  qreal QPainterPath::angleAtPercent(qreal t);
impl<'a> /*trait*/ QPainterPath_angleAtPercent<f64> for (f64) {
  fn angleAtPercent(self , rsthis: &mut QPainterPath) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QPainterPath14angleAtPercentEd()};
    let arg0 = self  as c_double;
    let mut ret = unsafe {_ZNK12QPainterPath14angleAtPercentEd(rsthis.qclsinst, arg0)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QPainterPath::closeSubpath();
impl /*struct*/ QPainterPath {
  pub fn closeSubpath<RetType, T: QPainterPath_closeSubpath<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.closeSubpath(self);
    // return 1;
  }
}

pub trait QPainterPath_closeSubpath<RetType> {
  fn closeSubpath(self , rsthis: &mut QPainterPath) -> RetType;
}

  // proto:  void QPainterPath::closeSubpath();
impl<'a> /*trait*/ QPainterPath_closeSubpath<()> for () {
  fn closeSubpath(self , rsthis: &mut QPainterPath) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QPainterPath12closeSubpathEv()};
     unsafe {_ZN12QPainterPath12closeSubpathEv(rsthis.qclsinst)};
    // return 1;
  }
}

