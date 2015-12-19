// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qwidget::QWidget;
use super::qsize::QSize;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  void QScrollArea::NewQScrollArea(QWidget * parent);
  fn _ZN11QScrollAreaC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QScrollArea::setWidgetResizable(bool resizable);
  fn _ZN11QScrollArea18setWidgetResizableEb(qthis: *mut c_void, arg0: int8_t) ;
  // proto:  void QScrollArea::NewQScrollArea(const QScrollArea & );
  fn _ZN11QScrollAreaC1ERKS_(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  void QScrollArea::setWidget(QWidget * widget);
  fn _ZN11QScrollArea9setWidgetEP7QWidget(qthis: *mut c_void, arg0: *mut c_void) ;
  // proto:  QWidget * QScrollArea::takeWidget();
  fn _ZN11QScrollArea10takeWidgetEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QScrollArea::ensureVisible(int x, int y, int xmargin, int ymargin);
  fn _ZN11QScrollArea13ensureVisibleEiiii(qthis: *mut c_void, arg0: c_int, arg1: c_int, arg2: c_int, arg3: c_int) ;
  // proto:  void QScrollArea::ensureWidgetVisible(QWidget * childWidget, int xmargin, int ymargin);
  fn _ZN11QScrollArea19ensureWidgetVisibleEP7QWidgetii(qthis: *mut c_void, arg0: *mut c_void, arg1: c_int, arg2: c_int) ;
  // proto:  QWidget * QScrollArea::widget();
  fn _ZNK11QScrollArea6widgetEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  QSize QScrollArea::sizeHint();
  fn _ZNK11QScrollArea8sizeHintEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  bool QScrollArea::widgetResizable();
  fn _ZNK11QScrollArea15widgetResizableEv(qthis: *mut c_void) -> int8_t;
  // proto:  void QScrollArea::FreeQScrollArea();
  fn _ZN11QScrollAreaD0Ev(qthis: *mut c_void) ;
  // proto:  bool QScrollArea::focusNextPrevChild(bool next);
  fn _ZN11QScrollArea18focusNextPrevChildEb(qthis: *mut c_void, arg0: int8_t) -> int8_t;
  // proto:  const QMetaObject * QScrollArea::metaObject();
  fn _ZNK11QScrollArea10metaObjectEv(qthis: *mut c_void) ;
}

