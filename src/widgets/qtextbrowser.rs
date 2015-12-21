// header block begin
#![feature(libc)]
#![feature(core)]
#![feature(collections)]
extern crate libc;
use self::libc::*;

// main block begin
// use block begin
use super::qurl::QUrl;
use super::qwidget::QWidget;
use super::qstring::QString;
use super::qstringlist::QStringList;
use super::qvariant::QVariant;

// ext block begin
#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
extern {
  // proto:  bool QTextBrowser::isBackwardAvailable();
  fn _ZNK12QTextBrowser19isBackwardAvailableEv(qthis: *mut c_void) -> c_char;
  // proto:  void QTextBrowser::reload();
  fn _ZN12QTextBrowser6reloadEv(qthis: *mut c_void);
  // proto:  bool QTextBrowser::openLinks();
  fn _ZNK12QTextBrowser9openLinksEv(qthis: *mut c_void) -> c_char;
  // proto:  void QTextBrowser::clearHistory();
  fn _ZN12QTextBrowser12clearHistoryEv(qthis: *mut c_void);
  // proto:  void QTextBrowser::highlighted(const QUrl & );
  fn _ZN12QTextBrowser11highlightedERK4QUrl(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  const QMetaObject * QTextBrowser::metaObject();
  fn _ZNK12QTextBrowser10metaObjectEv(qthis: *mut c_void);
  // proto:  QUrl QTextBrowser::historyUrl(int );
  fn _ZNK12QTextBrowser10historyUrlEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  void QTextBrowser::sourceChanged(const QUrl & );
  fn _ZN12QTextBrowser13sourceChangedERK4QUrl(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  bool QTextBrowser::isForwardAvailable();
  fn _ZNK12QTextBrowser18isForwardAvailableEv(qthis: *mut c_void) -> c_char;
  // proto:  bool QTextBrowser::openExternalLinks();
  fn _ZNK12QTextBrowser17openExternalLinksEv(qthis: *mut c_void) -> c_char;
  // proto:  void QTextBrowser::QTextBrowser(QWidget * parent);
  fn _ZN12QTextBrowserC1EP7QWidget(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  int QTextBrowser::backwardHistoryCount();
  fn _ZNK12QTextBrowser20backwardHistoryCountEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTextBrowser::home();
  fn _ZN12QTextBrowser4homeEv(qthis: *mut c_void);
  // proto:  void QTextBrowser::~QTextBrowser();
  fn _ZN12QTextBrowserD0Ev(qthis: *mut c_void);
  // proto:  void QTextBrowser::QTextBrowser(const QTextBrowser & );
  fn _ZN12QTextBrowserC1ERKS_(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QTextBrowser::setOpenLinks(bool open);
  fn _ZN12QTextBrowser12setOpenLinksEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QTextBrowser::forward();
  fn _ZN12QTextBrowser7forwardEv(qthis: *mut c_void);
  // proto:  void QTextBrowser::highlighted(const QString & );
  fn _ZN12QTextBrowser11highlightedERK7QString(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QString QTextBrowser::historyTitle(int );
  fn _ZNK12QTextBrowser12historyTitleEi(qthis: *mut c_void, arg0: c_int) -> *mut c_void;
  // proto:  void QTextBrowser::forwardAvailable(bool );
  fn _ZN12QTextBrowser16forwardAvailableEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QTextBrowser::setSearchPaths(const QStringList & paths);
  fn _ZN12QTextBrowser14setSearchPathsERK11QStringList(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QVariant QTextBrowser::loadResource(int type, const QUrl & name);
  fn _ZN12QTextBrowser12loadResourceEiRK4QUrl(qthis: *mut c_void, arg0: c_int, arg1: *mut c_void) -> *mut c_void;
  // proto:  QUrl QTextBrowser::source();
  fn _ZNK12QTextBrowser6sourceEv(qthis: *mut c_void) -> *mut c_void;
  // proto:  void QTextBrowser::historyChanged();
  fn _ZN12QTextBrowser14historyChangedEv(qthis: *mut c_void);
  // proto:  void QTextBrowser::setOpenExternalLinks(bool open);
  fn _ZN12QTextBrowser20setOpenExternalLinksEb(qthis: *mut c_void, arg0: c_char);
  // proto:  void QTextBrowser::setSource(const QUrl & name);
  fn _ZN12QTextBrowser9setSourceERK4QUrl(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  QStringList QTextBrowser::searchPaths();
  fn _ZNK12QTextBrowser11searchPathsEv(qthis: *mut c_void);
  // proto:  void QTextBrowser::backward();
  fn _ZN12QTextBrowser8backwardEv(qthis: *mut c_void);
  // proto:  int QTextBrowser::forwardHistoryCount();
  fn _ZNK12QTextBrowser19forwardHistoryCountEv(qthis: *mut c_void) -> c_int;
  // proto:  void QTextBrowser::anchorClicked(const QUrl & );
  fn _ZN12QTextBrowser13anchorClickedERK4QUrl(qthis: *mut c_void, arg0: *mut c_void);
  // proto:  void QTextBrowser::backwardAvailable(bool );
  fn _ZN12QTextBrowser17backwardAvailableEb(qthis: *mut c_void, arg0: c_char);
}

// body block begin
// class sizeof(QTextBrowser)=1
pub struct QTextBrowser {
  pub qclsinst: *mut c_void,
}

  // proto:  bool QTextBrowser::isBackwardAvailable();
impl /*struct*/ QTextBrowser {
  pub fn isBackwardAvailable<RetType, T: QTextBrowser_isBackwardAvailable<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isBackwardAvailable(self);
    // return 1;
  }
}

pub trait QTextBrowser_isBackwardAvailable<RetType> {
  fn isBackwardAvailable(self , rsthis: &mut QTextBrowser) -> RetType;
}

  // proto:  bool QTextBrowser::isBackwardAvailable();
impl<'a> /*trait*/ QTextBrowser_isBackwardAvailable<i8> for () {
  fn isBackwardAvailable(self , rsthis: &mut QTextBrowser) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTextBrowser19isBackwardAvailableEv()};
    let mut ret = unsafe {_ZNK12QTextBrowser19isBackwardAvailableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QTextBrowser::reload();
impl /*struct*/ QTextBrowser {
  pub fn reload<RetType, T: QTextBrowser_reload<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.reload(self);
    // return 1;
  }
}

pub trait QTextBrowser_reload<RetType> {
  fn reload(self , rsthis: &mut QTextBrowser) -> RetType;
}

  // proto:  void QTextBrowser::reload();
impl<'a> /*trait*/ QTextBrowser_reload<()> for () {
  fn reload(self , rsthis: &mut QTextBrowser) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextBrowser6reloadEv()};
     unsafe {_ZN12QTextBrowser6reloadEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  bool QTextBrowser::openLinks();
impl /*struct*/ QTextBrowser {
  pub fn openLinks<RetType, T: QTextBrowser_openLinks<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.openLinks(self);
    // return 1;
  }
}

pub trait QTextBrowser_openLinks<RetType> {
  fn openLinks(self , rsthis: &mut QTextBrowser) -> RetType;
}

  // proto:  bool QTextBrowser::openLinks();
impl<'a> /*trait*/ QTextBrowser_openLinks<i8> for () {
  fn openLinks(self , rsthis: &mut QTextBrowser) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTextBrowser9openLinksEv()};
    let mut ret = unsafe {_ZNK12QTextBrowser9openLinksEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QTextBrowser::clearHistory();
impl /*struct*/ QTextBrowser {
  pub fn clearHistory<RetType, T: QTextBrowser_clearHistory<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.clearHistory(self);
    // return 1;
  }
}

pub trait QTextBrowser_clearHistory<RetType> {
  fn clearHistory(self , rsthis: &mut QTextBrowser) -> RetType;
}

  // proto:  void QTextBrowser::clearHistory();
impl<'a> /*trait*/ QTextBrowser_clearHistory<()> for () {
  fn clearHistory(self , rsthis: &mut QTextBrowser) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextBrowser12clearHistoryEv()};
     unsafe {_ZN12QTextBrowser12clearHistoryEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTextBrowser::highlighted(const QUrl & );
impl /*struct*/ QTextBrowser {
  pub fn highlighted<RetType, T: QTextBrowser_highlighted<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.highlighted(self);
    // return 1;
  }
}

pub trait QTextBrowser_highlighted<RetType> {
  fn highlighted(self , rsthis: &mut QTextBrowser) -> RetType;
}

  // proto:  void QTextBrowser::highlighted(const QUrl & );
impl<'a> /*trait*/ QTextBrowser_highlighted<()> for (QUrl) {
  fn highlighted(self , rsthis: &mut QTextBrowser) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextBrowser11highlightedERK4QUrl()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QTextBrowser11highlightedERK4QUrl(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  const QMetaObject * QTextBrowser::metaObject();
impl /*struct*/ QTextBrowser {
  pub fn metaObject<RetType, T: QTextBrowser_metaObject<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.metaObject(self);
    // return 1;
  }
}

pub trait QTextBrowser_metaObject<RetType> {
  fn metaObject(self , rsthis: &mut QTextBrowser) -> RetType;
}

  // proto:  const QMetaObject * QTextBrowser::metaObject();
impl<'a> /*trait*/ QTextBrowser_metaObject<()> for () {
  fn metaObject(self , rsthis: &mut QTextBrowser) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTextBrowser10metaObjectEv()};
     unsafe {_ZNK12QTextBrowser10metaObjectEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  QUrl QTextBrowser::historyUrl(int );
impl /*struct*/ QTextBrowser {
  pub fn historyUrl<RetType, T: QTextBrowser_historyUrl<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.historyUrl(self);
    // return 1;
  }
}

pub trait QTextBrowser_historyUrl<RetType> {
  fn historyUrl(self , rsthis: &mut QTextBrowser) -> RetType;
}

  // proto:  QUrl QTextBrowser::historyUrl(int );
impl<'a> /*trait*/ QTextBrowser_historyUrl<QUrl> for (i32) {
  fn historyUrl(self , rsthis: &mut QTextBrowser) -> QUrl {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTextBrowser10historyUrlEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK12QTextBrowser10historyUrlEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QUrl{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextBrowser::sourceChanged(const QUrl & );
impl /*struct*/ QTextBrowser {
  pub fn sourceChanged<RetType, T: QTextBrowser_sourceChanged<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.sourceChanged(self);
    // return 1;
  }
}

pub trait QTextBrowser_sourceChanged<RetType> {
  fn sourceChanged(self , rsthis: &mut QTextBrowser) -> RetType;
}

  // proto:  void QTextBrowser::sourceChanged(const QUrl & );
impl<'a> /*trait*/ QTextBrowser_sourceChanged<()> for (QUrl) {
  fn sourceChanged(self , rsthis: &mut QTextBrowser) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextBrowser13sourceChangedERK4QUrl()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QTextBrowser13sourceChangedERK4QUrl(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  bool QTextBrowser::isForwardAvailable();
impl /*struct*/ QTextBrowser {
  pub fn isForwardAvailable<RetType, T: QTextBrowser_isForwardAvailable<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.isForwardAvailable(self);
    // return 1;
  }
}

pub trait QTextBrowser_isForwardAvailable<RetType> {
  fn isForwardAvailable(self , rsthis: &mut QTextBrowser) -> RetType;
}

  // proto:  bool QTextBrowser::isForwardAvailable();
impl<'a> /*trait*/ QTextBrowser_isForwardAvailable<i8> for () {
  fn isForwardAvailable(self , rsthis: &mut QTextBrowser) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTextBrowser18isForwardAvailableEv()};
    let mut ret = unsafe {_ZNK12QTextBrowser18isForwardAvailableEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  bool QTextBrowser::openExternalLinks();
impl /*struct*/ QTextBrowser {
  pub fn openExternalLinks<RetType, T: QTextBrowser_openExternalLinks<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.openExternalLinks(self);
    // return 1;
  }
}

pub trait QTextBrowser_openExternalLinks<RetType> {
  fn openExternalLinks(self , rsthis: &mut QTextBrowser) -> RetType;
}

  // proto:  bool QTextBrowser::openExternalLinks();
impl<'a> /*trait*/ QTextBrowser_openExternalLinks<i8> for () {
  fn openExternalLinks(self , rsthis: &mut QTextBrowser) -> i8 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTextBrowser17openExternalLinksEv()};
    let mut ret = unsafe {_ZNK12QTextBrowser17openExternalLinksEv(rsthis.qclsinst)};
    return ret as i8;
    // return 1;
  }
}

  // proto:  void QTextBrowser::QTextBrowser(QWidget * parent);
