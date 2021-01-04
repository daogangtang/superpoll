//! Windows-specific extensions.

use std::os::windows::process::CommandExt as _;

use crate::Command;

/// Windows-specific extensions to the [`Command`] builder.
pub trait CommandExt {
    /// Sets the [process creation flags][1] to be passed to `CreateProcess`.
    ///
    /// These will always be ORed with `CREATE_UNICODE_ENVIRONMENT`.
    ///
    /// [1]: https://docs.microsoft.com/en-us/windows/win32/procthread/process-creation-flags
    fn creation_flags(&mut self, flags: u32) -> &mut Command;
}

impl CommandExt for Command {
    fn creation_flags(&mut self, flags: u32) -> &mut Command {
        self.inner.creation_flags(flags);
        self
    }
}
