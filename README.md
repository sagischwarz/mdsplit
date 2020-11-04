# mdsplit

![ci badge](https://github.com/sagischwarz/mdsplit/workflows/Continuous%20integration/badge.svg)

This CLI tool will split a markdown file at H1 elements (# syntax), where the header must contain a German date string in the format `Dienstag, der 03. November 2020` and will create a file for every entry named by the corresponding date.


## Build

`cargo build --release`

## Usage

`mdsplit /path/to/markdown_file.md`

This will create a new folder named `result` next to the markdown file.