impl /*struct*/ QTextBrowser {
  pub fn NewQTextBrowser<T: QTextBrowser_NewQTextBrowser>(value: T) -> QTextBrowser {
    let rsthis = value.NewQTextBrowser();
    return rsthis;
    // return 1;
  }
}

pub trait QTextBrowser_NewQTextBrowser {
  fn NewQTextBrowser(self) -> QTextBrowser;
}

  // proto:  void QTextBrowser::QTextBrowser(QWidget * parent);
impl<'a> /*trait*/ QTextBrowser_NewQTextBrowser for (QWidget) {
  fn NewQTextBrowser(self) -> QTextBrowser {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextBrowserC1EP7QWidget()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QTextBrowserC1EP7QWidget(qthis, arg0)};
    let rsthis = QTextBrowser{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  int QTextBrowser::backwardHistoryCount();
impl /*struct*/ QTextBrowser {
  pub fn backwardHistoryCount<RetType, T: QTextBrowser_backwardHistoryCount<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.backwardHistoryCount(self);
    // return 1;
  }
}

pub trait QTextBrowser_backwardHistoryCount<RetType> {
  fn backwardHistoryCount(self , rsthis: &mut QTextBrowser) -> RetType;
}

  // proto:  int QTextBrowser::backwardHistoryCount();
