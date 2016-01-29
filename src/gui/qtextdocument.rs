// auto generated, do not modify.
// created: Thu Jan 28 22:38:45 2016
// src-file: /QtGui/qtextdocument.h
// dst-file: /src/gui/qtextdocument.rs
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
use super::super::core::qobject::*; // 771
use std::ops::Deref;
use super::super::core::qstring::*; // 771
use super::qabstracttextdocumentlayout::*; // 773
use super::super::core::qsize::*; // 771
use super::qtextcursor::*; // 773
use super::qpagedpaintdevice::*; // 773
// use super::qtextdocumentprivate::*; // 775
use super::super::core::qbytearray::*; // 771
use super::super::core::qurl::*; // 771
use super::super::core::qvariant::*; // 771
use super::qtextobject::*; // 773
// use super::qvector::*; // 775
use super::super::core::qregexp::*; // 771
use super::super::core::qregularexpression::*; // 771
use super::qtextoption::*; // 773
use super::qfont::*; // 773
// use super::qtextdocument::QAbstractUndoItem; // 773
use super::qpainter::*; // 773
use super::super::core::qrect::*; // 771
use super::super::core::qobjectdefs::*; // 771
use super::super::core::qchar::*; // 771
use super::qtextformat::*; // 773
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QTextDocument_Class_Size() -> c_int;
  // proto:  void QTextDocument::setDefaultStyleSheet(const QString & sheet);
  fn C_ZN13QTextDocument20setDefaultStyleSheetERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QAbstractTextDocumentLayout * QTextDocument::documentLayout();
  fn C_ZNK13QTextDocument14documentLayoutEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  bool QTextDocument::isModified();
  fn C_ZNK13QTextDocument10isModifiedEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  int QTextDocument::revision();
  fn C_ZNK13QTextDocument8revisionEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  QSizeF QTextDocument::pageSize();
  fn C_ZNK13QTextDocument8pageSizeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QTextDocument::redo(QTextCursor * cursor);
  fn C_ZN13QTextDocument4redoEP11QTextCursor(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  int QTextDocument::lineCount();
  fn C_ZNK13QTextDocument9lineCountEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QTextDocument::print(QPagedPaintDevice * printer);
  fn C_ZNK13QTextDocument5printEP17QPagedPaintDevice(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QTextDocumentPrivate * QTextDocument::docHandle();
  fn C_ZNK13QTextDocument9docHandleEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QString QTextDocument::toHtml(const QByteArray & encoding);
  fn C_ZNK13QTextDocument6toHtmlERK10QByteArray(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  int QTextDocument::availableUndoSteps();
  fn C_ZNK13QTextDocument18availableUndoStepsEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QTextDocument::setUndoRedoEnabled(bool enable);
  fn C_ZN13QTextDocument18setUndoRedoEnabledEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QTextDocument::undo(QTextCursor * cursor);
  fn C_ZN13QTextDocument4undoEP11QTextCursor(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QString QTextDocument::toPlainText();
  fn C_ZNK13QTextDocument11toPlainTextEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QTextDocument::addResource(int type, const QUrl & name, const QVariant & resource);
  fn C_ZN13QTextDocument11addResourceEiRK4QUrlRK8QVariant(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  QSizeF QTextDocument::size();
  fn C_ZNK13QTextDocument4sizeEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QTextObject * QTextDocument::object(int objectIndex);
  fn C_ZNK13QTextDocument6objectEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  QTextDocument * QTextDocument::clone(QObject * parent);
  fn C_ZNK13QTextDocument5cloneEP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QTextDocument::markContentsDirty(int from, int length);
  fn C_ZN13QTextDocument17markContentsDirtyEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int);
  // proto:  void QTextDocument::QTextDocument(QObject * parent);
  fn C_ZN13QTextDocumentC2EP7QObject(arg0: *mut c_void) -> u64;
  // proto:  int QTextDocument::characterCount();
  fn C_ZNK13QTextDocument14characterCountEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  QTextFrame * QTextDocument::rootFrame();
  fn C_ZNK13QTextDocument9rootFrameEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QTextBlock QTextDocument::firstBlock();
  fn C_ZNK13QTextDocument10firstBlockEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QTextDocument::blockCount();
  fn C_ZNK13QTextDocument10blockCountEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  qreal QTextDocument::idealWidth();
  fn C_ZNK13QTextDocument10idealWidthEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  void QTextDocument::adjustSize();
  fn C_ZN13QTextDocument10adjustSizeEv(qthis: u64 /* *mut c_void*/);
  // proto:  bool QTextDocument::isRedoAvailable();
  fn C_ZNK13QTextDocument15isRedoAvailableEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QVector<QTextFormat> QTextDocument::allFormats();
  fn C_ZNK13QTextDocument10allFormatsEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QString QTextDocument::defaultStyleSheet();
  fn C_ZNK13QTextDocument17defaultStyleSheetEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QTextBlock QTextDocument::lastBlock();
  fn C_ZNK13QTextDocument9lastBlockEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QTextDocument::QTextDocument(const QString & text, QObject * parent);
  fn C_ZN13QTextDocumentC2ERK7QStringP7QObject(arg0: *mut c_void, arg1: *mut c_void) -> u64;
  // proto:  bool QTextDocument::useDesignMetrics();
  fn C_ZNK13QTextDocument16useDesignMetricsEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  int QTextDocument::pageCount();
  fn C_ZNK13QTextDocument9pageCountEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QTextDocument::setTextWidth(qreal width);
  fn C_ZN13QTextDocument12setTextWidthEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  void QTextDocument::setDocumentMargin(qreal margin);
  fn C_ZN13QTextDocument17setDocumentMarginEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  bool QTextDocument::isUndoAvailable();
  fn C_ZNK13QTextDocument15isUndoAvailableEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  qreal QTextDocument::indentWidth();
  fn C_ZNK13QTextDocument11indentWidthEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  void QTextDocument::setUseDesignMetrics(bool b);
  fn C_ZN13QTextDocument19setUseDesignMetricsEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  // proto:  void QTextDocument::setDocumentLayout(QAbstractTextDocumentLayout * layout);
  fn C_ZN13QTextDocument17setDocumentLayoutEP27QAbstractTextDocumentLayout(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QTextDocument::setIndentWidth(qreal width);
  fn C_ZN13QTextDocument14setIndentWidthEd(qthis: u64 /* *mut c_void*/, arg0: c_double);
  // proto:  QUrl QTextDocument::baseUrl();
  fn C_ZNK13QTextDocument7baseUrlEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QTextFrame * QTextDocument::frameAt(int pos);
  fn C_ZNK13QTextDocument7frameAtEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  void QTextDocument::setDefaultTextOption(const QTextOption & option);
  fn C_ZN13QTextDocument20setDefaultTextOptionERK11QTextOption(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QFont QTextDocument::defaultFont();
  fn C_ZNK13QTextDocument11defaultFontEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QTextBlock QTextDocument::findBlockByNumber(int blockNumber);
  fn C_ZNK13QTextDocument17findBlockByNumberEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  QTextOption QTextDocument::defaultTextOption();
  fn C_ZNK13QTextDocument17defaultTextOptionEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QTextBlock QTextDocument::findBlock(int pos);
  fn C_ZNK13QTextDocument9findBlockEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  void QTextDocument::setBaseUrl(const QUrl & url);
  fn C_ZN13QTextDocument10setBaseUrlERK4QUrl(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QTextDocument::appendUndoItem(QAbstractUndoItem * );
  fn C_ZN13QTextDocument14appendUndoItemEP17QAbstractUndoItem(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QTextDocument::redo();
  fn C_ZN13QTextDocument4redoEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QTextDocument::drawContents(QPainter * painter, const QRectF & rect);
  fn C_ZN13QTextDocument12drawContentsEP8QPainterRK6QRectF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  QTextBlock QTextDocument::findBlockByLineNumber(int blockNumber);
  fn C_ZNK13QTextDocument21findBlockByLineNumberEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  void QTextDocument::undo();
  fn C_ZN13QTextDocument4undoEv(qthis: u64 /* *mut c_void*/);
  // proto:  qreal QTextDocument::textWidth();
  fn C_ZNK13QTextDocument9textWidthEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  const QMetaObject * QTextDocument::metaObject();
  fn C_ZNK13QTextDocument10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QTextDocument::availableRedoSteps();
  fn C_ZNK13QTextDocument18availableRedoStepsEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  QChar QTextDocument::characterAt(int pos);
  fn C_ZNK13QTextDocument11characterAtEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  void QTextDocument::setDefaultFont(const QFont & font);
  fn C_ZN13QTextDocument14setDefaultFontERK5QFont(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QTextObject * QTextDocument::objectForFormat(const QTextFormat & );
  fn C_ZNK13QTextDocument15objectForFormatERK11QTextFormat(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QTextDocument::isEmpty();
  fn C_ZNK13QTextDocument7isEmptyEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  bool QTextDocument::isUndoRedoEnabled();
  fn C_ZNK13QTextDocument17isUndoRedoEnabledEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  void QTextDocument::~QTextDocument();
  fn C_ZN13QTextDocumentD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  qreal QTextDocument::documentMargin();
  fn C_ZNK13QTextDocument14documentMarginEv(qthis: u64 /* *mut c_void*/) -> c_double;
  // proto:  void QTextDocument::setPageSize(const QSizeF & size);
  fn C_ZN13QTextDocument11setPageSizeERK6QSizeF(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QTextDocument::setHtml(const QString & html);
  fn C_ZN13QTextDocument7setHtmlERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QTextBlock QTextDocument::end();
  fn C_ZNK13QTextDocument3endEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QTextDocument::maximumBlockCount();
  fn C_ZNK13QTextDocument17maximumBlockCountEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QTextDocument::setPlainText(const QString & text);
  fn C_ZN13QTextDocument12setPlainTextERK7QString(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QTextDocument::clear();
  fn C_ZN13QTextDocument5clearEv(qthis: u64 /* *mut c_void*/);
  // proto:  QVariant QTextDocument::resource(int type, const QUrl & name);
  fn C_ZNK13QTextDocument8resourceEiRK4QUrl(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: *mut c_void) -> *mut c_void;
  // proto:  QTextBlock QTextDocument::begin();
  fn C_ZNK13QTextDocument5beginEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QTextDocument::setMaximumBlockCount(int maximum);
  fn C_ZN13QTextDocument20setMaximumBlockCountEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QTextDocument::setModified(bool m);
  fn C_ZN13QTextDocument11setModifiedEb(qthis: u64 /* *mut c_void*/, arg0: c_char);
  fn QAbstractUndoItem_Class_Size() -> c_int;
  // proto:  void QAbstractUndoItem::undo();
  fn C_ZN17QAbstractUndoItem4undoEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QAbstractUndoItem::redo();
  fn C_ZN17QAbstractUndoItem4redoEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QAbstractUndoItem::~QAbstractUndoItem();
  fn C_ZN17QAbstractUndoItemD2Ev(qthis: u64 /* *mut c_void*/);
  fn QTextDocument_SlotProxy_connect__ZN13QTextDocument21cursorPositionChangedERK11QTextCursor(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QTextDocument_SlotProxy_connect__ZN13QTextDocument19modificationChangedEb(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QTextDocument_SlotProxy_connect__ZN13QTextDocument13undoAvailableEb(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QTextDocument_SlotProxy_connect__ZN13QTextDocument15contentsChangedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QTextDocument_SlotProxy_connect__ZN13QTextDocument14baseUrlChangedERK4QUrl(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QTextDocument_SlotProxy_connect__ZN13QTextDocument17blockCountChangedEi(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QTextDocument_SlotProxy_connect__ZN13QTextDocument21documentLayoutChangedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QTextDocument_SlotProxy_connect__ZN13QTextDocument16undoCommandAddedEv(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QTextDocument_SlotProxy_connect__ZN13QTextDocument13redoAvailableEb(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QTextDocument_SlotProxy_connect__ZN13QTextDocument14contentsChangeEiii(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QTextDocument)=1
#[derive(Default)]
pub struct QTextDocument {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _cursorPositionChanged: QTextDocument_cursorPositionChanged_signal,
  pub _documentLayoutChanged: QTextDocument_documentLayoutChanged_signal,
  pub _undoCommandAdded: QTextDocument_undoCommandAdded_signal,
  pub _modificationChanged: QTextDocument_modificationChanged_signal,
  pub _redoAvailable: QTextDocument_redoAvailable_signal,
  pub _contentsChanged: QTextDocument_contentsChanged_signal,
  pub _baseUrlChanged: QTextDocument_baseUrlChanged_signal,
  pub _blockCountChanged: QTextDocument_blockCountChanged_signal,
  pub _undoAvailable: QTextDocument_undoAvailable_signal,
  pub _contentsChange: QTextDocument_contentsChange_signal,
}

// class sizeof(QAbstractUndoItem)=8
#[derive(Default)]
pub struct QAbstractUndoItem {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QTextDocument {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QTextDocument {
    return QTextDocument{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QTextDocument {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QTextDocument {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  void QTextDocument::setDefaultStyleSheet(const QString & sheet);
impl /*struct*/ QTextDocument {
  pub fn setDefaultStyleSheet<RetType, T: QTextDocument_setDefaultStyleSheet<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDefaultStyleSheet(self);
    // return 1;
  }
}

pub trait QTextDocument_setDefaultStyleSheet<RetType> {
  fn setDefaultStyleSheet(self , rsthis: & QTextDocument) -> RetType;
}

  // proto:  void QTextDocument::setDefaultStyleSheet(const QString & sheet);
impl<'a> /*trait*/ QTextDocument_setDefaultStyleSheet<()> for (&'a QString) {
  fn setDefaultStyleSheet(self , rsthis: & QTextDocument) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument20setDefaultStyleSheetERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN13QTextDocument20setDefaultStyleSheetERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QAbstractTextDocumentLayout * QTextDocument::documentLayout();
impl /*struct*/ QTextDocument {
  pub fn documentLayout<RetType, T: QTextDocument_documentLayout<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.documentLayout(self);
    // return 1;
  }
}

pub trait QTextDocument_documentLayout<RetType> {
  fn documentLayout(self , rsthis: & QTextDocument) -> RetType;
}

  // proto:  QAbstractTextDocumentLayout * QTextDocument::documentLayout();
impl<'a> /*trait*/ QTextDocument_documentLayout<QAbstractTextDocumentLayout> for () {
  fn documentLayout(self , rsthis: & QTextDocument) -> QAbstractTextDocumentLayout {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument14documentLayoutEv()};
    let mut ret = unsafe {C_ZNK13QTextDocument14documentLayoutEv(rsthis.qclsinst)};
    let mut ret1 = QAbstractTextDocumentLayout::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QTextDocument::isModified();
impl /*struct*/ QTextDocument {
  pub fn isModified<RetType, T: QTextDocument_isModified<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isModified(self);
    // return 1;
  }
}

pub trait QTextDocument_isModified<RetType> {
  fn isModified(self , rsthis: & QTextDocument) -> RetType;
}

  // proto:  bool QTextDocument::isModified();
impl<'a> /*trait*/ QTextDocument_isModified<i8> for () {
  fn isModified(self , rsthis: & QTextDocument) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument10isModifiedEv()};
    let mut ret = unsafe {C_ZNK13QTextDocument10isModifiedEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  int QTextDocument::revision();
impl /*struct*/ QTextDocument {
  pub fn revision<RetType, T: QTextDocument_revision<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.revision(self);
    // return 1;
  }
}

pub trait QTextDocument_revision<RetType> {
  fn revision(self , rsthis: & QTextDocument) -> RetType;
}

  // proto:  int QTextDocument::revision();
impl<'a> /*trait*/ QTextDocument_revision<i32> for () {
  fn revision(self , rsthis: & QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument8revisionEv()};
    let mut ret = unsafe {C_ZNK13QTextDocument8revisionEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  QSizeF QTextDocument::pageSize();
impl /*struct*/ QTextDocument {
  pub fn pageSize<RetType, T: QTextDocument_pageSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.pageSize(self);
    // return 1;
  }
}

pub trait QTextDocument_pageSize<RetType> {
  fn pageSize(self , rsthis: & QTextDocument) -> RetType;
}

  // proto:  QSizeF QTextDocument::pageSize();
impl<'a> /*trait*/ QTextDocument_pageSize<QSizeF> for () {
  fn pageSize(self , rsthis: & QTextDocument) -> QSizeF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument8pageSizeEv()};
    let mut ret = unsafe {C_ZNK13QTextDocument8pageSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSizeF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextDocument::redo(QTextCursor * cursor);
impl /*struct*/ QTextDocument {
  pub fn redo<RetType, T: QTextDocument_redo<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.redo(self);
    // return 1;
  }
}

pub trait QTextDocument_redo<RetType> {
  fn redo(self , rsthis: & QTextDocument) -> RetType;
}

  // proto:  void QTextDocument::redo(QTextCursor * cursor);
impl<'a> /*trait*/ QTextDocument_redo<()> for (&'a QTextCursor) {
  fn redo(self , rsthis: & QTextDocument) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument4redoEP11QTextCursor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN13QTextDocument4redoEP11QTextCursor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QTextDocument::lineCount();
impl /*struct*/ QTextDocument {
  pub fn lineCount<RetType, T: QTextDocument_lineCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.lineCount(self);
    // return 1;
  }
}

pub trait QTextDocument_lineCount<RetType> {
  fn lineCount(self , rsthis: & QTextDocument) -> RetType;
}

  // proto:  int QTextDocument::lineCount();
impl<'a> /*trait*/ QTextDocument_lineCount<i32> for () {
  fn lineCount(self , rsthis: & QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument9lineCountEv()};
    let mut ret = unsafe {C_ZNK13QTextDocument9lineCountEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  void QTextDocument::print(QPagedPaintDevice * printer);
impl /*struct*/ QTextDocument {
  pub fn print<RetType, T: QTextDocument_print<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.print(self);
    // return 1;
  }
}

pub trait QTextDocument_print<RetType> {
  fn print(self , rsthis: & QTextDocument) -> RetType;
}

  // proto:  void QTextDocument::print(QPagedPaintDevice * printer);
impl<'a> /*trait*/ QTextDocument_print<()> for (&'a QPagedPaintDevice) {
  fn print(self , rsthis: & QTextDocument) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument5printEP17QPagedPaintDevice()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZNK13QTextDocument5printEP17QPagedPaintDevice(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QTextDocumentPrivate * QTextDocument::docHandle();
impl /*struct*/ QTextDocument {
  pub fn docHandle<RetType, T: QTextDocument_docHandle<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.docHandle(self);
    // return 1;
  }
}

pub trait QTextDocument_docHandle<RetType> {
  fn docHandle(self , rsthis: & QTextDocument) -> RetType;
}

  // proto:  QTextDocumentPrivate * QTextDocument::docHandle();
impl<'a> /*trait*/ QTextDocument_docHandle<u64> for () {
  fn docHandle(self , rsthis: & QTextDocument) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument9docHandleEv()};
    let mut ret = unsafe {C_ZNK13QTextDocument9docHandleEv(rsthis.qclsinst)};
    return ret as u64; // 4
    // return 1;
  }
}

  // proto:  QString QTextDocument::toHtml(const QByteArray & encoding);
impl /*struct*/ QTextDocument {
  pub fn toHtml<RetType, T: QTextDocument_toHtml<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toHtml(self);
    // return 1;
  }
}

pub trait QTextDocument_toHtml<RetType> {
  fn toHtml(self , rsthis: & QTextDocument) -> RetType;
}

  // proto:  QString QTextDocument::toHtml(const QByteArray & encoding);
impl<'a> /*trait*/ QTextDocument_toHtml<QString> for (&'a QByteArray) {
  fn toHtml(self , rsthis: & QTextDocument) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument6toHtmlERK10QByteArray()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK13QTextDocument6toHtmlERK10QByteArray(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QTextDocument::availableUndoSteps();
impl /*struct*/ QTextDocument {
  pub fn availableUndoSteps<RetType, T: QTextDocument_availableUndoSteps<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.availableUndoSteps(self);
    // return 1;
  }
}

pub trait QTextDocument_availableUndoSteps<RetType> {
  fn availableUndoSteps(self , rsthis: & QTextDocument) -> RetType;
}

  // proto:  int QTextDocument::availableUndoSteps();
impl<'a> /*trait*/ QTextDocument_availableUndoSteps<i32> for () {
  fn availableUndoSteps(self , rsthis: & QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument18availableUndoStepsEv()};
    let mut ret = unsafe {C_ZNK13QTextDocument18availableUndoStepsEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  void QTextDocument::setUndoRedoEnabled(bool enable);
impl /*struct*/ QTextDocument {
  pub fn setUndoRedoEnabled<RetType, T: QTextDocument_setUndoRedoEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setUndoRedoEnabled(self);
    // return 1;
  }
}

pub trait QTextDocument_setUndoRedoEnabled<RetType> {
  fn setUndoRedoEnabled(self , rsthis: & QTextDocument) -> RetType;
}

  // proto:  void QTextDocument::setUndoRedoEnabled(bool enable);
impl<'a> /*trait*/ QTextDocument_setUndoRedoEnabled<()> for (i8) {
  fn setUndoRedoEnabled(self , rsthis: & QTextDocument) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument18setUndoRedoEnabledEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN13QTextDocument18setUndoRedoEnabledEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextDocument::undo(QTextCursor * cursor);
impl /*struct*/ QTextDocument {
  pub fn undo<RetType, T: QTextDocument_undo<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.undo(self);
    // return 1;
  }
}

pub trait QTextDocument_undo<RetType> {
  fn undo(self , rsthis: & QTextDocument) -> RetType;
}

  // proto:  void QTextDocument::undo(QTextCursor * cursor);
impl<'a> /*trait*/ QTextDocument_undo<()> for (&'a QTextCursor) {
  fn undo(self , rsthis: & QTextDocument) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument4undoEP11QTextCursor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN13QTextDocument4undoEP11QTextCursor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QTextDocument::toPlainText();
impl /*struct*/ QTextDocument {
  pub fn toPlainText<RetType, T: QTextDocument_toPlainText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.toPlainText(self);
    // return 1;
  }
}

pub trait QTextDocument_toPlainText<RetType> {
  fn toPlainText(self , rsthis: & QTextDocument) -> RetType;
}

  // proto:  QString QTextDocument::toPlainText();
impl<'a> /*trait*/ QTextDocument_toPlainText<QString> for () {
  fn toPlainText(self , rsthis: & QTextDocument) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument11toPlainTextEv()};
    let mut ret = unsafe {C_ZNK13QTextDocument11toPlainTextEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextDocument::addResource(int type, const QUrl & name, const QVariant & resource);
impl /*struct*/ QTextDocument {
  pub fn addResource<RetType, T: QTextDocument_addResource<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.addResource(self);
    // return 1;
  }
}

pub trait QTextDocument_addResource<RetType> {
  fn addResource(self , rsthis: & QTextDocument) -> RetType;
}

  // proto:  void QTextDocument::addResource(int type, const QUrl & name, const QVariant & resource);
impl<'a> /*trait*/ QTextDocument_addResource<()> for (i32, &'a QUrl, &'a QVariant) {
  fn addResource(self , rsthis: & QTextDocument) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument11addResourceEiRK4QUrlRK8QVariant()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {C_ZN13QTextDocument11addResourceEiRK4QUrlRK8QVariant(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  QSizeF QTextDocument::size();
impl /*struct*/ QTextDocument {
  pub fn size<RetType, T: QTextDocument_size<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.size(self);
    // return 1;
  }
}

pub trait QTextDocument_size<RetType> {
  fn size(self , rsthis: & QTextDocument) -> RetType;
}

  // proto:  QSizeF QTextDocument::size();
impl<'a> /*trait*/ QTextDocument_size<QSizeF> for () {
  fn size(self , rsthis: & QTextDocument) -> QSizeF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument4sizeEv()};
    let mut ret = unsafe {C_ZNK13QTextDocument4sizeEv(rsthis.qclsinst)};
    let mut ret1 = QSizeF::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QTextObject * QTextDocument::object(int objectIndex);
impl /*struct*/ QTextDocument {
  pub fn object<RetType, T: QTextDocument_object<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.object(self);
    // return 1;
  }
}

pub trait QTextDocument_object<RetType> {
  fn object(self , rsthis: & QTextDocument) -> RetType;
}

  // proto:  QTextObject * QTextDocument::object(int objectIndex);
impl<'a> /*trait*/ QTextDocument_object<QTextObject> for (i32) {
  fn object(self , rsthis: & QTextDocument) -> QTextObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument6objectEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZNK13QTextDocument6objectEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QTextObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QTextDocument * QTextDocument::clone(QObject * parent);
impl /*struct*/ QTextDocument {
  pub fn clone<RetType, T: QTextDocument_clone<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clone(self);
    // return 1;
  }
}

pub trait QTextDocument_clone<RetType> {
  fn clone(self , rsthis: & QTextDocument) -> RetType;
}

  // proto:  QTextDocument * QTextDocument::clone(QObject * parent);
impl<'a> /*trait*/ QTextDocument_clone<QTextDocument> for (&'a QObject) {
  fn clone(self , rsthis: & QTextDocument) -> QTextDocument {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument5cloneEP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK13QTextDocument5cloneEP7QObject(rsthis.qclsinst, arg0)};
    let mut ret1 = QTextDocument::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextDocument::markContentsDirty(int from, int length);
impl /*struct*/ QTextDocument {
  pub fn markContentsDirty<RetType, T: QTextDocument_markContentsDirty<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.markContentsDirty(self);
    // return 1;
  }
}

pub trait QTextDocument_markContentsDirty<RetType> {
  fn markContentsDirty(self , rsthis: & QTextDocument) -> RetType;
}

  // proto:  void QTextDocument::markContentsDirty(int from, int length);
impl<'a> /*trait*/ QTextDocument_markContentsDirty<()> for (i32, i32) {
  fn markContentsDirty(self , rsthis: & QTextDocument) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument17markContentsDirtyEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {C_ZN13QTextDocument17markContentsDirtyEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QTextDocument::QTextDocument(QObject * parent);
impl /*struct*/ QTextDocument {
  pub fn new<T: QTextDocument_new>(value: T) -> QTextDocument {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QTextDocument_new {
  fn new(self) -> QTextDocument;
}

  // proto:  void QTextDocument::QTextDocument(QObject * parent);
impl<'a> /*trait*/ QTextDocument_new for (&'a QObject) {
  fn new(self) -> QTextDocument {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocumentC2EP7QObject()};
    let ctysz: c_int = unsafe{QTextDocument_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN13QTextDocumentC2EP7QObject(arg0)};
    let rsthis = QTextDocument{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QTextDocument::characterCount();
impl /*struct*/ QTextDocument {
  pub fn characterCount<RetType, T: QTextDocument_characterCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.characterCount(self);
    // return 1;
  }
}

pub trait QTextDocument_characterCount<RetType> {
  fn characterCount(self , rsthis: & QTextDocument) -> RetType;
}

  // proto:  int QTextDocument::characterCount();
impl<'a> /*trait*/ QTextDocument_characterCount<i32> for () {
  fn characterCount(self , rsthis: & QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument14characterCountEv()};
    let mut ret = unsafe {C_ZNK13QTextDocument14characterCountEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  QTextFrame * QTextDocument::rootFrame();
impl /*struct*/ QTextDocument {
  pub fn rootFrame<RetType, T: QTextDocument_rootFrame<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rootFrame(self);
    // return 1;
  }
}

pub trait QTextDocument_rootFrame<RetType> {
  fn rootFrame(self , rsthis: & QTextDocument) -> RetType;
}

  // proto:  QTextFrame * QTextDocument::rootFrame();
impl<'a> /*trait*/ QTextDocument_rootFrame<QTextFrame> for () {
  fn rootFrame(self , rsthis: & QTextDocument) -> QTextFrame {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument9rootFrameEv()};
    let mut ret = unsafe {C_ZNK13QTextDocument9rootFrameEv(rsthis.qclsinst)};
    let mut ret1 = QTextFrame::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QTextBlock QTextDocument::firstBlock();
impl /*struct*/ QTextDocument {
  pub fn firstBlock<RetType, T: QTextDocument_firstBlock<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.firstBlock(self);
    // return 1;
  }
}

pub trait QTextDocument_firstBlock<RetType> {
  fn firstBlock(self , rsthis: & QTextDocument) -> RetType;
}

  // proto:  QTextBlock QTextDocument::firstBlock();
impl<'a> /*trait*/ QTextDocument_firstBlock<QTextBlock> for () {
  fn firstBlock(self , rsthis: & QTextDocument) -> QTextBlock {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument10firstBlockEv()};
    let mut ret = unsafe {C_ZNK13QTextDocument10firstBlockEv(rsthis.qclsinst)};
    let mut ret1 = QTextBlock::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QTextDocument::blockCount();
impl /*struct*/ QTextDocument {
  pub fn blockCount<RetType, T: QTextDocument_blockCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.blockCount(self);
    // return 1;
  }
}

pub trait QTextDocument_blockCount<RetType> {
  fn blockCount(self , rsthis: & QTextDocument) -> RetType;
}

  // proto:  int QTextDocument::blockCount();
impl<'a> /*trait*/ QTextDocument_blockCount<i32> for () {
  fn blockCount(self , rsthis: & QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument10blockCountEv()};
    let mut ret = unsafe {C_ZNK13QTextDocument10blockCountEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  qreal QTextDocument::idealWidth();
impl /*struct*/ QTextDocument {
  pub fn idealWidth<RetType, T: QTextDocument_idealWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.idealWidth(self);
    // return 1;
  }
}

pub trait QTextDocument_idealWidth<RetType> {
  fn idealWidth(self , rsthis: & QTextDocument) -> RetType;
}

  // proto:  qreal QTextDocument::idealWidth();
impl<'a> /*trait*/ QTextDocument_idealWidth<f64> for () {
  fn idealWidth(self , rsthis: & QTextDocument) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument10idealWidthEv()};
    let mut ret = unsafe {C_ZNK13QTextDocument10idealWidthEv(rsthis.qclsinst)};
    return ret as f64; // 1
    // return 1;
  }
}

  // proto:  void QTextDocument::adjustSize();
impl /*struct*/ QTextDocument {
  pub fn adjustSize<RetType, T: QTextDocument_adjustSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.adjustSize(self);
    // return 1;
  }
}

pub trait QTextDocument_adjustSize<RetType> {
  fn adjustSize(self , rsthis: & QTextDocument) -> RetType;
}

  // proto:  void QTextDocument::adjustSize();
impl<'a> /*trait*/ QTextDocument_adjustSize<()> for () {
  fn adjustSize(self , rsthis: & QTextDocument) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument10adjustSizeEv()};
     unsafe {C_ZN13QTextDocument10adjustSizeEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QTextDocument::isRedoAvailable();
impl /*struct*/ QTextDocument {
  pub fn isRedoAvailable<RetType, T: QTextDocument_isRedoAvailable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isRedoAvailable(self);
    // return 1;
  }
}

pub trait QTextDocument_isRedoAvailable<RetType> {
  fn isRedoAvailable(self , rsthis: & QTextDocument) -> RetType;
}

  // proto:  bool QTextDocument::isRedoAvailable();
impl<'a> /*trait*/ QTextDocument_isRedoAvailable<i8> for () {
  fn isRedoAvailable(self , rsthis: & QTextDocument) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument15isRedoAvailableEv()};
    let mut ret = unsafe {C_ZNK13QTextDocument15isRedoAvailableEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  QVector<QTextFormat> QTextDocument::allFormats();
impl /*struct*/ QTextDocument {
  pub fn allFormats<RetType, T: QTextDocument_allFormats<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.allFormats(self);
    // return 1;
  }
}

pub trait QTextDocument_allFormats<RetType> {
  fn allFormats(self , rsthis: & QTextDocument) -> RetType;
}

  // proto:  QVector<QTextFormat> QTextDocument::allFormats();
impl<'a> /*trait*/ QTextDocument_allFormats<u64> for () {
  fn allFormats(self , rsthis: & QTextDocument) -> u64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument10allFormatsEv()};
    let mut ret = unsafe {C_ZNK13QTextDocument10allFormatsEv(rsthis.qclsinst)};
    return ret as u64; // 5
    // return 1;
  }
}

  // proto:  QString QTextDocument::defaultStyleSheet();
impl /*struct*/ QTextDocument {
  pub fn defaultStyleSheet<RetType, T: QTextDocument_defaultStyleSheet<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.defaultStyleSheet(self);
    // return 1;
  }
}

