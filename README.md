Bluetooth Idle Device Daemon (`bidd`) is a small Rust utility that operates as
a `launchd` daemon and disables Bluetooth on MacOS machines if no devices are
connected after a period of time.

## Usage

To install `bidd` and set it up as a `launchd` daemon, you can run the
following:

```bash
# Build and install the binary from source
cargo install --git https://github.com/alexander-jackson/bidd.git

# Install the `launchd` configuration
bidd setup
```

The first time it runs, you will be asked to allow Bluetooth access.