impl<'a> /*trait*/ QTextBrowser_backwardHistoryCount<i32> for () {
  fn backwardHistoryCount(self , rsthis: &mut QTextBrowser) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTextBrowser20backwardHistoryCountEv()};
    let mut ret = unsafe {_ZNK12QTextBrowser20backwardHistoryCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QTextBrowser::home();
impl /*struct*/ QTextBrowser {
  pub fn home<RetType, T: QTextBrowser_home<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.home(self);
    // return 1;
  }
}

pub trait QTextBrowser_home<RetType> {
  fn home(self , rsthis: &mut QTextBrowser) -> RetType;
}

  // proto:  void QTextBrowser::home();
impl<'a> /*trait*/ QTextBrowser_home<()> for () {
  fn home(self , rsthis: &mut QTextBrowser) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextBrowser4homeEv()};
     unsafe {_ZN12QTextBrowser4homeEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTextBrowser::~QTextBrowser();
impl /*struct*/ QTextBrowser {
  pub fn FreeQTextBrowser<RetType, T: QTextBrowser_FreeQTextBrowser<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.FreeQTextBrowser(self);
    // return 1;
  }
}

pub trait QTextBrowser_FreeQTextBrowser<RetType> {
  fn FreeQTextBrowser(self , rsthis: &mut QTextBrowser) -> RetType;
}

  // proto:  void QTextBrowser::~QTextBrowser();
