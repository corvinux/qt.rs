// auto generated, do not modify.
// created: Tue Dec 22 23:21:28 2015
// src-file: /QtWidgets/qstyleditemdelegate.h
// dst-file: /src/widgets/qstyleditemdelegate.rs
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
use super::qabstractitemdelegate::QAbstractItemDelegate; // 773
use std::ops::Deref;
use super::qstyleoption::QStyleOptionViewItem; // 773
use super::super::core::qabstractitemmodel::QModelIndex; // 771
use super::super::core::qsize::QSize; // 771
use super::qwidget::QWidget; // 773
use super::qitemeditorfactory::QItemEditorFactory; // 773
use super::super::gui::qpainter::QPainter; // 771
use super::super::core::qobject::QObject; // 771
use super::super::core::qvariant::QVariant; // 771
use super::super::core::qlocale::QLocale; // 771
use super::super::core::qstring::QString; // 771
// <= use block end

// ext block begin =>
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]

extern {
  // proto:  QSize QStyledItemDelegate::sizeHint(const QStyleOptionViewItem & option, const QModelIndex & index);
  fn _ZNK19QStyledItemDelegate8sizeHintERK20QStyleOptionViewItemRK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  QWidget * QStyledItemDelegate::createEditor(QWidget * parent, const QStyleOptionViewItem & option, const QModelIndex & index);
  fn _ZNK19QStyledItemDelegate12createEditorEP7QWidgetRK20QStyleOptionViewItemRK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void) -> *mut c_void;
  // proto:  void QStyledItemDelegate::~QStyledItemDelegate();
  fn _ZN19QStyledItemDelegateD0Ev(qthis: *mut c_void);
  // proto:  void QStyledItemDelegate::updateEditorGeometry(QWidget * editor, const QStyleOptionViewItem & option, const QModelIndex & index);
  fn _ZNK19QStyledItemDelegate20updateEditorGeometryEP7QWidgetRK20QStyleOptionViewItemRK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  void QStyledItemDelegate::setEditorData(QWidget * editor, const QModelIndex & index);
  fn _ZNK19QStyledItemDelegate13setEditorDataEP7QWidgetRK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void);
  // proto:  void QStyledItemDelegate::QStyledItemDelegate(const QStyledItemDelegate & );
  fn _ZN19QStyledItemDelegateC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QStyledItemDelegate::setItemEditorFactory(QItemEditorFactory * factory);
  fn _ZN19QStyledItemDelegate20setItemEditorFactoryEP18QItemEditorFactory(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QStyledItemDelegate::paint(QPainter * painter, const QStyleOptionViewItem & option, const QModelIndex & index);
  fn _ZNK19QStyledItemDelegate5paintEP8QPainterRK20QStyleOptionViewItemRK11QModelIndex(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void, arg2: *mut c_void);
  // proto:  QItemEditorFactory * QStyledItemDelegate::itemEditorFactory();
  fn _ZNK19QStyledItemDelegate17itemEditorFactoryEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QStyledItemDelegate::QStyledItemDelegate(QObject * parent);
  fn _ZN19QStyledItemDelegateC1EP7QObject(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QString QStyledItemDelegate::displayText(const QVariant & value, const QLocale & locale);
  fn _ZNK19QStyledItemDelegate11displayTextERK8QVariantRK7QLocale(qthis: *mut c_void, arg0: *mut c_void, arg1: *mut c_void) -> *mut c_void;
  // proto:  const QMetaObject * QStyledItemDelegate::metaObject();
  fn _ZNK19QStyledItemDelegate10metaObjectEv(qthis: *mut c_void);
} // <= ext block end

// body block begin =>
// class sizeof(QStyledItemDelegate)=1
pub struct QStyledItemDelegate {
  qbase: QAbstractItemDelegate,
  pub qclsinst: *mut c_void,
}

impl /*struct*/ QStyledItemDelegate {
  pub fn inheritFrom(qthis: *mut c_void) -> QStyledItemDelegate {
    return QStyledItemDelegate{qbase: QAbstractItemDelegate::inheritFrom(qthis), qclsinst: qthis};
  }
}
impl Deref for QStyledItemDelegate {
  type Target = QAbstractItemDelegate;

  fn deref(&self) -> &QAbstractItemDelegate {
    return &self.qbase;
  }
}
impl AsRef<QAbstractItemDelegate> for QStyledItemDelegate {
  fn as_ref(&self) -> &QAbstractItemDelegate {
    return &self.qbase;
  }
}
  // proto:  QSize QStyledItemDelegate::sizeHint(const QStyleOptionViewItem & option, const QModelIndex & index);
impl /*struct*/ QStyledItemDelegate {
  pub fn sizeHint<RetType, T: QStyledItemDelegate_sizeHint<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.sizeHint(self);
    // return 1;
  }
}

