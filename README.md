# dasi-rs
[![Travis Build Status](https://img.shields.io/travis/TibFalch/dasi-rs.svg?style=flat-square)](https://travis-ci.org/TibFalch/dasi-rs)

Dasi (pronounced /ˈdɑːsi/) is a [Discord](https://discordapp.com) chat client's API.

## Requirements
### Linux
```bash
sudo -s
apt-get install libsodium18 libsodium-dev libopus0 libopus-dev libssl-dev
```
### Windows
If you're using MinGW on 64-bit. Run this in MinGW, to install all the requirements:
```
pacman -S mingw-w64-x86_64-pkg-config mingw-w64-x86_64-libsodium mingw-w64-x86_64-gcc mingw-w64-x86_64-openssl mingw-w64-x86_64-opus
```
### Mac

## Usage
Add this your Cargo.toml:
```toml
[dependencies]
dasi = {git = "https://github.com/TibFalch/dasi-rs.git"}
```

To create a Discord client for your Bot to control you need to call `Discord::from_bot_token("YOUR-TOKEN")`

## discord-rs
This is historically based on [SpaceManiac/discord-rs](https://github.com/SpaceManiac/discord-rs)
