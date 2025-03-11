# Zed golangci-lint

A Zed extension for running [golangci-lint](https://golangci-lint.run/) on your Go code.

## Configuration

```json
{
  "lsp": {
    "golangci-lint": {
      "initialization_options": {
        "command": ["golangci-lint", "run", "--out-format", "json", "--issues-exit-code=1"]
      }
    }
  },
  "languages": {
    "Go": {
      "language_servers": ["gopls", "golangci-lint"]
    }
  }
}
```
