// auto generated, do not modify.
// created: Mon Feb 22 23:57:02 2016
// src-file: /QtWidgets/qgraphicslinearlayout.h
// dst-file: /src/widgets/qgraphicslinearlayout.rs
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
use super::qgraphicslayout::*; // 773
use std::ops::Deref;
use super::qgraphicslayoutitem::*; // 773
use super::super::core::qrect::*; // 771
use super::super::core::qsize::*; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QGraphicsLinearLayout_Class_Size() -> c_int;
  // proto:  qreal QGraphicsLinearLayout::spacing();
  fn C_ZNK21QGraphicsLinearLayout7spacingEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  void QGraphicsLinearLayout::QGraphicsLinearLayout(QGraphicsLayoutItem * parent);
  fn C_ZN21QGraphicsLinearLayoutC2EP19QGraphicsLayoutItem(arg0: *mut c_void) -> u64;
  // proto:  QGraphicsLayoutItem * QGraphicsLinearLayout::itemAt(int index);
  fn C_ZNK21QGraphicsLinearLayout6itemAtEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  void QGraphicsLinearLayout::invalidate();
  fn C_ZN21QGraphicsLinearLayout10invalidateEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QGraphicsLinearLayout::setGeometry(const QRectF & rect);
  fn C_ZN21QGraphicsLinearLayout11setGeometryERK6QRectF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QGraphicsLinearLayout::addStretch(int stretch);
  fn C_ZN21QGraphicsLinearLayout10addStretchEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  int QGraphicsLinearLayout::count();
  fn C_ZNK21QGraphicsLinearLayout5countEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QGraphicsLinearLayout::setSpacing(qreal spacing);
  fn C_ZN21QGraphicsLinearLayout10setSpacingEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  void QGraphicsLinearLayout::insertItem(int index, QGraphicsLayoutItem * item);
  fn C_ZN21QGraphicsLinearLayout10insertItemEiP19QGraphicsLayoutItem(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void);
  // proto:  void QGraphicsLinearLayout::~QGraphicsLinearLayout();
  fn C_ZN21QGraphicsLinearLayoutD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QGraphicsLinearLayout::dump(int indent);
  fn C_ZNK21QGraphicsLinearLayout4dumpEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QGraphicsLinearLayout::setStretchFactor(QGraphicsLayoutItem * item, int stretch);
  fn C_ZN21QGraphicsLinearLayout16setStretchFactorEP19QGraphicsLayoutItemi(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: c_int);
  // proto:  void QGraphicsLinearLayout::addItem(QGraphicsLayoutItem * item);
  fn C_ZN21QGraphicsLinearLayout7addItemEP19QGraphicsLayoutItem(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  qreal QGraphicsLinearLayout::itemSpacing(int index);
  fn C_ZNK21QGraphicsLinearLayout11itemSpacingEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> c_double;
  // proto:  void QGraphicsLinearLayout::removeAt(int index);
  fn C_ZN21QGraphicsLinearLayout8removeAtEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QGraphicsLinearLayout::insertStretch(int index, int stretch);
  fn C_ZN21QGraphicsLinearLayout13insertStretchEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int);
  // proto:  void QGraphicsLinearLayout::setItemSpacing(int index, qreal spacing);
  fn C_ZN21QGraphicsLinearLayout14setItemSpacingEid(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_double);
  // proto:  void QGraphicsLinearLayout::removeItem(QGraphicsLayoutItem * item);
  fn C_ZN21QGraphicsLinearLayout10removeItemEP19QGraphicsLayoutItem(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  int QGraphicsLinearLayout::stretchFactor(QGraphicsLayoutItem * item);
  fn C_ZNK21QGraphicsLinearLayout13stretchFactorEP19QGraphicsLayoutItem(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> c_int;
} // <= ext block end

// body block begin =>
// class sizeof(QGraphicsLinearLayout)=1
#[derive(Default)]
pub struct QGraphicsLinearLayout {
  qbase: QGraphicsLayout,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QGraphicsLinearLayout {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QGraphicsLinearLayout {
    return QGraphicsLinearLayout{qbase: QGraphicsLayout::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QGraphicsLinearLayout {
  type Target = QGraphicsLayout;

  fn deref(&self) -> &QGraphicsLayout {
    return & self.qbase;
  }
}
impl AsRef<QGraphicsLayout> for QGraphicsLinearLayout {
  fn as_ref(& self) -> & QGraphicsLayout {
    return & self.qbase;
  }
}
  // proto:  qreal QGraphicsLinearLayout::spacing();
impl /*struct*/ QGraphicsLinearLayout {
  pub fn spacing<RetType, T: QGraphicsLinearLayout_spacing<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.spacing(self);
    // return 1;
  }
}

pub trait QGraphicsLinearLayout_spacing<RetType> {
  fn spacing(self , rsthis: & QGraphicsLinearLayout) -> RetType;
}

  // proto:  qreal QGraphicsLinearLayout::spacing();
impl<'a> /*trait*/ QGraphicsLinearLayout_spacing<f64> for () {
  fn spacing(self , rsthis: & QGraphicsLinearLayout) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QGraphicsLinearLayout7spacingEv()};
    let mut ret = unsafe {C_ZNK21QGraphicsLinearLayout7spacingEv(rsthis.qclsinst)};
    return ret as f64; // 1
    // return 1;
  }
}

  // proto:  void QGraphicsLinearLayout::QGraphicsLinearLayout(QGraphicsLayoutItem * parent);
impl /*struct*/ QGraphicsLinearLayout {
  pub fn new<T: QGraphicsLinearLayout_new>(value: T) -> QGraphicsLinearLayout {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsLinearLayout_new {
  fn new(self) -> QGraphicsLinearLayout;
}

  // proto:  void QGraphicsLinearLayout::QGraphicsLinearLayout(QGraphicsLayoutItem * parent);
impl<'a> /*trait*/ QGraphicsLinearLayout_new for (Option<&'a QGraphicsLayoutItem>) {
  fn new(self) -> QGraphicsLinearLayout {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QGraphicsLinearLayoutC2EP19QGraphicsLayoutItem()};
    let ctysz: c_int = unsafe{QGraphicsLinearLayout_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = (if self.is_none() {0} else {self.unwrap().qclsinst})  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN21QGraphicsLinearLayoutC2EP19QGraphicsLayoutItem(arg0)};
    let rsthis = QGraphicsLinearLayout{qbase: QGraphicsLayout::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QGraphicsLayoutItem * QGraphicsLinearLayout::itemAt(int index);
impl /*struct*/ QGraphicsLinearLayout {
  pub fn itemAt<RetType, T: QGraphicsLinearLayout_itemAt<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.itemAt(self);
    // return 1;
  }
}

pub trait QGraphicsLinearLayout_itemAt<RetType> {
  fn itemAt(self , rsthis: & QGraphicsLinearLayout) -> RetType;
}

  // proto:  QGraphicsLayoutItem * QGraphicsLinearLayout::itemAt(int index);
impl<'a> /*trait*/ QGraphicsLinearLayout_itemAt<QGraphicsLayoutItem> for (i32) {
  fn itemAt(self , rsthis: & QGraphicsLinearLayout) -> QGraphicsLayoutItem {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QGraphicsLinearLayout6itemAtEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZNK21QGraphicsLinearLayout6itemAtEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QGraphicsLayoutItem::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QGraphicsLinearLayout::invalidate();
impl /*struct*/ QGraphicsLinearLayout {
  pub fn invalidate<RetType, T: QGraphicsLinearLayout_invalidate<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.invalidate(self);
    // return 1;
  }
}

pub trait QGraphicsLinearLayout_invalidate<RetType> {
  fn invalidate(self , rsthis: & QGraphicsLinearLayout) -> RetType;
}

  // proto:  void QGraphicsLinearLayout::invalidate();
impl<'a> /*trait*/ QGraphicsLinearLayout_invalidate<()> for () {
  fn invalidate(self , rsthis: & QGraphicsLinearLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QGraphicsLinearLayout10invalidateEv()};
     unsafe {C_ZN21QGraphicsLinearLayout10invalidateEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsLinearLayout::setGeometry(const QRectF & rect);
impl /*struct*/ QGraphicsLinearLayout {
  pub fn setGeometry<RetType, T: QGraphicsLinearLayout_setGeometry<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setGeometry(self);
    // return 1;
  }
}

pub trait QGraphicsLinearLayout_setGeometry<RetType> {
  fn setGeometry(self , rsthis: & QGraphicsLinearLayout) -> RetType;
}

  // proto:  void QGraphicsLinearLayout::setGeometry(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsLinearLayout_setGeometry<()> for (&'a QRectF) {
  fn setGeometry(self , rsthis: & QGraphicsLinearLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QGraphicsLinearLayout11setGeometryERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN21QGraphicsLinearLayout11setGeometryERK6QRectF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsLinearLayout::addStretch(int stretch);
impl /*struct*/ QGraphicsLinearLayout {
  pub fn addStretch<RetType, T: QGraphicsLinearLayout_addStretch<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addStretch(self);
    // return 1;
  }
}

pub trait QGraphicsLinearLayout_addStretch<RetType> {
  fn addStretch(self , rsthis: & QGraphicsLinearLayout) -> RetType;
}

  // proto:  void QGraphicsLinearLayout::addStretch(int stretch);
impl<'a> /*trait*/ QGraphicsLinearLayout_addStretch<()> for (Option<i32>) {
  fn addStretch(self , rsthis: & QGraphicsLinearLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QGraphicsLinearLayout10addStretchEi()};
    let arg0 = (if self.is_none() {1} else {self.unwrap()})  as c_int;
     unsafe {C_ZN21QGraphicsLinearLayout10addStretchEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QGraphicsLinearLayout::count();
impl /*struct*/ QGraphicsLinearLayout {
  pub fn count<RetType, T: QGraphicsLinearLayout_count<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.count(self);
    // return 1;
  }
}

pub trait QGraphicsLinearLayout_count<RetType> {
  fn count(self , rsthis: & QGraphicsLinearLayout) -> RetType;
}

  // proto:  int QGraphicsLinearLayout::count();
impl<'a> /*trait*/ QGraphicsLinearLayout_count<i32> for () {
  fn count(self , rsthis: & QGraphicsLinearLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QGraphicsLinearLayout5countEv()};
    let mut ret = unsafe {C_ZNK21QGraphicsLinearLayout5countEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  void QGraphicsLinearLayout::setSpacing(qreal spacing);
impl /*struct*/ QGraphicsLinearLayout {
  pub fn setSpacing<RetType, T: QGraphicsLinearLayout_setSpacing<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setSpacing(self);
    // return 1;
  }
}

pub trait QGraphicsLinearLayout_setSpacing<RetType> {
  fn setSpacing(self , rsthis: & QGraphicsLinearLayout) -> RetType;
}

  // proto:  void QGraphicsLinearLayout::setSpacing(qreal spacing);
impl<'a> /*trait*/ QGraphicsLinearLayout_setSpacing<()> for (f64) {
  fn setSpacing(self , rsthis: & QGraphicsLinearLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QGraphicsLinearLayout10setSpacingEd()};
    let arg0 = self  as c_double;
     unsafe {C_ZN21QGraphicsLinearLayout10setSpacingEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsLinearLayout::insertItem(int index, QGraphicsLayoutItem * item);
impl /*struct*/ QGraphicsLinearLayout {
  pub fn insertItem<RetType, T: QGraphicsLinearLayout_insertItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.insertItem(self);
    // return 1;
  }
}

pub trait QGraphicsLinearLayout_insertItem<RetType> {
  fn insertItem(self , rsthis: & QGraphicsLinearLayout) -> RetType;
}

  // proto:  void QGraphicsLinearLayout::insertItem(int index, QGraphicsLayoutItem * item);
impl<'a> /*trait*/ QGraphicsLinearLayout_insertItem<()> for (i32, &'a QGraphicsLayoutItem) {
  fn insertItem(self , rsthis: & QGraphicsLinearLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QGraphicsLinearLayout10insertItemEiP19QGraphicsLayoutItem()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {C_ZN21QGraphicsLinearLayout10insertItemEiP19QGraphicsLayoutItem(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QGraphicsLinearLayout::~QGraphicsLinearLayout();
impl /*struct*/ QGraphicsLinearLayout {
  pub fn free<RetType, T: QGraphicsLinearLayout_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QGraphicsLinearLayout_free<RetType> {
  fn free(self , rsthis: & QGraphicsLinearLayout) -> RetType;
}

  // proto:  void QGraphicsLinearLayout::~QGraphicsLinearLayout();
impl<'a> /*trait*/ QGraphicsLinearLayout_free<()> for () {
  fn free(self , rsthis: & QGraphicsLinearLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QGraphicsLinearLayoutD2Ev()};
     unsafe {C_ZN21QGraphicsLinearLayoutD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QGraphicsLinearLayout::dump(int indent);
impl /*struct*/ QGraphicsLinearLayout {
  pub fn dump<RetType, T: QGraphicsLinearLayout_dump<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.dump(self);
    // return 1;
  }
}

pub trait QGraphicsLinearLayout_dump<RetType> {
  fn dump(self , rsthis: & QGraphicsLinearLayout) -> RetType;
}

  // proto:  void QGraphicsLinearLayout::dump(int indent);
impl<'a> /*trait*/ QGraphicsLinearLayout_dump<()> for (Option<i32>) {
  fn dump(self , rsthis: & QGraphicsLinearLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QGraphicsLinearLayout4dumpEi()};
    let arg0 = (if self.is_none() {0} else {self.unwrap()})  as c_int;
     unsafe {C_ZNK21QGraphicsLinearLayout4dumpEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsLinearLayout::setStretchFactor(QGraphicsLayoutItem * item, int stretch);
impl /*struct*/ QGraphicsLinearLayout {
  pub fn setStretchFactor<RetType, T: QGraphicsLinearLayout_setStretchFactor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setStretchFactor(self);
    // return 1;
  }
}

pub trait QGraphicsLinearLayout_setStretchFactor<RetType> {
  fn setStretchFactor(self , rsthis: & QGraphicsLinearLayout) -> RetType;
}

  // proto:  void QGraphicsLinearLayout::setStretchFactor(QGraphicsLayoutItem * item, int stretch);
impl<'a> /*trait*/ QGraphicsLinearLayout_setStretchFactor<()> for (&'a QGraphicsLayoutItem, i32) {
  fn setStretchFactor(self , rsthis: & QGraphicsLinearLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QGraphicsLinearLayout16setStretchFactorEP19QGraphicsLayoutItemi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {C_ZN21QGraphicsLinearLayout16setStretchFactorEP19QGraphicsLayoutItemi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QGraphicsLinearLayout::addItem(QGraphicsLayoutItem * item);
impl /*struct*/ QGraphicsLinearLayout {
  pub fn addItem<RetType, T: QGraphicsLinearLayout_addItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addItem(self);
    // return 1;
  }
}

pub trait QGraphicsLinearLayout_addItem<RetType> {
  fn addItem(self , rsthis: & QGraphicsLinearLayout) -> RetType;
}

  // proto:  void QGraphicsLinearLayout::addItem(QGraphicsLayoutItem * item);
impl<'a> /*trait*/ QGraphicsLinearLayout_addItem<()> for (&'a QGraphicsLayoutItem) {
  fn addItem(self , rsthis: & QGraphicsLinearLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QGraphicsLinearLayout7addItemEP19QGraphicsLayoutItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN21QGraphicsLinearLayout7addItemEP19QGraphicsLayoutItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  qreal QGraphicsLinearLayout::itemSpacing(int index);
impl /*struct*/ QGraphicsLinearLayout {
  pub fn itemSpacing<RetType, T: QGraphicsLinearLayout_itemSpacing<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.itemSpacing(self);
    // return 1;
  }
}

pub trait QGraphicsLinearLayout_itemSpacing<RetType> {
  fn itemSpacing(self , rsthis: & QGraphicsLinearLayout) -> RetType;
}

  // proto:  qreal QGraphicsLinearLayout::itemSpacing(int index);
impl<'a> /*trait*/ QGraphicsLinearLayout_itemSpacing<f64> for (i32) {
  fn itemSpacing(self , rsthis: & QGraphicsLinearLayout) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QGraphicsLinearLayout11itemSpacingEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZNK21QGraphicsLinearLayout11itemSpacingEi(rsthis.qclsinst, arg0)};
    return ret as f64; // 1
    // return 1;
  }
}

  // proto:  void QGraphicsLinearLayout::removeAt(int index);
impl /*struct*/ QGraphicsLinearLayout {
  pub fn removeAt<RetType, T: QGraphicsLinearLayout_removeAt<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.removeAt(self);
    // return 1;
  }
}

pub trait QGraphicsLinearLayout_removeAt<RetType> {
  fn removeAt(self , rsthis: & QGraphicsLinearLayout) -> RetType;
}

  // proto:  void QGraphicsLinearLayout::removeAt(int index);
impl<'a> /*trait*/ QGraphicsLinearLayout_removeAt<()> for (i32) {
  fn removeAt(self , rsthis: & QGraphicsLinearLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QGraphicsLinearLayout8removeAtEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN21QGraphicsLinearLayout8removeAtEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QGraphicsLinearLayout::insertStretch(int index, int stretch);
impl /*struct*/ QGraphicsLinearLayout {
  pub fn insertStretch<RetType, T: QGraphicsLinearLayout_insertStretch<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.insertStretch(self);
    // return 1;
  }
}

pub trait QGraphicsLinearLayout_insertStretch<RetType> {
  fn insertStretch(self , rsthis: & QGraphicsLinearLayout) -> RetType;
}

  // proto:  void QGraphicsLinearLayout::insertStretch(int index, int stretch);
impl<'a> /*trait*/ QGraphicsLinearLayout_insertStretch<()> for (i32, Option<i32>) {
  fn insertStretch(self , rsthis: & QGraphicsLinearLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QGraphicsLinearLayout13insertStretchEii()};
    let arg0 = self.0  as c_int;
    let arg1 = (if self.1.is_none() {1} else {self.1.unwrap()})  as c_int;
     unsafe {C_ZN21QGraphicsLinearLayout13insertStretchEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QGraphicsLinearLayout::setItemSpacing(int index, qreal spacing);
impl /*struct*/ QGraphicsLinearLayout {
  pub fn setItemSpacing<RetType, T: QGraphicsLinearLayout_setItemSpacing<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setItemSpacing(self);
    // return 1;
  }
}

pub trait QGraphicsLinearLayout_setItemSpacing<RetType> {
  fn setItemSpacing(self , rsthis: & QGraphicsLinearLayout) -> RetType;
}

  // proto:  void QGraphicsLinearLayout::setItemSpacing(int index, qreal spacing);
impl<'a> /*trait*/ QGraphicsLinearLayout_setItemSpacing<()> for (i32, f64) {
  fn setItemSpacing(self , rsthis: & QGraphicsLinearLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QGraphicsLinearLayout14setItemSpacingEid()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_double;
     unsafe {C_ZN21QGraphicsLinearLayout14setItemSpacingEid(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QGraphicsLinearLayout::removeItem(QGraphicsLayoutItem * item);
impl /*struct*/ QGraphicsLinearLayout {
  pub fn removeItem<RetType, T: QGraphicsLinearLayout_removeItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.removeItem(self);
    // return 1;
  }
}

pub trait QGraphicsLinearLayout_removeItem<RetType> {
  fn removeItem(self , rsthis: & QGraphicsLinearLayout) -> RetType;
}

  // proto:  void QGraphicsLinearLayout::removeItem(QGraphicsLayoutItem * item);
impl<'a> /*trait*/ QGraphicsLinearLayout_removeItem<()> for (&'a QGraphicsLayoutItem) {
  fn removeItem(self , rsthis: & QGraphicsLinearLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QGraphicsLinearLayout10removeItemEP19QGraphicsLayoutItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN21QGraphicsLinearLayout10removeItemEP19QGraphicsLayoutItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QGraphicsLinearLayout::stretchFactor(QGraphicsLayoutItem * item);
impl /*struct*/ QGraphicsLinearLayout {
  pub fn stretchFactor<RetType, T: QGraphicsLinearLayout_stretchFactor<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.stretchFactor(self);
    // return 1;
  }
}

pub trait QGraphicsLinearLayout_stretchFactor<RetType> {
  fn stretchFactor(self , rsthis: & QGraphicsLinearLayout) -> RetType;
}

  // proto:  int QGraphicsLinearLayout::stretchFactor(QGraphicsLayoutItem * item);
impl<'a> /*trait*/ QGraphicsLinearLayout_stretchFactor<i32> for (&'a QGraphicsLayoutItem) {
  fn stretchFactor(self , rsthis: & QGraphicsLinearLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QGraphicsLinearLayout13stretchFactorEP19QGraphicsLayoutItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK21QGraphicsLinearLayout13stretchFactorEP19QGraphicsLayoutItem(rsthis.qclsinst, arg0)};
    return ret as i32; // 1
    // return 1;
  }
}

// <= body block end

