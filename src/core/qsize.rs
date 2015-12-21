// auto generated, do not modify.
// created: Mon Dec 21 22:54:38 2015
// src-file: /QtCore/qsize.h
// dst-file: /src/core/qsize.rs
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
// use super::qsize::QSize; // 773
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  QSize QSize::boundedTo(const QSize & );
  fn _ZNK5QSize9boundedToERKS_(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QSize::isValid();
  fn _ZNK5QSize7isValidEv(qthis: *mut c_void) -> c_char;
  // proto:  bool QSize::isNull();
  fn _ZNK5QSize6isNullEv(qthis: *mut c_void) -> c_char;
  // proto:  void QSize::QSize();
  fn _ZN5QSizeC1Ev(qthis: *mut c_void);
  // proto:  QSize QSize::expandedTo(const QSize & );
  fn _ZNK5QSize10expandedToERKS_(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  int QSize::height();
  fn _ZNK5QSize6heightEv(qthis: *mut c_void) -> c_int;
  // proto:  int & QSize::rheight();
  fn _ZN5QSize7rheightEv(qthis: *mut c_void);
  // proto:  void QSize::QSize(int w, int h);
  fn _ZN5QSizeC1Eii(qthis: *mut c_void, arg0: c_int, arg1: c_int);
  // proto:  int QSize::width();
  fn _ZNK5QSize5widthEv(qthis: *mut c_void) -> c_int;
  // proto:  QSize QSize::transposed();
  fn _ZNK5QSize10transposedEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  int & QSize::rwidth();
  fn _ZN5QSize6rwidthEv(qthis: *mut c_void);
  // proto:  void QSize::setHeight(int h);
  fn _ZN5QSize9setHeightEi(qthis: *mut c_void, arg0: c_int);
  // proto:  bool QSize::isEmpty();
  fn _ZNK5QSize7isEmptyEv(qthis: *mut c_void) -> c_char;
  // proto:  void QSize::setWidth(int w);
  fn _ZN5QSize8setWidthEi(qthis: *mut c_void, arg0: c_int);
  // proto:  void QSize::transpose();
  fn _ZN5QSize9transposeEv(qthis: *mut c_void);
  // proto:  qreal & QSizeF::rheight();
  fn _ZN6QSizeF7rheightEv(qthis: *mut c_void);
  // proto:  qreal & QSizeF::rwidth();
  fn _ZN6QSizeF6rwidthEv(qthis: *mut c_void);
  // proto:  QSizeF QSizeF::transposed();
  fn _ZNK6QSizeF10transposedEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QSizeF::isValid();
  fn _ZNK6QSizeF7isValidEv(qthis: *mut c_void) -> c_char;
  // proto:  void QSizeF::setHeight(qreal h);
  fn _ZN6QSizeF9setHeightEd(qthis: *mut c_void, arg0: c_double);
  // proto:  void QSizeF::QSizeF();
  fn _ZN6QSizeFC1Ev(qthis: *mut c_void);
  // proto:  qreal QSizeF::width();
  fn _ZNK6QSizeF5widthEv(qthis: *mut c_void) -> c_double;
  // proto:  bool QSizeF::isNull();
  fn _ZNK6QSizeF6isNullEv(qthis: *mut c_void) -> c_char;
  // proto:  QSizeF QSizeF::boundedTo(const QSizeF & );
  fn _ZNK6QSizeF9boundedToERKS_(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  qreal QSizeF::height();
  fn _ZNK6QSizeF6heightEv(qthis: *mut c_void) -> c_double;
  // proto:  void QSizeF::transpose();
  fn _ZN6QSizeF9transposeEv(qthis: *mut c_void);
  // proto:  void QSizeF::QSizeF(const QSize & sz);
  fn _ZN6QSizeFC1ERK5QSize(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QSizeF QSizeF::expandedTo(const QSizeF & );
  fn _ZNK6QSizeF10expandedToERKS_(qthis: *mut c_void, arg0: *mut c_void) -> *mut c_void;
  // proto:  bool QSizeF::isEmpty();
  fn _ZNK6QSizeF7isEmptyEv(qthis: *mut c_void) -> c_char;
  // proto:  void QSizeF::setWidth(qreal w);
  fn _ZN6QSizeF8setWidthEd(qthis: *mut c_void, arg0: c_double);
  // proto:  QSize QSizeF::toSize();
  fn _ZNK6QSizeF6toSizeEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QSizeF::QSizeF(qreal w, qreal h);
  fn _ZN6QSizeFC1Edd(qthis: *mut c_void, arg0: c_double, arg1: c_double);
} // <= ext block end

// body block begin =>
// class sizeof(QSize)=8
pub struct QSize {
  pub qclsinst: *mut c_void,
}

// class sizeof(QSizeF)=16
pub struct QSizeF {
  pub qclsinst: *mut c_void,
}

  // proto:  QSize QSize::boundedTo(const QSize & );
impl /*struct*/ QSize {
  pub fn boundedTo<RetType, T: QSize_boundedTo<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.boundedTo(self);
    // return 1;
  }
}

pub trait QSize_boundedTo<RetType> {
  fn boundedTo(self , rsthis: &mut QSize) -> RetType;
}

  // proto:  QSize QSize::boundedTo(const QSize & );
impl<'a> /*trait*/ QSize_boundedTo<QSize> for (QSize) {
  fn boundedTo(self , rsthis: &mut QSize) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QSize9boundedToERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK5QSize9boundedToERKS_(rsthis.qclsinst, arg0)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  bool QSize::isValid();
impl /*struct*/ QSize {
  pub fn isValid<RetType, T: QSize_isValid<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QSize_isValid<RetType> {
  fn isValid(self , rsthis: &mut QSize) -> RetType;
}

  // proto:  bool QSize::isValid();
impl<'a> /*trait*/ QSize_isValid<i8> for () {
  fn isValid(self , rsthis: &mut QSize) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QSize7isValidEv()};
    let mut ret = unsafe {_ZNK5QSize7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QSize::isNull();
impl /*struct*/ QSize {
  pub fn isNull<RetType, T: QSize_isNull<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isNull(self);
    // return 1;
  }
}

pub trait QSize_isNull<RetType> {
  fn isNull(self , rsthis: &mut QSize) -> RetType;
}

  // proto:  bool QSize::isNull();
impl<'a> /*trait*/ QSize_isNull<i8> for () {
  fn isNull(self , rsthis: &mut QSize) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QSize6isNullEv()};
    let mut ret = unsafe {_ZNK5QSize6isNullEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QSize::QSize();
impl /*struct*/ QSize {
  pub fn NewQSize<T: QSize_NewQSize>(value: T) -> QSize {
    let rsthis = value.NewQSize();
    return rsthis;
    // return 1;
  }
}

pub trait QSize_NewQSize {
  fn NewQSize(self) -> QSize;
}

  // proto:  void QSize::QSize();
impl<'a> /*trait*/ QSize_NewQSize for () {
  fn NewQSize(self) -> QSize {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QSizeC1Ev()};
    unsafe {_ZN5QSizeC1Ev(qthis)};
    let rsthis = QSize{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QSize QSize::expandedTo(const QSize & );
impl /*struct*/ QSize {
  pub fn expandedTo<RetType, T: QSize_expandedTo<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.expandedTo(self);
    // return 1;
  }
}

pub trait QSize_expandedTo<RetType> {
  fn expandedTo(self , rsthis: &mut QSize) -> RetType;
}

  // proto:  QSize QSize::expandedTo(const QSize & );
impl<'a> /*trait*/ QSize_expandedTo<QSize> for (QSize) {
  fn expandedTo(self , rsthis: &mut QSize) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QSize10expandedToERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK5QSize10expandedToERKS_(rsthis.qclsinst, arg0)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  int QSize::height();
impl /*struct*/ QSize {
  pub fn height<RetType, T: QSize_height<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.height(self);
    // return 1;
  }
}

pub trait QSize_height<RetType> {
  fn height(self , rsthis: &mut QSize) -> RetType;
}

  // proto:  int QSize::height();
impl<'a> /*trait*/ QSize_height<i32> for () {
  fn height(self , rsthis: &mut QSize) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QSize6heightEv()};
    let mut ret = unsafe {_ZNK5QSize6heightEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  int & QSize::rheight();
impl /*struct*/ QSize {
  pub fn rheight<RetType, T: QSize_rheight<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.rheight(self);
    // return 1;
  }
}

