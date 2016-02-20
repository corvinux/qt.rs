// #[warn(non_snake_case)]
// #[warn(non_camel_case_types)]
// #[warn(unused_mut)]
// #[warn(unused_attributes)]
// #[warn(unused_imports)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(unused_mut)]
#![allow(unused_attributes)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

#[link(name = "Qt5Core")]
#[link(name = "Qt5Gui")]
#[link(name = "Qt5Widgets")]
#[link(name = "Qt5Network")]
#[link(name = "Qt5Qml")]
#[link(name = "Qt5Quick")]
#[link(name = "QtInline")]
extern {}  // 这行还是需要的

pub mod core;

#[cfg(feature = "widgets")]
pub mod gui;
#[cfg(feature = "widgets")]
pub mod widgets;

#[cfg(feature = "network")]
pub mod network;

#[cfg(feature = "qml")]
pub mod qml;
#[cfg(feature = "qml")]
pub mod quick;