pub trait QStyledItemDelegate_sizeHint<RetType> {
  fn sizeHint(self , rsthis: &mut QStyledItemDelegate) -> RetType;
}

  // proto:  QSize QStyledItemDelegate::sizeHint(const QStyleOptionViewItem & option, const QModelIndex & index);
impl<'a> /*trait*/ QStyledItemDelegate_sizeHint<QSize> for (QStyleOptionViewItem, QModelIndex) {
  fn sizeHint(self , rsthis: &mut QStyledItemDelegate) -> QSize {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QStyledItemDelegate8sizeHintERK20QStyleOptionViewItemRK11QModelIndex()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK19QStyledItemDelegate8sizeHintERK20QStyleOptionViewItemRK11QModelIndex(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QSize::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  QWidget * QStyledItemDelegate::createEditor(QWidget * parent, const QStyleOptionViewItem & option, const QModelIndex & index);
impl /*struct*/ QStyledItemDelegate {
  pub fn createEditor<RetType, T: QStyledItemDelegate_createEditor<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.createEditor(self);
    // return 1;
  }
}

pub trait QStyledItemDelegate_createEditor<RetType> {
  fn createEditor(self , rsthis: &mut QStyledItemDelegate) -> RetType;
}

  // proto:  QWidget * QStyledItemDelegate::createEditor(QWidget * parent, const QStyleOptionViewItem & option, const QModelIndex & index);
impl<'a> /*trait*/ QStyledItemDelegate_createEditor<QWidget> for (QWidget, QStyleOptionViewItem, QModelIndex) {
  fn createEditor(self , rsthis: &mut QStyledItemDelegate) -> QWidget {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QStyledItemDelegate12createEditorEP7QWidgetRK20QStyleOptionViewItemRK11QModelIndex()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK19QStyledItemDelegate12createEditorEP7QWidgetRK20QStyleOptionViewItemRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2)};
    let mut ret1 = QWidget::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QStyledItemDelegate::~QStyledItemDelegate();
impl /*struct*/ QStyledItemDelegate {
  pub fn FreeQStyledItemDelegate<RetType, T: QStyledItemDelegate_FreeQStyledItemDelegate<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQStyledItemDelegate(self);
    // return 1;
  }
}

pub trait QStyledItemDelegate_FreeQStyledItemDelegate<RetType> {
  fn FreeQStyledItemDelegate(self , rsthis: &mut QStyledItemDelegate) -> RetType;
}

  // proto:  void QStyledItemDelegate::~QStyledItemDelegate();
impl<'a> /*trait*/ QStyledItemDelegate_FreeQStyledItemDelegate<()> for () {
  fn FreeQStyledItemDelegate(self , rsthis: &mut QStyledItemDelegate) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QStyledItemDelegateD0Ev()};
     unsafe {_ZN19QStyledItemDelegateD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QStyledItemDelegate::updateEditorGeometry(QWidget * editor, const QStyleOptionViewItem & option, const QModelIndex & index);
impl /*struct*/ QStyledItemDelegate {
  pub fn updateEditorGeometry<RetType, T: QStyledItemDelegate_updateEditorGeometry<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.updateEditorGeometry(self);
    // return 1;
  }
}

pub trait QStyledItemDelegate_updateEditorGeometry<RetType> {
  fn updateEditorGeometry(self , rsthis: &mut QStyledItemDelegate) -> RetType;
}

  // proto:  void QStyledItemDelegate::updateEditorGeometry(QWidget * editor, const QStyleOptionViewItem & option, const QModelIndex & index);
