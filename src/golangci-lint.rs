use std::fs;
use zed::LanguageServerId;
use zed_extension_api::{self as zed, serde_json, GithubReleaseOptions, Result};

struct GolangciLintExtension {
    cached_lsp_binary_path: Option<String>,
}

impl GolangciLintExtension {
    fn language_server_binary(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<String> {
        if let Some(path) = worktree.which("golangci-lint-langserver") {
            return Ok(path);
        }
        if let Some(path) = &self.cached_lsp_binary_path {
            if fs::metadata(path).is_ok_and(|stat| stat.is_file()) {
                return Ok(path.into());
            }
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );
        let release = zed::latest_github_release(
            "nametake/golangci-lint-langserver",
            GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        let (platform, arch) = zed::current_platform();
        let asset_name = format!(
            "golangci-lint-langserver_{os}_{arch}.{extension}",
            arch = match arch {
                zed::Architecture::Aarch64 => "arm64",
                zed::Architecture::X86 => "i386",
                zed::Architecture::X8664 => "x86_64",
            },
            os = match platform {
                zed::Os::Mac => "Darwin",
                zed::Os::Linux => "Linux",
                zed::Os::Windows => "Windows",
            },
            extension = match platform {
                zed::Os::Mac | zed::Os::Linux => "tar.gz",
                zed::Os::Windows => "zip",
            }
        );

        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| format!("no asset found matching {:?}", asset_name))?;

        let version_dir = format!("golangci-lint-langserver-{}", release.version);
        let binary_path = format!("{version_dir}/golangci-lint-langserver");

        if !fs::metadata(&binary_path).is_ok_and(|stat| stat.is_file()) {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            zed::download_file(
                &asset.download_url,
                &version_dir,
                match platform {
                    zed::Os::Mac | zed::Os::Linux => zed::DownloadedFileType::GzipTar,
                    zed::Os::Windows => zed::DownloadedFileType::Zip,
                },
            )
            .map_err(|e| format!("failed to download file: {e}"))?;

            zed::make_file_executable(&binary_path)?;

            let entries =
                fs::read_dir(".").map_err(|e| format!("failed to list working directory {e}"))?;
            for entry in entries {
                let entry = entry.map_err(|e| format!("failed to load directory entry {e}"))?;
                if entry.file_name().to_str() != Some(&version_dir) {
                    fs::remove_dir_all(entry.path()).ok();
                }
            }
        }

        self.cached_lsp_binary_path = Some(binary_path.clone());
        Ok(binary_path)
    }
}

impl zed::Extension for GolangciLintExtension {
    fn new() -> Self {
        Self {
            cached_lsp_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let env = worktree.shell_env();
        let lsp_binary_path = self.language_server_binary(language_server_id, worktree)?;

        Ok(zed::Command {
            command: lsp_binary_path,
            args: vec![],
            env: env,
        })
    }
    fn language_server_initialization_options(
        &mut self,
        _language_server_id: &LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        let mut m = serde_json::Map::with_capacity(1);
        let cmd: Vec<serde_json::Value> = vec![
            "golangci-lint",
            "run",
            "--output.json.path",
            "stdout",
            "--show-stats=false",
            "--issues-exit-code=1",
        ]
        .iter()
        .map(|s| serde_json::Value::String(s.to_string()))
        .collect();
        m.insert("command".into(), serde_json::Value::Array(cmd));
        return Ok(Some(serde_json::Value::Object(m)));
    }
}

zed::register_extension!(GolangciLintExtension);
