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
  // proto:  void QOpenGLFunctions_1_2_DeprecatedBackend::NewQOpenGLFunctions_1_2_DeprecatedBackend(QOpenGLContext * context);
  fn _ZN38QOpenGLFunctions_1_2_DeprecatedBackendC1EP14QOpenGLContext(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto: static QOpenGLVersionStatus QOpenGLFunctions_1_2_DeprecatedBackend::versionStatus();
  fn _ZN38QOpenGLFunctions_1_2_DeprecatedBackend13versionStatusEv() ;
}

// body block begin
// class sizeof(QOpenGLFunctions_1_2_DeprecatedBackend)=1
pub struct QOpenGLFunctions_1_2_DeprecatedBackend {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QOpenGLFunctions_1_2_DeprecatedBackend {
  pub fn NewQOpenGLFunctions_1_2_DeprecatedBackend<T: QOpenGLFunctions_1_2_DeprecatedBackend_NewQOpenGLFunctions_1_2_DeprecatedBackend>(value: T) -> QOpenGLFunctions_1_2_DeprecatedBackend {
    let rsthis = value.NewQOpenGLFunctions_1_2_DeprecatedBackend();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLFunctions_1_2_DeprecatedBackend_NewQOpenGLFunctions_1_2_DeprecatedBackend {
  fn NewQOpenGLFunctions_1_2_DeprecatedBackend(self) -> QOpenGLFunctions_1_2_DeprecatedBackend;
}

// proto: void QOpenGLFunctions_1_2_DeprecatedBackend::NewQOpenGLFunctions_1_2_DeprecatedBackend(QOpenGLContext * context);
impl<'a> /*trait*/ QOpenGLFunctions_1_2_DeprecatedBackend_NewQOpenGLFunctions_1_2_DeprecatedBackend for (&'a mut QOpenGLContext) {
  fn NewQOpenGLFunctions_1_2_DeprecatedBackend(self) -> QOpenGLFunctions_1_2_DeprecatedBackend {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN38QOpenGLFunctions_1_2_DeprecatedBackendC1EP14QOpenGLContext()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN38QOpenGLFunctions_1_2_DeprecatedBackendC1EP14QOpenGLContext(qthis, arg0)};
    let rsthis = QOpenGLFunctions_1_2_DeprecatedBackend{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QOpenGLFunctions_1_2_DeprecatedBackend {
  pub fn versionStatus<RetType, T: QOpenGLFunctions_1_2_DeprecatedBackend_versionStatus<RetType>>(&mut self, value: T) -> RetType {
    return value.versionStatus(self);
    // return 1;
  }
}

pub trait QOpenGLFunctions_1_2_DeprecatedBackend_versionStatus<RetType> {
  fn versionStatus(self, rsthis: &mut QOpenGLFunctions_1_2_DeprecatedBackend) -> RetType;
}

// proto: static QOpenGLVersionStatus QOpenGLFunctions_1_2_DeprecatedBackend::versionStatus();
impl<'a> /*trait*/ QOpenGLFunctions_1_2_DeprecatedBackend_versionStatus<()> for () {
  fn versionStatus(self, rsthis: &mut QOpenGLFunctions_1_2_DeprecatedBackend) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN38QOpenGLFunctions_1_2_DeprecatedBackend13versionStatusEv()};
     unsafe {_ZN38QOpenGLFunctions_1_2_DeprecatedBackend13versionStatusEv()};
    // return 1;
  }
}

