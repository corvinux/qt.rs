// auto generated, do not modify.
// created: Mon Feb 22 23:57:02 2016
// src-file: /QtGui/qtexttable.h
// dst-file: /src/gui/qtexttable.rs
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
use std::ops::Deref;
use super::qtextformat::*; // 773
use super::qtextcursor::*; // 773
use super::qtextobject::*; // 773
// use super::qtexttable::QTextTableCell; // 773
use super::qtextdocument::*; // 773
use super::super::core::qobjectdefs::*; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QTextTableCell_Class_Size() -> c_int;
  // proto:  void QTextTableCell::QTextTableCell(const QTextTableCell & o);
  fn C_ZN14QTextTableCellC2ERKS_(arg0: *mut c_void) -> u64;
  // proto:  void QTextTableCell::setFormat(const QTextCharFormat & format);
  fn C_ZN14QTextTableCell9setFormatERK15QTextCharFormat(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  int QTextTableCell::lastPosition();
  fn C_ZNK14QTextTableCell12lastPositionEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QTextTableCell::~QTextTableCell();
  fn C_ZN14QTextTableCellD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  int QTextTableCell::rowSpan();
  fn C_ZNK14QTextTableCell7rowSpanEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  int QTextTableCell::firstPosition();
  fn C_ZNK14QTextTableCell13firstPositionEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  int QTextTableCell::tableCellFormatIndex();
  fn C_ZNK14QTextTableCell20tableCellFormatIndexEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  int QTextTableCell::columnSpan();
  fn C_ZNK14QTextTableCell10columnSpanEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  QTextCharFormat QTextTableCell::format();
  fn C_ZNK14QTextTableCell6formatEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QTextTableCell::row();
  fn C_ZNK14QTextTableCell3rowEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  bool QTextTableCell::isValid();
  fn C_ZNK14QTextTableCell7isValidEv(qthis: u64 /* *mut c_void*/) -> c_char;
  // proto:  QTextCursor QTextTableCell::lastCursorPosition();
  fn C_ZNK14QTextTableCell18lastCursorPositionEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  int QTextTableCell::column();
  fn C_ZNK14QTextTableCell6columnEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  QTextCursor QTextTableCell::firstCursorPosition();
  fn C_ZNK14QTextTableCell19firstCursorPositionEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QTextTableCell::QTextTableCell();
  fn C_ZN14QTextTableCellC2Ev() -> u64;
  fn QTextTable_Class_Size() -> c_int;
  // proto:  QTextTableCell QTextTable::cellAt(int row, int col);
  fn C_ZNK10QTextTable6cellAtEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int) -> *mut c_void;
  // proto:  int QTextTable::rows();
  fn C_ZNK10QTextTable4rowsEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QTextTable::removeRows(int pos, int num);
  fn C_ZN10QTextTable10removeRowsEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int);
  // proto:  int QTextTable::columns();
  fn C_ZNK10QTextTable7columnsEv(qthis: u64 /* *mut c_void*/) -> c_int;
  // proto:  void QTextTable::appendRows(int count);
  fn C_ZN10QTextTable10appendRowsEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QTextTable::resize(int rows, int cols);
  fn C_ZN10QTextTable6resizeEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int);
  // proto:  QTextTableCell QTextTable::cellAt(const QTextCursor & c);
  fn C_ZNK10QTextTable6cellAtERK11QTextCursor(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QTextTable::setFormat(const QTextTableFormat & format);
  fn C_ZN10QTextTable9setFormatERK16QTextTableFormat(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QTextTable::insertColumns(int pos, int num);
  fn C_ZN10QTextTable13insertColumnsEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int);
  // proto:  void QTextTable::splitCell(int row, int col, int numRows, int numCols);
  fn C_ZN10QTextTable9splitCellEiiii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int);
  // proto:  void QTextTable::mergeCells(int row, int col, int numRows, int numCols);
  fn C_ZN10QTextTable10mergeCellsEiiii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int);
  // proto:  void QTextTable::insertRows(int pos, int num);
  fn C_ZN10QTextTable10insertRowsEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int);
  // proto:  void QTextTable::~QTextTable();
  fn C_ZN10QTextTableD2Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QTextTable::QTextTable(QTextDocument * doc);
  fn C_ZN10QTextTableC2EP13QTextDocument(arg0: *mut c_void) -> u64;
  // proto:  QTextTableCell QTextTable::cellAt(int position);
  fn C_ZNK10QTextTable6cellAtEi(qthis: u64 /* *mut c_void*/, arg0: c_int) -> *mut c_void;
  // proto:  QTextCursor QTextTable::rowStart(const QTextCursor & c);
  fn C_ZNK10QTextTable8rowStartERK11QTextCursor(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  QTextTableFormat QTextTable::format();
  fn C_ZNK10QTextTable6formatEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QTextCursor QTextTable::rowEnd(const QTextCursor & c);
  fn C_ZNK10QTextTable6rowEndERK11QTextCursor(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  const QMetaObject * QTextTable::metaObject();
  fn C_ZNK10QTextTable10metaObjectEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QTextTable::removeColumns(int pos, int num);
  fn C_ZN10QTextTable13removeColumnsEii(qthis: u64 /* *mut c_void*/, arg0: c_int, arg1: c_int);
  // proto:  void QTextTable::appendColumns(int count);
  fn C_ZN10QTextTable13appendColumnsEi(qthis: u64 /* *mut c_void*/, arg0: c_int);
  // proto:  void QTextTable::mergeCells(const QTextCursor & cursor);
  fn C_ZN10QTextTable10mergeCellsERK11QTextCursor(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QTextTableCell)=16
#[derive(Default)]
pub struct QTextTableCell {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

// class sizeof(QTextTable)=1
#[derive(Default)]
pub struct QTextTable {
  qbase: QTextFrame,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QTextTableCell {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QTextTableCell {
    return QTextTableCell{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  void QTextTableCell::QTextTableCell(const QTextTableCell & o);
impl /*struct*/ QTextTableCell {
  pub fn new<T: QTextTableCell_new>(value: T) -> QTextTableCell {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QTextTableCell_new {
  fn new(self) -> QTextTableCell;
}

  // proto:  void QTextTableCell::QTextTableCell(const QTextTableCell & o);
impl<'a> /*trait*/ QTextTableCell_new for (&'a QTextTableCell) {
  fn new(self) -> QTextTableCell {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QTextTableCellC2ERKS_()};
    let ctysz: c_int = unsafe{QTextTableCell_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN14QTextTableCellC2ERKS_(arg0)};
    let rsthis = QTextTableCell{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QTextTableCell::setFormat(const QTextCharFormat & format);
impl /*struct*/ QTextTableCell {
  pub fn setFormat<RetType, T: QTextTableCell_setFormat<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFormat(self);
    // return 1;
  }
}

pub trait QTextTableCell_setFormat<RetType> {
  fn setFormat(self , rsthis: & QTextTableCell) -> RetType;
}

  // proto:  void QTextTableCell::setFormat(const QTextCharFormat & format);
impl<'a> /*trait*/ QTextTableCell_setFormat<()> for (&'a QTextCharFormat) {
  fn setFormat(self , rsthis: & QTextTableCell) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QTextTableCell9setFormatERK15QTextCharFormat()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN14QTextTableCell9setFormatERK15QTextCharFormat(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  int QTextTableCell::lastPosition();
impl /*struct*/ QTextTableCell {
  pub fn lastPosition<RetType, T: QTextTableCell_lastPosition<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.lastPosition(self);
    // return 1;
  }
}

pub trait QTextTableCell_lastPosition<RetType> {
  fn lastPosition(self , rsthis: & QTextTableCell) -> RetType;
}

  // proto:  int QTextTableCell::lastPosition();
impl<'a> /*trait*/ QTextTableCell_lastPosition<i32> for () {
  fn lastPosition(self , rsthis: & QTextTableCell) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QTextTableCell12lastPositionEv()};
    let mut ret = unsafe {C_ZNK14QTextTableCell12lastPositionEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  void QTextTableCell::~QTextTableCell();
impl /*struct*/ QTextTableCell {
  pub fn free<RetType, T: QTextTableCell_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QTextTableCell_free<RetType> {
  fn free(self , rsthis: & QTextTableCell) -> RetType;
}

  // proto:  void QTextTableCell::~QTextTableCell();
impl<'a> /*trait*/ QTextTableCell_free<()> for () {
  fn free(self , rsthis: & QTextTableCell) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QTextTableCellD2Ev()};
     unsafe {C_ZN14QTextTableCellD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QTextTableCell::rowSpan();
impl /*struct*/ QTextTableCell {
  pub fn rowSpan<RetType, T: QTextTableCell_rowSpan<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rowSpan(self);
    // return 1;
  }
}

pub trait QTextTableCell_rowSpan<RetType> {
  fn rowSpan(self , rsthis: & QTextTableCell) -> RetType;
}

  // proto:  int QTextTableCell::rowSpan();
impl<'a> /*trait*/ QTextTableCell_rowSpan<i32> for () {
  fn rowSpan(self , rsthis: & QTextTableCell) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QTextTableCell7rowSpanEv()};
    let mut ret = unsafe {C_ZNK14QTextTableCell7rowSpanEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  int QTextTableCell::firstPosition();
impl /*struct*/ QTextTableCell {
  pub fn firstPosition<RetType, T: QTextTableCell_firstPosition<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.firstPosition(self);
    // return 1;
  }
}

pub trait QTextTableCell_firstPosition<RetType> {
  fn firstPosition(self , rsthis: & QTextTableCell) -> RetType;
}

  // proto:  int QTextTableCell::firstPosition();
impl<'a> /*trait*/ QTextTableCell_firstPosition<i32> for () {
  fn firstPosition(self , rsthis: & QTextTableCell) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QTextTableCell13firstPositionEv()};
    let mut ret = unsafe {C_ZNK14QTextTableCell13firstPositionEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  int QTextTableCell::tableCellFormatIndex();
impl /*struct*/ QTextTableCell {
  pub fn tableCellFormatIndex<RetType, T: QTextTableCell_tableCellFormatIndex<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.tableCellFormatIndex(self);
    // return 1;
  }
}

pub trait QTextTableCell_tableCellFormatIndex<RetType> {
  fn tableCellFormatIndex(self , rsthis: & QTextTableCell) -> RetType;
}

  // proto:  int QTextTableCell::tableCellFormatIndex();
impl<'a> /*trait*/ QTextTableCell_tableCellFormatIndex<i32> for () {
  fn tableCellFormatIndex(self , rsthis: & QTextTableCell) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QTextTableCell20tableCellFormatIndexEv()};
    let mut ret = unsafe {C_ZNK14QTextTableCell20tableCellFormatIndexEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  int QTextTableCell::columnSpan();
impl /*struct*/ QTextTableCell {
  pub fn columnSpan<RetType, T: QTextTableCell_columnSpan<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.columnSpan(self);
    // return 1;
  }
}

pub trait QTextTableCell_columnSpan<RetType> {
  fn columnSpan(self , rsthis: & QTextTableCell) -> RetType;
}

  // proto:  int QTextTableCell::columnSpan();
impl<'a> /*trait*/ QTextTableCell_columnSpan<i32> for () {
  fn columnSpan(self , rsthis: & QTextTableCell) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QTextTableCell10columnSpanEv()};
    let mut ret = unsafe {C_ZNK14QTextTableCell10columnSpanEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  QTextCharFormat QTextTableCell::format();
impl /*struct*/ QTextTableCell {
  pub fn format<RetType, T: QTextTableCell_format<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.format(self);
    // return 1;
  }
}

pub trait QTextTableCell_format<RetType> {
  fn format(self , rsthis: & QTextTableCell) -> RetType;
}

  // proto:  QTextCharFormat QTextTableCell::format();
impl<'a> /*trait*/ QTextTableCell_format<QTextCharFormat> for () {
  fn format(self , rsthis: & QTextTableCell) -> QTextCharFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QTextTableCell6formatEv()};
    let mut ret = unsafe {C_ZNK14QTextTableCell6formatEv(rsthis.qclsinst)};
    let mut ret1 = QTextCharFormat::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QTextTableCell::row();
impl /*struct*/ QTextTableCell {
  pub fn row<RetType, T: QTextTableCell_row<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.row(self);
    // return 1;
  }
}

pub trait QTextTableCell_row<RetType> {
  fn row(self , rsthis: & QTextTableCell) -> RetType;
}

  // proto:  int QTextTableCell::row();
impl<'a> /*trait*/ QTextTableCell_row<i32> for () {
  fn row(self , rsthis: & QTextTableCell) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QTextTableCell3rowEv()};
    let mut ret = unsafe {C_ZNK14QTextTableCell3rowEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  bool QTextTableCell::isValid();
impl /*struct*/ QTextTableCell {
  pub fn isValid<RetType, T: QTextTableCell_isValid<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QTextTableCell_isValid<RetType> {
  fn isValid(self , rsthis: & QTextTableCell) -> RetType;
}

  // proto:  bool QTextTableCell::isValid();
impl<'a> /*trait*/ QTextTableCell_isValid<i8> for () {
  fn isValid(self , rsthis: & QTextTableCell) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QTextTableCell7isValidEv()};
    let mut ret = unsafe {C_ZNK14QTextTableCell7isValidEv(rsthis.qclsinst)};
    return ret as i8; // 1
    // return 1;
  }
}

  // proto:  QTextCursor QTextTableCell::lastCursorPosition();
impl /*struct*/ QTextTableCell {
  pub fn lastCursorPosition<RetType, T: QTextTableCell_lastCursorPosition<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.lastCursorPosition(self);
    // return 1;
  }
}

