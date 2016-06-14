# dasi-rs
[![Travis Build Status](https://img.shields.io/travis/tifalch/dasi-rs.svg?style=flat-square)](https://travis-ci.org/tibfalch/dasi-rs)

Dasi (pronounced /ˈdɑːsi/) is a [Discord](https://discordapp.com) chat client's API.

See the [documentation](http://tifalch.github.io/dasi-rs/dasi/)!

## Requirements
Note: libsodium and libopus aren't needed, if you don't parse the voice feature
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
dasi = {git = "https://github.com/tifalch/dasi-rs.git"}
```

To create a Discord client for your Bot to control you need to call `Discord::from_bot_token("YOUR-TOKEN")`

## discord-rs
This is historically based on [SpaceManiac/discord-rs](https://github.com/SpaceManiac/discord-rs)
