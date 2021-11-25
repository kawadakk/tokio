//! No-op signal handling for unsupported platforms
use crate::signal::registry::{EventId, EventInfo, Init, Storage};
use std::{
    future::{pending, Future},
    io,
};

#[derive(Debug)]
pub(crate) struct OsStorage {}

impl Init for OsStorage {
    fn init() -> Self {
        Self {}
    }
}

impl Storage for OsStorage {
    fn event_info(&self, _: EventId) -> Option<&EventInfo> {
        None
    }

    fn for_each<'a, F>(&'a self, _: F)
    where
        F: FnMut(&'a EventInfo),
    {
    }
}

#[derive(Debug)]
pub(crate) struct OsExtraData {}

impl Init for OsExtraData {
    fn init() -> Self {
        Self {}
    }
}

pub fn ctrl_c() -> io::Result<CtrlC> {
    Ok(CtrlC {})
}

#[must_use = "streams do nothing unless polled"]
#[derive(Debug)]
pub struct CtrlC {}

impl CtrlC {
    pub fn recv(&mut self) -> impl Future<Output = Option<()>> {
        pending()
    }
}
