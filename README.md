# Gexport
Manage and synchronize environment variables between interactive shell sessions for Bash and Zsh.

Functions similarly to `set -U` in Fish, just for Bash / Zsh and with syntax based on
standard POSIX `export`.


## Installation
### Requirements
- Bash: [bash-preexec.sh](https://github.com/rcaloras/bash-preexec)
- Zsh: -

### Cargo
```shell
cargo install gexport
```

### Manual
[Download release binary](https://github.com/Flachz/gexport/releases/latest) and extract to `/usr/local/bin`
for system-wide install or `~/.local/bin` for user install.

## Setup
### Bash
Ensure [bash-preexec.sh](https://github.com/rcaloras/bash-preexec) is installed and is
correctly initialized. For initialization of bash-preexec you should have something similar
to this in your .bashrc:
```shell
[[ -f ~/.bash-preexec.sh ]] && source ~/.bash-preexec.sh
```

Then to setup gexport itself:
```shell
echo 'eval "$(gexport --init bash)"' >> ~/.bashrc
```

### Zsh
```shell
echo 'eval "$(gexport --init zsh)"' >> ~/.zshrc
```