pub trait QTextDocument_defaultStyleSheet<RetType> {
  fn defaultStyleSheet(self , rsthis: & QTextDocument) -> RetType;
}

  // proto:  QString QTextDocument::defaultStyleSheet();
impl<'a> /*trait*/ QTextDocument_defaultStyleSheet<QString> for () {
  fn defaultStyleSheet(self , rsthis: & QTextDocument) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument17defaultStyleSheetEv()};
    let mut ret = unsafe {C_ZNK13QTextDocument17defaultStyleSheetEv(rsthis.qclsinst)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QTextBlock QTextDocument::lastBlock();
impl /*struct*/ QTextDocument {
  pub fn lastBlock<RetType, T: QTextDocument_lastBlock<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.lastBlock(self);
    // return 1;
  }
}

pub trait QTextDocument_lastBlock<RetType> {
  fn lastBlock(self , rsthis: & QTextDocument) -> RetType;
}

  // proto:  QTextBlock QTextDocument::lastBlock();
impl<'a> /*trait*/ QTextDocument_lastBlock<QTextBlock> for () {
  fn lastBlock(self , rsthis: & QTextDocument) -> QTextBlock {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument9lastBlockEv()};
    let mut ret = unsafe {C_ZNK13QTextDocument9lastBlockEv(rsthis.qclsinst)};
    let mut ret1 = QTextBlock::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextDocument::QTextDocument(const QString & text, QObject * parent);
