# Zed golangci-lint

A Zed extension for running [golangci-lint](https://golangci-lint.run/) on your Go code.

### Configuration

# For newer version of golangci-lint (v2)
```json
{
  "lsp": {
    "golangci-lint": {
      "initialization_options": {
        "command": [
          "golangci-lint",
          "run",
          "--output.json.path",
          "stdout",
          "--show-stats=false",
          "--output.text.path="
        ]
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
# Or for older version of golangci-lint (v1)
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
