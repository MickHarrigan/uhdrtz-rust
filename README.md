# uhdrtz-rust
2023 Capstone Project at UMBC

Ultra High Definition Real Time Zoetrope

[Here](https://github.com/MickHarrigan/uhdrtz-rust) is a link to the repo itself.

## Team Members
- Mick Harrigan
- Christian Lostoski
- Daniel Cleaver
- Nomso Ashiogwu


## Running and Installation
Information on the hardware used and how to set up the full system are [here](https://github.com/MickHarrigan/uhdrtz-rust/wiki).

To compile and run this system first clone the repo wherever with
```
git clone git@github.com:MickHarrigan/uhdrtz-rust.git
```

then install Rust and the toolchain needed from [rustup.rs](https://rustup.rs/).

Install the dependencies (Debian)
```
sudo apt install -y libasound2-dev pkg-config clang libudev-dev libdbus-1-dev
```

From here navigate to the root of the repo and run
```
cargo run --release
```
to build and run the system.

## Major Libraries used
The UHDRTZ is built with [Bevy](https://bevyengine.org/) as the foundation of the system. The camera uses [Nokhwa](https://github.com/l1npengtul/nokhwa) to take images and output them into a raw buffer.