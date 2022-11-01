//! Process handling for platforms that don't support spawning processes.

use crate::io::{AsyncRead, AsyncWrite, ReadBuf};
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
pub(crate) enum ChildStdio {}

impl AsyncWrite for ChildStdio {
    fn poll_write(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
        _buf: &[u8],
    ) -> Poll<io::Result<usize>> {
        match *self {}
    }

    fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        match *self {}
    }

    fn poll_shutdown(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        match *self {}
    }
}

impl AsyncRead for ChildStdio {
    fn poll_read(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
        _buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        match *self {}
    }
}

pub(crate) fn convert_to_stdio(io: ChildStdio) -> io::Result<Stdio> {
    match io {}
}

pub(super) fn stdio<T>(_: T) -> io::Result<ChildStdio> {
    unimplemented!()
}
