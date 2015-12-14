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
  // proto:  void QTextBlockUserData::FreeQTextBlockUserData();
  fn _ZN18QTextBlockUserDataD0Ev(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QTextBlockUserData)=8
pub struct QTextBlockUserData {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QTextBlockUserData {
  pub fn FreeQTextBlockUserData<T: QTextBlockUserData_FreeQTextBlockUserData>(&mut self, value: T)  {
     value.FreeQTextBlockUserData(self);
    // return 1;
  }
}

pub trait QTextBlockUserData_FreeQTextBlockUserData {
  fn FreeQTextBlockUserData(self, rsthis: &mut QTextBlockUserData) ;
}

// proto:  void QTextBlockUserData::FreeQTextBlockUserData();
impl<'a> /*trait*/ QTextBlockUserData_FreeQTextBlockUserData for () {
  fn FreeQTextBlockUserData(self, rsthis: &mut QTextBlockUserData)  {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QTextBlockUserDataD0Ev()};
     unsafe {_ZN18QTextBlockUserDataD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