pub trait QSize_rheight<RetType> {
  fn rheight(self , rsthis: &mut QSize) -> RetType;
}

  // proto:  int & QSize::rheight();
impl<'a> /*trait*/ QSize_rheight<()> for () {
  fn rheight(self , rsthis: &mut QSize) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QSize7rheightEv()};
     unsafe {_ZN5QSize7rheightEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QSize::QSize(int w, int h);
impl<'a> /*trait*/ QSize_NewQSize for (i32, i32) {
  fn NewQSize(self) -> QSize {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QSizeC1Eii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN5QSizeC1Eii(qthis, arg0, arg1)};
    let rsthis = QSize{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QSize::width();
impl /*struct*/ QSize {
  pub fn width<RetType, T: QSize_width<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.width(self);
    // return 1;
  }
}

pub trait QSize_width<RetType> {
  fn width(self , rsthis: &mut QSize) -> RetType;
}

  // proto:  int QSize::width();
impl<'a> /*trait*/ QSize_width<i32> for () {
  fn width(self , rsthis: &mut QSize) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QSize5widthEv()};
    let mut ret = unsafe {_ZNK5QSize5widthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  QSize QSize::transposed();
impl /*struct*/ QSize {
  pub fn transposed<RetType, T: QSize_transposed<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.transposed(self);
    // return 1;
  }
}