impl<'a> /*trait*/ QStyledItemDelegate_updateEditorGeometry<()> for (QWidget, QStyleOptionViewItem, QModelIndex) {
  fn updateEditorGeometry(self , rsthis: &mut QStyledItemDelegate) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QStyledItemDelegate20updateEditorGeometryEP7QWidgetRK20QStyleOptionViewItemRK11QModelIndex()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZNK19QStyledItemDelegate20updateEditorGeometryEP7QWidgetRK20QStyleOptionViewItemRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  void QStyledItemDelegate::setEditorData(QWidget * editor, const QModelIndex & index);
impl /*struct*/ QStyledItemDelegate {
  pub fn setEditorData<RetType, T: QStyledItemDelegate_setEditorData<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setEditorData(self);
    // return 1;
  }
}

pub trait QStyledItemDelegate_setEditorData<RetType> {
  fn setEditorData(self , rsthis: &mut QStyledItemDelegate) -> RetType;
}

  // proto:  void QStyledItemDelegate::setEditorData(QWidget * editor, const QModelIndex & index);
impl<'a> /*trait*/ QStyledItemDelegate_setEditorData<()> for (QWidget, QModelIndex) {
  fn setEditorData(self , rsthis: &mut QStyledItemDelegate) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QStyledItemDelegate13setEditorDataEP7QWidgetRK11QModelIndex()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
     unsafe {_ZNK19QStyledItemDelegate13setEditorDataEP7QWidgetRK11QModelIndex(rsthis.qclsinst, arg0, arg1)};
    // return 1;
  }
}

  // proto:  void QStyledItemDelegate::QStyledItemDelegate(const QStyledItemDelegate & );
impl /*struct*/ QStyledItemDelegate {
  pub fn NewQStyledItemDelegate<T: QStyledItemDelegate_NewQStyledItemDelegate>(value: T) -> QStyledItemDelegate {
    let rsthis = value.NewQStyledItemDelegate();
    return rsthis;
    // return 1;
  }
}

pub trait QStyledItemDelegate_NewQStyledItemDelegate {
  fn NewQStyledItemDelegate(self) -> QStyledItemDelegate;
}

  // proto:  void QStyledItemDelegate::QStyledItemDelegate(const QStyledItemDelegate & );
impl<'a> /*trait*/ QStyledItemDelegate_NewQStyledItemDelegate for (QStyledItemDelegate) {
  fn NewQStyledItemDelegate(self) -> QStyledItemDelegate {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QStyledItemDelegateC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN19QStyledItemDelegateC1ERKS_(qthis, arg0)};
    let rsthis = QStyledItemDelegate{/**/qbase: QAbstractItemDelegate::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QStyledItemDelegate::setItemEditorFactory(QItemEditorFactory * factory);
impl /*struct*/ QStyledItemDelegate {
  pub fn setItemEditorFactory<RetType, T: QStyledItemDelegate_setItemEditorFactory<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setItemEditorFactory(self);
    // return 1;
  }
}

pub trait QStyledItemDelegate_setItemEditorFactory<RetType> {
  fn setItemEditorFactory(self , rsthis: &mut QStyledItemDelegate) -> RetType;
}

  // proto:  void QStyledItemDelegate::setItemEditorFactory(QItemEditorFactory * factory);
impl<'a> /*trait*/ QStyledItemDelegate_setItemEditorFactory<()> for (QItemEditorFactory) {
  fn setItemEditorFactory(self , rsthis: &mut QStyledItemDelegate) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QStyledItemDelegate20setItemEditorFactoryEP18QItemEditorFactory()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN19QStyledItemDelegate20setItemEditorFactoryEP18QItemEditorFactory(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QStyledItemDelegate::paint(QPainter * painter, const QStyleOptionViewItem & option, const QModelIndex & index);
impl /*struct*/ QStyledItemDelegate {
  pub fn paint<RetType, T: QStyledItemDelegate_paint<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.paint(self);
    // return 1;
  }
}

