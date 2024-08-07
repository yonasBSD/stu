# STU

[![Crate Status](https://img.shields.io/crates/v/stu.svg)](https://crates.io/crates/stu)

S3 Terminal UI

## About

STU is the TUI explorer application for Amazon S3 (AWS S3) written in Rust using [ratatui](https://github.com/ratatui-org/ratatui).

<img src="./img/demo.gif">

## Installation

### Cargo

```
$ cargo install --locked stu
```

### Homebrew (macOS)

```
$ brew install lusingander/tap/stu
```

### AUR (Arch Linux)

```
$ paru -S stu
```

### Binary

You can download binaries from [releases](https://github.com/lusingander/stu/releases).

## Usage

After installation, run the following command:

```
$ stu
```

Basically, you can use it in [the same way as the AWS CLI](https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-files.html).

In other words, if the default profile settings exist or [the environment variables are set](https://docs.aws.amazon.com/cli/latest/userguide/cli-configure-envvars.html), you do not need to specify any options.

### Options

```
STU - S3 Terminal UI

Usage: stu [OPTIONS]

Options:
  -r, --region <REGION>     AWS region
  -e, --endpoint-url <URL>  AWS endpoint url
  -p, --profile <NAME>      AWS profile name
  -b, --bucket <NAME>       Target bucket name
      --debug               Output debug logs
  -h, --help                Print help
  -V, --version             Print version
```

Here are some examples of how to run with options:

```sh
# Connect by specifying the profile
$ stu --profile foo-profile

# Show only the specified bucket objects
$ stu --bucket bar-bucket

# Connect to localstack, minio, etc.
$ stu --endpoint-url http://localhost:12345

# Connect by specifying environment variables
$ AWS_ACCESS_KEY_ID=abc AWS_SECRET_ACCESS_KEY=xyz stu
```

### Keybindings

The basic key bindings are as follows:

| Key                  | Description                        |
| -------------------- | ---------------------------------- |
| <kbd>Ctrl-C</kbd>    | Quit app                           |
| <kbd>Esc</kbd>       | Quit app / Close dialog            |
| <kbd>Enter</kbd>     | Confirm / Open selected item       |
| <kbd>Backspace</kbd> | Go back to previous / Close dialog |
| <kbd>j/k</kbd>       | Select item / Scroll               |
| <kbd>?</kbd>         | Show help                          |

Detailed operations on each view can be displayed by pressing `?` key.

### Config

Config is loaded from `$STU_ROOT_DIR/config.toml`.

- If `STU_ROOT_DIR` environment variable is not set, `~/.stu` is used by default.
- If the file does not exist, it will be created automatically at startup.
- If no value is set, the default value will be set.

The values that can be set are as follows:

- `download_dir`: _string_ - Directory to save when downloading objects (_default_: `$STU_ROOT_DIR/download`)
- `preview.highlight`: _bool_ - Whether syntax highlighting is enabled in preview (_default_: `false`)

## Features / Screenshots

### Bucket list

- Show list of buckets
  - filter/sort items

<img src="./img/bucket-list.png" width=400> <img src="./img/bucket-list-filter.png" width=400> <img src="./img/bucket-list-sort.png" width=400>

### Object list

- Show list of objects in a hierarchy
  - filter/sort items

<img src="./img/object-list-simple.png" width=400> <img src="./img/object-list-hierarchy.png" width=400> <img src="./img/object-list-many.png" width=400> <img src="./img/object-list-filter.png" width=400> <img src="./img/object-list-sort.png" width=400>

### Object detail

- Show object details
- Show object versions
- Download object
- Preview object (text file only)
  - syntax highlighting (by [syntect](https://github.com/trishume/syntect))
- Copy resource name to clipboard

<img src="./img/object-detail.png" width=400> <img src="./img/object-version.png" width=400> <img src="./img/object-download.png" width=400> <img src="./img/object-preview.png" width=400> <img src="./img/object-details-copy.png" width=400>

## Troubleshooting

- If you cannot connect to AWS S3, first check whether you can connect using the AWS CLI with the same settings.
- By running with the `--debug` flag, logs will be output to `$STU_ROOT_DIR/debug.log`.
  - Currently, application events and AWS SDK logs are output.
  - Pressing `F12` while the application is running will dump the application state to the log.
- When reporting a problem, please include the information like the following.
  - Application version
  - Operating system and version
  - Terminal you are using
  - Steps to reproduce the issue
  - Relevant log files or error messages

## License

MIT
