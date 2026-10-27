# tlu-fetch

A small command-line tool for Linux. It shows basic information about your system:

- username
- CPU model
- total memory

## Requirements

You need:

- Linux
- [Git](https://git-scm.com/)
- [Rust](https://www.rust-lang.org/tools/install) (with Cargo)

If you do not have Rust yet, install it with:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then restart your terminal, or run:

```bash
source "$HOME/.cargo/env"
```

## Install on Linux

1. Clone the repository:

```bash
git clone https://github.com/o0travaa/tlu-fetch.git
cd tlu-fetch
```

2. Build the program:

```bash
cargo build --release
```

3. Run it:

```bash
./target/release/tlu-fetch
```

You should see output like this:

```text
Info about your system:
  Username: your_username
  CPU: your CPU name
  Memory: 16384000 kB
```

## Optional: install the command system-wide

After a release build, you can copy the binary to a folder in your `PATH`:

```bash
sudo cp target/release/tlu-fetch /usr/local/bin/
```

Then you can run it from any directory:

```bash
tlu-fetch
```

## Notes

- This tool works on Linux only, because it reads `/proc/cpuinfo` and `/proc/meminfo`.
- It does not take extra arguments. Just run the command.
