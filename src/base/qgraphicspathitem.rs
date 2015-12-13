// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qpainterpath::QPainterPath;
use super::qgraphicsitem::QGraphicsItem;
use super::qpointf::QPointF;
use super::qrectf::QRectF;
use super::qpainter::QPainter;
use super::qstyleoptiongraphicsitem::QStyleOptionGraphicsItem;
use super::qwidget::QWidget;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QGraphicsPathItem::setPath(const QPainterPath & path);
  fn _ZN17QGraphicsPathItem7setPathERK12QPainterPath(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGraphicsPathItem::NewQGraphicsPathItem(const QPainterPath & path, QGraphicsItem * parent);
  fn _ZN17QGraphicsPathItemC1ERK12QPainterPathP13QGraphicsItem(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) ;
  // proto:  bool QGraphicsPathItem::contains(const QPointF & point);
  fn _ZNK17QGraphicsPathItem8containsERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  QRectF QGraphicsPathItem::boundingRect();
  fn _ZNK17QGraphicsPathItem12boundingRectEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsPathItem::NewQGraphicsPathItem(const QGraphicsPathItem & );
  fn _ZN17QGraphicsPathItemC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QGraphicsPathItem::type_();
  fn _ZNK17QGraphicsPathItem4typeEv(qthis: *mut c_void) -> c_int;
  // proto:  QPainterPath QGraphicsPathItem::opaqueArea();
  fn _ZNK17QGraphicsPathItem10opaqueAreaEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QPainterPath QGraphicsPathItem::path();
  fn _ZNK17QGraphicsPathItem4pathEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QGraphicsPathItem::FreeQGraphicsPathItem();
  fn _ZN17QGraphicsPathItemD0Ev(qthis: *mut c_void) ;
  // proto:  QPainterPath QGraphicsPathItem::shape();
  fn _ZNK17QGraphicsPathItem5shapeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QGraphicsPathItem::isObscuredBy(const QGraphicsItem * item);
  fn _ZNK17QGraphicsPathItem12isObscuredByEPK13QGraphicsItem(qthis: *mut c_void, arg0: *mut c_void) -> int8_t;
  // proto:  void QGraphicsPathItem::NewQGraphicsPathItem(QGraphicsItem * parent);
  fn _ZN17QGraphicsPathItemC1EP13QGraphicsItem(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QGraphicsPathItem::paint(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
  fn _ZN17QGraphicsPathItem5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) ;
}

// body block begin
// class sizeof(QGraphicsPathItem)=1
pub struct QGraphicsPathItem {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QGraphicsPathItem {
  pub fn setPath<T: QGraphicsPathItem_setPath>(&mut self, value: T)  {
     value.setPath(self);
    // return 1;
  }
}

pub trait QGraphicsPathItem_setPath {
  fn setPath(self, rsthis: &mut QGraphicsPathItem) ;
}

// proto:  void QGraphicsPathItem::setPath(const QPainterPath & path);
impl<'a> /*trait*/ QGraphicsPathItem_setPath for (&'a  QPainterPath) {
  fn setPath(self, rsthis: &mut QGraphicsPathItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsPathItem7setPathERK12QPainterPath()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN17QGraphicsPathItem7setPathERK12QPainterPath(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsPathItem {
  pub fn NewQGraphicsPathItem<T: QGraphicsPathItem_NewQGraphicsPathItem>(value: T) -> QGraphicsPathItem {
    let rsthis = value.NewQGraphicsPathItem();
    return rsthis;
    // return 1;
  }
}

pub trait QGraphicsPathItem_NewQGraphicsPathItem {
  fn NewQGraphicsPathItem(self) -> QGraphicsPathItem;
}

// proto: void QGraphicsPathItem::NewQGraphicsPathItem(const QPainterPath & path, QGraphicsItem * parent);
impl<'a> /*trait*/ QGraphicsPathItem_NewQGraphicsPathItem for (&'a  QPainterPath, &'a mut QGraphicsItem) {
  fn NewQGraphicsPathItem(self) -> QGraphicsPathItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsPathItemC1ERK12QPainterPathP13QGraphicsItem()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    unsafe {_ZN17QGraphicsPathItemC1ERK12QPainterPathP13QGraphicsItem(qthis, arg0, arg1)};
    let rsthis = QGraphicsPathItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGraphicsPathItem {
  pub fn contains<T: QGraphicsPathItem_contains>(&mut self, value: T) -> i8 {
    return value.contains(self);
    // return 1;
  }
}

pub trait QGraphicsPathItem_contains {
  fn contains(self, rsthis: &mut QGraphicsPathItem) -> i8;
}

// proto:  bool QGraphicsPathItem::contains(const QPointF & point);
impl<'a> /*trait*/ QGraphicsPathItem_contains for (&'a  QPointF) {
  fn contains(self, rsthis: &mut QGraphicsPathItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsPathItem8containsERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK17QGraphicsPathItem8containsERK7QPointF(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

impl /*struct*/ QGraphicsPathItem {
  pub fn boundingRect<T: QGraphicsPathItem_boundingRect>(&mut self, value: T) -> QRectF {
    return value.boundingRect(self);
    // return 1;
  }
}

pub trait QGraphicsPathItem_boundingRect {
  fn boundingRect(self, rsthis: &mut QGraphicsPathItem) -> QRectF;
}

// proto:  QRectF QGraphicsPathItem::boundingRect();
impl<'a> /*trait*/ QGraphicsPathItem_boundingRect for () {
  fn boundingRect(self, rsthis: &mut QGraphicsPathItem) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsPathItem12boundingRectEv()};
    let mut ret = unsafe {_ZNK17QGraphicsPathItem12boundingRectEv(rsthis.qclsinst)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto: void QGraphicsPathItem::NewQGraphicsPathItem(const QGraphicsPathItem & );
impl<'a> /*trait*/ QGraphicsPathItem_NewQGraphicsPathItem for (&'a  QGraphicsPathItem) {
  fn NewQGraphicsPathItem(self) -> QGraphicsPathItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsPathItemC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN17QGraphicsPathItemC1ERKS_(qthis, arg0)};
    let rsthis = QGraphicsPathItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGraphicsPathItem {
  pub fn type_<T: QGraphicsPathItem_type_>(&mut self, value: T) -> i32 {
    return value.type_(self);
    // return 1;
  }
}

pub trait QGraphicsPathItem_type_ {
  fn type_(self, rsthis: &mut QGraphicsPathItem) -> i32;
}

// proto:  int QGraphicsPathItem::type_();
impl<'a> /*trait*/ QGraphicsPathItem_type_ for () {
  fn type_(self, rsthis: &mut QGraphicsPathItem) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsPathItem4typeEv()};
    let mut ret = unsafe {_ZNK17QGraphicsPathItem4typeEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QGraphicsPathItem {
  pub fn opaqueArea<T: QGraphicsPathItem_opaqueArea>(&mut self, value: T) -> QPainterPath {
    return value.opaqueArea(self);
    // return 1;
  }
}

pub trait QGraphicsPathItem_opaqueArea {
  fn opaqueArea(self, rsthis: &mut QGraphicsPathItem) -> QPainterPath;
}

// proto:  QPainterPath QGraphicsPathItem::opaqueArea();
impl<'a> /*trait*/ QGraphicsPathItem_opaqueArea for () {
  fn opaqueArea(self, rsthis: &mut QGraphicsPathItem) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsPathItem10opaqueAreaEv()};
    let mut ret = unsafe {_ZNK17QGraphicsPathItem10opaqueAreaEv(rsthis.qclsinst)};
    let mut ret1 = QPainterPath{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsPathItem {
  pub fn path<T: QGraphicsPathItem_path>(&mut self, value: T) -> QPainterPath {
    return value.path(self);
    // return 1;
  }
}

pub trait QGraphicsPathItem_path {
  fn path(self, rsthis: &mut QGraphicsPathItem) -> QPainterPath;
}

// proto:  QPainterPath QGraphicsPathItem::path();
impl<'a> /*trait*/ QGraphicsPathItem_path for () {
  fn path(self, rsthis: &mut QGraphicsPathItem) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsPathItem4pathEv()};
    let mut ret = unsafe {_ZNK17QGraphicsPathItem4pathEv(rsthis.qclsinst)};
    let mut ret1 = QPainterPath{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsPathItem {
  pub fn FreeQGraphicsPathItem<T: QGraphicsPathItem_FreeQGraphicsPathItem>(&mut self, value: T)  {
     value.FreeQGraphicsPathItem(self);
    // return 1;
  }
}

pub trait QGraphicsPathItem_FreeQGraphicsPathItem {
  fn FreeQGraphicsPathItem(self, rsthis: &mut QGraphicsPathItem) ;
}

// proto:  void QGraphicsPathItem::FreeQGraphicsPathItem();
impl<'a> /*trait*/ QGraphicsPathItem_FreeQGraphicsPathItem for () {
  fn FreeQGraphicsPathItem(self, rsthis: &mut QGraphicsPathItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsPathItemD0Ev()};
     unsafe {_ZN17QGraphicsPathItemD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QGraphicsPathItem {
  pub fn shape<T: QGraphicsPathItem_shape>(&mut self, value: T) -> QPainterPath {
    return value.shape(self);
    // return 1;
  }
}

pub trait QGraphicsPathItem_shape {
  fn shape(self, rsthis: &mut QGraphicsPathItem) -> QPainterPath;
}

// proto:  QPainterPath QGraphicsPathItem::shape();
impl<'a> /*trait*/ QGraphicsPathItem_shape for () {
  fn shape(self, rsthis: &mut QGraphicsPathItem) -> QPainterPath {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsPathItem5shapeEv()};
    let mut ret = unsafe {_ZNK17QGraphicsPathItem5shapeEv(rsthis.qclsinst)};
    let mut ret1 = QPainterPath{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

impl /*struct*/ QGraphicsPathItem {
  pub fn isObscuredBy<T: QGraphicsPathItem_isObscuredBy>(&mut self, value: T) -> i8 {
    return value.isObscuredBy(self);
    // return 1;
  }
}

pub trait QGraphicsPathItem_isObscuredBy {
  fn isObscuredBy(self, rsthis: &mut QGraphicsPathItem) -> i8;
}

// proto:  bool QGraphicsPathItem::isObscuredBy(const QGraphicsItem * item);
impl<'a> /*trait*/ QGraphicsPathItem_isObscuredBy for (&'a  QGraphicsItem) {
  fn isObscuredBy(self, rsthis: &mut QGraphicsPathItem) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QGraphicsPathItem12isObscuredByEPK13QGraphicsItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK17QGraphicsPathItem12isObscuredByEPK13QGraphicsItem(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

// proto: void QGraphicsPathItem::NewQGraphicsPathItem(QGraphicsItem * parent);
impl<'a> /*trait*/ QGraphicsPathItem_NewQGraphicsPathItem for (&'a mut QGraphicsItem) {
  fn NewQGraphicsPathItem(self) -> QGraphicsPathItem {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsPathItemC1EP13QGraphicsItem()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN17QGraphicsPathItemC1EP13QGraphicsItem(qthis, arg0)};
    let rsthis = QGraphicsPathItem{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QGraphicsPathItem {
  pub fn paint<T: QGraphicsPathItem_paint>(&mut self, value: T)  {
     value.paint(self);
    // return 1;
  }
}

pub trait QGraphicsPathItem_paint {
  fn paint(self, rsthis: &mut QGraphicsPathItem) ;
}

// proto:  void QGraphicsPathItem::paint(QPainter * painter, const QStyleOptionGraphicsItem * option, QWidget * widget);
impl<'a> /*trait*/ QGraphicsPathItem_paint for (&'a mut QPainter, &'a  QStyleOptionGraphicsItem, &'a mut QWidget) {
  fn paint(self, rsthis: &mut QGraphicsPathItem)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QGraphicsPathItem5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZN17QGraphicsPathItem5paintEP8QPainterPK24QStyleOptionGraphicsItemP7QWidget(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

