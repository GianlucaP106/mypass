# MyPass
### A simple, secure and user friendly CLI password manager


## Overview
Simple and user friendly CLI password manager. Uses good standards for encryption but not audited so use at your own risk. 
Entirely local (no external communications). Useful to store secrets locally.
## Build from source

>Building from source requires rust and cargo installed

```bash
git clone https://github.com/GianlucaP106/mypass ~/.mypass/src \
    && cd ~/.mypass/src \
    && cargo build -r \
    && cp ./target/release/mypass .. \
    && cd
```

#### Add to PATH

```bash
export PATH="$PATH:$HOME/.mypass"
```


## Basic Usage
```bash
# Create a master key (is used to encrypt and decrypt)
# The master password is currently not being cached anywhere, so it will prompt you for it at every sensitive operation
mypass config master 

# Create a password entry interactively
mypass create

# View all entries
mypass view all

# interactively view one entry and view its secret value
mypass view one -v
```
