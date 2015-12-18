// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qwidget::QWidget;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QVBoxLayout::NewQVBoxLayout();
  fn _ZN11QVBoxLayoutC1Ev(qthis: *mut c_void) ;
  // proto:  const QMetaObject * QVBoxLayout::metaObject();
  fn _ZNK11QVBoxLayout10metaObjectEv(qthis: *mut c_void) ;
  // proto:  void QVBoxLayout::NewQVBoxLayout(const QVBoxLayout & );
  fn _ZN11QVBoxLayoutC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QVBoxLayout::NewQVBoxLayout(QWidget * parent);
  fn _ZN11QVBoxLayoutC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QVBoxLayout::FreeQVBoxLayout();
  fn _ZN11QVBoxLayoutD0Ev(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QVBoxLayout)=1
pub struct QVBoxLayout {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QVBoxLayout {
  pub fn NewQVBoxLayout<T: QVBoxLayout_NewQVBoxLayout>(value: T) -> QVBoxLayout {
    let rsthis = value.NewQVBoxLayout();
    return rsthis;
    // return 1;
  }
}

pub trait QVBoxLayout_NewQVBoxLayout {
  fn NewQVBoxLayout(self) -> QVBoxLayout;
}

// proto: void QVBoxLayout::NewQVBoxLayout();
impl<'a> /*trait*/ QVBoxLayout_NewQVBoxLayout for () {
  fn NewQVBoxLayout(self) -> QVBoxLayout {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QVBoxLayoutC1Ev()};
    unsafe {_ZN11QVBoxLayoutC1Ev(qthis)};
    let rsthis = QVBoxLayout{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QVBoxLayout {
  pub fn metaObject<RetType, T: QVBoxLayout_metaObject<RetType>>(&mut self, value: T) -> RetType {
    return value.metaObject(self);
    // return 1;
  }
}

pub trait QVBoxLayout_metaObject<RetType> {
  fn metaObject(self, rsthis: &mut QVBoxLayout) -> RetType;
}

// proto:  const QMetaObject * QVBoxLayout::metaObject();
impl<'a> /*trait*/ QVBoxLayout_metaObject<()> for () {
  fn metaObject(self, rsthis: &mut QVBoxLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QVBoxLayout10metaObjectEv()};
     unsafe {_ZNK11QVBoxLayout10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

// proto: void QVBoxLayout::NewQVBoxLayout(const QVBoxLayout & );
impl<'a> /*trait*/ QVBoxLayout_NewQVBoxLayout for (&'a  QVBoxLayout) {
  fn NewQVBoxLayout(self) -> QVBoxLayout {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QVBoxLayoutC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QVBoxLayoutC1ERKS_(qthis, arg0)};
    let rsthis = QVBoxLayout{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QVBoxLayout::NewQVBoxLayout(QWidget * parent);
impl<'a> /*trait*/ QVBoxLayout_NewQVBoxLayout for (&'a mut QWidget) {
  fn NewQVBoxLayout(self) -> QVBoxLayout {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QVBoxLayoutC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QVBoxLayoutC1EP7QWidget(qthis, arg0)};
    let rsthis = QVBoxLayout{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QVBoxLayout {
  pub fn FreeQVBoxLayout<RetType, T: QVBoxLayout_FreeQVBoxLayout<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQVBoxLayout(self);
    // return 1;
  }
}

pub trait QVBoxLayout_FreeQVBoxLayout<RetType> {
  fn FreeQVBoxLayout(self, rsthis: &mut QVBoxLayout) -> RetType;
}

// proto:  void QVBoxLayout::FreeQVBoxLayout();
impl<'a> /*trait*/ QVBoxLayout_FreeQVBoxLayout<()> for () {
  fn FreeQVBoxLayout(self, rsthis: &mut QVBoxLayout) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QVBoxLayoutD0Ev()};
     unsafe {_ZN11QVBoxLayoutD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

