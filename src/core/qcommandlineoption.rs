// auto generated, do not modify.
// created: Mon Dec 21 22:54:38 2015
// src-file: /QtCore/qcommandlineoption.h
// dst-file: /src/core/qcommandlineoption.rs
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
use super::qstring::QString; // 773
use super::qstringlist::QStringList; // 773
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  void QCommandLineOption::setValueName(const QString & name);
  fn _ZN18QCommandLineOption12setValueNameERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QStringList QCommandLineOption::names();
  fn _ZNK18QCommandLineOption5namesEv(qthis: *mut c_void);
  // proto:  void QCommandLineOption::QCommandLineOption(const QCommandLineOption & other);
  fn _ZN18QCommandLineOptionC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QCommandLineOption::setDescription(const QString & description);
  fn _ZN18QCommandLineOption14setDescriptionERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QCommandLineOption::QCommandLineOption(const QString & name, const QString & description, const QString & valueName, const QString & defaultValue);
  fn _ZN18QCommandLineOptionC1ERK7QStringS2_S2_S2_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void, arg3: *mut c_void);
  // proto:  QString QCommandLineOption::valueName();
  fn _ZNK18QCommandLineOption9valueNameEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QCommandLineOption::QCommandLineOption(const QStringList & names, const QString & description, const QString & valueName, const QString & defaultValue);
  fn _ZN18QCommandLineOptionC1ERK11QStringListRK7QStringS5_S5_(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void, arg3: *mut c_void);
  // proto:  void QCommandLineOption::swap(QCommandLineOption & other);
  fn _ZN18QCommandLineOption4swapERS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QString QCommandLineOption::description();
  fn _ZNK18QCommandLineOption11descriptionEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QCommandLineOption::~QCommandLineOption();
  fn _ZN18QCommandLineOptionD0Ev(qthis: *mut c_void);
  // proto:  void QCommandLineOption::QCommandLineOption(const QStringList & names);
  fn _ZN18QCommandLineOptionC1ERK11QStringList(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QCommandLineOption::setDefaultValue(const QString & defaultValue);
  fn _ZN18QCommandLineOption15setDefaultValueERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QCommandLineOption::setDefaultValues(const QStringList & defaultValues);
  fn _ZN18QCommandLineOption16setDefaultValuesERK11QStringList(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QCommandLineOption::QCommandLineOption(const QString & name);
  fn _ZN18QCommandLineOptionC1ERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QStringList QCommandLineOption::defaultValues();
  fn _ZNK18QCommandLineOption13defaultValuesEv(qthis: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QCommandLineOption)=1
pub struct QCommandLineOption {
  pub qclsinst: *mut c_void,
}

  // proto:  void QCommandLineOption::setValueName(const QString & name);
impl /*struct*/ QCommandLineOption {
  pub fn setValueName<RetType, T: QCommandLineOption_setValueName<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setValueName(self);
    // return 1;
  }
}

pub trait QCommandLineOption_setValueName<RetType> {
  fn setValueName(self , rsthis: &mut QCommandLineOption) -> RetType;
}

  // proto:  void QCommandLineOption::setValueName(const QString & name);
impl<'a> /*trait*/ QCommandLineOption_setValueName<()> for (QString) {
  fn setValueName(self , rsthis: &mut QCommandLineOption) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCommandLineOption12setValueNameERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN18QCommandLineOption12setValueNameERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QStringList QCommandLineOption::names();
impl /*struct*/ QCommandLineOption {
  pub fn names<RetType, T: QCommandLineOption_names<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.names(self);
    // return 1;
  }
}

pub trait QCommandLineOption_names<RetType> {
  fn names(self , rsthis: &mut QCommandLineOption) -> RetType;
}

  // proto:  QStringList QCommandLineOption::names();
