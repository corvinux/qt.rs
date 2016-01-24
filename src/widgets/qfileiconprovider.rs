// auto generated, do not modify.
// created: Sun Jan 24 17:41:38 2016
// src-file: /QtWidgets/qfileiconprovider.h
// dst-file: /src/widgets/qfileiconprovider.rs
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
use std::ops::Deref;
use super::super::core::qfileinfo::QFileInfo; // 771
use super::super::core::qstring::QString; // 771
use super::super::gui::qicon::QIcon; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QFileIconProvider_Class_Size() -> c_int;
  // proto:  QString QFileIconProvider::type(const QFileInfo & info);
  fn C_ZNK17QFileIconProvider4typeERK9QFileInfo(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  QIcon QFileIconProvider::icon(const QFileInfo & info);
  fn C_ZNK17QFileIconProvider4iconERK9QFileInfo(qthis: u64 /* *mut c_void*/, arg0: *mut c_void) -> *mut c_void;
  // proto:  void QFileIconProvider::QFileIconProvider();
  fn C_ZN17QFileIconProviderC2Ev() -> u64;
  // proto:  void QFileIconProvider::~QFileIconProvider();
  fn C_ZN17QFileIconProviderD2Ev(qthis: u64 /* *mut c_void*/);
} // <= ext block end

// body block begin =>
// class sizeof(QFileIconProvider)=1
#[derive(Default)]
pub struct QFileIconProvider {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QFileIconProvider {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QFileIconProvider {
    return QFileIconProvider{qclsinst: qthis, ..Default::default()};
  }
}
  // proto:  QString QFileIconProvider::type(const QFileInfo & info);
impl /*struct*/ QFileIconProvider {
  pub fn type_<RetType, T: QFileIconProvider_type_<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.type_(self);
    // return 1;
  }
}

pub trait QFileIconProvider_type_<RetType> {
  fn type_(self , rsthis: & QFileIconProvider) -> RetType;
}

  // proto:  QString QFileIconProvider::type(const QFileInfo & info);
impl<'a> /*trait*/ QFileIconProvider_type_<QString> for (&'a QFileInfo) {
  fn type_(self , rsthis: & QFileIconProvider) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QFileIconProvider4typeERK9QFileInfo()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK17QFileIconProvider4typeERK9QFileInfo(rsthis.qclsinst, arg0)};
    let mut ret1 = QString::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QIcon QFileIconProvider::icon(const QFileInfo & info);
impl /*struct*/ QFileIconProvider {
  pub fn icon<RetType, T: QFileIconProvider_icon<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.icon(self);
    // return 1;
  }
}

pub trait QFileIconProvider_icon<RetType> {
  fn icon(self , rsthis: & QFileIconProvider) -> RetType;
}

  // proto:  QIcon QFileIconProvider::icon(const QFileInfo & info);
impl<'a> /*trait*/ QFileIconProvider_icon<QIcon> for (&'a QFileInfo) {
  fn icon(self , rsthis: & QFileIconProvider) -> QIcon {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK17QFileIconProvider4iconERK9QFileInfo()};
    let arg0 = self.qclsinst  as *mut c_void;
    let mut ret = unsafe {C_ZNK17QFileIconProvider4iconERK9QFileInfo(rsthis.qclsinst, arg0)};
    let mut ret1 = QIcon::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QFileIconProvider::QFileIconProvider();
impl /*struct*/ QFileIconProvider {
  pub fn new<T: QFileIconProvider_new>(value: T) -> QFileIconProvider {
    let rsthis = value.new();
    return rsthis;
    // return 1;
  }
}

pub trait QFileIconProvider_new {
  fn new(self) -> QFileIconProvider;
}

  // proto:  void QFileIconProvider::QFileIconProvider();
impl<'a> /*trait*/ QFileIconProvider_new for () {
  fn new(self) -> QFileIconProvider {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QFileIconProviderC2Ev()};
    let ctysz: c_int = unsafe{QFileIconProvider_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let qthis: u64 = unsafe {C_ZN17QFileIconProviderC2Ev()};
    let rsthis = QFileIconProvider{qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QFileIconProvider::~QFileIconProvider();
impl /*struct*/ QFileIconProvider {
  pub fn free<RetType, T: QFileIconProvider_free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.free(self);
    // return 1;
  }
}

pub trait QFileIconProvider_free<RetType> {
  fn free(self , rsthis: & QFileIconProvider) -> RetType;
}

  // proto:  void QFileIconProvider::~QFileIconProvider();
impl<'a> /*trait*/ QFileIconProvider_free<()> for () {
  fn free(self , rsthis: & QFileIconProvider) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN17QFileIconProviderD2Ev()};
     unsafe {C_ZN17QFileIconProviderD2Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