pub trait QSize_transposed<RetType> {
  fn transposed(self , rsthis: &mut QSize) -> RetType;
}

  // proto:  QSize QSize::transposed();
impl<'a> /*trait*/ QSize_transposed<QSize> for () {
  fn transposed(self , rsthis: &mut QSize) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QSize10transposedEv()};
    let mut ret = unsafe {_ZNK5QSize10transposedEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  int & QSize::rwidth();
impl /*struct*/ QSize {
  pub fn rwidth<RetType, T: QSize_rwidth<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.rwidth(self);
    // return 1;
  }
}

pub trait QSize_rwidth<RetType> {
  fn rwidth(self , rsthis: &mut QSize) -> RetType;
}

  // proto:  int & QSize::rwidth();
impl<'a> /*trait*/ QSize_rwidth<()> for () {
  fn rwidth(self , rsthis: &mut QSize) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QSize6rwidthEv()};
     unsafe {_ZN5QSize6rwidthEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QSize::setHeight(int h);
impl /*struct*/ QSize {
  pub fn setHeight<RetType, T: QSize_setHeight<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setHeight(self);
    // return 1;
  }
}

pub trait QSize_setHeight<RetType> {
  fn setHeight(self , rsthis: &mut QSize) -> RetType;
}

  // proto:  void QSize::setHeight(int h);
impl<'a> /*trait*/ QSize_setHeight<()> for (i32) {
  fn setHeight(self , rsthis: &mut QSize) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QSize9setHeightEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN5QSize9setHeightEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QSize::isEmpty();
impl /*struct*/ QSize {
  pub fn isEmpty<RetType, T: QSize_isEmpty<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isEmpty(self);
    // return 1;
  }
}

pub trait QSize_isEmpty<RetType> {
  fn isEmpty(self , rsthis: &mut QSize) -> RetType;
}

  // proto:  bool QSize::isEmpty();
