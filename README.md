
# genv

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![Crates.io](https://img.shields.io/crates/v/genv.svg?logo=rust)](https://crates.io/crates/genv)
[![Rust](https://img.shields.io/badge/Rust-1.79+-orange.svg)](https://www.rust-lang.org/)
[![Shells](https://img.shields.io/badge/Shells-bash%20%7C%20zsh%20%7C%20fish-green.svg)]()

---

A minimal, portable environment variable manager.
No systemd. No daemons. 

* Stores vars in `~/.config/genv/env`
* Works in any POSIX shell (`bash`, `zsh`, `dash`) + `fish`
* Subcommands: `add`, `edit`, `remove`, `list`, `export`

---

## Build

```bash
git clone https://github.com/objz/genv.git
cd genv
cargo build --release
```

The binary is at `target/release/genv`.

---

## Usage

### Add a variable

```bash
genv add TEST 123
```

### Edit a variable

```bash
genv edit TEST 456
```

### Remove a variable

```bash
genv remove TEST
```

### List variables

```bash
genv list
TEST = 456
```

### Export for your shell

bash/zsh:

```bash
eval "$(genv export)"
```

fish:

```bash
genv export | source
```

Put the appropriate line in your shell’s init file (`~/.bashrc`, `~/.zshrc`, or `~/.config/fish/config.fish`) to load all vars automatically in every new session.

---

Licensed under GPLv3. Don’t strip the license, thanks.

