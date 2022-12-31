#!/bin/env bash
set -e
cross build --release --target aarch64-unknown-linux-musl
ssh pi@raspberrypi.local mkdir -p /home/pi/VendingMachine
scp -r target/aarch64-unknown-linux-musl/release/sound-player sound-player.service sounds .env.example pi@raspberrypi.local:VendingMachine
