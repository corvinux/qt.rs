// auto generated, do not modify.
// created: Mon Dec 21 22:54:38 2015
// src-file: /QtCore/qscopedpointer.h
// dst-file: /src/core/qscopedpointer.rs
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
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto: static void QScopedPointerPodDeleter::cleanup(void * pointer);
  fn _ZN24QScopedPointerPodDeleter7cleanupEPv(arg0: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QScopedPointerPodDeleter)=1
pub struct QScopedPointerPodDeleter {
  pub qclsinst: *mut c_void,
}

  // proto: static void QScopedPointerPodDeleter::cleanup(void * pointer);
impl /*struct*/ QScopedPointerPodDeleter {
  pub fn cleanup_s<RetType, T: QScopedPointerPodDeleter_cleanup_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.cleanup_s();
    // return 1;
  }
}

pub trait QScopedPointerPodDeleter_cleanup_s<RetType> {
  fn cleanup_s(self ) -> RetType;
}

  // proto: static void QScopedPointerPodDeleter::cleanup(void * pointer);
impl<'a> /*trait*/ QScopedPointerPodDeleter_cleanup_s<()> for (*mut c_void) {
  fn cleanup_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN24QScopedPointerPodDeleter7cleanupEPv()};
    let arg0 = self  as *mut c_void;
     unsafe {_ZN24QScopedPointerPodDeleter7cleanupEPv(arg0)};
    // return 1;
  }
}

// <= body block end

