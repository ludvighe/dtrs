# DT

A cli tool to add or subtract specific durations (weeks, days, hours, minutes, seconds) from a given datetime.

## Usage

```sh
dt [OPTIONS] [DATETIME] [OP] [DURATION]
```

```sh
# Example usage with date
$ dt "$(date -Ins)" + 31d
-> 2024-12-12 20:21:39.019227081 +01:00
```

### Arguments

**DATETIME:** The base datetime to operate on. Defaults to now.

**OP:** "+" or "-" depending on what operation to perform.

**DURATION:** Duration to add or subtract from the datetime. Format as a number followed by a unit, e.g., "3d" for 3 days or "2h" for 2 hours. Supported units: - 'w' for weeks - 'd' for days - 'h' for hours - 'm' for minutes - 's' for seconds.

## Build

Build and output to binary `target/release/dt`.

```sh
cargo build --release
```
