# Rust template repository.

An opinionated starting point for rust projects such as

 - systemd services
 - command line tools
 - client programs
 - server programs
 - libraries and daemons


# Logging 

The program will log in 2 places by default:

 - `stdout`
 - `syslog`

There is a simple `-v` `--verbose` flag that can be toggled on/off to increase and decrease the level of the logs.

Enabling verbose mode will simply add `Trace` and `Debug` levels to the default configuration.

| Default Runtime   | +Verbose       |
|-------------------|----------------|
 | Info, Warn, Error | +Trace, +Debug |

