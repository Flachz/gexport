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
For manual installation (change version to the correct value according to your system).
```shell
version='gexport-x86_64-unknown-linux-musl'
curl https://github.com/Flachz/gexport/releases/latest/download/${version}.tar.xz -o gexport.tar.xz
tar xJf gexport.tar.xz
sudo cp ${version}/gexport /usr/local/bin
```

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