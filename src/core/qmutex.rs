// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QMutex::~QMutex();
  fn _ZN6QMutexD0Ev(qthis: *mut c_void);
  // proto:  bool QMutex::tryLock(int timeout);
  fn _ZN6QMutex7tryLockEi(qthis: *mut c_void, arg0: c_int) -> c_char;
  // proto:  void QMutex::QMutex(const QMutex & );
  fn _ZN6QMutexC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QMutex::lock();
  fn _ZN6QMutex4lockEv(qthis: *mut c_void);
  // proto:  void QMutex::unlock();
  fn _ZN6QMutex6unlockEv(qthis: *mut c_void);
}

// body block begin
// class sizeof(QMutex)=1
pub struct QMutex {
  pub qclsinst: *mut c_void,
}

  // proto:  void QMutex::~QMutex();
impl /*struct*/ QMutex {
  pub fn FreeQMutex<RetType, T: QMutex_FreeQMutex<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQMutex(self);
    // return 1;
  }
}

pub trait QMutex_FreeQMutex<RetType> {
  fn FreeQMutex(self , rsthis: &mut QMutex) -> RetType;
}

  // proto:  void QMutex::~QMutex();
impl<'a> /*trait*/ QMutex_FreeQMutex<()> for () {
  fn FreeQMutex(self , rsthis: &mut QMutex) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMutexD0Ev()};
     unsafe {_ZN6QMutexD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QMutex::tryLock(int timeout);
impl /*struct*/ QMutex {
  pub fn tryLock<RetType, T: QMutex_tryLock<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.tryLock(self);
    // return 1;
  }
}

pub trait QMutex_tryLock<RetType> {
  fn tryLock(self , rsthis: &mut QMutex) -> RetType;
}

  // proto:  bool QMutex::tryLock(int timeout);
impl<'a> /*trait*/ QMutex_tryLock<i8> for (i32) {
  fn tryLock(self , rsthis: &mut QMutex) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMutex7tryLockEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZN6QMutex7tryLockEi(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QMutex::QMutex(const QMutex & );
impl /*struct*/ QMutex {
  pub fn NewQMutex<T: QMutex_NewQMutex>(value: T) -> QMutex {
    let rsthis = value.NewQMutex();
    return rsthis;
    // return 1;
  }
}

pub trait QMutex_NewQMutex {
  fn NewQMutex(self) -> QMutex;
}

  // proto:  void QMutex::QMutex(const QMutex & );
impl<'a> /*trait*/ QMutex_NewQMutex for (QMutex) {
  fn NewQMutex(self) -> QMutex {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMutexC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN6QMutexC1ERKS_(qthis, arg0)};
    let rsthis = QMutex{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QMutex::lock();
impl /*struct*/ QMutex {
  pub fn lock<RetType, T: QMutex_lock<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.lock(self);
    // return 1;
  }
}

pub trait QMutex_lock<RetType> {
  fn lock(self , rsthis: &mut QMutex) -> RetType;
}

  // proto:  void QMutex::lock();
impl<'a> /*trait*/ QMutex_lock<()> for () {
  fn lock(self , rsthis: &mut QMutex) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMutex4lockEv()};
     unsafe {_ZN6QMutex4lockEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QMutex::unlock();
impl /*struct*/ QMutex {
  pub fn unlock<RetType, T: QMutex_unlock<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.unlock(self);
    // return 1;
  }
}

pub trait QMutex_unlock<RetType> {
  fn unlock(self , rsthis: &mut QMutex) -> RetType;
}

  // proto:  void QMutex::unlock();
impl<'a> /*trait*/ QMutex_unlock<()> for () {
  fn unlock(self , rsthis: &mut QMutex) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN6QMutex6unlockEv()};
     unsafe {_ZN6QMutex6unlockEv(rsthis.qclsinst)};
    // return 1;
  }
}

