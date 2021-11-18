//! Process handling for platforms that don't support spawning processes.

use crate::io::PollEvented;
use crate::process::kill::Kill;
use crate::process::SpawnedChild;

use std::future::Future;
use std::io;
use std::pin::Pin;
use std::process::Stdio;
use std::process::{Command as StdCommand, ExitStatus};
use std::task::Context;
use std::task::Poll;

#[must_use = "futures do nothing unless polled"]
#[derive(Debug)]
pub(crate) enum Child {}

pub(crate) fn spawn_child(_: &mut StdCommand) -> io::Result<SpawnedChild> {
    unimplemented!("this platform does not support spawning processes")
}

impl Child {
    pub(crate) fn id(&self) -> u32 {
        match *self {}
    }

    pub(crate) fn try_wait(&mut self) -> io::Result<Option<ExitStatus>> {
        match *self {}
    }
}

impl Kill for Child {
    fn kill(&mut self) -> io::Result<()> {
        match *self {}
    }
}

impl Future for Child {
    type Output = io::Result<ExitStatus>;

    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Self::Output> {
        match *self {}
    }
}

#[derive(Debug)]
pub(crate) enum NeverSource {}

pub(crate) type ChildStdin = PollEvented<NeverSource>;
pub(crate) type ChildStdout = PollEvented<NeverSource>;
pub(crate) type ChildStderr = PollEvented<NeverSource>;

impl mio::event::Source for NeverSource {
    fn register(&mut self, _: &mio::Registry, _: mio::Token, _: mio::Interest) -> io::Result<()> {
        match *self {}
    }
    fn reregister(&mut self, _: &mio::Registry, _: mio::Token, _: mio::Interest) -> io::Result<()> {
        match *self {}
    }
    fn deregister(&mut self, _: &mio::Registry) -> io::Result<()> {
        match *self {}
    }
}

impl std::io::Read for &NeverSource {
    fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
        match **self {}
    }
}

impl std::io::Write for &NeverSource {
    fn write(&mut self, _: &[u8]) -> io::Result<usize> {
        match **self {}
    }

    fn flush(&mut self) -> io::Result<()> {
        match **self {}
    }
}

pub(crate) fn convert_to_stdio(io: PollEvented<NeverSource>) -> io::Result<Stdio> {
    let source: &NeverSource = &io;
    match *source {}
}
