# MyPass
### A simple, secure and user friendly CLI password manager

<br>

# Description
Simple and user friendly CLI password manager. Uses good standards for encryption but not audited so use at your own risk. 
Entirely local (no external communications). Useful to store secrets locally.

# Installation
> Currently only for MacOS or Linux
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


## Download release
 **Coming soon**



## Basic Usage
```bash
# Create a master key (is used to encrypt and decrypt)
# The master password is currently not being cached anywhere,
# so it will prompt you for it at every sensitive operation
mypass config master 

# Create a password entry interactively
mypass create

# View all entries
mypass view

# View one password entry and its secret value
mypass view -n $ENTRY_NUMBER -p

# interactively view one entry and its secret value
mypass view one -p
```
