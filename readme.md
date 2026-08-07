### 0-shell
A minimalist Unix-like shell implemented in Rust.

### Description

**0-shell** is a minimalist Unix shell written in Rust. 

It provides a command-line interface to execute basic Unix commands and interact with the file system.

The project implements shell features such as command parsing, file operations, and Unix-style command behavior without using external shell programs.

---

### Features

The shell provides:

- Interactive prompt
- Command parsing and execution
- Error handling
- Graceful exit with `Ctrl+D`

Implemented commands:

- `echo`
- `cd`
- `ls` (`-l`, `-a`, `-F`)
- `pwd`
- `cat`
- `cp`
- `rm` (`-r`)
- `mv`
- `mkdir`
- `help` (displays available commands and their options)
- `exit`

---

### Installation

Clone the repository:

```bash
git clone https://github.com/iaboudou/0-shell.git
cd 0-shell