impl<'a> /*trait*/ QTextDocument_new for (&'a QString, &'a QObject) {
  fn new(self) -> QTextDocument {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocumentC2ERK7QStringP7QObject()};
    let ctysz: c_int = unsafe{QTextDocument_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN13QTextDocumentC2ERK7QStringP7QObject(arg0, arg1)};
    let rsthis = QTextDocument{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  bool QTextDocument::useDesignMetrics();
impl /*struct*/ QTextDocument {
  pub fn useDesignMetrics<RetType, T: QTextDocument_useDesignMetrics<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.useDesignMetrics(self);
    // return 1;
  }
}

pub trait QTextDocument_useDesignMetrics<RetType> {
  fn useDesignMetrics(self , rsthis: & QTextDocument) -> RetType;
}

  // proto:  bool QTextDocument::useDesignMetrics();
impl<'a> /*trait*/ QTextDocument_useDesignMetrics<i8> for () {
  fn useDesignMetrics(self , rsthis: & QTextDocument) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument16useDesignMetricsEv()};
    let mut ret = unsafe {C_ZNK13QTextDocument16useDesignMetricsEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  int QTextDocument::pageCount();
impl /*struct*/ QTextDocument {
  pub fn pageCount<RetType, T: QTextDocument_pageCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.pageCount(self);
    // return 1;
  }
}

