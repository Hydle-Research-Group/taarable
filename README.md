# TAARABLE
A USB communication interface for the TAAR robotic arm.

This app creates a Slint user interface over UART communication, allowing for command sending and machine controls.

> [!WARNING]
> TAARABLE is currently under active development, and the interface standard is subject to changes. See the [roadmap issue](https://github.com/Hydle-Research-Group/taarable/issues/1) for an outline of the project timeline.

## Usage

To start, clone the repository:

```sh
git clone https://github.com/Hydle-Research-Group/taarable.git
```

Then use `cargo run` to launch the interface. For local development, Slint hot-reloading can be enabled via:

```sh
SLINT_LIVE_PREVIEW=1 cargo run
```

## Interface Standard

TAAR firmware implements an interface standard set by TAARABLE. The sender (TAARABLE) can send raw GCODE commands, while the receiver (TAAR) can respond with a JSON-based format.

### GCODE Standard

TAAR uses a GCODE style similar to that of [Marlin](https://marlinfw.org/meta/gcode/). Many commands listed share similar arguments and formats to the Marlin project, allowing for Marlin-flavored sequences to be ran on TAAR machines. 

| GCODE Command | Description |
| - | - |
| `G0 X<position> Y<position> Z<position> F<rate>` | Moves the end affector in a linear motion _[See Marlin Equivalent](https://marlinfw.org/docs/gcode/G000-G001.html)_ |
| `G4 P<ms>` | Pause the command queue for a set amount of time _[See Marlin Equivalent](https://marlinfw.org/docs/gcode/G004.html)_ | 
| `G60` | Save the current end effector position in memory |
| `G61` | Move the end effector to a previously saved position |
| `G92 X<position> Y<position> Z<position>` | Set end effector position _[See Marlin Equivalent](https://marlinfw.org/docs/gcode/G092.html)_ |
| `M02` | End the sequence, ignoring all commands after |

### JSON Standard

| JSON String | Description |
| - | - |
| `{ "info": "<message>" }` | An information response |
| `{ "warning": "<message>" }` | A warning response |
| `{ "error": "<message>" }` | An error response |

## Free & Open-Source

TAARABLE is 100% free with no drawbacks or limitations. There is no "premium" version; you get the latest and greatest, all licensed under the GPL-3.0.

All source code is public, to anyone. There is no "hidden mechanism" included in this repository; every reference and used factor exists completely and fully.
