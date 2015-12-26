// auto generated, do not modify.
// created: Sat Dec 26 12:15:38 2015
// src-file: /QtWidgets/qfontdialog.h
// dst-file: /src/widgets/qfontdialog.rs
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
use super::qdialog::QDialog; // 773
use std::ops::Deref;
use super::qwidget::QWidget; // 773
use super::super::gui::qfont::QFont; // 771
use super::super::core::qobject::QObject; // 771
use super::super::core::qstring::QString; // 771
// <= use block end

// ext block begin =>
// #[link(name = "Qt5Core")]
// #[link(name = "Qt5Gui")]
// #[link(name = "Qt5Widgets")]
// #[link(name = "QtInline")]

extern {
  fn QFontDialog_Class_Size() -> c_int;
  // proto:  void QFontDialog::QFontDialog(QWidget * parent);
  fn dector_ZN11QFontDialogC1EP7QWidget(arg0: *mut c_void) -> *mut c_void;
  fn _ZN11QFontDialogC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QFontDialog::QFontDialog(const QFontDialog & );
  fn dector_ZN11QFontDialogC1ERKS_(arg0: *mut c_void) -> *mut c_void;
  fn _ZN11QFontDialogC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QFontDialog::currentFontChanged(const QFont & font);
  fn _ZN11QFontDialog18currentFontChangedERK5QFont(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QFontDialog::open(QObject * receiver, const char * member);
  fn _ZN11QFontDialog4openEP7QObjectPKc(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_char);
  // proto:  void QFontDialog::QFontDialog(const QFont & initial, QWidget * parent);
  fn dector_ZN11QFontDialogC1ERK5QFontP7QWidget(arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  fn _ZN11QFontDialogC1ERK5QFontP7QWidget(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  QFont QFontDialog::currentFont();
  fn _ZNK11QFontDialog11currentFontEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QFontDialog::setVisible(bool visible);
  fn _ZN11QFontDialog10setVisibleEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QFontDialog::~QFontDialog();
  fn _ZN11QFontDialogD0Ev(qthis: *mut c_void);
  // proto:  void QFontDialog::fontSelected(const QFont & font);
  fn _ZN11QFontDialog12fontSelectedERK5QFont(qthis: *mut c_void, arg0: *mut c_void);
  // proto: static QFont QFontDialog::getFont(bool * ok, QWidget * parent);
  fn _ZN11QFontDialog7getFontEPbP7QWidget(arg0: *mut c_char, arg1: *mut c_void) -> *mut c_void;
  // proto:  const QMetaObject * QFontDialog::metaObject();
  fn _ZNK11QFontDialog10metaObjectEv(qthis: *mut c_void);
  // proto:  QFont QFontDialog::selectedFont();
  fn _ZNK11QFontDialog12selectedFontEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QFontDialog::setCurrentFont(const QFont & font);
  fn _ZN11QFontDialog14setCurrentFontERK5QFont(qthis: *mut c_void, arg0: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QFontDialog)=1
pub struct QFontDialog {
  qbase: QDialog,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QFontDialog {
  pub fn inheritFrom(qthis: *mut c_void) -> QFontDialog {
    return QFontDialog{qbase: QDialog::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QFontDialog {
  type Target = QDialog;

  fn deref(&self) -> &QDialog {
    return & self.qbase;
  }
}
impl AsRef<QDialog> for QFontDialog {
  fn as_ref(& self) -> & QDialog {
    return & self.qbase;
  }
}
  // proto:  void QFontDialog::QFontDialog(QWidget * parent);
impl /*struct*/ QFontDialog {
  pub fn New<T: QFontDialog_New>(value: T) -> QFontDialog {
    let rsthis = value.New();
    return rsthis;
    // return 1;
  }
}

pub trait QFontDialog_New {
  fn New(self) -> QFontDialog;
}

  // proto:  void QFontDialog::QFontDialog(QWidget * parent);
impl<'a> /*trait*/ QFontDialog_New for (&'a QWidget) {
  fn New(self) -> QFontDialog {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFontDialogC1EP7QWidget()};
    let ctysz: c_int = unsafe{QFontDialog_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN11QFontDialogC1EP7QWidget(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN11QFontDialogC1EP7QWidget(arg0)};
    let rsthis = QFontDialog{/**/qbase: QDialog::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QFontDialog::QFontDialog(const QFontDialog & );
impl<'a> /*trait*/ QFontDialog_New for (&'a QFontDialog) {
  fn New(self) -> QFontDialog {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFontDialogC1ERKS_()};
    let ctysz: c_int = unsafe{QFontDialog_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.qclsinst  as *mut c_void;
    // unsafe {_ZN11QFontDialogC1ERKS_(qthis, arg0)};
    let qthis: *mut c_void = unsafe {dector_ZN11QFontDialogC1ERKS_(arg0)};
    let rsthis = QFontDialog{/**/qbase: QDialog::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QFontDialog::currentFontChanged(const QFont & font);
impl /*struct*/ QFontDialog {
  pub fn currentFontChanged<RetType, T: QFontDialog_currentFontChanged<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.currentFontChanged(self);
    // return 1;
  }
}

pub trait QFontDialog_currentFontChanged<RetType> {
  fn currentFontChanged(self , rsthis: & QFontDialog) -> RetType;
}

  // proto:  void QFontDialog::currentFontChanged(const QFont & font);
impl<'a> /*trait*/ QFontDialog_currentFontChanged<()> for (&'a QFont) {
  fn currentFontChanged(self , rsthis: & QFontDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFontDialog18currentFontChangedERK5QFont()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QFontDialog18currentFontChangedERK5QFont(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QFontDialog::open(QObject * receiver, const char * member);
impl /*struct*/ QFontDialog {
  pub fn open<RetType, T: QFontDialog_open<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.open(self);
    // return 1;
  }
}

pub trait QFontDialog_open<RetType> {
  fn open(self , rsthis: & QFontDialog) -> RetType;
}

  // proto:  void QFontDialog::open(QObject * receiver, const char * member);
impl<'a> /*trait*/ QFontDialog_open<()> for (&'a QObject, &'a  String) {
  fn open(self , rsthis: & QFontDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFontDialog4openEP7QObjectPKc()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.as_ptr()  as *mut c_char;
     unsafe {_ZN11QFontDialog4openEP7QObjectPKc(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QFontDialog::QFontDialog(const QFont & initial, QWidget * parent);
impl<'a> /*trait*/ QFontDialog_New for (&'a QFont, &'a QWidget) {
  fn New(self) -> QFontDialog {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFontDialogC1ERK5QFontP7QWidget()};
    let ctysz: c_int = unsafe{QFontDialog_Class_Size()};
    let qthis_ph: *mut c_void = unsafe{calloc(1, ctysz as usize)};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    // unsafe {_ZN11QFontDialogC1ERK5QFontP7QWidget(qthis, arg0, arg1)};
    let qthis: *mut c_void = unsafe {dector_ZN11QFontDialogC1ERK5QFontP7QWidget(arg0, arg1)};
    let rsthis = QFontDialog{/**/qbase: QDialog::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QFont QFontDialog::currentFont();
impl /*struct*/ QFontDialog {
  pub fn currentFont<RetType, T: QFontDialog_currentFont<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.currentFont(self);
    // return 1;
  }
}

pub trait QFontDialog_currentFont<RetType> {
  fn currentFont(self , rsthis: & QFontDialog) -> RetType;
}

  // proto:  QFont QFontDialog::currentFont();
impl<'a> /*trait*/ QFontDialog_currentFont<QFont> for () {
  fn currentFont(self , rsthis: & QFontDialog) -> QFont {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFontDialog11currentFontEv()};
    let mut ret = unsafe {_ZNK11QFontDialog11currentFontEv(rsthis.qclsinst)};
    let mut ret1 = QFont::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QFontDialog::setVisible(bool visible);
impl /*struct*/ QFontDialog {
  pub fn setVisible<RetType, T: QFontDialog_setVisible<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setVisible(self);
    // return 1;
  }
}

pub trait QFontDialog_setVisible<RetType> {
  fn setVisible(self , rsthis: & QFontDialog) -> RetType;
}

  // proto:  void QFontDialog::setVisible(bool visible);
impl<'a> /*trait*/ QFontDialog_setVisible<()> for (i8) {
  fn setVisible(self , rsthis: & QFontDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFontDialog10setVisibleEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN11QFontDialog10setVisibleEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QFontDialog::~QFontDialog();
impl /*struct*/ QFontDialog {
  pub fn Free<RetType, T: QFontDialog_Free<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.Free(self);
    // return 1;
  }
}

pub trait QFontDialog_Free<RetType> {
  fn Free(self , rsthis: & QFontDialog) -> RetType;
}

  // proto:  void QFontDialog::~QFontDialog();
impl<'a> /*trait*/ QFontDialog_Free<()> for () {
  fn Free(self , rsthis: & QFontDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFontDialogD0Ev()};
     unsafe {_ZN11QFontDialogD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QFontDialog::fontSelected(const QFont & font);
impl /*struct*/ QFontDialog {
  pub fn fontSelected<RetType, T: QFontDialog_fontSelected<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.fontSelected(self);
    // return 1;
  }
}

pub trait QFontDialog_fontSelected<RetType> {
  fn fontSelected(self , rsthis: & QFontDialog) -> RetType;
}

  // proto:  void QFontDialog::fontSelected(const QFont & font);
impl<'a> /*trait*/ QFontDialog_fontSelected<()> for (&'a QFont) {
  fn fontSelected(self , rsthis: & QFontDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFontDialog12fontSelectedERK5QFont()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QFontDialog12fontSelectedERK5QFont(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto: static QFont QFontDialog::getFont(bool * ok, QWidget * parent);
impl /*struct*/ QFontDialog {
  pub fn getFont_s<RetType, T: QFontDialog_getFont_s<RetType>>( overload_args: T) -> RetType {
    return overload_args.getFont_s();
    // return 1;
  }
}

pub trait QFontDialog_getFont_s<RetType> {
  fn getFont_s(self ) -> RetType;
}

  // proto: static QFont QFontDialog::getFont(bool * ok, QWidget * parent);
impl<'a> /*trait*/ QFontDialog_getFont_s<QFont> for (&'a mut Vec<i8>, &'a QWidget) {
  fn getFont_s(self ) -> QFont {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFontDialog7getFontEPbP7QWidget()};
    let arg0 = self.0.as_ptr()  as *mut c_char;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN11QFontDialog7getFontEPbP7QWidget(arg0, arg1)};
    let mut ret1 = QFont::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  const QMetaObject * QFontDialog::metaObject();
impl /*struct*/ QFontDialog {
  pub fn metaObject<RetType, T: QFontDialog_metaObject<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QFontDialog_metaObject<RetType> {
  fn metaObject(self , rsthis: & QFontDialog) -> RetType;
}

  // proto:  const QMetaObject * QFontDialog::metaObject();
impl<'a> /*trait*/ QFontDialog_metaObject<()> for () {
  fn metaObject(self , rsthis: & QFontDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFontDialog10metaObjectEv()};
     unsafe {_ZNK11QFontDialog10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QFont QFontDialog::selectedFont();
impl /*struct*/ QFontDialog {
  pub fn selectedFont<RetType, T: QFontDialog_selectedFont<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.selectedFont(self);
    // return 1;
  }
}

pub trait QFontDialog_selectedFont<RetType> {
  fn selectedFont(self , rsthis: & QFontDialog) -> RetType;
}

  // proto:  QFont QFontDialog::selectedFont();
impl<'a> /*trait*/ QFontDialog_selectedFont<QFont> for () {
  fn selectedFont(self , rsthis: & QFontDialog) -> QFont {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QFontDialog12selectedFontEv()};
    let mut ret = unsafe {_ZNK11QFontDialog12selectedFontEv(rsthis.qclsinst)};
    let mut ret1 = QFont::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QFontDialog::setCurrentFont(const QFont & font);
impl /*struct*/ QFontDialog {
  pub fn setCurrentFont<RetType, T: QFontDialog_setCurrentFont<RetType>>(& self,  overload_args: T) -> RetType {
    return overload_args.setCurrentFont(self);
    // return 1;
  }
}

pub trait QFontDialog_setCurrentFont<RetType> {
  fn setCurrentFont(self , rsthis: & QFontDialog) -> RetType;
}

  // proto:  void QFontDialog::setCurrentFont(const QFont & font);
impl<'a> /*trait*/ QFontDialog_setCurrentFont<()> for (&'a QFont) {
  fn setCurrentFont(self , rsthis: & QFontDialog) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QFontDialog14setCurrentFontERK5QFont()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QFontDialog14setCurrentFontERK5QFont(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// <= body block end

