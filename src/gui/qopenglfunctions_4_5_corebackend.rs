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
  // proto: static QOpenGLVersionStatus QOpenGLFunctions_4_5_CoreBackend::versionStatus();
  fn _ZN32QOpenGLFunctions_4_5_CoreBackend13versionStatusEv();
  // proto:  void QOpenGLFunctions_4_5_CoreBackend::QOpenGLFunctions_4_5_CoreBackend(QOpenGLContext * context);
  fn _ZN32QOpenGLFunctions_4_5_CoreBackendC1EP14QOpenGLContext(qthis: *mut c_void, arg0: *mut c_void);
}

// body block begin
// class sizeof(QOpenGLFunctions_4_5_CoreBackend)=1
pub struct QOpenGLFunctions_4_5_CoreBackend {
  pub qclsinst: *mut c_void,
}

  // proto: static QOpenGLVersionStatus QOpenGLFunctions_4_5_CoreBackend::versionStatus();
impl /*struct*/ QOpenGLFunctions_4_5_CoreBackend {
  pub fn versionStatus_s<RetType, T: QOpenGLFunctions_4_5_CoreBackend_versionStatus_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.versionStatus_s();
    // return 1;
  }
}

pub trait QOpenGLFunctions_4_5_CoreBackend_versionStatus_s<RetType> {
  fn versionStatus_s(self ) -> RetType;
}

  // proto: static QOpenGLVersionStatus QOpenGLFunctions_4_5_CoreBackend::versionStatus();
impl<'a> /*trait*/ QOpenGLFunctions_4_5_CoreBackend_versionStatus_s<()> for () {
  fn versionStatus_s(self ) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN32QOpenGLFunctions_4_5_CoreBackend13versionStatusEv()};
     unsafe {_ZN32QOpenGLFunctions_4_5_CoreBackend13versionStatusEv()};
    // return 1;
  }
}

  // proto:  void QOpenGLFunctions_4_5_CoreBackend::QOpenGLFunctions_4_5_CoreBackend(QOpenGLContext * context);
impl /*struct*/ QOpenGLFunctions_4_5_CoreBackend {
  pub fn NewQOpenGLFunctions_4_5_CoreBackend<T: QOpenGLFunctions_4_5_CoreBackend_NewQOpenGLFunctions_4_5_CoreBackend>(value: T) -> QOpenGLFunctions_4_5_CoreBackend {
    let rsthis = value.NewQOpenGLFunctions_4_5_CoreBackend();
    return rsthis;
    // return 1;
  }
}

pub trait QOpenGLFunctions_4_5_CoreBackend_NewQOpenGLFunctions_4_5_CoreBackend {
  fn NewQOpenGLFunctions_4_5_CoreBackend(self) -> QOpenGLFunctions_4_5_CoreBackend;
}

  // proto:  void QOpenGLFunctions_4_5_CoreBackend::QOpenGLFunctions_4_5_CoreBackend(QOpenGLContext * context);
impl<'a> /*trait*/ QOpenGLFunctions_4_5_CoreBackend_NewQOpenGLFunctions_4_5_CoreBackend for (QOpenGLContext) {
  fn NewQOpenGLFunctions_4_5_CoreBackend(self) -> QOpenGLFunctions_4_5_CoreBackend {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN32QOpenGLFunctions_4_5_CoreBackendC1EP14QOpenGLContext()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN32QOpenGLFunctions_4_5_CoreBackendC1EP14QOpenGLContext(qthis, arg0)};
    let rsthis = QOpenGLFunctions_4_5_CoreBackend{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

