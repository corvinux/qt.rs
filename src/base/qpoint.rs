// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  int & QPoint::ry();
  fn _ZN6QPoint2ryEv(qthis: *mut c_void) ;
  // proto: static int QPoint::dotProduct(const QPoint & p1, const QPoint & p2);
  fn _ZN6QPoint10dotProductERKS_S1_(arg0: *mut c_void, arg1: *mut c_void) -> c_int;
  // proto:  int QPoint::x();
  fn _ZNK6QPoint1xEv(qthis: *mut c_void) ;
  // proto:  void QPoint::NewQPoint(int xpos, int ypos);
  fn _ZN6QPointC1Eii(qthis: *mut c_void, arg0: c_int, arg1: c_int) ;
  // proto:  int QPoint::y();
  fn _ZNK6QPoint1yEv(qthis: *mut c_void) ;
  // proto:  void QPoint::setX(int x);
  fn _ZN6QPoint4setXEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  bool QPoint::isNull();
  fn _ZNK6QPoint6isNullEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QPoint::NewQPoint();
  fn _ZN6QPointC1Ev(qthis: *mut c_void) ;
  // proto:  void QPoint::setY(int y);
  fn _ZN6QPoint4setYEi(qthis: *mut c_void, arg0: c_int) ;
  // proto:  int & QPoint::rx();
  fn _ZN6QPoint2rxEv(qthis: *mut c_void) ;
  // proto:  int QPoint::manhattanLength();
  fn _ZNK6QPoint15manhattanLengthEv(qthis: *mut c_void) -> c_int;
}

// body block begin
// class sizeof(QPoint)=8
pub struct QPoint {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QPoint {
  pub fn ry<T: QPoint_ry>(&mut self, value: T)  {
     value.ry(self);
    // return 1;
  }
}

pub trait QPoint_ry {
  fn ry(self, rsthis: &mut QPoint) ;
}

// proto:  int & QPoint::ry();
impl<'a> /*trait*/ QPoint_ry for () {
  fn ry(self, rsthis: &mut QPoint)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QPoint2ryEv()};
     unsafe {_ZN6QPoint2ryEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QPoint {
  pub fn dotProduct<T: QPoint_dotProduct>(&mut self, value: T) -> i32 {
    return value.dotProduct(self);
    // return 1;
  }
}

pub trait QPoint_dotProduct {
  fn dotProduct(self, rsthis: &mut QPoint) -> i32;
}

// proto: static int QPoint::dotProduct(const QPoint & p1, const QPoint & p2);
impl<'a> /*trait*/ QPoint_dotProduct for (&'a  QPoint, &'a  QPoint) {
  fn dotProduct(self, rsthis: &mut QPoint) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QPoint10dotProductERKS_S1_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN6QPoint10dotProductERKS_S1_(arg0, arg1)};
    return ret as i32;
    // return 1;
  }
}

impl /*struct*/ QPoint {
  pub fn x<T: QPoint_x>(&mut self, value: T)  {
     value.x(self);
    // return 1;
  }
}

pub trait QPoint_x {
  fn x(self, rsthis: &mut QPoint) ;
}

// proto:  int QPoint::x();
impl<'a> /*trait*/ QPoint_x for () {
  fn x(self, rsthis: &mut QPoint)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QPoint1xEv()};
     unsafe {_ZNK6QPoint1xEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QPoint {
  pub fn NewQPoint<T: QPoint_NewQPoint>(value: T) -> QPoint {
    let rsthis = value.NewQPoint();
    return rsthis;
    // return 1;
  }
}

pub trait QPoint_NewQPoint {
  fn NewQPoint(self) -> QPoint;
}

// proto: void QPoint::NewQPoint(int xpos, int ypos);
impl<'a> /*trait*/ QPoint_NewQPoint for (i32, i32) {
  fn NewQPoint(self) -> QPoint {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QPointC1Eii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    unsafe {_ZN6QPointC1Eii(qthis, arg0, arg1)};
    let rsthis = QPoint{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPoint {
  pub fn y<T: QPoint_y>(&mut self, value: T)  {
     value.y(self);
    // return 1;
  }
}

pub trait QPoint_y {
  fn y(self, rsthis: &mut QPoint) ;
}

// proto:  int QPoint::y();
impl<'a> /*trait*/ QPoint_y for () {
  fn y(self, rsthis: &mut QPoint)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QPoint1yEv()};
     unsafe {_ZNK6QPoint1yEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QPoint {
  pub fn setX<T: QPoint_setX>(&mut self, value: T)  {
     value.setX(self);
    // return 1;
  }
}

pub trait QPoint_setX {
  fn setX(self, rsthis: &mut QPoint) ;
}

// proto:  void QPoint::setX(int x);
impl<'a> /*trait*/ QPoint_setX for (i32) {
  fn setX(self, rsthis: &mut QPoint)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QPoint4setXEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN6QPoint4setXEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPoint {
  pub fn isNull<T: QPoint_isNull>(&mut self, value: T) -> i8 {
    return value.isNull(self);
    // return 1;
  }
}

pub trait QPoint_isNull {
  fn isNull(self, rsthis: &mut QPoint) -> i8;
}

// proto:  bool QPoint::isNull();
impl<'a> /*trait*/ QPoint_isNull for () {
  fn isNull(self, rsthis: &mut QPoint) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QPoint6isNullEv()};
    let mut ret = unsafe {_ZNK6QPoint6isNullEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto: void QPoint::NewQPoint();
impl<'a> /*trait*/ QPoint_NewQPoint for () {
  fn NewQPoint(self) -> QPoint {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QPointC1Ev()};
    unsafe {_ZN6QPointC1Ev(qthis)};
    let rsthis = QPoint{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QPoint {
  pub fn setY<T: QPoint_setY>(&mut self, value: T)  {
     value.setY(self);
    // return 1;
  }
}

pub trait QPoint_setY {
  fn setY(self, rsthis: &mut QPoint) ;
}

// proto:  void QPoint::setY(int y);
impl<'a> /*trait*/ QPoint_setY for (i32) {
  fn setY(self, rsthis: &mut QPoint)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QPoint4setYEi()};
    let arg0 = self  as c_int;
     unsafe {_ZN6QPoint4setYEi(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

impl /*struct*/ QPoint {
  pub fn rx<T: QPoint_rx>(&mut self, value: T)  {
     value.rx(self);
    // return 1;
  }
}

pub trait QPoint_rx {
  fn rx(self, rsthis: &mut QPoint) ;
}

// proto:  int & QPoint::rx();
impl<'a> /*trait*/ QPoint_rx for () {
  fn rx(self, rsthis: &mut QPoint)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QPoint2rxEv()};
     unsafe {_ZN6QPoint2rxEv(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QPoint {
  pub fn manhattanLength<T: QPoint_manhattanLength>(&mut self, value: T) -> i32 {
    return value.manhattanLength(self);
    // return 1;
  }
}

pub trait QPoint_manhattanLength {
  fn manhattanLength(self, rsthis: &mut QPoint) -> i32;
}

// proto:  int QPoint::manhattanLength();
impl<'a> /*trait*/ QPoint_manhattanLength for () {
  fn manhattanLength(self, rsthis: &mut QPoint) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK6QPoint15manhattanLengthEv()};
    let mut ret = unsafe {_ZNK6QPoint15manhattanLengthEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