pub trait QTextDocument_pageCount<RetType> {
  fn pageCount(self , rsthis: & QTextDocument) -> RetType;
}

  // proto:  int QTextDocument::pageCount();
impl<'a> /*trait*/ QTextDocument_pageCount<i32> for () {
  fn pageCount(self , rsthis: & QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument9pageCountEv()};
    let mut ret = unsafe {C_ZNK13QTextDocument9pageCountEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  void QTextDocument::setTextWidth(qreal width);
impl /*struct*/ QTextDocument {
  pub fn setTextWidth<RetType, T: QTextDocument_setTextWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setTextWidth(self);
    // return 1;
  }
}

pub trait QTextDocument_setTextWidth<RetType> {
  fn setTextWidth(self , rsthis: & QTextDocument) -> RetType;
}

  // proto:  void QTextDocument::setTextWidth(qreal width);
impl<'a> /*trait*/ QTextDocument_setTextWidth<()> for (f64) {
  fn setTextWidth(self , rsthis: & QTextDocument) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument12setTextWidthEd()};
    let arg0 = self  as c_double;
     unsafe {C_ZN13QTextDocument12setTextWidthEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextDocument::setDocumentMargin(qreal margin);
impl /*struct*/ QTextDocument {
  pub fn setDocumentMargin<RetType, T: QTextDocument_setDocumentMargin<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDocumentMargin(self);
    // return 1;
  }
}

pub trait QTextDocument_setDocumentMargin<RetType> {
  fn setDocumentMargin(self , rsthis: & QTextDocument) -> RetType;
}

  // proto:  void QTextDocument::setDocumentMargin(qreal margin);
impl<'a> /*trait*/ QTextDocument_setDocumentMargin<()> for (f64) {
  fn setDocumentMargin(self , rsthis: & QTextDocument) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument17setDocumentMarginEd()};
    let arg0 = self  as c_double;
     unsafe {C_ZN13QTextDocument17setDocumentMarginEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QTextDocument::isUndoAvailable();
impl /*struct*/ QTextDocument {
  pub fn isUndoAvailable<RetType, T: QTextDocument_isUndoAvailable<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isUndoAvailable(self);
    // return 1;
  }
}

pub trait QTextDocument_isUndoAvailable<RetType> {
  fn isUndoAvailable(self , rsthis: & QTextDocument) -> RetType;
}

  // proto:  bool QTextDocument::isUndoAvailable();
impl<'a> /*trait*/ QTextDocument_isUndoAvailable<i8> for () {
  fn isUndoAvailable(self , rsthis: & QTextDocument) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument15isUndoAvailableEv()};
    let mut ret = unsafe {C_ZNK13QTextDocument15isUndoAvailableEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  qreal QTextDocument::indentWidth();
impl /*struct*/ QTextDocument {
  pub fn indentWidth<RetType, T: QTextDocument_indentWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.indentWidth(self);
    // return 1;
  }
}

pub trait QTextDocument_indentWidth<RetType> {
  fn indentWidth(self , rsthis: & QTextDocument) -> RetType;
}

  // proto:  qreal QTextDocument::indentWidth();
impl<'a> /*trait*/ QTextDocument_indentWidth<f64> for () {
  fn indentWidth(self , rsthis: & QTextDocument) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument11indentWidthEv()};
    let mut ret = unsafe {C_ZNK13QTextDocument11indentWidthEv(rsthis.qclsinst)};
    return ret as f64; // 1
    // return 1;
  }
}

  // proto:  void QTextDocument::setUseDesignMetrics(bool b);
impl /*struct*/ QTextDocument {
  pub fn setUseDesignMetrics<RetType, T: QTextDocument_setUseDesignMetrics<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setUseDesignMetrics(self);
    // return 1;
  }
}

pub trait QTextDocument_setUseDesignMetrics<RetType> {
  fn setUseDesignMetrics(self , rsthis: & QTextDocument) -> RetType;
}

  // proto:  void QTextDocument::setUseDesignMetrics(bool b);
impl<'a> /*trait*/ QTextDocument_setUseDesignMetrics<()> for (i8) {
  fn setUseDesignMetrics(self , rsthis: & QTextDocument) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument19setUseDesignMetricsEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN13QTextDocument19setUseDesignMetricsEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextDocument::setDocumentLayout(QAbstractTextDocumentLayout * layout);
impl /*struct*/ QTextDocument {
  pub fn setDocumentLayout<RetType, T: QTextDocument_setDocumentLayout<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDocumentLayout(self);
    // return 1;
  }
}

pub trait QTextDocument_setDocumentLayout<RetType> {
  fn setDocumentLayout(self , rsthis: & QTextDocument) -> RetType;
}

  // proto:  void QTextDocument::setDocumentLayout(QAbstractTextDocumentLayout * layout);
impl<'a> /*trait*/ QTextDocument_setDocumentLayout<()> for (&'a QAbstractTextDocumentLayout) {
  fn setDocumentLayout(self , rsthis: & QTextDocument) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument17setDocumentLayoutEP27QAbstractTextDocumentLayout()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN13QTextDocument17setDocumentLayoutEP27QAbstractTextDocumentLayout(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextDocument::setIndentWidth(qreal width);
impl /*struct*/ QTextDocument {
  pub fn setIndentWidth<RetType, T: QTextDocument_setIndentWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setIndentWidth(self);
    // return 1;
  }
}

pub trait QTextDocument_setIndentWidth<RetType> {
  fn setIndentWidth(self , rsthis: & QTextDocument) -> RetType;
}

  // proto:  void QTextDocument::setIndentWidth(qreal width);
impl<'a> /*trait*/ QTextDocument_setIndentWidth<()> for (f64) {
  fn setIndentWidth(self , rsthis: & QTextDocument) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument14setIndentWidthEd()};
    let arg0 = self  as c_double;
     unsafe {C_ZN13QTextDocument14setIndentWidthEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QUrl QTextDocument::baseUrl();
impl /*struct*/ QTextDocument {
  pub fn baseUrl<RetType, T: QTextDocument_baseUrl<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.baseUrl(self);
    // return 1;
  }
}

