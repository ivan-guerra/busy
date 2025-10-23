# busy

https://github.com/user-attachments/assets/3221ee10-713e-4edc-9c7a-ea92ebd5ee41

An application to make you look busy at work by automatically moving your mouse
cursor to prevent your system from going idle.

## Features

- Automatically moves mouse cursor between two positions at configurable intervals
- Optional mouse clicking at each position
- Minimal system resource usage
- Press ESC to stop
- Cross-platform support (Linux and Windows)

## Installation

Download the latest release for your platform from the [releases page][1]:

- `bz-x86_64-pc-windows-gnu.zip` for Windows
- `bz-x86_64-unknown-linux-gnu.zip` for Linux

Extract the archive and run the `bz` executable.

## Usage

bz is console application. By default, it moves the mouse cursor every 5 seconds
between two positions on the screen without clicking. The application supports
customizing the update interval and enabling mouse clicks at each position:

```bash
An application to make you look busy at work.

Usage: bz [OPTIONS]

Options:
  -u, --update-interval <UPDATE_INTERVAL>  Update interval in seconds [default: 5]
  -c, --click                              Click at the end of the movement
  -h, --help                               Print help
  -V, --version                            Print version
```

Press the **ESC** key at any time to stop the application.

[1]: https://github.com/ivan-guerra/busy/releases