impl<'a> /*trait*/ QSize_isEmpty<i8> for () {
  fn isEmpty(self , rsthis: &mut QSize) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QSize7isEmptyEv()};
    let mut ret = unsafe {_ZNK5QSize7isEmptyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QSize::setWidth(int w);
impl /*struct*/ QSize {
  pub fn setWidth<RetType, T: QSize_setWidth<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setWidth(self);
    // return 1;
  }
}

pub trait QSize_setWidth<RetType> {
  fn setWidth(self , rsthis: &mut QSize) -> RetType;
}

  // proto:  void QSize::setWidth(int w);
impl<'a> /*trait*/ QSize_setWidth<()> for (i32) {
  fn setWidth(self , rsthis: &mut QSize) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QSize8setWidthEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN5QSize8setWidthEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QSize::transpose();
impl /*struct*/ QSize {
  pub fn transpose<RetType, T: QSize_transpose<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.transpose(self);
    // return 1;
  }
}

pub trait QSize_transpose<RetType> {
  fn transpose(self , rsthis: &mut QSize) -> RetType;
}

  // proto:  void QSize::transpose();
impl<'a> /*trait*/ QSize_transpose<()> for () {
  fn transpose(self , rsthis: &mut QSize) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QSize9transposeEv()};
     unsafe {_ZN5QSize9transposeEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  qreal & QSizeF::rheight();
impl /*struct*/ QSizeF {
  pub fn rheight<RetType, T: QSizeF_rheight<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.rheight(self);
    // return 1;
  }
}

pub trait QSizeF_rheight<RetType> {
  fn rheight(self , rsthis: &mut QSizeF) -> RetType;
}

  // proto:  qreal & QSizeF::rheight();
impl<'a> /*trait*/ QSizeF_rheight<()> for () {
  fn rheight(self , rsthis: &mut QSizeF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QSizeF7rheightEv()};
     unsafe {_ZN6QSizeF7rheightEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  qreal & QSizeF::rwidth();
impl /*struct*/ QSizeF {
  pub fn rwidth<RetType, T: QSizeF_rwidth<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.rwidth(self);
    // return 1;
  }
}

pub trait QSizeF_rwidth<RetType> {
  fn rwidth(self , rsthis: &mut QSizeF) -> RetType;
}

  // proto:  qreal & QSizeF::rwidth();
impl<'a> /*trait*/ QSizeF_rwidth<()> for () {
  fn rwidth(self , rsthis: &mut QSizeF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QSizeF6rwidthEv()};
     unsafe {_ZN6QSizeF6rwidthEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QSizeF QSizeF::transposed();
impl /*struct*/ QSizeF {
  pub fn transposed<RetType, T: QSizeF_transposed<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.transposed(self);
    // return 1;
  }
}

pub trait QSizeF_transposed<RetType> {
  fn transposed(self , rsthis: &mut QSizeF) -> RetType;
}

  // proto:  QSizeF QSizeF::transposed();
impl<'a> /*trait*/ QSizeF_transposed<QSizeF> for () {
  fn transposed(self , rsthis: &mut QSizeF) -> QSizeF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QSizeF10transposedEv()};
    let mut ret = unsafe {_ZNK6QSizeF10transposedEv(rsthis.qclsinst)};
    let mut ret1 = QSizeF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  bool QSizeF::isValid();
impl /*struct*/ QSizeF {
  pub fn isValid<RetType, T: QSizeF_isValid<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isValid(self);
    // return 1;
  }
}

pub trait QSizeF_isValid<RetType> {
  fn isValid(self , rsthis: &mut QSizeF) -> RetType;
}

  // proto:  bool QSizeF::isValid();
impl<'a> /*trait*/ QSizeF_isValid<i8> for () {
  fn isValid(self , rsthis: &mut QSizeF) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QSizeF7isValidEv()};
    let mut ret = unsafe {_ZNK6QSizeF7isValidEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QSizeF::setHeight(qreal h);
impl /*struct*/ QSizeF {
  pub fn setHeight<RetType, T: QSizeF_setHeight<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setHeight(self);
    // return 1;
  }
}