// body block begin
// class sizeof(QScrollArea)=1
pub struct QScrollArea {
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QScrollArea {
  pub fn NewQScrollArea<T: QScrollArea_NewQScrollArea>(value: T) -> QScrollArea {
    let rsthis = value.NewQScrollArea();
    return rsthis;
    // return 1;
  }
}

pub trait QScrollArea_NewQScrollArea {
  fn NewQScrollArea(self) -> QScrollArea;
}

// proto: void QScrollArea::NewQScrollArea(QWidget * parent);
impl<'a> /*trait*/ QScrollArea_NewQScrollArea for (&'a mut QWidget) {
  fn NewQScrollArea(self) -> QScrollArea {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QScrollAreaC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QScrollAreaC1EP7QWidget(qthis, arg0)};
    let rsthis = QScrollArea{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  void QScrollArea::setWidgetResizable(bool resizable);
impl /*struct*/ QScrollArea {
  pub fn setWidgetResizable<RetType, T: QScrollArea_setWidgetResizable<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setWidgetResizable(self);
    // return 1;
  }
}

pub trait QScrollArea_setWidgetResizable<RetType> {
  fn setWidgetResizable(self , rsthis: &mut QScrollArea) -> RetType;
}

// proto:  void QScrollArea::setWidgetResizable(bool resizable);
impl<'a> /*trait*/ QScrollArea_setWidgetResizable<()> for (i8) {
  fn setWidgetResizable(self , rsthis: &mut QScrollArea) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QScrollArea18setWidgetResizableEb()};
    let arg0 = self  as int8_t;
     unsafe {_ZN11QScrollArea18setWidgetResizableEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto: void QScrollArea::NewQScrollArea(const QScrollArea & );
impl<'a> /*trait*/ QScrollArea_NewQScrollArea for (&'a  QScrollArea) {
  fn NewQScrollArea(self) -> QScrollArea {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QScrollAreaC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN11QScrollAreaC1ERKS_(qthis, arg0)};
    let rsthis = QScrollArea{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

// proto:  void QScrollArea::setWidget(QWidget * widget);
impl /*struct*/ QScrollArea {
  pub fn setWidget<RetType, T: QScrollArea_setWidget<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.setWidget(self);
    // return 1;
  }
}

pub trait QScrollArea_setWidget<RetType> {
  fn setWidget(self , rsthis: &mut QScrollArea) -> RetType;
}

// proto:  void QScrollArea::setWidget(QWidget * widget);
impl<'a> /*trait*/ QScrollArea_setWidget<()> for (&'a mut QWidget) {
  fn setWidget(self , rsthis: &mut QScrollArea) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QScrollArea9setWidgetEP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN11QScrollArea9setWidgetEP7QWidget(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

// proto:  QWidget * QScrollArea::takeWidget();
impl /*struct*/ QScrollArea {
  pub fn takeWidget<RetType, T: QScrollArea_takeWidget<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.takeWidget(self);
    // return 1;
  }
}

pub trait QScrollArea_takeWidget<RetType> {
  fn takeWidget(self , rsthis: &mut QScrollArea) -> RetType;
}

// proto:  QWidget * QScrollArea::takeWidget();
impl<'a> /*trait*/ QScrollArea_takeWidget<QWidget> for () {
  fn takeWidget(self , rsthis: &mut QScrollArea) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QScrollArea10takeWidgetEv()};
    let mut ret = unsafe {_ZN11QScrollArea10takeWidgetEv(rsthis.qclsinst)};
    let mut ret1 = QWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  void QScrollArea::ensureVisible(int x, int y, int xmargin, int ymargin);
impl /*struct*/ QScrollArea {
  pub fn ensureVisible<RetType, T: QScrollArea_ensureVisible<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.ensureVisible(self);
    // return 1;
  }
}

pub trait QScrollArea_ensureVisible<RetType> {
  fn ensureVisible(self , rsthis: &mut QScrollArea) -> RetType;
}

// proto:  void QScrollArea::ensureVisible(int x, int y, int xmargin, int ymargin);
impl<'a> /*trait*/ QScrollArea_ensureVisible<()> for (i32, i32, i32, i32) {
  fn ensureVisible(self , rsthis: &mut QScrollArea) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QScrollArea13ensureVisibleEiiii()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
    let arg3 = self.3  as c_int;
     unsafe {_ZN11QScrollArea13ensureVisibleEiiii(rsthis.qclsinst, arg0, arg1, arg2, arg3)};
    // return 1;
  }
}

// proto:  void QScrollArea::ensureWidgetVisible(QWidget * childWidget, int xmargin, int ymargin);
impl /*struct*/ QScrollArea {
  pub fn ensureWidgetVisible<RetType, T: QScrollArea_ensureWidgetVisible<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.ensureWidgetVisible(self);
    // return 1;
  }
}

pub trait QScrollArea_ensureWidgetVisible<RetType> {
  fn ensureWidgetVisible(self , rsthis: &mut QScrollArea) -> RetType;
}