pub trait QTextTableCell_lastCursorPosition<RetType> {
  fn lastCursorPosition(self , rsthis: & QTextTableCell) -> RetType;
}

  // proto:  QTextCursor QTextTableCell::lastCursorPosition();
impl<'a> /*trait*/ QTextTableCell_lastCursorPosition<QTextCursor> for () {
  fn lastCursorPosition(self , rsthis: & QTextTableCell) -> QTextCursor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QTextTableCell18lastCursorPositionEv()};
    let mut ret = unsafe {C_ZNK14QTextTableCell18lastCursorPositionEv(rsthis.qclsinst)};
    let mut ret1 = QTextCursor::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QTextTableCell::column();
impl /*struct*/ QTextTableCell {
  pub fn column<RetType, T: QTextTableCell_column<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.column(self);
    // return 1;
  }
}

pub trait QTextTableCell_column<RetType> {
  fn column(self , rsthis: & QTextTableCell) -> RetType;
}

  // proto:  int QTextTableCell::column();
impl<'a> /*trait*/ QTextTableCell_column<i32> for () {
  fn column(self , rsthis: & QTextTableCell) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QTextTableCell6columnEv()};
    let mut ret = unsafe {C_ZNK14QTextTableCell6columnEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  QTextCursor QTextTableCell::firstCursorPosition();
impl /*struct*/ QTextTableCell {
  pub fn firstCursorPosition<RetType, T: QTextTableCell_firstCursorPosition<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.firstCursorPosition(self);
    // return 1;
  }
}