pub trait QTextDocument_baseUrl<RetType> {
  fn baseUrl(self , rsthis: & QTextDocument) -> RetType;
}

  // proto:  QUrl QTextDocument::baseUrl();
impl<'a> /*trait*/ QTextDocument_baseUrl<QUrl> for () {
  fn baseUrl(self , rsthis: & QTextDocument) -> QUrl {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument7baseUrlEv()};
    let mut ret = unsafe {C_ZNK13QTextDocument7baseUrlEv(rsthis.qclsinst)};
    let mut ret1 = QUrl::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QTextFrame * QTextDocument::frameAt(int pos);
impl /*struct*/ QTextDocument {
  pub fn frameAt<RetType, T: QTextDocument_frameAt<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.frameAt(self);
    // return 1;
  }
}

pub trait QTextDocument_frameAt<RetType> {
  fn frameAt(self , rsthis: & QTextDocument) -> RetType;
}

  // proto:  QTextFrame * QTextDocument::frameAt(int pos);
impl<'a> /*trait*/ QTextDocument_frameAt<QTextFrame> for (i32) {
  fn frameAt(self , rsthis: & QTextDocument) -> QTextFrame {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument7frameAtEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZNK13QTextDocument7frameAtEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QTextFrame::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextDocument::setDefaultTextOption(const QTextOption & option);
impl /*struct*/ QTextDocument {
  pub fn setDefaultTextOption<RetType, T: QTextDocument_setDefaultTextOption<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDefaultTextOption(self);
    // return 1;
  }
}

pub trait QTextDocument_setDefaultTextOption<RetType> {
  fn setDefaultTextOption(self , rsthis: & QTextDocument) -> RetType;
}

  // proto:  void QTextDocument::setDefaultTextOption(const QTextOption & option);
impl<'a> /*trait*/ QTextDocument_setDefaultTextOption<()> for (&'a QTextOption) {
  fn setDefaultTextOption(self , rsthis: & QTextDocument) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument20setDefaultTextOptionERK11QTextOption()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN13QTextDocument20setDefaultTextOptionERK11QTextOption(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QFont QTextDocument::defaultFont();
impl /*struct*/ QTextDocument {
  pub fn defaultFont<RetType, T: QTextDocument_defaultFont<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.defaultFont(self);
    // return 1;
  }
}

pub trait QTextDocument_defaultFont<RetType> {
  fn defaultFont(self , rsthis: & QTextDocument) -> RetType;
}

  // proto:  QFont QTextDocument::defaultFont();
impl<'a> /*trait*/ QTextDocument_defaultFont<QFont> for () {
  fn defaultFont(self , rsthis: & QTextDocument) -> QFont {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument11defaultFontEv()};
    let mut ret = unsafe {C_ZNK13QTextDocument11defaultFontEv(rsthis.qclsinst)};
    let mut ret1 = QFont::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QTextBlock QTextDocument::findBlockByNumber(int blockNumber);
impl /*struct*/ QTextDocument {
  pub fn findBlockByNumber<RetType, T: QTextDocument_findBlockByNumber<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.findBlockByNumber(self);
    // return 1;
  }
}

pub trait QTextDocument_findBlockByNumber<RetType> {
  fn findBlockByNumber(self , rsthis: & QTextDocument) -> RetType;
}

  // proto:  QTextBlock QTextDocument::findBlockByNumber(int blockNumber);
impl<'a> /*trait*/ QTextDocument_findBlockByNumber<QTextBlock> for (i32) {
  fn findBlockByNumber(self , rsthis: & QTextDocument) -> QTextBlock {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument17findBlockByNumberEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZNK13QTextDocument17findBlockByNumberEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QTextBlock::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QTextOption QTextDocument::defaultTextOption();
impl /*struct*/ QTextDocument {
  pub fn defaultTextOption<RetType, T: QTextDocument_defaultTextOption<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.defaultTextOption(self);
    // return 1;
  }
}

pub trait QTextDocument_defaultTextOption<RetType> {
  fn defaultTextOption(self , rsthis: & QTextDocument) -> RetType;
}

  // proto:  QTextOption QTextDocument::defaultTextOption();
impl<'a> /*trait*/ QTextDocument_defaultTextOption<QTextOption> for () {
  fn defaultTextOption(self , rsthis: & QTextDocument) -> QTextOption {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument17defaultTextOptionEv()};
    let mut ret = unsafe {C_ZNK13QTextDocument17defaultTextOptionEv(rsthis.qclsinst)};
    let mut ret1 = QTextOption::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QTextBlock QTextDocument::findBlock(int pos);
impl /*struct*/ QTextDocument {
  pub fn findBlock<RetType, T: QTextDocument_findBlock<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.findBlock(self);
    // return 1;
  }
}

pub trait QTextDocument_findBlock<RetType> {
  fn findBlock(self , rsthis: & QTextDocument) -> RetType;
}

  // proto:  QTextBlock QTextDocument::findBlock(int pos);
impl<'a> /*trait*/ QTextDocument_findBlock<QTextBlock> for (i32) {
  fn findBlock(self , rsthis: & QTextDocument) -> QTextBlock {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument9findBlockEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZNK13QTextDocument9findBlockEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QTextBlock::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextDocument::setBaseUrl(const QUrl & url);
impl /*struct*/ QTextDocument {
  pub fn setBaseUrl<RetType, T: QTextDocument_setBaseUrl<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setBaseUrl(self);
    // return 1;
  }
}

pub trait QTextDocument_setBaseUrl<RetType> {
  fn setBaseUrl(self , rsthis: & QTextDocument) -> RetType;
}

  // proto:  void QTextDocument::setBaseUrl(const QUrl & url);
impl<'a> /*trait*/ QTextDocument_setBaseUrl<()> for (&'a QUrl) {
  fn setBaseUrl(self , rsthis: & QTextDocument) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument10setBaseUrlERK4QUrl()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN13QTextDocument10setBaseUrlERK4QUrl(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextDocument::appendUndoItem(QAbstractUndoItem * );
impl /*struct*/ QTextDocument {
  pub fn appendUndoItem<RetType, T: QTextDocument_appendUndoItem<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.appendUndoItem(self);
    // return 1;
  }
}

pub trait QTextDocument_appendUndoItem<RetType> {
  fn appendUndoItem(self , rsthis: & QTextDocument) -> RetType;
}

  // proto:  void QTextDocument::appendUndoItem(QAbstractUndoItem * );
impl<'a> /*trait*/ QTextDocument_appendUndoItem<()> for (&'a QAbstractUndoItem) {
  fn appendUndoItem(self , rsthis: & QTextDocument) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument14appendUndoItemEP17QAbstractUndoItem()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN13QTextDocument14appendUndoItemEP17QAbstractUndoItem(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextDocument::redo();
impl<'a> /*trait*/ QTextDocument_redo<()> for () {
  fn redo(self , rsthis: & QTextDocument) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument4redoEv()};
     unsafe {C_ZN13QTextDocument4redoEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTextDocument::drawContents(QPainter * painter, const QRectF & rect);
impl /*struct*/ QTextDocument {
  pub fn drawContents<RetType, T: QTextDocument_drawContents<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.drawContents(self);
    // return 1;
  }
}

pub trait QTextDocument_drawContents<RetType> {
  fn drawContents(self , rsthis: & QTextDocument) -> RetType;
}

  // proto:  void QTextDocument::drawContents(QPainter * painter, const QRectF & rect);
impl<'a> /*trait*/ QTextDocument_drawContents<()> for (&'a QPainter, &'a QRectF) {
  fn drawContents(self , rsthis: & QTextDocument) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument12drawContentsEP8QPainterRK6QRectF()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {C_ZN13QTextDocument12drawContentsEP8QPainterRK6QRectF(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QTextBlock QTextDocument::findBlockByLineNumber(int blockNumber);
impl /*struct*/ QTextDocument {
  pub fn findBlockByLineNumber<RetType, T: QTextDocument_findBlockByLineNumber<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.findBlockByLineNumber(self);
    // return 1;
  }
}

pub trait QTextDocument_findBlockByLineNumber<RetType> {
  fn findBlockByLineNumber(self , rsthis: & QTextDocument) -> RetType;
}

  // proto:  QTextBlock QTextDocument::findBlockByLineNumber(int blockNumber);
impl<'a> /*trait*/ QTextDocument_findBlockByLineNumber<QTextBlock> for (i32) {
  fn findBlockByLineNumber(self , rsthis: & QTextDocument) -> QTextBlock {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument21findBlockByLineNumberEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZNK13QTextDocument21findBlockByLineNumberEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QTextBlock::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextDocument::undo();
impl<'a> /*trait*/ QTextDocument_undo<()> for () {
  fn undo(self , rsthis: & QTextDocument) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument4undoEv()};
     unsafe {C_ZN13QTextDocument4undoEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  qreal QTextDocument::textWidth();
impl /*struct*/ QTextDocument {
  pub fn textWidth<RetType, T: QTextDocument_textWidth<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.textWidth(self);
    // return 1;
  }
}

pub trait QTextDocument_textWidth<RetType> {
  fn textWidth(self , rsthis: & QTextDocument) -> RetType;
}

  // proto:  qreal QTextDocument::textWidth();
impl<'a> /*trait*/ QTextDocument_textWidth<f64> for () {
  fn textWidth(self , rsthis: & QTextDocument) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument9textWidthEv()};
    let mut ret = unsafe {C_ZNK13QTextDocument9textWidthEv(rsthis.qclsinst)};
    return ret as f64; // 1
    // return 1;
  }
}

  // proto:  const QMetaObject * QTextDocument::metaObject();
impl /*struct*/ QTextDocument {
  pub fn metaObject<RetType, T: QTextDocument_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QTextDocument_metaObject<RetType> {
  fn metaObject(self , rsthis: & QTextDocument) -> RetType;
}

  // proto:  const QMetaObject * QTextDocument::metaObject();
impl<'a> /*trait*/ QTextDocument_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QTextDocument) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument10metaObjectEv()};
    let mut ret = unsafe {C_ZNK13QTextDocument10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QTextDocument::availableRedoSteps();
impl /*struct*/ QTextDocument {
  pub fn availableRedoSteps<RetType, T: QTextDocument_availableRedoSteps<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.availableRedoSteps(self);
    // return 1;
  }
}

pub trait QTextDocument_availableRedoSteps<RetType> {
  fn availableRedoSteps(self , rsthis: & QTextDocument) -> RetType;
}

