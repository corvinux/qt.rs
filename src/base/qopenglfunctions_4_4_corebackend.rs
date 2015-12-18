// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qopenglcontext::QOpenGLContext;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QOpenGLFunctions_4_4_CoreBackend::NewQOpenGLFunctions_4_4_CoreBackend(QOpenGLContext * context);
  fn _ZN32QOpenGLFunctions_4_4_CoreBackendC1EP14QOpenGLContext(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto: static QOpenGLVersionStatus QOpenGLFunctions_4_4_CoreBackend::versionStatus();
  fn _ZN32QOpenGLFunctions_4_4_CoreBackend13versionStatusEv() ;
}

// body block begin
// class sizeof(QOpenGLFunctions_4_4_CoreBackend)=1
pub struct QOpenGLFunctions_4_4_CoreBackend {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QOpenGLFunctions_4_4_CoreBackend {
  pub fn NewQOpenGLFunctions_4_4_CoreBackend<T: QOpenGLFunctions_4_4_CoreBackend_NewQOpenGLFunctions_4_4_CoreBackend>(value: T) -> QOpenGLFunctions_4_4_CoreBackend {
    let rsthis = value.NewQOpenGLFunctions_4_4_CoreBackend();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLFunctions_4_4_CoreBackend_NewQOpenGLFunctions_4_4_CoreBackend {
  fn NewQOpenGLFunctions_4_4_CoreBackend(self) -> QOpenGLFunctions_4_4_CoreBackend;
}

// proto: void QOpenGLFunctions_4_4_CoreBackend::NewQOpenGLFunctions_4_4_CoreBackend(QOpenGLContext * context);
impl<'a> /*trait*/ QOpenGLFunctions_4_4_CoreBackend_NewQOpenGLFunctions_4_4_CoreBackend for (&'a mut QOpenGLContext) {
  fn NewQOpenGLFunctions_4_4_CoreBackend(self) -> QOpenGLFunctions_4_4_CoreBackend {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN32QOpenGLFunctions_4_4_CoreBackendC1EP14QOpenGLContext()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN32QOpenGLFunctions_4_4_CoreBackendC1EP14QOpenGLContext(qthis, arg0)};
    let rsthis = QOpenGLFunctions_4_4_CoreBackend{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions_4_4_CoreBackend {
  pub fn versionStatus<RetType, T: QOpenGLFunctions_4_4_CoreBackend_versionStatus<RetType>>(&mut self, value: T) -> RetType {
    return value.versionStatus(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_4_4_CoreBackend_versionStatus<RetType> {
  fn versionStatus(self, rsthis: &mut QOpenGLFunctions_4_4_CoreBackend) -> RetType;
}

// proto: static QOpenGLVersionStatus QOpenGLFunctions_4_4_CoreBackend::versionStatus();
impl<'a> /*trait*/ QOpenGLFunctions_4_4_CoreBackend_versionStatus<()> for () {
  fn versionStatus(self, rsthis: &mut QOpenGLFunctions_4_4_CoreBackend) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN32QOpenGLFunctions_4_4_CoreBackend13versionStatusEv()};
     unsafe {_ZN32QOpenGLFunctions_4_4_CoreBackend13versionStatusEv()};
    // return 1;
  }
}

