// auto generated, do not modify.
// created: Sun Jan 17 17:37:11 2016
// src-file: /QtCore/qstringbuilder.h
// dst-file: /src/core/qstringbuilder.rs
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
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QAbstractConcatenable_Class_Size() -> c_int;
} // <= ext block end

// body block begin =>
// class sizeof(QAbstractConcatenable)=1
#[derive(Default)]
pub struct QAbstractConcatenable {
  // qbase: None,
  pub qclsinst: u64 /* *mut c_void*/,
}

impl /*struct*/ QAbstractConcatenable {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QAbstractConcatenable {
    return QAbstractConcatenable{qclsinst: qthis, ..Default::default()};
  }
}
// <= body block end