  // proto:  int QTextDocument::availableRedoSteps();
impl<'a> /*trait*/ QTextDocument_availableRedoSteps<i32> for () {
  fn availableRedoSteps(self , rsthis: & QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument18availableRedoStepsEv()};
    let mut ret = unsafe {C_ZNK13QTextDocument18availableRedoStepsEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  QChar QTextDocument::characterAt(int pos);
impl /*struct*/ QTextDocument {
  pub fn characterAt<RetType, T: QTextDocument_characterAt<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.characterAt(self);
    // return 1;
  }
}

pub trait QTextDocument_characterAt<RetType> {
  fn characterAt(self , rsthis: & QTextDocument) -> RetType;
}

  // proto:  QChar QTextDocument::characterAt(int pos);
impl<'a> /*trait*/ QTextDocument_characterAt<QChar> for (i32) {
  fn characterAt(self , rsthis: & QTextDocument) -> QChar {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument11characterAtEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZNK13QTextDocument11characterAtEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QChar::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextDocument::setDefaultFont(const QFont & font);
impl /*struct*/ QTextDocument {
  pub fn setDefaultFont<RetType, T: QTextDocument_setDefaultFont<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setDefaultFont(self);
    // return 1;
  }
}

pub trait QTextDocument_setDefaultFont<RetType> {
  fn setDefaultFont(self , rsthis: & QTextDocument) -> RetType;
}

  // proto:  void QTextDocument::setDefaultFont(const QFont & font);
impl<'a> /*trait*/ QTextDocument_setDefaultFont<()> for (&'a QFont) {
  fn setDefaultFont(self , rsthis: & QTextDocument) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument14setDefaultFontERK5QFont()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN13QTextDocument14setDefaultFontERK5QFont(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QTextObject * QTextDocument::objectForFormat(const QTextFormat & );
impl /*struct*/ QTextDocument {
  pub fn objectForFormat<RetType, T: QTextDocument_objectForFormat<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.objectForFormat(self);
    // return 1;
  }
}

pub trait QTextDocument_objectForFormat<RetType> {
  fn objectForFormat(self , rsthis: & QTextDocument) -> RetType;
}

  // proto:  QTextObject * QTextDocument::objectForFormat(const QTextFormat & );
impl<'a> /*trait*/ QTextDocument_objectForFormat<QTextObject> for (&'a QTextFormat) {
  fn objectForFormat(self , rsthis: & QTextDocument) -> QTextObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument15objectForFormatERK11QTextFormat()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK13QTextDocument15objectForFormatERK11QTextFormat(rsthis.qclsinst, arg0)};
    let mut ret1 = QTextObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  bool QTextDocument::isEmpty();
impl /*struct*/ QTextDocument {
  pub fn isEmpty<RetType, T: QTextDocument_isEmpty<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isEmpty(self);
    // return 1;
  }
}

pub trait QTextDocument_isEmpty<RetType> {
  fn isEmpty(self , rsthis: & QTextDocument) -> RetType;
}

  // proto:  bool QTextDocument::isEmpty();
impl<'a> /*trait*/ QTextDocument_isEmpty<i8> for () {
  fn isEmpty(self , rsthis: & QTextDocument) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument7isEmptyEv()};
    let mut ret = unsafe {C_ZNK13QTextDocument7isEmptyEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  bool QTextDocument::isUndoRedoEnabled();
impl /*struct*/ QTextDocument {
  pub fn isUndoRedoEnabled<RetType, T: QTextDocument_isUndoRedoEnabled<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isUndoRedoEnabled(self);
    // return 1;
  }
}

pub trait QTextDocument_isUndoRedoEnabled<RetType> {
  fn isUndoRedoEnabled(self , rsthis: & QTextDocument) -> RetType;
}

  // proto:  bool QTextDocument::isUndoRedoEnabled();
impl<'a> /*trait*/ QTextDocument_isUndoRedoEnabled<i8> for () {
  fn isUndoRedoEnabled(self , rsthis: & QTextDocument) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument17isUndoRedoEnabledEv()};
    let mut ret = unsafe {C_ZNK13QTextDocument17isUndoRedoEnabledEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  void QTextDocument::~QTextDocument();
impl /*struct*/ QTextDocument {
  pub fn free<RetType, T: QTextDocument_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QTextDocument_free<RetType> {
  fn free(self , rsthis: & QTextDocument) -> RetType;
}

  // proto:  void QTextDocument::~QTextDocument();
impl<'a> /*trait*/ QTextDocument_free<()> for () {
  fn free(self , rsthis: & QTextDocument) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocumentD2Ev()};
     unsafe {C_ZN13QTextDocumentD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  qreal QTextDocument::documentMargin();
impl /*struct*/ QTextDocument {
  pub fn documentMargin<RetType, T: QTextDocument_documentMargin<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.documentMargin(self);
    // return 1;
  }
}

pub trait QTextDocument_documentMargin<RetType> {
  fn documentMargin(self , rsthis: & QTextDocument) -> RetType;
}

  // proto:  qreal QTextDocument::documentMargin();
impl<'a> /*trait*/ QTextDocument_documentMargin<f64> for () {
  fn documentMargin(self , rsthis: & QTextDocument) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument14documentMarginEv()};
    let mut ret = unsafe {C_ZNK13QTextDocument14documentMarginEv(rsthis.qclsinst)};
    return ret as f64; // 1
    // return 1;
  }
}

  // proto:  void QTextDocument::setPageSize(const QSizeF & size);
impl /*struct*/ QTextDocument {
  pub fn setPageSize<RetType, T: QTextDocument_setPageSize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPageSize(self);
    // return 1;
  }
}

pub trait QTextDocument_setPageSize<RetType> {
  fn setPageSize(self , rsthis: & QTextDocument) -> RetType;
}

  // proto:  void QTextDocument::setPageSize(const QSizeF & size);
impl<'a> /*trait*/ QTextDocument_setPageSize<()> for (&'a QSizeF) {
  fn setPageSize(self , rsthis: & QTextDocument) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument11setPageSizeERK6QSizeF()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN13QTextDocument11setPageSizeERK6QSizeF(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextDocument::setHtml(const QString & html);
impl /*struct*/ QTextDocument {
  pub fn setHtml<RetType, T: QTextDocument_setHtml<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setHtml(self);
    // return 1;
  }
}

pub trait QTextDocument_setHtml<RetType> {
  fn setHtml(self , rsthis: & QTextDocument) -> RetType;
}

  // proto:  void QTextDocument::setHtml(const QString & html);
impl<'a> /*trait*/ QTextDocument_setHtml<()> for (&'a QString) {
  fn setHtml(self , rsthis: & QTextDocument) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument7setHtmlERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN13QTextDocument7setHtmlERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QTextBlock QTextDocument::end();
impl /*struct*/ QTextDocument {
  pub fn end<RetType, T: QTextDocument_end<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.end(self);
    // return 1;
  }
}

pub trait QTextDocument_end<RetType> {
  fn end(self , rsthis: & QTextDocument) -> RetType;
}

  // proto:  QTextBlock QTextDocument::end();
impl<'a> /*trait*/ QTextDocument_end<QTextBlock> for () {
  fn end(self , rsthis: & QTextDocument) -> QTextBlock {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument3endEv()};
    let mut ret = unsafe {C_ZNK13QTextDocument3endEv(rsthis.qclsinst)};
    let mut ret1 = QTextBlock::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QTextDocument::maximumBlockCount();
impl /*struct*/ QTextDocument {
  pub fn maximumBlockCount<RetType, T: QTextDocument_maximumBlockCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.maximumBlockCount(self);
    // return 1;
  }
}

pub trait QTextDocument_maximumBlockCount<RetType> {
  fn maximumBlockCount(self , rsthis: & QTextDocument) -> RetType;
}

  // proto:  int QTextDocument::maximumBlockCount();
impl<'a> /*trait*/ QTextDocument_maximumBlockCount<i32> for () {
  fn maximumBlockCount(self , rsthis: & QTextDocument) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument17maximumBlockCountEv()};
    let mut ret = unsafe {C_ZNK13QTextDocument17maximumBlockCountEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  void QTextDocument::setPlainText(const QString & text);
impl /*struct*/ QTextDocument {
  pub fn setPlainText<RetType, T: QTextDocument_setPlainText<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPlainText(self);
    // return 1;
  }
}

pub trait QTextDocument_setPlainText<RetType> {
  fn setPlainText(self , rsthis: & QTextDocument) -> RetType;
}

  // proto:  void QTextDocument::setPlainText(const QString & text);
impl<'a> /*trait*/ QTextDocument_setPlainText<()> for (&'a QString) {
  fn setPlainText(self , rsthis: & QTextDocument) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument12setPlainTextERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN13QTextDocument12setPlainTextERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextDocument::clear();
impl /*struct*/ QTextDocument {
  pub fn clear<RetType, T: QTextDocument_clear<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.clear(self);
    // return 1;
  }
}

pub trait QTextDocument_clear<RetType> {
  fn clear(self , rsthis: & QTextDocument) -> RetType;
}

  // proto:  void QTextDocument::clear();
impl<'a> /*trait*/ QTextDocument_clear<()> for () {
  fn clear(self , rsthis: & QTextDocument) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument5clearEv()};
     unsafe {C_ZN13QTextDocument5clearEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QVariant QTextDocument::resource(int type, const QUrl & name);
impl /*struct*/ QTextDocument {
  pub fn resource<RetType, T: QTextDocument_resource<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.resource(self);
    // return 1;
  }
}

pub trait QTextDocument_resource<RetType> {
  fn resource(self , rsthis: & QTextDocument) -> RetType;
}

  // proto:  QVariant QTextDocument::resource(int type, const QUrl & name);
impl<'a> /*trait*/ QTextDocument_resource<QVariant> for (i32, &'a QUrl) {
  fn resource(self , rsthis: & QTextDocument) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument8resourceEiRK4QUrl()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK13QTextDocument8resourceEiRK4QUrl(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QVariant::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QTextBlock QTextDocument::begin();
impl /*struct*/ QTextDocument {
  pub fn begin<RetType, T: QTextDocument_begin<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.begin(self);
    // return 1;
  }
}

pub trait QTextDocument_begin<RetType> {
  fn begin(self , rsthis: & QTextDocument) -> RetType;
}

