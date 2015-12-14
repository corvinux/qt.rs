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
  // proto:  void QTextFrameLayoutData::FreeQTextFrameLayoutData();
  fn _ZN20QTextFrameLayoutDataD0Ev(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QTextFrameLayoutData)=8
pub struct QTextFrameLayoutData {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTextFrameLayoutData {
  pub fn FreeQTextFrameLayoutData<T: QTextFrameLayoutData_FreeQTextFrameLayoutData>(&mut self, value: T)  {
     value.FreeQTextFrameLayoutData(self);
    // return 1;
  }
}

pub trait QTextFrameLayoutData_FreeQTextFrameLayoutData {
  fn FreeQTextFrameLayoutData(self, rsthis: &mut QTextFrameLayoutData) ;
}

// proto:  void QTextFrameLayoutData::FreeQTextFrameLayoutData();
impl<'a> /*trait*/ QTextFrameLayoutData_FreeQTextFrameLayoutData for () {
  fn FreeQTextFrameLayoutData(self, rsthis: &mut QTextFrameLayoutData)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN20QTextFrameLayoutDataD0Ev()};
     unsafe {_ZN20QTextFrameLayoutDataD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

