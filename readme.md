# rdm - Recursive Directory MD5

rdm is a command-line tool written in Rust that can recursively calculate the md5 value of a directory.

## Installation

You can download precompiled binaries from [here](https://github.com/padeyao4/rdm/releases), or use cargo to install:

```bash
cargo install --git https://github.com/padeyao4/rdm.git
```

## Usage

```bash
rdm [OPTIONS] <DIR>
```

where `<DIR>` is the directory to calculate the md5 value.

The optional arguments are:

- `-j`, `--json`: Print json formatted output, including the md5 values of each file and subdirectory.
- `-a`, `--all`: Include hidden files and directories in calculation. By default, they are ignored.
- `-h`, `--help`: Print help information.

## Example

Assume there is a directory structure like this:

```text
test/
├── a.txt
├── b.txt
└── sub/
    ├── c.txt
    └── d.txt
```

Running rdm test will produce output like this:

```text
0a7c8f6b9e2d3a0b9c7f8d6e4f3a2c9d
```

Running `rdm -j test` will produce output like this:

```json
{
  "name": "test",
  "hash": "0a7c8f6b9e2d3a0b9c7f8d6e4f3a2c9d",
  "children": [
    {
      "name": "a.txt",
      "hash": "4124bc0a9335c27f086f24ba207a4912"
    },
    {
      "name": "b.txt",
      "hash": "47bce5c74f589f4867dbd57e9ca9f808"
    },
    {
      "name": "sub",
      "hash": "1fb8beedfcabfdffddbbdcdeecdfcd03",
      "children": [
        {
          "name": "c.txt",
          "hash": "1fb8beedfcabfdffddbbdcde74f589f4"
        },
        {
          "name": "d.txt",
          "hash": "1fb8beedfcabfdffddbbdcd47bce5c74"
        }
      ]
    }
  ]
}
```

## License

rdm is licensed under the MIT license. See LICENSE for more details.
