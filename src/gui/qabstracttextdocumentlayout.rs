// auto generated, do not modify.
// created: Mon Dec 21 22:54:38 2015
// src-file: /QtGui/qabstracttextdocumentlayout.h
// dst-file: /src/gui/qabstracttextdocumentlayout.rs
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
use super::qtextdocument::QTextDocument; // 773
use super::qtextformat::QTextFormat; // 773
use super::super::core::qsize::QSizeF; // 771
use super::qpainter::QPainter; // 773
use super::super::core::qrect::QRectF; // 771
use super::super::core::qobject::QObject; // 771
use super::super::core::qpoint::QPointF; // 771
use super::qpaintdevice::QPaintDevice; // 773
use super::qtextobject::QTextBlock; // 773
use super::super::core::qstring::QString; // 771
// use super::qabstracttextdocumentlayout::QTextObjectInterface; // 773
use super::qtextobject::QTextFrame; // 773
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  void QTextObjectInterface::~QTextObjectInterface();
  fn _ZN20QTextObjectInterfaceD0Ev(qthis: *mut c_void);
  // proto:  QSizeF QTextObjectInterface::intrinsicSize(QTextDocument * doc, int posInDocument, const QTextFormat & format);
  fn _ZN20QTextObjectInterface13intrinsicSizeEP13QTextDocumentiRK11QTextFormat(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: *mut c_void) -> *mut c_void;
  // proto:  void QTextObjectInterface::drawObject(QPainter * painter, const QRectF & rect, QTextDocument * doc, int posInDocument, const QTextFormat & format);
  fn _ZN20QTextObjectInterface10drawObjectEP8QPainterRK6QRectFP13QTextDocumentiRK11QTextFormat(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void, arg3: c_int, arg4: *mut c_void);
  // proto:  const QMetaObject * QAbstractTextDocumentLayout::metaObject();
  fn _ZNK27QAbstractTextDocumentLayout10metaObjectEv(qthis: *mut c_void);
  // proto:  void QAbstractTextDocumentLayout::registerHandler(int objectType, QObject * component);
  fn _ZN27QAbstractTextDocumentLayout15registerHandlerEiP7QObject(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void);
  // proto:  int QAbstractTextDocumentLayout::pageCount();
  fn _ZNK27QAbstractTextDocumentLayout9pageCountEv(qthis: *mut c_void) -> c_int;
  // proto:  void QAbstractTextDocumentLayout::~QAbstractTextDocumentLayout();
  fn _ZN27QAbstractTextDocumentLayoutD0Ev(qthis: *mut c_void);
  // proto:  void QAbstractTextDocumentLayout::setPaintDevice(QPaintDevice * device);
  fn _ZN27QAbstractTextDocumentLayout14setPaintDeviceEP12QPaintDevice(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QAbstractTextDocumentLayout::pageCountChanged(int newPages);
  fn _ZN27QAbstractTextDocumentLayout16pageCountChangedEi(qthis: *mut c_void, arg0: c_int);
  // proto:  QTextDocument * QAbstractTextDocumentLayout::document();
  fn _ZNK27QAbstractTextDocumentLayout8documentEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QAbstractTextDocumentLayout::update(const QRectF & );
  fn _ZN27QAbstractTextDocumentLayout6updateERK6QRectF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QAbstractTextDocumentLayout::updateBlock(const QTextBlock & block);
  fn _ZN27QAbstractTextDocumentLayout11updateBlockERK10QTextBlock(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QAbstractTextDocumentLayout::unregisterHandler(int objectType, QObject * component);
  fn _ZN27QAbstractTextDocumentLayout17unregisterHandlerEiP7QObject(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void);
  // proto:  void QAbstractTextDocumentLayout::QAbstractTextDocumentLayout(QTextDocument * doc);
  fn _ZN27QAbstractTextDocumentLayoutC1EP13QTextDocument(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QSizeF QAbstractTextDocumentLayout::documentSize();
  fn _ZNK27QAbstractTextDocumentLayout12documentSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QPaintDevice * QAbstractTextDocumentLayout::paintDevice();
  fn _ZNK27QAbstractTextDocumentLayout11paintDeviceEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QString QAbstractTextDocumentLayout::anchorAt(const QPointF & pos);
  fn _ZNK27QAbstractTextDocumentLayout8anchorAtERK7QPointF(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  QTextObjectInterface * QAbstractTextDocumentLayout::handlerForObject(int objectType);
  fn _ZNK27QAbstractTextDocumentLayout16handlerForObjectEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  QRectF QAbstractTextDocumentLayout::frameBoundingRect(QTextFrame * frame);
  fn _ZNK27QAbstractTextDocumentLayout17frameBoundingRectEP10QTextFrame(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QAbstractTextDocumentLayout::documentSizeChanged(const QSizeF & newSize);
  fn _ZN27QAbstractTextDocumentLayout19documentSizeChangedERK6QSizeF(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QRectF QAbstractTextDocumentLayout::blockBoundingRect(const QTextBlock & block);
  fn _ZNK27QAbstractTextDocumentLayout17blockBoundingRectERK10QTextBlock(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
} // <= ext block end

// body block begin =>
// class sizeof(QTextObjectInterface)=8
pub struct QTextObjectInterface {
  pub qclsinst: *mut c_void,
}

// class sizeof(QAbstractTextDocumentLayout)=1
pub struct QAbstractTextDocumentLayout {
  pub qclsinst: *mut c_void,
}

  // proto:  void QTextObjectInterface::~QTextObjectInterface();
impl /*struct*/ QTextObjectInterface {
  pub fn FreeQTextObjectInterface<RetType, T: QTextObjectInterface_FreeQTextObjectInterface<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQTextObjectInterface(self);
    // return 1;
  }
}

pub trait QTextObjectInterface_FreeQTextObjectInterface<RetType> {
  fn FreeQTextObjectInterface(self , rsthis: &mut QTextObjectInterface) -> RetType;
}

  // proto:  void QTextObjectInterface::~QTextObjectInterface();
impl<'a> /*trait*/ QTextObjectInterface_FreeQTextObjectInterface<()> for () {
  fn FreeQTextObjectInterface(self , rsthis: &mut QTextObjectInterface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QTextObjectInterfaceD0Ev()};
     unsafe {_ZN20QTextObjectInterfaceD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QSizeF QTextObjectInterface::intrinsicSize(QTextDocument * doc, int posInDocument, const QTextFormat & format);
impl /*struct*/ QTextObjectInterface {
  pub fn intrinsicSize<RetType, T: QTextObjectInterface_intrinsicSize<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.intrinsicSize(self);
    // return 1;
  }
}

pub trait QTextObjectInterface_intrinsicSize<RetType> {
  fn intrinsicSize(self , rsthis: &mut QTextObjectInterface) -> RetType;
}

  // proto:  QSizeF QTextObjectInterface::intrinsicSize(QTextDocument * doc, int posInDocument, const QTextFormat & format);
impl<'a> /*trait*/ QTextObjectInterface_intrinsicSize<QSizeF> for (QTextDocument, i32, QTextFormat) {
  fn intrinsicSize(self , rsthis: &mut QTextObjectInterface) -> QSizeF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QTextObjectInterface13intrinsicSizeEP13QTextDocumentiRK11QTextFormat()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN20QTextObjectInterface13intrinsicSizeEP13QTextDocumentiRK11QTextFormat(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QSizeF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextObjectInterface::drawObject(QPainter * painter, const QRectF & rect, QTextDocument * doc, int posInDocument, const QTextFormat & format);
impl /*struct*/ QTextObjectInterface {
  pub fn drawObject<RetType, T: QTextObjectInterface_drawObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.drawObject(self);
    // return 1;
  }
}

pub trait QTextObjectInterface_drawObject<RetType> {
  fn drawObject(self , rsthis: &mut QTextObjectInterface) -> RetType;
}

  // proto:  void QTextObjectInterface::drawObject(QPainter * painter, const QRectF & rect, QTextDocument * doc, int posInDocument, const QTextFormat & format);
impl<'a> /*trait*/ QTextObjectInterface_drawObject<()> for (QPainter, QRectF, QTextDocument, i32, QTextFormat) {
  fn drawObject(self , rsthis: &mut QTextObjectInterface) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QTextObjectInterface10drawObjectEP8QPainterRK6QRectFP13QTextDocumentiRK11QTextFormat()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3  as c_int;
    let arg4 = self.4.qclsinst  as *mut c_void;
     unsafe {_ZN20QTextObjectInterface10drawObjectEP8QPainterRK6QRectFP13QTextDocumentiRK11QTextFormat(rsthis.qclsinst, arg0, arg1, arg2, arg3, arg4)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QAbstractTextDocumentLayout::metaObject();
impl /*struct*/ QAbstractTextDocumentLayout {
  pub fn metaObject<RetType, T: QAbstractTextDocumentLayout_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QAbstractTextDocumentLayout_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QAbstractTextDocumentLayout) -> RetType;
}

  // proto:  const QMetaObject * QAbstractTextDocumentLayout::metaObject();
impl<'a> /*trait*/ QAbstractTextDocumentLayout_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QAbstractTextDocumentLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK27QAbstractTextDocumentLayout10metaObjectEv()};
     unsafe {_ZNK27QAbstractTextDocumentLayout10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QAbstractTextDocumentLayout::registerHandler(int objectType, QObject * component);
impl /*struct*/ QAbstractTextDocumentLayout {
  pub fn registerHandler<RetType, T: QAbstractTextDocumentLayout_registerHandler<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.registerHandler(self);
    // return 1;
  }
}

pub trait QAbstractTextDocumentLayout_registerHandler<RetType> {
  fn registerHandler(self , rsthis: &mut QAbstractTextDocumentLayout) -> RetType;
}

  // proto:  void QAbstractTextDocumentLayout::registerHandler(int objectType, QObject * component);
impl<'a> /*trait*/ QAbstractTextDocumentLayout_registerHandler<()> for (i32, QObject) {
  fn registerHandler(self , rsthis: &mut QAbstractTextDocumentLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QAbstractTextDocumentLayout15registerHandlerEiP7QObject()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN27QAbstractTextDocumentLayout15registerHandlerEiP7QObject(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  int QAbstractTextDocumentLayout::pageCount();
impl /*struct*/ QAbstractTextDocumentLayout {
  pub fn pageCount<RetType, T: QAbstractTextDocumentLayout_pageCount<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.pageCount(self);
    // return 1;
  }
}

pub trait QAbstractTextDocumentLayout_pageCount<RetType> {
  fn pageCount(self , rsthis: &mut QAbstractTextDocumentLayout) -> RetType;
}

  // proto:  int QAbstractTextDocumentLayout::pageCount();
impl<'a> /*trait*/ QAbstractTextDocumentLayout_pageCount<i32> for () {
  fn pageCount(self , rsthis: &mut QAbstractTextDocumentLayout) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK27QAbstractTextDocumentLayout9pageCountEv()};
    let mut ret = unsafe {_ZNK27QAbstractTextDocumentLayout9pageCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QAbstractTextDocumentLayout::~QAbstractTextDocumentLayout();
impl /*struct*/ QAbstractTextDocumentLayout {
  pub fn FreeQAbstractTextDocumentLayout<RetType, T: QAbstractTextDocumentLayout_FreeQAbstractTextDocumentLayout<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQAbstractTextDocumentLayout(self);
    // return 1;
  }
}

pub trait QAbstractTextDocumentLayout_FreeQAbstractTextDocumentLayout<RetType> {
  fn FreeQAbstractTextDocumentLayout(self , rsthis: &mut QAbstractTextDocumentLayout) -> RetType;
}

  // proto:  void QAbstractTextDocumentLayout::~QAbstractTextDocumentLayout();
impl<'a> /*trait*/ QAbstractTextDocumentLayout_FreeQAbstractTextDocumentLayout<()> for () {
  fn FreeQAbstractTextDocumentLayout(self , rsthis: &mut QAbstractTextDocumentLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QAbstractTextDocumentLayoutD0Ev()};
     unsafe {_ZN27QAbstractTextDocumentLayoutD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QAbstractTextDocumentLayout::setPaintDevice(QPaintDevice * device);
impl /*struct*/ QAbstractTextDocumentLayout {
  pub fn setPaintDevice<RetType, T: QAbstractTextDocumentLayout_setPaintDevice<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setPaintDevice(self);
    // return 1;
  }
}

pub trait QAbstractTextDocumentLayout_setPaintDevice<RetType> {
  fn setPaintDevice(self , rsthis: &mut QAbstractTextDocumentLayout) -> RetType;
}

  // proto:  void QAbstractTextDocumentLayout::setPaintDevice(QPaintDevice * device);
impl<'a> /*trait*/ QAbstractTextDocumentLayout_setPaintDevice<()> for (QPaintDevice) {
  fn setPaintDevice(self , rsthis: &mut QAbstractTextDocumentLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QAbstractTextDocumentLayout14setPaintDeviceEP12QPaintDevice()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN27QAbstractTextDocumentLayout14setPaintDeviceEP12QPaintDevice(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QAbstractTextDocumentLayout::pageCountChanged(int newPages);
impl /*struct*/ QAbstractTextDocumentLayout {
  pub fn pageCountChanged<RetType, T: QAbstractTextDocumentLayout_pageCountChanged<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.pageCountChanged(self);
    // return 1;
  }
}

pub trait QAbstractTextDocumentLayout_pageCountChanged<RetType> {
  fn pageCountChanged(self , rsthis: &mut QAbstractTextDocumentLayout) -> RetType;
}

  // proto:  void QAbstractTextDocumentLayout::pageCountChanged(int newPages);
impl<'a> /*trait*/ QAbstractTextDocumentLayout_pageCountChanged<()> for (i32) {
  fn pageCountChanged(self , rsthis: &mut QAbstractTextDocumentLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QAbstractTextDocumentLayout16pageCountChangedEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN27QAbstractTextDocumentLayout16pageCountChangedEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QTextDocument * QAbstractTextDocumentLayout::document();
impl /*struct*/ QAbstractTextDocumentLayout {
  pub fn document<RetType, T: QAbstractTextDocumentLayout_document<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.document(self);
    // return 1;
  }
}

pub trait QAbstractTextDocumentLayout_document<RetType> {
  fn document(self , rsthis: &mut QAbstractTextDocumentLayout) -> RetType;
}

  // proto:  QTextDocument * QAbstractTextDocumentLayout::document();
impl<'a> /*trait*/ QAbstractTextDocumentLayout_document<QTextDocument> for () {
  fn document(self , rsthis: &mut QAbstractTextDocumentLayout) -> QTextDocument {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK27QAbstractTextDocumentLayout8documentEv()};
    let mut ret = unsafe {_ZNK27QAbstractTextDocumentLayout8documentEv(rsthis.qclsinst)};
    let mut ret1 = QTextDocument{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QAbstractTextDocumentLayout::update(const QRectF & );
impl /*struct*/ QAbstractTextDocumentLayout {
  pub fn update<RetType, T: QAbstractTextDocumentLayout_update<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.update(self);
    // return 1;
  }
}

pub trait QAbstractTextDocumentLayout_update<RetType> {
  fn update(self , rsthis: &mut QAbstractTextDocumentLayout) -> RetType;
}

  // proto:  void QAbstractTextDocumentLayout::update(const QRectF & );
impl<'a> /*trait*/ QAbstractTextDocumentLayout_update<()> for (QRectF) {
  fn update(self , rsthis: &mut QAbstractTextDocumentLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QAbstractTextDocumentLayout6updateERK6QRectF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN27QAbstractTextDocumentLayout6updateERK6QRectF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QAbstractTextDocumentLayout::updateBlock(const QTextBlock & block);
impl /*struct*/ QAbstractTextDocumentLayout {
  pub fn updateBlock<RetType, T: QAbstractTextDocumentLayout_updateBlock<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.updateBlock(self);
    // return 1;
  }
}

pub trait QAbstractTextDocumentLayout_updateBlock<RetType> {
  fn updateBlock(self , rsthis: &mut QAbstractTextDocumentLayout) -> RetType;
}

  // proto:  void QAbstractTextDocumentLayout::updateBlock(const QTextBlock & block);
impl<'a> /*trait*/ QAbstractTextDocumentLayout_updateBlock<()> for (QTextBlock) {
  fn updateBlock(self , rsthis: &mut QAbstractTextDocumentLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QAbstractTextDocumentLayout11updateBlockERK10QTextBlock()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN27QAbstractTextDocumentLayout11updateBlockERK10QTextBlock(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QAbstractTextDocumentLayout::unregisterHandler(int objectType, QObject * component);
impl /*struct*/ QAbstractTextDocumentLayout {
  pub fn unregisterHandler<RetType, T: QAbstractTextDocumentLayout_unregisterHandler<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.unregisterHandler(self);
    // return 1;
  }
}

pub trait QAbstractTextDocumentLayout_unregisterHandler<RetType> {
  fn unregisterHandler(self , rsthis: &mut QAbstractTextDocumentLayout) -> RetType;
}

  // proto:  void QAbstractTextDocumentLayout::unregisterHandler(int objectType, QObject * component);
impl<'a> /*trait*/ QAbstractTextDocumentLayout_unregisterHandler<()> for (i32, QObject) {
  fn unregisterHandler(self , rsthis: &mut QAbstractTextDocumentLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QAbstractTextDocumentLayout17unregisterHandlerEiP7QObject()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZN27QAbstractTextDocumentLayout17unregisterHandlerEiP7QObject(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QAbstractTextDocumentLayout::QAbstractTextDocumentLayout(QTextDocument * doc);
impl /*struct*/ QAbstractTextDocumentLayout {
  pub fn NewQAbstractTextDocumentLayout<T: QAbstractTextDocumentLayout_NewQAbstractTextDocumentLayout>(value: T) -> QAbstractTextDocumentLayout {
    let rsthis = value.NewQAbstractTextDocumentLayout();
    return rsthis;
    // return 1;
  }
}

pub trait QAbstractTextDocumentLayout_NewQAbstractTextDocumentLayout {
  fn NewQAbstractTextDocumentLayout(self) -> QAbstractTextDocumentLayout;
}

  // proto:  void QAbstractTextDocumentLayout::QAbstractTextDocumentLayout(QTextDocument * doc);
impl<'a> /*trait*/ QAbstractTextDocumentLayout_NewQAbstractTextDocumentLayout for (QTextDocument) {
  fn NewQAbstractTextDocumentLayout(self) -> QAbstractTextDocumentLayout {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QAbstractTextDocumentLayoutC1EP13QTextDocument()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN27QAbstractTextDocumentLayoutC1EP13QTextDocument(qthis, arg0)};
    let rsthis = QAbstractTextDocumentLayout{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QSizeF QAbstractTextDocumentLayout::documentSize();
impl /*struct*/ QAbstractTextDocumentLayout {
  pub fn documentSize<RetType, T: QAbstractTextDocumentLayout_documentSize<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.documentSize(self);
    // return 1;
  }
}

pub trait QAbstractTextDocumentLayout_documentSize<RetType> {
  fn documentSize(self , rsthis: &mut QAbstractTextDocumentLayout) -> RetType;
}

  // proto:  QSizeF QAbstractTextDocumentLayout::documentSize();
impl<'a> /*trait*/ QAbstractTextDocumentLayout_documentSize<QSizeF> for () {
  fn documentSize(self , rsthis: &mut QAbstractTextDocumentLayout) -> QSizeF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK27QAbstractTextDocumentLayout12documentSizeEv()};
    let mut ret = unsafe {_ZNK27QAbstractTextDocumentLayout12documentSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSizeF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QPaintDevice * QAbstractTextDocumentLayout::paintDevice();
impl /*struct*/ QAbstractTextDocumentLayout {
  pub fn paintDevice<RetType, T: QAbstractTextDocumentLayout_paintDevice<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.paintDevice(self);
    // return 1;
  }
}

pub trait QAbstractTextDocumentLayout_paintDevice<RetType> {
  fn paintDevice(self , rsthis: &mut QAbstractTextDocumentLayout) -> RetType;
}

  // proto:  QPaintDevice * QAbstractTextDocumentLayout::paintDevice();
impl<'a> /*trait*/ QAbstractTextDocumentLayout_paintDevice<QPaintDevice> for () {
  fn paintDevice(self , rsthis: &mut QAbstractTextDocumentLayout) -> QPaintDevice {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK27QAbstractTextDocumentLayout11paintDeviceEv()};
    let mut ret = unsafe {_ZNK27QAbstractTextDocumentLayout11paintDeviceEv(rsthis.qclsinst)};
    let mut ret1 = QPaintDevice{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QString QAbstractTextDocumentLayout::anchorAt(const QPointF & pos);
impl /*struct*/ QAbstractTextDocumentLayout {
  pub fn anchorAt<RetType, T: QAbstractTextDocumentLayout_anchorAt<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.anchorAt(self);
    // return 1;
  }
}

pub trait QAbstractTextDocumentLayout_anchorAt<RetType> {
  fn anchorAt(self , rsthis: &mut QAbstractTextDocumentLayout) -> RetType;
}

  // proto:  QString QAbstractTextDocumentLayout::anchorAt(const QPointF & pos);
impl<'a> /*trait*/ QAbstractTextDocumentLayout_anchorAt<QString> for (QPointF) {
  fn anchorAt(self , rsthis: &mut QAbstractTextDocumentLayout) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK27QAbstractTextDocumentLayout8anchorAtERK7QPointF()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK27QAbstractTextDocumentLayout8anchorAtERK7QPointF(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QTextObjectInterface * QAbstractTextDocumentLayout::handlerForObject(int objectType);
impl /*struct*/ QAbstractTextDocumentLayout {
  pub fn handlerForObject<RetType, T: QAbstractTextDocumentLayout_handlerForObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.handlerForObject(self);
    // return 1;
  }
}

pub trait QAbstractTextDocumentLayout_handlerForObject<RetType> {
  fn handlerForObject(self , rsthis: &mut QAbstractTextDocumentLayout) -> RetType;
}

  // proto:  QTextObjectInterface * QAbstractTextDocumentLayout::handlerForObject(int objectType);
impl<'a> /*trait*/ QAbstractTextDocumentLayout_handlerForObject<QTextObjectInterface> for (i32) {
  fn handlerForObject(self , rsthis: &mut QAbstractTextDocumentLayout) -> QTextObjectInterface {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK27QAbstractTextDocumentLayout16handlerForObjectEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK27QAbstractTextDocumentLayout16handlerForObjectEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QTextObjectInterface{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QRectF QAbstractTextDocumentLayout::frameBoundingRect(QTextFrame * frame);
impl /*struct*/ QAbstractTextDocumentLayout {
  pub fn frameBoundingRect<RetType, T: QAbstractTextDocumentLayout_frameBoundingRect<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.frameBoundingRect(self);
    // return 1;
  }
}

pub trait QAbstractTextDocumentLayout_frameBoundingRect<RetType> {
  fn frameBoundingRect(self , rsthis: &mut QAbstractTextDocumentLayout) -> RetType;
}

  // proto:  QRectF QAbstractTextDocumentLayout::frameBoundingRect(QTextFrame * frame);
impl<'a> /*trait*/ QAbstractTextDocumentLayout_frameBoundingRect<QRectF> for (QTextFrame) {
  fn frameBoundingRect(self , rsthis: &mut QAbstractTextDocumentLayout) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK27QAbstractTextDocumentLayout17frameBoundingRectEP10QTextFrame()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK27QAbstractTextDocumentLayout17frameBoundingRectEP10QTextFrame(rsthis.qclsinst, arg0)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QAbstractTextDocumentLayout::documentSizeChanged(const QSizeF & newSize);
impl /*struct*/ QAbstractTextDocumentLayout {
  pub fn documentSizeChanged<RetType, T: QAbstractTextDocumentLayout_documentSizeChanged<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.documentSizeChanged(self);
    // return 1;
  }
}

pub trait QAbstractTextDocumentLayout_documentSizeChanged<RetType> {
  fn documentSizeChanged(self , rsthis: &mut QAbstractTextDocumentLayout) -> RetType;
}

  // proto:  void QAbstractTextDocumentLayout::documentSizeChanged(const QSizeF & newSize);
impl<'a> /*trait*/ QAbstractTextDocumentLayout_documentSizeChanged<()> for (QSizeF) {
  fn documentSizeChanged(self , rsthis: &mut QAbstractTextDocumentLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN27QAbstractTextDocumentLayout19documentSizeChangedERK6QSizeF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN27QAbstractTextDocumentLayout19documentSizeChangedERK6QSizeF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QRectF QAbstractTextDocumentLayout::blockBoundingRect(const QTextBlock & block);
impl /*struct*/ QAbstractTextDocumentLayout {
  pub fn blockBoundingRect<RetType, T: QAbstractTextDocumentLayout_blockBoundingRect<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.blockBoundingRect(self);
    // return 1;
  }
}

pub trait QAbstractTextDocumentLayout_blockBoundingRect<RetType> {
  fn blockBoundingRect(self , rsthis: &mut QAbstractTextDocumentLayout) -> RetType;
}

  // proto:  QRectF QAbstractTextDocumentLayout::blockBoundingRect(const QTextBlock & block);
impl<'a> /*trait*/ QAbstractTextDocumentLayout_blockBoundingRect<QRectF> for (QTextBlock) {
  fn blockBoundingRect(self , rsthis: &mut QAbstractTextDocumentLayout) -> QRectF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK27QAbstractTextDocumentLayout17blockBoundingRectERK10QTextBlock()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK27QAbstractTextDocumentLayout17blockBoundingRectERK10QTextBlock(rsthis.qclsinst, arg0)};
    let mut ret1 = QRectF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// <= body block end