// proto:  void QScrollArea::ensureWidgetVisible(QWidget * childWidget, int xmargin, int ymargin);
impl<'a> /*trait*/ QScrollArea_ensureWidgetVisible<()> for (&'a mut QWidget, i32, i32) {
  fn ensureWidgetVisible(self , rsthis: &mut QScrollArea) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QScrollArea19ensureWidgetVisibleEP7QWidgetii()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1  as c_int;
    let arg2 = self.2  as c_int;
     unsafe {_ZN11QScrollArea19ensureWidgetVisibleEP7QWidgetii(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

// proto:  QWidget * QScrollArea::widget();
impl /*struct*/ QScrollArea {
  pub fn widget<RetType, T: QScrollArea_widget<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.widget(self);
    // return 1;
  }
}

pub trait QScrollArea_widget<RetType> {
  fn widget(self , rsthis: &mut QScrollArea) -> RetType;
}

// proto:  QWidget * QScrollArea::widget();
impl<'a> /*trait*/ QScrollArea_widget<QWidget> for () {
  fn widget(self , rsthis: &mut QScrollArea) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QScrollArea6widgetEv()};
    let mut ret = unsafe {_ZNK11QScrollArea6widgetEv(rsthis.qclsinst)};
    let mut ret1 = QWidget{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  QSize QScrollArea::sizeHint();
impl /*struct*/ QScrollArea {
  pub fn sizeHint<RetType, T: QScrollArea_sizeHint<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.sizeHint(self);
    // return 1;
  }
}

pub trait QScrollArea_sizeHint<RetType> {
  fn sizeHint(self , rsthis: &mut QScrollArea) -> RetType;
}

// proto:  QSize QScrollArea::sizeHint();
impl<'a> /*trait*/ QScrollArea_sizeHint<QSize> for () {
  fn sizeHint(self , rsthis: &mut QScrollArea) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QScrollArea8sizeHintEv()};
    let mut ret = unsafe {_ZNK11QScrollArea8sizeHintEv(rsthis.qclsinst)};
    let mut ret1 = QSize{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

// proto:  bool QScrollArea::widgetResizable();
impl /*struct*/ QScrollArea {
  pub fn widgetResizable<RetType, T: QScrollArea_widgetResizable<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.widgetResizable(self);
    // return 1;
  }
}

pub trait QScrollArea_widgetResizable<RetType> {
  fn widgetResizable(self , rsthis: &mut QScrollArea) -> RetType;
}

// proto:  bool QScrollArea::widgetResizable();
impl<'a> /*trait*/ QScrollArea_widgetResizable<i8> for () {
  fn widgetResizable(self , rsthis: &mut QScrollArea) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QScrollArea15widgetResizableEv()};
    let mut ret = unsafe {_ZNK11QScrollArea15widgetResizableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

// proto:  void QScrollArea::FreeQScrollArea();
impl /*struct*/ QScrollArea {
  pub fn FreeQScrollArea<RetType, T: QScrollArea_FreeQScrollArea<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.FreeQScrollArea(self);
    // return 1;
  }
}

pub trait QScrollArea_FreeQScrollArea<RetType> {
  fn FreeQScrollArea(self , rsthis: &mut QScrollArea) -> RetType;
}

// proto:  void QScrollArea::FreeQScrollArea();
impl<'a> /*trait*/ QScrollArea_FreeQScrollArea<()> for () {
  fn FreeQScrollArea(self , rsthis: &mut QScrollArea) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QScrollAreaD0Ev()};
     unsafe {_ZN11QScrollAreaD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

// proto:  bool QScrollArea::focusNextPrevChild(bool next);
impl /*struct*/ QScrollArea {
  pub fn focusNextPrevChild<RetType, T: QScrollArea_focusNextPrevChild<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.focusNextPrevChild(self);
    // return 1;
  }
}

pub trait QScrollArea_focusNextPrevChild<RetType> {
  fn focusNextPrevChild(self , rsthis: &mut QScrollArea) -> RetType;
}

// proto:  bool QScrollArea::focusNextPrevChild(bool next);
impl<'a> /*trait*/ QScrollArea_focusNextPrevChild<i8> for (i8) {
  fn focusNextPrevChild(self , rsthis: &mut QScrollArea) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN11QScrollArea18focusNextPrevChildEb()};
    let arg0 = self  as int8_t;
    let mut ret = unsafe {_ZN11QScrollArea18focusNextPrevChildEb(rsthis.qclsinst, arg0)};
    return ret as i8;
    // return 1;
  }
}

// proto:  const QMetaObject * QScrollArea::metaObject();
impl /*struct*/ QScrollArea {
  pub fn metaObject<RetType, T: QScrollArea_metaObject<RetType>>(&mut self, overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QScrollArea_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QScrollArea) -> RetType;
}

// proto:  const QMetaObject * QScrollArea::metaObject();
impl<'a> /*trait*/ QScrollArea_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QScrollArea) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK11QScrollArea10metaObjectEv()};
     unsafe {_ZNK11QScrollArea10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

