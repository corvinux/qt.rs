// auto generated, do not modify.
// created: Mon Dec 21 22:54:38 2015
// src-file: /QtWidgets/qgraphicseffect.h
// dst-file: /src/widgets/qgraphicseffect.rs
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
use super::super::gui::qcolor::QColor; // 771
use super::super::core::qobject::QObject; // 771
use super::super::core::qrect::QRectF; // 771
use super::super::core::qpoint::QPointF; // 771
use super::super::gui::qbrush::QBrush; // 771
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  void QGraphicsColorizeEffect::setColor(const QColor & c);
  fn _ZN23QGraphicsColorizeEffect8setColorERK6QColor(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QGraphicsColorizeEffect::strengthChanged(qreal strength);
  fn _ZN23QGraphicsColorizeEffect15strengthChangedEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QGraphicsColorizeEffect::QGraphicsColorizeEffect(const QGraphicsColorizeEffect & );
  fn _ZN23QGraphicsColorizeEffectC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QGraphicsColorizeEffect::setStrength(qreal strength);
  fn _ZN23QGraphicsColorizeEffect11setStrengthEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QGraphicsColorizeEffect::QGraphicsColorizeEffect(QObject * parent);
  fn _ZN23QGraphicsColorizeEffectC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  qreal QGraphicsColorizeEffect::strength();
  fn _ZNK23QGraphicsColorizeEffect8strengthEv(qthis: *mut c_void) -> c_double;
  // proto:  void QGraphicsColorizeEffect::~QGraphicsColorizeEffect();
  fn _ZN23QGraphicsColorizeEffectD0Ev(qthis: *mut c_void);
  // proto:  void QGraphicsColorizeEffect::colorChanged(const QColor & color);
  fn _ZN23QGraphicsColorizeEffect12colorChangedERK6QColor(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QColor QGraphicsColorizeEffect::color();
  fn _ZNK23QGraphicsColorizeEffect5colorEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  const QMetaObject * QGraphicsColorizeEffect::metaObject();
  fn _ZNK23QGraphicsColorizeEffect10metaObjectEv(qthis: *mut c_void);
  // proto:  QRectF QGraphicsEffect::boundingRectFor(const QRectF & sourceRect);
  fn _ZNK15QGraphicsEffect15boundingRectForERK6QRectF(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QGraphicsEffectSource * QGraphicsEffect::source();
  fn _ZNK15QGraphicsEffect6sourceEv(qthis: *mut c_void);
  // proto:  void QGraphicsEffect::update();
  fn _ZN15QGraphicsEffect6updateEv(qthis: *mut c_void);
  // proto:  void QGraphicsEffect::setEnabled(bool enable);
  fn _ZN15QGraphicsEffect10setEnabledEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QGraphicsEffect::enabledChanged(bool enabled);
  fn _ZN15QGraphicsEffect14enabledChangedEb(qthis: *mut c_void, arg0: c_char);
  // proto:  const QMetaObject * QGraphicsEffect::metaObject();
  fn _ZNK15QGraphicsEffect10metaObjectEv(qthis: *mut c_void);
  // proto:  bool QGraphicsEffect::isEnabled();
  fn _ZNK15QGraphicsEffect9isEnabledEv(qthis: *mut c_void) -> c_char;
  // proto:  QRectF QGraphicsEffect::boundingRect();
  fn _ZNK15QGraphicsEffect12boundingRectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsEffect::~QGraphicsEffect();
  fn _ZN15QGraphicsEffectD0Ev(qthis: *mut c_void);
  // proto:  void QGraphicsEffect::QGraphicsEffect(const QGraphicsEffect & );
  fn _ZN15QGraphicsEffectC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QGraphicsEffect::QGraphicsEffect(QObject * parent);
  fn _ZN15QGraphicsEffectC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QGraphicsDropShadowEffect::blurRadiusChanged(qreal blurRadius);
  fn _ZN25QGraphicsDropShadowEffect17blurRadiusChangedEd(qthis: *mut c_void, arg0: c_double);
  // proto:  const QMetaObject * QGraphicsDropShadowEffect::metaObject();
  fn _ZNK25QGraphicsDropShadowEffect10metaObjectEv(qthis: *mut c_void);
  // proto:  QRectF QGraphicsDropShadowEffect::boundingRectFor(const QRectF & rect);
  fn _ZNK25QGraphicsDropShadowEffect15boundingRectForERK6QRectF(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsDropShadowEffect::QGraphicsDropShadowEffect(QObject * parent);
  fn _ZN25QGraphicsDropShadowEffectC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QPointF QGraphicsDropShadowEffect::offset();
  fn _ZNK25QGraphicsDropShadowEffect6offsetEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsDropShadowEffect::setYOffset(qreal dy);
  fn _ZN25QGraphicsDropShadowEffect10setYOffsetEd(qthis: *mut c_void, arg0: c_double);
  // proto:  qreal QGraphicsDropShadowEffect::xOffset();
  fn _ZNK25QGraphicsDropShadowEffect7xOffsetEv(qthis: *mut c_void) -> c_double;
  // proto:  qreal QGraphicsDropShadowEffect::blurRadius();
  fn _ZNK25QGraphicsDropShadowEffect10blurRadiusEv(qthis: *mut c_void) -> c_double;
  // proto:  QColor QGraphicsDropShadowEffect::color();
  fn _ZNK25QGraphicsDropShadowEffect5colorEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsDropShadowEffect::setColor(const QColor & color);
  fn _ZN25QGraphicsDropShadowEffect8setColorERK6QColor(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QGraphicsDropShadowEffect::setOffset(const QPointF & ofs);
  fn _ZN25QGraphicsDropShadowEffect9setOffsetERK7QPointF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QGraphicsDropShadowEffect::offsetChanged(const QPointF & offset);
  fn _ZN25QGraphicsDropShadowEffect13offsetChangedERK7QPointF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QGraphicsDropShadowEffect::setOffset(qreal dx, qreal dy);
  fn _ZN25QGraphicsDropShadowEffect9setOffsetEdd(qthis: *mut c_void, arg0: c_double, arg1: c_double);
  // proto:  void QGraphicsDropShadowEffect::setOffset(qreal d);
  fn _ZN25QGraphicsDropShadowEffect9setOffsetEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QGraphicsDropShadowEffect::QGraphicsDropShadowEffect(const QGraphicsDropShadowEffect & );
  fn _ZN25QGraphicsDropShadowEffectC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QGraphicsDropShadowEffect::colorChanged(const QColor & color);
  fn _ZN25QGraphicsDropShadowEffect12colorChangedERK6QColor(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  qreal QGraphicsDropShadowEffect::yOffset();
  fn _ZNK25QGraphicsDropShadowEffect7yOffsetEv(qthis: *mut c_void) -> c_double;
  // proto:  void QGraphicsDropShadowEffect::setXOffset(qreal dx);
  fn _ZN25QGraphicsDropShadowEffect10setXOffsetEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QGraphicsDropShadowEffect::setBlurRadius(qreal blurRadius);
  fn _ZN25QGraphicsDropShadowEffect13setBlurRadiusEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QGraphicsDropShadowEffect::~QGraphicsDropShadowEffect();
  fn _ZN25QGraphicsDropShadowEffectD0Ev(qthis: *mut c_void);
  // proto:  void QGraphicsOpacityEffect::QGraphicsOpacityEffect(QObject * parent);
  fn _ZN22QGraphicsOpacityEffectC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QGraphicsOpacityEffect::~QGraphicsOpacityEffect();
  fn _ZN22QGraphicsOpacityEffectD0Ev(qthis: *mut c_void);
  // proto:  void QGraphicsOpacityEffect::setOpacityMask(const QBrush & mask);
  fn _ZN22QGraphicsOpacityEffect14setOpacityMaskERK6QBrush(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QGraphicsOpacityEffect::QGraphicsOpacityEffect(const QGraphicsOpacityEffect & );
  fn _ZN22QGraphicsOpacityEffectC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QBrush QGraphicsOpacityEffect::opacityMask();
  fn _ZNK22QGraphicsOpacityEffect11opacityMaskEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsOpacityEffect::opacityChanged(qreal opacity);
  fn _ZN22QGraphicsOpacityEffect14opacityChangedEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QGraphicsOpacityEffect::opacityMaskChanged(const QBrush & mask);
  fn _ZN22QGraphicsOpacityEffect18opacityMaskChangedERK6QBrush(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  const QMetaObject * QGraphicsOpacityEffect::metaObject();
  fn _ZNK22QGraphicsOpacityEffect10metaObjectEv(qthis: *mut c_void);
  // proto:  qreal QGraphicsOpacityEffect::opacity();
  fn _ZNK22QGraphicsOpacityEffect7opacityEv(qthis: *mut c_void) -> c_double;
  // proto:  void QGraphicsOpacityEffect::setOpacity(qreal opacity);
  fn _ZN22QGraphicsOpacityEffect10setOpacityEd(qthis: *mut c_void, arg0: c_double);
  // proto:  qreal QGraphicsBlurEffect::blurRadius();
  fn _ZNK19QGraphicsBlurEffect10blurRadiusEv(qthis: *mut c_void) -> c_double;
  // proto:  void QGraphicsBlurEffect::setBlurRadius(qreal blurRadius);
  fn _ZN19QGraphicsBlurEffect13setBlurRadiusEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QGraphicsBlurEffect::~QGraphicsBlurEffect();
  fn _ZN19QGraphicsBlurEffectD0Ev(qthis: *mut c_void);
  // proto:  void QGraphicsBlurEffect::QGraphicsBlurEffect(const QGraphicsBlurEffect & );
  fn _ZN19QGraphicsBlurEffectC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  const QMetaObject * QGraphicsBlurEffect::metaObject();
  fn _ZNK19QGraphicsBlurEffect10metaObjectEv(qthis: *mut c_void);
  // proto:  QRectF QGraphicsBlurEffect::boundingRectFor(const QRectF & rect);
  fn _ZNK19QGraphicsBlurEffect15boundingRectForERK6QRectF(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsBlurEffect::QGraphicsBlurEffect(QObject * parent);
  fn _ZN19QGraphicsBlurEffectC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QGraphicsBlurEffect::blurRadiusChanged(qreal blurRadius);
  fn _ZN19QGraphicsBlurEffect17blurRadiusChangedEd(qthis: *mut c_void, arg0: c_double);
} // <= ext block end

// body block begin =>
// class sizeof(QGraphicsColorizeEffect)=1
pub struct QGraphicsColorizeEffect {
  pub qclsinst: *mut c_void,
}

// class sizeof(QGraphicsEffect)=1
pub struct QGraphicsEffect {
  pub qclsinst: *mut c_void,
}

// class sizeof(QGraphicsDropShadowEffect)=1
pub struct QGraphicsDropShadowEffect {
  pub qclsinst: *mut c_void,
}

// class sizeof(QGraphicsOpacityEffect)=1
pub struct QGraphicsOpacityEffect {
  pub qclsinst: *mut c_void,
}

// class sizeof(QGraphicsBlurEffect)=1
pub struct QGraphicsBlurEffect {
  pub qclsinst: *mut c_void,
}

  // proto:  void QGraphicsColorizeEffect::setColor(const QColor & c);
impl /*struct*/ QGraphicsColorizeEffect {
  pub fn setColor<RetType, T: QGraphicsColorizeEffect_setColor<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setColor(self);
    // return 1;
  }
}

pub trait QGraphicsColorizeEffect_setColor<RetType> {
  fn setColor(self , rsthis: &mut QGraphicsColorizeEffect) -> RetType;
}

  // proto:  void QGraphicsColorizeEffect::setColor(const QColor & c);
impl<'a> /*trait*/ QGraphicsColorizeEffect_setColor<()> for (QColor) {
  fn setColor(self , rsthis: &mut QGraphicsColorizeEffect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QGraphicsColorizeEffect8setColorERK6QColor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN23QGraphicsColorizeEffect8setColorERK6QColor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsColorizeEffect::strengthChanged(qreal strength);
impl /*struct*/ QGraphicsColorizeEffect {
  pub fn strengthChanged<RetType, T: QGraphicsColorizeEffect_strengthChanged<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.strengthChanged(self);
    // return 1;
  }
}

pub trait QGraphicsColorizeEffect_strengthChanged<RetType> {
  fn strengthChanged(self , rsthis: &mut QGraphicsColorizeEffect) -> RetType;
}

  // proto:  void QGraphicsColorizeEffect::strengthChanged(qreal strength);
impl<'a> /*trait*/ QGraphicsColorizeEffect_strengthChanged<()> for (f64) {
  fn strengthChanged(self , rsthis: &mut QGraphicsColorizeEffect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QGraphicsColorizeEffect15strengthChangedEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN23QGraphicsColorizeEffect15strengthChangedEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsColorizeEffect::QGraphicsColorizeEffect(const QGraphicsColorizeEffect & );
impl /*struct*/ QGraphicsColorizeEffect {
  pub fn NewQGraphicsColorizeEffect<T: QGraphicsColorizeEffect_NewQGraphicsColorizeEffect>(value: T) -> QGraphicsColorizeEffect {
    let rsthis = value.NewQGraphicsColorizeEffect();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsColorizeEffect_NewQGraphicsColorizeEffect {
  fn NewQGraphicsColorizeEffect(self) -> QGraphicsColorizeEffect;
}

  // proto:  void QGraphicsColorizeEffect::QGraphicsColorizeEffect(const QGraphicsColorizeEffect & );
impl<'a> /*trait*/ QGraphicsColorizeEffect_NewQGraphicsColorizeEffect for (QGraphicsColorizeEffect) {
  fn NewQGraphicsColorizeEffect(self) -> QGraphicsColorizeEffect {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QGraphicsColorizeEffectC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN23QGraphicsColorizeEffectC1ERKS_(qthis, arg0)};
    let rsthis = QGraphicsColorizeEffect{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QGraphicsColorizeEffect::setStrength(qreal strength);
impl /*struct*/ QGraphicsColorizeEffect {
  pub fn setStrength<RetType, T: QGraphicsColorizeEffect_setStrength<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setStrength(self);
    // return 1;
  }
}

pub trait QGraphicsColorizeEffect_setStrength<RetType> {
  fn setStrength(self , rsthis: &mut QGraphicsColorizeEffect) -> RetType;
}

  // proto:  void QGraphicsColorizeEffect::setStrength(qreal strength);
impl<'a> /*trait*/ QGraphicsColorizeEffect_setStrength<()> for (f64) {
  fn setStrength(self , rsthis: &mut QGraphicsColorizeEffect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QGraphicsColorizeEffect11setStrengthEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN23QGraphicsColorizeEffect11setStrengthEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsColorizeEffect::QGraphicsColorizeEffect(QObject * parent);
impl<'a> /*trait*/ QGraphicsColorizeEffect_NewQGraphicsColorizeEffect for (QObject) {
  fn NewQGraphicsColorizeEffect(self) -> QGraphicsColorizeEffect {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QGraphicsColorizeEffectC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN23QGraphicsColorizeEffectC1EP7QObject(qthis, arg0)};
    let rsthis = QGraphicsColorizeEffect{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  qreal QGraphicsColorizeEffect::strength();
impl /*struct*/ QGraphicsColorizeEffect {
  pub fn strength<RetType, T: QGraphicsColorizeEffect_strength<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.strength(self);
    // return 1;
  }
}

pub trait QGraphicsColorizeEffect_strength<RetType> {
  fn strength(self , rsthis: &mut QGraphicsColorizeEffect) -> RetType;
}

  // proto:  qreal QGraphicsColorizeEffect::strength();
impl<'a> /*trait*/ QGraphicsColorizeEffect_strength<f64> for () {
  fn strength(self , rsthis: &mut QGraphicsColorizeEffect) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QGraphicsColorizeEffect8strengthEv()};
    let mut ret = unsafe {_ZNK23QGraphicsColorizeEffect8strengthEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QGraphicsColorizeEffect::~QGraphicsColorizeEffect();
impl /*struct*/ QGraphicsColorizeEffect {
  pub fn FreeQGraphicsColorizeEffect<RetType, T: QGraphicsColorizeEffect_FreeQGraphicsColorizeEffect<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQGraphicsColorizeEffect(self);
    // return 1;
  }
}

pub trait QGraphicsColorizeEffect_FreeQGraphicsColorizeEffect<RetType> {
  fn FreeQGraphicsColorizeEffect(self , rsthis: &mut QGraphicsColorizeEffect) -> RetType;
}

  // proto:  void QGraphicsColorizeEffect::~QGraphicsColorizeEffect();
impl<'a> /*trait*/ QGraphicsColorizeEffect_FreeQGraphicsColorizeEffect<()> for () {
  fn FreeQGraphicsColorizeEffect(self , rsthis: &mut QGraphicsColorizeEffect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QGraphicsColorizeEffectD0Ev()};
     unsafe {_ZN23QGraphicsColorizeEffectD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsColorizeEffect::colorChanged(const QColor & color);
impl /*struct*/ QGraphicsColorizeEffect {
  pub fn colorChanged<RetType, T: QGraphicsColorizeEffect_colorChanged<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.colorChanged(self);
    // return 1;
  }
}

pub trait QGraphicsColorizeEffect_colorChanged<RetType> {
  fn colorChanged(self , rsthis: &mut QGraphicsColorizeEffect) -> RetType;
}

  // proto:  void QGraphicsColorizeEffect::colorChanged(const QColor & color);
impl<'a> /*trait*/ QGraphicsColorizeEffect_colorChanged<()> for (QColor) {
  fn colorChanged(self , rsthis: &mut QGraphicsColorizeEffect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QGraphicsColorizeEffect12colorChangedERK6QColor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN23QGraphicsColorizeEffect12colorChangedERK6QColor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QColor QGraphicsColorizeEffect::color();
impl /*struct*/ QGraphicsColorizeEffect {
  pub fn color<RetType, T: QGraphicsColorizeEffect_color<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.color(self);
    // return 1;
  }
}

pub trait QGraphicsColorizeEffect_color<RetType> {
  fn color(self , rsthis: &mut QGraphicsColorizeEffect) -> RetType;
}

  // proto:  QColor QGraphicsColorizeEffect::color();
impl<'a> /*trait*/ QGraphicsColorizeEffect_color<QColor> for () {
  fn color(self , rsthis: &mut QGraphicsColorizeEffect) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QGraphicsColorizeEffect5colorEv()};
    let mut ret = unsafe {_ZNK23QGraphicsColorizeEffect5colorEv(rsthis.qclsinst)};
    let mut ret1 = QColor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  const QMetaObject * QGraphicsColorizeEffect::metaObject();
impl /*struct*/ QGraphicsColorizeEffect {
  pub fn metaObject<RetType, T: QGraphicsColorizeEffect_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QGraphicsColorizeEffect_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QGraphicsColorizeEffect) -> RetType;
}

  // proto:  const QMetaObject * QGraphicsColorizeEffect::metaObject();
impl<'a> /*trait*/ QGraphicsColorizeEffect_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QGraphicsColorizeEffect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QGraphicsColorizeEffect10metaObjectEv()};
     unsafe {_ZNK23QGraphicsColorizeEffect10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QRectF QGraphicsEffect::boundingRectFor(const QRectF & sourceRect);
impl /*struct*/ QGraphicsEffect {
  pub fn boundingRectFor<RetType, T: QGraphicsEffect_boundingRectFor<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.boundingRectFor(self);
    // return 1;
  }
}

pub trait QGraphicsEffect_boundingRectFor<RetType> {
  fn boundingRectFor(self , rsthis: &mut QGraphicsEffect) -> RetType;
}

  // proto:  QRectF QGraphicsEffect::boundingRectFor(const QRectF & sourceRect);
impl<'a> /*trait*/ QGraphicsEffect_boundingRectFor<QRectF> for (QRectF) {
  fn boundingRectFor(self , rsthis: &mut QGraphicsEffect) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsEffect15boundingRectForERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK15QGraphicsEffect15boundingRectForERK6QRectF(rsthis.qclsinst, arg0)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QGraphicsEffectSource * QGraphicsEffect::source();
impl /*struct*/ QGraphicsEffect {
  pub fn source<RetType, T: QGraphicsEffect_source<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.source(self);
    // return 1;
  }
}

pub trait QGraphicsEffect_source<RetType> {
  fn source(self , rsthis: &mut QGraphicsEffect) -> RetType;
}

  // proto:  QGraphicsEffectSource * QGraphicsEffect::source();
impl<'a> /*trait*/ QGraphicsEffect_source<()> for () {
  fn source(self , rsthis: &mut QGraphicsEffect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsEffect6sourceEv()};
     unsafe {_ZNK15QGraphicsEffect6sourceEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsEffect::update();
impl /*struct*/ QGraphicsEffect {
  pub fn update<RetType, T: QGraphicsEffect_update<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.update(self);
    // return 1;
  }
}

pub trait QGraphicsEffect_update<RetType> {
  fn update(self , rsthis: &mut QGraphicsEffect) -> RetType;
}

  // proto:  void QGraphicsEffect::update();
impl<'a> /*trait*/ QGraphicsEffect_update<()> for () {
  fn update(self , rsthis: &mut QGraphicsEffect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsEffect6updateEv()};
     unsafe {_ZN15QGraphicsEffect6updateEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsEffect::setEnabled(bool enable);
impl /*struct*/ QGraphicsEffect {
  pub fn setEnabled<RetType, T: QGraphicsEffect_setEnabled<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setEnabled(self);
    // return 1;
  }
}

pub trait QGraphicsEffect_setEnabled<RetType> {
  fn setEnabled(self , rsthis: &mut QGraphicsEffect) -> RetType;
}

  // proto:  void QGraphicsEffect::setEnabled(bool enable);
impl<'a> /*trait*/ QGraphicsEffect_setEnabled<()> for (i8) {
  fn setEnabled(self , rsthis: &mut QGraphicsEffect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsEffect10setEnabledEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN15QGraphicsEffect10setEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsEffect::enabledChanged(bool enabled);
impl /*struct*/ QGraphicsEffect {
  pub fn enabledChanged<RetType, T: QGraphicsEffect_enabledChanged<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.enabledChanged(self);
    // return 1;
  }
}

pub trait QGraphicsEffect_enabledChanged<RetType> {
  fn enabledChanged(self , rsthis: &mut QGraphicsEffect) -> RetType;
}

  // proto:  void QGraphicsEffect::enabledChanged(bool enabled);
impl<'a> /*trait*/ QGraphicsEffect_enabledChanged<()> for (i8) {
  fn enabledChanged(self , rsthis: &mut QGraphicsEffect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsEffect14enabledChangedEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN15QGraphicsEffect14enabledChangedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QGraphicsEffect::metaObject();
impl /*struct*/ QGraphicsEffect {
  pub fn metaObject<RetType, T: QGraphicsEffect_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QGraphicsEffect_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QGraphicsEffect) -> RetType;
}

  // proto:  const QMetaObject * QGraphicsEffect::metaObject();
impl<'a> /*trait*/ QGraphicsEffect_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QGraphicsEffect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsEffect10metaObjectEv()};
     unsafe {_ZNK15QGraphicsEffect10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QGraphicsEffect::isEnabled();
impl /*struct*/ QGraphicsEffect {
  pub fn isEnabled<RetType, T: QGraphicsEffect_isEnabled<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isEnabled(self);
    // return 1;
  }
}

pub trait QGraphicsEffect_isEnabled<RetType> {
  fn isEnabled(self , rsthis: &mut QGraphicsEffect) -> RetType;
}

  // proto:  bool QGraphicsEffect::isEnabled();
impl<'a> /*trait*/ QGraphicsEffect_isEnabled<i8> for () {
  fn isEnabled(self , rsthis: &mut QGraphicsEffect) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsEffect9isEnabledEv()};
    let mut ret = unsafe {_ZNK15QGraphicsEffect9isEnabledEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QRectF QGraphicsEffect::boundingRect();
impl /*struct*/ QGraphicsEffect {
  pub fn boundingRect<RetType, T: QGraphicsEffect_boundingRect<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.boundingRect(self);
    // return 1;
  }
}

pub trait QGraphicsEffect_boundingRect<RetType> {
  fn boundingRect(self , rsthis: &mut QGraphicsEffect) -> RetType;
}

  // proto:  QRectF QGraphicsEffect::boundingRect();
impl<'a> /*trait*/ QGraphicsEffect_boundingRect<QRectF> for () {
  fn boundingRect(self , rsthis: &mut QGraphicsEffect) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK15QGraphicsEffect12boundingRectEv()};
    let mut ret = unsafe {_ZNK15QGraphicsEffect12boundingRectEv(rsthis.qclsinst)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsEffect::~QGraphicsEffect();
impl /*struct*/ QGraphicsEffect {
  pub fn FreeQGraphicsEffect<RetType, T: QGraphicsEffect_FreeQGraphicsEffect<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQGraphicsEffect(self);
    // return 1;
  }
}

pub trait QGraphicsEffect_FreeQGraphicsEffect<RetType> {
  fn FreeQGraphicsEffect(self , rsthis: &mut QGraphicsEffect) -> RetType;
}

  // proto:  void QGraphicsEffect::~QGraphicsEffect();
impl<'a> /*trait*/ QGraphicsEffect_FreeQGraphicsEffect<()> for () {
  fn FreeQGraphicsEffect(self , rsthis: &mut QGraphicsEffect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsEffectD0Ev()};
     unsafe {_ZN15QGraphicsEffectD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsEffect::QGraphicsEffect(const QGraphicsEffect & );
impl /*struct*/ QGraphicsEffect {
  pub fn NewQGraphicsEffect<T: QGraphicsEffect_NewQGraphicsEffect>(value: T) -> QGraphicsEffect {
    let rsthis = value.NewQGraphicsEffect();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsEffect_NewQGraphicsEffect {
  fn NewQGraphicsEffect(self) -> QGraphicsEffect;
}

  // proto:  void QGraphicsEffect::QGraphicsEffect(const QGraphicsEffect & );
impl<'a> /*trait*/ QGraphicsEffect_NewQGraphicsEffect for (QGraphicsEffect) {
  fn NewQGraphicsEffect(self) -> QGraphicsEffect {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsEffectC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN15QGraphicsEffectC1ERKS_(qthis, arg0)};
    let rsthis = QGraphicsEffect{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QGraphicsEffect::QGraphicsEffect(QObject * parent);
impl<'a> /*trait*/ QGraphicsEffect_NewQGraphicsEffect for (QObject) {
  fn NewQGraphicsEffect(self) -> QGraphicsEffect {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN15QGraphicsEffectC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN15QGraphicsEffectC1EP7QObject(qthis, arg0)};
    let rsthis = QGraphicsEffect{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QGraphicsDropShadowEffect::blurRadiusChanged(qreal blurRadius);
impl /*struct*/ QGraphicsDropShadowEffect {
  pub fn blurRadiusChanged<RetType, T: QGraphicsDropShadowEffect_blurRadiusChanged<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.blurRadiusChanged(self);
    // return 1;
  }
}

pub trait QGraphicsDropShadowEffect_blurRadiusChanged<RetType> {
  fn blurRadiusChanged(self , rsthis: &mut QGraphicsDropShadowEffect) -> RetType;
}

  // proto:  void QGraphicsDropShadowEffect::blurRadiusChanged(qreal blurRadius);
impl<'a> /*trait*/ QGraphicsDropShadowEffect_blurRadiusChanged<()> for (f64) {
  fn blurRadiusChanged(self , rsthis: &mut QGraphicsDropShadowEffect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QGraphicsDropShadowEffect17blurRadiusChangedEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN25QGraphicsDropShadowEffect17blurRadiusChangedEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QGraphicsDropShadowEffect::metaObject();
impl /*struct*/ QGraphicsDropShadowEffect {
  pub fn metaObject<RetType, T: QGraphicsDropShadowEffect_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QGraphicsDropShadowEffect_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QGraphicsDropShadowEffect) -> RetType;
}

  // proto:  const QMetaObject * QGraphicsDropShadowEffect::metaObject();
impl<'a> /*trait*/ QGraphicsDropShadowEffect_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QGraphicsDropShadowEffect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QGraphicsDropShadowEffect10metaObjectEv()};
     unsafe {_ZNK25QGraphicsDropShadowEffect10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QRectF QGraphicsDropShadowEffect::boundingRectFor(const QRectF & rect);
impl /*struct*/ QGraphicsDropShadowEffect {
  pub fn boundingRectFor<RetType, T: QGraphicsDropShadowEffect_boundingRectFor<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.boundingRectFor(self);
    // return 1;
  }
}

pub trait QGraphicsDropShadowEffect_boundingRectFor<RetType> {
  fn boundingRectFor(self , rsthis: &mut QGraphicsDropShadowEffect) -> RetType;
}

  // proto:  QRectF QGraphicsDropShadowEffect::boundingRectFor(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsDropShadowEffect_boundingRectFor<QRectF> for (QRectF) {
  fn boundingRectFor(self , rsthis: &mut QGraphicsDropShadowEffect) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QGraphicsDropShadowEffect15boundingRectForERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK25QGraphicsDropShadowEffect15boundingRectForERK6QRectF(rsthis.qclsinst, arg0)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsDropShadowEffect::QGraphicsDropShadowEffect(QObject * parent);
impl /*struct*/ QGraphicsDropShadowEffect {
  pub fn NewQGraphicsDropShadowEffect<T: QGraphicsDropShadowEffect_NewQGraphicsDropShadowEffect>(value: T) -> QGraphicsDropShadowEffect {
    let rsthis = value.NewQGraphicsDropShadowEffect();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsDropShadowEffect_NewQGraphicsDropShadowEffect {
  fn NewQGraphicsDropShadowEffect(self) -> QGraphicsDropShadowEffect;
}

  // proto:  void QGraphicsDropShadowEffect::QGraphicsDropShadowEffect(QObject * parent);
impl<'a> /*trait*/ QGraphicsDropShadowEffect_NewQGraphicsDropShadowEffect for (QObject) {
  fn NewQGraphicsDropShadowEffect(self) -> QGraphicsDropShadowEffect {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QGraphicsDropShadowEffectC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN25QGraphicsDropShadowEffectC1EP7QObject(qthis, arg0)};
    let rsthis = QGraphicsDropShadowEffect{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QPointF QGraphicsDropShadowEffect::offset();
impl /*struct*/ QGraphicsDropShadowEffect {
  pub fn offset<RetType, T: QGraphicsDropShadowEffect_offset<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.offset(self);
    // return 1;
  }
}

pub trait QGraphicsDropShadowEffect_offset<RetType> {
  fn offset(self , rsthis: &mut QGraphicsDropShadowEffect) -> RetType;
}

  // proto:  QPointF QGraphicsDropShadowEffect::offset();
impl<'a> /*trait*/ QGraphicsDropShadowEffect_offset<QPointF> for () {
  fn offset(self , rsthis: &mut QGraphicsDropShadowEffect) -> QPointF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QGraphicsDropShadowEffect6offsetEv()};
    let mut ret = unsafe {_ZNK25QGraphicsDropShadowEffect6offsetEv(rsthis.qclsinst)};
    let mut ret1 = QPointF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsDropShadowEffect::setYOffset(qreal dy);
impl /*struct*/ QGraphicsDropShadowEffect {
  pub fn setYOffset<RetType, T: QGraphicsDropShadowEffect_setYOffset<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setYOffset(self);
    // return 1;
  }
}

pub trait QGraphicsDropShadowEffect_setYOffset<RetType> {
  fn setYOffset(self , rsthis: &mut QGraphicsDropShadowEffect) -> RetType;
}

  // proto:  void QGraphicsDropShadowEffect::setYOffset(qreal dy);
impl<'a> /*trait*/ QGraphicsDropShadowEffect_setYOffset<()> for (f64) {
  fn setYOffset(self , rsthis: &mut QGraphicsDropShadowEffect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QGraphicsDropShadowEffect10setYOffsetEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN25QGraphicsDropShadowEffect10setYOffsetEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  qreal QGraphicsDropShadowEffect::xOffset();
impl /*struct*/ QGraphicsDropShadowEffect {
  pub fn xOffset<RetType, T: QGraphicsDropShadowEffect_xOffset<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.xOffset(self);
    // return 1;
  }
}

pub trait QGraphicsDropShadowEffect_xOffset<RetType> {
  fn xOffset(self , rsthis: &mut QGraphicsDropShadowEffect) -> RetType;
}

  // proto:  qreal QGraphicsDropShadowEffect::xOffset();
impl<'a> /*trait*/ QGraphicsDropShadowEffect_xOffset<f64> for () {
  fn xOffset(self , rsthis: &mut QGraphicsDropShadowEffect) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QGraphicsDropShadowEffect7xOffsetEv()};
    let mut ret = unsafe {_ZNK25QGraphicsDropShadowEffect7xOffsetEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  qreal QGraphicsDropShadowEffect::blurRadius();
impl /*struct*/ QGraphicsDropShadowEffect {
  pub fn blurRadius<RetType, T: QGraphicsDropShadowEffect_blurRadius<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.blurRadius(self);
    // return 1;
  }
}

pub trait QGraphicsDropShadowEffect_blurRadius<RetType> {
  fn blurRadius(self , rsthis: &mut QGraphicsDropShadowEffect) -> RetType;
}

  // proto:  qreal QGraphicsDropShadowEffect::blurRadius();
impl<'a> /*trait*/ QGraphicsDropShadowEffect_blurRadius<f64> for () {
  fn blurRadius(self , rsthis: &mut QGraphicsDropShadowEffect) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QGraphicsDropShadowEffect10blurRadiusEv()};
    let mut ret = unsafe {_ZNK25QGraphicsDropShadowEffect10blurRadiusEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  QColor QGraphicsDropShadowEffect::color();
impl /*struct*/ QGraphicsDropShadowEffect {
  pub fn color<RetType, T: QGraphicsDropShadowEffect_color<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.color(self);
    // return 1;
  }
}

pub trait QGraphicsDropShadowEffect_color<RetType> {
  fn color(self , rsthis: &mut QGraphicsDropShadowEffect) -> RetType;
}

  // proto:  QColor QGraphicsDropShadowEffect::color();
impl<'a> /*trait*/ QGraphicsDropShadowEffect_color<QColor> for () {
  fn color(self , rsthis: &mut QGraphicsDropShadowEffect) -> QColor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QGraphicsDropShadowEffect5colorEv()};
    let mut ret = unsafe {_ZNK25QGraphicsDropShadowEffect5colorEv(rsthis.qclsinst)};
    let mut ret1 = QColor{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsDropShadowEffect::setColor(const QColor & color);
impl /*struct*/ QGraphicsDropShadowEffect {
  pub fn setColor<RetType, T: QGraphicsDropShadowEffect_setColor<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setColor(self);
    // return 1;
  }
}

pub trait QGraphicsDropShadowEffect_setColor<RetType> {
  fn setColor(self , rsthis: &mut QGraphicsDropShadowEffect) -> RetType;
}

  // proto:  void QGraphicsDropShadowEffect::setColor(const QColor & color);
impl<'a> /*trait*/ QGraphicsDropShadowEffect_setColor<()> for (QColor) {
  fn setColor(self , rsthis: &mut QGraphicsDropShadowEffect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QGraphicsDropShadowEffect8setColorERK6QColor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN25QGraphicsDropShadowEffect8setColorERK6QColor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsDropShadowEffect::setOffset(const QPointF & ofs);
impl /*struct*/ QGraphicsDropShadowEffect {
  pub fn setOffset<RetType, T: QGraphicsDropShadowEffect_setOffset<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setOffset(self);
    // return 1;
  }
}

pub trait QGraphicsDropShadowEffect_setOffset<RetType> {
  fn setOffset(self , rsthis: &mut QGraphicsDropShadowEffect) -> RetType;
}

  // proto:  void QGraphicsDropShadowEffect::setOffset(const QPointF & ofs);
impl<'a> /*trait*/ QGraphicsDropShadowEffect_setOffset<()> for (QPointF) {
  fn setOffset(self , rsthis: &mut QGraphicsDropShadowEffect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QGraphicsDropShadowEffect9setOffsetERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN25QGraphicsDropShadowEffect9setOffsetERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsDropShadowEffect::offsetChanged(const QPointF & offset);
impl /*struct*/ QGraphicsDropShadowEffect {
  pub fn offsetChanged<RetType, T: QGraphicsDropShadowEffect_offsetChanged<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.offsetChanged(self);
    // return 1;
  }
}

pub trait QGraphicsDropShadowEffect_offsetChanged<RetType> {
  fn offsetChanged(self , rsthis: &mut QGraphicsDropShadowEffect) -> RetType;
}

  // proto:  void QGraphicsDropShadowEffect::offsetChanged(const QPointF & offset);
impl<'a> /*trait*/ QGraphicsDropShadowEffect_offsetChanged<()> for (QPointF) {
  fn offsetChanged(self , rsthis: &mut QGraphicsDropShadowEffect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QGraphicsDropShadowEffect13offsetChangedERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN25QGraphicsDropShadowEffect13offsetChangedERK7QPointF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsDropShadowEffect::setOffset(qreal dx, qreal dy);
impl<'a> /*trait*/ QGraphicsDropShadowEffect_setOffset<()> for (f64, f64) {
  fn setOffset(self , rsthis: &mut QGraphicsDropShadowEffect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QGraphicsDropShadowEffect9setOffsetEdd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
     unsafe {_ZN25QGraphicsDropShadowEffect9setOffsetEdd(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QGraphicsDropShadowEffect::setOffset(qreal d);
impl<'a> /*trait*/ QGraphicsDropShadowEffect_setOffset<()> for (f64) {
  fn setOffset(self , rsthis: &mut QGraphicsDropShadowEffect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QGraphicsDropShadowEffect9setOffsetEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN25QGraphicsDropShadowEffect9setOffsetEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsDropShadowEffect::QGraphicsDropShadowEffect(const QGraphicsDropShadowEffect & );
impl<'a> /*trait*/ QGraphicsDropShadowEffect_NewQGraphicsDropShadowEffect for (QGraphicsDropShadowEffect) {
  fn NewQGraphicsDropShadowEffect(self) -> QGraphicsDropShadowEffect {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QGraphicsDropShadowEffectC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN25QGraphicsDropShadowEffectC1ERKS_(qthis, arg0)};
    let rsthis = QGraphicsDropShadowEffect{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QGraphicsDropShadowEffect::colorChanged(const QColor & color);
impl /*struct*/ QGraphicsDropShadowEffect {
  pub fn colorChanged<RetType, T: QGraphicsDropShadowEffect_colorChanged<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.colorChanged(self);
    // return 1;
  }
}

pub trait QGraphicsDropShadowEffect_colorChanged<RetType> {
  fn colorChanged(self , rsthis: &mut QGraphicsDropShadowEffect) -> RetType;
}

  // proto:  void QGraphicsDropShadowEffect::colorChanged(const QColor & color);
impl<'a> /*trait*/ QGraphicsDropShadowEffect_colorChanged<()> for (QColor) {
  fn colorChanged(self , rsthis: &mut QGraphicsDropShadowEffect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QGraphicsDropShadowEffect12colorChangedERK6QColor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN25QGraphicsDropShadowEffect12colorChangedERK6QColor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  qreal QGraphicsDropShadowEffect::yOffset();
impl /*struct*/ QGraphicsDropShadowEffect {
  pub fn yOffset<RetType, T: QGraphicsDropShadowEffect_yOffset<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.yOffset(self);
    // return 1;
  }
}

pub trait QGraphicsDropShadowEffect_yOffset<RetType> {
  fn yOffset(self , rsthis: &mut QGraphicsDropShadowEffect) -> RetType;
}

  // proto:  qreal QGraphicsDropShadowEffect::yOffset();
impl<'a> /*trait*/ QGraphicsDropShadowEffect_yOffset<f64> for () {
  fn yOffset(self , rsthis: &mut QGraphicsDropShadowEffect) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK25QGraphicsDropShadowEffect7yOffsetEv()};
    let mut ret = unsafe {_ZNK25QGraphicsDropShadowEffect7yOffsetEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QGraphicsDropShadowEffect::setXOffset(qreal dx);
impl /*struct*/ QGraphicsDropShadowEffect {
  pub fn setXOffset<RetType, T: QGraphicsDropShadowEffect_setXOffset<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setXOffset(self);
    // return 1;
  }
}

pub trait QGraphicsDropShadowEffect_setXOffset<RetType> {
  fn setXOffset(self , rsthis: &mut QGraphicsDropShadowEffect) -> RetType;
}

  // proto:  void QGraphicsDropShadowEffect::setXOffset(qreal dx);
impl<'a> /*trait*/ QGraphicsDropShadowEffect_setXOffset<()> for (f64) {
  fn setXOffset(self , rsthis: &mut QGraphicsDropShadowEffect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QGraphicsDropShadowEffect10setXOffsetEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN25QGraphicsDropShadowEffect10setXOffsetEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsDropShadowEffect::setBlurRadius(qreal blurRadius);
impl /*struct*/ QGraphicsDropShadowEffect {
  pub fn setBlurRadius<RetType, T: QGraphicsDropShadowEffect_setBlurRadius<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setBlurRadius(self);
    // return 1;
  }
}

pub trait QGraphicsDropShadowEffect_setBlurRadius<RetType> {
  fn setBlurRadius(self , rsthis: &mut QGraphicsDropShadowEffect) -> RetType;
}

  // proto:  void QGraphicsDropShadowEffect::setBlurRadius(qreal blurRadius);
impl<'a> /*trait*/ QGraphicsDropShadowEffect_setBlurRadius<()> for (f64) {
  fn setBlurRadius(self , rsthis: &mut QGraphicsDropShadowEffect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QGraphicsDropShadowEffect13setBlurRadiusEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN25QGraphicsDropShadowEffect13setBlurRadiusEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsDropShadowEffect::~QGraphicsDropShadowEffect();
impl /*struct*/ QGraphicsDropShadowEffect {
  pub fn FreeQGraphicsDropShadowEffect<RetType, T: QGraphicsDropShadowEffect_FreeQGraphicsDropShadowEffect<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQGraphicsDropShadowEffect(self);
    // return 1;
  }
}

pub trait QGraphicsDropShadowEffect_FreeQGraphicsDropShadowEffect<RetType> {
  fn FreeQGraphicsDropShadowEffect(self , rsthis: &mut QGraphicsDropShadowEffect) -> RetType;
}

  // proto:  void QGraphicsDropShadowEffect::~QGraphicsDropShadowEffect();
impl<'a> /*trait*/ QGraphicsDropShadowEffect_FreeQGraphicsDropShadowEffect<()> for () {
  fn FreeQGraphicsDropShadowEffect(self , rsthis: &mut QGraphicsDropShadowEffect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN25QGraphicsDropShadowEffectD0Ev()};
     unsafe {_ZN25QGraphicsDropShadowEffectD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsOpacityEffect::QGraphicsOpacityEffect(QObject * parent);
impl /*struct*/ QGraphicsOpacityEffect {
  pub fn NewQGraphicsOpacityEffect<T: QGraphicsOpacityEffect_NewQGraphicsOpacityEffect>(value: T) -> QGraphicsOpacityEffect {
    let rsthis = value.NewQGraphicsOpacityEffect();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsOpacityEffect_NewQGraphicsOpacityEffect {
  fn NewQGraphicsOpacityEffect(self) -> QGraphicsOpacityEffect;
}

  // proto:  void QGraphicsOpacityEffect::QGraphicsOpacityEffect(QObject * parent);
impl<'a> /*trait*/ QGraphicsOpacityEffect_NewQGraphicsOpacityEffect for (QObject) {
  fn NewQGraphicsOpacityEffect(self) -> QGraphicsOpacityEffect {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QGraphicsOpacityEffectC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN22QGraphicsOpacityEffectC1EP7QObject(qthis, arg0)};
    let rsthis = QGraphicsOpacityEffect{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QGraphicsOpacityEffect::~QGraphicsOpacityEffect();
impl /*struct*/ QGraphicsOpacityEffect {
  pub fn FreeQGraphicsOpacityEffect<RetType, T: QGraphicsOpacityEffect_FreeQGraphicsOpacityEffect<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQGraphicsOpacityEffect(self);
    // return 1;
  }
}

pub trait QGraphicsOpacityEffect_FreeQGraphicsOpacityEffect<RetType> {
  fn FreeQGraphicsOpacityEffect(self , rsthis: &mut QGraphicsOpacityEffect) -> RetType;
}

  // proto:  void QGraphicsOpacityEffect::~QGraphicsOpacityEffect();
impl<'a> /*trait*/ QGraphicsOpacityEffect_FreeQGraphicsOpacityEffect<()> for () {
  fn FreeQGraphicsOpacityEffect(self , rsthis: &mut QGraphicsOpacityEffect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QGraphicsOpacityEffectD0Ev()};
     unsafe {_ZN22QGraphicsOpacityEffectD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsOpacityEffect::setOpacityMask(const QBrush & mask);
impl /*struct*/ QGraphicsOpacityEffect {
  pub fn setOpacityMask<RetType, T: QGraphicsOpacityEffect_setOpacityMask<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setOpacityMask(self);
    // return 1;
  }
}

pub trait QGraphicsOpacityEffect_setOpacityMask<RetType> {
  fn setOpacityMask(self , rsthis: &mut QGraphicsOpacityEffect) -> RetType;
}

  // proto:  void QGraphicsOpacityEffect::setOpacityMask(const QBrush & mask);
impl<'a> /*trait*/ QGraphicsOpacityEffect_setOpacityMask<()> for (QBrush) {
  fn setOpacityMask(self , rsthis: &mut QGraphicsOpacityEffect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QGraphicsOpacityEffect14setOpacityMaskERK6QBrush()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN22QGraphicsOpacityEffect14setOpacityMaskERK6QBrush(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsOpacityEffect::QGraphicsOpacityEffect(const QGraphicsOpacityEffect & );
impl<'a> /*trait*/ QGraphicsOpacityEffect_NewQGraphicsOpacityEffect for (QGraphicsOpacityEffect) {
  fn NewQGraphicsOpacityEffect(self) -> QGraphicsOpacityEffect {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QGraphicsOpacityEffectC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN22QGraphicsOpacityEffectC1ERKS_(qthis, arg0)};
    let rsthis = QGraphicsOpacityEffect{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QBrush QGraphicsOpacityEffect::opacityMask();
impl /*struct*/ QGraphicsOpacityEffect {
  pub fn opacityMask<RetType, T: QGraphicsOpacityEffect_opacityMask<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.opacityMask(self);
    // return 1;
  }
}

pub trait QGraphicsOpacityEffect_opacityMask<RetType> {
  fn opacityMask(self , rsthis: &mut QGraphicsOpacityEffect) -> RetType;
}

  // proto:  QBrush QGraphicsOpacityEffect::opacityMask();
impl<'a> /*trait*/ QGraphicsOpacityEffect_opacityMask<QBrush> for () {
  fn opacityMask(self , rsthis: &mut QGraphicsOpacityEffect) -> QBrush {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QGraphicsOpacityEffect11opacityMaskEv()};
    let mut ret = unsafe {_ZNK22QGraphicsOpacityEffect11opacityMaskEv(rsthis.qclsinst)};
    let mut ret1 = QBrush{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsOpacityEffect::opacityChanged(qreal opacity);
impl /*struct*/ QGraphicsOpacityEffect {
  pub fn opacityChanged<RetType, T: QGraphicsOpacityEffect_opacityChanged<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.opacityChanged(self);
    // return 1;
  }
}

pub trait QGraphicsOpacityEffect_opacityChanged<RetType> {
  fn opacityChanged(self , rsthis: &mut QGraphicsOpacityEffect) -> RetType;
}

  // proto:  void QGraphicsOpacityEffect::opacityChanged(qreal opacity);
impl<'a> /*trait*/ QGraphicsOpacityEffect_opacityChanged<()> for (f64) {
  fn opacityChanged(self , rsthis: &mut QGraphicsOpacityEffect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QGraphicsOpacityEffect14opacityChangedEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN22QGraphicsOpacityEffect14opacityChangedEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsOpacityEffect::opacityMaskChanged(const QBrush & mask);
impl /*struct*/ QGraphicsOpacityEffect {
  pub fn opacityMaskChanged<RetType, T: QGraphicsOpacityEffect_opacityMaskChanged<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.opacityMaskChanged(self);
    // return 1;
  }
}

pub trait QGraphicsOpacityEffect_opacityMaskChanged<RetType> {
  fn opacityMaskChanged(self , rsthis: &mut QGraphicsOpacityEffect) -> RetType;
}

  // proto:  void QGraphicsOpacityEffect::opacityMaskChanged(const QBrush & mask);
impl<'a> /*trait*/ QGraphicsOpacityEffect_opacityMaskChanged<()> for (QBrush) {
  fn opacityMaskChanged(self , rsthis: &mut QGraphicsOpacityEffect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QGraphicsOpacityEffect18opacityMaskChangedERK6QBrush()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN22QGraphicsOpacityEffect18opacityMaskChangedERK6QBrush(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QGraphicsOpacityEffect::metaObject();
impl /*struct*/ QGraphicsOpacityEffect {
  pub fn metaObject<RetType, T: QGraphicsOpacityEffect_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QGraphicsOpacityEffect_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QGraphicsOpacityEffect) -> RetType;
}

  // proto:  const QMetaObject * QGraphicsOpacityEffect::metaObject();
impl<'a> /*trait*/ QGraphicsOpacityEffect_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QGraphicsOpacityEffect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QGraphicsOpacityEffect10metaObjectEv()};
     unsafe {_ZNK22QGraphicsOpacityEffect10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  qreal QGraphicsOpacityEffect::opacity();
impl /*struct*/ QGraphicsOpacityEffect {
  pub fn opacity<RetType, T: QGraphicsOpacityEffect_opacity<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.opacity(self);
    // return 1;
  }
}

pub trait QGraphicsOpacityEffect_opacity<RetType> {
  fn opacity(self , rsthis: &mut QGraphicsOpacityEffect) -> RetType;
}

  // proto:  qreal QGraphicsOpacityEffect::opacity();
impl<'a> /*trait*/ QGraphicsOpacityEffect_opacity<f64> for () {
  fn opacity(self , rsthis: &mut QGraphicsOpacityEffect) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK22QGraphicsOpacityEffect7opacityEv()};
    let mut ret = unsafe {_ZNK22QGraphicsOpacityEffect7opacityEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QGraphicsOpacityEffect::setOpacity(qreal opacity);
impl /*struct*/ QGraphicsOpacityEffect {
  pub fn setOpacity<RetType, T: QGraphicsOpacityEffect_setOpacity<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setOpacity(self);
    // return 1;
  }
}

pub trait QGraphicsOpacityEffect_setOpacity<RetType> {
  fn setOpacity(self , rsthis: &mut QGraphicsOpacityEffect) -> RetType;
}

  // proto:  void QGraphicsOpacityEffect::setOpacity(qreal opacity);
impl<'a> /*trait*/ QGraphicsOpacityEffect_setOpacity<()> for (f64) {
  fn setOpacity(self , rsthis: &mut QGraphicsOpacityEffect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN22QGraphicsOpacityEffect10setOpacityEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN22QGraphicsOpacityEffect10setOpacityEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  qreal QGraphicsBlurEffect::blurRadius();
impl /*struct*/ QGraphicsBlurEffect {
  pub fn blurRadius<RetType, T: QGraphicsBlurEffect_blurRadius<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.blurRadius(self);
    // return 1;
  }
}

pub trait QGraphicsBlurEffect_blurRadius<RetType> {
  fn blurRadius(self , rsthis: &mut QGraphicsBlurEffect) -> RetType;
}

  // proto:  qreal QGraphicsBlurEffect::blurRadius();
impl<'a> /*trait*/ QGraphicsBlurEffect_blurRadius<f64> for () {
  fn blurRadius(self , rsthis: &mut QGraphicsBlurEffect) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsBlurEffect10blurRadiusEv()};
    let mut ret = unsafe {_ZNK19QGraphicsBlurEffect10blurRadiusEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QGraphicsBlurEffect::setBlurRadius(qreal blurRadius);
impl /*struct*/ QGraphicsBlurEffect {
  pub fn setBlurRadius<RetType, T: QGraphicsBlurEffect_setBlurRadius<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setBlurRadius(self);
    // return 1;
  }
}

pub trait QGraphicsBlurEffect_setBlurRadius<RetType> {
  fn setBlurRadius(self , rsthis: &mut QGraphicsBlurEffect) -> RetType;
}

  // proto:  void QGraphicsBlurEffect::setBlurRadius(qreal blurRadius);
impl<'a> /*trait*/ QGraphicsBlurEffect_setBlurRadius<()> for (f64) {
  fn setBlurRadius(self , rsthis: &mut QGraphicsBlurEffect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsBlurEffect13setBlurRadiusEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN19QGraphicsBlurEffect13setBlurRadiusEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsBlurEffect::~QGraphicsBlurEffect();
impl /*struct*/ QGraphicsBlurEffect {
  pub fn FreeQGraphicsBlurEffect<RetType, T: QGraphicsBlurEffect_FreeQGraphicsBlurEffect<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQGraphicsBlurEffect(self);
    // return 1;
  }
}

pub trait QGraphicsBlurEffect_FreeQGraphicsBlurEffect<RetType> {
  fn FreeQGraphicsBlurEffect(self , rsthis: &mut QGraphicsBlurEffect) -> RetType;
}

  // proto:  void QGraphicsBlurEffect::~QGraphicsBlurEffect();
impl<'a> /*trait*/ QGraphicsBlurEffect_FreeQGraphicsBlurEffect<()> for () {
  fn FreeQGraphicsBlurEffect(self , rsthis: &mut QGraphicsBlurEffect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsBlurEffectD0Ev()};
     unsafe {_ZN19QGraphicsBlurEffectD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsBlurEffect::QGraphicsBlurEffect(const QGraphicsBlurEffect & );
impl /*struct*/ QGraphicsBlurEffect {
  pub fn NewQGraphicsBlurEffect<T: QGraphicsBlurEffect_NewQGraphicsBlurEffect>(value: T) -> QGraphicsBlurEffect {
    let rsthis = value.NewQGraphicsBlurEffect();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsBlurEffect_NewQGraphicsBlurEffect {
  fn NewQGraphicsBlurEffect(self) -> QGraphicsBlurEffect;
}

  // proto:  void QGraphicsBlurEffect::QGraphicsBlurEffect(const QGraphicsBlurEffect & );
impl<'a> /*trait*/ QGraphicsBlurEffect_NewQGraphicsBlurEffect for (QGraphicsBlurEffect) {
  fn NewQGraphicsBlurEffect(self) -> QGraphicsBlurEffect {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsBlurEffectC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN19QGraphicsBlurEffectC1ERKS_(qthis, arg0)};
    let rsthis = QGraphicsBlurEffect{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  const QMetaObject * QGraphicsBlurEffect::metaObject();
impl /*struct*/ QGraphicsBlurEffect {
  pub fn metaObject<RetType, T: QGraphicsBlurEffect_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QGraphicsBlurEffect_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QGraphicsBlurEffect) -> RetType;
}

  // proto:  const QMetaObject * QGraphicsBlurEffect::metaObject();
impl<'a> /*trait*/ QGraphicsBlurEffect_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QGraphicsBlurEffect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsBlurEffect10metaObjectEv()};
     unsafe {_ZNK19QGraphicsBlurEffect10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QRectF QGraphicsBlurEffect::boundingRectFor(const QRectF & rect);
impl /*struct*/ QGraphicsBlurEffect {
  pub fn boundingRectFor<RetType, T: QGraphicsBlurEffect_boundingRectFor<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.boundingRectFor(self);
    // return 1;
  }
}

pub trait QGraphicsBlurEffect_boundingRectFor<RetType> {
  fn boundingRectFor(self , rsthis: &mut QGraphicsBlurEffect) -> RetType;
}

  // proto:  QRectF QGraphicsBlurEffect::boundingRectFor(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsBlurEffect_boundingRectFor<QRectF> for (QRectF) {
  fn boundingRectFor(self , rsthis: &mut QGraphicsBlurEffect) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsBlurEffect15boundingRectForERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK19QGraphicsBlurEffect15boundingRectForERK6QRectF(rsthis.qclsinst, arg0)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsBlurEffect::QGraphicsBlurEffect(QObject * parent);
impl<'a> /*trait*/ QGraphicsBlurEffect_NewQGraphicsBlurEffect for (QObject) {
  fn NewQGraphicsBlurEffect(self) -> QGraphicsBlurEffect {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsBlurEffectC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN19QGraphicsBlurEffectC1EP7QObject(qthis, arg0)};
    let rsthis = QGraphicsBlurEffect{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QGraphicsBlurEffect::blurRadiusChanged(qreal blurRadius);
impl /*struct*/ QGraphicsBlurEffect {
  pub fn blurRadiusChanged<RetType, T: QGraphicsBlurEffect_blurRadiusChanged<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.blurRadiusChanged(self);
    // return 1;
  }
}

pub trait QGraphicsBlurEffect_blurRadiusChanged<RetType> {
  fn blurRadiusChanged(self , rsthis: &mut QGraphicsBlurEffect) -> RetType;
}

  // proto:  void QGraphicsBlurEffect::blurRadiusChanged(qreal blurRadius);
impl<'a> /*trait*/ QGraphicsBlurEffect_blurRadiusChanged<()> for (f64) {
  fn blurRadiusChanged(self , rsthis: &mut QGraphicsBlurEffect) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsBlurEffect17blurRadiusChangedEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN19QGraphicsBlurEffect17blurRadiusChangedEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// <= body block end

