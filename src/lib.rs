use zed_extension_api::{self as zed, LanguageServerId, Result};

struct CudaExtension;

impl zed::Extension for CudaExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let command = worktree
            .which("clangd")
            .ok_or_else(|| "CUDA support requires clangd on the project PATH".to_string())?;
        Ok(zed::Command {
            command,
            args: vec![],
            env: Default::default(),
        })
    }
}

zed::register_extension!(CudaExtension);