  // proto:  QTextBlock QTextDocument::begin();
impl<'a> /*trait*/ QTextDocument_begin<QTextBlock> for () {
  fn begin(self , rsthis: & QTextDocument) -> QTextBlock {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK13QTextDocument5beginEv()};
    let mut ret = unsafe {C_ZNK13QTextDocument5beginEv(rsthis.qclsinst)};
    let mut ret1 = QTextBlock::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextDocument::setMaximumBlockCount(int maximum);
impl /*struct*/ QTextDocument {
  pub fn setMaximumBlockCount<RetType, T: QTextDocument_setMaximumBlockCount<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMaximumBlockCount(self);
    // return 1;
  }
}

pub trait QTextDocument_setMaximumBlockCount<RetType> {
  fn setMaximumBlockCount(self , rsthis: & QTextDocument) -> RetType;
}

  // proto:  void QTextDocument::setMaximumBlockCount(int maximum);
impl<'a> /*trait*/ QTextDocument_setMaximumBlockCount<()> for (i32) {
  fn setMaximumBlockCount(self , rsthis: & QTextDocument) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument20setMaximumBlockCountEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN13QTextDocument20setMaximumBlockCountEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextDocument::setModified(bool m);
impl /*struct*/ QTextDocument {
  pub fn setModified<RetType, T: QTextDocument_setModified<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setModified(self);
    // return 1;
  }
}

pub trait QTextDocument_setModified<RetType> {
  fn setModified(self , rsthis: & QTextDocument) -> RetType;
}

  // proto:  void QTextDocument::setModified(bool m);