pub trait QTextTableCell_firstCursorPosition<RetType> {
  fn firstCursorPosition(self , rsthis: & QTextTableCell) -> RetType;
}

  // proto:  QTextCursor QTextTableCell::firstCursorPosition();
impl<'a> /*trait*/ QTextTableCell_firstCursorPosition<QTextCursor> for () {
  fn firstCursorPosition(self , rsthis: & QTextTableCell) -> QTextCursor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK14QTextTableCell19firstCursorPositionEv()};
    let mut ret = unsafe {C_ZNK14QTextTableCell19firstCursorPositionEv(rsthis.qclsinst)};
    let mut ret1 = QTextCursor::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextTableCell::QTextTableCell();
impl<'a> /*trait*/ QTextTableCell_new for () {
  fn new(self) -> QTextTableCell {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN14QTextTableCellC2Ev()};
    let ctysz: c_int = unsafe{QTextTableCell_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN14QTextTableCellC2Ev()};
    let rsthis = QTextTableCell{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QTextTable {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QTextTable {
    return QTextTable{qbase: QTextFrame::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QTextTable {
  type Target = QTextFrame;

  fn deref(&self) -> &QTextFrame {
    return & self.qbase;
  }
}
impl AsRef<QTextFrame> for QTextTable {
  fn as_ref(& self) -> & QTextFrame {
    return & self.qbase;
  }
}
  // proto:  QTextTableCell QTextTable::cellAt(int row, int col);
impl /*struct*/ QTextTable {
  pub fn cellAt<RetType, T: QTextTable_cellAt<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.cellAt(self);
    // return 1;
  }
}

pub trait QTextTable_cellAt<RetType> {
  fn cellAt(self , rsthis: & QTextTable) -> RetType;
}

  // proto:  QTextTableCell QTextTable::cellAt(int row, int col);
impl<'a> /*trait*/ QTextTable_cellAt<QTextTableCell> for (i32, i32) {
  fn cellAt(self , rsthis: & QTextTable) -> QTextTableCell {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextTable6cellAtEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let mut ret = unsafe {C_ZNK10QTextTable6cellAtEii(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QTextTableCell::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  int QTextTable::rows();
impl /*struct*/ QTextTable {
  pub fn rows<RetType, T: QTextTable_rows<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rows(self);
    // return 1;
  }
}

pub trait QTextTable_rows<RetType> {
  fn rows(self , rsthis: & QTextTable) -> RetType;
}

  // proto:  int QTextTable::rows();
impl<'a> /*trait*/ QTextTable_rows<i32> for () {
  fn rows(self , rsthis: & QTextTable) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextTable4rowsEv()};
    let mut ret = unsafe {C_ZNK10QTextTable4rowsEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  void QTextTable::removeRows(int pos, int num);
impl /*struct*/ QTextTable {
  pub fn removeRows<RetType, T: QTextTable_removeRows<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.removeRows(self);
    // return 1;
  }
}

pub trait QTextTable_removeRows<RetType> {
  fn removeRows(self , rsthis: & QTextTable) -> RetType;
}

  // proto:  void QTextTable::removeRows(int pos, int num);
impl<'a> /*trait*/ QTextTable_removeRows<()> for (i32, i32) {
  fn removeRows(self , rsthis: & QTextTable) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextTable10removeRowsEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {C_ZN10QTextTable10removeRowsEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  int QTextTable::columns();
impl /*struct*/ QTextTable {
  pub fn columns<RetType, T: QTextTable_columns<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.columns(self);
    // return 1;
  }
}

pub trait QTextTable_columns<RetType> {
  fn columns(self , rsthis: & QTextTable) -> RetType;
}

  // proto:  int QTextTable::columns();
impl<'a> /*trait*/ QTextTable_columns<i32> for () {
  fn columns(self , rsthis: & QTextTable) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextTable7columnsEv()};
    let mut ret = unsafe {C_ZNK10QTextTable7columnsEv(rsthis.qclsinst)};
    return ret as i32; // 1
    // return 1;
  }
}

  // proto:  void QTextTable::appendRows(int count);
impl /*struct*/ QTextTable {
  pub fn appendRows<RetType, T: QTextTable_appendRows<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.appendRows(self);
    // return 1;
  }
}

