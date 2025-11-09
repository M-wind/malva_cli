# Marlva Cli

Based on [malva](https://github.com/g-plane/malva) lib. All credits to the malva project.

## Getting Started

### Usage

#### Build

```bash
git https://github.com/M-wind/malva_cli.git
cargo build --release
```

#### With conform.nvim

```bash
require("conform").setup({
    formatters = {
        malva = {
            inherit = false,
            command = "malva",
            args = { "-c", "your config path", "format", "$FILENAME" },
        }
    },
    formatters_by_ft = {
        css = { "malva" },
        ...
    } 
})
```

## Configuration

Please refer to [Configuration](https://malva.netlify.app/).