pub trait QSizeF_setHeight<RetType> {
  fn setHeight(self , rsthis: &mut QSizeF) -> RetType;
}

  // proto:  void QSizeF::setHeight(qreal h);
impl<'a> /*trait*/ QSizeF_setHeight<()> for (f64) {
  fn setHeight(self , rsthis: &mut QSizeF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QSizeF9setHeightEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN6QSizeF9setHeightEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QSizeF::QSizeF();
impl /*struct*/ QSizeF {
  pub fn NewQSizeF<T: QSizeF_NewQSizeF>(value: T) -> QSizeF {
    let rsthis = value.NewQSizeF();
    return rsthis;
    // return 1;
  }
}

pub trait QSizeF_NewQSizeF {
  fn NewQSizeF(self) -> QSizeF;
}

  // proto:  void QSizeF::QSizeF();
impl<'a> /*trait*/ QSizeF_NewQSizeF for () {
  fn NewQSizeF(self) -> QSizeF {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QSizeFC1Ev()};
    unsafe {_ZN6QSizeFC1Ev(qthis)};
    let rsthis = QSizeF{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  qreal QSizeF::width();
impl /*struct*/ QSizeF {
  pub fn width<RetType, T: QSizeF_width<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.width(self);
    // return 1;
  }
}

pub trait QSizeF_width<RetType> {
  fn width(self , rsthis: &mut QSizeF) -> RetType;
}

  // proto:  qreal QSizeF::width();
impl<'a> /*trait*/ QSizeF_width<f64> for () {
  fn width(self , rsthis: &mut QSizeF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QSizeF5widthEv()};
    let mut ret = unsafe {_ZNK6QSizeF5widthEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  bool QSizeF::isNull();
impl /*struct*/ QSizeF {
  pub fn isNull<RetType, T: QSizeF_isNull<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isNull(self);
    // return 1;
  }
}

pub trait QSizeF_isNull<RetType> {
  fn isNull(self , rsthis: &mut QSizeF) -> RetType;
}

  // proto:  bool QSizeF::isNull();
impl<'a> /*trait*/ QSizeF_isNull<i8> for () {
  fn isNull(self , rsthis: &mut QSizeF) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QSizeF6isNullEv()};
    let mut ret = unsafe {_ZNK6QSizeF6isNullEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  QSizeF QSizeF::boundedTo(const QSizeF & );
impl /*struct*/ QSizeF {
  pub fn boundedTo<RetType, T: QSizeF_boundedTo<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.boundedTo(self);
    // return 1;
  }
}

pub trait QSizeF_boundedTo<RetType> {
  fn boundedTo(self , rsthis: &mut QSizeF) -> RetType;
}

  // proto:  QSizeF QSizeF::boundedTo(const QSizeF & );
impl<'a> /*trait*/ QSizeF_boundedTo<QSizeF> for (QSizeF) {
  fn boundedTo(self , rsthis: &mut QSizeF) -> QSizeF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QSizeF9boundedToERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK6QSizeF9boundedToERKS_(rsthis.qclsinst, arg0)};
    let mut ret1 = QSizeF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  qreal QSizeF::height();
impl /*struct*/ QSizeF {
  pub fn height<RetType, T: QSizeF_height<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.height(self);
    // return 1;
  }
}

pub trait QSizeF_height<RetType> {
  fn height(self , rsthis: &mut QSizeF) -> RetType;
}

  // proto:  qreal QSizeF::height();
impl<'a> /*trait*/ QSizeF_height<f64> for () {
  fn height(self , rsthis: &mut QSizeF) -> f64 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QSizeF6heightEv()};
    let mut ret = unsafe {_ZNK6QSizeF6heightEv(rsthis.qclsinst)};
    return ret as f64;
    // return 1;
  }
}

  // proto:  void QSizeF::transpose();
impl /*struct*/ QSizeF {
  pub fn transpose<RetType, T: QSizeF_transpose<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.transpose(self);
    // return 1;
  }
}