pub trait QTextTable_appendRows<RetType> {
  fn appendRows(self , rsthis: & QTextTable) -> RetType;
}

  // proto:  void QTextTable::appendRows(int count);
impl<'a> /*trait*/ QTextTable_appendRows<()> for (i32) {
  fn appendRows(self , rsthis: & QTextTable) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextTable10appendRowsEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN10QTextTable10appendRowsEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextTable::resize(int rows, int cols);
impl /*struct*/ QTextTable {
  pub fn resize<RetType, T: QTextTable_resize<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.resize(self);
    // return 1;
  }
}

pub trait QTextTable_resize<RetType> {
  fn resize(self , rsthis: & QTextTable) -> RetType;
}

  // proto:  void QTextTable::resize(int rows, int cols);
impl<'a> /*trait*/ QTextTable_resize<()> for (i32, i32) {
  fn resize(self , rsthis: & QTextTable) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextTable6resizeEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {C_ZN10QTextTable6resizeEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  QTextTableCell QTextTable::cellAt(const QTextCursor & c);
impl<'a> /*trait*/ QTextTable_cellAt<QTextTableCell> for (&'a QTextCursor) {
  fn cellAt(self , rsthis: & QTextTable) -> QTextTableCell {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextTable6cellAtERK11QTextCursor()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK10QTextTable6cellAtERK11QTextCursor(rsthis.qclsinst, arg0)};
    let mut ret1 = QTextTableCell::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextTable::setFormat(const QTextTableFormat & format);
impl /*struct*/ QTextTable {
  pub fn setFormat<RetType, T: QTextTable_setFormat<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setFormat(self);
    // return 1;
  }
}

