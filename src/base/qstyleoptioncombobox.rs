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
  // proto: void QStyleOptionComboBox::NewQStyleOptionComboBox(const QStyleOptionComboBox & other);
  fn _ZN20QStyleOptionComboBoxC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  // proto: void QStyleOptionComboBox::NewQStyleOptionComboBox();
  fn _ZN20QStyleOptionComboBoxC1Ev(qthis: *mut c_void) -> i32;
  // proto: void QStyleOptionComboBox::NewQStyleOptionComboBox(int version);
  fn _ZN20QStyleOptionComboBoxC1Ei(qthis: *mut c_void, arg0: c_int) -> i32;
}

// body block begin
// class sizeof(QStyleOptionComboBox)=1
pub struct QStyleOptionComboBox {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QStyleOptionComboBox {
  pub fn NewQStyleOptionComboBox<T: QStyleOptionComboBox_NewQStyleOptionComboBox>(value: T) -> QStyleOptionComboBox {
    let rsthis = value.NewQStyleOptionComboBox();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionComboBox_NewQStyleOptionComboBox {
  fn NewQStyleOptionComboBox(self) -> QStyleOptionComboBox;
}

// proto: void QStyleOptionComboBox::NewQStyleOptionComboBox(const QStyleOptionComboBox & other);
impl<'a> /*trait*/ QStyleOptionComboBox_NewQStyleOptionComboBox for (&'a  QStyleOptionComboBox) {
  fn NewQStyleOptionComboBox(self) -> QStyleOptionComboBox {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QStyleOptionComboBoxC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN20QStyleOptionComboBoxC1ERKS_(qthis, arg0)};
    let rsthis = QStyleOptionComboBox{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QStyleOptionComboBox::NewQStyleOptionComboBox();
impl<'a> /*trait*/ QStyleOptionComboBox_NewQStyleOptionComboBox for () {
  fn NewQStyleOptionComboBox(self) -> QStyleOptionComboBox {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QStyleOptionComboBoxC1Ev()};
    unsafe {_ZN20QStyleOptionComboBoxC1Ev(qthis)};
    let rsthis = QStyleOptionComboBox{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QStyleOptionComboBox::NewQStyleOptionComboBox(int version);
impl<'a> /*trait*/ QStyleOptionComboBox_NewQStyleOptionComboBox for (i32) {
  fn NewQStyleOptionComboBox(self) -> QStyleOptionComboBox {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QStyleOptionComboBoxC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN20QStyleOptionComboBoxC1Ei(qthis, arg0)};
    let rsthis = QStyleOptionComboBox{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