impl<'a> /*trait*/ QTextBrowser_FreeQTextBrowser<()> for () {
  fn FreeQTextBrowser(self , rsthis: &mut QTextBrowser) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextBrowserD0Ev()};
     unsafe {_ZN12QTextBrowserD0Ev(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTextBrowser::QTextBrowser(const QTextBrowser & );
impl<'a> /*trait*/ QTextBrowser_NewQTextBrowser for (QTextBrowser) {
  fn NewQTextBrowser(self) -> QTextBrowser {
    let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextBrowserC1ERKS_()};
    let arg0 = self.qclsinst  as *mut c_void;
    unsafe {_ZN12QTextBrowserC1ERKS_(qthis, arg0)};
    let rsthis = QTextBrowser{qclsinst: qthis};
    return rsthis;
    // return 1;
  }
}

  // proto:  void QTextBrowser::setOpenLinks(bool open);
impl /*struct*/ QTextBrowser {
  pub fn setOpenLinks<RetType, T: QTextBrowser_setOpenLinks<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setOpenLinks(self);
    // return 1;
  }
}

pub trait QTextBrowser_setOpenLinks<RetType> {
  fn setOpenLinks(self , rsthis: &mut QTextBrowser) -> RetType;
}

  // proto:  void QTextBrowser::setOpenLinks(bool open);