pub trait QTextTable_setFormat<RetType> {
  fn setFormat(self , rsthis: & QTextTable) -> RetType;
}

  // proto:  void QTextTable::setFormat(const QTextTableFormat & format);
impl<'a> /*trait*/ QTextTable_setFormat<()> for (&'a QTextTableFormat) {
  fn setFormat(self , rsthis: & QTextTable) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextTable9setFormatERK16QTextTableFormat()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN10QTextTable9setFormatERK16QTextTableFormat(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextTable::insertColumns(int pos, int num);
impl /*struct*/ QTextTable {
  pub fn insertColumns<RetType, T: QTextTable_insertColumns<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.insertColumns(self);
    // return 1;
  }
}

pub trait QTextTable_insertColumns<RetType> {
  fn insertColumns(self , rsthis: & QTextTable) -> RetType;
}

  // proto:  void QTextTable::insertColumns(int pos, int num);
impl<'a> /*trait*/ QTextTable_insertColumns<()> for (i32, i32) {
  fn insertColumns(self , rsthis: & QTextTable) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextTable13insertColumnsEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {C_ZN10QTextTable13insertColumnsEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QTextTable::splitCell(int row, int col, int numRows, int numCols);
impl /*struct*/ QTextTable {
  pub fn splitCell<RetType, T: QTextTable_splitCell<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.splitCell(self);
    // return 1;
  }
}

pub trait QTextTable_splitCell<RetType> {
  fn splitCell(self , rsthis: & QTextTable) -> RetType;
}

