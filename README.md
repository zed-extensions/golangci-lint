# zed_golangci_lint

A Zed extension for running [golangci-lint](https://golangci-lint.run/) on your Go code.

## Fork motivation

The upstream repo has several issues, which are adressed by this fork:

1. It is unmaintained
2. It has a faulty asset reference
3. It has no configuration documentation
4. It misses the required `languages` list in `extension.toml`
5. It uses an old version of the `zed_extension_api`

## Configuration

In order to use this fork, there are essentially three things that must be done:

1. Install the fork as a Dev Extension
2. Configure the golangci-lint language server
3. Restart Zed

### 1. Install the fork as a Dev Extension

1. Clone this repo
2. In Zed, open Extensions
3. Click "Install Dev Extension"
4. Select the filesystem path of the cloned repo

#### 2. Configure the golangci-lint language server in Zed settings (`settings.json`)

```json
{
  "lsp": {
    // add the "golangci-lint" block
    "golangci-lint": {
      "initialization_options": {
        "command": [
          "golangci-lint",
          "run",
          "--out-format",
          "json",
          "--issues-exit-code=1"
        ]
      }
    }
  },
  "languages": {
    "Go": {
      // add  the "golangci-lint" element
      "language_servers": ["gopls", "golangci-lint"]
    }
  }
}
```

### 3. Restart Zed
