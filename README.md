# TAARABLE
A UART communication interface for the TAAR-`x` robotic arms.

This app creates a Slint user interface over UART communication, allowing for command sending and maching homing/jogging controls.

> [!WARNING]
> TAARABLE is currently under active development, and the command standard is subject to changes. See the [roadmap issue](https://github.com/Hydle-Research-Group/taarable/issues/1) for an outline of the project timeline.

## Usage

To start, clone the repository:

```sh
git clone https://github.com/Hydle-Research-Group/taarable.git
```

Then run `cargo run` to launch the interface. For local development, Slint hot-reloading can be enabled via:

```sh
SLINT_LIVE_PREVIEW=1 cargo run
```

## Command Standard

All TAAR-`x` firmware implement the command standard set by TAARABLE. Some models may not implement all commands (see _Supported By_)

| Command | Description | Supported By |
| - | - | - |
| `home` | Homing sequence (calculate machine limits) | `TAAR-1` |
| `arm+` | Move arm up 1° | `TAAR-1` |
| `arm-` | Move arm down 1° | `TAAR-1` |
| `base+` | Move base right 1° | `TAAR-1` |
| `base-` | Move base left 1° | `TAAR-1` |
| `moveto [x: float] [y: float] [z: float]` | Move to (`x`, `y`, `z`) performing inverse kinematics | `TAAR-1` |

## Free & Open-Source

TAARABLE is 100% free with no drawbacks or limitations. There is no "premium" version; you get the latest and greatest, all licensed under the GPL-3.0.

All source code is public, to anyone. There is no "hidden mechanism" included in this repository; every reference and used factor exists completely and fully.
