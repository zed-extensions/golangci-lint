use std::fs;
use zed::LanguageServerId;
use zed_extension_api::{self as zed, GithubReleaseOptions, Result};

struct GolangciLintExtension {
    cached_binary_path: Option<String>,
}

#[derive(Clone)]
struct GolangciLintLangserverBinary {
    path: String,
    environment: Option<Vec<(String, String)>>,
}

impl GolangciLintExtension {
    fn language_server_binary(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<GolangciLintLangserverBinary> {
        if let Some(path) = worktree.which("golangci-lint-langserver") {
            let environment = worktree.shell_env();
            return Ok(GolangciLintLangserverBinary {
                path,
                environment: Some(environment),
            });
        }

        if let Some(path) = &self.cached_binary_path {
            if fs::metadata(&path).map_or(false, |stat| stat.is_file()) {
                return Ok(GolangciLintLangserverBinary {
                    path: path.clone(),
                    environment: None,
                });
            }
        }

        zed::set_language_server_installation_status(
            &language_server_id,
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

        if !fs::metadata(&binary_path).map_or(false, |stat| stat.is_file()) {
            zed::set_language_server_installation_status(
                &language_server_id,
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
                    fs::remove_dir_all(&entry.path()).ok();
                }
            }
        }

        self.cached_binary_path = Some(binary_path.clone());
        Ok(GolangciLintLangserverBinary {
            path: binary_path,
            environment: None,
        })
    }
}

impl zed::Extension for GolangciLintExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let golangci_lint_langserver_binary =
            self.language_server_binary(language_server_id, worktree)?;
        Ok(zed::Command {
            command: golangci_lint_langserver_binary.path,
            args: vec![],
            env: golangci_lint_langserver_binary
                .environment
                .unwrap_or_default(),
        })
    }
}

zed::register_extension!(GolangciLintExtension);
