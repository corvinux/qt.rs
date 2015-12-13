// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qreadwritelock::QReadWriteLock;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  fn _ZNK11QReadLocker13readWriteLockEv() -> i32;
  fn _ZN11QReadLockerD0Ev() -> i32;
  fn _ZN11QReadLockerC1EP14QReadWriteLock(qthis: *mut c_void, arg0: *mut c_void) -> i32;
  fn _ZN11QReadLockerC1ERKS_(qthis: *mut c_void, arg0: *const c_void) -> i32;
  fn _ZN11QReadLocker6relockEv() -> i32;
  fn _ZN11QReadLocker6unlockEv() -> i32;
}

// body block begin
// class sizeof(QReadLocker)=4
pub struct QReadLocker {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QReadLocker {
  pub fn readWriteLock<T: QReadLocker_readWriteLock>(&mut self, value: T) -> i32 {
    value.readWriteLock(self);
    return 1;
  }
}

pub trait QReadLocker_readWriteLock {
  fn readWriteLock(self, this: &mut QReadLocker) -> i32;
}

// proto: QReadWriteLock * QReadLocker::readWriteLock();
impl<'a> /*trait*/ QReadLocker_readWriteLock for () {
  fn readWriteLock(self, this: &mut QReadLocker) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QReadLocker13readWriteLockEv()};
    unsafe {_ZNK11QReadLocker13readWriteLockEv()};
    return 1;
  }
}

impl /*struct*/ QReadLocker {
  pub fn FreeQReadLocker<T: QReadLocker_FreeQReadLocker>(&mut self, value: T) -> i32 {
    value.FreeQReadLocker(self);
    return 1;
  }
}

pub trait QReadLocker_FreeQReadLocker {
  fn FreeQReadLocker(self, this: &mut QReadLocker) -> i32;
}

// proto: void QReadLocker::FreeQReadLocker();
impl<'a> /*trait*/ QReadLocker_FreeQReadLocker for () {
  fn FreeQReadLocker(self, this: &mut QReadLocker) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QReadLockerD0Ev()};
    unsafe {_ZN11QReadLockerD0Ev()};
    return 1;
  }
}

impl /*struct*/ QReadLocker {
  pub fn NewQReadLocker<T: QReadLocker_NewQReadLocker>(value: T) -> QReadLocker {
    let rsthis = value.NewQReadLocker();
    return rsthis;
    // return 1;
  }
}

pub trait QReadLocker_NewQReadLocker {
  fn NewQReadLocker(self) -> QReadLocker;
}

// proto: void QReadLocker::NewQReadLocker(QReadWriteLock * readWriteLock);
impl<'a> /*trait*/ QReadLocker_NewQReadLocker for (&'a mut QReadWriteLock) {
  fn NewQReadLocker(self) -> QReadLocker {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QReadLockerC1EP14QReadWriteLock()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QReadLockerC1EP14QReadWriteLock(qthis, arg0)};
    let rsthis = QReadLocker{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto: void QReadLocker::NewQReadLocker(const QReadLocker & );
impl<'a> /*trait*/ QReadLocker_NewQReadLocker for (&'a  QReadLocker) {
  fn NewQReadLocker(self) -> QReadLocker {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QReadLockerC1ERKS_()};
    let arg0 = self.qclsinst  as *const c_void;
    unsafe {_ZN11QReadLockerC1ERKS_(qthis, arg0)};
    let rsthis = QReadLocker{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

impl /*struct*/ QReadLocker {
  pub fn relock<T: QReadLocker_relock>(&mut self, value: T) -> i32 {
    value.relock(self);
    return 1;
  }
}

pub trait QReadLocker_relock {
  fn relock(self, this: &mut QReadLocker) -> i32;
}

// proto: void QReadLocker::relock();
impl<'a> /*trait*/ QReadLocker_relock for () {
  fn relock(self, this: &mut QReadLocker) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QReadLocker6relockEv()};
    unsafe {_ZN11QReadLocker6relockEv()};
    return 1;
  }
}

impl /*struct*/ QReadLocker {
  pub fn unlock<T: QReadLocker_unlock>(&mut self, value: T) -> i32 {
    value.unlock(self);
    return 1;
  }
}

pub trait QReadLocker_unlock {
  fn unlock(self, this: &mut QReadLocker) -> i32;
}

// proto: void QReadLocker::unlock();
impl<'a> /*trait*/ QReadLocker_unlock for () {
  fn unlock(self, this: &mut QReadLocker) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QReadLocker6unlockEv()};
    unsafe {_ZN11QReadLocker6unlockEv()};
    return 1;
  }
}

