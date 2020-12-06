use std::ffi::CString;

use either::Either;

use crate::DesktopEntry;

mod apps;
mod dialog;

macro_rules! delegate {
    (pub fn $name:ident ( &mut self ) -> $ret:ty $(, wrap_with ($wrap:path))?) => {
        delegate!(pub fn $name ( & [mut] self, ) -> $ret $(, wrap_with ($wrap))?);
    };
    (pub fn $name:ident ( &mut self, $($ident:ident : $tp:ty),* ) -> $ret:ty $(, wrap_with ($wrap:path))?) => {
        delegate!(pub fn $name ( & [mut] self, $($ident : $tp),* ) -> $ret $(, wrap_with ($wrap))?);
    };
    (pub fn $name:ident ( & $([$m:ident])? self ) -> $ret:ty $(, wrap_with ($wrap:path))?) => {
        delegate!(pub fn $name ( & $([$m])? self, ) -> $ret $(, wrap_with ($wrap))?);
    };
    (pub fn $name:ident ( & $([$m:ident])? self, $($ident:ident : $tp:ty),* ) -> $ret:ty $(, wrap_with ($wrap:path))?) => {
        pub fn $name ( & $($m)? self, $($ident : $tp),* ) -> $ret {
            match self {
                Mode::AppsMode(mode) => $($wrap)?(mode.$name($($ident),*)),
                Mode::DialogMode(mode) => $($wrap)?(mode.$name($($ident),*)),
            }
        }
    }
}

pub enum Mode {
    AppsMode(apps::AppsMode),
    DialogMode(dialog::DialogMode),
}

impl Mode {
    pub fn apps(entries: Vec<DesktopEntry>, term: Vec<CString>) -> Self {
        Self::AppsMode(apps::AppsMode::new(entries, term))
    }

    pub fn dialog() -> Self {
        Self::DialogMode(dialog::DialogMode::new())
    }

    delegate!(pub fn eval(&mut self, idx: usize) -> std::convert::Infallible);
    delegate!(pub fn entries_len(&self) -> usize);
    delegate!(pub fn list_item(&self, idx: usize) -> crate::draw::ListItem<'_>);

    pub fn text_entries(&self) -> impl Iterator<Item = &str> + '_ {
        match self {
            Mode::AppsMode(mode) => Either::Left(mode.text_entries()),
            Mode::DialogMode(mode) => Either::Right(mode.text_entries()),
        }
        .into_iter()
    }
}