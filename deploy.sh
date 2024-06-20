#!/bin/env bash
# cross builds and automatically deploys to the pi
set -e
RASPI=${1:-raspberrypi.local}
cross build --release --target aarch64-unknown-linux-musl
ssh pi@$RASPI mkdir -p /home/pi/VendingMachine
scp -r target/aarch64-unknown-linux-musl/release/sound-player asound.conf sound-player.service sounds .env.example pi@$RASPI:VendingMachine
