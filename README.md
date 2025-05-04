# Zed golangci-lint

A Zed extension for running [golangci-lint](https://golangci-lint.run/) on your Go code.

### Configuration for golangci-lint v1

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

### Configuration for golangci-lint v2:

```json
{
  "lsp": {
    "golangci-lint": {
      "initialization_options": {
        "command": ["golangci-lint", "run", "--output.json.path", "stdout", "--issues-exit-code=1"]
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