impl<'a> /*trait*/ QTextBrowser_setOpenLinks<()> for (i8) {
  fn setOpenLinks(self , rsthis: &mut QTextBrowser) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextBrowser12setOpenLinksEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN12QTextBrowser12setOpenLinksEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextBrowser::forward();
impl /*struct*/ QTextBrowser {
  pub fn forward<RetType, T: QTextBrowser_forward<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.forward(self);
    // return 1;
  }
}

pub trait QTextBrowser_forward<RetType> {
  fn forward(self , rsthis: &mut QTextBrowser) -> RetType;
}

  // proto:  void QTextBrowser::forward();
impl<'a> /*trait*/ QTextBrowser_forward<()> for () {
  fn forward(self , rsthis: &mut QTextBrowser) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextBrowser7forwardEv()};
     unsafe {_ZN12QTextBrowser7forwardEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTextBrowser::highlighted(const QString & );
impl<'a> /*trait*/ QTextBrowser_highlighted<()> for (QString) {
  fn highlighted(self , rsthis: &mut QTextBrowser) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextBrowser11highlightedERK7QString()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QTextBrowser11highlightedERK7QString(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QString QTextBrowser::historyTitle(int );
impl /*struct*/ QTextBrowser {
  pub fn historyTitle<RetType, T: QTextBrowser_historyTitle<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.historyTitle(self);
    // return 1;
  }
}

pub trait QTextBrowser_historyTitle<RetType> {
  fn historyTitle(self , rsthis: &mut QTextBrowser) -> RetType;
}

  // proto:  QString QTextBrowser::historyTitle(int );
impl<'a> /*trait*/ QTextBrowser_historyTitle<QString> for (i32) {
  fn historyTitle(self , rsthis: &mut QTextBrowser) -> QString {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTextBrowser12historyTitleEi()};
    let arg0 = self  as c_int;
    let mut ret = unsafe {_ZNK12QTextBrowser12historyTitleEi(rsthis.qclsinst, arg0)};
    let mut ret1 = QString{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextBrowser::forwardAvailable(bool );
impl /*struct*/ QTextBrowser {
  pub fn forwardAvailable<RetType, T: QTextBrowser_forwardAvailable<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.forwardAvailable(self);
    // return 1;
  }
}

pub trait QTextBrowser_forwardAvailable<RetType> {
  fn forwardAvailable(self , rsthis: &mut QTextBrowser) -> RetType;
}

  // proto:  void QTextBrowser::forwardAvailable(bool );
impl<'a> /*trait*/ QTextBrowser_forwardAvailable<()> for (i8) {
  fn forwardAvailable(self , rsthis: &mut QTextBrowser) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextBrowser16forwardAvailableEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN12QTextBrowser16forwardAvailableEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextBrowser::setSearchPaths(const QStringList & paths);
impl /*struct*/ QTextBrowser {
  pub fn setSearchPaths<RetType, T: QTextBrowser_setSearchPaths<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setSearchPaths(self);
    // return 1;
  }
}

pub trait QTextBrowser_setSearchPaths<RetType> {
  fn setSearchPaths(self , rsthis: &mut QTextBrowser) -> RetType;
}

  // proto:  void QTextBrowser::setSearchPaths(const QStringList & paths);
impl<'a> /*trait*/ QTextBrowser_setSearchPaths<()> for (QStringList) {
  fn setSearchPaths(self , rsthis: &mut QTextBrowser) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextBrowser14setSearchPathsERK11QStringList()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QTextBrowser14setSearchPathsERK11QStringList(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QVariant QTextBrowser::loadResource(int type, const QUrl & name);
impl /*struct*/ QTextBrowser {
  pub fn loadResource<RetType, T: QTextBrowser_loadResource<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.loadResource(self);
    // return 1;
  }
}

