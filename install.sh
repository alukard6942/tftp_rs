#!/bin/bash
# File: install.sh
# Author: alukard <alukard6942@github>
# Date: 11.06.2022
# Last Modified Date: 11.06.2022


cd ~/repo/tftp-rs
cargo build --release

ln -s $PWD/target/release/server /usr/sbin/server
ln -s $PWD/target/release/client /usr/bin/client
