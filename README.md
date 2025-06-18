# Zed golangci-lint

A Zed extension for running [golangci-lint](https://golangci-lint.run/) on your Go code.

### Configuration

```json
{
  "lsp": {
    "golangci-lint": {
      "initialization_options": {
        "command": ["golangci-lint", "run", "--output.json.path=stdout", "--issues-exit-code=1", "--show-stats=false"]
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
