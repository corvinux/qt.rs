// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qobject::QObject;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QParallelAnimationGroup::FreeQParallelAnimationGroup();
  fn _ZN23QParallelAnimationGroupD0Ev(qthis: *mut c_void) ;
  // proto:  void QParallelAnimationGroup::NewQParallelAnimationGroup(const QParallelAnimationGroup & );
  fn _ZN23QParallelAnimationGroupC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  int QParallelAnimationGroup::duration();
  fn _ZNK23QParallelAnimationGroup8durationEv(qthis: *mut c_void) -> c_int;
  // proto:  void QParallelAnimationGroup::NewQParallelAnimationGroup(QObject * parent);
  fn _ZN23QParallelAnimationGroupC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  const QMetaObject * QParallelAnimationGroup::metaObject();
  fn _ZNK23QParallelAnimationGroup10metaObjectEv(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QParallelAnimationGroup)=1
pub struct QParallelAnimationGroup {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QParallelAnimationGroup {
  pub fn FreeQParallelAnimationGroup<RetType, T: QParallelAnimationGroup_FreeQParallelAnimationGroup<RetType>>(&mut self, value: T) -> RetType {
    return value.FreeQParallelAnimationGroup(self);
    // return 1;
  }
}

pub trait QParallelAnimationGroup_FreeQParallelAnimationGroup<RetType> {
  fn FreeQParallelAnimationGroup(self, rsthis: &mut QParallelAnimationGroup) -> RetType;
}

// proto:  void QParallelAnimationGroup::FreeQParallelAnimationGroup();
impl<'a> /*trait*/ QParallelAnimationGroup_FreeQParallelAnimationGroup<()> for () {
  fn FreeQParallelAnimationGroup(self, rsthis: &mut QParallelAnimationGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QParallelAnimationGroupD0Ev()};
     unsafe {_ZN23QParallelAnimationGroupD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

impl /*struct*/ QParallelAnimationGroup {
  pub fn NewQParallelAnimationGroup<T: QParallelAnimationGroup_NewQParallelAnimationGroup>(value: T) -> QParallelAnimationGroup {
    let rsthis = value.NewQParallelAnimationGroup();
    return rsthis;
    // return 1;
  }
}

pub trait QParallelAnimationGroup_NewQParallelAnimationGroup {
  fn NewQParallelAnimationGroup(self) -> QParallelAnimationGroup;
}

// proto: void QParallelAnimationGroup::NewQParallelAnimationGroup(const QParallelAnimationGroup & );
impl<'a> /*trait*/ QParallelAnimationGroup_NewQParallelAnimationGroup for (&'a  QParallelAnimationGroup) {
  fn NewQParallelAnimationGroup(self) -> QParallelAnimationGroup {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QParallelAnimationGroupC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN23QParallelAnimationGroupC1ERKS_(qthis, arg0)};
    let rsthis = QParallelAnimationGroup{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QParallelAnimationGroup {
  pub fn duration<RetType, T: QParallelAnimationGroup_duration<RetType>>(&mut self, value: T) -> RetType {
    return value.duration(self);
    // return 1;
  }
}

pub trait QParallelAnimationGroup_duration<RetType> {
  fn duration(self, rsthis: &mut QParallelAnimationGroup) -> RetType;
}

// proto:  int QParallelAnimationGroup::duration();
impl<'a> /*trait*/ QParallelAnimationGroup_duration<i32> for () {
  fn duration(self, rsthis: &mut QParallelAnimationGroup) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QParallelAnimationGroup8durationEv()};
    let mut ret = unsafe {_ZNK23QParallelAnimationGroup8durationEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

// proto: void QParallelAnimationGroup::NewQParallelAnimationGroup(QObject * parent);
impl<'a> /*trait*/ QParallelAnimationGroup_NewQParallelAnimationGroup for (&'a mut QObject) {
  fn NewQParallelAnimationGroup(self) -> QParallelAnimationGroup {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN23QParallelAnimationGroupC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN23QParallelAnimationGroupC1EP7QObject(qthis, arg0)};
    let rsthis = QParallelAnimationGroup{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QParallelAnimationGroup {
  pub fn metaObject<RetType, T: QParallelAnimationGroup_metaObject<RetType>>(&mut self, value: T) -> RetType {
    return value.metaObject(self);
    // return 1;
  }
}

pub trait QParallelAnimationGroup_metaObject<RetType> {
  fn metaObject(self, rsthis: &mut QParallelAnimationGroup) -> RetType;
}

// proto:  const QMetaObject * QParallelAnimationGroup::metaObject();
impl<'a> /*trait*/ QParallelAnimationGroup_metaObject<()> for () {
  fn metaObject(self, rsthis: &mut QParallelAnimationGroup) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK23QParallelAnimationGroup10metaObjectEv()};
     unsafe {_ZNK23QParallelAnimationGroup10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