pub trait QTextBrowser_loadResource<RetType> {
  fn loadResource(self , rsthis: &mut QTextBrowser) -> RetType;
}

  // proto:  QVariant QTextBrowser::loadResource(int type, const QUrl & name);
impl<'a> /*trait*/ QTextBrowser_loadResource<QVariant> for (i32, QUrl) {
  fn loadResource(self , rsthis: &mut QTextBrowser) -> QVariant {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextBrowser12loadResourceEiRK4QUrl()};
    let arg0 = self.0  as c_int;
    let arg1 = self.1.qclsinst  as *mut c_void;
    let mut ret = unsafe {_ZN12QTextBrowser12loadResourceEiRK4QUrl(rsthis.qclsinst, arg0, arg1)};
    let mut ret1 = QVariant{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  QUrl QTextBrowser::source();
impl /*struct*/ QTextBrowser {
  pub fn source<RetType, T: QTextBrowser_source<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.source(self);
    // return 1;
  }
}

pub trait QTextBrowser_source<RetType> {
  fn source(self , rsthis: &mut QTextBrowser) -> RetType;
}

  // proto:  QUrl QTextBrowser::source();
impl<'a> /*trait*/ QTextBrowser_source<QUrl> for () {
  fn source(self , rsthis: &mut QTextBrowser) -> QUrl {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTextBrowser6sourceEv()};
    let mut ret = unsafe {_ZNK12QTextBrowser6sourceEv(rsthis.qclsinst)};
    let mut ret1 = QUrl{qclsinst: ret};
    return ret1;
    // return 1;
  }
}

  // proto:  void QTextBrowser::historyChanged();
impl /*struct*/ QTextBrowser {
  pub fn historyChanged<RetType, T: QTextBrowser_historyChanged<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.historyChanged(self);
    // return 1;
  }
}

pub trait QTextBrowser_historyChanged<RetType> {
  fn historyChanged(self , rsthis: &mut QTextBrowser) -> RetType;
}

  // proto:  void QTextBrowser::historyChanged();
impl<'a> /*trait*/ QTextBrowser_historyChanged<()> for () {
  fn historyChanged(self , rsthis: &mut QTextBrowser) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextBrowser14historyChangedEv()};
     unsafe {_ZN12QTextBrowser14historyChangedEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTextBrowser::setOpenExternalLinks(bool open);
impl /*struct*/ QTextBrowser {
  pub fn setOpenExternalLinks<RetType, T: QTextBrowser_setOpenExternalLinks<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setOpenExternalLinks(self);
    // return 1;
  }
}

pub trait QTextBrowser_setOpenExternalLinks<RetType> {
  fn setOpenExternalLinks(self , rsthis: &mut QTextBrowser) -> RetType;
}

  // proto:  void QTextBrowser::setOpenExternalLinks(bool open);
impl<'a> /*trait*/ QTextBrowser_setOpenExternalLinks<()> for (i8) {
  fn setOpenExternalLinks(self , rsthis: &mut QTextBrowser) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextBrowser20setOpenExternalLinksEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN12QTextBrowser20setOpenExternalLinksEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextBrowser::setSource(const QUrl & name);
impl /*struct*/ QTextBrowser {
  pub fn setSource<RetType, T: QTextBrowser_setSource<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.setSource(self);
    // return 1;
  }
}

pub trait QTextBrowser_setSource<RetType> {
  fn setSource(self , rsthis: &mut QTextBrowser) -> RetType;
}

  // proto:  void QTextBrowser::setSource(const QUrl & name);
impl<'a> /*trait*/ QTextBrowser_setSource<()> for (QUrl) {
  fn setSource(self , rsthis: &mut QTextBrowser) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextBrowser9setSourceERK4QUrl()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QTextBrowser9setSourceERK4QUrl(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  QStringList QTextBrowser::searchPaths();
impl /*struct*/ QTextBrowser {
  pub fn searchPaths<RetType, T: QTextBrowser_searchPaths<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.searchPaths(self);
    // return 1;
  }
}