pub trait QStyledItemDelegate_paint<RetType> {
  fn paint(self , rsthis: &mut QStyledItemDelegate) -> RetType;
}

  // proto:  void QStyledItemDelegate::paint(QPainter * painter, const QStyleOptionViewItem & option, const QModelIndex & index);
impl<'a> /*trait*/ QStyledItemDelegate_paint<()> for (QPainter, QStyleOptionViewItem, QModelIndex) {
  fn paint(self , rsthis: &mut QStyledItemDelegate) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QStyledItemDelegate5paintEP8QPainterRK20QStyleOptionViewItemRK11QModelIndex()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let arg2 = self.2.qclsinst  as *mut c_void;
     unsafe {_ZNK19QStyledItemDelegate5paintEP8QPainterRK20QStyleOptionViewItemRK11QModelIndex(rsthis.qclsinst, arg0, arg1, arg2)};
    // return 1;
  }
}

  // proto:  QItemEditorFactory * QStyledItemDelegate::itemEditorFactory();
impl /*struct*/ QStyledItemDelegate {
  pub fn itemEditorFactory<RetType, T: QStyledItemDelegate_itemEditorFactory<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.itemEditorFactory(self);
    // return 1;
  }
}

pub trait QStyledItemDelegate_itemEditorFactory<RetType> {
  fn itemEditorFactory(self , rsthis: &mut QStyledItemDelegate) -> RetType;
}

  // proto:  QItemEditorFactory * QStyledItemDelegate::itemEditorFactory();
impl<'a> /*trait*/ QStyledItemDelegate_itemEditorFactory<QItemEditorFactory> for () {
  fn itemEditorFactory(self , rsthis: &mut QStyledItemDelegate) -> QItemEditorFactory {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QStyledItemDelegate17itemEditorFactoryEv()};
    let mut ret = unsafe {_ZNK19QStyledItemDelegate17itemEditorFactoryEv(rsthis.qclsinst)};
    let mut ret1 = QItemEditorFactory::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  void QStyledItemDelegate::QStyledItemDelegate(QObject * parent);
impl<'a> /*trait*/ QStyledItemDelegate_NewQStyledItemDelegate for (QObject) {
  fn NewQStyledItemDelegate(self) -> QStyledItemDelegate {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN19QStyledItemDelegateC1EP7QObject()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN19QStyledItemDelegateC1EP7QObject(qthis, arg0)};
    let rsthis = QStyledItemDelegate{/**/qbase: QAbstractItemDelegate::inheritFrom(qthis), /**/qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  QString QStyledItemDelegate::displayText(const QVariant & value, const QLocale & locale);
impl /*struct*/ QStyledItemDelegate {
  pub fn displayText<RetType, T: QStyledItemDelegate_displayText<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.displayText(self);
    // return 1;
  }
}

pub trait QStyledItemDelegate_displayText<RetType> {
  fn displayText(self , rsthis: &mut QStyledItemDelegate) -> RetType;
}

  // proto:  QString QStyledItemDelegate::displayText(const QVariant & value, const QLocale & locale);
impl<'a> /*trait*/ QStyledItemDelegate_displayText<QString> for (QVariant, QLocale) {
  fn displayText(self , rsthis: &mut QStyledItemDelegate) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QStyledItemDelegate11displayTextERK8QVariantRK7QLocale()};
    let arg0 = self.0.qclsinst  as *mut c_void;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZNK19QStyledItemDelegate11displayTextERK8QVariantRK7QLocale(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QString::inheritFrom(ret);
    return ret1;
    // return 1;
  }
}

  // proto:  const QMetaObject * QStyledItemDelegate::metaObject();
impl /*struct*/ QStyledItemDelegate {
  pub fn metaObject<RetType, T: QStyledItemDelegate_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QStyledItemDelegate_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QStyledItemDelegate) -> RetType;
}

  // proto:  const QMetaObject * QStyledItemDelegate::metaObject();
impl<'a> /*trait*/ QStyledItemDelegate_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QStyledItemDelegate) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK19QStyledItemDelegate10metaObjectEv()};
     unsafe {_ZNK19QStyledItemDelegate10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

// <= body block end