impl<'a> /*trait*/ QCommandLineOption_names<()> for () {
  fn names(self , rsthis: &mut QCommandLineOption) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QCommandLineOption5namesEv()};
     unsafe {_ZNK18QCommandLineOption5namesEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QCommandLineOption::QCommandLineOption(const QCommandLineOption & other);
impl /*struct*/ QCommandLineOption {
  pub fn NewQCommandLineOption<T: QCommandLineOption_NewQCommandLineOption>(value: T) -> QCommandLineOption {
    let rsthis = value.NewQCommandLineOption();
    return rsthis;
    // return 1;
  }
}

pub trait QCommandLineOption_NewQCommandLineOption {
  fn NewQCommandLineOption(self) -> QCommandLineOption;
}

  // proto:  void QCommandLineOption::QCommandLineOption(const QCommandLineOption & other);
impl<'a> /*trait*/ QCommandLineOption_NewQCommandLineOption for (QCommandLineOption) {
  fn NewQCommandLineOption(self) -> QCommandLineOption {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCommandLineOptionC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN18QCommandLineOptionC1ERKS_(qthis, arg0)};
    let rsthis = QCommandLineOption{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QCommandLineOption::setDescription(const QString & description);
impl /*struct*/ QCommandLineOption {
  pub fn setDescription<RetType, T: QCommandLineOption_setDescription<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setDescription(self);
    // return 1;
  }
}

pub trait QCommandLineOption_setDescription<RetType> {
  fn setDescription(self , rsthis: &mut QCommandLineOption) -> RetType;
}

  // proto:  void QCommandLineOption::setDescription(const QString & description);
impl<'a> /*trait*/ QCommandLineOption_setDescription<()> for (QString) {
  fn setDescription(self , rsthis: &mut QCommandLineOption) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCommandLineOption14setDescriptionERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN18QCommandLineOption14setDescriptionERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QCommandLineOption::QCommandLineOption(const QString & name, const QString & description, const QString & valueName, const QString & defaultValue);
impl<'a> /*trait*/ QCommandLineOption_NewQCommandLineOption for (QString, QString, QString, QString) {
  fn NewQCommandLineOption(self) -> QCommandLineOption {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCommandLineOptionC1ERK7QStringS2_S2_S2_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3.qclsinst  as *mut c_void;
    unsafe {_ZN18QCommandLineOptionC1ERK7QStringS2_S2_S2_(qthis, arg0, arg1, arg2, arg3)};
    let rsthis = QCommandLineOption{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QString QCommandLineOption::valueName();
impl /*struct*/ QCommandLineOption {
  pub fn valueName<RetType, T: QCommandLineOption_valueName<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.valueName(self);
    // return 1;
  }
}

pub trait QCommandLineOption_valueName<RetType> {
  fn valueName(self , rsthis: &mut QCommandLineOption) -> RetType;
}

  // proto:  QString QCommandLineOption::valueName();
impl<'a> /*trait*/ QCommandLineOption_valueName<QString> for () {
  fn valueName(self , rsthis: &mut QCommandLineOption) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QCommandLineOption9valueNameEv()};
    let mut ret = unsafe {_ZNK18QCommandLineOption9valueNameEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QCommandLineOption::QCommandLineOption(const QStringList & names, const QString & description, const QString & valueName, const QString & defaultValue);
impl<'a> /*trait*/ QCommandLineOption_NewQCommandLineOption for (QStringList, QString, QString, QString) {
  fn NewQCommandLineOption(self) -> QCommandLineOption {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCommandLineOptionC1ERK11QStringListRK7QStringS5_S5_()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let arg3 = self.3.qclsinst  as *mut c_void;
    unsafe {_ZN18QCommandLineOptionC1ERK11QStringListRK7QStringS5_S5_(qthis, arg0, arg1, arg2, arg3)};
    let rsthis = QCommandLineOption{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QCommandLineOption::swap(QCommandLineOption & other);
impl /*struct*/ QCommandLineOption {
  pub fn swap<RetType, T: QCommandLineOption_swap<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.swap(self);
    // return 1;
  }
}

pub trait QCommandLineOption_swap<RetType> {
  fn swap(self , rsthis: &mut QCommandLineOption) -> RetType;
}

  // proto:  void QCommandLineOption::swap(QCommandLineOption & other);
impl<'a> /*trait*/ QCommandLineOption_swap<()> for (QCommandLineOption) {
  fn swap(self , rsthis: &mut QCommandLineOption) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCommandLineOption4swapERS_()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN18QCommandLineOption4swapERS_(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QCommandLineOption::description();
impl /*struct*/ QCommandLineOption {
  pub fn description<RetType, T: QCommandLineOption_description<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.description(self);
    // return 1;
  }
}

pub trait QCommandLineOption_description<RetType> {
  fn description(self , rsthis: &mut QCommandLineOption) -> RetType;
}

  // proto:  QString QCommandLineOption::description();
impl<'a> /*trait*/ QCommandLineOption_description<QString> for () {
  fn description(self , rsthis: &mut QCommandLineOption) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QCommandLineOption11descriptionEv()};
    let mut ret = unsafe {_ZNK18QCommandLineOption11descriptionEv(rsthis.qclsinst)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QCommandLineOption::~QCommandLineOption();
impl /*struct*/ QCommandLineOption {
  pub fn FreeQCommandLineOption<RetType, T: QCommandLineOption_FreeQCommandLineOption<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQCommandLineOption(self);
    // return 1;
  }
}

pub trait QCommandLineOption_FreeQCommandLineOption<RetType> {
  fn FreeQCommandLineOption(self , rsthis: &mut QCommandLineOption) -> RetType;
}

  // proto:  void QCommandLineOption::~QCommandLineOption();
impl<'a> /*trait*/ QCommandLineOption_FreeQCommandLineOption<()> for () {
  fn FreeQCommandLineOption(self , rsthis: &mut QCommandLineOption) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCommandLineOptionD0Ev()};
     unsafe {_ZN18QCommandLineOptionD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QCommandLineOption::QCommandLineOption(const QStringList & names);
impl<'a> /*trait*/ QCommandLineOption_NewQCommandLineOption for (QStringList) {
  fn NewQCommandLineOption(self) -> QCommandLineOption {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCommandLineOptionC1ERK11QStringList()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN18QCommandLineOptionC1ERK11QStringList(qthis, arg0)};
    let rsthis = QCommandLineOption{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QCommandLineOption::setDefaultValue(const QString & defaultValue);
impl /*struct*/ QCommandLineOption {
  pub fn setDefaultValue<RetType, T: QCommandLineOption_setDefaultValue<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setDefaultValue(self);
    // return 1;
  }
}

pub trait QCommandLineOption_setDefaultValue<RetType> {
  fn setDefaultValue(self , rsthis: &mut QCommandLineOption) -> RetType;
}

  // proto:  void QCommandLineOption::setDefaultValue(const QString & defaultValue);
impl<'a> /*trait*/ QCommandLineOption_setDefaultValue<()> for (QString) {
  fn setDefaultValue(self , rsthis: &mut QCommandLineOption) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCommandLineOption15setDefaultValueERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN18QCommandLineOption15setDefaultValueERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QCommandLineOption::setDefaultValues(const QStringList & defaultValues);
impl /*struct*/ QCommandLineOption {
  pub fn setDefaultValues<RetType, T: QCommandLineOption_setDefaultValues<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setDefaultValues(self);
    // return 1;
  }
}

pub trait QCommandLineOption_setDefaultValues<RetType> {
  fn setDefaultValues(self , rsthis: &mut QCommandLineOption) -> RetType;
}

  // proto:  void QCommandLineOption::setDefaultValues(const QStringList & defaultValues);
impl<'a> /*trait*/ QCommandLineOption_setDefaultValues<()> for (QStringList) {
  fn setDefaultValues(self , rsthis: &mut QCommandLineOption) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCommandLineOption16setDefaultValuesERK11QStringList()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN18QCommandLineOption16setDefaultValuesERK11QStringList(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QCommandLineOption::QCommandLineOption(const QString & name);
impl<'a> /*trait*/ QCommandLineOption_NewQCommandLineOption for (QString) {
  fn NewQCommandLineOption(self) -> QCommandLineOption {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN18QCommandLineOptionC1ERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN18QCommandLineOptionC1ERK7QString(qthis, arg0)};
    let rsthis = QCommandLineOption{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QStringList QCommandLineOption::defaultValues();
impl /*struct*/ QCommandLineOption {
  pub fn defaultValues<RetType, T: QCommandLineOption_defaultValues<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.defaultValues(self);
    // return 1;
  }
}

pub trait QCommandLineOption_defaultValues<RetType> {
  fn defaultValues(self , rsthis: &mut QCommandLineOption) -> RetType;
}

  // proto:  QStringList QCommandLineOption::defaultValues();
impl<'a> /*trait*/ QCommandLineOption_defaultValues<()> for () {
  fn defaultValues(self , rsthis: &mut QCommandLineOption) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK18QCommandLineOption13defaultValuesEv()};
     unsafe {_ZNK18QCommandLineOption13defaultValuesEv(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