  // proto:  void QTextTable::splitCell(int row, int col, int numRows, int numCols);
impl<'a> /*trait*/ QTextTable_splitCell<()> for (i32, i32, i32, i32) {
  fn splitCell(self , rsthis: & QTextTable) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextTable9splitCellEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
     unsafe {C_ZN10QTextTable9splitCellEiiii(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  void QTextTable::mergeCells(int row, int col, int numRows, int numCols);
impl /*struct*/ QTextTable {
  pub fn mergeCells<RetType, T: QTextTable_mergeCells<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mergeCells(self);
    // return 1;
  }
}

pub trait QTextTable_mergeCells<RetType> {
  fn mergeCells(self , rsthis: & QTextTable) -> RetType;
}

  // proto:  void QTextTable::mergeCells(int row, int col, int numRows, int numCols);
impl<'a> /*trait*/ QTextTable_mergeCells<()> for (i32, i32, i32, i32) {
  fn mergeCells(self , rsthis: & QTextTable) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextTable10mergeCellsEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
     unsafe {C_ZN10QTextTable10mergeCellsEiiii(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

  // proto:  void QTextTable::insertRows(int pos, int num);
impl /*struct*/ QTextTable {
  pub fn insertRows<RetType, T: QTextTable_insertRows<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.insertRows(self);
    // return 1;
  }
}

pub trait QTextTable_insertRows<RetType> {
  fn insertRows(self , rsthis: & QTextTable) -> RetType;
}

  // proto:  void QTextTable::insertRows(int pos, int num);
impl<'a> /*trait*/ QTextTable_insertRows<()> for (i32, i32) {
  fn insertRows(self , rsthis: & QTextTable) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextTable10insertRowsEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {C_ZN10QTextTable10insertRowsEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QTextTable::~QTextTable();
impl /*struct*/ QTextTable {
  pub fn free<RetType, T: QTextTable_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QTextTable_free<RetType> {
  fn free(self , rsthis: & QTextTable) -> RetType;
}

  // proto:  void QTextTable::~QTextTable();
impl<'a> /*trait*/ QTextTable_free<()> for () {
  fn free(self , rsthis: & QTextTable) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextTableD2Ev()};
     unsafe {C_ZN10QTextTableD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTextTable::QTextTable(QTextDocument * doc);
impl /*struct*/ QTextTable {
  pub fn new<T: QTextTable_new>(value: T) -> QTextTable {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QTextTable_new {
  fn new(self) -> QTextTable;
}

  // proto:  void QTextTable::QTextTable(QTextDocument * doc);
impl<'a> /*trait*/ QTextTable_new for (&'a QTextDocument) {
  fn new(self) -> QTextTable {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextTableC2EP13QTextDocument()};
    let ctysz: c_int = unsafe{QTextTable_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    let qthis: u64 = unsafe {C_ZN10QTextTableC2EP13QTextDocument(arg0)};
    let rsthis = QTextTable{qbase: QTextFrame::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  QTextTableCell QTextTable::cellAt(int position);
impl<'a> /*trait*/ QTextTable_cellAt<QTextTableCell> for (i32) {
  fn cellAt(self , rsthis: & QTextTable) -> QTextTableCell {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextTable6cellAtEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {C_ZNK10QTextTable6cellAtEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QTextTableCell::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QTextCursor QTextTable::rowStart(const QTextCursor & c);
impl /*struct*/ QTextTable {
  pub fn rowStart<RetType, T: QTextTable_rowStart<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rowStart(self);
    // return 1;
  }
}

pub trait QTextTable_rowStart<RetType> {
  fn rowStart(self , rsthis: & QTextTable) -> RetType;
}

  // proto:  QTextCursor QTextTable::rowStart(const QTextCursor & c);
impl<'a> /*trait*/ QTextTable_rowStart<QTextCursor> for (&'a QTextCursor) {
  fn rowStart(self , rsthis: & QTextTable) -> QTextCursor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextTable8rowStartERK11QTextCursor()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK10QTextTable8rowStartERK11QTextCursor(rsthis.qclsinst, arg0)};
    let mut ret1 = QTextCursor::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QTextTableFormat QTextTable::format();
impl /*struct*/ QTextTable {
  pub fn format<RetType, T: QTextTable_format<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.format(self);
    // return 1;
  }
}

pub trait QTextTable_format<RetType> {
  fn format(self , rsthis: & QTextTable) -> RetType;
}

  // proto:  QTextTableFormat QTextTable::format();
impl<'a> /*trait*/ QTextTable_format<QTextTableFormat> for () {
  fn format(self , rsthis: & QTextTable) -> QTextTableFormat {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextTable6formatEv()};
    let mut ret = unsafe {C_ZNK10QTextTable6formatEv(rsthis.qclsinst)};
    let mut ret1 = QTextTableFormat::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QTextCursor QTextTable::rowEnd(const QTextCursor & c);
impl /*struct*/ QTextTable {
  pub fn rowEnd<RetType, T: QTextTable_rowEnd<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.rowEnd(self);
    // return 1;
  }
}

pub trait QTextTable_rowEnd<RetType> {
  fn rowEnd(self , rsthis: & QTextTable) -> RetType;
}

  // proto:  QTextCursor QTextTable::rowEnd(const QTextCursor & c);
impl<'a> /*trait*/ QTextTable_rowEnd<QTextCursor> for (&'a QTextCursor) {
  fn rowEnd(self , rsthis: & QTextTable) -> QTextCursor {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextTable6rowEndERK11QTextCursor()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK10QTextTable6rowEndERK11QTextCursor(rsthis.qclsinst, arg0)};
    let mut ret1 = QTextCursor::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  const QMetaObject * QTextTable::metaObject();
impl /*struct*/ QTextTable {
  pub fn metaObject<RetType, T: QTextTable_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QTextTable_metaObject<RetType> {
  fn metaObject(self , rsthis: & QTextTable) -> RetType;
}

  // proto:  const QMetaObject * QTextTable::metaObject();
impl<'a> /*trait*/ QTextTable_metaObject<QMetaObject> for () {
  fn metaObject(self , rsthis: & QTextTable) -> QMetaObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK10QTextTable10metaObjectEv()};
    let mut ret = unsafe {C_ZNK10QTextTable10metaObjectEv(rsthis.qclsinst)};
    let mut ret1 = QMetaObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextTable::removeColumns(int pos, int num);
impl /*struct*/ QTextTable {
  pub fn removeColumns<RetType, T: QTextTable_removeColumns<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.removeColumns(self);
    // return 1;
  }
}

pub trait QTextTable_removeColumns<RetType> {
  fn removeColumns(self , rsthis: & QTextTable) -> RetType;
}

  // proto:  void QTextTable::removeColumns(int pos, int num);
impl<'a> /*trait*/ QTextTable_removeColumns<()> for (i32, i32) {
  fn removeColumns(self , rsthis: & QTextTable) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextTable13removeColumnsEii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
     unsafe {C_ZN10QTextTable13removeColumnsEii(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QTextTable::appendColumns(int count);
impl /*struct*/ QTextTable {
  pub fn appendColumns<RetType, T: QTextTable_appendColumns<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.appendColumns(self);
    // return 1;
  }
}

pub trait QTextTable_appendColumns<RetType> {
  fn appendColumns(self , rsthis: & QTextTable) -> RetType;
}

  // proto:  void QTextTable::appendColumns(int count);
impl<'a> /*trait*/ QTextTable_appendColumns<()> for (i32) {
  fn appendColumns(self , rsthis: & QTextTable) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextTable13appendColumnsEi()};
    let arg0 = self  as c_int;
     unsafe {C_ZN10QTextTable13appendColumnsEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextTable::mergeCells(const QTextCursor & cursor);
impl<'a> /*trait*/ QTextTable_mergeCells<()> for (&'a QTextCursor) {
  fn mergeCells(self , rsthis: & QTextTable) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN10QTextTable10mergeCellsERK11QTextCursor()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {C_ZN10QTextTable10mergeCellsERK11QTextCursor(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// <= body block end