pub trait QTextBrowser_searchPaths<RetType> {
  fn searchPaths(self , rsthis: &mut QTextBrowser) -> RetType;
}

  // proto:  QStringList QTextBrowser::searchPaths();
impl<'a> /*trait*/ QTextBrowser_searchPaths<()> for () {
  fn searchPaths(self , rsthis: &mut QTextBrowser) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTextBrowser11searchPathsEv()};
     unsafe {_ZNK12QTextBrowser11searchPathsEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  void QTextBrowser::backward();
impl /*struct*/ QTextBrowser {
  pub fn backward<RetType, T: QTextBrowser_backward<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.backward(self);
    // return 1;
  }
}

pub trait QTextBrowser_backward<RetType> {
  fn backward(self , rsthis: &mut QTextBrowser) -> RetType;
}

  // proto:  void QTextBrowser::backward();
impl<'a> /*trait*/ QTextBrowser_backward<()> for () {
  fn backward(self , rsthis: &mut QTextBrowser) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextBrowser8backwardEv()};
     unsafe {_ZN12QTextBrowser8backwardEv(rsthis.qclsinst)};
    // return 1;
  }
}

  // proto:  int QTextBrowser::forwardHistoryCount();
impl /*struct*/ QTextBrowser {
  pub fn forwardHistoryCount<RetType, T: QTextBrowser_forwardHistoryCount<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.forwardHistoryCount(self);
    // return 1;
  }
}

pub trait QTextBrowser_forwardHistoryCount<RetType> {
  fn forwardHistoryCount(self , rsthis: &mut QTextBrowser) -> RetType;
}

  // proto:  int QTextBrowser::forwardHistoryCount();
impl<'a> /*trait*/ QTextBrowser_forwardHistoryCount<i32> for () {
  fn forwardHistoryCount(self , rsthis: &mut QTextBrowser) -> i32 {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZNK12QTextBrowser19forwardHistoryCountEv()};
    let mut ret = unsafe {_ZNK12QTextBrowser19forwardHistoryCountEv(rsthis.qclsinst)};
    return ret as i32;
    // return 1;
  }
}

  // proto:  void QTextBrowser::anchorClicked(const QUrl & );
impl /*struct*/ QTextBrowser {
  pub fn anchorClicked<RetType, T: QTextBrowser_anchorClicked<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.anchorClicked(self);
    // return 1;
  }
}

pub trait QTextBrowser_anchorClicked<RetType> {
  fn anchorClicked(self , rsthis: &mut QTextBrowser) -> RetType;
}

  // proto:  void QTextBrowser::anchorClicked(const QUrl & );
impl<'a> /*trait*/ QTextBrowser_anchorClicked<()> for (QUrl) {
  fn anchorClicked(self , rsthis: &mut QTextBrowser) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextBrowser13anchorClickedERK4QUrl()};
    let arg0 = self.qclsinst  as *mut c_void;
     unsafe {_ZN12QTextBrowser13anchorClickedERK4QUrl(rsthis.qclsinst, arg0)};
    // return 1;
  }
}

  // proto:  void QTextBrowser::backwardAvailable(bool );
impl /*struct*/ QTextBrowser {
  pub fn backwardAvailable<RetType, T: QTextBrowser_backwardAvailable<RetType>>(&mut self,  overload_args: T) -> RetType {
    return overload_args.backwardAvailable(self);
    // return 1;
  }
}

pub trait QTextBrowser_backwardAvailable<RetType> {
  fn backwardAvailable(self , rsthis: &mut QTextBrowser) -> RetType;
}

  // proto:  void QTextBrowser::backwardAvailable(bool );
impl<'a> /*trait*/ QTextBrowser_backwardAvailable<()> for (i8) {
  fn backwardAvailable(self , rsthis: &mut QTextBrowser) -> () {
    // let qthis: *mut c_void = unsafe{calloc(1, 32)};
    // unsafe{_ZN12QTextBrowser17backwardAvailableEb()};
    let arg0 = self  as c_char;
     unsafe {_ZN12QTextBrowser17backwardAvailableEb(rsthis.qclsinst, arg0)};
    // return 1;
  }
}
