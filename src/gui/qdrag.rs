// auto generated, do not modify.
// created: Wed Dec 30 23:22:52 2015
// src-file: /QtGui/qdrag.h
// dst-file: /src/gui/qdrag.rs
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
use super::super::core::qobject::QObject; // 771
use std::ops::Deref;
use super::super::core::qmimedata::QMimeData; // 771
use super::qpixmap::QPixmap; // 773
use super::super::core::qpoint::QPoint; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QDrag_Class_Size() -> c_int;
  // proto:  QObject * QDrag::target();
  fn _ZNK5QDrag6targetEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QMimeData * QDrag::mimeData();
  fn _ZNK5QDrag8mimeDataEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QDrag::QDrag(QObject * dragSource);
  fn dector_ZN5QDragC1EP7QObject(arg0: *mut c_void) -> *mut c_void;
  fn _ZN5QDragC1EP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QDrag::~QDrag();
  fn _ZN5QDragD0Ev(qthis: u64 /* *mut c_void*/);
  // proto:  void QDrag::QDrag(const QDrag & );
  fn dector_ZN5QDragC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN5QDragC1ERKS_(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  void QDrag::setHotSpot(const QPoint & hotspot);
  fn _ZN5QDrag10setHotSpotERK6QPoint(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  const QMetaObject * QDrag::metaObject();
  fn _ZNK5QDrag10metaObjectEv(qthis: u64 /* *mut c_void*/);
  // proto:  void QDrag::setMimeData(QMimeData * data);
  fn _ZN5QDrag11setMimeDataEP9QMimeData(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QPixmap QDrag::pixmap();
  fn _ZNK5QDrag6pixmapEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  QPoint QDrag::hotSpot();
  fn _ZNK5QDrag7hotSpotEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QDrag::setPixmap(const QPixmap & );
  fn _ZN5QDrag9setPixmapERK7QPixmap(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  // proto:  QObject * QDrag::source();
  fn _ZNK5QDrag6sourceEv(qthis: u64 /* *mut c_void*/) -> *mut c_void;
  // proto:  void QDrag::targetChanged(QObject * newTarget);
  fn _ZN5QDrag13targetChangedEP7QObject(qthis: u64 /* *mut c_void*/, arg0: *mut c_void);
  fn QDrag_SlotProxy_connect__ZN5QDrag13targetChangedEP7QObject(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
  fn QDrag_SlotProxy_connect__ZN5QDrag13actionChangedEN2Qt10DropActionE(qthis: *mut c_void, ffifptr: *mut c_void, rsfptr: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QDrag)=1
#[derive(Default)]
pub struct QDrag {
  qbase: QObject,
  pub qclsinst: u64 /* *mut c_void*/,
  pub _targetChanged_1: QDrag_targetChanged_signal,
  pub _actionChanged_1: QDrag_actionChanged_signal,
}

impl /*struct*/ QDrag {
  pub fn inheritFrom(qthis: u64 /* *mut c_void*/) -> QDrag {
    return QDrag{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
  }
}
impl Deref for QDrag {
  type Target = QObject;

  fn deref(&self) -> &QObject {
    return & self.qbase;
  }
}
impl AsRef<QObject> for QDrag {
  fn as_ref(& self) -> & QObject {
    return & self.qbase;
  }
}
  // proto:  QObject * QDrag::target();
impl /*struct*/ QDrag {
  pub fn target<RetType, T: QDrag_target<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.target(self);
    // return 1;
  }
}

pub trait QDrag_target<RetType> {
  fn target(self , rsthis: & QDrag) -> RetType;
}

  // proto:  QObject * QDrag::target();
impl<'a> /*trait*/ QDrag_target<QObject> for () {
  fn target(self , rsthis: & QDrag) -> QObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDrag6targetEv()};
    let mut ret = unsafe {_ZNK5QDrag6targetEv(rsthis.qclsinst)};
    let mut ret1 = QObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QMimeData * QDrag::mimeData();
impl /*struct*/ QDrag {
  pub fn mimeData<RetType, T: QDrag_mimeData<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.mimeData(self);
    // return 1;
  }
}

pub trait QDrag_mimeData<RetType> {
  fn mimeData(self , rsthis: & QDrag) -> RetType;
}

  // proto:  QMimeData * QDrag::mimeData();
impl<'a> /*trait*/ QDrag_mimeData<QMimeData> for () {
  fn mimeData(self , rsthis: & QDrag) -> QMimeData {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDrag8mimeDataEv()};
    let mut ret = unsafe {_ZNK5QDrag8mimeDataEv(rsthis.qclsinst)};
    let mut ret1 = QMimeData::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QDrag::QDrag(QObject * dragSource);
impl /*struct*/ QDrag {
  pub fn New<T: QDrag_New>(value: T) -> QDrag {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QDrag_New {
  fn New(self) -> QDrag;
}

  // proto:  void QDrag::QDrag(QObject * dragSource);
impl<'a> /*trait*/ QDrag_New for (&'a QObject) {
  fn New(self) -> QDrag {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QDragC1EP7QObject()};
    let ctysz: c_int = unsafe{QDrag_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN5QDragC1EP7QObject(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN5QDragC1EP7QObject(arg0)} as u64;
    let rsthis = QDrag{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QDrag::~QDrag();
impl /*struct*/ QDrag {
  pub fn Free<RetType, T: QDrag_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QDrag_Free<RetType> {
  fn Free(self , rsthis: & QDrag) -> RetType;
}

  // proto:  void QDrag::~QDrag();
impl<'a> /*trait*/ QDrag_Free<()> for () {
  fn Free(self , rsthis: & QDrag) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QDragD0Ev()};
     unsafe {_ZN5QDragD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QDrag::QDrag(const QDrag & );
impl<'a> /*trait*/ QDrag_New for (&'a QDrag) {
  fn New(self) -> QDrag {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QDragC1ERKS_()};
    let ctysz: c_int = unsafe{QDrag_Class_Size()};
    let qthis_ph: u64 = unsafe{calloc(1, ctysz as usize)} as u64;
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN5QDragC1ERKS_(qthis, arg0)};
    let qthis: u64 = unsafe {dector_ZN5QDragC1ERKS_(arg0)} as u64;
    let rsthis = QDrag{qbase: QObject::inheritFrom(qthis), qclsinst: qthis, ..Default::default()};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QDrag::setHotSpot(const QPoint & hotspot);
impl /*struct*/ QDrag {
  pub fn setHotSpot<RetType, T: QDrag_setHotSpot<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setHotSpot(self);
    // return 1;
  }
}

pub trait QDrag_setHotSpot<RetType> {
  fn setHotSpot(self , rsthis: & QDrag) -> RetType;
}

  // proto:  void QDrag::setHotSpot(const QPoint & hotspot);
impl<'a> /*trait*/ QDrag_setHotSpot<()> for (&'a QPoint) {
  fn setHotSpot(self , rsthis: & QDrag) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QDrag10setHotSpotERK6QPoint()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN5QDrag10setHotSpotERK6QPoint(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QDrag::metaObject();
impl /*struct*/ QDrag {
  pub fn metaObject<RetType, T: QDrag_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QDrag_metaObject<RetType> {
  fn metaObject(self , rsthis: & QDrag) -> RetType;
}

  // proto:  const QMetaObject * QDrag::metaObject();
impl<'a> /*trait*/ QDrag_metaObject<()> for () {
  fn metaObject(self , rsthis: & QDrag) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDrag10metaObjectEv()};
     unsafe {_ZNK5QDrag10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QDrag::setMimeData(QMimeData * data);
impl /*struct*/ QDrag {
  pub fn setMimeData<RetType, T: QDrag_setMimeData<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setMimeData(self);
    // return 1;
  }
}

pub trait QDrag_setMimeData<RetType> {
  fn setMimeData(self , rsthis: & QDrag) -> RetType;
}

  // proto:  void QDrag::setMimeData(QMimeData * data);
impl<'a> /*trait*/ QDrag_setMimeData<()> for (&'a QMimeData) {
  fn setMimeData(self , rsthis: & QDrag) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QDrag11setMimeDataEP9QMimeData()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN5QDrag11setMimeDataEP9QMimeData(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QPixmap QDrag::pixmap();
impl /*struct*/ QDrag {
  pub fn pixmap<RetType, T: QDrag_pixmap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.pixmap(self);
    // return 1;
  }
}

pub trait QDrag_pixmap<RetType> {
  fn pixmap(self , rsthis: & QDrag) -> RetType;
}

  // proto:  QPixmap QDrag::pixmap();
impl<'a> /*trait*/ QDrag_pixmap<QPixmap> for () {
  fn pixmap(self , rsthis: & QDrag) -> QPixmap {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDrag6pixmapEv()};
    let mut ret = unsafe {_ZNK5QDrag6pixmapEv(rsthis.qclsinst)};
    let mut ret1 = QPixmap::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  QPoint QDrag::hotSpot();
impl /*struct*/ QDrag {
  pub fn hotSpot<RetType, T: QDrag_hotSpot<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.hotSpot(self);
    // return 1;
  }
}

pub trait QDrag_hotSpot<RetType> {
  fn hotSpot(self , rsthis: & QDrag) -> RetType;
}

  // proto:  QPoint QDrag::hotSpot();
impl<'a> /*trait*/ QDrag_hotSpot<QPoint> for () {
  fn hotSpot(self , rsthis: & QDrag) -> QPoint {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDrag7hotSpotEv()};
    let mut ret = unsafe {_ZNK5QDrag7hotSpotEv(rsthis.qclsinst)};
    let mut ret1 = QPoint::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QDrag::setPixmap(const QPixmap & );
impl /*struct*/ QDrag {
  pub fn setPixmap<RetType, T: QDrag_setPixmap<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setPixmap(self);
    // return 1;
  }
}

pub trait QDrag_setPixmap<RetType> {
  fn setPixmap(self , rsthis: & QDrag) -> RetType;
}

  // proto:  void QDrag::setPixmap(const QPixmap & );
impl<'a> /*trait*/ QDrag_setPixmap<()> for (&'a QPixmap) {
  fn setPixmap(self , rsthis: & QDrag) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QDrag9setPixmapERK7QPixmap()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN5QDrag9setPixmapERK7QPixmap(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QObject * QDrag::source();
impl /*struct*/ QDrag {
  pub fn source<RetType, T: QDrag_source<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.source(self);
    // return 1;
  }
}

pub trait QDrag_source<RetType> {
  fn source(self , rsthis: & QDrag) -> RetType;
}

  // proto:  QObject * QDrag::source();
impl<'a> /*trait*/ QDrag_source<QObject> for () {
  fn source(self , rsthis: & QDrag) -> QObject {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK5QDrag6sourceEv()};
    let mut ret = unsafe {_ZNK5QDrag6sourceEv(rsthis.qclsinst)};
    let mut ret1 = QObject::inheritFrom(ret as u64);
    return ret1;
    // return 1;
  }
}

  // proto:  void QDrag::targetChanged(QObject * newTarget);
impl /*struct*/ QDrag {
  pub fn targetChanged<RetType, T: QDrag_targetChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.targetChanged(self);
    // return 1;
  }
}

pub trait QDrag_targetChanged<RetType> {
  fn targetChanged(self , rsthis: & QDrag) -> RetType;
}

  // proto:  void QDrag::targetChanged(QObject * newTarget);
impl<'a> /*trait*/ QDrag_targetChanged<()> for (&'a QObject) {
  fn targetChanged(self , rsthis: & QDrag) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN5QDrag13targetChangedEP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN5QDrag13targetChangedEP7QObject(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

#[derive(Default)] // for QDrag_targetChanged
pub struct QDrag_targetChanged_signal{poi:u64}
impl /* struct */ QDrag {
  pub fn targetChanged_1(&self) -> QDrag_targetChanged_signal {
     return QDrag_targetChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QDrag_targetChanged_signal {
  pub fn connect<T: QDrag_targetChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QDrag_targetChanged_signal_connect {
  fn connect(self, sigthis: QDrag_targetChanged_signal);
}

#[derive(Default)] // for QDrag_actionChanged
pub struct QDrag_actionChanged_signal{poi:u64}
impl /* struct */ QDrag {
  pub fn actionChanged_1(&self) -> QDrag_actionChanged_signal {
     return QDrag_actionChanged_signal{poi:self.qclsinst};
  }
}
impl /* struct */ QDrag_actionChanged_signal {
  pub fn connect<T: QDrag_actionChanged_signal_connect>(self, overload_args: T) {
    overload_args.connect(self);
  }
}
pub trait QDrag_actionChanged_signal_connect {
  fn connect(self, sigthis: QDrag_actionChanged_signal);
}

// targetChanged(class QObject *)
extern fn QDrag_targetChanged_signal_connect_cb_0(rsfptr:fn(QObject), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = QObject::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
extern fn QDrag_targetChanged_signal_connect_cb_box_0(rsfptr_raw:*mut fn(QObject), arg0: *mut c_void) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = QObject::inheritFrom(arg0 as u64);
  rsfptr(rsarg0);
}
impl /* trait */ QDrag_targetChanged_signal_connect for fn(QObject) {
  fn connect(self, sigthis: QDrag_targetChanged_signal) {
    // do smth...
    self as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QDrag_targetChanged_signal_connect_cb_0 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QDrag_SlotProxy_connect__ZN5QDrag13targetChangedEP7QObject(arg0, arg1, arg2)};
  }
}
impl /* trait */ QDrag_targetChanged_signal_connect for Box<fn(QObject)> {
  fn connect(self, sigthis: QDrag_targetChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QDrag_targetChanged_signal_connect_cb_box_0 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QDrag_SlotProxy_connect__ZN5QDrag13targetChangedEP7QObject(arg0, arg1, arg2)};
  }
}
// actionChanged(Qt::DropAction)
extern fn QDrag_actionChanged_signal_connect_cb_1(rsfptr:fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
extern fn QDrag_actionChanged_signal_connect_cb_box_1(rsfptr_raw:*mut fn(i32), arg0: c_int) {
  println!("{}:{}", file!(), line!());
  let rsfptr = unsafe{Box::from_raw(rsfptr_raw)};
  let rsarg0 = arg0 as i32;
  rsfptr(rsarg0);
}
impl /* trait */ QDrag_actionChanged_signal_connect for fn(i32) {
  fn connect(self, sigthis: QDrag_actionChanged_signal) {
    // do smth...
    self as u64;
    self as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QDrag_actionChanged_signal_connect_cb_1 as *mut c_void;
    let arg2 = self as *mut c_void;
    unsafe {QDrag_SlotProxy_connect__ZN5QDrag13actionChangedEN2Qt10DropActionE(arg0, arg1, arg2)};
  }
}
impl /* trait */ QDrag_actionChanged_signal_connect for Box<fn(i32)> {
  fn connect(self, sigthis: QDrag_actionChanged_signal) {
    // do smth...
    // Box::into_raw(self) as u64;
    // Box::into_raw(self) as *mut c_void;
    let arg0 = sigthis.poi as *mut c_void;
    let arg1 = QDrag_actionChanged_signal_connect_cb_box_1 as *mut c_void;
    let arg2 = Box::into_raw(self) as *mut c_void;
    unsafe {QDrag_SlotProxy_connect__ZN5QDrag13actionChangedEN2Qt10DropActionE(arg0, arg1, arg2)};
  }
}
// <= body block end

