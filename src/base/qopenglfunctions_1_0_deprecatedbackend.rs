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
  // proto:  void QOpenGLFunctions_1_0_DeprecatedBackend::QOpenGLFunctions_1_0_DeprecatedBackend(QOpenGLContext * context);
  fn _ZN38QOpenGLFunctions_1_0_DeprecatedBackendC1EP14QOpenGLContext(qthis: *mut c_void, arg0: *mut c_void);
  // proto: static QOpenGLVersionStatus QOpenGLFunctions_1_0_DeprecatedBackend::versionStatus();
  fn _ZN38QOpenGLFunctions_1_0_DeprecatedBackend13versionStatusEv();
}

// body block begin
// class sizeof(QOpenGLFunctions_1_0_DeprecatedBackend)=1
pub struct QOpenGLFunctions_1_0_DeprecatedBackend {
  pub qclsinst: *mut c_void,
}

  // proto:  void QOpenGLFunctions_1_0_DeprecatedBackend::QOpenGLFunctions_1_0_DeprecatedBackend(QOpenGLContext * context);
impl /*struct*/ QOpenGLFunctions_1_0_DeprecatedBackend {
  pub fn NewQOpenGLFunctions_1_0_DeprecatedBackend<T: QOpenGLFunctions_1_0_DeprecatedBackend_NewQOpenGLFunctions_1_0_DeprecatedBackend>(value: T) -> QOpenGLFunctions_1_0_DeprecatedBackend {
    let rsthis = value.NewQOpenGLFunctions_1_0_DeprecatedBackend();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLFunctions_1_0_DeprecatedBackend_NewQOpenGLFunctions_1_0_DeprecatedBackend {
  fn NewQOpenGLFunctions_1_0_DeprecatedBackend(self) -> QOpenGLFunctions_1_0_DeprecatedBackend;
}

  // proto:  void QOpenGLFunctions_1_0_DeprecatedBackend::QOpenGLFunctions_1_0_DeprecatedBackend(QOpenGLContext * context);
impl<'a> /*trait*/ QOpenGLFunctions_1_0_DeprecatedBackend_NewQOpenGLFunctions_1_0_DeprecatedBackend for (QOpenGLContext) {
  fn NewQOpenGLFunctions_1_0_DeprecatedBackend(self) -> QOpenGLFunctions_1_0_DeprecatedBackend {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN38QOpenGLFunctions_1_0_DeprecatedBackendC1EP14QOpenGLContext()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN38QOpenGLFunctions_1_0_DeprecatedBackendC1EP14QOpenGLContext(qthis, arg0)};
    let rsthis = QOpenGLFunctions_1_0_DeprecatedBackend{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto: static QOpenGLVersionStatus QOpenGLFunctions_1_0_DeprecatedBackend::versionStatus();
impl /*struct*/ QOpenGLFunctions_1_0_DeprecatedBackend {
  pub fn versionStatus_s<RetType, T: QOpenGLFunctions_1_0_DeprecatedBackend_versionStatus_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.versionStatus_s();
    // return 1;
  }
}

pub trait QOpenGLFunctions_1_0_DeprecatedBackend_versionStatus_s<RetType> {
  fn versionStatus_s(self ) -> RetType;
}

  // proto: static QOpenGLVersionStatus QOpenGLFunctions_1_0_DeprecatedBackend::versionStatus();
impl<'a> /*trait*/ QOpenGLFunctions_1_0_DeprecatedBackend_versionStatus_s<()> for () {
  fn versionStatus_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN38QOpenGLFunctions_1_0_DeprecatedBackend13versionStatusEv()};
     unsafe {_ZN38QOpenGLFunctions_1_0_DeprecatedBackend13versionStatusEv()};
    // return 1;
  }
}

