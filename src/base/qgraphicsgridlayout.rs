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
  // proto:  void QGraphicsGridLayout::setRowPreferredHeight(int row, qreal height);
  fn _ZN19QGraphicsGridLayout21setRowPreferredHeightEid(qthis: *mut c_void, arg0: c_int, arg1: c_double) ;
  // proto:  int QGraphicsGridLayout::columnCount();
  fn _ZNK19QGraphicsGridLayout11columnCountEv(qthis: *mut c_void) -> c_int;
  // proto:  QGraphicsLayoutItem * QGraphicsGridLayout::itemAt(int index);
  fn _ZNK19QGraphicsGridLayout6itemAtEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  int QGraphicsGridLayout::count();
  fn _ZNK19QGraphicsGridLayout5countEv(qthis: *mut c_void) -> c_int;
  // proto:  void QGraphicsGridLayout::setColumnFixedWidth(int column, qreal width);
  fn _ZN19QGraphicsGridLayout19setColumnFixedWidthEid(qthis: *mut c_void, arg0: c_int, arg1: c_double) ;
  // proto:  void QGraphicsGridLayout::setColumnMaximumWidth(int column, qreal width);
  fn _ZN19QGraphicsGridLayout21setColumnMaximumWidthEid(qthis: *mut c_void, arg0: c_int, arg1: c_double) ;
  // proto:  int QGraphicsGridLayout::rowStretchFactor(int row);
  fn _ZNK19QGraphicsGridLayout16rowStretchFactorEi(qthis: *mut c_void, arg0: c_int) -> c_int;
  // proto:  double QGraphicsGridLayout::verticalSpacing();
  fn _ZNK19QGraphicsGridLayout15verticalSpacingEv(qthis: *mut c_void) -> c_double;
  // proto:  int QGraphicsGridLayout::columnStretchFactor(int column);
  fn _ZNK19QGraphicsGridLayout19columnStretchFactorEi(qthis: *mut c_void, arg0: c_int) -> c_int;
  // proto:  void QGraphicsGridLayout::setRowMaximumHeight(int row, qreal height);
  fn _ZN19QGraphicsGridLayout19setRowMaximumHeightEid(qthis: *mut c_void, arg0: c_int, arg1: c_double) ;
  // proto:  void QGraphicsGridLayout::removeItem(QGraphicsLayoutItem * item);
  fn _ZN19QGraphicsGridLayout10removeItemEP19QGraphicsLayoutItem(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGraphicsGridLayout::FreeQGraphicsGridLayout();
  fn _ZN19QGraphicsGridLayoutD0Ev(qthis: *mut c_void) ;
  // proto:  double QGraphicsGridLayout::rowMinimumHeight(int row);
  fn _ZNK19QGraphicsGridLayout16rowMinimumHeightEi(qthis: *mut c_void, arg0: c_int) -> c_double;
  // proto:  double QGraphicsGridLayout::rowMaximumHeight(int row);
  fn _ZNK19QGraphicsGridLayout16rowMaximumHeightEi(qthis: *mut c_void, arg0: c_int) -> c_double;
  // proto:  void QGraphicsGridLayout::NewQGraphicsGridLayout(QGraphicsLayoutItem * parent);
  fn _ZN19QGraphicsGridLayoutC1EP19QGraphicsLayoutItem(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGraphicsGridLayout::setColumnSpacing(int column, qreal spacing);
  fn _ZN19QGraphicsGridLayout16setColumnSpacingEid(qthis: *mut c_void, arg0: c_int, arg1: c_double) ;
  // proto:  double QGraphicsGridLayout::rowSpacing(int row);
  fn _ZNK19QGraphicsGridLayout10rowSpacingEi(qthis: *mut c_void, arg0: c_int) -> c_double;
  // proto:  double QGraphicsGridLayout::columnMaximumWidth(int column);
  fn _ZNK19QGraphicsGridLayout18columnMaximumWidthEi(qthis: *mut c_void, arg0: c_int) -> c_double;
  // proto:  void QGraphicsGridLayout::setRowFixedHeight(int row, qreal height);
  fn _ZN19QGraphicsGridLayout17setRowFixedHeightEid(qthis: *mut c_void, arg0: c_int, arg1: c_double) ;
  // proto:  double QGraphicsGridLayout::rowPreferredHeight(int row);
  fn _ZNK19QGraphicsGridLayout18rowPreferredHeightEi(qthis: *mut c_void, arg0: c_int) -> c_double;
  // proto:  QGraphicsLayoutItem * QGraphicsGridLayout::itemAt(int row, int column);
  fn _ZNK19QGraphicsGridLayout6itemAtEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  void QGraphicsGridLayout::setVerticalSpacing(qreal spacing);
  fn _ZN19QGraphicsGridLayout18setVerticalSpacingEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  void QGraphicsGridLayout::setGeometry(const QRectF & rect);
  fn _ZN19QGraphicsGridLayout11setGeometryERK6QRectF(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QGraphicsGridLayout::rowCount();
  fn _ZNK19QGraphicsGridLayout8rowCountEv(qthis: *mut c_void) -> c_int;
  // proto:  void QGraphicsGridLayout::setSpacing(qreal spacing);
  fn _ZN19QGraphicsGridLayout10setSpacingEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  void QGraphicsGridLayout::setRowStretchFactor(int row, int stretch);
  fn _ZN19QGraphicsGridLayout19setRowStretchFactorEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  double QGraphicsGridLayout::columnMinimumWidth(int column);
  fn _ZNK19QGraphicsGridLayout18columnMinimumWidthEi(qthis: *mut c_void, arg0: c_int) -> c_double;
  // proto:  void QGraphicsGridLayout::setColumnMinimumWidth(int column, qreal width);
  fn _ZN19QGraphicsGridLayout21setColumnMinimumWidthEid(qthis: *mut c_void, arg0: c_int, arg1: c_double) ;
  // proto:  void QGraphicsGridLayout::setRowMinimumHeight(int row, qreal height);
  fn _ZN19QGraphicsGridLayout19setRowMinimumHeightEid(qthis: *mut c_void, arg0: c_int, arg1: c_double) ;
  // proto:  void QGraphicsGridLayout::setHorizontalSpacing(qreal spacing);
  fn _ZN19QGraphicsGridLayout20setHorizontalSpacingEd(qthis: *mut c_void, arg0: c_double) ;
  // proto:  double QGraphicsGridLayout::horizontalSpacing();
  fn _ZNK19QGraphicsGridLayout17horizontalSpacingEv(qthis: *mut c_void) -> c_double;
  // proto:  void QGraphicsGridLayout::setColumnStretchFactor(int column, int stretch);
  fn _ZN19QGraphicsGridLayout22setColumnStretchFactorEii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  void QGraphicsGridLayout::invalidate();
  fn _ZN19QGraphicsGridLayout10invalidateEv(qthis: *mut c_void) ;
  // proto:  double QGraphicsGridLayout::columnPreferredWidth(int column);
  fn _ZNK19QGraphicsGridLayout20columnPreferredWidthEi(qthis: *mut c_void, arg0: c_int) -> c_double;
  // proto:  void QGraphicsGridLayout::setColumnPreferredWidth(int column, qreal width);
  fn _ZN19QGraphicsGridLayout23setColumnPreferredWidthEid(qthis: *mut c_void, arg0: c_int, arg1: c_double) ;
  // proto:  double QGraphicsGridLayout::columnSpacing(int column);
  fn _ZNK19QGraphicsGridLayout13columnSpacingEi(qthis: *mut c_void, arg0: c_int) -> c_double;
  // proto:  void QGraphicsGridLayout::NewQGraphicsGridLayout(const QGraphicsGridLayout & );
  fn _ZN19QGraphicsGridLayoutC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGraphicsGridLayout::setRowSpacing(int row, qreal spacing);
  fn _ZN19QGraphicsGridLayout13setRowSpacingEid(qthis: *mut c_void, arg0: c_int, arg1: c_double) ;
  // proto:  void QGraphicsGridLayout::removeAt(int index);
  fn _ZN19QGraphicsGridLayout8removeAtEi(qthis: *mut c_void, arg0: c_int) ;
}

// body block begin
// class sizeof(QGraphicsGridLayout)=1
pub struct QGraphicsGridLayout {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QGraphicsGridLayout {
  pub fn setRowPreferredHeight<T: QGraphicsGridLayout_setRowPreferredHeight>(&mut self, value: T)  {
     value.setRowPreferredHeight(self);
    // return 1;
  }
}

pub trait QGraphicsGridLayout_setRowPreferredHeight {
  fn setRowPreferredHeight(self, rsthis: &mut QGraphicsGridLayout) ;
}

// proto:  void QGraphicsGridLayout::setRowPreferredHeight(int row, qreal height);
impl<'a> /*trait*/ QGraphicsGridLayout_setRowPreferredHeight for (i32, f64) {
  fn setRowPreferredHeight(self, rsthis: &mut QGraphicsGridLayout)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsGridLayout21setRowPreferredHeightEid()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_double;
     unsafe {_ZN19QGraphicsGridLayout21setRowPreferredHeightEid(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsGridLayout {
  pub fn columnCount<T: QGraphicsGridLayout_columnCount>(&mut self, value: T) -> i32 {
    return value.columnCount(self);
    // return 1;
  }
}

pub trait QGraphicsGridLayout_columnCount {
  fn columnCount(self, rsthis: &mut QGraphicsGridLayout) -> i32;
}

// proto:  int QGraphicsGridLayout::columnCount();
impl<'a> /*trait*/ QGraphicsGridLayout_columnCount for () {
  fn columnCount(self, rsthis: &mut QGraphicsGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsGridLayout11columnCountEv()};
    let mut ret = unsafe {_ZNK19QGraphicsGridLayout11columnCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QGraphicsGridLayout {
  pub fn itemAt<T: QGraphicsGridLayout_itemAt>(&mut self, value: T)  {
     value.itemAt(self);
    // return 1;
  }
}

pub trait QGraphicsGridLayout_itemAt {
  fn itemAt(self, rsthis: &mut QGraphicsGridLayout) ;
}

// proto:  QGraphicsLayoutItem * QGraphicsGridLayout::itemAt(int index);
impl<'a> /*trait*/ QGraphicsGridLayout_itemAt for (i32) {
  fn itemAt(self, rsthis: &mut QGraphicsGridLayout)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsGridLayout6itemAtEi()};
    let arg0 = self  as c_int;
     unsafe {_ZNK19QGraphicsGridLayout6itemAtEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsGridLayout {
  pub fn count<T: QGraphicsGridLayout_count>(&mut self, value: T) -> i32 {
    return value.count(self);
    // return 1;
  }
}

pub trait QGraphicsGridLayout_count {
  fn count(self, rsthis: &mut QGraphicsGridLayout) -> i32;
}

// proto:  int QGraphicsGridLayout::count();
impl<'a> /*trait*/ QGraphicsGridLayout_count for () {
  fn count(self, rsthis: &mut QGraphicsGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsGridLayout5countEv()};
    let mut ret = unsafe {_ZNK19QGraphicsGridLayout5countEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QGraphicsGridLayout {
  pub fn setColumnFixedWidth<T: QGraphicsGridLayout_setColumnFixedWidth>(&mut self, value: T)  {
     value.setColumnFixedWidth(self);
    // return 1;
  }
}

pub trait QGraphicsGridLayout_setColumnFixedWidth {
  fn setColumnFixedWidth(self, rsthis: &mut QGraphicsGridLayout) ;
}

// proto:  void QGraphicsGridLayout::setColumnFixedWidth(int column, qreal width);
impl<'a> /*trait*/ QGraphicsGridLayout_setColumnFixedWidth for (i32, f64) {
  fn setColumnFixedWidth(self, rsthis: &mut QGraphicsGridLayout)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsGridLayout19setColumnFixedWidthEid()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_double;
     unsafe {_ZN19QGraphicsGridLayout19setColumnFixedWidthEid(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsGridLayout {
  pub fn setColumnMaximumWidth<T: QGraphicsGridLayout_setColumnMaximumWidth>(&mut self, value: T)  {
     value.setColumnMaximumWidth(self);
    // return 1;
  }
}

pub trait QGraphicsGridLayout_setColumnMaximumWidth {
  fn setColumnMaximumWidth(self, rsthis: &mut QGraphicsGridLayout) ;
}

// proto:  void QGraphicsGridLayout::setColumnMaximumWidth(int column, qreal width);
impl<'a> /*trait*/ QGraphicsGridLayout_setColumnMaximumWidth for (i32, f64) {
  fn setColumnMaximumWidth(self, rsthis: &mut QGraphicsGridLayout)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsGridLayout21setColumnMaximumWidthEid()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_double;
     unsafe {_ZN19QGraphicsGridLayout21setColumnMaximumWidthEid(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsGridLayout {
  pub fn rowStretchFactor<T: QGraphicsGridLayout_rowStretchFactor>(&mut self, value: T) -> i32 {
    return value.rowStretchFactor(self);
    // return 1;
  }
}

pub trait QGraphicsGridLayout_rowStretchFactor {
  fn rowStretchFactor(self, rsthis: &mut QGraphicsGridLayout) -> i32;
}

// proto:  int QGraphicsGridLayout::rowStretchFactor(int row);
impl<'a> /*trait*/ QGraphicsGridLayout_rowStretchFactor for (i32) {
  fn rowStretchFactor(self, rsthis: &mut QGraphicsGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsGridLayout16rowStretchFactorEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK19QGraphicsGridLayout16rowStretchFactorEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QGraphicsGridLayout {
  pub fn verticalSpacing<T: QGraphicsGridLayout_verticalSpacing>(&mut self, value: T) -> f64 {
    return value.verticalSpacing(self);
    // return 1;
  }
}

pub trait QGraphicsGridLayout_verticalSpacing {
  fn verticalSpacing(self, rsthis: &mut QGraphicsGridLayout) -> f64;
}

// proto:  double QGraphicsGridLayout::verticalSpacing();
impl<'a> /*trait*/ QGraphicsGridLayout_verticalSpacing for () {
  fn verticalSpacing(self, rsthis: &mut QGraphicsGridLayout) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsGridLayout15verticalSpacingEv()};
    let mut ret = unsafe {_ZNK19QGraphicsGridLayout15verticalSpacingEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QGraphicsGridLayout {
  pub fn columnStretchFactor<T: QGraphicsGridLayout_columnStretchFactor>(&mut self, value: T) -> i32 {
    return value.columnStretchFactor(self);
    // return 1;
  }
}

pub trait QGraphicsGridLayout_columnStretchFactor {
  fn columnStretchFactor(self, rsthis: &mut QGraphicsGridLayout) -> i32;
}

// proto:  int QGraphicsGridLayout::columnStretchFactor(int column);
impl<'a> /*trait*/ QGraphicsGridLayout_columnStretchFactor for (i32) {
  fn columnStretchFactor(self, rsthis: &mut QGraphicsGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsGridLayout19columnStretchFactorEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK19QGraphicsGridLayout19columnStretchFactorEi(rsthis.qclsinst, arg0)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QGraphicsGridLayout {
  pub fn setRowMaximumHeight<T: QGraphicsGridLayout_setRowMaximumHeight>(&mut self, value: T)  {
     value.setRowMaximumHeight(self);
    // return 1;
  }
}

pub trait QGraphicsGridLayout_setRowMaximumHeight {
  fn setRowMaximumHeight(self, rsthis: &mut QGraphicsGridLayout) ;
}

// proto:  void QGraphicsGridLayout::setRowMaximumHeight(int row, qreal height);
impl<'a> /*trait*/ QGraphicsGridLayout_setRowMaximumHeight for (i32, f64) {
  fn setRowMaximumHeight(self, rsthis: &mut QGraphicsGridLayout)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsGridLayout19setRowMaximumHeightEid()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_double;
     unsafe {_ZN19QGraphicsGridLayout19setRowMaximumHeightEid(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsGridLayout {
  pub fn removeItem<T: QGraphicsGridLayout_removeItem>(&mut self, value: T)  {
     value.removeItem(self);
    // return 1;
  }
}

pub trait QGraphicsGridLayout_removeItem {
  fn removeItem(self, rsthis: &mut QGraphicsGridLayout) ;
}

// proto:  void QGraphicsGridLayout::removeItem(QGraphicsLayoutItem * item);
impl<'a> /*trait*/ QGraphicsGridLayout_removeItem for (&'a mut QGraphicsLayoutItem) {
  fn removeItem(self, rsthis: &mut QGraphicsGridLayout)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsGridLayout10removeItemEP19QGraphicsLayoutItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN19QGraphicsGridLayout10removeItemEP19QGraphicsLayoutItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsGridLayout {
  pub fn FreeQGraphicsGridLayout<T: QGraphicsGridLayout_FreeQGraphicsGridLayout>(&mut self, value: T)  {
     value.FreeQGraphicsGridLayout(self);
    // return 1;
  }
}

pub trait QGraphicsGridLayout_FreeQGraphicsGridLayout {
  fn FreeQGraphicsGridLayout(self, rsthis: &mut QGraphicsGridLayout) ;
}

// proto:  void QGraphicsGridLayout::FreeQGraphicsGridLayout();
impl<'a> /*trait*/ QGraphicsGridLayout_FreeQGraphicsGridLayout for () {
  fn FreeQGraphicsGridLayout(self, rsthis: &mut QGraphicsGridLayout)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsGridLayoutD0Ev()};
     unsafe {_ZN19QGraphicsGridLayoutD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsGridLayout {
  pub fn rowMinimumHeight<T: QGraphicsGridLayout_rowMinimumHeight>(&mut self, value: T) -> f64 {
    return value.rowMinimumHeight(self);
    // return 1;
  }
}

pub trait QGraphicsGridLayout_rowMinimumHeight {
  fn rowMinimumHeight(self, rsthis: &mut QGraphicsGridLayout) -> f64;
}

// proto:  double QGraphicsGridLayout::rowMinimumHeight(int row);
impl<'a> /*trait*/ QGraphicsGridLayout_rowMinimumHeight for (i32) {
  fn rowMinimumHeight(self, rsthis: &mut QGraphicsGridLayout) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsGridLayout16rowMinimumHeightEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK19QGraphicsGridLayout16rowMinimumHeightEi(rsthis.qclsinst, arg0)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QGraphicsGridLayout {
  pub fn rowMaximumHeight<T: QGraphicsGridLayout_rowMaximumHeight>(&mut self, value: T) -> f64 {
    return value.rowMaximumHeight(self);
    // return 1;
  }
}

pub trait QGraphicsGridLayout_rowMaximumHeight {
  fn rowMaximumHeight(self, rsthis: &mut QGraphicsGridLayout) -> f64;
}

// proto:  double QGraphicsGridLayout::rowMaximumHeight(int row);
impl<'a> /*trait*/ QGraphicsGridLayout_rowMaximumHeight for (i32) {
  fn rowMaximumHeight(self, rsthis: &mut QGraphicsGridLayout) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsGridLayout16rowMaximumHeightEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK19QGraphicsGridLayout16rowMaximumHeightEi(rsthis.qclsinst, arg0)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QGraphicsGridLayout {
  pub fn NewQGraphicsGridLayout<T: QGraphicsGridLayout_NewQGraphicsGridLayout>(value: T) -> QGraphicsGridLayout {
    let rsthis = value.NewQGraphicsGridLayout();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsGridLayout_NewQGraphicsGridLayout {
  fn NewQGraphicsGridLayout(self) -> QGraphicsGridLayout;
}

// proto: void QGraphicsGridLayout::NewQGraphicsGridLayout(QGraphicsLayoutItem * parent);
impl<'a> /*trait*/ QGraphicsGridLayout_NewQGraphicsGridLayout for (&'a mut QGraphicsLayoutItem) {
  fn NewQGraphicsGridLayout(self) -> QGraphicsGridLayout {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsGridLayoutC1EP19QGraphicsLayoutItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN19QGraphicsGridLayoutC1EP19QGraphicsLayoutItem(qthis, arg0)};
    let rsthis = QGraphicsGridLayout{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGraphicsGridLayout {
  pub fn setColumnSpacing<T: QGraphicsGridLayout_setColumnSpacing>(&mut self, value: T)  {
     value.setColumnSpacing(self);
    // return 1;
  }
}

pub trait QGraphicsGridLayout_setColumnSpacing {
  fn setColumnSpacing(self, rsthis: &mut QGraphicsGridLayout) ;
}

// proto:  void QGraphicsGridLayout::setColumnSpacing(int column, qreal spacing);
impl<'a> /*trait*/ QGraphicsGridLayout_setColumnSpacing for (i32, f64) {
  fn setColumnSpacing(self, rsthis: &mut QGraphicsGridLayout)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsGridLayout16setColumnSpacingEid()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_double;
     unsafe {_ZN19QGraphicsGridLayout16setColumnSpacingEid(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsGridLayout {
  pub fn rowSpacing<T: QGraphicsGridLayout_rowSpacing>(&mut self, value: T) -> f64 {
    return value.rowSpacing(self);
    // return 1;
  }
}

pub trait QGraphicsGridLayout_rowSpacing {
  fn rowSpacing(self, rsthis: &mut QGraphicsGridLayout) -> f64;
}

// proto:  double QGraphicsGridLayout::rowSpacing(int row);
impl<'a> /*trait*/ QGraphicsGridLayout_rowSpacing for (i32) {
  fn rowSpacing(self, rsthis: &mut QGraphicsGridLayout) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsGridLayout10rowSpacingEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK19QGraphicsGridLayout10rowSpacingEi(rsthis.qclsinst, arg0)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QGraphicsGridLayout {
  pub fn columnMaximumWidth<T: QGraphicsGridLayout_columnMaximumWidth>(&mut self, value: T) -> f64 {
    return value.columnMaximumWidth(self);
    // return 1;
  }
}

pub trait QGraphicsGridLayout_columnMaximumWidth {
  fn columnMaximumWidth(self, rsthis: &mut QGraphicsGridLayout) -> f64;
}

// proto:  double QGraphicsGridLayout::columnMaximumWidth(int column);
impl<'a> /*trait*/ QGraphicsGridLayout_columnMaximumWidth for (i32) {
  fn columnMaximumWidth(self, rsthis: &mut QGraphicsGridLayout) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsGridLayout18columnMaximumWidthEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK19QGraphicsGridLayout18columnMaximumWidthEi(rsthis.qclsinst, arg0)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QGraphicsGridLayout {
  pub fn setRowFixedHeight<T: QGraphicsGridLayout_setRowFixedHeight>(&mut self, value: T)  {
     value.setRowFixedHeight(self);
    // return 1;
  }
}

pub trait QGraphicsGridLayout_setRowFixedHeight {
  fn setRowFixedHeight(self, rsthis: &mut QGraphicsGridLayout) ;
}

// proto:  void QGraphicsGridLayout::setRowFixedHeight(int row, qreal height);
impl<'a> /*trait*/ QGraphicsGridLayout_setRowFixedHeight for (i32, f64) {
  fn setRowFixedHeight(self, rsthis: &mut QGraphicsGridLayout)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsGridLayout17setRowFixedHeightEid()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_double;
     unsafe {_ZN19QGraphicsGridLayout17setRowFixedHeightEid(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsGridLayout {
  pub fn rowPreferredHeight<T: QGraphicsGridLayout_rowPreferredHeight>(&mut self, value: T) -> f64 {
    return value.rowPreferredHeight(self);
    // return 1;
  }
}

pub trait QGraphicsGridLayout_rowPreferredHeight {
  fn rowPreferredHeight(self, rsthis: &mut QGraphicsGridLayout) -> f64;
}

// proto:  double QGraphicsGridLayout::rowPreferredHeight(int row);
impl<'a> /*trait*/ QGraphicsGridLayout_rowPreferredHeight for (i32) {
  fn rowPreferredHeight(self, rsthis: &mut QGraphicsGridLayout) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsGridLayout18rowPreferredHeightEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK19QGraphicsGridLayout18rowPreferredHeightEi(rsthis.qclsinst, arg0)};
    return ret as f64;
    // return 1;
  }
}

// proto:  QGraphicsLayoutItem * QGraphicsGridLayout::itemAt(int row, int column);
impl<'a> /*trait*/ QGraphicsGridLayout_itemAt for (i32, i32) {
  fn itemAt(self, rsthis: &mut QGraphicsGridLayout)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsGridLayout6itemAtEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZNK19QGraphicsGridLayout6itemAtEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsGridLayout {
  pub fn setVerticalSpacing<T: QGraphicsGridLayout_setVerticalSpacing>(&mut self, value: T)  {
     value.setVerticalSpacing(self);
    // return 1;
  }
}

pub trait QGraphicsGridLayout_setVerticalSpacing {
  fn setVerticalSpacing(self, rsthis: &mut QGraphicsGridLayout) ;
}

// proto:  void QGraphicsGridLayout::setVerticalSpacing(qreal spacing);
impl<'a> /*trait*/ QGraphicsGridLayout_setVerticalSpacing for (f64) {
  fn setVerticalSpacing(self, rsthis: &mut QGraphicsGridLayout)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsGridLayout18setVerticalSpacingEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN19QGraphicsGridLayout18setVerticalSpacingEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsGridLayout {
  pub fn setGeometry<T: QGraphicsGridLayout_setGeometry>(&mut self, value: T)  {
     value.setGeometry(self);
    // return 1;
  }
}

pub trait QGraphicsGridLayout_setGeometry {
  fn setGeometry(self, rsthis: &mut QGraphicsGridLayout) ;
}

// proto:  void QGraphicsGridLayout::setGeometry(const QRectF & rect);
impl<'a> /*trait*/ QGraphicsGridLayout_setGeometry for (&'a  QRectF) {
  fn setGeometry(self, rsthis: &mut QGraphicsGridLayout)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsGridLayout11setGeometryERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN19QGraphicsGridLayout11setGeometryERK6QRectF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsGridLayout {
  pub fn rowCount<T: QGraphicsGridLayout_rowCount>(&mut self, value: T) -> i32 {
    return value.rowCount(self);
    // return 1;
  }
}

pub trait QGraphicsGridLayout_rowCount {
  fn rowCount(self, rsthis: &mut QGraphicsGridLayout) -> i32;
}

// proto:  int QGraphicsGridLayout::rowCount();
impl<'a> /*trait*/ QGraphicsGridLayout_rowCount for () {
  fn rowCount(self, rsthis: &mut QGraphicsGridLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsGridLayout8rowCountEv()};
    let mut ret = unsafe {_ZNK19QGraphicsGridLayout8rowCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QGraphicsGridLayout {
  pub fn setSpacing<T: QGraphicsGridLayout_setSpacing>(&mut self, value: T)  {
     value.setSpacing(self);
    // return 1;
  }
}

pub trait QGraphicsGridLayout_setSpacing {
  fn setSpacing(self, rsthis: &mut QGraphicsGridLayout) ;
}

// proto:  void QGraphicsGridLayout::setSpacing(qreal spacing);
impl<'a> /*trait*/ QGraphicsGridLayout_setSpacing for (f64) {
  fn setSpacing(self, rsthis: &mut QGraphicsGridLayout)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsGridLayout10setSpacingEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN19QGraphicsGridLayout10setSpacingEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsGridLayout {
  pub fn setRowStretchFactor<T: QGraphicsGridLayout_setRowStretchFactor>(&mut self, value: T)  {
     value.setRowStretchFactor(self);
    // return 1;
  }
}

pub trait QGraphicsGridLayout_setRowStretchFactor {
  fn setRowStretchFactor(self, rsthis: &mut QGraphicsGridLayout) ;
}

// proto:  void QGraphicsGridLayout::setRowStretchFactor(int row, int stretch);
impl<'a> /*trait*/ QGraphicsGridLayout_setRowStretchFactor for (i32, i32) {
  fn setRowStretchFactor(self, rsthis: &mut QGraphicsGridLayout)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsGridLayout19setRowStretchFactorEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN19QGraphicsGridLayout19setRowStretchFactorEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsGridLayout {
  pub fn columnMinimumWidth<T: QGraphicsGridLayout_columnMinimumWidth>(&mut self, value: T) -> f64 {
    return value.columnMinimumWidth(self);
    // return 1;
  }
}

pub trait QGraphicsGridLayout_columnMinimumWidth {
  fn columnMinimumWidth(self, rsthis: &mut QGraphicsGridLayout) -> f64;
}

// proto:  double QGraphicsGridLayout::columnMinimumWidth(int column);
impl<'a> /*trait*/ QGraphicsGridLayout_columnMinimumWidth for (i32) {
  fn columnMinimumWidth(self, rsthis: &mut QGraphicsGridLayout) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsGridLayout18columnMinimumWidthEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK19QGraphicsGridLayout18columnMinimumWidthEi(rsthis.qclsinst, arg0)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QGraphicsGridLayout {
  pub fn setColumnMinimumWidth<T: QGraphicsGridLayout_setColumnMinimumWidth>(&mut self, value: T)  {
     value.setColumnMinimumWidth(self);
    // return 1;
  }
}

pub trait QGraphicsGridLayout_setColumnMinimumWidth {
  fn setColumnMinimumWidth(self, rsthis: &mut QGraphicsGridLayout) ;
}

// proto:  void QGraphicsGridLayout::setColumnMinimumWidth(int column, qreal width);
impl<'a> /*trait*/ QGraphicsGridLayout_setColumnMinimumWidth for (i32, f64) {
  fn setColumnMinimumWidth(self, rsthis: &mut QGraphicsGridLayout)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsGridLayout21setColumnMinimumWidthEid()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_double;
     unsafe {_ZN19QGraphicsGridLayout21setColumnMinimumWidthEid(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsGridLayout {
  pub fn setRowMinimumHeight<T: QGraphicsGridLayout_setRowMinimumHeight>(&mut self, value: T)  {
     value.setRowMinimumHeight(self);
    // return 1;
  }
}

pub trait QGraphicsGridLayout_setRowMinimumHeight {
  fn setRowMinimumHeight(self, rsthis: &mut QGraphicsGridLayout) ;
}

// proto:  void QGraphicsGridLayout::setRowMinimumHeight(int row, qreal height);
impl<'a> /*trait*/ QGraphicsGridLayout_setRowMinimumHeight for (i32, f64) {
  fn setRowMinimumHeight(self, rsthis: &mut QGraphicsGridLayout)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsGridLayout19setRowMinimumHeightEid()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_double;
     unsafe {_ZN19QGraphicsGridLayout19setRowMinimumHeightEid(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsGridLayout {
  pub fn setHorizontalSpacing<T: QGraphicsGridLayout_setHorizontalSpacing>(&mut self, value: T)  {
     value.setHorizontalSpacing(self);
    // return 1;
  }
}

pub trait QGraphicsGridLayout_setHorizontalSpacing {
  fn setHorizontalSpacing(self, rsthis: &mut QGraphicsGridLayout) ;
}

// proto:  void QGraphicsGridLayout::setHorizontalSpacing(qreal spacing);
impl<'a> /*trait*/ QGraphicsGridLayout_setHorizontalSpacing for (f64) {
  fn setHorizontalSpacing(self, rsthis: &mut QGraphicsGridLayout)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsGridLayout20setHorizontalSpacingEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN19QGraphicsGridLayout20setHorizontalSpacingEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsGridLayout {
  pub fn horizontalSpacing<T: QGraphicsGridLayout_horizontalSpacing>(&mut self, value: T) -> f64 {
    return value.horizontalSpacing(self);
    // return 1;
  }
}

pub trait QGraphicsGridLayout_horizontalSpacing {
  fn horizontalSpacing(self, rsthis: &mut QGraphicsGridLayout) -> f64;
}

// proto:  double QGraphicsGridLayout::horizontalSpacing();
impl<'a> /*trait*/ QGraphicsGridLayout_horizontalSpacing for () {
  fn horizontalSpacing(self, rsthis: &mut QGraphicsGridLayout) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsGridLayout17horizontalSpacingEv()};
    let mut ret = unsafe {_ZNK19QGraphicsGridLayout17horizontalSpacingEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QGraphicsGridLayout {
  pub fn setColumnStretchFactor<T: QGraphicsGridLayout_setColumnStretchFactor>(&mut self, value: T)  {
     value.setColumnStretchFactor(self);
    // return 1;
  }
}

pub trait QGraphicsGridLayout_setColumnStretchFactor {
  fn setColumnStretchFactor(self, rsthis: &mut QGraphicsGridLayout) ;
}

// proto:  void QGraphicsGridLayout::setColumnStretchFactor(int column, int stretch);
impl<'a> /*trait*/ QGraphicsGridLayout_setColumnStretchFactor for (i32, i32) {
  fn setColumnStretchFactor(self, rsthis: &mut QGraphicsGridLayout)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsGridLayout22setColumnStretchFactorEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {_ZN19QGraphicsGridLayout22setColumnStretchFactorEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsGridLayout {
  pub fn invalidate<T: QGraphicsGridLayout_invalidate>(&mut self, value: T)  {
     value.invalidate(self);
    // return 1;
  }
}

pub trait QGraphicsGridLayout_invalidate {
  fn invalidate(self, rsthis: &mut QGraphicsGridLayout) ;
}

// proto:  void QGraphicsGridLayout::invalidate();
impl<'a> /*trait*/ QGraphicsGridLayout_invalidate for () {
  fn invalidate(self, rsthis: &mut QGraphicsGridLayout)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsGridLayout10invalidateEv()};
     unsafe {_ZN19QGraphicsGridLayout10invalidateEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsGridLayout {
  pub fn columnPreferredWidth<T: QGraphicsGridLayout_columnPreferredWidth>(&mut self, value: T) -> f64 {
    return value.columnPreferredWidth(self);
    // return 1;
  }
}

pub trait QGraphicsGridLayout_columnPreferredWidth {
  fn columnPreferredWidth(self, rsthis: &mut QGraphicsGridLayout) -> f64;
}

// proto:  double QGraphicsGridLayout::columnPreferredWidth(int column);
impl<'a> /*trait*/ QGraphicsGridLayout_columnPreferredWidth for (i32) {
  fn columnPreferredWidth(self, rsthis: &mut QGraphicsGridLayout) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsGridLayout20columnPreferredWidthEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK19QGraphicsGridLayout20columnPreferredWidthEi(rsthis.qclsinst, arg0)};
    return ret as f64;
    // return 1;
  }
}

impl /*struct*/ QGraphicsGridLayout {
  pub fn setColumnPreferredWidth<T: QGraphicsGridLayout_setColumnPreferredWidth>(&mut self, value: T)  {
     value.setColumnPreferredWidth(self);
    // return 1;
  }
}

pub trait QGraphicsGridLayout_setColumnPreferredWidth {
  fn setColumnPreferredWidth(self, rsthis: &mut QGraphicsGridLayout) ;
}

// proto:  void QGraphicsGridLayout::setColumnPreferredWidth(int column, qreal width);
impl<'a> /*trait*/ QGraphicsGridLayout_setColumnPreferredWidth for (i32, f64) {
  fn setColumnPreferredWidth(self, rsthis: &mut QGraphicsGridLayout)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsGridLayout23setColumnPreferredWidthEid()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_double;
     unsafe {_ZN19QGraphicsGridLayout23setColumnPreferredWidthEid(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsGridLayout {
  pub fn columnSpacing<T: QGraphicsGridLayout_columnSpacing>(&mut self, value: T) -> f64 {
    return value.columnSpacing(self);
    // return 1;
  }
}

pub trait QGraphicsGridLayout_columnSpacing {
  fn columnSpacing(self, rsthis: &mut QGraphicsGridLayout) -> f64;
}

// proto:  double QGraphicsGridLayout::columnSpacing(int column);
impl<'a> /*trait*/ QGraphicsGridLayout_columnSpacing for (i32) {
  fn columnSpacing(self, rsthis: &mut QGraphicsGridLayout) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QGraphicsGridLayout13columnSpacingEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK19QGraphicsGridLayout13columnSpacingEi(rsthis.qclsinst, arg0)};
    return ret as f64;
    // return 1;
  }
}

// proto: void QGraphicsGridLayout::NewQGraphicsGridLayout(const QGraphicsGridLayout & );
impl<'a> /*trait*/ QGraphicsGridLayout_NewQGraphicsGridLayout for (&'a  QGraphicsGridLayout) {
  fn NewQGraphicsGridLayout(self) -> QGraphicsGridLayout {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsGridLayoutC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN19QGraphicsGridLayoutC1ERKS_(qthis, arg0)};
    let rsthis = QGraphicsGridLayout{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGraphicsGridLayout {
  pub fn setRowSpacing<T: QGraphicsGridLayout_setRowSpacing>(&mut self, value: T)  {
     value.setRowSpacing(self);
    // return 1;
  }
}

pub trait QGraphicsGridLayout_setRowSpacing {
  fn setRowSpacing(self, rsthis: &mut QGraphicsGridLayout) ;
}

// proto:  void QGraphicsGridLayout::setRowSpacing(int row, qreal spacing);
impl<'a> /*trait*/ QGraphicsGridLayout_setRowSpacing for (i32, f64) {
  fn setRowSpacing(self, rsthis: &mut QGraphicsGridLayout)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsGridLayout13setRowSpacingEid()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_double;
     unsafe {_ZN19QGraphicsGridLayout13setRowSpacingEid(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsGridLayout {
  pub fn removeAt<T: QGraphicsGridLayout_removeAt>(&mut self, value: T)  {
     value.removeAt(self);
    // return 1;
  }
}

pub trait QGraphicsGridLayout_removeAt {
  fn removeAt(self, rsthis: &mut QGraphicsGridLayout) ;
}

// proto:  void QGraphicsGridLayout::removeAt(int index);
impl<'a> /*trait*/ QGraphicsGridLayout_removeAt for (i32) {
  fn removeAt(self, rsthis: &mut QGraphicsGridLayout)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QGraphicsGridLayout8removeAtEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN19QGraphicsGridLayout8removeAtEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

