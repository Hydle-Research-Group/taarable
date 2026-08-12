# TAARABLE
A terminal interface for USB communication with the [TAAR robotic arm](https://github.com/Hydle-Research-Group/taar).

## Usage

To start, clone the repository:

```sh
git clone https://github.com/Hydle-Research-Group/taarable.git
```

Then run via `cargo run`. This will start a REPL for USB communication with the arm.

## Interface Standard

TAAR firmware implements an interface standard set by TAARABLE. The sender (TAARABLE) can send raw GCODE commands, while the receiver (TAAR) can respond with a JSON-based format.

### GCODE Standard

TAAR uses a GCODE style similar to that of [Marlin](https://marlinfw.org/meta/gcode/). Many commands listed share similar arguments and formats to the Marlin project, allowing for Marlin-flavored sequences to be ran on TAAR machines. 

| GCODE Command | Description |
| - | - |
| `G0 X<position> Y<position> Z<position> F<rate>` | Moves the end affector in a linear motion _[See Marlin Equivalent](https://marlinfw.org/docs/gcode/G000-G001.html)_ |
| `G4 P<ms>` | Pause the command queue for a set amount of time _[See Marlin Equivalent](https://marlinfw.org/docs/gcode/G004.html)_ | 
| `G6 X<direction> Y<direction> Z<direction> A<direction>` | Step an individual motor in a specified direction (where `X` is the base, `Y` is the shoulder, `Z` is the elbow, and `A` is the hand) _[See Marlin Equivalent](https://marlinfw.org/docs/gcode/G006.html)_ |
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
| `{ "queue": "<continue\|quit>" }` | A command queue response (`continue` or `quit`) |

## Free & Open-Source

TAARABLE is 100% free with no drawbacks or limitations. There is no "premium" version; you get the latest and greatest, all licensed under the GPL-3.0.

All source code is public, to anyone. There is no "hidden mechanism" included in this repository; every reference and used factor exists completely and fully.
