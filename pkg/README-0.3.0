
# envm

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://www.gnu.org/licenses/gpl-3.0)
[![AUR](https://img.shields.io/aur/version/envm.svg?logo=archlinux)](https://aur.archlinux.org/packages/envm)
[![Rust](https://img.shields.io/badge/Rust-1.79+-orange.svg)](https://www.rust-lang.org/)
[![Shells](https://img.shields.io/badge/Shells-bash%20%7C%20zsh%20%7C%20fish-green.svg)]()

---

A minimal, portable environment variable manager.
No systemd. No daemons. 

* Stores vars in `~/.config/envm/env`
* Works in any POSIX shell (`bash`, `zsh`, `dash`) + `fish`
* Subcommands: `add`, `edit`, `remove`, `list`, `export`, `completions`

---

## Install

On Arch Linux, `envm` is available on the **AUR**:

```bash
paru -S envm
# or
yay -S envm
```

---

## Build

```bash
git clone https://github.com/objz/envm.git
cd envm
cargo build --release
```

The binary is at `target/release/envm`.

---

## Usage

### Add a variable

```bash
envm add TEST 123
```

### Edit a variable

```bash
envm edit TEST 456
```

### Remove a variable

```bash
envm remove TEST
```

### List variables

```bash
envm list
TEST = 456
```

### Export for your shell

bash/zsh:

```bash
eval "$(envm export)"
```

fish:

```bash
envm export | source
```

Put the appropriate line in your shell's init file (`~/.bashrc`, `~/.zshrc`, or `~/.config/fish/config.fish`) to load all vars automatically in every new session.

### Shell completions

```bash
# bash - add to ~/.bashrc
eval "$(envm completions bash)"

# zsh - add to ~/.zshrc
eval "$(envm completions zsh)"

# fish
envm completions fish > ~/.config/fish/completions/envm.fish
```

Tab-completing `envm edit` or `envm remove` will suggest your existing variable names.

---

Licensed under GPLv3. Don't strip the license, thanks.
