// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qgraphicslayoutitem::QGraphicsLayoutItem;
use super::qrectf::QRectF;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  double QGraphicsLinearLayout::spacing();
  fn _ZNK21QGraphicsLinearLayout7spacingEv(qthis: *mut c_void) -> c_double;
  // proto:  void QGraphicsLinearLayout::NewQGraphicsLinearLayout(QGraphicsLayoutItem * parent);
  fn _ZN21QGraphicsLinearLayoutC1EP19QGraphicsLayoutItem(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGraphicsLinearLayout::NewQGraphicsLinearLayout(const QGraphicsLinearLayout & );
  fn _ZN21QGraphicsLinearLayoutC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QGraphicsLayoutItem * QGraphicsLinearLayout::itemAt(int index);
  fn _ZNK21QGraphicsLinearLayout6itemAtEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QGraphicsLinearLayout::invalidate();
  fn _ZN21QGraphicsLinearLayout10invalidateEv(qthis: *mut c_void) ;
  // proto:  void QGraphicsLinearLayout::setGeometry(const QRectF & rect);
  fn _ZN21QGraphicsLinearLayout11setGeometryERK6QRectF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGraphicsLinearLayout::addStretch(int stretch);
  fn _ZN21QGraphicsLinearLayout10addStretchEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  int QGraphicsLinearLayout::count();
  fn _ZNK21QGraphicsLinearLayout5countEv(qthis: *mut c_void) -> c_int;
  // proto:  void QGraphicsLinearLayout::setSpacing(qreal spacing);
  fn _ZN21QGraphicsLinearLayout10setSpacingEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  void QGraphicsLinearLayout::insertItem(int index, QGraphicsLayoutItem * item);
  fn _ZN21QGraphicsLinearLayout10insertItemEiP19QGraphicsLayoutItem(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) ;
  // proto:  void QGraphicsLinearLayout::FreeQGraphicsLinearLayout();
  fn _ZN21QGraphicsLinearLayoutD0Ev(qthis: *mut c_void) ;
  // proto:  void QGraphicsLinearLayout::dump(int indent);
  fn _ZNK21QGraphicsLinearLayout4dumpEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QGraphicsLinearLayout::setStretchFactor(QGraphicsLayoutItem * item, int stretch);
  fn _ZN21QGraphicsLinearLayout16setStretchFactorEP19QGraphicsLayoutItemi(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int) ;
  // proto:  void QGraphicsLinearLayout::addItem(QGraphicsLayoutItem * item);
  fn _ZN21QGraphicsLinearLayout7addItemEP19QGraphicsLayoutItem(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  double QGraphicsLinearLayout::itemSpacing(int index);
  fn _ZNK21QGraphicsLinearLayout11itemSpacingEi(qthis: *mut c_void, arg0: c_int) -> c_double;
  // proto:  void QGraphicsLinearLayout::removeAt(int index);
  fn _ZN21QGraphicsLinearLayout8removeAtEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  void QGraphicsLinearLayout::insertStretch(int index, int stretch);
  fn _ZN21QGraphicsLinearLayout13insertStretchEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  void QGraphicsLinearLayout::setItemSpacing(int index, qreal spacing);
  fn _ZN21QGraphicsLinearLayout14setItemSpacingEid(qthis: *mut c_void, arg0: c_int, arg1: c_double) ;
  // proto:  void QGraphicsLinearLayout::removeItem(QGraphicsLayoutItem * item);
  fn _ZN21QGraphicsLinearLayout10removeItemEP19QGraphicsLayoutItem(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QGraphicsLinearLayout::stretchFactor(QGraphicsLayoutItem * item);
  fn _ZNK21QGraphicsLinearLayout13stretchFactorEP19QGraphicsLayoutItem(qthis: *mut c_void, arg0: *mut c_void) -> c_int;
}

// body block begin
// class sizeof(QGraphicsLinearLayout)=1
pub struct QGraphicsLinearLayout {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QGraphicsLinearLayout {
  pub fn spacing<RetType, T: QGraphicsLinearLayout_spacing<RetType>>(&mut self, value: T) -> RetType {
    return value.spacing(self);
    // return 1;
  }
}

pub trait QGraphicsLinearLayout_spacing<RetType> {
  fn spacing(self, rsthis: &mut QGraphicsLinearLayout) -> RetType;
}

// proto:  double QGraphicsLinearLayout::spacing();
impl<'a> /*trait*/ QGraphicsLinearLayout_spacing<f64> for () {
  fn spacing(self, rsthis: &mut QGraphicsLinearLayout) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QGraphicsLinearLayout7spacingEv()};
    let mut ret = unsafe {_ZNK21QGraphicsLinearLayout7spacingEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QGraphicsLinearLayout {
  pub fn NewQGraphicsLinearLayout<T: QGraphicsLinearLayout_NewQGraphicsLinearLayout>(value: T) -> QGraphicsLinearLayout {
    let rsthis = value.NewQGraphicsLinearLayout();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsLinearLayout_NewQGraphicsLinearLayout {
  fn NewQGraphicsLinearLayout(self) -> QGraphicsLinearLayout;
}

// proto: void QGraphicsLinearLayout::NewQGraphicsLinearLayout(QGraphicsLayoutItem * parent);
impl<'a> /*trait*/ QGraphicsLinearLayout_NewQGraphicsLinearLayout for (&'a mut QGraphicsLayoutItem) {
  fn NewQGraphicsLinearLayout(self) -> QGraphicsLinearLayout {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QGraphicsLinearLayoutC1EP19QGraphicsLayoutItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN21QGraphicsLinearLayoutC1EP19QGraphicsLayoutItem(qthis, arg0)};
    let rsthis = QGraphicsLinearLayout{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QGraphicsLinearLayout::NewQGraphicsLinearLayout(const QGraphicsLinearLayout & );
impl<'a> /*trait*/ QGraphicsLinearLayout_NewQGraphicsLinearLayout for (&'a  QGraphicsLinearLayout) {
  fn NewQGraphicsLinearLayout(self) -> QGraphicsLinearLayout {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QGraphicsLinearLayoutC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN21QGraphicsLinearLayoutC1ERKS_(qthis, arg0)};
    let rsthis = QGraphicsLinearLayout{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGraphicsLinearLayout {
  pub fn itemAt<RetType, T: QGraphicsLinearLayout_itemAt<RetType>>(&mut self, value: T) -> RetType {
    return value.itemAt(self);
    // return 1;
  }
}

pub trait QGraphicsLinearLayout_itemAt<RetType> {
  fn itemAt(self, rsthis: &mut QGraphicsLinearLayout) -> RetType;
}

// proto:  QGraphicsLayoutItem * QGraphicsLinearLayout::itemAt(int index);
impl<'a> /*trait*/ QGraphicsLinearLayout_itemAt<()> for (i32) {
  fn itemAt(self, rsthis: &mut QGraphicsLinearLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QGraphicsLinearLayout6itemAtEi()};
    let arg0 = self  as c_int;
     unsafe {_ZNK21QGraphicsLinearLayout6itemAtEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsLinearLayout {
  pub fn invalidate<RetType, T: QGraphicsLinearLayout_invalidate<RetType>>(&mut self, value: T) -> RetType {
    return value.invalidate(self);
    // return 1;
  }
}

pub trait QGraphicsLinearLayout_invalidate<RetType> {
  fn invalidate(self, rsthis: &mut QGraphicsLinearLayout) -> RetType;
}

// proto:  void QGraphicsLinearLayout::invalidate();
impl<'a> /*trait*/ QGraphicsLinearLayout_invalidate<()> for () {
  fn invalidate(self, rsthis: &mut QGraphicsLinearLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QGraphicsLinearLayout10invalidateEv()};
     unsafe {_ZN21QGraphicsLinearLayout10invalidateEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsLinearLayout {
  pub fn setGeometry<RetType, T: QGraphicsLinearLayout_setGeometry<RetType>>(&mut self, value: T) -> RetType {
    return value.setGeometry(self);
    // return 1;
  }
}

pub trait QGraphicsLinearLayout_setGeometry<RetType> {
  fn setGeometry(self, rsthis: &mut QGraphicsLinearLayout) -> RetType;
}

// proto:  void QGraphicsLinearLayout::setGeometry(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsLinearLayout_setGeometry<()> for (&'a  QRectF) {
  fn setGeometry(self, rsthis: &mut QGraphicsLinearLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QGraphicsLinearLayout11setGeometryERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN21QGraphicsLinearLayout11setGeometryERK6QRectF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsLinearLayout {
  pub fn addStretch<RetType, T: QGraphicsLinearLayout_addStretch<RetType>>(&mut self, value: T) -> RetType {
    return value.addStretch(self);
    // return 1;
  }
}

pub trait QGraphicsLinearLayout_addStretch<RetType> {
  fn addStretch(self, rsthis: &mut QGraphicsLinearLayout) -> RetType;
}

// proto:  void QGraphicsLinearLayout::addStretch(int stretch);
impl<'a> /*trait*/ QGraphicsLinearLayout_addStretch<()> for (i32) {
  fn addStretch(self, rsthis: &mut QGraphicsLinearLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QGraphicsLinearLayout10addStretchEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN21QGraphicsLinearLayout10addStretchEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsLinearLayout {
  pub fn count<RetType, T: QGraphicsLinearLayout_count<RetType>>(&mut self, value: T) -> RetType {
    return value.count(self);
    // return 1;
  }
}

pub trait QGraphicsLinearLayout_count<RetType> {
  fn count(self, rsthis: &mut QGraphicsLinearLayout) -> RetType;
}

// proto:  int QGraphicsLinearLayout::count();
impl<'a> /*trait*/ QGraphicsLinearLayout_count<i32> for () {
  fn count(self, rsthis: &mut QGraphicsLinearLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QGraphicsLinearLayout5countEv()};
    let mut ret = unsafe {_ZNK21QGraphicsLinearLayout5countEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QGraphicsLinearLayout {
  pub fn setSpacing<RetType, T: QGraphicsLinearLayout_setSpacing<RetType>>(&mut self, value: T) -> RetType {
    return value.setSpacing(self);
    // return 1;
  }
}

pub trait QGraphicsLinearLayout_setSpacing<RetType> {
  fn setSpacing(self, rsthis: &mut QGraphicsLinearLayout) -> RetType;
}

// proto:  void QGraphicsLinearLayout::setSpacing(qreal spacing);
impl<'a> /*trait*/ QGraphicsLinearLayout_setSpacing<()> for (f64) {
  fn setSpacing(self, rsthis: &mut QGraphicsLinearLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QGraphicsLinearLayout10setSpacingEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN21QGraphicsLinearLayout10setSpacingEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsLinearLayout {
  pub fn insertItem<RetType, T: QGraphicsLinearLayout_insertItem<RetType>>(&mut self, value: T) -> RetType {
    return value.insertItem(self);
    // return 1;
  }
}

pub trait QGraphicsLinearLayout_insertItem<RetType> {
  fn insertItem(self, rsthis: &mut QGraphicsLinearLayout) -> RetType;
}

// proto:  void QGraphicsLinearLayout::insertItem(int index, QGraphicsLayoutItem * item);
impl<'a> /*trait*/ QGraphicsLinearLayout_insertItem<()> for (i32, &'a mut QGraphicsLayoutItem) {
  fn insertItem(self, rsthis: &mut QGraphicsLinearLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QGraphicsLinearLayout10insertItemEiP19QGraphicsLayoutItem()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN21QGraphicsLinearLayout10insertItemEiP19QGraphicsLayoutItem(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsLinearLayout {
  pub fn FreeQGraphicsLinearLayout<RetType, T: QGraphicsLinearLayout_FreeQGraphicsLinearLayout<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQGraphicsLinearLayout(self);
    // return 1;
  }
}

pub trait QGraphicsLinearLayout_FreeQGraphicsLinearLayout<RetType> {
  fn FreeQGraphicsLinearLayout(self, rsthis: &mut QGraphicsLinearLayout) -> RetType;
}

// proto:  void QGraphicsLinearLayout::FreeQGraphicsLinearLayout();
impl<'a> /*trait*/ QGraphicsLinearLayout_FreeQGraphicsLinearLayout<()> for () {
  fn FreeQGraphicsLinearLayout(self, rsthis: &mut QGraphicsLinearLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QGraphicsLinearLayoutD0Ev()};
     unsafe {_ZN21QGraphicsLinearLayoutD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsLinearLayout {
  pub fn dump<RetType, T: QGraphicsLinearLayout_dump<RetType>>(&mut self, value: T) -> RetType {
    return value.dump(self);
    // return 1;
  }
}

pub trait QGraphicsLinearLayout_dump<RetType> {
  fn dump(self, rsthis: &mut QGraphicsLinearLayout) -> RetType;
}

// proto:  void QGraphicsLinearLayout::dump(int indent);
impl<'a> /*trait*/ QGraphicsLinearLayout_dump<()> for (i32) {
  fn dump(self, rsthis: &mut QGraphicsLinearLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QGraphicsLinearLayout4dumpEi()};
    let arg0 = self  as c_int;
     unsafe {_ZNK21QGraphicsLinearLayout4dumpEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsLinearLayout {
  pub fn setStretchFactor<RetType, T: QGraphicsLinearLayout_setStretchFactor<RetType>>(&mut self, value: T) -> RetType {
    return value.setStretchFactor(self);
    // return 1;
  }
}

pub trait QGraphicsLinearLayout_setStretchFactor<RetType> {
  fn setStretchFactor(self, rsthis: &mut QGraphicsLinearLayout) -> RetType;
}

// proto:  void QGraphicsLinearLayout::setStretchFactor(QGraphicsLayoutItem * item, int stretch);
impl<'a> /*trait*/ QGraphicsLinearLayout_setStretchFactor<()> for (&'a mut QGraphicsLayoutItem, i32) {
  fn setStretchFactor(self, rsthis: &mut QGraphicsLinearLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QGraphicsLinearLayout16setStretchFactorEP19QGraphicsLayoutItemi()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
     unsafe {_ZN21QGraphicsLinearLayout16setStretchFactorEP19QGraphicsLayoutItemi(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsLinearLayout {
  pub fn addItem<RetType, T: QGraphicsLinearLayout_addItem<RetType>>(&mut self, value: T) -> RetType {
    return value.addItem(self);
    // return 1;
  }
}

pub trait QGraphicsLinearLayout_addItem<RetType> {
  fn addItem(self, rsthis: &mut QGraphicsLinearLayout) -> RetType;
}

// proto:  void QGraphicsLinearLayout::addItem(QGraphicsLayoutItem * item);
impl<'a> /*trait*/ QGraphicsLinearLayout_addItem<()> for (&'a mut QGraphicsLayoutItem) {
  fn addItem(self, rsthis: &mut QGraphicsLinearLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QGraphicsLinearLayout7addItemEP19QGraphicsLayoutItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN21QGraphicsLinearLayout7addItemEP19QGraphicsLayoutItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsLinearLayout {
  pub fn itemSpacing<RetType, T: QGraphicsLinearLayout_itemSpacing<RetType>>(&mut self, value: T) -> RetType {
    return value.itemSpacing(self);
    // return 1;
  }
}

pub trait QGraphicsLinearLayout_itemSpacing<RetType> {
  fn itemSpacing(self, rsthis: &mut QGraphicsLinearLayout) -> RetType;
}

// proto:  double QGraphicsLinearLayout::itemSpacing(int index);
impl<'a> /*trait*/ QGraphicsLinearLayout_itemSpacing<f64> for (i32) {
  fn itemSpacing(self, rsthis: &mut QGraphicsLinearLayout) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QGraphicsLinearLayout11itemSpacingEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK21QGraphicsLinearLayout11itemSpacingEi(rsthis.qclsinst, arg0)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QGraphicsLinearLayout {
  pub fn removeAt<RetType, T: QGraphicsLinearLayout_removeAt<RetType>>(&mut self, value: T) -> RetType {
    return value.removeAt(self);
    // return 1;
  }
}

pub trait QGraphicsLinearLayout_removeAt<RetType> {
  fn removeAt(self, rsthis: &mut QGraphicsLinearLayout) -> RetType;
}

// proto:  void QGraphicsLinearLayout::removeAt(int index);
impl<'a> /*trait*/ QGraphicsLinearLayout_removeAt<()> for (i32) {
  fn removeAt(self, rsthis: &mut QGraphicsLinearLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QGraphicsLinearLayout8removeAtEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN21QGraphicsLinearLayout8removeAtEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsLinearLayout {
  pub fn insertStretch<RetType, T: QGraphicsLinearLayout_insertStretch<RetType>>(&mut self, value: T) -> RetType {
    return value.insertStretch(self);
    // return 1;
  }
}

pub trait QGraphicsLinearLayout_insertStretch<RetType> {
  fn insertStretch(self, rsthis: &mut QGraphicsLinearLayout) -> RetType;
}

// proto:  void QGraphicsLinearLayout::insertStretch(int index, int stretch);
impl<'a> /*trait*/ QGraphicsLinearLayout_insertStretch<()> for (i32, i32) {
  fn insertStretch(self, rsthis: &mut QGraphicsLinearLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QGraphicsLinearLayout13insertStretchEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN21QGraphicsLinearLayout13insertStretchEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsLinearLayout {
  pub fn setItemSpacing<RetType, T: QGraphicsLinearLayout_setItemSpacing<RetType>>(&mut self, value: T) -> RetType {
    return value.setItemSpacing(self);
    // return 1;
  }
}

pub trait QGraphicsLinearLayout_setItemSpacing<RetType> {
  fn setItemSpacing(self, rsthis: &mut QGraphicsLinearLayout) -> RetType;
}

// proto:  void QGraphicsLinearLayout::setItemSpacing(int index, qreal spacing);
impl<'a> /*trait*/ QGraphicsLinearLayout_setItemSpacing<()> for (i32, f64) {
  fn setItemSpacing(self, rsthis: &mut QGraphicsLinearLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QGraphicsLinearLayout14setItemSpacingEid()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_double;
     unsafe {_ZN21QGraphicsLinearLayout14setItemSpacingEid(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsLinearLayout {
  pub fn removeItem<RetType, T: QGraphicsLinearLayout_removeItem<RetType>>(&mut self, value: T) -> RetType {
    return value.removeItem(self);
    // return 1;
  }
}

pub trait QGraphicsLinearLayout_removeItem<RetType> {
  fn removeItem(self, rsthis: &mut QGraphicsLinearLayout) -> RetType;
}

// proto:  void QGraphicsLinearLayout::removeItem(QGraphicsLayoutItem * item);
impl<'a> /*trait*/ QGraphicsLinearLayout_removeItem<()> for (&'a mut QGraphicsLayoutItem) {
  fn removeItem(self, rsthis: &mut QGraphicsLinearLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN21QGraphicsLinearLayout10removeItemEP19QGraphicsLayoutItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN21QGraphicsLinearLayout10removeItemEP19QGraphicsLayoutItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsLinearLayout {
  pub fn stretchFactor<RetType, T: QGraphicsLinearLayout_stretchFactor<RetType>>(&mut self, value: T) -> RetType {
    return value.stretchFactor(self);
    // return 1;
  }
}

pub trait QGraphicsLinearLayout_stretchFactor<RetType> {
  fn stretchFactor(self, rsthis: &mut QGraphicsLinearLayout) -> RetType;
}

// proto:  int QGraphicsLinearLayout::stretchFactor(QGraphicsLayoutItem * item);
impl<'a> /*trait*/ QGraphicsLinearLayout_stretchFactor<i32> for (&'a mut QGraphicsLayoutItem) {
  fn stretchFactor(self, rsthis: &mut QGraphicsLinearLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK21QGraphicsLinearLayout13stretchFactorEP19QGraphicsLayoutItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK21QGraphicsLinearLayout13stretchFactorEP19QGraphicsLayoutItem(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

