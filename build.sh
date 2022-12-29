#!/bin/env bash
cross build --release --target aarch64-unknown-linux-musl
scp target/aarch64-unknown-linux-musl/release/sound-player pi@raspberrypi.local:VendingMachine
scp -r sounds pi@raspberrypi.local:VendingMachine
scp -r .env pi@raspberrypi.local:VendingMachine/.env.example