pub trait QSizeF_transpose<RetType> {
  fn transpose(self , rsthis: &mut QSizeF) -> RetType;
}

  // proto:  void QSizeF::transpose();
impl<'a> /*trait*/ QSizeF_transpose<()> for () {
  fn transpose(self , rsthis: &mut QSizeF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QSizeF9transposeEv()};
     unsafe {_ZN6QSizeF9transposeEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QSizeF::QSizeF(const QSize & sz);
impl<'a> /*trait*/ QSizeF_NewQSizeF for (QSize) {
  fn NewQSizeF(self) -> QSizeF {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QSizeFC1ERK5QSize()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN6QSizeFC1ERK5QSize(qthis, arg0)};
    let rsthis = QSizeF{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QSizeF QSizeF::expandedTo(const QSizeF & );
impl /*struct*/ QSizeF {
  pub fn expandedTo<RetType, T: QSizeF_expandedTo<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.expandedTo(self);
    // return 1;
  }
}

pub trait QSizeF_expandedTo<RetType> {
  fn expandedTo(self , rsthis: &mut QSizeF) -> RetType;
}

  // proto:  QSizeF QSizeF::expandedTo(const QSizeF & );
impl<'a> /*trait*/ QSizeF_expandedTo<QSizeF> for (QSizeF) {
  fn expandedTo(self , rsthis: &mut QSizeF) -> QSizeF {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QSizeF10expandedToERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK6QSizeF10expandedToERKS_(rsthis.qclsinst, arg0)};
    let mut ret1 = QSizeF{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  bool QSizeF::isEmpty();
impl /*struct*/ QSizeF {
  pub fn isEmpty<RetType, T: QSizeF_isEmpty<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isEmpty(self);
    // return 1;
  }
}

pub trait QSizeF_isEmpty<RetType> {
  fn isEmpty(self , rsthis: &mut QSizeF) -> RetType;
}

  // proto:  bool QSizeF::isEmpty();
impl<'a> /*trait*/ QSizeF_isEmpty<i8> for () {
  fn isEmpty(self , rsthis: &mut QSizeF) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QSizeF7isEmptyEv()};
    let mut ret = unsafe {_ZNK6QSizeF7isEmptyEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QSizeF::setWidth(qreal w);
impl /*struct*/ QSizeF {
  pub fn setWidth<RetType, T: QSizeF_setWidth<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setWidth(self);
    // return 1;
  }
}

pub trait QSizeF_setWidth<RetType> {
  fn setWidth(self , rsthis: &mut QSizeF) -> RetType;
}

  // proto:  void QSizeF::setWidth(qreal w);
impl<'a> /*trait*/ QSizeF_setWidth<()> for (f64) {
  fn setWidth(self , rsthis: &mut QSizeF) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QSizeF8setWidthEd()};
    let arg0 = self  as c_double;
     unsafe {_ZN6QSizeF8setWidthEd(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QSize QSizeF::toSize();
impl /*struct*/ QSizeF {
  pub fn toSize<RetType, T: QSizeF_toSize<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.toSize(self);
    // return 1;
  }
}

pub trait QSizeF_toSize<RetType> {
  fn toSize(self , rsthis: &mut QSizeF) -> RetType;
}

  // proto:  QSize QSizeF::toSize();
impl<'a> /*trait*/ QSizeF_toSize<QSize> for () {
  fn toSize(self , rsthis: &mut QSizeF) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QSizeF6toSizeEv()};
    let mut ret = unsafe {_ZNK6QSizeF6toSizeEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QSizeF::QSizeF(qreal w, qreal h);
impl<'a> /*trait*/ QSizeF_NewQSizeF for (f64, f64) {
  fn NewQSizeF(self) -> QSizeF {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QSizeFC1Edd()};
    let arg0 = self.0  as c_double;
    let arg1 = self.1  as c_double;
    unsafe {_ZN6QSizeFC1Edd(qthis, arg0, arg1)};
    let rsthis = QSizeF{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// <= body block end