impl<'a> /*trait*/ QTextDocument_setModified<()> for (i8) {
  fn setModified(self , rsthis: & QTextDocument) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN13QTextDocument11setModifiedEb()};
    let arg0 = self  as c_char;
     unsafe {C_ZN13QTextDocument11setModifiedEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QAbstractUndoItem {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QAbstractUndoItem {
    return QAbstractUndoItem{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QAbstractUndoItem::undo();
impl /*struct*/ QAbstractUndoItem {
  pub fn undo<RetType, T: QAbstractUndoItem_undo<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.undo(self);
    // return 1;
  }
}

pub trait QAbstractUndoItem_undo<RetType> {
  fn undo(self , rsthis: & QAbstractUndoItem) -> RetType;
}

  // proto:  void QAbstractUndoItem::undo();
impl<'a> /*trait*/ QAbstractUndoItem_undo<()> for () {
  fn undo(self , rsthis: & QAbstractUndoItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QAbstractUndoItem4undoEv()};
     unsafe {C_ZN17QAbstractUndoItem4undoEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QAbstractUndoItem::redo();
impl /*struct*/ QAbstractUndoItem {
  pub fn redo<RetType, T: QAbstractUndoItem_redo<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.redo(self);
    // return 1;
  }
}

pub trait QAbstractUndoItem_redo<RetType> {
  fn redo(self , rsthis: & QAbstractUndoItem) -> RetType;
}

  // proto:  void QAbstractUndoItem::redo();
impl<'a> /*trait*/ QAbstractUndoItem_redo<()> for () {
  fn redo(self , rsthis: & QAbstractUndoItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QAbstractUndoItem4redoEv()};
     unsafe {C_ZN17QAbstractUndoItem4redoEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QAbstractUndoItem::~QAbstractUndoItem();
impl /*struct*/ QAbstractUndoItem {
  pub fn free<RetType, T: QAbstractUndoItem_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QAbstractUndoItem_free<RetType> {
  fn free(self , rsthis: & QAbstractUndoItem) -> RetType;
}

  // proto:  void QAbstractUndoItem::~QAbstractUndoItem();
impl<'a> /*trait*/ QAbstractUndoItem_free<()> for () {
  fn free(self , rsthis: & QAbstractUndoItem) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QAbstractUndoItemD2Ev()};
     unsafe {C_ZN17QAbstractUndoItemD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

#[derive(Default)] // for QTextDocument_cursorPositionChanged
pub struct QTextDocument_cursorPositionChanged_signal{poi:u64}
impl /* struct */ QTextDocument {
  pub fn cursorPositionChanged(&self) -> QTextDocument_cursorPositionChanged_signal {
     return QTextDocument_cursorPositionChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QTextDocument_cursorPositionChanged_signal {
  pub fn connect<T: QTextDocument_cursorPositionChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QTextDocument_cursorPositionChanged_signal_connect {
  fn connect(self, sigthis: QTextDocument_cursorPositionChanged_signal);
}

#[derive(Default)] // for QTextDocument_documentLayoutChanged
pub struct QTextDocument_documentLayoutChanged_signal{poi:u64}
impl /* struct */ QTextDocument {
  pub fn documentLayoutChanged(&self) -> QTextDocument_documentLayoutChanged_signal {
     return QTextDocument_documentLayoutChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QTextDocument_documentLayoutChanged_signal {
  pub fn connect<T: QTextDocument_documentLayoutChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QTextDocument_documentLayoutChanged_signal_connect {
  fn connect(self, sigthis: QTextDocument_documentLayoutChanged_signal);
}

#[derive(Default)] // for QTextDocument_undoCommandAdded
pub struct QTextDocument_undoCommandAdded_signal{poi:u64}
impl /* struct */ QTextDocument {
  pub fn undoCommandAdded(&self) -> QTextDocument_undoCommandAdded_signal {
     return QTextDocument_undoCommandAdded_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QTextDocument_undoCommandAdded_signal {
  pub fn connect<T: QTextDocument_undoCommandAdded_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QTextDocument_undoCommandAdded_signal_connect {
  fn connect(self, sigthis: QTextDocument_undoCommandAdded_signal);
}

#[derive(Default)] // for QTextDocument_modificationChanged
pub struct QTextDocument_modificationChanged_signal{poi:u64}
impl /* struct */ QTextDocument {
  pub fn modificationChanged(&self) -> QTextDocument_modificationChanged_signal {
     return QTextDocument_modificationChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QTextDocument_modificationChanged_signal {
  pub fn connect<T: QTextDocument_modificationChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QTextDocument_modificationChanged_signal_connect {
  fn connect(self, sigthis: QTextDocument_modificationChanged_signal);
}

#[derive(Default)] // for QTextDocument_redoAvailable
pub struct QTextDocument_redoAvailable_signal{poi:u64}
impl /* struct */ QTextDocument {
  pub fn redoAvailable(&self) -> QTextDocument_redoAvailable_signal {
     return QTextDocument_redoAvailable_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QTextDocument_redoAvailable_signal {
  pub fn connect<T: QTextDocument_redoAvailable_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QTextDocument_redoAvailable_signal_connect {
  fn connect(self, sigthis: QTextDocument_redoAvailable_signal);
}

#[derive(Default)] // for QTextDocument_contentsChanged
pub struct QTextDocument_contentsChanged_signal{poi:u64}
impl /* struct */ QTextDocument {
  pub fn contentsChanged(&self) -> QTextDocument_contentsChanged_signal {
     return QTextDocument_contentsChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QTextDocument_contentsChanged_signal {
  pub fn connect<T: QTextDocument_contentsChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QTextDocument_contentsChanged_signal_connect {
  fn connect(self, sigthis: QTextDocument_contentsChanged_signal);
}

#[derive(Default)] // for QTextDocument_baseUrlChanged
pub struct QTextDocument_baseUrlChanged_signal{poi:u64}
impl /* struct */ QTextDocument {
  pub fn baseUrlChanged(&self) -> QTextDocument_baseUrlChanged_signal {
     return QTextDocument_baseUrlChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QTextDocument_baseUrlChanged_signal {
  pub fn connect<T: QTextDocument_baseUrlChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QTextDocument_baseUrlChanged_signal_connect {
  fn connect(self, sigthis: QTextDocument_baseUrlChanged_signal);
}

#[derive(Default)] // for QTextDocument_blockCountChanged
pub struct QTextDocument_blockCountChanged_signal{poi:u64}
impl /* struct */ QTextDocument {
  pub fn blockCountChanged(&self) -> QTextDocument_blockCountChanged_signal {
     return QTextDocument_blockCountChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QTextDocument_blockCountChanged_signal {
  pub fn connect<T: QTextDocument_blockCountChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QTextDocument_blockCountChanged_signal_connect {
  fn connect(self, sigthis: QTextDocument_blockCountChanged_signal);
}

#[derive(Default)] // for QTextDocument_undoAvailable
pub struct QTextDocument_undoAvailable_signal{poi:u64}
impl /* struct */ QTextDocument {
  pub fn undoAvailable(&self) -> QTextDocument_undoAvailable_signal {
     return QTextDocument_undoAvailable_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QTextDocument_undoAvailable_signal {
  pub fn connect<T: QTextDocument_undoAvailable_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QTextDocument_undoAvailable_signal_connect {
  fn connect(self, sigthis: QTextDocument_undoAvailable_signal);
}

#[derive(Default)] // for QTextDocument_contentsChange
pub struct QTextDocument_contentsChange_signal{poi:u64}
impl /* struct */ QTextDocument {
  pub fn contentsChange(&self) -> QTextDocument_contentsChange_signal {
     return QTextDocument_contentsChange_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QTextDocument_contentsChange_signal {
  pub fn connect<T: QTextDocument_contentsChange_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QTextDocument_contentsChange_signal_connect {
  fn connect(self, sigthis: QTextDocument_contentsChange_signal);
}

// cursorPositionChanged(const class QTextCursor &)
extern fn QTextDocument_cursorPositionChanged_signal_connect_cb_0(rsfptr:fn(QTextCursor), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QTextCursor::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QTextDocument_cursorPositionChanged_signal_connect_cb_box_0(rsfptr_raw:*mut Box<Fn(QTextCursor)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QTextCursor::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QTextDocument_cursorPositionChanged_signal_connect for fn(QTextCursor) {
  fn connect(self, sigthis: QTextDocument_cursorPositionChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTextDocument_cursorPositionChanged_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QTextDocument_SlotProxy_connect__ZN13QTextDocument21cursorPositionChangedERK11QTextCursor(arg0, arg1, arg2)};
  }
}
impl /* trait */ QTextDocument_cursorPositionChanged_signal_connect for Box<Fn(QTextCursor)> {
  fn connect(self, sigthis: QTextDocument_cursorPositionChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTextDocument_cursorPositionChanged_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QTextDocument_SlotProxy_connect__ZN13QTextDocument21cursorPositionChangedERK11QTextCursor(arg0, arg1, arg2)};
  }
}
// modificationChanged(_Bool)
extern fn QTextDocument_modificationChanged_signal_connect_cb_1(rsfptr:fn(i8), arg0: c_char) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i8;
  rsfptr(rsarg0);
}
extern fn QTextDocument_modificationChanged_signal_connect_cb_box_1(rsfptr_raw:*mut Box<Fn(i8)>, arg0: c_char) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i8;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QTextDocument_modificationChanged_signal_connect for fn(i8) {
  fn connect(self, sigthis: QTextDocument_modificationChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTextDocument_modificationChanged_signal_connect_cb_1 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QTextDocument_SlotProxy_connect__ZN13QTextDocument19modificationChangedEb(arg0, arg1, arg2)};
  }
}
impl /* trait */ QTextDocument_modificationChanged_signal_connect for Box<Fn(i8)> {
  fn connect(self, sigthis: QTextDocument_modificationChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTextDocument_modificationChanged_signal_connect_cb_box_1 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QTextDocument_SlotProxy_connect__ZN13QTextDocument19modificationChangedEb(arg0, arg1, arg2)};
  }
}
// undoAvailable(_Bool)
extern fn QTextDocument_undoAvailable_signal_connect_cb_2(rsfptr:fn(i8), arg0: c_char) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i8;
  rsfptr(rsarg0);
}
extern fn QTextDocument_undoAvailable_signal_connect_cb_box_2(rsfptr_raw:*mut Box<Fn(i8)>, arg0: c_char) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i8;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QTextDocument_undoAvailable_signal_connect for fn(i8) {
  fn connect(self, sigthis: QTextDocument_undoAvailable_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTextDocument_undoAvailable_signal_connect_cb_2 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QTextDocument_SlotProxy_connect__ZN13QTextDocument13undoAvailableEb(arg0, arg1, arg2)};
  }
}
impl /* trait */ QTextDocument_undoAvailable_signal_connect for Box<Fn(i8)> {
  fn connect(self, sigthis: QTextDocument_undoAvailable_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTextDocument_undoAvailable_signal_connect_cb_box_2 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QTextDocument_SlotProxy_connect__ZN13QTextDocument13undoAvailableEb(arg0, arg1, arg2)};
  }
}
// contentsChanged()
extern fn QTextDocument_contentsChanged_signal_connect_cb_3(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QTextDocument_contentsChanged_signal_connect_cb_box_3(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QTextDocument_contentsChanged_signal_connect for fn() {
  fn connect(self, sigthis: QTextDocument_contentsChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTextDocument_contentsChanged_signal_connect_cb_3 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QTextDocument_SlotProxy_connect__ZN13QTextDocument15contentsChangedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QTextDocument_contentsChanged_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QTextDocument_contentsChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTextDocument_contentsChanged_signal_connect_cb_box_3 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QTextDocument_SlotProxy_connect__ZN13QTextDocument15contentsChangedEv(arg0, arg1, arg2)};
  }
}
// baseUrlChanged(const class QUrl &)
extern fn QTextDocument_baseUrlChanged_signal_connect_cb_4(rsfptr:fn(QUrl), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QUrl::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QTextDocument_baseUrlChanged_signal_connect_cb_box_4(rsfptr_raw:*mut Box<Fn(QUrl)>, arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QUrl::inheritFrom(arg0 as u64);
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QTextDocument_baseUrlChanged_signal_connect for fn(QUrl) {
  fn connect(self, sigthis: QTextDocument_baseUrlChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTextDocument_baseUrlChanged_signal_connect_cb_4 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QTextDocument_SlotProxy_connect__ZN13QTextDocument14baseUrlChangedERK4QUrl(arg0, arg1, arg2)};
  }
}
impl /* trait */ QTextDocument_baseUrlChanged_signal_connect for Box<Fn(QUrl)> {
  fn connect(self, sigthis: QTextDocument_baseUrlChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTextDocument_baseUrlChanged_signal_connect_cb_box_4 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QTextDocument_SlotProxy_connect__ZN13QTextDocument14baseUrlChangedERK4QUrl(arg0, arg1, arg2)};
  }
}
// blockCountChanged(int)
extern fn QTextDocument_blockCountChanged_signal_connect_cb_5(rsfptr:fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
extern fn QTextDocument_blockCountChanged_signal_connect_cb_box_5(rsfptr_raw:*mut Box<Fn(i32)>, arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QTextDocument_blockCountChanged_signal_connect for fn(i32) {
  fn connect(self, sigthis: QTextDocument_blockCountChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTextDocument_blockCountChanged_signal_connect_cb_5 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QTextDocument_SlotProxy_connect__ZN13QTextDocument17blockCountChangedEi(arg0, arg1, arg2)};
  }
}
impl /* trait */ QTextDocument_blockCountChanged_signal_connect for Box<Fn(i32)> {
  fn connect(self, sigthis: QTextDocument_blockCountChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTextDocument_blockCountChanged_signal_connect_cb_box_5 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QTextDocument_SlotProxy_connect__ZN13QTextDocument17blockCountChangedEi(arg0, arg1, arg2)};
  }
}
// documentLayoutChanged()
extern fn QTextDocument_documentLayoutChanged_signal_connect_cb_6(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QTextDocument_documentLayoutChanged_signal_connect_cb_box_6(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QTextDocument_documentLayoutChanged_signal_connect for fn() {
  fn connect(self, sigthis: QTextDocument_documentLayoutChanged_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTextDocument_documentLayoutChanged_signal_connect_cb_6 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QTextDocument_SlotProxy_connect__ZN13QTextDocument21documentLayoutChangedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QTextDocument_documentLayoutChanged_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QTextDocument_documentLayoutChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTextDocument_documentLayoutChanged_signal_connect_cb_box_6 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QTextDocument_SlotProxy_connect__ZN13QTextDocument21documentLayoutChangedEv(arg0, arg1, arg2)};
  }
}
// undoCommandAdded()
extern fn QTextDocument_undoCommandAdded_signal_connect_cb_7(rsfptr:fn(), ) {
  println!("{}:{}", file!(), line!());
  rsfptr();
}
extern fn QTextDocument_undoCommandAdded_signal_connect_cb_box_7(rsfptr_raw:*mut Box<Fn()>, ) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  // rsfptr();
  unsafe{(*rsfptr_raw)()};
}
impl /* trait */ QTextDocument_undoCommandAdded_signal_connect for fn() {
  fn connect(self, sigthis: QTextDocument_undoCommandAdded_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTextDocument_undoCommandAdded_signal_connect_cb_7 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QTextDocument_SlotProxy_connect__ZN13QTextDocument16undoCommandAddedEv(arg0, arg1, arg2)};
  }
}
impl /* trait */ QTextDocument_undoCommandAdded_signal_connect for Box<Fn()> {
  fn connect(self, sigthis: QTextDocument_undoCommandAdded_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTextDocument_undoCommandAdded_signal_connect_cb_box_7 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QTextDocument_SlotProxy_connect__ZN13QTextDocument16undoCommandAddedEv(arg0, arg1, arg2)};
  }
}
// redoAvailable(_Bool)
extern fn QTextDocument_redoAvailable_signal_connect_cb_8(rsfptr:fn(i8), arg0: c_char) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i8;
  rsfptr(rsarg0);
}
extern fn QTextDocument_redoAvailable_signal_connect_cb_box_8(rsfptr_raw:*mut Box<Fn(i8)>, arg0: c_char) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i8;
  // rsfptr(rsarg0);
  unsafe{(*rsfptr_raw)(rsarg0)};
}
impl /* trait */ QTextDocument_redoAvailable_signal_connect for fn(i8) {
  fn connect(self, sigthis: QTextDocument_redoAvailable_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTextDocument_redoAvailable_signal_connect_cb_8 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QTextDocument_SlotProxy_connect__ZN13QTextDocument13redoAvailableEb(arg0, arg1, arg2)};
  }
}
impl /* trait */ QTextDocument_redoAvailable_signal_connect for Box<Fn(i8)> {
  fn connect(self, sigthis: QTextDocument_redoAvailable_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTextDocument_redoAvailable_signal_connect_cb_box_8 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QTextDocument_SlotProxy_connect__ZN13QTextDocument13redoAvailableEb(arg0, arg1, arg2)};
  }
}
// contentsChange(int, int, int)
extern fn QTextDocument_contentsChange_signal_connect_cb_9(rsfptr:fn(i32, i32, i32), arg0: c_int, arg1: c_int, arg2: c_int) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i32;
  let rsarg1 = arg1 as i32;
  let rsarg2 = arg2 as i32;
  rsfptr(rsarg0,rsarg1,rsarg2);
}
extern fn QTextDocument_contentsChange_signal_connect_cb_box_9(rsfptr_raw:*mut Box<Fn(i32, i32, i32)>, arg0: c_int, arg1: c_int, arg2: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  let rsarg1 = arg1 as i32;
  let rsarg2 = arg2 as i32;
  // rsfptr(rsarg0,rsarg1,rsarg2);
  unsafe{(*rsfptr_raw)(rsarg0,rsarg1,rsarg2)};
}
impl /* trait */ QTextDocument_contentsChange_signal_connect for fn(i32, i32, i32) {
  fn connect(self, sigthis: QTextDocument_contentsChange_signal) {
    // do smth...
    // self as u64; // error for Fn, Ok for fn
    self as *mut c_void as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTextDocument_contentsChange_signal_connect_cb_9 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QTextDocument_SlotProxy_connect__ZN13QTextDocument14contentsChangeEiii(arg0, arg1, arg2)};
  }
}
impl /* trait */ QTextDocument_contentsChange_signal_connect for Box<Fn(i32, i32, i32)> {
  fn connect(self, sigthis: QTextDocument_contentsChange_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QTextDocument_contentsChange_signal_connect_cb_box_9 as *mut c_void;
    let arg2 = Box::into_raw(Box::new(self)) as *mut c_void;
    unsafe {QTextDocument_SlotProxy_connect__ZN13QTextDocument14contentsChangeEiii(arg0, arg1, arg2)};
  }
}
// <= body block end

