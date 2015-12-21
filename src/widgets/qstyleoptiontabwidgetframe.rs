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
  // proto:  void QStyleOptionTabWidgetFrame::QStyleOptionTabWidgetFrame(const QStyleOptionTabWidgetFrame & other);
  fn _ZN26QStyleOptionTabWidgetFrameC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QStyleOptionTabWidgetFrame::QStyleOptionTabWidgetFrame(int version);
  fn _ZN26QStyleOptionTabWidgetFrameC1Ei(qthis: *mut c_void, arg0: c_int);
  // proto:  void QStyleOptionTabWidgetFrame::QStyleOptionTabWidgetFrame();
  fn _ZN26QStyleOptionTabWidgetFrameC1Ev(qthis: *mut c_void);
}

// body block begin
// class sizeof(QStyleOptionTabWidgetFrame)=1
pub struct QStyleOptionTabWidgetFrame {
  pub qclsinst: *mut c_void,
}

  // proto:  void QStyleOptionTabWidgetFrame::QStyleOptionTabWidgetFrame(const QStyleOptionTabWidgetFrame & other);
impl /*struct*/ QStyleOptionTabWidgetFrame {
  pub fn NewQStyleOptionTabWidgetFrame<T: QStyleOptionTabWidgetFrame_NewQStyleOptionTabWidgetFrame>(value: T) -> QStyleOptionTabWidgetFrame {
    let rsthis = value.NewQStyleOptionTabWidgetFrame();
    return rsthis;
    // return 1;
  }
}

pub trait QStyleOptionTabWidgetFrame_NewQStyleOptionTabWidgetFrame {
  fn NewQStyleOptionTabWidgetFrame(self) -> QStyleOptionTabWidgetFrame;
}

  // proto:  void QStyleOptionTabWidgetFrame::QStyleOptionTabWidgetFrame(const QStyleOptionTabWidgetFrame & other);
impl<'a> /*trait*/ QStyleOptionTabWidgetFrame_NewQStyleOptionTabWidgetFrame for (QStyleOptionTabWidgetFrame) {
  fn NewQStyleOptionTabWidgetFrame(self) -> QStyleOptionTabWidgetFrame {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QStyleOptionTabWidgetFrameC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN26QStyleOptionTabWidgetFrameC1ERKS_(qthis, arg0)};
    let rsthis = QStyleOptionTabWidgetFrame{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionTabWidgetFrame::QStyleOptionTabWidgetFrame(int version);
impl<'a> /*trait*/ QStyleOptionTabWidgetFrame_NewQStyleOptionTabWidgetFrame for (i32) {
  fn NewQStyleOptionTabWidgetFrame(self) -> QStyleOptionTabWidgetFrame {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QStyleOptionTabWidgetFrameC1Ei()};
    let arg0 = self  as c_int;
    unsafe {_ZN26QStyleOptionTabWidgetFrameC1Ei(qthis, arg0)};
    let rsthis = QStyleOptionTabWidgetFrame{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyleOptionTabWidgetFrame::QStyleOptionTabWidgetFrame();
impl<'a> /*trait*/ QStyleOptionTabWidgetFrame_NewQStyleOptionTabWidgetFrame for () {
  fn NewQStyleOptionTabWidgetFrame(self) -> QStyleOptionTabWidgetFrame {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN26QStyleOptionTabWidgetFrameC1Ev()};
    unsafe {_ZN26QStyleOptionTabWidgetFrameC1Ev(qthis)};
    let rsthis = QStyleOptionTabWidgetFrame{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

